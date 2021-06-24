import { IExtrinsic } from '@polkadot/types/types'
import { compactToU8a, stringToU8a } from '@polkadot/util'
import { blake2AsHex } from '@polkadot/util-crypto'
import BN from 'bn.js'
import fs from 'fs'
import { decodeAddress } from '@polkadot/keyring'
import { Bytes } from '@polkadot/types'
import { createType } from '@joystream/types'
import { MetadataInput } from './types'

export type AnyMessage<T> = T & {
  toJSON(): Record<string, unknown>
}

export type AnyMetadataClass<T> = {
  decode(binary: Uint8Array): AnyMessage<T>
  encode(obj: T): { finish(): Uint8Array }
  toObject(obj: AnyMessage<T>): Record<string, unknown>
}
export class Utils {
  private static LENGTH_ADDRESS = 32 + 1 // publicKey + prefix
  private static LENGTH_ERA = 2 // assuming mortals
  private static LENGTH_SIGNATURE = 64 // assuming ed25519 or sr25519
  private static LENGTH_VERSION = 1 // 0x80 & version

  public static calcTxLength = (extrinsic?: IExtrinsic | null, nonce?: BN): BN => {
    return new BN(
      Utils.LENGTH_VERSION +
        Utils.LENGTH_ADDRESS +
        Utils.LENGTH_SIGNATURE +
        Utils.LENGTH_ERA +
        compactToU8a(nonce || 0).length +
        (extrinsic ? extrinsic.encodedLength : 0)
    )
  }

  /** hash(accountId + salt) */
  public static hashVote(accountId: string, salt: string): string {
    const accountU8a = decodeAddress(accountId)
    const saltU8a = stringToU8a(salt)
    const voteU8a = new Uint8Array(accountU8a.length + saltU8a.length)
    voteU8a.set(accountU8a)
    voteU8a.set(saltU8a, accountU8a.length)

    const hash = blake2AsHex(voteU8a, 256)
    return hash
  }

  public static wait(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms))
  }

  public static readRuntimeFromFile(path: string): string {
    return '0x' + fs.readFileSync(path).toString('hex')
  }

  public static camelToSnakeCase(key: string): string {
    return key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`)
  }

  public static metadataToBytes<T>(metaClass: AnyMetadataClass<T>, obj: T): Bytes {
    return createType('Bytes', '0x' + Buffer.from(metaClass.encode(obj).finish()).toString('hex'))
  }

  public static metadataFromBytes<T>(metaClass: AnyMetadataClass<T>, bytes: Bytes): T {
    // We use `toObject()` to get rid of .prototype defaults for optional fields
    return metaClass.toObject(metaClass.decode(bytes.toU8a(true))) as T
  }

  public static getDeserializedMetadataFormInput<T>(
    metadataClass: AnyMetadataClass<T>,
    input: MetadataInput<T>
  ): T | null {
    if (typeof input.value === 'string') {
      try {
        return Utils.metadataFromBytes(metadataClass, createType('Bytes', input.value))
      } catch (e) {
        if (!input.expectFailure) {
          throw e
        }
        return null
      }
    }

    return input.value
  }

  public static getMetadataBytesFromInput<T>(metadataClass: AnyMetadataClass<T>, input: MetadataInput<T>): Bytes {
    return typeof input.value === 'string'
      ? createType('Bytes', input.value)
      : Utils.metadataToBytes(metadataClass, input.value)
  }

  public static bytesToString(b: Bytes): string {
    return (
      Buffer.from(b.toU8a(true))
        .toString()
        // eslint-disable-next-line no-control-regex
        .replace(/\u0000/g, '')
    )
  }

  public static assert(condition: any, msg?: string): asserts condition {
    if (!condition) {
      throw new Error(msg || 'Assertion failed')
    }
  }
}
