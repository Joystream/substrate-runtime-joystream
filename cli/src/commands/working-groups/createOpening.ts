import WorkingGroupsCommandBase from '../../base/WorkingGroupsCommandBase'
import { GroupMember } from '../../Types'
import chalk from 'chalk'
import { apiModuleByGroup } from '../../Api'
import { JsonSchemaPrompter } from '../../helpers/JsonSchemaPrompt'
import { JSONSchema } from '@apidevtools/json-schema-ref-parser'
import OpeningParamsSchema from '../../json-schemas/WorkingGroupOpening.schema.json'
import { WorkingGroupOpening as OpeningParamsJson } from '../../json-schemas/typings/WorkingGroupOpening.schema'
import { IOFlags, getInputJson, ensureOutputFileIsWriteable, saveOutputJsonToFile } from '../../helpers/InputOutput'
import ExitCodes from '../../ExitCodes'
import { flags } from '@oclif/command'
import { AugmentedSubmittables } from '@polkadot/api/types'
import { formatBalance } from '@polkadot/util'
import BN from 'bn.js'

const OPENING_STAKE = new BN(2000)

export default class WorkingGroupsCreateOpening extends WorkingGroupsCommandBase {
  static description = 'Create working group opening (requires lead access)'
  static flags = {
    ...WorkingGroupsCommandBase.flags,
    input: IOFlags.input,
    output: flags.string({
      char: 'o',
      required: false,
      description: 'Path to the file where the output JSON should be saved (this output can be then reused as input)',
    }),
    edit: flags.boolean({
      char: 'e',
      required: false,
      description:
        'If provided along with --input - launches in edit mode allowing to modify the input before sending the exstinsic',
      dependsOn: ['input'],
    }),
    dryRun: flags.boolean({
      required: false,
      description:
        'If provided along with --output - skips sending the actual extrinsic' +
        '(can be used to generate a "draft" which can be provided as input later)',
      dependsOn: ['output'],
    }),
  }

  createTxParams(
    openingParamsJson: OpeningParamsJson
  ): Parameters<AugmentedSubmittables<'promise'>['membershipWorkingGroup']['addOpening']> {
    return [
      openingParamsJson.description,
      'Regular',
      {
        stake_amount: openingParamsJson.stakingPolicy.amount,
        leaving_unstaking_period: openingParamsJson.stakingPolicy.unstakingPeriod,
      },
      // TODO: Proper bigint handling?
      openingParamsJson.rewardPerBlock?.toString() || null,
    ]
  }

  async promptForData(lead: GroupMember, rememberedInput?: OpeningParamsJson): Promise<OpeningParamsJson> {
    const openingDefaults = rememberedInput
    const openingPrompt = new JsonSchemaPrompter<OpeningParamsJson>(
      (OpeningParamsSchema as unknown) as JSONSchema,
      openingDefaults
    )
    const openingParamsJson = await openingPrompt.promptAll()

    return openingParamsJson
  }

  async getInputFromFile(filePath: string): Promise<OpeningParamsJson> {
    const inputParams = await getInputJson<OpeningParamsJson>(filePath, (OpeningParamsSchema as unknown) as JSONSchema)

    return inputParams as OpeningParamsJson
  }

  async promptForStakeTopUp(stakingAccount: string): Promise<void> {
    this.log(`You need to stake ${chalk.bold(formatBalance(OPENING_STAKE))} in order to create a new opening.`)

    const [balances] = await this.getApi().getAccountsBalancesInfo([stakingAccount])
    const missingBalance = OPENING_STAKE.sub(balances.availableBalance)
    if (missingBalance.gtn(0)) {
      await this.requireConfirmation(
        `Do you wish to transfer remaining ${chalk.bold(
          formatBalance(missingBalance)
        )} to your staking account? (${stakingAccount})`
      )
      const account = await this.promptForAccount('Choose account to transfer the funds from')
      await this.sendAndFollowNamedTx(await this.getDecodedPair(account), 'balances', 'transferKeepAlive', [
        stakingAccount,
        missingBalance,
      ])
    }
  }

  async run() {
    // lead-only gate
    const lead = await this.getRequiredLeadContext()

    const {
      flags: { input, output, edit, dryRun },
    } = this.parse(WorkingGroupsCreateOpening)

    ensureOutputFileIsWriteable(output)

    let tryAgain = false
    let rememberedInput: OpeningParamsJson | undefined
    do {
      if (edit) {
        rememberedInput = await this.getInputFromFile(input as string)
      }
      // Either prompt for the data or get it from input file
      const openingJson =
        !input || edit || tryAgain
          ? await this.promptForData(lead, rememberedInput)
          : await this.getInputFromFile(input)

      // Remember the provided/fetched data in a variable
      rememberedInput = openingJson

      await this.promptForStakeTopUp(lead.stakingAccount.toString())

      // Generate and ask to confirm tx params
      const txParams = this.createTxParams(openingJson)
      this.jsonPrettyPrint(JSON.stringify(txParams))
      const confirmed = await this.simplePrompt({
        type: 'confirm',
        message: 'Do you confirm these extrinsic parameters?',
      })
      if (!confirmed) {
        tryAgain = await this.simplePrompt({ type: 'confirm', message: 'Try again with remembered input?' })
        continue
      }

      // Save output to file
      if (output) {
        try {
          saveOutputJsonToFile(output, rememberedInput)
          this.log(chalk.green(`Output succesfully saved in: ${chalk.white(output)}!`))
        } catch (e) {
          this.warn(`Could not save output to ${output}!`)
        }
      }

      if (dryRun) {
        this.exit(ExitCodes.OK)
      }

      // Send the tx
      const txSuccess = await this.sendAndFollowNamedTx(
        await this.getDecodedPair(lead.roleAccount.toString()),
        apiModuleByGroup[this.group],
        'addOpening',
        txParams,
        true // warnOnly
      )

      // Display a success message on success or ask to try again on error
      if (txSuccess) {
        this.log(chalk.green('Opening succesfully created!'))
        tryAgain = false
      } else {
        tryAgain = await this.simplePrompt({ type: 'confirm', message: 'Try again with remembered input?' })
      }
    } while (tryAgain)
  }
}
