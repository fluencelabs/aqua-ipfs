#!/usr/bin/env bash
set -o pipefail -o nounset -o errexit

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"
PACKAGE_DIR="$(pwd)/../aqua-ipfs"

(
    rm -rf $PACKAGE_DIR
    mkdir -p $PACKAGE_DIR
)

(
    echo "*** copy wasm files ***"
    cd ../service
    cp artifacts/*.wasm "$PACKAGE_DIR"
)

(
    echo "*** copy on_start script ***"
    cp on_start.json "$PACKAGE_DIR"
    cp on_start.air "$PACKAGE_DIR"
)

PURE_CID=$(ipfs add -q --only-hash --cid-version=1 --chunker=size-262144 $PACKAGE_DIR/ipfs_pure.wasm)
EFFECTOR_CID=$(ipfs add -q --only-hash --cid-version=1 --chunker=size-262144 $PACKAGE_DIR/ipfs_effector.wasm)
mv $PACKAGE_DIR/ipfs_pure.wasm "$PACKAGE_DIR"/"$PURE_CID".wasm
mv $PACKAGE_DIR/ipfs_effector.wasm "$PACKAGE_DIR"/"$EFFECTOR_CID".wasm
cp ipfs_pure_config.json "$PACKAGE_DIR"/"$PURE_CID"_config.json
cp ipfs_effector_config.json "$PACKAGE_DIR"/"$EFFECTOR_CID"_config.json

# write blueprint.json
echo "{}" | jq --arg pure_cid "$PURE_CID" --arg effector_cid "$EFFECTOR_CID" '{"name": "ipfs_pure", "dependencies":[{"/":$effector_cid},{"/":$pure_cid}]}' > "$PACKAGE_DIR/blueprint.json"

(
    echo "*** create builtin distribution package ***"
    cd ..
    tar -f aqua-ipfs.tar.gz -zcvv ./aqua-ipfs
)

echo "*** done ***"
