## Aqua API of IPFS-adapter
Public API is defined in [ipfs-api.aqua](/aqua/ipfs-api.aqua).

ipfs-adapter service API is defined in [ipfs.aqua](/aqua/ipfs.aqua).

### How to use Aqua API in TypeScript
There's a simple example in [demo.ts](/aqua/typescript/demo.ts)

## Run demo.ts example on TestNet
While in `aqua` directory, `npm start` will run `demo.ts` against Fluence TestNet

### Run demo.ts example locally
1. Go to `local-network` 
2. Run `docker compose up -d` to start Fluence cluster of 3 nodes
3. Go back to `../aqua`
4. Run `npm run start:local`

## How to update ipfs.aqua from ipfs_pure.wasm
While in `aqua` directory, run `npm run generate-aqua`
