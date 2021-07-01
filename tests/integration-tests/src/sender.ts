import { ApiPromise, Keyring } from '@polkadot/api'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { ISubmittableResult, AnyJson } from '@polkadot/types/types/'
import { AccountId, EventRecord } from '@polkadot/types/interfaces'
import { DispatchError, DispatchResult } from '@polkadot/types/interfaces/system'
import { KeyringPair } from '@polkadot/keyring/types'
import Debugger from 'debug'
import AsyncLock from 'async-lock'
import { assert } from 'chai'
import BN from 'bn.js'

export enum LogLevel {
  None,
  Debug,
  Verbose,
}

const nonceCacheByAccount = new Map<string, number>()

export class Sender {
  private readonly api: ApiPromise
  private static readonly asyncLock: AsyncLock = new AsyncLock()
  private readonly keyring: Keyring
  private readonly debug: Debugger.Debugger
  private logs: LogLevel = LogLevel.None
  private static instance = 0

  constructor(api: ApiPromise, keyring: Keyring, label: string) {
    this.api = api
    this.keyring = keyring
    this.debug = Debugger(`Sender:${Sender.instance++}:${label}`)
  }

  // Synchronize all sending of transactions into mempool, so we can always safely read
  // the next account nonce taking mempool into account. This is safe as long as all sending of transactions
  // from same account occurs in the same process. Returns a promise of the Extrinsic Dispatch Result ISubmittableResult.
  // The promise resolves on tx finalization (For both Dispatch success and failure)
  // The promise is rejected if transaction is rejected by node.

  public setLogLevel(level: LogLevel): void {
    this.logs = level
  }

  public async signAndSend(
    tx: SubmittableExtrinsic<'promise'>,
    account: AccountId | string
  ): Promise<ISubmittableResult> {
    const addr = this.keyring.encodeAddress(account)
    const senderKeyPair: KeyringPair = this.keyring.getPair(addr)

    let finalized: { (result: ISubmittableResult): void }
    const whenFinalized: Promise<ISubmittableResult> = new Promise(async (resolve, reject) => {
      finalized = resolve
    })

    // saved human representation of the signed tx, will be set before it is submitted.
    // On error it is logged to help in debugging.
    let sentTx: AnyJson

    const handleEvents = (result: ISubmittableResult) => {
      if (result.status.isFuture) {
        // Its virtually impossible for us to continue with tests
        // when this occurs and we don't expect the tests to handle this correctly
        // so just abort!
        console.error('Future Tx, aborting!')
        process.exit(-1)
      }

      if (!(result.status.isInBlock || result.status.isFinalized)) {
        return
      }

      const success = result.findRecord('system', 'ExtrinsicSuccess')
      const failed = result.findRecord('system', 'ExtrinsicFailed')

      // Log failed transactions
      if (this.logs === LogLevel.Debug || this.logs === LogLevel.Verbose) {
        if (failed) {
          const record = failed as EventRecord
          assert(record)
          const {
            event: { data },
          } = record
          const err = data[0] as DispatchError
          if (err.isModule) {
            try {
              const { name } = this.api.registry.findMetaError(err.asModule)
              this.debug('Dispatch Error:', name, sentTx)
            } catch (findmetaerror) {
              // example Error: findMetaError: Unable to find Error with index 0x1400/[{"index":20,"error":0}]
              // Happens for dispatchable calls that don't explicitly use `-> DispatchResult` return value even
              // if they return an error enum variant from the decl_error! macro
              this.debug('Dispatch Error (error details not found):', err.asModule.toHuman(), sentTx)
            }
          } else {
            this.debug('Dispatch Error:', err.toHuman(), sentTx)
          }
        } else {
          assert(success)
          const sudid = result.findRecord('sudo', 'Sudid')
          if (sudid) {
            const dispatchResult = sudid.event.data[0] as DispatchResult
            assert(dispatchResult)
            if (dispatchResult.isError) {
              const err = dispatchResult.asError
              if (err.isModule) {
                try {
                  const { name } = this.api.registry.findMetaError(err.asModule)
                  this.debug('Sudo Dispatch Failed', name, sentTx)
                } catch (findmetaerror) {
                  // example Error: findMetaError: Unable to find Error with index 0x1400/[{"index":20,"error":0}]
                  this.debug('Sudo Dispatch Failed (error details not found)', err.asModule.toHuman(), sentTx)
                }
              } else {
                this.debug('Sudo Dispatch Failed', err.toHuman(), sentTx)
              }
            }
          }
        }
      }

      // Always resolve irrespective of success or failure. Error handling should
      // be dealt with by caller.
      if (success || failed) finalized(result)
    }

    // We used to do this: Sender.asyncLock.acquire(`${senderKeyPair.address}` ...
    // Instead use a single lock for all calls, to force all transactions to be submitted in same order
    // of call to signAndSend. Otherwise it raises chance of race conditions.
    // It happens in rare cases and has lead some tests to fail occasionally in the past
    await Sender.asyncLock.acquire('tx-queue', async () => {
      // The node sometimes returns invalid account nonce at the exact time a new block is produced
      // For a split second the node will then not take "pending" transactions into account,
      // that's why we must partialy rely on cached nonce
      const nodeNonce = await this.api.rpc.system.accountNextIndex(senderKeyPair.address)
      const cachedNonce = nonceCacheByAccount.get(senderKeyPair.address)
      const nonce = BN.max(nodeNonce, new BN(cachedNonce || 0))
      const signedTx = tx.sign(senderKeyPair, { nonce })
      sentTx = signedTx.toHuman()
      const { method, section } = signedTx.method
      try {
        await signedTx.send(handleEvents)
        if (this.logs === LogLevel.Verbose) {
          this.debug('Submitted tx:', `${section}.${method} (nonce: ${nonce})`)
        }
        nonceCacheByAccount.set(account.toString(), nonce.toNumber() + 1)
      } catch (err) {
        if (this.logs === LogLevel.Debug || this.logs === LogLevel.Verbose) {
          this.debug(
            'Submitting tx failed:',
            sentTx,
            err,
            signedTx.method.args.map((a) => a.toHuman())
          )
        }
        throw err
      }
    })

    return whenFinalized
  }
}
