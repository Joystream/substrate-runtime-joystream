import fs from 'fs'
import path from 'path'
import inquirer from 'inquirer'
import ExitCodes from '../ExitCodes'
import { CLIError } from '@oclif/errors'
import ApiCommandBase from './ApiCommandBase'
import { Keyring } from '@polkadot/api'
import { formatBalance } from '@polkadot/util'
import { MemberDetails, NamedKeyringPair } from '../Types'
import { DeriveBalancesAll } from '@polkadot/api-derive/types'
import { memberHandle, toFixedLength } from '../helpers/display'
import { AccountId, MemberId } from '@joystream/types/common'
import { KeyringPair, KeyringInstance, KeyringOptions } from '@polkadot/keyring/types'
import { KeypairType } from '@polkadot/util-crypto/types'
import createDevelopmentKeyring from '@polkadot/keyring/testing'
import chalk from 'chalk'
import { mnemonicGenerate } from '@polkadot/util-crypto'
import { validateAddress } from '../helpers/validation'
import slug from 'slug'
import { Membership } from '@joystream/types/members'
import BN from 'bn.js'

const ACCOUNTS_DIRNAME = 'accounts'
export const DEFAULT_ACCOUNT_TYPE = 'sr25519'
export const KEYRING_OPTIONS: KeyringOptions = {
  type: DEFAULT_ACCOUNT_TYPE,
}
export const STAKING_ACCOUNT_CANDIDATE_STAKE = new BN(200)

/**
 * Abstract base class for account-related commands.
 *
 * All the accounts available in the CLI are stored in the form of json backup files inside:
 * { APP_DATA_PATH }/{ ACCOUNTS_DIRNAME } (ie. ~/.local/share/joystream-cli/accounts on Ubuntu)
 * Where: APP_DATA_PATH is provided by StateAwareCommandBase and ACCOUNTS_DIRNAME is a const (see above).
 */
export default abstract class AccountsCommandBase extends ApiCommandBase {
  private keyring: KeyringInstance | undefined

  getKeyring(): KeyringInstance {
    if (!this.keyring) {
      this.error('Trying to access Keyring before AccountsCommandBase initialization', {
        exit: ExitCodes.UnexpectedException,
      })
    }
    return this.keyring
  }

  isKeyAvailable(key: AccountId | string): boolean {
    return this.getKeyring()
      .getPairs()
      .some((p) => p.address === key.toString())
  }

  getAccountsDirPath(): string {
    return path.join(this.getAppDataPath(), ACCOUNTS_DIRNAME)
  }

  getAccountFileName(accountName: string): string {
    return `${slug(accountName)}.json`
  }

  getAccountFilePath(accountName: string): string {
    return path.join(this.getAccountsDirPath(), this.getAccountFileName(accountName))
  }

  isAccountNameTaken(accountName: string): boolean {
    return this.getPairs().some((p) => this.getAccountFileName(p.meta.name) === this.getAccountFileName(accountName))
  }

  private initAccountsFs(): void {
    if (!fs.existsSync(this.getAccountsDirPath())) {
      fs.mkdirSync(this.getAccountsDirPath())
    }
  }

  async createAccount(
    name?: string,
    masterKey?: KeyringPair,
    password?: string,
    type?: KeypairType
  ): Promise<NamedKeyringPair> {
    while (!name || this.isAccountNameTaken(name)) {
      if (name) {
        this.warn(`Account ${chalk.white(name)} already exists... Try different name`)
      }
      name = await this.simplePrompt({ message: 'New account name' })
    }

    if (!masterKey) {
      const keyring = new Keyring(KEYRING_OPTIONS)
      const mnemonic = mnemonicGenerate()
      keyring.addFromMnemonic(mnemonic, { name, whenCreated: Date.now() }, type)
      masterKey = keyring.getPairs()[0]
      this.log(chalk.white(`${chalk.bold('New account memonic: ')}${mnemonic}`))
    } else {
      const existingAcc = this.getPairs().find((p) => p.address === masterKey!.address)
      if (existingAcc) {
        this.error(`Account with this key already exists (${chalk.white(existingAcc.meta.name)})`, {
          exit: ExitCodes.InvalidInput,
        })
      }
      await this.requestPairDecoding(masterKey, 'Current account password')
      if (!masterKey.meta.name) {
        masterKey.meta.name = name
      }
    }

    while (password === undefined) {
      password = await this.promptForPassword("Set new account's password")
      const password2 = await this.promptForPassword("Confirm new account's password")

      if (password !== password2) {
        this.warn('Passwords are not the same!')
        password = undefined
      }
    }
    if (!password) {
      this.warn('Using empty password is not recommended!')
    }

    const destPath = this.getAccountFilePath(name)
    fs.writeFileSync(destPath, JSON.stringify(masterKey.toJson(password)))

    this.getKeyring().addPair(masterKey)

    this.log(chalk.greenBright(`\nNew account succesfully created!`))

    return masterKey as NamedKeyringPair
  }

  fetchAccountFromJsonFile(jsonBackupFilePath: string): NamedKeyringPair {
    if (!fs.existsSync(jsonBackupFilePath)) {
      throw new CLIError('Input file does not exist!', { exit: ExitCodes.FileNotFound })
    }
    if (path.extname(jsonBackupFilePath) !== '.json') {
      throw new CLIError('Invalid input file: File extension should be .json', { exit: ExitCodes.InvalidFile })
    }
    let accountJsonObj: any
    try {
      accountJsonObj = require(jsonBackupFilePath)
    } catch (e) {
      throw new CLIError('Provided backup file is not valid or cannot be accessed', { exit: ExitCodes.InvalidFile })
    }
    if (typeof accountJsonObj !== 'object' || accountJsonObj === null) {
      throw new CLIError('Provided backup file is not valid', { exit: ExitCodes.InvalidFile })
    }

    if (!accountJsonObj.meta) accountJsonObj.meta = {}
    // Normalize the CLI account name based on file name
    // (makes sure getFilePath(name) will always point to the correct file, preserving backward-compatibility
    // with older CLI versions)
    accountJsonObj.meta.name = path.basename(jsonBackupFilePath, '.json')

    const keyring = new Keyring(KEYRING_OPTIONS)
    let account: NamedKeyringPair
    try {
      // Try adding and retrieving the keys in order to validate that the backup file is correct
      keyring.addFromJson(accountJsonObj)
      account = keyring.getPair(accountJsonObj.address) as NamedKeyringPair // We can be sure it's named, because we forced it before
    } catch (e) {
      throw new CLIError(`Provided backup file is not valid (${e.message})`, { exit: ExitCodes.InvalidFile })
    }

    return account
  }

  private fetchAccountOrNullFromFile(jsonFilePath: string): NamedKeyringPair | null {
    try {
      return this.fetchAccountFromJsonFile(jsonFilePath)
    } catch (e) {
      // Here in case of a typical CLIError we just return null (otherwise we throw)
      if (!(e instanceof CLIError)) throw e
      return null
    }
  }

  fetchAccounts(): NamedKeyringPair[] {
    let files: string[] = []
    const accountDir = this.getAccountsDirPath()
    try {
      files = fs.readdirSync(accountDir)
    } catch (e) {
      // Do nothing
    }

    // We have to assert the type, because TS is not aware that we're filtering out the nulls at the end
    return files
      .map((fileName) => {
        const filePath = path.join(accountDir, fileName)
        return this.fetchAccountOrNullFromFile(filePath)
      })
      .filter((account) => account !== null) as NamedKeyringPair[]
  }

  getPairs(includeDevAccounts = true): NamedKeyringPair[] {
    return this.getKeyring()
      .getPairs()
      .filter((p) => includeDevAccounts || !p.meta.isTesting) as NamedKeyringPair[]
  }

  getPair(key: string): NamedKeyringPair {
    return this.getKeyring().getPair(key) as NamedKeyringPair
  }

  async getDecodedPair(key: string): Promise<NamedKeyringPair> {
    const pair = this.getPair(key)

    return (await this.requestPairDecoding(pair)) as NamedKeyringPair
  }

  async requestPairDecoding(pair: KeyringPair, message?: string): Promise<KeyringPair> {
    // Skip if pair already unlocked
    if (!pair.isLocked) {
      return pair
    }

    // First - try decoding using empty string
    try {
      pair.decodePkcs8('')
      return pair
    } catch (e) {
      // Continue...
    }

    let isPassValid = false
    while (!isPassValid) {
      try {
        const password = await this.promptForPassword(
          message || `Enter ${pair.meta.name ? pair.meta.name : pair.address} account password`
        )
        pair.decodePkcs8(password)
        isPassValid = true
      } catch (e) {
        this.warn('Invalid password... Try again.')
      }
    }

    return pair
  }

  initKeyring(): void {
    this.keyring = this.getApi().isDevelopment
      ? createDevelopmentKeyring(KEYRING_OPTIONS)
      : new Keyring(KEYRING_OPTIONS)
    const accounts = this.fetchAccounts()
    accounts.forEach((a) => this.getKeyring().addPair(a))
  }

  async promptForPassword(message = "Your account's password"): Promise<string> {
    const { password } = await inquirer.prompt([
      {
        name: 'password',
        type: 'password',
        message,
      },
    ])

    return password
  }

  async promptForAccount(
    message = 'Select an account',
    createIfUnavailable = true,
    includeDevAccounts = true,
    showBalances = true
  ): Promise<string> {
    const pairs = this.getPairs(includeDevAccounts)

    if (!pairs.length) {
      this.warn('No accounts available!')
      if (createIfUnavailable) {
        await this.requireConfirmation('Do you want to create a new account?', true)
        pairs.push(await this.createAccount())
      } else {
        this.exit()
      }
    }

    let balances: DeriveBalancesAll[] = []
    if (showBalances) {
      balances = await this.getApi().getAccountsBalancesInfo(pairs.map((p) => p.address))
    }

    const longestNameLen: number = pairs.reduce((prev, curr) => Math.max(curr.meta.name.length, prev), 0)
    const nameColLength: number = Math.min(longestNameLen + 1, 20)
    const chosenKey = await this.simplePrompt({
      message,
      type: 'list',
      choices: pairs.map((p, i) => ({
        name:
          `${toFixedLength(p.meta.name, nameColLength)} | ` +
          `${p.address} | ` +
          ((showBalances || '') &&
            `${formatBalance(balances[i].availableBalance)} / ` + `${formatBalance(balances[i].votingBalance)}`),
        value: p.address,
      })),
    })

    return chosenKey
  }

  promptForCustomAddress(): Promise<string> {
    return this.simplePrompt({
      message: 'Provide custom address',
      validate: (a) => validateAddress(a),
    })
  }

  async promptForAnyAddress(message = 'Select an address'): Promise<string> {
    const type: 'available' | 'new' | 'custom' = await this.simplePrompt({
      message,
      type: 'list',
      choices: [
        { name: 'Available account', value: 'available' },
        { name: 'New account', value: 'new' },
        { name: 'Custom address', value: 'custom' },
      ],
    })

    if (type === 'available') {
      return this.promptForAccount()
    } else if (type === 'new') {
      return (await this.createAccount()).address
    } else {
      return this.promptForCustomAddress()
    }
  }

  async getRequiredMemberContext(): Promise<MemberDetails> {
    // TODO: Limit only to a set of members provided by the user?
    const allMembers = await this.getApi().allMembers()
    const availableMembers = await Promise.all(
      allMembers
        .filter(([, m]) => this.isKeyAvailable(m.controller_account.toString()))
        .map(([id, m]) => this.getApi().memberDetails(id, m))
    )

    if (!availableMembers.length) {
      this.error('No member controller key available!', { exit: ExitCodes.AccessDenied })
    } else if (availableMembers.length === 1) {
      return availableMembers[0]
    } else {
      return this.promptForMember(availableMembers, 'Choose member context')
    }
  }

  async promptForMember(availableMembers: MemberDetails[], message = 'Choose a member'): Promise<MemberDetails> {
    const memberIndex = await this.simplePrompt({
      type: 'list',
      message,
      choices: availableMembers.map((m, i) => ({
        name: memberHandle(m),
        value: i,
      })),
    })

    return availableMembers[memberIndex]
  }

  async promptForStakingAccount(stakeValue: BN, memberId: MemberId, member: Membership): Promise<string> {
    this.log(`Required stake: ${formatBalance(stakeValue)}`)
    let stakingAccount: string
    while (true) {
      stakingAccount = await this.promptForAnyAddress('Choose staking account')
      const { balances } = await this.getApi().getAccountSummary(stakingAccount)
      const stakingStatus = await this.getApi().stakingAccountStatus(stakingAccount)

      if (balances.lockedBalance.gtn(0)) {
        this.warn('This account is already used for other staking purposes, choose different account...')
        continue
      }

      if (stakingStatus && !stakingStatus.member_id.eq(memberId)) {
        this.warn('This account is already used as staking accout by other member, choose different account...')
        continue
      }

      let additionalStakingAccountCosts = new BN(0)
      if (!stakingStatus || (stakingStatus && stakingStatus.confirmed.isFalse)) {
        if (!this.isKeyAvailable(stakingAccount)) {
          this.warn(
            'Account is not a confirmed staking account and cannot be directly accessed via CLI, choose different account...'
          )
          continue
        }
        this.warn(
          `This account is not a confirmed staking account. ` +
            `Additional funds (fees) may be required to set it as a staking account.`
        )
        if (!stakingStatus) {
          additionalStakingAccountCosts = await this.getApi().estimateFee(
            await this.getDecodedPair(stakingAccount),
            this.getOriginalApi().tx.members.addStakingAccountCandidate(memberId)
          )
          additionalStakingAccountCosts = additionalStakingAccountCosts.add(STAKING_ACCOUNT_CANDIDATE_STAKE)
        }
      }

      const requiredStakingAccountBalance = stakeValue.add(additionalStakingAccountCosts)
      const missingStakingAccountBalance = requiredStakingAccountBalance.sub(balances.availableBalance)
      if (missingStakingAccountBalance.gtn(0)) {
        this.warn(
          `Not enough available staking account balance! Missing: ${chalk.cyan(
            formatBalance(missingStakingAccountBalance)
          )}.` +
            (additionalStakingAccountCosts.gtn(0)
              ? ` (includes ${formatBalance(
                  additionalStakingAccountCosts
                )} which is a required fee and candidate stake for adding a new staking account)`
              : '')
        )
        const transferTokens = await this.simplePrompt({
          type: 'confirm',
          message: `Do you want to transfer ${chalk.cyan(
            formatBalance(missingStakingAccountBalance)
          )} from another account?`,
        })
        if (transferTokens) {
          const key = await this.promptForAccount('Choose source account')
          await this.sendAndFollowNamedTx(await this.getDecodedPair(key), 'balances', 'transferKeepAlive', [
            stakingAccount,
            missingStakingAccountBalance,
          ])
        } else {
          continue
        }
      }

      if (!stakingStatus) {
        await this.sendAndFollowNamedTx(
          await this.getDecodedPair(stakingAccount),
          'members',
          'addStakingAccountCandidate',
          [memberId]
        )
      }

      if (!stakingStatus || stakingStatus.confirmed.isFalse) {
        await this.sendAndFollowNamedTx(
          await this.getDecodedPair(member.controller_account.toString()),
          'members',
          'confirmStakingAccount',
          [memberId, stakingAccount]
        )
      }

      break
    }

    return stakingAccount
  }

  async init() {
    await super.init()
    try {
      this.initAccountsFs()
    } catch (e) {
      throw this.createDataDirInitError()
    }
    await this.initKeyring()
  }
}
