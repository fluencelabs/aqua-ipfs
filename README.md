# Fluence IPFS adapter

## How to use it in TypeScript
1. Add the following to your dependencies
   1. `@fluencelabs/aqua-ipfs`
   2. `@fluencelabs/fluence` 
   3. `@fluencelabs/fluence-network-environment`
2. Import and call
```typescript
import { put, get_from } from '@fluencelabs/aqua-ipfs';
import { createClient } from "@fluencelabs/fluence";
import { krasnodar } from "@fluencelabs/fluence-network-environment";

const fluence = await createClient(krasnodar[1]);
let cid = "Qm...";
let ipfs = "/ip4/x.x.x.x/tcp/5001/"
let getResult = await get_from(fluence, fluence.relayPeerId, cid, ipfs, { ttl: 10000 });
```

## Directory structure
- `service` contains Rust service that implements all the IPFS calls by using `ipfs` cli mounted binary
- `local-network` contains Docker Compose YAML config to run a local Fluence network of 3 nodes
- `aqua` Aqua API of IPFS-adapter, go to its [README](/aqua/README.md)
- `example` A simple example of how to use ipfs adapter from TypeScript
