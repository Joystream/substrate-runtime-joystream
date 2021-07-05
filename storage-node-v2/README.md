storage-node-v2
===============

Jostream storage subsystem.

[![oclif](https://img.shields.io/badge/cli-oclif-brightgreen.svg)](https://oclif.io)
[![Version](https://img.shields.io/npm/v/storage-node-v2.svg)](https://npmjs.org/package/storage-node-v2)
[![Downloads/week](https://img.shields.io/npm/dw/storage-node-v2.svg)](https://npmjs.org/package/storage-node-v2)
[![License](https://img.shields.io/npm/l/storage-node-v2.svg)](https://github.com/shamil-gadelshin/storage-node-v2/blob/master/package.json)

<!-- toc -->
* [Usage](#usage)
* [Commands](#commands)
<!-- tocstop -->
# Usage
<!-- usage -->
```sh-session
$ npm install -g storage-node-v2
$ storage-node COMMAND
running command...
$ storage-node (-v|--version|version)
storage-node-v2/0.1.0 darwin-x64 node-v14.17.0
$ storage-node --help [COMMAND]
USAGE
  $ storage-node COMMAND
...
```
<!-- usagestop -->
# Commands
<!-- commands -->
* [`storage-node dev:init`](#storage-node-devinit)
* [`storage-node dev:multihash`](#storage-node-devmultihash)
* [`storage-node dev:upload`](#storage-node-devupload)
* [`storage-node dev:verify-bag-id`](#storage-node-devverify-bag-id)
* [`storage-node help [COMMAND]`](#storage-node-help-command)
* [`storage-node leader:cancel-invite`](#storage-node-leadercancel-invite)
* [`storage-node leader:create-bucket`](#storage-node-leadercreate-bucket)
* [`storage-node leader:delete-bucket`](#storage-node-leaderdelete-bucket)
* [`storage-node leader:invite-operator`](#storage-node-leaderinvite-operator)
* [`storage-node leader:remove-operator`](#storage-node-leaderremove-operator)
* [`storage-node leader:set-uploading-block`](#storage-node-leaderset-uploading-block)
* [`storage-node leader:update-bag`](#storage-node-leaderupdate-bag)
* [`storage-node leader:update-bag-limit`](#storage-node-leaderupdate-bag-limit)
* [`storage-node leader:update-blacklist [FILE]`](#storage-node-leaderupdate-blacklist-file)
* [`storage-node leader:update-data-fee`](#storage-node-leaderupdate-data-fee)
* [`storage-node leader:update-dynamic-bag-policy`](#storage-node-leaderupdate-dynamic-bag-policy)
* [`storage-node leader:update-voucher-limits`](#storage-node-leaderupdate-voucher-limits)
* [`storage-node operator:accept-invitation`](#storage-node-operatoraccept-invitation)
* [`storage-node operator:set-bucket-limits`](#storage-node-operatorset-bucket-limits)
* [`storage-node operator:set-metadata`](#storage-node-operatorset-metadata)
* [`storage-node operator:update-bucket-status`](#storage-node-operatorupdate-bucket-status)
* [`storage-node server [FILE]`](#storage-node-server-file)

## `storage-node dev:init`

Initialize development environment. Sets Alice as storage working group leader.

```
USAGE
  $ storage-node dev:init

OPTIONS
  -h, --help               show CLI help
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/dev/init.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/dev/init.ts)_

## `storage-node dev:multihash`

Creates a multihash (blake3) for a file.

```
USAGE
  $ storage-node dev:multihash

OPTIONS
  -f, --file=file  (required) Path for a hashing file.
  -h, --help       show CLI help
```

_See code: [src/commands/dev/multihash.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/dev/multihash.ts)_

## `storage-node dev:upload`

Upload data object (development mode only).

```
USAGE
  $ storage-node dev:upload

OPTIONS
  -c, --cid=cid            (required) Data object IPFS content ID.
  -h, --help               show CLI help
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -s, --size=size          (required) Data object size.
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/dev/upload.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/dev/upload.ts)_

## `storage-node dev:verify-bag-id`

The command verifies bag id supported by the storage node. Requires chain connection.

```
USAGE
  $ storage-node dev:verify-bag-id

OPTIONS
  -h, --help
      show CLI help

  -i, --bagId=bagId
      (required) 
             Bag ID. Format: {bag_type}:{sub_type}:{id}.
             - Bag types: 'static', 'dynamic'
             - Sub types: 'static:council', 'static:wg', 'dynamic:member', 'dynamic:channel'
             - Id: 
               - absent for 'static:council'
               - working group name for 'static:wg'
               - integer for 'dynamic:member' and 'dynamic:channel'
             Examples:
             - static:council
             - static:wg:storage
             - dynamic:member:4

  -k, --keyfile=keyfile
      Key file for the account. Mandatory in non-dev environment.

  -m, --dev
      Use development mode

  -p, --password=password
      Key file password (optional).

  -u, --apiUrl=apiUrl
      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/dev/verify-bag-id.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/dev/verify-bag-id.ts)_

## `storage-node help [COMMAND]`

display help for storage-node

```
USAGE
  $ storage-node help [COMMAND]

ARGUMENTS
  COMMAND  command to show help for

OPTIONS
  --all  see all commands in CLI
```

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v3.0.1/src/commands/help.ts)_

## `storage-node leader:cancel-invite`

Cancel a storage bucket operator invite. Requires storage working group leader permissions.

```
USAGE
  $ storage-node leader:cancel-invite

OPTIONS
  -h, --help               show CLI help
  -i, --bucketId=bucketId  (required) Storage bucket ID
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/cancel-invite.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/cancel-invite.ts)_

## `storage-node leader:create-bucket`

Create new storage bucket. Requires storage working group leader permissions.

```
USAGE
  $ storage-node leader:create-bucket

OPTIONS
  -a, --allow              Accepts new bags
  -h, --help               show CLI help
  -i, --invited=invited    Invited storage operator ID (storage WG worker ID)
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -n, --number=number      Storage bucket max total objects number
  -p, --password=password  Key file password (optional).
  -s, --size=size          Storage bucket max total objects size
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/create-bucket.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/create-bucket.ts)_

## `storage-node leader:delete-bucket`

Delete a storage bucket. Requires storage working group leader permissions.

```
USAGE
  $ storage-node leader:delete-bucket

OPTIONS
  -h, --help               show CLI help
  -i, --bucketId=bucketId  (required) Storage bucket ID
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/delete-bucket.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/delete-bucket.ts)_

## `storage-node leader:invite-operator`

Invite a storage bucket operator. Requires storage working group leader permissions.

```
USAGE
  $ storage-node leader:invite-operator

OPTIONS
  -h, --help                   show CLI help
  -i, --bucketId=bucketId      (required) Storage bucket ID
  -k, --keyfile=keyfile        Key file for the account. Mandatory in non-dev environment.
  -m, --dev                    Use development mode
  -p, --password=password      Key file password (optional).
  -u, --apiUrl=apiUrl          Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
  -w, --operatorId=operatorId  (required) Storage bucket operator ID (storage group worker ID)
```

_See code: [src/commands/leader/invite-operator.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/invite-operator.ts)_

## `storage-node leader:remove-operator`

Remove a storage bucket operator. Requires storage working group leader permissions.

```
USAGE
  $ storage-node leader:remove-operator

OPTIONS
  -h, --help               show CLI help
  -i, --bucketId=bucketId  (required) Storage bucket ID
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/remove-operator.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/remove-operator.ts)_

## `storage-node leader:set-uploading-block`

Set global uploading block. Requires storage working group leader permissions.

```
USAGE
  $ storage-node leader:set-uploading-block

OPTIONS
  -d, --disable            Disables global uploading block.
  -e, --enable             Enables global uploading block (default).
  -h, --help               show CLI help
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/set-uploading-block.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/set-uploading-block.ts)_

## `storage-node leader:update-bag`

Add/remove a storage bucket from a bag (adds by default).

```
USAGE
  $ storage-node leader:update-bag

OPTIONS
  -b, --bucket=bucket
      (required) Storage bucket ID

  -h, --help
      show CLI help

  -i, --bagId=bagId
      (required) 
             Bag ID. Format: {bag_type}:{sub_type}:{id}.
             - Bag types: 'static', 'dynamic'
             - Sub types: 'static:council', 'static:wg', 'dynamic:member', 'dynamic:channel'
             - Id: 
               - absent for 'static:council'
               - working group name for 'static:wg'
               - integer for 'dynamic:member' and 'dynamic:channel'
             Examples:
             - static:council
             - static:wg:storage
             - dynamic:member:4

  -k, --keyfile=keyfile
      Key file for the account. Mandatory in non-dev environment.

  -m, --dev
      Use development mode

  -p, --password=password
      Key file password (optional).

  -r, --remove
      Remove a bucket from the bag

  -u, --apiUrl=apiUrl
      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/update-bag.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/update-bag.ts)_

## `storage-node leader:update-bag-limit`

Update StorageBucketsPerBagLimit variable in the Joystream node storage.

```
USAGE
  $ storage-node leader:update-bag-limit

OPTIONS
  -h, --help               show CLI help
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -l, --limit=limit        (required) New StorageBucketsPerBagLimit value
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/update-bag-limit.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/update-bag-limit.ts)_

## `storage-node leader:update-blacklist [FILE]`

describe the command here

```
USAGE
  $ storage-node leader:update-blacklist [FILE]

OPTIONS
  -f, --force
  -h, --help       show CLI help
  -n, --name=name  name to print
```

_See code: [src/commands/leader/update-blacklist.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/update-blacklist.ts)_

## `storage-node leader:update-data-fee`

Update data size fee. Requires storage working group leader permissions.

```
USAGE
  $ storage-node leader:update-data-fee

OPTIONS
  -f, --fee=fee            (required) New data size fee
  -h, --help               show CLI help
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/update-data-fee.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/update-data-fee.ts)_

## `storage-node leader:update-dynamic-bag-policy`

Update number of storage buckets used in the dynamic bag creation policy.

```
USAGE
  $ storage-node leader:update-dynamic-bag-policy

OPTIONS
  -c, --channel            Channel dynamic bag type
  -e, --member             Member dynamic bag type (default)
  -h, --help               show CLI help
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -n, --number=number      (required) New storage buckets number
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/update-dynamic-bag-policy.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/update-dynamic-bag-policy.ts)_

## `storage-node leader:update-voucher-limits`

Update VoucherMaxObjectsSizeLimit and VoucherMaxObjectsNumberLimit for the Joystream node storage.

```
USAGE
  $ storage-node leader:update-voucher-limits

OPTIONS
  -h, --help               show CLI help
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -o, --objects=objects    (required) New 'max voucher object number limit' value
  -p, --password=password  Key file password (optional).
  -s, --size=size          (required) New 'max voucher object size limit' value
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
```

_See code: [src/commands/leader/update-voucher-limits.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/leader/update-voucher-limits.ts)_

## `storage-node operator:accept-invitation`

Accept pending storage bucket invitation.

```
USAGE
  $ storage-node operator:accept-invitation

OPTIONS
  -h, --help               show CLI help
  -i, --bucketId=bucketId  (required) Storage bucket ID
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
  -w, --workerId=workerId  (required) Storage operator worker ID
```

_See code: [src/commands/operator/accept-invitation.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/operator/accept-invitation.ts)_

## `storage-node operator:set-bucket-limits`

Set VoucherObjectsSizeLimit and VoucherObjectsNumberLimit for the storage bucket.

```
USAGE
  $ storage-node operator:set-bucket-limits

OPTIONS
  -h, --help               show CLI help
  -i, --bucketId=bucketId  (required) Storage bucket ID
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -o, --objects=objects    (required) New 'voucher object number limit' value
  -p, --password=password  Key file password (optional).
  -s, --size=size          (required) New 'voucher object size limit' value
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
  -w, --workerId=workerId  (required) Storage operator worker ID
```

_See code: [src/commands/operator/set-bucket-limits.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/operator/set-bucket-limits.ts)_

## `storage-node operator:set-metadata`

Accept pending storage bucket invitation.

```
USAGE
  $ storage-node operator:set-metadata

OPTIONS
  -h, --help                   show CLI help
  -i, --bucketId=bucketId      (required) Storage bucket ID
  -k, --keyfile=keyfile        Key file for the account. Mandatory in non-dev environment.
  -m, --dev                    Use development mode
  -m, --metadata=metadata      Storage bucket operator metadata
  -p, --password=password      Key file password (optional).
  -u, --apiUrl=apiUrl          Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
  -w, --operatorId=operatorId  (required) Storage bucket operator ID (storage group worker ID)
```

_See code: [src/commands/operator/set-metadata.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/operator/set-metadata.ts)_

## `storage-node operator:update-bucket-status`

Update storage bucket status (accepting new bags).

```
USAGE
  $ storage-node operator:update-bucket-status

OPTIONS
  -d, --disable            Disables accepting new bags.
  -e, --enable             Enables accepting new bags (default).
  -h, --help               show CLI help
  -i, --bucketId=bucketId  (required) Storage bucket ID
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
  -w, --workerId=workerId  (required) Storage operator worker ID
```

_See code: [src/commands/operator/update-bucket-status.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/operator/update-bucket-status.ts)_

## `storage-node server [FILE]`

Starts the storage node server.

```
USAGE
  $ storage-node server [FILE]

OPTIONS
  -d, --uploads=uploads    (required) Data uploading directory (absolute path).
  -h, --help               show CLI help
  -k, --keyfile=keyfile    Key file for the account. Mandatory in non-dev environment.
  -m, --dev                Use development mode
  -o, --port=port          (required) Server port.
  -p, --password=password  Key file password (optional).
  -u, --apiUrl=apiUrl      Runtime API URL. Mandatory in non-dev environment. Default is ws://localhost:9944
  -w, --worker=worker      (required) Storage provider worker ID
```

_See code: [src/commands/server.ts](https://github.com/Joystream/joystream/blob/v0.1.0/src/commands/server.ts)_
<!-- commandsstop -->
