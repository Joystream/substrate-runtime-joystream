import { Keyring } from '@polkadot/keyring'
import { KeyringPair } from '@polkadot/keyring/types'
import { ApiPromise } from '@polkadot/api'
import { SubmittableExtrinsic } from '@polkadot/api/types'
import { DispatchError } from '@polkadot/types/interfaces/system'
import { TypeRegistry } from '@polkadot/types'
import { ISubmittableResult } from '@polkadot/types/types'

// TODO: Move to @joystream/js soon

export function getAlicePair(): KeyringPair {
  const keyring = new Keyring({ type: 'sr25519' })
  keyring.addFromUri('//Alice', { name: 'Alice' })
  const ALICE = keyring.getPairs()[0]

  return ALICE
}

export function getKeyFromSuri(suri: string): KeyringPair {
  const keyring = new Keyring({ type: 'sr25519' })

  // Assume a SURI, add to keyring and return keypair
  return keyring.addFromUri(suri)
}

export class ExtrinsicsHelper {
  api: ApiPromise
  noncesByAddress: Map<string, number>

  constructor(api: ApiPromise, initialNonces?: [string, number][]) {
    this.api = api
    this.noncesByAddress = new Map<string, number>(initialNonces)
  }

  private async nextNonce(address: string): Promise<number> {
    const nonce = this.noncesByAddress.get(address) || (await this.api.query.system.account(address)).nonce.toNumber()
    this.noncesByAddress.set(address, nonce + 1)

    return nonce
  }

  async sendAndCheck(
    sender: KeyringPair,
    extrinsics: SubmittableExtrinsic<'promise'>[],
    errorMessage: string
  ): Promise<ISubmittableResult[]> {
    const promises: Promise<ISubmittableResult>[] = []
    for (const tx of extrinsics) {
      const nonce = await this.nextNonce(sender.address)
      promises.push(
        new Promise<ISubmittableResult>((resolve, reject) => {
          tx.signAndSend(sender, { nonce }, (result) => {
            let txError: string | null = null
            if (result.isError) {
              txError = `Transaction failed with status: ${result.status.type}`
              reject(new Error(`${errorMessage} - ${txError}`))
            }

            if (result.status.isInBlock) {
              result.events
                .filter(({ event }) => event.section === 'system')
                .forEach(({ event }) => {
                  if (event.method === 'ExtrinsicFailed') {
                    const dispatchError = event.data[0] as DispatchError
                    let errorMsg = dispatchError.toString()
                    if (dispatchError.isModule) {
                      try {
                        // Need to assert that registry is of TypeRegistry type, since Registry intefrace
                        // seems outdated and doesn't include DispatchErrorModule as possible argument for "findMetaError"
                        const { name, documentation } = (this.api.registry as TypeRegistry).findMetaError(
                          dispatchError.asModule
                        )
                        errorMsg = `${name} (${documentation})`
                      } catch (e) {
                        // This probably means we don't have this error in the metadata
                        // In this case - continue (we'll just display dispatchError.toString())
                      }
                    }
                    reject(new Error(`${errorMessage} - Extrinsic execution error: ${errorMsg}`))
                  } else if (event.method === 'ExtrinsicSuccess') {
                    resolve(result)
                  }
                })
            }
          })
        })
      )
    }
    return await Promise.all(promises)
  }
}
