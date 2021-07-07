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
use marine_rs_sdk::WasmLoggerBuilder;

use std::fs;
use serde::{Deserialize, Serialize};
use multiaddr::{Multiaddr, Protocol, multihash::Multihash};
use std::str::FromStr;

const CONFIG_FILE_PATH: &str = "/tmp/multiaddr_config";
const DEFAULT_TIMEOUT_SEC: u64 = 1u64;
const DEFAULT_LOCAL_MULTIADDR: &str = "/ip4/127.0.0.1/tcp/5001";
module_manifest!();

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub timeout: u64,
    pub external_multiaddr: Option<Multiaddr>,
    pub local_multiaddr: Multiaddr,
}

fn save_external_multiaddr(multiaddr: Multiaddr) {
    let mut config = load_config();
    config.external_multiaddr = Some(multiaddr);
    write_config(config);
}

fn load_external_multiaddr() -> eyre::Result<Multiaddr> {
    load_config().external_multiaddr.ok_or(eyre::eyre!("multiaddr is not set"))
}

pub fn write_config(config: Config) {
    fs::write(CONFIG_FILE_PATH, toml::to_string(&config).unwrap()).unwrap();
}

pub fn load_config() -> Config {
    let file_content = fs::read_to_string(CONFIG_FILE_PATH).unwrap();
    let config: Config = toml::from_str(&file_content).unwrap();
    config
}

pub(crate) fn create_config() {
    if fs::metadata(CONFIG_FILE_PATH).is_err() {
        write_config(Config {
            timeout: DEFAULT_TIMEOUT_SEC,
            external_multiaddr: None,
            local_multiaddr: Multiaddr::from_str(DEFAULT_LOCAL_MULTIADDR).unwrap(),
        });
    }
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
    create_config();
}

#[marine]
pub fn invoke() -> String {
    "IPFS_RPC wasm example, it allows to:\ninvoke\nput\nget".to_string()
}

#[marine]
pub fn put(file_path: String) -> IpfsResult {
    log::info!("put called with {:?}", file_path);
    let timeout = load_config().timeout;
    ipfs_put(file_path, load_config().local_multiaddr.to_string(), timeout)
}

#[marine]
pub fn get_from(hash: String, multiaddr: String) -> IpfsResult {
    log::info!("get called with hash: {}", hash);
    let config = load_config();
    let timeout = config.timeout;
    let local_maddr = config.local_multiaddr.to_string();

    let particle_id = marine_rs_sdk::get_call_parameters().particle_id;
    let connect_result = ipfs_connect(multiaddr, local_maddr.clone(), timeout.clone());

    if !connect_result.success {
        return connect_result;
    }

    let particle_vault_path = format!("/tmp/vault/{}", particle_id);
    let file_path = format!("{}/{}", particle_vault_path, hash);
    let get_result = ipfs_get(hash, file_path.clone(), local_maddr, timeout);

    if get_result.success {
        IpfsResult { success: true, result: file_path }
    } else {
        get_result
    }
}

#[marine]
pub fn get_external_multiaddr() -> IpfsResult {
    load_external_multiaddr().map(|m| m.to_string()).into()
}

#[marine]
pub fn set_external_multiaddr(multiaddr: String) -> IpfsResult {
    if load_external_multiaddr().is_ok() {
        return eyre::Result::<()>::Err(eyre::eyre!("external multiaddr can only be set once")).into();
    }

    let call_parameters = marine_rs_sdk::get_call_parameters();
    if call_parameters.init_peer_id != call_parameters.service_creator_peer_id {
        return eyre::Result::<()>::Err(eyre::eyre!("only service creator can set external multiaddr")).into();
    }

    let config = load_config();
    let timeout = config.timeout;
    let local_maddr = config.local_multiaddr.to_string();
    let result: eyre::Result<()> = try {
        let mut multiaddr = Multiaddr::from_str(&multiaddr)?;
        let mut passed_peer_id = None;
        match multiaddr.iter().count() {
            3 => {
                passed_peer_id = multiaddr.pop();
            }
            2 => {}
            n => Err(eyre::eyre!("multiaddr should containt 2 or 3 components, {} given", n))?,
        }

        let set_result = ipfs_set_external_multiaddr(multiaddr.to_string(), local_maddr.clone(), timeout.clone());
        if !set_result.success {
            return set_result;
        }

        let peer_id_result = ipfs_get_peer_id(local_maddr, timeout);
        if !peer_id_result.success {
            return peer_id_result;
        }

        let peer_id = Protocol::P2p(Multihash::from_bytes(&bs58::decode(peer_id_result.result).into_vec()?)?);
        if passed_peer_id.is_some() && passed_peer_id != Some(peer_id.clone()) {
            Err(eyre::eyre!("given peer id is different from node peer_id: given {}, actual {}", passed_peer_id.unwrap().to_string(), peer_id.to_string()))?;
        }

        multiaddr.push(peer_id);
        save_external_multiaddr(multiaddr);
        ()
    };

    result.into()
}

#[marine]
pub fn get_local_multiaddr() -> IpfsResult {
    IpfsResult { success: true, result: load_config().local_multiaddr.to_string() }
}

#[marine]
pub fn set_local_multiaddr(multiaddr: String) -> IpfsResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();
    if call_parameters.init_peer_id != call_parameters.service_creator_peer_id {
        return eyre::Result::<()>::Err(eyre::eyre!("only service creator can set local multiaddr")).into();
    }

    let result: eyre::Result<()> = try {
        let mut config = load_config();
        config.local_multiaddr = Multiaddr::from_str(&multiaddr)?;
        write_config(config);
        ()
    };

    result.into()
}

#[marine]
pub fn set_timeout(timeout_sec: u64) {
    let mut config = load_config();
    config.timeout = timeout_sec;
    write_config(config);
}


#[marine]
#[link(wasm_import_module = "ipfs_effector")]
extern "C" {
    #[link_name = "connect"]
    pub fn ipfs_connect(external_multiaddr: String, local_multiaddr: String, timeout_sec: u64) -> IpfsResult;

    /// Put provided file to ipfs, return ipfs hash of the file.
    #[link_name = "put"]
    pub fn ipfs_put(file_path: String, local_multiaddr: String, timeout_sec: u64) -> IpfsResult;

    /// Get file from ipfs by hash.
    #[link_name = "get"]
    pub fn ipfs_get(hash: String, file_path: String, local_multiaddr: String, timeout_sec: u64) -> IpfsResult;

    #[link_name = "get_peer_id"]
    pub fn ipfs_get_peer_id(local_multiaddr: String, timeout_sec: u64) -> IpfsResult;

    #[link_name = "set_external_multiaddr"]
    pub fn ipfs_set_external_multiaddr(external_multiaddr: String, local_multiaddr: String, timeout_sec: u64) -> IpfsResult;
}
