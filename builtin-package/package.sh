#!/usr/bin/env bash
set -o pipefail -o nounset -o errexit

# set current working directory to script directory to run script from everywhere
cd "$(dirname "$0")"
SCRIPT_DIR="$(pwd)"

(
    echo "*** copy wasm files ***"
    cd ../service
    cp artifacts/*.wasm "$SCRIPT_DIR"
)

(
    echo "*** create builtin distribution package ***"
    cd ..
    mv builtin-package aqua-ipfs
    tar --exclude="package.sh" -f aqua-ipfs.tar.gz -zcv ./aqua-ipfs
    mv aqua-ipfs builtin-package
)

echo "*** done ***"
