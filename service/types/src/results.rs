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
    pub result: String,
}

impl From<Result<String>> for IpfsResult {
    fn from(result: Result<String>) -> Self {
        match result {
            Ok(result) => Self { success: true, result },
            Err(err) => Self { success: false, result: err.to_string() }
        }
    }
}

impl From<Result<()>> for IpfsResult {
    fn from(result: Result<()>) -> Self {
        match result {
            Ok(_) => Self { success: true, result: "".to_string() },
            Err(err) => Self { success: false, result: err.to_string() }
        }
    }
}
