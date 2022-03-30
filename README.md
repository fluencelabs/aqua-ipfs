# Aqua IPFS bindings
Native IPFS integration to Aqua language. Orchestrate IPFS file transfer with Aqua scripts.

## Documentation
See [Aqua Book](https://fluence.dev/aqua-book/libraries/aqua-ipfs).

## How to use it in TypeScript
There's a simple example in [example](/example/index.ts)

## Directory structure
- `aqua` Aqua API of AquaIPFS. See [aqua/README](/aqua/README.md)
- `service` contains Rust service that implements all the IPFS calls by using `ipfs` cli mounted binary
- `example` A simple example of how to use ipfs adapter from TypeScript
- `local-network` contains Docker Compose YAML config to run a local Fluence network of 3 nodes

# Contribution
Contributions are welcome!

`aqua-ipfs` integrates with IPFS by using `ipfs` CLI, so it's possible to expose virtually any IPFS API to Aqua. Feel free to open an issue or contribute APIs and patterns you find useful.
