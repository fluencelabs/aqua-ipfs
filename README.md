# Aqua IPFS bindings
Native IPFS integration to Aqua language. Orchestrate IPFS file transfer with Aqua scripts.

## Quick installation with @fluencelabs/cli
```
fluence dep npm i @fluencelabs/aqua-ipfs
```

## Quick Aqua example

```
import "@fluencelabs/aqua-ipfs/ipfs-api.aqua"
import "@fluencelabs/aqua-lib/builtin.aqua"

const PERIOD_SEC = 10

func install_scheduled_script_from_ipfs_to_peer(from: PeerId, to: PeerId, cid: string) -> ?string:
  script_id: ?string

  ipfs_maddr <- get_external_api_multiaddr(from)
  if ipfs_maddr.success:
    get <- get_from(to, cid, ipfs_maddr.multiaddr)
    if get.success:
      script_id <- Script.add_from_vault(get.path, ?[PERIOD_SEC])

  <- script_id
```

## Documentation
See [aqua-ipfs in Aqua Book](https://fluence.dev/docs/aqua-book/libraries/aqua-ipfs).

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
