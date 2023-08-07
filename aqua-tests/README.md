# Aqua-IPFS API tests

## How to run

- `npm i`
- `pip3 install -r requirements.txt`
- `pip install -U pytest`
- `pytest -n auto`

## Adding new test

Before adding new test go to the aqua-tests dir first, then run `npm run secret`
to add a new key-pair for the new test.
Name it the same way the test function will be called (e.g. `test_create_resource`)
This is required for tests to run in parallel. Key-pairs could've been generated on the fly
but it's a bit faster to not waste time on it each time the tests are run
