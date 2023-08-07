import delegator
import random
import json
import os
import tempfile
import inspect
from config import get_local

delegator.run("npx fluence dep npm i", block=True)
default_peers = json.loads(delegator.run(f"node ./getDefaultPeers.js", block=True).out)


def get_relays():
    env = os.environ.get("FLUENCE_ENV")
    if env == "local":
        peers = get_local()
    else:
        if env is None:
            env = "testnet"
        peers = [peer["multiaddr"] for peer in default_peers[env]]

    assert len(peers) != 0, "No relays found"
    return peers


relays = get_relays()
peer_ids = [relay.split("/")[-1] for relay in relays]


def get_random_list_item(ar):
    return ar[random.randint(0, len(ar) - 1)]


def get_random_relay():
    return get_random_list_item(relays)


def get_random_peer_id():
    return get_random_list_item(peer_ids)


def run_aqua(func, args, relay=get_random_relay()):
    # "a" : arg1, "b" : arg2 .....
    data = {chr(97 + i): arg for (i, arg) in enumerate(args)}
    call = f"{func}(" + ", ".join([chr(97 + i) for i in range(0, len(args))]) + ")"
    # inspect.stack method inspects the current execution stack as the name suggests
    # it's possible to infer that the minus 39th element of the stack always contains info
    # about the test function that is currently running. The third element is the function's name
    try:
        test_name = inspect.stack()[-39][3]
    except:
        # when running one test at a time, the stack is shorter so we need to use a different index
        test_name = inspect.stack()[-32][3]

    command = f"npx fluence run -k {test_name} --relay {relay} -f '{call}' --data '{json.dumps(data)}' --import 'node_modules' --quiet --particle-id"
    print(command)
    c = delegator.run(command, block=True)
    lines = c.out.splitlines()
    particle_id = lines[0] if len(lines) != 0 else ""

    if len(c.err.strip()) != 0:
        print(f"{particle_id}\n{c.err}")

    result = "\n".join(lines[1:])

    try:
        result = json.loads(result)
        print(result)
        return result
    except:
        print(result)
        return result


def put_dag(api, data):
    with tempfile.NamedTemporaryFile() as tmp:
        tmp.write(data.encode())
        tmp.flush()
        c = delegator.run(f"ipfs --api {api} dag put {tmp.name}", block=True)
        if len(c.err.strip()) != 0:
            print(f"dag put error: {c.err}")
        return c.out.strip()


def test_put_get_dag():
    dag = """
{
  "name": "ipfs_pure",
  "dependencies": [
    {
      "/": "bafkreibrmbfv7ab4dokljanddvq5nah66cdody2biusqgqlfqduwn4avdi"
    },
    {
      "/": "bafybeicovoqrw75mskauoaknyxpla7xadtv5m2lphlrtjdj7dlacm6wawi"
    }
  ]
}
"""
    relay_multiaddr = get_random_relay()
    relay_peer_id = relay_multiaddr.split("/")[-1]
    ext_api_endpoint = run_aqua(
        "get_external_api_multiaddr", [relay_peer_id], relay=relay_multiaddr
    )
    assert ext_api_endpoint["success"] == True
    cid = put_dag(ext_api_endpoint["multiaddr"], dag)
    assert cid != ""

    blueprint = run_aqua(
        "load_blueprint_from_vault", [relay_peer_id, cid], relay=relay_multiaddr
    )

    assert blueprint["name"] == "ipfs_pure"
    assert (
        blueprint["dependencies"][0]["/"]
        == "bafkreibrmbfv7ab4dokljanddvq5nah66cdody2biusqgqlfqduwn4avdi"
    )
    assert (
        blueprint["dependencies"][1]["/"]
        == "bafybeicovoqrw75mskauoaknyxpla7xadtv5m2lphlrtjdj7dlacm6wawi"
    )
