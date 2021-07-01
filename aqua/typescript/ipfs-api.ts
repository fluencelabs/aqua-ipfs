/**
 *
 * This file is auto-generated. Do not edit manually: changes may be erased.
 * Generated by Aqua compiler: https://github.com/fluencelabs/aqua/. 
 * If you find any bugs, please write an issue on GitHub: https://github.com/fluencelabs/aqua/issues
 * Aqua version: 0.1.8-160
 *
 */
import { FluenceClient, PeerIdB58 } from '@fluencelabs/fluence';
import { RequestFlowBuilder } from '@fluencelabs/fluence/dist/api.unstable';
import { RequestFlow } from '@fluencelabs/fluence/dist/internal/RequestFlow';



export async function put(client: FluenceClient, node: string, path: string, config?: {ttl?: number}): Promise<{result:string;success:boolean}> {
    let request: RequestFlow;
    const promise = new Promise<{result:string;success:boolean}>((resolve, reject) => {
        request = new RequestFlowBuilder()
            .disableInjections()
            .withTTL(config?.ttl || 5000)
            .withRawScript(
                `
(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (seq
       (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
       (call %init_peer_id% ("getDataSrv" "node") [] node)
      )
      (call %init_peer_id% ("getDataSrv" "path") [] path)
     )
     (call -relay- ("op" "noop") [])
    )
    (xor
     (call node ("ipfs-adapter" "put") [path] result)
     (seq
      (call -relay- ("op" "noop") [])
      (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
     )
    )
   )
   (call -relay- ("op" "noop") [])
  )
  (xor
   (call %init_peer_id% ("callbackSrv" "response") [result])
   (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
  )
 )
 (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
)

            `,
            )
            .configHandler((h) => {
                h.on('getDataSrv', '-relay-', () => {
                    return client.relayPeerId!;
                });
                h.on('getDataSrv', 'node', () => {return node;});
h.on('getDataSrv', 'path', () => {return path;});
                h.onEvent('callbackSrv', 'response', (args) => {
  const [res] = args;
  resolve(res);
});

                h.onEvent('errorHandlingSrv', 'error', (args) => {
                    // assuming error is the single argument
                    const [err] = args;
                    reject(err);
                });
            })
            .handleScriptError(reject)
            .handleTimeout(() => {
                reject('Request timed out for put');
            })
            .build();
    });
    await client.initiateFlow(request!);
    return promise;
}
      


export async function get_from(client: FluenceClient, node: string, hash: string, from: string, config?: {ttl?: number}): Promise<{result:string;success:boolean}> {
    let request: RequestFlow;
    const promise = new Promise<{result:string;success:boolean}>((resolve, reject) => {
        request = new RequestFlowBuilder()
            .disableInjections()
            .withTTL(config?.ttl || 5000)
            .withRawScript(
                `
(xor
 (seq
  (seq
   (seq
    (seq
     (seq
      (seq
       (seq
        (call %init_peer_id% ("getDataSrv" "-relay-") [] -relay-)
        (call %init_peer_id% ("getDataSrv" "node") [] node)
       )
       (call %init_peer_id% ("getDataSrv" "hash") [] hash)
      )
      (call %init_peer_id% ("getDataSrv" "from") [] from)
     )
     (call -relay- ("op" "noop") [])
    )
    (xor
     (call node ("ipfs-adapter" "get_from") [hash from] result)
     (seq
      (call -relay- ("op" "noop") [])
      (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 1])
     )
    )
   )
   (call -relay- ("op" "noop") [])
  )
  (xor
   (call %init_peer_id% ("callbackSrv" "response") [result])
   (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 2])
  )
 )
 (call %init_peer_id% ("errorHandlingSrv" "error") [%last_error% 3])
)

            `,
            )
            .configHandler((h) => {
                h.on('getDataSrv', '-relay-', () => {
                    return client.relayPeerId!;
                });
                h.on('getDataSrv', 'node', () => {return node;});
h.on('getDataSrv', 'hash', () => {return hash;});
h.on('getDataSrv', 'from', () => {return from;});
                h.onEvent('callbackSrv', 'response', (args) => {
  const [res] = args;
  resolve(res);
});

                h.onEvent('errorHandlingSrv', 'error', (args) => {
                    // assuming error is the single argument
                    const [err] = args;
                    reject(err);
                });
            })
            .handleScriptError(reject)
            .handleTimeout(() => {
                reject('Request timed out for get_from');
            })
            .build();
    });
    await client.initiateFlow(request!);
    return promise;
}
      