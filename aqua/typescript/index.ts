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

export * from './ipfs-api';

import {createClient, registerServiceFunction, setLogLevel, FluenceClient} from "@fluencelabs/fluence";
import {krasnodar} from "@fluencelabs/fluence-network-environment";



async function main() {
    // setLogLevel('DEBUG');
    const client = await createClient(krasnodar[1]);
    console.log("created a client %s with relay %s", client.selfPeerId, client.relayPeerId);
    
    return;
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });

