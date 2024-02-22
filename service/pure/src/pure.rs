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

use types::{
    IpfsCatResult, IpfsGetPeerIdResult, IpfsGetResult, IpfsMultiaddrResult, IpfsPutResult,
    IpfsResult,
};

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::ParticleParameters;
use marine_rs_sdk::WasmLoggerBuilder;

use eyre::WrapErr;
use multiaddr::{multihash::Multihash, Multiaddr, Protocol};
use serde::{Deserialize, Serialize};
use std::fs;
use std::str::FromStr;

const CONFIG_FILE_PATH: &str = "/tmp/multiaddr_config";
const DEFAULT_TIMEOUT_SEC: u64 = 10u64;
module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
    reset_config();
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub timeout: u64,
    pub external_api_multiaddr: Option<Multiaddr>,
    pub external_swarm_multiaddr: Option<Multiaddr>,
    pub local_api_multiaddr: Option<Multiaddr>,
}

fn save_external_api_multiaddr(multiaddr: Multiaddr) {
    let mut config = load_config();
    config.external_api_multiaddr = Some(multiaddr);
    write_config(config);
}

fn load_external_api_multiaddr() -> eyre::Result<Multiaddr> {
    load_config()
        .external_api_multiaddr
        .ok_or(eyre::eyre!("external api multiaddr is not set"))
}

fn save_local_api_multiaddr(multiaddr: Multiaddr) {
    let mut config = load_config();
    config.local_api_multiaddr = Some(multiaddr);
    write_config(config);
}

fn load_local_api_multiaddr() -> eyre::Result<Multiaddr> {
    load_config()
        .local_api_multiaddr
        .ok_or(eyre::eyre!("local api multiaddr is not set"))
}

pub fn write_config(config: Config) {
    fs::write(CONFIG_FILE_PATH, toml::to_string(&config).unwrap()).unwrap();
}

pub fn load_config() -> Config {
    let file_content = fs::read_to_string(CONFIG_FILE_PATH).unwrap();
    let config: Config = toml::from_str(&file_content).unwrap();
    config
}

pub(crate) fn reset_config() {
    write_config(Config {
        timeout: DEFAULT_TIMEOUT_SEC,
        external_api_multiaddr: None,
        external_swarm_multiaddr: None,
        local_api_multiaddr: None,
    });
}

pub fn get_peer_id(api_multiaddr: String, timeout: u64) -> eyre::Result<Protocol<'static>> {
    let peer_id_result = ipfs_get_peer_id(api_multiaddr, timeout);
    if !peer_id_result.success {
        Err(eyre::eyre!(peer_id_result.error.clone()))?;
    }

    Ok(Protocol::P2p(
        Multihash::from_bytes(&bs58::decode(peer_id_result.peer_id.clone()).into_vec()?).wrap_err(
            format!(
                "error parsing peer_id from 'ipfs id': {}",
                peer_id_result.peer_id
            ),
        )?,
    ))
}

fn vault_path(particle: &ParticleParameters) -> String {
    format!("/tmp/vault/{}-{}", particle.id, particle.token)
}

#[marine]
pub fn connect(multiaddr: String) -> IpfsResult {
    if Multiaddr::from_str(&multiaddr).is_err() {
        return Err(eyre::eyre!("invalid multiaddr: {}", multiaddr)).into();
    }

    let timeout = load_config().timeout;
    let local_maddr = load_local_api_multiaddr().map(|m| m.to_string());

    if local_maddr.is_ok() {
        ipfs_connect(multiaddr, local_maddr.unwrap(), timeout)
    } else {
        local_maddr.map(drop).into()
    }
}

#[marine]
pub fn put(file_path: String) -> IpfsPutResult {
    let timeout = load_config().timeout;
    let local_maddr = load_local_api_multiaddr().map(|m| m.to_string());
    if local_maddr.is_ok() {
        ipfs_put(file_path, local_maddr.unwrap(), timeout)
    } else {
        local_maddr.into()
    }
}

#[marine]
pub fn dag_put(file_path: String) -> IpfsPutResult {
    let timeout = load_config().timeout;
    let local_maddr = load_local_api_multiaddr().map(|m| m.to_string());
    if local_maddr.is_ok() {
        ipfs_dag_put(file_path, local_maddr.unwrap(), timeout)
    } else {
        local_maddr.into()
    }
}

#[marine]
pub fn get(hash: String) -> IpfsGetResult {
    let local_maddr = load_local_api_multiaddr().map(|m| m.to_string());
    if local_maddr.is_ok() {
        get_from(hash, local_maddr.unwrap())
    } else {
        local_maddr.into()
    }
}

#[marine]
pub fn get_from(hash: String, external_multiaddr: String) -> IpfsGetResult {
    let config = load_config();
    let timeout = config.timeout;

    if Multiaddr::from_str(&external_multiaddr).is_err() {
        return Err(eyre::eyre!("invalid multiaddr: {}", external_multiaddr)).into();
    }

    let particle = marine_rs_sdk::get_call_parameters().particle;
    let particle_vault_path = vault_path(&particle);
    let path = format!("{}/{}", particle_vault_path, hash);
    let get_result = ipfs_get(hash, &path, external_multiaddr, timeout);

    if get_result.success {
        Ok(path).into()
    } else {
        Err(eyre::eyre!(get_result.error)).into()
    }
}

#[marine]
pub fn dag_get(hash: String) -> IpfsGetResult {
    let local_maddr = load_local_api_multiaddr().map(|m| m.to_string());
    if local_maddr.is_ok() {
        dag_get_from(hash, local_maddr.unwrap())
    } else {
        local_maddr.into()
    }
}

#[marine]
pub fn dag_get_from(hash: String, external_multiaddr: String) -> IpfsGetResult {
    let config = load_config();
    let timeout = config.timeout;

    if Multiaddr::from_str(&external_multiaddr).is_err() {
        return Err(eyre::eyre!("invalid multiaddr: {}", external_multiaddr)).into();
    }

    let particle = marine_rs_sdk::get_call_parameters().particle;
    let particle_vault_path = vault_path(&particle);
    let path = format!("{}/{}", particle_vault_path, hash);
    let get_result = ipfs_dag_get(hash, &path, external_multiaddr, timeout);

    if get_result.success {
        Ok(path).into()
    } else {
        Err(eyre::eyre!(get_result.error)).into()
    }
}

#[marine]
pub fn cat(hash: String) -> IpfsCatResult {
    let local_maddr = load_local_api_multiaddr().map(|m| m.to_string());
    if local_maddr.is_ok() {
        cat_from(hash, local_maddr.unwrap())
    } else {
        local_maddr.into()
    }
}

#[marine]
pub fn cat_from(hash: String, external_multiaddr: String) -> IpfsCatResult {
    let config = load_config();
    let timeout = config.timeout;

    if Multiaddr::from_str(&external_multiaddr).is_err() {
        return Err(eyre::eyre!("invalid multiaddr: {}", external_multiaddr)).into();
    }

    ipfs_cat(hash, external_multiaddr, timeout)
}

#[marine]
pub fn get_external_api_multiaddr() -> IpfsMultiaddrResult {
    load_external_api_multiaddr().map(|m| m.to_string()).into()
}

#[marine]
pub fn set_external_api_multiaddr(multiaddr: String) -> IpfsResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();
    if call_parameters.particle.init_peer_id != call_parameters.service_creator_peer_id {
        return eyre::Result::<()>::Err(eyre::eyre!(
            "only service creator can set external api multiaddr"
        ))
        .into();
    }

    let config = load_config();
    let timeout = config.timeout;

    let result: eyre::Result<()> = try {
        let mut multiaddr = Multiaddr::from_str(&multiaddr)
            .wrap_err(format!("invalid multiaddr: {}", multiaddr))?;
        let local_maddr = load_local_api_multiaddr()?.to_string();
        let mut passed_peer_id = None;
        match multiaddr.iter().count() {
            3 => {
                passed_peer_id = multiaddr.pop();
            }
            2 => {}
            n => Err(eyre::eyre!(
                "multiaddr should contain 2 or 3 components, {} given",
                n
            ))?,
        }

        let peer_id = get_peer_id(local_maddr, timeout)?;
        if passed_peer_id.is_some() && passed_peer_id != Some(peer_id.clone()) {
            Err(eyre::eyre!(
                "given peer id is different from node peer_id: given {}, actual {}",
                passed_peer_id.unwrap().to_string(),
                peer_id.to_string()
            ))?;
        }

        multiaddr.push(peer_id);
        save_external_api_multiaddr(multiaddr);
        ()
    };

    result.into()
}

#[marine]
pub fn get_local_api_multiaddr() -> IpfsMultiaddrResult {
    load_local_api_multiaddr().map(|m| m.to_string()).into()
}

#[marine]
pub fn set_local_api_multiaddr(multiaddr: String) -> IpfsResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();
    if call_parameters.particle.init_peer_id != call_parameters.service_creator_peer_id {
        return eyre::Result::<()>::Err(eyre::eyre!(
            "only service creator can set local api multiaddr"
        ))
        .into();
    }

    let result: eyre::Result<()> = try {
        save_local_api_multiaddr(
            Multiaddr::from_str(&multiaddr)
                .wrap_err(format!("invalid multiaddr: {}", multiaddr))?,
        )
    };

    result.into()
}

#[marine]
pub fn get_external_swarm_multiaddr() -> IpfsMultiaddrResult {
    load_config()
        .external_swarm_multiaddr
        .ok_or(eyre::eyre!("multiaddr is not set"))
        .map(|m| m.to_string())
        .into()
}

#[marine]
pub fn set_external_swarm_multiaddr(multiaddr: String) -> IpfsResult {
    let call_parameters = marine_rs_sdk::get_call_parameters();
    if call_parameters.particle.init_peer_id != call_parameters.service_creator_peer_id {
        return eyre::Result::<()>::Err(eyre::eyre!(
            "only service creator can set external swarm multiaddr"
        ))
        .into();
    }

    let result: eyre::Result<()> = try {
        let mut config = load_config();

        let mut multiaddr = Multiaddr::from_str(&multiaddr)
            .wrap_err(format!("invalid multiaddr: {}", multiaddr))?;
        let local_maddr = load_local_api_multiaddr()?.to_string();

        let mut passed_peer_id = None;
        match multiaddr.iter().count() {
            3 => {
                passed_peer_id = multiaddr.pop();
            }
            2 => {}
            n => Err(eyre::eyre!(
                "multiaddr should contain 2 or 3 components, {} given",
                n
            ))?,
        }

        let peer_id = get_peer_id(local_maddr.clone(), config.timeout)?;
        if passed_peer_id.is_some() && passed_peer_id != Some(peer_id.clone()) {
            Err(eyre::eyre!(
                "given peer id is different from node peer_id: given {}, actual {}",
                passed_peer_id.unwrap().to_string(),
                peer_id.to_string()
            ))?;
        }

        multiaddr.push(peer_id);
        config.external_swarm_multiaddr = Some(multiaddr);
        write_config(config)
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
#[module_import("ipfs_effector")]
extern "C" {
    #[link_name = "connect"]
    pub fn ipfs_connect(
        external_multiaddr: String,
        api_multiaddr: String,
        timeout_sec: u64,
    ) -> IpfsResult;

    /// Put provided file to ipfs, return ipfs hash of the file.
    #[link_name = "put"]
    pub fn ipfs_put(file_path: String, api_multiaddr: String, timeout_sec: u64) -> IpfsPutResult;

    /// Put provided dag to ipfs, return ipfs hash of the dag.
    #[link_name = "dag_put"]
    pub fn ipfs_dag_put(dag: String, api_multiaddr: String, timeout_sec: u64) -> IpfsPutResult;

    /// Get file from ipfs by hash.
    #[link_name = "get"]
    pub fn ipfs_get(
        hash: String,
        file_path: &str,
        api_multiaddr: String,
        timeout_sec: u64,
    ) -> IpfsResult;

    // Get dag from ipfs by hash.
    #[link_name = "dag_get"]
    pub fn ipfs_dag_get(
        hash: String,
        file_path: &str,
        api_multiaddr: String,
        timeout_sec: u64,
    ) -> IpfsResult;

    /// Get file from ipfs by hash.
    #[link_name = "cat"]
    pub fn ipfs_cat(hash: String, api_multiaddr: String, timeout_sec: u64) -> IpfsCatResult;

    #[link_name = "get_peer_id"]
    pub fn ipfs_get_peer_id(local_multiaddr: String, timeout_sec: u64) -> IpfsGetPeerIdResult;
}
