import * as multihash from 'multihashes'
import fs from 'fs'
import { createHash } from 'blake3'

export function hashFile(filename: string): Promise<string> {
  const fileStream = fs.createReadStream(filename).pipe(createHash())

  return new Promise((resolve, reject) => {
    let hash: Uint8Array
    fileStream.on('data', function (chunk) {
      hash = chunk
    })
    fileStream.on('end', function () {
      const encoded = multihash.encode(hash, 'blake3')
      const result = multihash.toB58String(encoded)

      resolve(result)
    })
    fileStream.on('error', function (err) {
      reject(err)
    })
  })
}
