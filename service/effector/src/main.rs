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

#![feature(try_blocks)]

#[cfg(target_arch = "wasm32")]
mod effector;

/*
   _initialize function that calls __wasm_call_ctors is required to mitigade memory leak
   that is described in https://github.com/WebAssembly/wasi-libc/issues/298

   In short, without this code rust wraps every export function
   with __wasm_call_ctors/__wasm_call_dtors calls. This causes memory leaks. When compiler sees
   an explicit call to __wasm_call_ctors in _initialize function, it disables export wrapping.

   TODO: remove when updating to marine-rs-sdk with fix
*/
#[cfg(target_arch = "wasm32")]
extern "C" {
    pub fn __wasm_call_ctors();
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
fn _initialize() {
    unsafe {
        __wasm_call_ctors();
    }
}

#[cfg(target_arch = "wasm32")]
pub fn main() {
    _initialize(); // As __wasm_call_ctors still does necessary work, we call it at the start of the module
    effector::main()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {}
