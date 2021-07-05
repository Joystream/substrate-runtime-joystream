#!/usr/bin/env ts-node

import { createApi } from '../src/services/runtime/api'
import { getAlicePair } from '../src/services/runtime/accounts'
import { UploadTokenRequestBody, signTokenBody, UploadTokenRequest } from '../src/services/helpers/auth'
import { exit } from 'process'

//Wasm init
createApi('ws://localhost:9944').then(() => {

  const alice = getAlicePair()

  const tokenRequestBody: UploadTokenRequestBody = {
    memberId: 0,
    accountId: alice.address,
    dataObjectId: 2,
    storageBucketId: 0,
    bagId: 'static:council'
  }
  
  const signature = signTokenBody(tokenRequestBody, alice)
  const tokenRequest: UploadTokenRequest = {
    data: tokenRequestBody,
    signature
  }

  console.log(JSON.stringify(tokenRequest))

  exit(0)
})

