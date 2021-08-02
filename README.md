# Aqua IPFS bindings
Native IPFS integration to Aqua language. Orchestrate IPFS file transfer with Aqua scripts.

## Documentation
See [Aqua Book](https://fluence.dev/aqua-book/libraries/aqua-ipfs).

## How to use it in TypeScript
1. Add the following to your dependencies
   - `@fluencelabs/aqua-ipfs`
   - `@fluencelabs/fluence` 
   - `@fluencelabs/fluence-network-environment`

2. Import and call
```typescript
import { get_and_cache } from '@fluencelabs/aqua-ipfs';
import { createClient } from "@fluencelabs/fluence";
import { krasnodar } from "@fluencelabs/fluence-network-environment";

// connect to the Fluence network
const fluence = await createClient(krasnodar[1]);
// get some file's or dir's IPFS CID
let cid = "Qm...";
let ipfsMultiaddr = "/ip4/x.x.x.x/tcp/5001/"
// And cache it on the IPFS node running along the Fluence node we've connected to
let localCID = await get_and_cache(fluence, fluence.relayPeerId, cid, ipfs, { ttl: 10000 });
cid === localCID // true
```

## Directory structure
- `aqua` Aqua API of IPFS-adapter, go to its [README](/aqua/README.md)
- `service` contains Rust service that implements all the IPFS calls by using `ipfs` cli mounted binary
- `example` A simple example of how to use ipfs adapter from TypeScript
- `local-network` contains Docker Compose YAML config to run a local Fluence network of 3 nodes

# Contribution
Contributions are welcome!

`ipfs-adapter` integrates with IPFS by using `ipfs` CLI, so it's possible to expose virtually any IPFS API to Aqua. Feel free to open an issue or contribute APIs and patterns you find useful.
