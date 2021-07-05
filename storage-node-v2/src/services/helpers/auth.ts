import { KeyringPair } from '@polkadot/keyring/types'
import { u8aToHex } from '@polkadot/util'
import { signatureVerify } from '@polkadot/util-crypto'
import base64url from 'base64url'

export interface UploadTokenRequest {
  data: UploadTokenRequestBody
  signature: string
}

export interface UploadTokenRequestBody extends RequestData {
  memberId: number
  accountId: string
}

export interface RequestData {
  dataObjectId: number
  storageBucketId: number
  bagId: string
}

export interface UploadTokenBody extends RequestData {
  validUntil: number // timestamp
  nonce: string
}

export interface UploadToken {
  data: UploadTokenBody
  signature: string
}

export function parseUploadToken(tokenString: string): UploadToken {
  return JSON.parse(base64url.decode(tokenString))
}

export function verifyTokenSignature(
  token: UploadToken | UploadTokenRequest,
  address: string
): boolean {
  const message = JSON.stringify(token.data)
  const { isValid } = signatureVerify(message, token.signature, address)

  return isValid
}

export function signTokenBody(
  tokenBody: UploadTokenBody | UploadTokenRequestBody,
  account: KeyringPair
): string {
  const message = JSON.stringify(tokenBody)
  const signature = u8aToHex(account.sign(message))

  return signature
}

export function createUploadToken(
  tokenBody: UploadTokenBody,
  account: KeyringPair
): string {
  const signature = signTokenBody(tokenBody, account)

  const token = {
    data: tokenBody,
    signature,
  }

  return base64url.encode(JSON.stringify(token))
}
