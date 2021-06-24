import ExitCodes from '../ExitCodes'
import Command from '@oclif/command'
import inquirer, { DistinctQuestion } from 'inquirer'
import chalk from 'chalk'
import inquirerDatepicker from 'inquirer-datepicker-prompt'

/**
 * Abstract base class for pretty much all commands
 * (prevents console.log from hanging the process and unifies the default exit code)
 */
export default abstract class DefaultCommandBase extends Command {
  protected indentGroupsOpened = 0
  protected jsonPrettyIdent = ''

  openIndentGroup() {
    console.group()
    ++this.indentGroupsOpened
  }

  closeIndentGroup() {
    console.groupEnd()
    --this.indentGroupsOpened
  }

  async simplePrompt(question: DistinctQuestion) {
    const { result } = await inquirer.prompt([
      {
        ...question,
        name: 'result',
        // prefix = 2 spaces for each group - 1 (because 1 is always added by default)
        prefix: Array.from(new Array(this.indentGroupsOpened))
          .map(() => '  ')
          .join('')
          .slice(1),
      },
    ])

    return result
  }

  async requireConfirmation(
    message = 'Are you sure you want to execute this action?',
    defaultVal = false
  ): Promise<void> {
    const { confirmed } = await inquirer.prompt([{ type: 'confirm', name: 'confirmed', message, default: defaultVal }])
    if (!confirmed) this.exit(ExitCodes.OK)
  }

  private jsonPrettyIndented(line: string) {
    return `${this.jsonPrettyIdent}${line}`
  }

  private jsonPrettyOpen(char: '{' | '[') {
    this.jsonPrettyIdent += '    '
    return chalk.gray(char) + '\n'
  }

  private jsonPrettyClose(char: '}' | ']') {
    this.jsonPrettyIdent = this.jsonPrettyIdent.slice(0, -4)
    return this.jsonPrettyIndented(chalk.gray(char))
  }

  private jsonPrettyKeyVal(key: string, val: any): string {
    return this.jsonPrettyIndented(chalk.white(`${key}: ${this.jsonPrettyAny(val)}`))
  }

  private jsonPrettyObj(obj: { [key: string]: any }): string {
    return (
      this.jsonPrettyOpen('{') +
      Object.keys(obj)
        .map((k) => this.jsonPrettyKeyVal(k, obj[k]))
        .join(',\n') +
      '\n' +
      this.jsonPrettyClose('}')
    )
  }

  private jsonPrettyArr(arr: any[]): string {
    return (
      this.jsonPrettyOpen('[') +
      arr.map((v) => this.jsonPrettyIndented(this.jsonPrettyAny(v))).join(',\n') +
      '\n' +
      this.jsonPrettyClose(']')
    )
  }

  private jsonPrettyAny(val: any): string {
    if (Array.isArray(val)) {
      return this.jsonPrettyArr(val)
    } else if (typeof val === 'object' && val !== null) {
      return this.jsonPrettyObj(val)
    } else if (typeof val === 'string') {
      return chalk.green(`"${val}"`)
    }

    // Number, boolean etc.
    return chalk.cyan(val)
  }

  jsonPrettyPrint(json: string) {
    try {
      const parsed = JSON.parse(json)
      console.log(this.jsonPrettyAny(parsed))
    } catch (e) {
      console.log(this.jsonPrettyAny(json))
    }
  }

  async finally(err: any) {
    // called after run and catch regardless of whether or not the command errored
    // We'll force exit here, in case there is no error, to prevent console.log from hanging the process
    if (!err) this.exit(ExitCodes.OK)
    super.finally(err)
  }

  async init() {
    inquirer.registerPrompt('datetime', inquirerDatepicker)
  }
}
