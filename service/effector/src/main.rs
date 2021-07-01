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
#![feature(try_blocks)]

use types::IpfsResult;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;

use eyre::Result;

module_manifest!();

fn unwrap_mounted_binary_result(result: MountedBinaryResult) -> Result<String> {
    result.into_std().ok_or(eyre::eyre!("stdout or stderr contains non valid UTF8 string"))?.map_err(|e| eyre::eyre!("ipfs cli call failed: {}", e))
}

#[inline]
fn get_timeout_string(timeout: u64) -> String { format!("{}s", timeout) }

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn connect(multiaddr: String, timeout_sec: u64) -> IpfsResult {
    log::info!("connect called with multiaddr {}", multiaddr);

    let cmd = vec![
        String::from("swarm"),
        String::from("connect"),
        String::from("--timeout"),
        get_timeout_string(timeout_sec),
        multiaddr
    ];

    unwrap_mounted_binary_result(ipfs(cmd)).map(|_| ()).into()
}

/// Put file from specified path to IPFS and return its hash.
#[marine]
pub fn put(file_path: String, timeout_sec: u64) -> IpfsResult {
    log::info!("put called with file path {}", file_path);

    if !std::path::Path::new(&file_path).exists() {
        return IpfsResult { success: false, result: format!("path {} doesn't exist", file_path) }
    }

    let cmd = vec![
        String::from("add"),
        String::from("--timeout"),
        get_timeout_string(timeout_sec),
        String::from("-Q"),
    ];

    log::info!("ipfs put args {:?}", cmd);

    unwrap_mounted_binary_result(ipfs(cmd)).into()
}

/// Get file by provided hash from IPFS, saves it to a temporary file and returns a path to it.
#[marine]
pub fn get(hash: String, file_path: String, timeout_sec: u64) -> IpfsResult {
    log::info!("get called with hash {}", hash);

    let cmd = vec![
        String::from("get"),
        String::from("--timeout"),
        get_timeout_string(timeout_sec),
        String::from("-o"),
        file_path,
        hash,
    ];

    log::info!("ipfs get args {:?}", cmd);

    unwrap_mounted_binary_result(ipfs(cmd)).map(|output| {
        log::info!("ipfs get output: {}", output);
    }).into()
}

#[marine]
pub fn get_peer_id(timeout_sec: u64) -> IpfsResult {
    let result: Result<String> = try {
        let cmd = vec![
            String::from("id"),
            String::from("--timeout"),
            get_timeout_string(timeout_sec),
        ];

        let result: serde_json::Value = serde_json::from_str(&unwrap_mounted_binary_result(ipfs(cmd))?)?;
        result.get("ID").ok_or(eyre::eyre!("ID field not found in response"))?.as_str().ok_or(eyre::eyre!("ID value is not string"))?.to_string()
    };

    result.map_err(|e| eyre::eyre!("get_peer_id: {:?}", e)).into()
}

#[marine]
pub fn set_external_multiaddr(multiaddr: String, timeout_sec: u64) -> IpfsResult {

    let cmd = vec![
        String::from("config"),
        String::from("--timeout"),
        get_timeout_string(timeout_sec),
        String::from("Addresses.Announce"),
        format!("[\"{}\"]", multiaddr),
        String::from("--json"),
    ];

    unwrap_mounted_binary_result(ipfs(cmd)).map(|_| ()).into()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
    /// Execute provided cmd as a parameters of ipfs cli, return result.
    pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}
