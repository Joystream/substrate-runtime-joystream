import express from 'express'
import path from 'path'
import cors from 'cors'
import { Express, NextFunction } from 'express-serve-static-core'
import * as OpenApiValidator from 'express-openapi-validator'
import {
  HttpError,
  OpenAPIV3,
} from 'express-openapi-validator/dist/framework/types'
import { KeyringPair } from '@polkadot/keyring/types'
import { ApiPromise } from '@polkadot/api'
import {
  RequestData,
  verifyTokenSignature,
  parseUploadToken,
  UploadToken,
} from '../helpers/auth'
import { checkRemoveNonce } from '../../services/helpers/tokenNonceKeeper'
import { httpLogger } from '../../services/logger'

// Creates web API application.
export async function createApp(
  api: ApiPromise,
  account: KeyringPair,
  workerId: number,
  uploadsDir: string
): Promise<Express> {
  const spec = path.join(__dirname, './../../api-spec/openapi.yaml')

  const app = express()

  app.use(cors())
  app.use(express.json())
  app.use(httpLogger())

  app.use(
    // Set parameters for each request.
    (req: express.Request, res: express.Response, next: NextFunction) => {
      res.locals.uploadsDir = uploadsDir
      res.locals.storageProviderAccount = account
      res.locals.workerId = workerId
      res.locals.api = api
      next()
    },
    // Setup OpenAPiValidator
    OpenApiValidator.middleware({
      apiSpec: spec,
      validateApiSpec: true,
      validateResponses: true,
      validateRequests: true,
      operationHandlers: {
        basePath: path.join(__dirname, './controllers'),
        resolver: OpenApiValidator.resolvers.modulePathResolver,
      },
      fileUploader: { dest: uploadsDir },
      validateSecurity: {
        handlers: {
          UploadAuth: validateUpload(api, account),
        },
      },
    })
  )

  // Request validation error handler.
  /* eslint-disable @typescript-eslint/no-unused-vars */ // Required signature.
  app.use(
    (
      err: HttpError,
      req: express.Request,
      res: express.Response,
      next: express.NextFunction
    ) => {
      res.status(err.status).json({
        type: 'request_validation',
        message: err.message,
        errors: err.errors,
      })
      next(err)
    }
  )

  return app
}

// Defines a signature for a upload validation function.
type ValidateUploadFunction = (
  req: express.Request,
  scopes: string[],
  schema: OpenAPIV3.SecuritySchemeObject
) => boolean | Promise<boolean>

// Creates upload validation function.
function validateUpload(
  api: ApiPromise,
  account: KeyringPair
): ValidateUploadFunction {
  // We don't use these variables yet.
  /* eslint-disable @typescript-eslint/no-unused-vars */
  return (
    req: express.Request,
    scopes: string[],
    schema: OpenAPIV3.SecuritySchemeObject
  ) => {
    const tokenString = req.headers['x-api-key'] as string
    const token = parseUploadToken(tokenString)

    const sourceTokenRequest: RequestData = {
      dataObjectId: parseInt(req.body.dataObjectId),
      storageBucketId: parseInt(req.body.storageBucketId),
      bagId: req.body.bagId,
    }

    verifyUploadTokenData(account.address, token, sourceTokenRequest)

    return true
  }
}

// Throws exceptions on errors.
function verifyUploadTokenData(
  accountAddress: string,
  token: UploadToken,
  request: RequestData
): void {
  if (!verifyTokenSignature(token, accountAddress)) {
    throw new Error('Invalid signature')
  }

  if (token.data.dataObjectId !== request.dataObjectId) {
    throw new Error('Unexpected dataObjectId')
  }

  if (token.data.storageBucketId !== request.storageBucketId) {
    throw new Error('Unexpected storageBucketId')
  }

  if (token.data.bagId !== request.bagId) {
    throw new Error('Unexpected bagId')
  }

  if (token.data.validUntil < Date.now()) {
    throw new Error('Token expired')
  }

  if (!checkRemoveNonce(token.data.nonce)) {
    throw new Error('Nonce not found')
  }
}
