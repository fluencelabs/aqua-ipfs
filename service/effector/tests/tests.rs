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

#[cfg(test)]
mod tests {
    use marine_rs_sdk_test::marine_test;

    #[marine_test(
        ipfs_effector(
            config_path = "Config_error.toml", 
            modules_dir = "../../artifacts"
        )
    )]
    fn connect_failed() {
        let mut effector = marine_test_env::ipfs_effector::ServiceInterface::new();
        let result = effector.connect("/ip4/127.0.0.1/tcp/5001".to_string(), "/ip4/127.0.0.1/tcp/5001".to_string(), 5u64);
        assert!(!result.success);
    }

    #[marine_test(
        ipfs_effector(
            config_path = "Config_put.toml", 
            modules_dir = "../../artifacts"
        )
    )]
    fn put_result() {
        let mut effector = marine_test_env::ipfs_effector::ServiceInterface::new();
        let result = effector.put("tmp".to_string(), "api_multiaddr".to_string(), 1);
        assert_eq!("hash", result.hash);
    }
}
