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

    #[marine_test(config_path = "Config.toml", modules_dir = "../../artifacts")]
    fn invalid_multiaddr() {
        let invalid_multiaddr = "invalid_multiaddr".to_string();
        let result = ipfs_pure.set_local_api_multiaddr(invalid_multiaddr.clone());
        assert!(!result.success);
        assert_eq!(format!("invalid multiaddr: {}", invalid_multiaddr), result.error);
    }

    #[marine_test(config_path = "Config.toml", modules_dir = "../../artifacts")]
    fn set_get_external_api_multiaddr() {
        let multiaddr = "/ip4/127.0.0.1/tcp/9992";
        let result = ipfs_pure.set_external_api_multiaddr(multiaddr.to_string());
        assert!(result.success);

        let peer_id = ipfs_effector.get_peer_id("/ip4/127.0.0.1/tcp/5001".to_string(), 0).peer_id;

        let result = ipfs_pure.get_external_api_multiaddr();
        assert!(result.success);
        assert_eq!(format!("{}/p2p/{}", multiaddr, peer_id), result.multiaddr);
    }
}
