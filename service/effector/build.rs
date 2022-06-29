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
use marine_rs_sdk_test::generate_marine_test_env;
use marine_rs_sdk_test::ServiceDescription;
fn main() {
    let services = vec![(
        "ipfs_put".to_string(),
        ServiceDescription {
            config_path: "tests/Config_put.toml".to_string(),
            modules_dir: Some("../artifacts".to_string()),
        },
    ),
    (
        "ipfs_error".to_string(),
        ServiceDescription {
            config_path: "tests/Config_error.toml".to_string(),
            modules_dir: Some("../artifacts".to_string()),
        },
    )
    ];

    let target = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target != "wasm32" {
        generate_marine_test_env(services, "marine_test_env.rs", file!());
    }

    println!("cargo:rerun-if-changed=tests.rs");

}
