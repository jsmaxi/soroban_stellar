# Soroban Stellar

https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup

Install Rust on the workstation.

Open PowerShell in administrator mode.

```

rustup target add wasm32-unknown-unknown

winget install --id Stellar.StellarCLI --version 22.0.0

stellar keys generate --global alice --network testnet --fund

stellar keys address alice

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
