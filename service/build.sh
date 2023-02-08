#!/usr/bin/env bash
set -o errexit -o nounset -o pipefail

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"

# This script builds all subprojects and puts all created Wasm modules in one dir
cd effector
marine build --release

cd ../pure
marine build --release

cd ..
mkdir -p artifacts
rm -f artifacts/*.wasm
cp target/wasm32-wasi/release/ipfs_effector.wasm artifacts/
cp target/wasm32-wasi/release/ipfs_pure.wasm artifacts/
marine aqua artifacts/ipfs_pure.wasm -s Ipfs -i aqua-ipfs >../aqua/ipfs.aqua
