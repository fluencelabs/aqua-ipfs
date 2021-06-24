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

mod results;

use crate::results::IpfsResult;

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::WasmLoggerBuilder;

use std::fs;
use std::path::PathBuf;

const MULTIADDR_FILE_PATH: &str = "/tmp/multiaddr_config";
const RPC_TMP_FILEPATH: &str = "/tmp/ipfs_rpc_file";

module_manifest!();

fn load_multiaddr() -> eyre::Result<String> {
    Ok(fs::read_to_string(MULTIADDR_FILE_PATH)?)
}

fn save_multiaddr(multiaddr: String) -> eyre::Result<()> {
    Ok(fs::write(MULTIADDR_FILE_PATH, multiaddr)?)
}

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn invoke() -> String {
    "IPFS_RPC wasm example, it allows to:\ninvoke\nput\nget".to_string()
}

#[marine]
pub fn put(file_content: Vec<u8>) -> String {
    log::info!("put called with {:?}", file_content);

    let rpc_tmp_filepath = RPC_TMP_FILEPATH.to_string();

    let r = fs::write(PathBuf::from(rpc_tmp_filepath.clone()), file_content);
    if let Err(e) = r {
        return format!("file can't be written: {}", e);
    }

    ipfs_put(rpc_tmp_filepath)
}

#[marine]
pub fn get(hash: String) -> Vec<u8> {
    log::info!("get called with hash: {}", hash);

    let file_path = ipfs_get(hash);
    fs::read(file_path).unwrap_or_else(|_| b"error while reading file".to_vec())
}

#[marine]
pub fn get_multiaddr() -> IpfsResult {
    load_multiaddr().into()
}

pub fn set_multiaddr(multiaddr: String) -> IpfsResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();
    if load_multiaddr().is_ok() || call_parameters.init_peer_id != call_parameters.service_creator_peer_id {
        return eyre::Result::<()>::Err(eyre::eyre!("only service creator can set multiaddr only once")).into();
    }

    save_multiaddr(multiaddr).into()
}


#[marine]
#[link(wasm_import_module = "ipfs_effector")]
extern "C" {
    /// Put provided file to ipfs, return ipfs hash of the file.
    #[link_name = "put"]
    pub fn ipfs_put(file_path: String) -> String;

    /// Get file from ipfs by hash.
    #[link_name = "get"]
    pub fn ipfs_get(hash: String) -> String;
}
