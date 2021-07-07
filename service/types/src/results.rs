/*
 * Copyright 2021 Fluence Labs Limited
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

use marine_rs_sdk::marine;
use eyre::Result;

#[marine]
pub struct IpfsResult {
    pub success: bool,
    pub error: String,
}

impl From<Result<()>> for IpfsResult {
    fn from(result: Result<()>) -> Self {
        match result {
            Ok(_) => Self { success: true, error: "".to_string() },
            Err(err) => Self { success: false, error: err.to_string() }
        }
    }
}

#[marine]
pub struct IpfsGetFromResult {
    pub success: bool,
    pub error: String,
    pub path: String,
}

impl From<Result<String>> for IpfsGetFromResult {
    fn from(result: Result<String>) -> Self {
        match result {
            Ok(path) => Self { success: true, error: "".to_string(), path },
            Err(err) => Self { success: false, error: err.to_string(), path: "".to_string() }
        }
    }
}

#[marine]
pub struct IpfsPutResult {
    pub success: bool,
    pub error: String,
    pub hash: String,
}

impl From<Result<String>> for IpfsPutResult {
    fn from(result: Result<String>) -> Self {
        match result {
            Ok(hash) => Self { success: true, error: "".to_string(), hash },
            Err(err) => Self { success: false, error: err.to_string(), hash: "".to_string() }
        }
    }
}

#[marine]
pub struct IpfsGetPeerIdResult {
    pub success: bool,
    pub error: String,
    pub peer_id: String,
}

impl From<Result<String>> for IpfsGetPeerIdResult {
    fn from(result: Result<String>) -> Self {
        match result {
            Ok(peer_id) => Self { success: true, error: "".to_string(), peer_id },
            Err(err) => Self { success: false, error: err.to_string(), peer_id: "".to_string() }
        }
    }
}

#[marine]
pub struct IpfsMultiaddrResult {
    pub success: bool,
    pub error: String,
    pub multiaddr: String,
}

impl From<Result<String>> for IpfsMultiaddrResult {
    fn from(result: Result<String>) -> Self {
        match result {
            Ok(multiaddr) => Self { success: true, error: "".to_string(), multiaddr },
            Err(err) => Self { success: false, error: err.to_string(), multiaddr: "".to_string() }
        }
    }
}
