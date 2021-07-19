# Aqua API of IPFS-adapter
Public API is defined in [ipfs-api.aqua](./ipfs-api.aqua).

ipfs-adapter service API is defined in [ipfs.aqua](./ipfs.aqua).

## How to use in TypeScript
Take a look at [index.ts](typescript/index.ts)

### Run index.ts example on TestNet
`npm start` will run `index.ts` against Fluence TestNet

### Run index.ts example locally
1. Go to `../local-network` 
2. Run `docker compose up -d` to start Fluence cluster of 3 nodes
3. Go back to `../aqua`
4. Run `npm run start:local`

## How to update ipfs.aqua from ipfs_pure.wasm
Run `npm run generate-aqua`
