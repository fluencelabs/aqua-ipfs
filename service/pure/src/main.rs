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

use types::IpfsResult;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use std::fs;
use serde::{Deserialize, Serialize};

const CONFIG_FILE_PATH: &str = "/tmp/multiaddr_config";
const DEFAULT_TIMEOUT_SEC: u64 = 1u64;
module_manifest!();

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub timeout: u64,
    pub multiaddr: Option<String>
}

fn save_multiaddr(multiaddr: String) {
    let mut config = load_config();
    config.multiaddr = Some(multiaddr);
    write_config(config);
}

fn load_multiaddr() -> eyre::Result<String> {
    load_config().multiaddr.ok_or(eyre::eyre!("multiaddr is not set"))
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
            multiaddr: None
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
    ipfs_put(file_path, timeout)
}

#[marine]
pub fn get_from(hash: String, multiaddr: String) -> IpfsResult {
    log::info!("get called with hash: {}", hash);
    let timeout = load_config().timeout;

    let particle_id = marine_rs_sdk::get_call_parameters().particle_id;
    let connect_result = ipfs_connect(multiaddr, timeout.clone());

    if !connect_result.success {
        return connect_result;
    }

    let particle_vault_path = format!("/tmp/vault/{}", particle_id);
    let file_path = format!("{}/{}", particle_vault_path, hash);
    ipfs_get(hash, file_path, timeout)
}

#[marine]
pub fn get_multiaddr() -> IpfsResult {
    load_multiaddr().into()
}

#[marine]
pub fn set_multiaddr(multiaddr: String) -> IpfsResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();
    if load_multiaddr().is_ok() || call_parameters.init_peer_id != call_parameters.service_creator_peer_id {
        return eyre::Result::<()>::Err(eyre::eyre!("only service creator can set multiaddr only once")).into();
    }

    let timeout = load_config().timeout;
    let set_result = ipfs_set_external_multiaddr(multiaddr.clone(), timeout.clone());
    if !set_result.success {
        return set_result;
    }

    let peer_id_result = ipfs_get_peer_id(timeout);
    if !peer_id_result.success {
        return peer_id_result;
    }

    // trim trailing /
    let multiaddr = if multiaddr.ends_with("/") { multiaddr[..multiaddr.len() - 1].to_string() } else { multiaddr.clone() };
    save_multiaddr(format!("{}/{}", multiaddr, peer_id_result.result));
    Ok(()).into()
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
    pub fn ipfs_connect(multiaddr: String, timeout_sec: u64) -> IpfsResult;


    /// Put provided file to ipfs, return ipfs hash of the file.
    #[link_name = "put"]
    pub fn ipfs_put(file_path: String, timeout_sec: u64) -> IpfsResult;

    /// Get file from ipfs by hash.
    #[link_name = "get"]
    pub fn ipfs_get(hash: String, file_path: String, timeout_sec: u64) -> IpfsResult;

    #[link_name = "get_peer_id"]
    pub fn ipfs_get_peer_id(timeout_sec: u64) -> IpfsResult;

    #[link_name = "set_external_multiaddr"]
    pub fn ipfs_set_external_multiaddr(multiaddr: String, timeout_sec: u64) -> IpfsResult;
}
