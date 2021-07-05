import NodeCache from 'node-cache'

// TODO: move to config or set to 10 seconds
export const TokenExpirationPeriod: number = 100 * 1000 // seconds
const MaxNonces = 100000

const nonceCache = new NodeCache({
  stdTTL: TokenExpirationPeriod,
  deleteOnExpire: true,
  maxKeys: MaxNonces,
})

export function createNonce(): string {
  const nonce = process.hrtime.bigint().toString()

  nonceCache.set(nonce, null, TokenExpirationPeriod)

  return nonce
}

export function checkRemoveNonce(nonce: string): boolean {
  const deletedEntries = nonceCache.del(nonce)

  return deletedEntries > 0
}
