import Ajv from 'ajv'
import inquirer, { DistinctQuestion } from 'inquirer'
import _ from 'lodash'
import RefParser, { JSONSchema } from '@apidevtools/json-schema-ref-parser'
import chalk from 'chalk'
import { BOOL_PROMPT_OPTIONS } from './prompting'

type CustomPromptMethod = () => Promise<any>
type CustomPrompt = DistinctQuestion | CustomPromptMethod | { $item: CustomPrompt } | 'skip'

// For the explaination of "string & { x: never }", see: https://github.com/microsoft/TypeScript/issues/29729
// eslint-disable-next-line @typescript-eslint/ban-types
export type JsonSchemaCustomPrompts<T = Record<string, unknown>> = [keyof T | (string & {}) | RegExp, CustomPrompt][]

export class JsonSchemaPrompter<JsonResult> {
  schema: JSONSchema
  schemaPath: string
  customPropmpts?: JsonSchemaCustomPrompts
  ajv: Ajv.Ajv
  filledObject: Partial<JsonResult>

  constructor(
    schema: JSONSchema,
    defaults?: Partial<JsonResult>,
    customPrompts?: JsonSchemaCustomPrompts,
    schemaPath = '.'
  ) {
    this.customPropmpts = customPrompts
    this.schema = schema
    this.schemaPath = schemaPath
    // allErrors prevents .validate from setting only one error when in fact there are multiple
    this.ajv = new Ajv({ allErrors: true })
    this.filledObject = defaults || {}
  }

  private oneOfToOptions(oneOf: JSONSchema[], currentValue: any) {
    let defaultValue: any
    const choices: { name: string; value: number | string }[] = []

    oneOf.forEach((pSchema, index) => {
      if (pSchema.description) {
        choices.push({ name: pSchema.description, value: index.toString() })
      } else if (pSchema.type === 'object' && pSchema.properties) {
        choices.push({ name: `{ ${Object.keys(pSchema.properties).join(', ')} }`, value: index.toString() })
        // Supports defaults for enum variants:
        if (
          typeof currentValue === 'object' &&
          currentValue !== null &&
          Object.keys(currentValue).join(',') === Object.keys(pSchema.properties).join(',')
        ) {
          defaultValue = index.toString()
        }
      } else {
        choices.push({ name: index.toString(), value: index.toString() })
      }
    })

    return { choices, default: defaultValue }
  }

  private getCustomPrompt(propertyPath: string): CustomPrompt | undefined {
    const found = this.customPropmpts?.find(([pathToMatch]) =>
      pathToMatch instanceof RegExp ? pathToMatch.test(propertyPath) : propertyPath === pathToMatch
    )

    return found ? found[1] : undefined
  }

  private propertyDisplayName(propertyPath: string) {
    return chalk.green(propertyPath)
  }

  private async prompt(
    schema: JSONSchema,
    propertyPath = '',
    custom?: CustomPrompt,
    allPropsRequired = false
  ): Promise<any> {
    const customPrompt: CustomPrompt | undefined = custom || this.getCustomPrompt(propertyPath)
    const propDisplayName = this.propertyDisplayName(propertyPath)
    const currentValue = _.get(this.filledObject, propertyPath)
    const type = Array.isArray(schema.type) ? schema.type[0] : schema.type

    if (customPrompt === 'skip') {
      return
    }

    // Automatically handle "null" values (useful for enum variants)
    if (type === 'null') {
      _.set(this.filledObject, propertyPath, null)
      return null
    }

    // Custom prompt
    if (typeof customPrompt === 'function') {
      return await this.promptWithRetry(customPrompt, propertyPath, true)
    }

    // oneOf
    if (schema.oneOf) {
      const oneOf = schema.oneOf as JSONSchema[]
      const options = this.oneOfToOptions(oneOf, currentValue)
      const choosen = await this.inquirerSinglePrompt({
        message: propDisplayName,
        type: 'list',
        ...options,
      })
      if (choosen !== options.default) {
        _.set(this.filledObject, propertyPath, undefined) // Clear any previous value if different variant selected
      }
      return await this.prompt(oneOf[parseInt(choosen)], propertyPath)
    }

    // object
    if (type === 'object' && schema.properties) {
      const value: Record<string, any> = {}
      for (const [pName, pSchema] of Object.entries(schema.properties)) {
        const objectPropertyPath = propertyPath ? `${propertyPath}.${pName}` : pName
        const propertyCustomPrompt = this.getCustomPrompt(objectPropertyPath)

        if (propertyCustomPrompt === 'skip') {
          continue
        }

        let confirmed = true
        const required = allPropsRequired || (Array.isArray(schema.required) && schema.required.includes(pName))

        if (!required) {
          confirmed = await this.inquirerSinglePrompt({
            message: `Do you want to provide optional ${chalk.greenBright(objectPropertyPath)}?`,
            type: 'confirm',
            default:
              _.get(this.filledObject, objectPropertyPath) !== undefined &&
              _.get(this.filledObject, objectPropertyPath) !== null,
          })
        }
        if (confirmed) {
          value[pName] = await this.prompt(pSchema, objectPropertyPath)
        } else {
          _.set(this.filledObject, objectPropertyPath, null)
        }
      }
      return value
    }

    // array
    if (type === 'array' && schema.items) {
      return await this.promptWithRetry(() => this.promptArray(schema, propertyPath), propertyPath, true)
    }

    // "primitive" values:
    const basicPromptOptions: DistinctQuestion = {
      message: propDisplayName,
      default: currentValue !== undefined ? currentValue : schema.default,
    }

    let additionalPromptOptions: DistinctQuestion | undefined
    let normalizer: (v: any) => any = (v) => v

    // Prompt options
    if (schema.enum) {
      additionalPromptOptions = { type: 'list', choices: schema.enum as any[] }
    } else if (type === 'boolean') {
      additionalPromptOptions = BOOL_PROMPT_OPTIONS
    }

    // Normalizers
    if (type === 'integer') {
      normalizer = (v) => (parseInt(v).toString() === v ? parseInt(v) : v)
    }

    if (type === 'number') {
      normalizer = (v) => (Number(v).toString() === v ? Number(v) : v)
    }

    const promptOptions = { ...basicPromptOptions, ...additionalPromptOptions, ...customPrompt }
    // Need to wrap in retry, because "validate" will not get called if "type" is "list" etc.
    return await this.promptWithRetry(
      async () => normalizer(await this.promptSimple(promptOptions, propertyPath, normalizer)),
      propertyPath
    )
  }

  private setValueAndGetError(propertyPath: string, value: any, nestedErrors = false): string | null {
    _.set(this.filledObject as Record<string, unknown>, propertyPath, value)
    this.ajv.validate(this.schema, this.filledObject) as boolean
    return this.ajv.errors
      ? this.ajv.errors
          .filter((e) => (nestedErrors ? e.dataPath.startsWith(`.${propertyPath}`) : e.dataPath === `.${propertyPath}`))
          .map((e) => (e.dataPath.replace(`.${propertyPath}`, '') || 'This value') + ` ${e.message}`)
          .join(', ')
      : null
  }

  private async promptArray(schema: JSONSchema, propertyPath: string) {
    if (!schema.items) {
      return []
    }
    const { maxItems = Number.MAX_SAFE_INTEGER } = schema
    let currItem = 0
    const result = []
    while (currItem < maxItems) {
      const next = await this.inquirerSinglePrompt({
        ...BOOL_PROMPT_OPTIONS,
        message: `Do you want to add another item to ${this.propertyDisplayName(propertyPath)} array?`,
        default: _.get(this.filledObject, `${propertyPath}[${currItem}]`) !== undefined,
      })
      if (!next) {
        break
      }
      const itemSchema = Array.isArray(schema.items) ? schema.items[schema.items.length % currItem] : schema.items
      result.push(await this.prompt(typeof itemSchema === 'boolean' ? {} : itemSchema, `${propertyPath}[${currItem}]`))

      ++currItem
    }

    return result
  }

  private async promptSimple(promptOptions: DistinctQuestion, propertyPath: string, normalize?: (v: any) => any) {
    const result = await this.inquirerSinglePrompt({
      ...promptOptions,
      validate: (v) => {
        v = normalize ? normalize(v) : v
        return (
          this.setValueAndGetError(propertyPath, v) ||
          (promptOptions.validate ? promptOptions.validate(v) : true) ||
          true
        )
      },
    })

    return result
  }

  private async promptWithRetry(customMethod: CustomPromptMethod, propertyPath: string, nestedErrors = false) {
    let error: string | null
    let value: any
    do {
      value = await customMethod()
      error = this.setValueAndGetError(propertyPath, value, nestedErrors)
      if (error) {
        console.log('\n')
        console.log('Provided value:', value)
        console.warn(`ERROR: ${error}`)
        console.warn(`Try providing the input for ${propertyPath} again...`)
      }
    } while (error)

    return value
  }

  async getMainSchema() {
    return await RefParser.dereference(this.schemaPath, this.schema, {})
  }

  async promptAll(allPropsRequired = false) {
    await this.prompt(await this.getMainSchema(), '', undefined, allPropsRequired)
    return this.filledObject as JsonResult
  }

  async promptMultipleProps<P extends keyof JsonResult & string, PA extends readonly P[]>(
    props: PA
  ): Promise<{ [K in PA[number]]: Exclude<JsonResult[K], undefined> }> {
    const result: Partial<{ [K in PA[number]]: Exclude<JsonResult[K], undefined> }> = {}
    for (const prop of props) {
      result[prop] = await this.promptSingleProp(prop)
    }

    return result as { [K in PA[number]]: Exclude<JsonResult[K], undefined> }
  }

  async promptSingleProp<P extends keyof JsonResult & string>(
    p: P,
    customPrompt?: CustomPrompt
  ): Promise<Exclude<JsonResult[P], undefined>> {
    const mainSchema = await this.getMainSchema()
    await this.prompt(mainSchema.properties![p] as JSONSchema, p, customPrompt)
    return this.filledObject[p] as Exclude<JsonResult[P], undefined>
  }

  async inquirerSinglePrompt(question: DistinctQuestion) {
    const { result } = await inquirer.prompt([
      {
        ...question,
        name: 'result',
        prefix: '',
      },
    ])

    return result
  }
}
