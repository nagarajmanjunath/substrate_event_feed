# Substrate Node Template

[![Try on playground](https://img.shields.io/badge/Playground-Node_Template-brightgreen?logo=Parity%20Substrate)](https://docs.substrate.io/playground/) [![Matrix](https://img.shields.io/matrix/substrate-technical:matrix.org)](https://matrix.to/#/#substrate-technical:matrix.org)

A fresh FRAME-based [Substrate](https://www.substrate.io/) node, ready for hacking :rocket:

### Task

Write a simple pallet for an oracle event feed

- An event is arbitrary length bytes

- Only a single authorised account may post an event

- The pallet should store the last 1 hour of events

- Notes down any known security issues, or things to be improved if you are running out of time

### Notice:

Instead of 1 hour, I'm setting it up to 100 seconds, so that we can see the event removal update in a shorter time. If anyone wants to increase the time, please go to /pallets/pallet_oracle_feed/src/lib.rs file

- And check line no: 85

  ```sh
  		let time: u64 = T::TimeProvider::now().as_secs().saturating_sub(100) ;

  ```

- Change that 100 into the number of seconds you want to remove the event

### Youtube video of overall flow

Before moving to the codebase. I strongly recommend to watch this youtube video(created by me), to get a brief idea about the entire flow.

```
https://youtu.be/WFVGPdlJe1A

```

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

Once the build is completed successfully, below command will start the single-node development chain with non-persistent state:

```bash
./target/release/node-template --dev

```

### Test

The below command will execute all the test cases defined in /pallets/pallet_oracle_feed/src/test.rs file

```sh
cargo test -p pallet_oracle_feed
```

### Single-Node Development Chain

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/substrate-developer-hub/substrate-node-template/blob/main/node/src/chain_spec.rs#L49).
> At the same time the following accounts will be pre-funded:
>
> - Alice (Sudo or Root Origin by default) -
> - Bob
> - Alice//stash
> - Bob//stash

> Above code will run the blockchain node in local machine port : 9944

```
ws://127.0.0.1:9944

```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.

### Things to be improved

- Benchmarking should be added
- More testcase need to be covered
