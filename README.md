## Substrate_Oracle_Event_Feed

Using Rust for substrate_Oracle_Event_Feed

This project which is developed to handle the oracle event feed, which takes arbitarty bytes as input and create the event, The event will be cleared after certain time. It is build using Substrate and oracle event feed is implemented using substrate pallet.

### Rust Setup

For prerequisites and detailed build instructions please read the [Rust](./docs/rust-setup.md).

### Steps To Start the Substrate

Build the substrate node template:

```shell
 cargo build --release
```
## Running `substrate node-template`

```shell
 ./target/release/node-template --dev
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain.
[Click here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your local node template.


 
