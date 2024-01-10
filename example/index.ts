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
import { Fluence, stage, Relay } from "@fluencelabs/js-client";

import { put, get_from, set_timeout } from "./generated/export.js";

import { createHelia } from "helia";
import { unixfs } from "@helia/unixfs";
import all from "it-all";
import uint8ArrayConcat from "uint8arrays/concat.js";

// Multi address of the IPFS node
// we will work with in Fluence Network
const IPFS_MULTIADDR = "/dns4/ipfs.fluence.dev/tcp/5001";

/**
 * @param environment - array of network nodes (two are needed)
 * @note Pass addresses of local nodes to experiment locally
 */
async function main(environment: Relay[]) {
  const relay = environment[0];
  const node = environment[1];

  const helia = await createHelia();
  console.log("📗 Created Helia Node");

  const fs = unixfs(helia);
  console.log("📗 Created UnixFS");

  const content = "Hello, Fluence!";
  const encoder = new TextEncoder();

  const cid = await fs.addBytes(encoder.encode(content));
  console.log("📗 Uploaded content, got CID:", cid.toString());

  let stream = await fs.cat(cid);
  let data = uint8ArrayConcat(await all(stream));
  const decoder = new TextDecoder();
  console.log("📗 Retrieved content: ", decoder.decode(data));

  await Fluence.connect(relay);
  const client = Fluence.getClient();

  console.log(
    "📗 Created a Fluence Peer %s with Relay %s",
    client.getPeerId(),
    client.getRelayPeerId()
  );

  // default IPFS timeout is 1 sec, set to 10 secs to retrieve file from remote node
  await set_timeout(node.peerId, 10);
  console.log("📘 Ipfs.set_timeout");

  let getResult = await get_from(node.peerId, cid.toString(), IPFS_MULTIADDR, {
    ttl: 20000,
  });
  console.log("📘 Ipfs.get", getResult);

  let putResult = await put(node.peerId, getResult.path, {
    ttl: 20000,
  });
  console.log("📘 Ipfs.put", putResult);

  await helia.stop();
}

main(stage)
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
