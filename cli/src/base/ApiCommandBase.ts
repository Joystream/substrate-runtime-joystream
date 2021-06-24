import ExitCodes from '../ExitCodes'
import { CLIError } from '@oclif/errors'
import StateAwareCommandBase from './StateAwareCommandBase'
import Api from '../Api'
import { getTypeDef, Option, Tuple } from '@polkadot/types'
import { Registry, Codec, TypeDef, TypeDefInfo } from '@polkadot/types/types'
import { Vec, Struct, Enum } from '@polkadot/types/codec'
import { WsProvider } from '@polkadot/api'
import { KeyringPair } from '@polkadot/keyring/types'
import chalk from 'chalk'
import { InterfaceTypes } from '@polkadot/types/types/registry'
import { ApiMethodArg, ApiMethodNamedArgs, ApiParamsOptions, ApiParamOptions } from '../Types'
import { createParamOptions } from '../helpers/promptOptions'
import { AugmentedSubmittables, SubmittableExtrinsic } from '@polkadot/api/types'
import { DistinctQuestion } from 'inquirer'
import { BOOL_PROMPT_OPTIONS } from '../helpers/prompting'
import { DispatchError } from '@polkadot/types/interfaces/system'
import { formatBalance } from '@polkadot/util'
import BN from 'bn.js'
import _ from 'lodash'

export class ExtrinsicFailedError extends Error {}

/**
 * Abstract base class for commands that require access to the API.
 */
export default abstract class ApiCommandBase extends StateAwareCommandBase {
  private api: Api | null = null

  getApi(): Api {
    if (!this.api) throw new CLIError('Tried to get API before initialization.', { exit: ExitCodes.ApiError })
    return this.api
  }

  // Shortcuts
  getOriginalApi() {
    return this.getApi().getOriginalApi()
  }

  getUnaugmentedApi() {
    return this.getApi().getUnaugmentedApi()
  }

  getTypesRegistry(): Registry {
    return this.getOriginalApi().registry
  }

  createType<K extends keyof InterfaceTypes>(typeName: K, value?: unknown): InterfaceTypes[K] {
    return this.getOriginalApi().createType(typeName, value)
  }

  async init() {
    await super.init()
    let apiUri: string = this.getPreservedState().apiUri

    if (!apiUri) {
      this.warn("You haven't provided a Joystream node websocket api uri for the CLI to connect to yet!")
      apiUri = await this.promptForApiUri()
    }

    let queryNodeUri: string = this.getPreservedState().queryNodeUri
    if (!queryNodeUri) {
      this.warn("You haven't provided a Joystream query node uri for the CLI to connect to yet!")
      queryNodeUri = await this.promptForQueryNodeUri()
    }

    const { metadataCache } = this.getPreservedState()
    this.api = await Api.create(apiUri, metadataCache, queryNodeUri === 'none' ? undefined : queryNodeUri)

    const { genesisHash, runtimeVersion } = this.getOriginalApi()
    const metadataKey = `${genesisHash}-${runtimeVersion.specVersion}`
    if (!metadataCache[metadataKey]) {
      // Add new entry to metadata cache
      metadataCache[metadataKey] = await this.getOriginalApi().runtimeMetadata.toJSON()
      await this.setPreservedState({ metadataCache })
    }
  }

  async promptForApiUri(): Promise<string> {
    let selectedNodeUri = await this.simplePrompt({
      type: 'list',
      message: 'Choose a node websocket api uri:',
      choices: [
        {
          name: 'Local node (ws://localhost:9944)',
          value: 'ws://localhost:9944',
        },
        {
          name: 'Current Testnet official Joystream node (wss://rome-rpc-endpoint.joystream.org:9944/)',
          value: 'wss://rome-rpc-endpoint.joystream.org:9944/',
        },
        {
          name: 'Custom endpoint',
          value: '',
        },
      ],
    })

    if (!selectedNodeUri) {
      do {
        selectedNodeUri = await this.simplePrompt({
          type: 'input',
          message: 'Provide a WS endpoint uri',
        })
        if (!this.isApiUriValid(selectedNodeUri)) {
          this.warn('Provided uri seems incorrect! Please try again...')
        }
      } while (!this.isApiUriValid(selectedNodeUri))
    }

    await this.setPreservedState({ apiUri: selectedNodeUri })

    return selectedNodeUri
  }

  async promptForQueryNodeUri(): Promise<string> {
    let selectedUri = await this.simplePrompt({
      type: 'list',
      message: 'Choose a query node endpoint:',
      choices: [
        {
          name: 'Local query node (http://localhost:8081/graphql)',
          value: 'http://localhost:8081/graphql',
        },
        {
          name: 'Jsgenesis-hosted query node (https://hydra.joystream.org/graphql)',
          value: 'https://hydra.joystream.org/graphql',
        },
        {
          name: 'Custom endpoint',
          value: '',
        },
        {
          name: "No endpoint (if you don't use query node some features will not be available)",
          value: 'none',
        },
      ],
    })

    if (!selectedUri) {
      do {
        selectedUri = await this.simplePrompt({
          type: 'input',
          message: 'Provide a query node endpoint',
        })
        if (!this.isApiUriValid(selectedUri)) {
          this.warn('Provided uri seems incorrect! Please try again...')
        }
      } while (!this.isApiUriValid(selectedUri))
    }

    await this.setPreservedState({ queryNodeUri: selectedUri })

    return selectedUri
  }

  isApiUriValid(uri: string) {
    try {
      // eslint-disable-next-line no-new
      new WsProvider(uri)
    } catch (e) {
      return false
    }
    return true
  }

  isQueryNodeUriValid(uri: string) {
    let url: URL
    try {
      url = new URL(uri)
    } catch (_) {
      return false
    }

    return url.protocol === 'http:' || url.protocol === 'https:'
  }

  // This is needed to correctly handle some structs, enums etc.
  // Where the main typeDef doesn't provide enough information
  protected getRawTypeDef(type: keyof InterfaceTypes) {
    const instance = this.createType(type)
    return getTypeDef(instance.toRawType())
  }

  // Prettifier for type names which are actually JSON strings
  protected prettifyJsonTypeName(json: string) {
    const obj = JSON.parse(json) as { [key: string]: string }
    return (
      '{\n' +
      Object.keys(obj)
        .map((prop) => `  ${prop}${chalk.white(':' + obj[prop])}`)
        .join('\n') +
      '\n}'
    )
  }

  // Get param name based on TypeDef object
  protected paramName(typeDef: TypeDef) {
    return chalk.green(
      typeDef.displayName ||
        typeDef.name ||
        (typeDef.type.startsWith('{') ? this.prettifyJsonTypeName(typeDef.type) : typeDef.type)
    )
  }

  // Prompt for simple/plain value (provided as string) of given type
  async promptForSimple(typeDef: TypeDef, paramOptions?: ApiParamOptions): Promise<Codec> {
    // If no default provided - get default value resulting from providing empty string
    const defaultValueString =
      paramOptions?.value?.default?.toString() || this.createType(typeDef.type as any, '').toString()

    let typeSpecificOptions: DistinctQuestion = { type: 'input' }
    if (typeDef.type === 'bool') {
      typeSpecificOptions = BOOL_PROMPT_OPTIONS
    }

    const providedValue = await this.simplePrompt({
      message: `Provide value for ${this.paramName(typeDef)}`,
      ...typeSpecificOptions,
      // We want to avoid showing default value like '0x', because it falsely suggests
      // that user needs to provide the value as hex
      default: (defaultValueString === '0x' ? '' : defaultValueString) || undefined,
      validate: paramOptions?.validator,
    })
    return this.createType(typeDef.type as any, providedValue)
  }

  // Prompt for Option<Codec> value
  async promptForOption(typeDef: TypeDef, paramOptions?: ApiParamOptions): Promise<Option<Codec>> {
    const subtype = typeDef.sub as TypeDef // We assume that Opion always has a single subtype
    const defaultValue = paramOptions?.value?.default as Option<Codec> | undefined
    const confirmed = await this.simplePrompt({
      message: `Do you want to provide the optional ${this.paramName(typeDef)} parameter?`,
      type: 'confirm',
      default: defaultValue ? defaultValue.isSome : false,
    })

    if (confirmed) {
      this.openIndentGroup()
      const value = await this.promptForParam(
        subtype.type,
        createParamOptions(subtype.name, defaultValue?.unwrapOr(undefined))
      )
      this.closeIndentGroup()
      return this.createType(`Option<${subtype.type}>` as any, value)
    }

    return this.createType(`Option<${subtype.type}>` as any, null)
  }

  // Prompt for Tuple
  // TODO: Not well tested yet
  async promptForTuple(typeDef: TypeDef, paramOptions?: ApiParamOptions): Promise<Tuple> {
    console.log(chalk.grey(`Providing values for ${this.paramName(typeDef)} tuple:`))

    this.openIndentGroup()
    const result: ApiMethodArg[] = []
    // We assume that for Tuple there is always at least 1 subtype (pethaps it's even always an array?)
    const subtypes: TypeDef[] = Array.isArray(typeDef.sub) ? typeDef.sub! : [typeDef.sub!]
    const defaultValue = paramOptions?.value?.default as Tuple | undefined

    for (const [index, subtype] of Object.entries(subtypes)) {
      const entryDefaultVal = defaultValue && defaultValue[parseInt(index)]
      const inputParam = await this.promptForParam(subtype.type, createParamOptions(subtype.name, entryDefaultVal))
      result.push(inputParam)
    }
    this.closeIndentGroup()

    return new Tuple(this.getTypesRegistry(), subtypes.map((subtype) => subtype.type) as any, result)
  }

  // Prompt for Struct
  async promptForStruct(typeDef: TypeDef, paramOptions?: ApiParamOptions): Promise<ApiMethodArg> {
    console.log(chalk.grey(`Providing values for ${this.paramName(typeDef)} struct:`))

    this.openIndentGroup()
    const structType = typeDef.type
    const rawTypeDef = this.getRawTypeDef(structType as keyof InterfaceTypes)
    // We assume struct typeDef always has array of typeDefs inside ".sub"
    const structSubtypes = rawTypeDef.sub as TypeDef[]
    const structDefault = paramOptions?.value?.default as Struct | undefined

    const structValues: { [key: string]: ApiMethodArg } = {}
    for (const subtype of structSubtypes) {
      const fieldOptions = paramOptions?.nestedOptions && paramOptions.nestedOptions[subtype.name!]
      const fieldDefaultValue = fieldOptions?.value?.default || (structDefault && structDefault.get(subtype.name!))
      const finalFieldOptions: ApiParamOptions = {
        forcedName: subtype.name,
        ...fieldOptions, // "forcedName" above should be overriden with "fieldOptions.forcedName" if available
        value: fieldDefaultValue && { ...fieldOptions?.value, default: fieldDefaultValue },
      }
      structValues[subtype.name!] = await this.promptForParam(subtype.type, finalFieldOptions)
    }
    this.closeIndentGroup()

    return this.createType(structType as any, structValues)
  }

  // Prompt for Vec
  async promptForVec(typeDef: TypeDef, paramOptions?: ApiParamOptions): Promise<Vec<Codec>> {
    console.log(chalk.grey(`Providing values for ${this.paramName(typeDef)} vector:`))

    this.openIndentGroup()
    // We assume Vec always has one TypeDef as ".sub"
    const subtype = typeDef.sub as TypeDef
    const defaultValue = paramOptions?.value?.default as Vec<Codec> | undefined
    const entries: Codec[] = []
    let addAnother = false
    do {
      addAnother = await this.simplePrompt({
        message: `Do you want to add another entry to ${this.paramName(typeDef)} vector (currently: ${
          entries.length
        })?`,
        type: 'confirm',
        default: defaultValue ? entries.length < defaultValue.length : false,
      })
      const defaultEntryValue = defaultValue && defaultValue[entries.length]
      if (addAnother) {
        entries.push(await this.promptForParam(subtype.type, createParamOptions(subtype.name, defaultEntryValue)))
      }
    } while (addAnother)
    this.closeIndentGroup()

    return this.createType(`Vec<${subtype.type}>` as any, entries)
  }

  // Prompt for Enum
  async promptForEnum(typeDef: TypeDef, paramOptions?: ApiParamOptions): Promise<Enum> {
    const enumType = typeDef.type as keyof InterfaceTypes
    const rawTypeDef = this.getRawTypeDef(enumType)
    // We assume enum always has array on TypeDefs inside ".sub"
    const enumSubtypes = rawTypeDef.sub as TypeDef[]
    const defaultValue = paramOptions?.value?.default as Enum | undefined

    const enumSubtypeName = await this.simplePrompt({
      message: `Choose value for ${this.paramName(typeDef)}:`,
      type: 'list',
      choices: enumSubtypes.map((subtype) => ({
        name: subtype.name,
        value: subtype.name,
      })),
      default: defaultValue?.type,
    })

    const enumSubtype = enumSubtypes.find((st) => st.name === enumSubtypeName)!

    if (enumSubtype.type !== 'Null') {
      const subtypeOptions = createParamOptions(enumSubtype.name, defaultValue?.value)
      return this.createType(enumType as any, {
        [enumSubtype.name!]: await this.promptForParam(enumSubtype.type, subtypeOptions),
      })
    }

    return this.createType(enumType as any, enumSubtype.name)
  }

  // Prompt for param based on "paramType" string (ie. Option<MemeberId>)
  // TODO: This may not yet work for all possible types
  async promptForParam(
    paramType: string,
    paramOptions?: ApiParamOptions // TODO: This is not fully implemented for all types yet
  ): Promise<ApiMethodArg> {
    const typeDef = getTypeDef(paramType)
    const rawTypeDef = this.getRawTypeDef(paramType as keyof InterfaceTypes)

    if (paramOptions?.forcedName) {
      typeDef.name = paramOptions.forcedName
    }

    if (paramOptions?.value?.locked) {
      return paramOptions.value.default
    }

    if (rawTypeDef.info === TypeDefInfo.Option) {
      return await this.promptForOption(typeDef, paramOptions)
    } else if (rawTypeDef.info === TypeDefInfo.Tuple) {
      return await this.promptForTuple(typeDef, paramOptions)
    } else if (rawTypeDef.info === TypeDefInfo.Struct) {
      return await this.promptForStruct(typeDef, paramOptions)
    } else if (rawTypeDef.info === TypeDefInfo.Enum) {
      return await this.promptForEnum(typeDef, paramOptions)
    } else if (rawTypeDef.info === TypeDefInfo.Vec) {
      return await this.promptForVec(typeDef, paramOptions)
    } else {
      return await this.promptForSimple(typeDef, paramOptions)
    }
  }

  // More typesafe version
  async promptForType(type: keyof InterfaceTypes, options?: ApiParamOptions) {
    return await this.promptForParam(type, options)
  }

  async promptForExtrinsicParams(
    module: string,
    method: string,
    paramsOptions?: ApiParamsOptions
  ): Promise<ApiMethodArg[]> {
    const extrinsicMethod = (await this.getUnaugmentedApi().tx)[module][method]
    const values: ApiMethodArg[] = []

    this.openIndentGroup()
    for (const arg of extrinsicMethod.meta.args.toArray()) {
      const argName = arg.name.toString()
      const argType = arg.type.toString()
      let argOptions = paramsOptions && paramsOptions[argName]
      if (!argOptions?.forcedName) {
        argOptions = { ...argOptions, forcedName: argName }
      }
      values.push(await this.promptForParam(argType, argOptions))
    }
    this.closeIndentGroup()

    return values
  }

  sendExtrinsic(account: KeyringPair, tx: SubmittableExtrinsic<'promise'>): Promise<void> {
    return new Promise((resolve, reject) => {
      let unsubscribe: () => void
      tx.signAndSend(account, {}, (result) => {
        // Implementation loosely based on /pioneer/packages/react-signer/src/Modal.tsx
        if (!result || !result.status) {
          return
        }

        if (result.status.isInBlock) {
          unsubscribe()
          result.events
            .filter(({ event }) => event.section === 'system')
            .forEach(({ event }) => {
              if (event.method === 'ExtrinsicFailed') {
                const dispatchError = event.data[0] as DispatchError
                let errorMsg = dispatchError.toString()
                if (dispatchError.isModule) {
                  try {
                    const { name, documentation } = this.getOriginalApi().registry.findMetaError(dispatchError.asModule)
                    errorMsg = `${name} (${documentation})`
                  } catch (e) {
                    // This probably means we don't have this error in the metadata
                    // In this case - continue (we'll just display dispatchError.toString())
                  }
                }
                reject(new ExtrinsicFailedError(`Extrinsic execution error: ${errorMsg}`))
              } else if (event.method === 'ExtrinsicSuccess') {
                resolve()
              }
            })
        } else if (result.isError) {
          reject(new ExtrinsicFailedError('Extrinsic execution error!'))
        }
      })
        .then((unsubFunc) => (unsubscribe = unsubFunc))
        .catch((e) =>
          reject(new ExtrinsicFailedError(`Cannot send the extrinsic: ${e.message ? e.message : JSON.stringify(e)}`))
        )
    })
  }

  async sendAndFollowTx(
    account: KeyringPair,
    tx: SubmittableExtrinsic<'promise'>,
    warnOnly = false // If specified - only warning will be displayed in case of failure (instead of error beeing thrown)
  ): Promise<boolean> {
    // Calculate fee and ask for confirmation
    const fee = await this.getApi().estimateFee(account, tx)

    await this.requireConfirmation(
      `Tx fee of ${chalk.cyan(formatBalance(fee))} will be deduced from you account, do you confirm the transfer?`
    )

    try {
      await this.sendExtrinsic(account, tx)
      this.log(chalk.green(`Extrinsic successful!`))
      return true
    } catch (e) {
      if (e instanceof ExtrinsicFailedError && warnOnly) {
        this.warn(`Extrinsic failed! ${e.message}`)
        return false
      } else if (e instanceof ExtrinsicFailedError) {
        throw new CLIError(`Extrinsic failed! ${e.message}`, { exit: ExitCodes.ApiError })
      } else {
        throw e
      }
    }
  }

  private humanize(p: unknown): any {
    if (Array.isArray(p)) {
      return p.map((v) => this.humanize(v))
    } else if (typeof p === 'object' && p !== null) {
      if ((p as any).toHuman) {
        return (p as Codec).toHuman()
      } else if (p instanceof BN) {
        return p.toString()
      } else {
        return _.mapValues(p, this.humanize.bind(this))
      }
    }

    return p
  }

  async sendAndFollowNamedTx<
    Module extends keyof AugmentedSubmittables<'promise'>,
    Method extends keyof AugmentedSubmittables<'promise'>[Module] & string,
    Submittable extends AugmentedSubmittables<'promise'>[Module][Method]
  >(
    account: KeyringPair,
    module: Module,
    method: Method,
    params: Submittable extends (...args: any[]) => any ? Parameters<Submittable> : [],
    warnOnly = false
  ): Promise<boolean> {
    this.log(
      chalk.white(
        `\nSending ${module}.${method} extrinsic from ${account.meta.name ? account.meta.name : account.address}...`
      )
    )
    console.log('Params:', this.humanize(params))
    const tx = await this.getUnaugmentedApi().tx[module][method](...params)
    return await this.sendAndFollowTx(account, tx, warnOnly)
  }

  async buildAndSendExtrinsic<
    Module extends keyof AugmentedSubmittables<'promise'>,
    Method extends keyof AugmentedSubmittables<'promise'>[Module] & string
  >(
    account: KeyringPair,
    module: Module,
    method: Method,
    paramsOptions?: ApiParamsOptions,
    warnOnly = false // If specified - only warning will be displayed (instead of error beeing thrown)
  ): Promise<ApiMethodArg[]> {
    const params = await this.promptForExtrinsicParams(module, method, paramsOptions)
    await this.sendAndFollowNamedTx(account, module, method, params as any, warnOnly)

    return params
  }

  extrinsicArgsFromDraft(module: string, method: string, draftFilePath: string): ApiMethodNamedArgs {
    let draftJSONObj
    const parsedArgs: ApiMethodNamedArgs = []
    const extrinsicMethod = this.getUnaugmentedApi().tx[module][method]
    try {
      // eslint-disable-next-line @typescript-eslint/no-var-requires
      draftJSONObj = require(draftFilePath)
    } catch (e) {
      throw new CLIError(`Could not load draft from: ${draftFilePath}`, { exit: ExitCodes.InvalidFile })
    }
    if (!draftJSONObj || !Array.isArray(draftJSONObj) || draftJSONObj.length !== extrinsicMethod.meta.args.length) {
      throw new CLIError(`The draft file at ${draftFilePath} is invalid!`, { exit: ExitCodes.InvalidFile })
    }
    for (const [index, arg] of Object.entries(extrinsicMethod.meta.args.toArray())) {
      const argName = arg.name.toString()
      const argType = arg.type.toString()
      try {
        parsedArgs.push({ name: argName, value: this.createType(argType as any, draftJSONObj[parseInt(index)]) })
      } catch (e) {
        throw new CLIError(`Couldn't parse ${argName} value from draft at ${draftFilePath}!`, {
          exit: ExitCodes.InvalidFile,
        })
      }
    }

    return parsedArgs
  }
}
