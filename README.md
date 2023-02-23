# Aqua IPFS bindings

[![npm](https://img.shields.io/npm/v/@fluencelabs/aqua-ipfs)](https://www.npmjs.com/package/@fluencelabs/aqua-ipfs)

Aqua-ipfs is a native [IPFS](https://ipfs.tech/) integration to [Aqua](https://fluence.dev/docs/aqua-book/introduction) language. It lets one call the API of an IPFS daemon, e.g., to transfer files between peers & services or to orchestrate IPFS nodes.

## Quick Installation and Usage

First, make sure you have [Fluence CLI](https://github.com/fluencelabs/fluence-cli) installed. After this, installation is as simple as:

```
fluence dep npm i @fluencelabs/aqua-ipfs
```

Next, run:

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

To run it in Marine REPL (mrepl), one needs to install `mrepl` via `cargo install mrepl`
first. After that:

```
cd service
mrepl
new Config.toml
call ipfs_pure set_local_api_multiaddr ["/ip4/134.209.186.43/tcp/5001/p2p/12D3KooWEhCqQ9NBnmtSfNeXSNfhgccmH86xodkCUxZNEXab6pkw"]

call ipfs_pure set_external_api_multiaddr ["/ip4/134.209.186.43/tcp/5001/p2p/12D3KooWEhCqQ9NBnmtSfNeXSNfhgccmH86xodkCUxZNEXab6pkw"]

call ipfs_pure get_from ["QmfBRabun4FpaHV4wVXtnqtopUTro93XJHiWhNZscViCaq", "/ip4/134.209.186.43/tcp/5001/p2p/12D3KooWEhCqQ9NBnmtSfNeXSNfhgccmH86xodkCUxZNEXab6pkw"]
```

You can use `interface` and `help` inside `mrepl` to further discover what's possible.

A simple example of using `aqua-ipfs` in TypeScript is available [here](./example/index.ts).


## Documentation

Comprehensive documentation including API and usage examples can be found in [Aqua Book](https://fluence.dev/docs/aqua-book/libraries/aqua-ipfs).


## Repository Structure

- [**aqua**](./aqua) is Aqua API of Aqua-ipfs. See its [README](./aqua/README.md) for details.
- [**builtin-package**](./builtin-package) Files necessary to use Aqua-ipfs on peers
- [**example**](./example) A simple example of how to use IPFS adapter from TypeScript
- [**local-network**](./local-network) contains Docker Compose YAML config to run a local Fluence network of 3 nodes
- [**service**](./service) contains Rust service that implements all IPFS calls by using `ipfs` CLI mounted binary


## Support

Please, file an [issue](https://github.com/fluencelabs/aqua-ipfs/issues) if you find a bug. You can also contact us at [Discord](https://discord.com/invite/5qSnPZKh7u) or [Telegram](https://t.me/fluence_project).  We will do our best to resolve the issue ASAP.


## Contributing

Any interested person is welcome to contribute to the project. Please, make sure you read and follow some basic [rules](./CONTRIBUTING.md).


## License

All software code is copyright (c) Fluence Labs, Inc. under the [Apache-2.0](./LICENSE) license.

