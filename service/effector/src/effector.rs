/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(improper_ctypes)]

use eyre::{Result, WrapErr};
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use std::fs;

use itertools::Itertools;
use types::{IpfsCatResult, IpfsGetPeerIdResult, IpfsPutResult, IpfsResult};

module_manifest!();

/// Default chunk size for `ipfs add` command to produce stable CIDs.
const CHUCK_SIZE: usize = 262144;

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

/// Run `ipfs` mounted binary with the specified arguments
fn run_ipfs(cmd: Vec<String>) -> Result<String> {
    let result = ipfs(cmd.clone());

    result
        .into_std()
        .ok_or(eyre::eyre!(
            "stdout or stderr contains non valid UTF8 string"
        ))?
        .map_err(|e| eyre::eyre!("ipfs cli call failed \n{:?}: {}", cmd.iter().join("  "), e))
}

#[inline]
fn format_timeout(timeout_sec: u64) -> String {
    format!("{}s", timeout_sec)
}

fn make_cmd_args(args: Vec<String>, api_multiaddr: String, timeout_sec: u64) -> Vec<String> {
    args.into_iter()
        .chain(vec![
            String::from("--timeout"),
            format_timeout(timeout_sec),
            String::from("--api"),
            api_multiaddr,
        ])
        .collect()
}

#[marine]
pub fn connect(multiaddr: String, api_multiaddr: String, timeout_sec: u64) -> IpfsResult {
    let args = vec![String::from("swarm"), String::from("connect"), multiaddr];
    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    run_ipfs(cmd).map(|_| ()).into()
}

/// Put file from specified path to IPFS and return its hash.
#[marine]
pub fn put(file_path: String, api_multiaddr: String, timeout_sec: u64) -> IpfsPutResult {
    if !std::path::Path::new(&file_path).exists() {
        return IpfsPutResult {
            success: false,
            error: format!("path {} doesn't exist", file_path),
            hash: "".to_string(),
        };
    }

    let args = vec![
        String::from("add"),
        String::from("-Q"),
        inject_vault_host_path(file_path),
        String::from("--cid-version=1"),
        format!("--chunker=size-{}", CHUCK_SIZE),
    ];
    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);
    run_ipfs(cmd).map(|res| res.trim().to_string()).into()
}

/// Put dag from specified path to IPFS and return its hash.
#[marine]
pub fn dag_put(file_path: String, api_multiaddr: String, timeout_sec: u64) -> IpfsPutResult {
    if !std::path::Path::new(&file_path).exists() {
        return IpfsPutResult {
            success: false,
            error: format!("path {} doesn't exist", file_path),
            hash: "".to_string(),
        };
    }

    let args = vec![
        String::from("dag"),
        String::from("put"),
        inject_vault_host_path(file_path),
    ];
    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);
    run_ipfs(cmd).map(|res| res.trim().to_string()).into()
}

/// Get file by provided hash from IPFS, save it to a `file_path`, and return that path
#[marine]
pub fn get(hash: String, file_path: &str, api_multiaddr: String, timeout_sec: u64) -> IpfsResult {
    let args = vec![
        String::from("get"),
        String::from("-o"),
        inject_vault_host_path(file_path.to_string()),
        hash,
    ];
    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    run_ipfs(cmd).map(drop).into()
}

/// Get dag by provided hash from IPFS, save it to a `file_path`, and return that path
#[marine]
pub fn dag_get(
    hash: String,
    file_path: &str,
    api_multiaddr: String,
    timeout_sec: u64,
) -> IpfsResult {
    let args = vec![String::from("dag"), String::from("get"), hash];
    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    let result: Result<()> = try {
        let dag = run_ipfs(cmd)?;
        fs::write(inject_vault_host_path(file_path.to_string()), dag)?
    };

    result
        .map(|_| ())
        .map_err(|e| eyre::eyre!("dag_get error: {:?}", e))
        .into()
}

#[marine]
pub fn get_peer_id(api_multiaddr: String, timeout_sec: u64) -> IpfsGetPeerIdResult {
    let result: Result<String> = try {
        let cmd = make_cmd_args(vec![String::from("id")], api_multiaddr, timeout_sec);

        let result = run_ipfs(cmd)?;
        let result: serde_json::Value =
            serde_json::from_str(&result).wrap_err("ipfs response parsing failed")?;
        result
            .get("ID")
            .ok_or(eyre::eyre!("ID field not found in response"))?
            .as_str()
            .ok_or(eyre::eyre!("ID value is not string"))?
            .to_string()
    };

    result
        .map_err(|e| eyre::eyre!("get_peer_id error: {:?}", e))
        .into()
}

/// Cat file by provided hash from IPFS,
#[marine]
pub fn cat(hash: String, api_multiaddr: String, timeout_sec: u64) -> IpfsCatResult {
    let args = vec![String::from("cat"), hash];
    let cmd = make_cmd_args(args, api_multiaddr, timeout_sec);

    run_ipfs(cmd)
        .map_err(|e| eyre::eyre!("ipfs cat error: {:?}", e))
        .into()
}

#[marine]
#[host_import]
extern "C" {
    /// Execute provided cmd as a parameters of ipfs cli, return result.
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}

fn inject_vault_host_path(path: String) -> String {
    let vault = "/tmp/vault";
    if let Some(stripped) = path.strip_prefix(&vault) {
        let host_vault_path = std::env::var(vault).expect("vault must be mapped to /tmp/vault");
        format!("/{}/{}", host_vault_path, stripped)
    } else {
        path
    }
}
