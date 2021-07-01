# Joystream

This is the main code repository for all Joystream software. In this mono-repo you will find all the software required to run a Joystream network: The Joystream full node, runtime and all reusable substrate runtime modules that make up the Joystream runtime. In addition to all front-end apps and infrastructure servers necessary for operating the network.

## Overview

The Joystream network builds on a pre-release version of [substrate v2.0](https://substrate.dev/) and adds additional
functionality to support the [various roles](https://www.joystream.org/roles) that can be entered into on the platform.

## Development Tools

The following tools are required for building, testing and contributing to this repo:

- [Rust](https://www.rust-lang.org/tools/install) toolchain - _required_
- [nodejs](https://nodejs.org/) v14.x - _required_
- [yarn classic](https://classic.yarnpkg.com/en/docs/install) package manager v1.22.x- _required_
- [docker](https://www.docker.com/get-started) and docker-compose - _optional_
- [ansible](https://www.ansible.com/) - _optional_

If you use VSCode as your code editor we recommend using the workspace [settings](devops/vscode/settings.json) for recommend eslint plugin to function properly.

After cloning the repo run the following initialization scripts:

```sh
# Install rust toolchain
./setup.sh

# Install npm package dependencies
# Also good habit to run this when switching between branches
yarn install

# run some tests
yarn cargo-checks
```

## Software

**Substrate blockchain**

- [joystream-node](./node)
- [runtime](./runtime)
- [runtime modules](./runtime-modules)

**Server Applications - infrastructure**

- [Storage Node](./storage-node) - Media Storage Infrastructure
- [Query Node](https://github.com/Joystream/joystream/tree/query_node/query-node) - _under development_
- [Discovery Node](https://github.com/Joystream/joystream/tree/init_discovery_node/discovery_node) - _under development_

**Front-end Applications**

- [Pioneer](./pioneer) - Main UI for accessing all Joystream features
- [Atlas](https://github.com/Joystream/joystream/tree/init_atlas/atlas) - Media Player- _under development_

**Tools and CLI**

- [joystream-cli](./cli) - CLI for community and governance activities

**Testing infrastructure**

- [Network integration](./tests/network-tests) - Joystream network integration testing framework

## Exploring the network with Pioneer

Pioneer is currently the main web interface to interact with the network:

Currently hosted on: https://testnet.joystream.org

You may have to disable some privacy/ad-blocker extensions in your browser for proper functionality, especially when browsing media.

You can also run a local development instance:

The HEAD of the master branch should always be used for the correct version of the applications to connect to the current testnet:

```sh
git checkout master
yarn install
yarn workspace pioneer start
```

This runs a local development web server on port 3000.

Use the link below to browse the network using the publicly hosted endpoint:
http://localhost:3000/?rpc=wss://rome-rpc-endpoint.joystream.org:9944/

## Running a local full node

You can also run your our own joystream-node:

```sh
git checkout master
WASM_BUILD_TOOLCHAIN=nightly-2020-10-06 cargo build --release
./target/release/joystream-node -- --pruning archive --chain testnets/joy-testnet-4.json
```

Wait for the node to sync to the latest block, then change pioneer settings "remote node" option to "Local Node", or follow the link below:

http://localhost:3000/?rpc=ws://localhost:9944/

Learn more about [joystream-node](node/README.md).

A step by step guide to setup a full node and validator on the Joystream testnet, can be found [here](https://github.com/Joystream/helpdesk/tree/master/roles/validators).

### Integration tests

```bash
tests/network-tests/run-tests.sh
```

### Contributing

We have lots of good first [issues](https://github.com/Joystream/joystream/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) open to help you get started on contributing code. If you are not a developer you can still make valuable contributions by testing our software and providing feedback and opening new issues.

A description of our [branching model](https://github.com/Joystream/joystream/issues/638) will help you to understand where work on different software components happens, and consequently where to direct your pull requests.

We rely on `eslint` for code quality of our JavaScript and TypeScript code and `prettier` for consistent formatting. For Rust we rely on `rustfmt` and `clippy`.

The [husky](https://www.npmjs.com/package/husky#ci-servers) npm package is used to manage the project git-hooks. This is automatically installed and setup when you run `yarn install`.

When you `git commit` and `git push` some scripts will run automatically to ensure committed code passes lint, tests, and code-style checks.

During a rebase/merge you may want to skip all hooks, you can use `HUSKY_SKIP_HOOKS` environment variable.

```
HUSKY_SKIP_HOOKS=1 git rebase ...
```

## RLS Extension in VScode or Atom Editors

If you use RLS extension in your IDE, start your editor with the `BUILD_DUMMY_WASM_BINARY=1` environment set to workaround a build issue that occurs in the IDE only.

`BUILD_DUMMY_WASM_BINARY=1 code ./joystream`

## Authors

See the list of [contributors](https://github.com/Joystream/joystream/graphs/contributors) who participated in this project.

## License

All software under this project is licensed as [GPLv3](./LICENSE) unless otherwise indicated.

## Acknowledgments

Thanks to the whole [Parity Tech](https://www.parity.io/) team for making substrate and helping in chat with tips, suggestions, tutorials and answering all our questions during development.
