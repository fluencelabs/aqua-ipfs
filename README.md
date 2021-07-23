# Fluence IPFS adapter

## How to use it in TypeScript
1. Add the following to your dependencies
   - `@fluencelabs/aqua-ipfs`
   - `@fluencelabs/fluence` 
   - `@fluencelabs/fluence-network-environment`
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
- `aqua` Aqua API of IPFS-adapter, go to its [README](/aqua/README.md)
- `service` contains Rust service that implements all the IPFS calls by using `ipfs` cli mounted binary
- `example` A simple example of how to use ipfs adapter from TypeScript
- `local-network` contains Docker Compose YAML config to run a local Fluence network of 3 nodes
