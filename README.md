# Fluence IPFS adapter
- `service` contains Rust service that implements all the IPFS calls by using `ipfs` cli mounted binary
- `local-network` contains Docker Compose YAML config to run a local Fluence network of 3 nodes
- `aqua` Aqua API of IPFS-adapter, read below

## Aqua API of IPFS-adapter
Public API is defined in [ipfs-api.aqua](./ipfs-api.aqua).

ipfs-adapter service API is defined in [ipfs.aqua](./ipfs.aqua).

### How to use Aqua API in TypeScript
There's a simple example in [demo.ts](typescript/demo.ts)

## Run index.ts example on TestNet
While in `aqua` directory, `npm start` will run `index.ts` against Fluence TestNet

### Run demo.ts example locally
1. Go to `local-network` 
2. Run `docker compose up -d` to start Fluence cluster of 3 nodes
3. Go back to `../aqua`
4. Run `npm run start:local`

## How to update ipfs.aqua from ipfs_pure.wasm
While in `aqua` directory, run `npm run generate-aqua`
