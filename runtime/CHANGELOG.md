### Version 9.7.0 - Sumer - runtime upgrade - May 27 2021
- Introduced new content pallet the new content directory
- Improved data_directory pallet
  - Any storage provider to handle uploads of new content
  - Integration with new content directory
  - Introduction of quota vouchers
  - Reset data directory
- Added new working group instance for Operations

### Version 9.3.0 - Antioch - new chain - April 7 2021
- Following chain failure due to a debug in older version of substrate (v2.0.0-rc4) updated to substrate v2.0.1
- Same runtime features as babylon

### Version 7.9.0 - Babylon - runtime upgrade - December 21 2020
- Introduction of new and improved content directory

### Version 7.4.0 - Alexandria - new chain - September 21 2020
- Update to substrate v2.0.0-rc4

### Version 6.21.0 - (Constantinople) runtime upgrade B (Nicaea) - July 29 2020

- Introduction of general Working Group runtime module
- Adds a new instance of the working group module - the Storage Working Group which
  replaces the old actors module for managing the Storge Provider enrollment process
- New governance proposals to support new working groups

### Version 6.15.0 - (Constantinople) runtime upgrade A - June 2020

- Updated runtime to sort out type name clashes between the proposal discussion module
  and forum module, in preparing to roll out proposal discussion system in pioneer.
- Increased ROLE_PARAMETERS_REWARD_MAX_VALUE to 100,000

### Version 6.13.0 - (Constantinople) runtime upgrade - May 20th 2020

- New proposal system that strengthens the governance structure of the platform
- Adjusted inflation curve to better reflect a new realistic economic system for the platform

### Version 6.8.0 (Rome release - new chain) - March 9th 2020

- New versioned and permissioned content mangement system that powers a new media experience.
- Content Working Group - introduces staked content curator roles to maintain quality of content and ensure that is meets the platform's terms of service.
- Update of core substrate to pre-release of version 2.0 - [c37bb08](https://github.com/paritytech/substrate/commit/c37bb08535c49a12320af7facfd555ce05cce2e8)

### Version 5.4.0 (Acropolis) - Athens testnet update 3 - June 22n 2019

- New Forum - v1.0.0
- Discovery Service to support new Storage Provider implementation.
- Removing all previously uploaded content

### Version 5.3.0 - Athens testnet update 2 - April 27th 2019

- Fix to configure an onchain primary storage provider liason to work around incomplete storage server implementation.

### Version 5.2.0 - Athens testnet update 1 - April 17th 2019

- Fix account locking for staked roles to allow storage providers to submit transaction
- Update substrate version to `6dfc3e8b057bb00322136251a0f10305fbb1ad8f` from v1 branch

### Version 5.1.0 - Athens testnet release - April 14th 2019

- Storage Role
- Update to substrate version at `89bbb7b6d0e076f0eda736b330f5f792aa2e2991`

### Version 4 - Bug Fixes - March 4th 2019 - `9941dd`

- Allow illiquid accounts to pay transaction fees. Fixes unstaking and setting memo, by permitting extrinsics which do not require more than a transaction fee to be accepted into mempool.
- Updated Cargo dependencies to use forked substrate repo `github.com/joystream/substrate`

On-chain runtime upgrade performed with sudo `consensus::setCode()`

### Version 3 - Sparta - March 1st 2019 - `1ca4cc`

- Basic substrate node - based on substrate `1ca4cc0a16a357782bb1028bb57376594ca232a0`
- Block Authoring - only Aura (enabling GRANDPA in future release)
- Council Elections
- Council Runtime upgrade proposal
- Simple PoS validator staking
- Memo (account status message)
