const {
  krasnodar,
  stage,
  testNet,
} = require('@fluencelabs/fluence-network-environment')

console.log(
  JSON.stringify({
    krasnodar,
    stage,
    testnet: testNet,
  }),
)
