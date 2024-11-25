# Soroban Stellar

https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup

Install Rust on the workstation.

Open PowerShell in administrator mode.

```

rustup target add wasm32-unknown-unknown

winget install --id Stellar.StellarCLI --version 22.0.0

stellar keys generate --global alice --network testnet --fund

stellar keys address alice

stellar contract --help

stellar contract init soroban-hello-world

cd soroban-hello-world

cargo test

stellar contract build

stellar contract optimize --wasm target/wasm32-unknown-unknown/release/hello_world.wasm

stellar contract deploy `
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm `
  --source alice `
  --network testnet

stellar contract invoke `
  --id <enter id from previous step> `
  --source alice `
  --network testnet `
  -- `
  hello `
  --to RPC

```

Frontend Dapp:

https://developers.stellar.org/docs/build/apps/dapp-frontend

https://astro.build/

https://github.com/stellar/soroban-template-astro

https://www.freighter.app/

---

To add the required crates (packages), for example:

```

cargo add soroban-token-sdk

```

To test, deploy to testnet, local or okashi dev environments might not always work as expected.

Playground (online): https://okashi.dev/playground/

https://crates.io/search?q=soroban_sdk

https://github.com/stellar/soroban-examples/tree/v21.6.0

https://github.com/stellar/soroban-examples/blob/main/token/src/metadata.rs

https://github.com/stellar/rs-soroban-sdk/blob/main/soroban-sdk/src/token.rs

https://docs.rs/soroban-sdk/latest/soroban_sdk/

Standards example: https://github.com/script3/sep-41-token/tree/main/sep-41/src

https://jamesbachini.com/building-rust-smart-contracts-on-stellar-soroban/ and https://www.youtube.com/watch?v=BxxRlYkhwPs

https://github.com/net2devcrypto/ and https://www.youtube.com/watch?v=1YttXux6yVY

https://developers.stellar.org/docs/build/guides/storage/choosing-the-right-storage

https://testnet.stellarchain.io/accounts/...address

https://stellar.expert/explorer/public

https://stellar.org/blog/developers/evm-to-soroban-understanding-data-types-or-solidity-to-rust-series-pt-2

https://developers.stellar.org/docs/build/smart-contracts/example-contracts/events

https://developers.stellar.org/docs/build/guides/events/publish

OpenZeppelin standards: https://github.com/OpenZeppelin/openzeppelin-contracts/tree/master/contracts/interfaces
