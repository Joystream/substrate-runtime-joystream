import { flags } from '@oclif/command'
import { CLIError } from '@oclif/errors'
import { displayNameValueTable } from '../../helpers/display'
import { Codec } from '@polkadot/types/types'
import ExitCodes from '../../ExitCodes'
import chalk from 'chalk'
import { NameValueObj, ApiMethodArg, UnaugmentedApiPromise } from '../../Types'
import ApiCommandBase from '../../base/ApiCommandBase'
import { AugmentedConst } from '@polkadot/api/types'

// Command flags type
type ApiInspectFlags = {
  type: string
  module: string
  method: string
  exec: boolean
  callArgs: string
}

// Currently "inspectable" api types
const TYPES_AVAILABLE = ['query', 'consts'] as const

// String literals type based on TYPES_AVAILABLE const.
// It works as if we specified: type ApiType = 'query' | 'consts'...;
type ApiType = typeof TYPES_AVAILABLE[number]

export default class ApiInspect extends ApiCommandBase {
  static description =
    'Lists available node API modules/methods and/or their description(s), ' +
    'or calls one of the API methods (depending on provided arguments and flags)'

  static examples = [
    '$ api:inspect',
    '$ api:inspect -t=query',
    '$ api:inspect -t=query -M=members',
    '$ api:inspect -t=query -M=members -m=membershipById',
    '$ api:inspect -t=query -M=members -m=membershipById -e',
    '$ api:inspect -t=query -M=members -m=membershipById -e -a=1',
  ]

  static flags = {
    type: flags.string({
      char: 't',
      description:
        'Specifies the type/category of the inspected request (ie. "query", "consts" etc.).\n' +
        'If no "--module" flag is provided then all available modules in that type will be listed.\n' +
        'If this flag is not provided then all available types will be listed.',
    }),
    module: flags.string({
      char: 'M',
      description:
        'Specifies the api module, ie. "system", "staking" etc.\n' +
        'If no "--method" flag is provided then all methods in that module will be listed along with the descriptions.',
      dependsOn: ['type'],
    }),
    method: flags.string({
      char: 'm',
      description: 'Specifies the api method to call/describe.',
      dependsOn: ['module'],
    }),
    exec: flags.boolean({
      char: 'e',
      description:
        'Provide this flag if you want to execute the actual call, instead of displaying the method description (which is default)',
      dependsOn: ['method'],
    }),
    callArgs: flags.string({
      char: 'a',
      description:
        'Specifies the arguments to use when calling a method. Multiple arguments can be separated with a comma, ie. "-a=arg1,arg2".\n' +
        'You can omit this flag even if the method requires some aguments.\n' +
        'In that case you will be promted to provide value for each required argument.\n' +
        "Ommiting this flag is recommended when input parameters are of more complex types (and it's hard to specify them as just simple comma-separated strings)",
      dependsOn: ['exec'],
    }),
  }

  getMethodMeta(apiType: ApiType, apiModule: string, apiMethod: string) {
    if (apiType === 'query') {
      return this.getUnaugmentedApi().query[apiModule][apiMethod].creator.meta
    } else {
      // Currently the only other optoin is api.consts
      const method = (this.getUnaugmentedApi().consts[apiModule][apiMethod] as unknown) as AugmentedConst<'promise'>
      return method.meta
    }
  }

  getMethodDescription(apiType: ApiType, apiModule: string, apiMethod: string): string {
    const description: string = this.getMethodMeta(apiType, apiModule, apiMethod).documentation.join(' ')
    return description || 'No description available.'
  }

  getQueryMethodParamsTypes(apiModule: string, apiMethod: string): string[] {
    const method = this.getUnaugmentedApi().query[apiModule][apiMethod]
    const { type } = method.creator.meta
    if (type.isDoubleMap) {
      return [type.asDoubleMap.key1.toString(), type.asDoubleMap.key2.toString()]
    }
    if (type.isMap) {
      return [type.asMap.key.toString()]
    }
    return []
  }

  getMethodReturnType(apiType: ApiType, apiModule: string, apiMethod: string): string {
    if (apiType === 'query') {
      const method = this.getUnaugmentedApi().query[apiModule][apiMethod]
      const {
        meta: { type, modifier },
      } = method.creator
      let typeName = type.toString()
      if (type.isDoubleMap) {
        typeName = type.asDoubleMap.value.toString()
      }
      if (type.isMap) {
        typeName = type.asMap.value.toString()
      }

      return modifier.isOptional ? `Option<${typeName}>` : typeName
    }
    // Fallback for "consts"
    return this.getMethodMeta(apiType, apiModule, apiMethod).type.toString()
  }

  // Validate the flags - throws an error if flags.type, flags.module or flags.method is invalid / does not exist in the api.
  // Returns type, module and method which validity we can be sure about (notice they may still be "undefined" if weren't provided).
  validateFlags(
    api: UnaugmentedApiPromise,
    flags: ApiInspectFlags
  ): { apiType: ApiType | undefined; apiModule: string | undefined; apiMethod: string | undefined } {
    let apiType: ApiType | undefined
    const { module: apiModule, method: apiMethod } = flags

    if (flags.type !== undefined) {
      const availableTypes: readonly string[] = TYPES_AVAILABLE
      if (!availableTypes.includes(flags.type)) {
        throw new CLIError('Such type is not available', { exit: ExitCodes.InvalidInput })
      }
      apiType = flags.type as ApiType
      if (apiModule !== undefined) {
        if (!api[apiType][apiModule]) {
          throw new CLIError('Such module was not found', { exit: ExitCodes.InvalidInput })
        }
        if (apiMethod !== undefined && !api[apiType][apiModule][apiMethod]) {
          throw new CLIError('Such method was not found', { exit: ExitCodes.InvalidInput })
        }
      }
    }

    return { apiType, apiModule, apiMethod }
  }

  // Request values for params using array of param types (strings)
  async requestParamsValues(paramTypes: string[]): Promise<ApiMethodArg[]> {
    const result: ApiMethodArg[] = []
    for (const [key, paramType] of Object.entries(paramTypes)) {
      this.log(chalk.bold.white(`Parameter no. ${parseInt(key) + 1} (${paramType}):`))
      const paramValue = await this.promptForParam(paramType)
      result.push(paramValue)
    }

    return result
  }

  async run() {
    const api: UnaugmentedApiPromise = this.getUnaugmentedApi()
    const flags: ApiInspectFlags = this.parse(ApiInspect).flags as ApiInspectFlags
    const availableTypes: readonly string[] = TYPES_AVAILABLE
    const { apiType, apiModule, apiMethod } = this.validateFlags(api, flags)

    // Executing a call
    if (apiType && apiModule && apiMethod && flags.exec) {
      let result: Codec

      if (apiType === 'query') {
        // Api query - call with (or without) arguments
        let args: (string | ApiMethodArg)[] = flags.callArgs ? flags.callArgs.split(',') : []
        const paramsTypes: string[] = this.getQueryMethodParamsTypes(apiModule, apiMethod)
        if (args.length < paramsTypes.length) {
          this.warn('Some parameters are missing! Please, provide the missing parameters:')
          const missingParamsValues = await this.requestParamsValues(paramsTypes.slice(args.length))
          args = args.concat(missingParamsValues)
        }
        result = await api.query[apiModule][apiMethod](...args)
      } else {
        // Api consts - just assign the value
        result = api.consts[apiModule][apiMethod]
      }

      this.log(chalk.green(result.toString()))
    }
    // Describing a method
    else if (apiType && apiModule && apiMethod) {
      this.log(chalk.bold.white(`${apiType}.${apiModule}.${apiMethod}`))
      const description: string = this.getMethodDescription(apiType, apiModule, apiMethod)
      this.log(`\n${description}\n`)
      const typesRows: NameValueObj[] = []
      if (apiType === 'query') {
        typesRows.push({
          name: 'Params:',
          value: this.getQueryMethodParamsTypes(apiModule, apiMethod).join(', ') || '-',
        })
      }
      typesRows.push({ name: 'Returns:', value: this.getMethodReturnType(apiType, apiModule, apiMethod) })
      displayNameValueTable(typesRows)
    }
    // Displaying all available methods
    else if (apiType && apiModule) {
      const module = api[apiType][apiModule]
      const rows: NameValueObj[] = Object.keys(module).map((key: string) => {
        return { name: key, value: this.getMethodDescription(apiType, apiModule, key) }
      })
      displayNameValueTable(rows)
    }
    // Displaying all available modules
    else if (apiType) {
      this.log(chalk.bold.white('Available modules:'))
      this.log(
        Object.keys(api[apiType])
          .map((key) => chalk.white(key))
          .join('\n')
      )
    }
    // Displaying all available types
    else {
      this.log(chalk.bold.white('Available types:'))
      this.log(availableTypes.map((type) => chalk.white(type)).join('\n'))
    }
  }
}
