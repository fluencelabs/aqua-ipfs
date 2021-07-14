/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { put, get_from, set_timeout, get_external_swarm_multiaddr, get_external_api_multiaddr } from './ipfs-api';

import {createClient, setLogLevel} from "@fluencelabs/fluence";
import {stage, krasnodar, Node, testNet} from "@fluencelabs/fluence-network-environment";
const { create, globSource, urlSource, CID } = require('ipfs-http-client');
const all = require('it-all');
const uint8ArrayConcat = require('uint8arrays/concat')


let local: Node[] = [
    {
        peerId: "12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK",
        multiaddr: "/ip4/127.0.0.1/tcp/9990/ws/p2p/12D3KooWHBG9oaVx4i3vi6c1rSBUm7MLBmyGmmbHoZ23pmjDCnvK"
    },
    {
        peerId: "12D3KooWRABanQHUn28dxavN9ZS1zZghqoZVAYtFpoN7FdtoGTFv",
        multiaddr: "/ip4/127.0.0.1/tcp/9991/ws/p2p/12D3KooWRABanQHUn28dxavN9ZS1zZghqoZVAYtFpoN7FdtoGTFv"
    },
    {
        peerId: "12D3KooWFpQ7LHxcC9FEBUh3k4nSCC12jBhijJv3gJbi7wsNYzJ5",
        multiaddr: "/ip4/127.0.0.1/tcp/9992/ws/p2p/12D3KooWFpQ7LHxcC9FEBUh3k4nSCC12jBhijJv3gJbi7wsNYzJ5"
    },
];

async function provideFile(url: string, host: Node): Promise<{ file: typeof CID, swarmAddr: string, rpcAddr: string }> {
    const provider = await createClient(host);

    var swarmAddr;
    var result = await get_external_swarm_multiaddr(provider, provider.relayPeerId!);
    if (result.success) {
        swarmAddr = result.multiaddr;
    } else {
        console.error("Failed to retrieve external swarm multiaddr from %s: ", provider.relayPeerId);
        throw result.error;
    }

    var rpcAddr;
    var result = await get_external_api_multiaddr(provider, provider.relayPeerId!);
    if (result.success) {
        rpcAddr = result.multiaddr;
    } else {
        console.error("Failed to retrieve external api multiaddr from %s: ", provider.relayPeerId);
        throw result.error;
    }

    const ipfs = create(rpcAddr);
    console.log("📗 created ipfs client to %s", rpcAddr);

    await ipfs.id();
    console.log("📗 connected to ipfs");

    let source = urlSource('https://images.adsttc.com/media/images/5ecd/d4ac/b357/65c6/7300/009d/large_jpg/02C.jpg?1590547607');
    const file = await ipfs.add(source);
    console.log("📗 uploaded file:", file);

    let files = await ipfs.get(file.cid);
    for await (const file of files) {
        const content = uint8ArrayConcat(await all(file.content));
        console.log("📗 downloaded file of length ", content.length);
    }

    return { file, swarmAddr, rpcAddr };
}

async function main(environment: Node[]) {
    // setLogLevel('DEBUG');
    
    let providerHost = environment[0];
    console.log("uploading file from URL to node %s", providerHost.multiaddr);
    let url = 'https://images.adsttc.com/media/images/5ecd/d4ac/b357/65c6/7300/009d/large_jpg/02C.jpg?1590547607';
    let { file, swarmAddr, rpcAddr } = await provideFile(url, providerHost);

    const fluence = await createClient(environment[1]);
    console.log("📗 created a fluence client %s with relay %s", fluence.selfPeerId, fluence.relayPeerId);

    // default IPFS timeout is 1 sec, set to 10 secs to retrieve file from remote node
    await set_timeout(fluence, environment[2].peerId, 10);

    console.log("📘 file hash: ", file.cid);
    let getResult = await get_from(fluence, environment[2].peerId, file.cid.toString(), swarmAddr, { ttl: 10000 });
    console.log("📘 Ipfs.get", getResult);

    let putResult = await put(fluence, environment[2].peerId, getResult.path, { ttl: 10000 });
    console.log("📘 Ipfs.put", putResult);

    return;
}

let args = process.argv.slice(2);
var environment: Node[];
if (args.length >= 1 && args[0] == "local") {
    environment = local;
    console.log("📘 Will connect to local nodes");
} else {
    environment = testNet;
    console.log("📘 Will connect to testNet");
}

main(environment)
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });

