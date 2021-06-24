import { flags } from '@oclif/command'
import { CLIError } from '@oclif/errors'
import ExitCodes from '../ExitCodes'
import fs from 'fs'
import path from 'path'
import Ajv from 'ajv'
import $RefParser, { JSONSchema } from '@apidevtools/json-schema-ref-parser'
import chalk from 'chalk'

export const IOFlags = {
  input: flags.string({
    char: 'i',
    required: false,
    description: `Path to JSON file to use as input (if not specified - the input can be provided interactively)`,
  }),
  output: flags.string({
    char: 'o',
    required: false,
    description:
      'Path to the directory where the output JSON file should be placed (the output file can be then reused as input)',
  }),
}

export async function getInputJson<T>(inputPath?: string, schema?: JSONSchema, schemaPath?: string): Promise<T | null> {
  if (inputPath) {
    let content, jsonObj
    try {
      content = fs.readFileSync(inputPath).toString()
    } catch (e) {
      throw new CLIError(`Cannot access the input file at: ${inputPath}`, { exit: ExitCodes.FsOperationFailed })
    }
    try {
      jsonObj = JSON.parse(content)
    } catch (e) {
      throw new CLIError(`JSON parsing failed for file: ${inputPath}`, { exit: ExitCodes.InvalidInput })
    }
    if (schema) {
      await validateInput(jsonObj, schema, schemaPath)
    }

    return jsonObj as T
  }

  return null
}

export async function validateInput(input: unknown, schema: JSONSchema, schemaPath?: string): Promise<void> {
  const ajv = new Ajv({ allErrors: true })
  schema = await $RefParser.dereference(schemaPath || '.', schema, {})
  const valid = ajv.validate(schema, input) as boolean
  if (!valid) {
    throw new CLIError(`Input JSON file is not valid: ${ajv.errorsText()}`)
  }
}

export function saveOutputJson(outputPath: string | undefined, fileName: string, data: any): void {
  if (outputPath) {
    let outputFilePath = path.join(outputPath, fileName)
    let postfix = 0
    while (fs.existsSync(outputFilePath)) {
      fileName = fileName.replace(/(_[0-9]+)?\.json/, `_${++postfix}.json`)
      outputFilePath = path.join(outputPath, fileName)
    }
    saveOutputJsonToFile(outputFilePath, data)

    console.log(`${chalk.green('Output succesfully saved to:')} ${chalk.white(outputFilePath)}`)
  }
}

// Output as file:

export function saveOutputJsonToFile(outputFilePath: string, data: any): void {
  try {
    fs.writeFileSync(outputFilePath, JSON.stringify(data, null, 4))
  } catch (e) {
    throw new CLIError(`Could not save the output to: ${outputFilePath}. Check permissions...`, {
      exit: ExitCodes.FsOperationFailed,
    })
  }
}

export function ensureOutputFileIsWriteable(outputFilePath: string | undefined): void {
  if (outputFilePath === undefined) {
    return
  }

  if (path.extname(outputFilePath) !== '.json') {
    throw new CLIError(`Output path ${outputFilePath} is not a JSON file!`, { exit: ExitCodes.InvalidInput })
  }

  if (fs.existsSync(outputFilePath)) {
    // File already exists - warn the user and check it it's writeable
    console.warn(`WARNING: ${outputFilePath} already exists and it will get overriden!`)
    try {
      fs.accessSync(`${outputFilePath}`, fs.constants.W_OK)
    } catch (e) {
      throw new CLIError(`Output path ${outputFilePath} is not writeable!`, { exit: ExitCodes.InvalidInput })
    }
  } else {
    // File does not exist yet - check if the directory is writeable
    try {
      fs.accessSync(`${path.dirname(outputFilePath)}`, fs.constants.W_OK)
    } catch (e) {
      throw new CLIError(`Output directory ${path.dirname(outputFilePath)} is not writeable!`, {
        exit: ExitCodes.InvalidInput,
      })
    }
  }
}
