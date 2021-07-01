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

import { put, get_from } from './ipfs-api';

import {createClient, registerServiceFunction, setLogLevel, FluenceClient} from "@fluencelabs/fluence";
import {krasnodar, Node} from "@fluencelabs/fluence-network-environment";
const { create, globSource, urlSource } = require('ipfs-http-client');
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

async function main() {
    // setLogLevel('DEBUG');
    const fluence = await createClient(local[1]);
    console.log("created a fluence client %s with relay %s", fluence.selfPeerId, fluence.relayPeerId);
    
    let ipfsMultiaddr = 'http://127.0.0.1:5001';
    const ipfs = create(ipfsMultiaddr);
    console.log("created ipfs client");

    await ipfs.id();
    console.log("connected to ipfs");

    let source = urlSource('https://miro.medium.com/max/1172/0*4HuRgZhewW30VU0x.png');
    const file = await ipfs.add(source);
    console.log("uploaded file:", file);

    let files = await ipfs.get(file.cid);
    for await (const file of files) {
        const content = uint8ArrayConcat(await all(file.content));
        console.log("downloaded file of length ", content.length);
    }

    console.log("file hash: ", file.cid.value);
    let getResult = await get_from(fluence, local[2].peerId, file.cid.value, ipfsMultiaddr, { ttl: 10000 });
    console.log("Ipfs.get", getResult);

    let putResult = await put(fluence, local[2].peerId, { ttl: 10000 });
    console.log("Ipfs.put", putResult);

    return;
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });

