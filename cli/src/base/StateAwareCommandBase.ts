import fs from 'fs'
import path from 'path'
import ExitCodes from '../ExitCodes'
import { CLIError } from '@oclif/errors'
import lockFile from 'proper-lockfile'
import DefaultCommandBase from './DefaultCommandBase'
import os from 'os'
import _ from 'lodash'
import { WorkingGroups } from '../Types'

// Type for the state object (which is preserved as json in the state file)
type StateObject = {
  apiUri: string
  queryNodeUri: string
  defaultWorkingGroup: WorkingGroups
  metadataCache: Record<string, any>
}

// State object default values
const DEFAULT_STATE: StateObject = {
  apiUri: '',
  queryNodeUri: '',
  defaultWorkingGroup: WorkingGroups.StorageProviders,
  metadataCache: {},
}

// State file path (relative to getAppDataPath())
const STATE_FILE = '/state.json'

// Possible data directory access errors
enum DataDirErrorType {
  Init = 0,
  Read = 1,
  Write = 2,
}

/**
 * Abstract base class for commands that need to work with the preserved state.
 *
 * The preserved state is kept in a json file inside the data directory.
 * The state object contains all the information that needs to be preserved across sessions, ie. the default account
 * choosen by the user after executing account:choose command etc. (see "StateObject" type above).
 */
export default abstract class StateAwareCommandBase extends DefaultCommandBase {
  getAppDataPath(): string {
    const systemAppDataPath =
      process.env.APPDATA ||
      (process.platform === 'darwin'
        ? path.join(os.homedir(), '/Library/Application Support')
        : path.join(os.homedir(), '/.local/share'))
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const packageJson: { name?: string } = require('../../package.json')
    if (!packageJson || !packageJson.name) {
      throw new CLIError('Cannot get package name from package.json!')
    }
    return path.join(systemAppDataPath, _.kebabCase(packageJson.name))
  }

  getStateFilePath(): string {
    return path.join(this.getAppDataPath(), STATE_FILE)
  }

  private createDataDirFsError(errorType: DataDirErrorType, specificPath = '') {
    const actionStrs: { [x in DataDirErrorType]: string } = {
      [DataDirErrorType.Init]: 'initialize',
      [DataDirErrorType.Read]: 'read from',
      [DataDirErrorType.Write]: 'write into',
    }

    const errorMsg =
      `Unexpected error while trying to ${actionStrs[errorType]} the data directory.` +
      `(${path.join(this.getAppDataPath(), specificPath)})! Permissions issue?`

    return new CLIError(errorMsg, { exit: ExitCodes.FsOperationFailed })
  }

  createDataReadError(specificPath = ''): CLIError {
    return this.createDataDirFsError(DataDirErrorType.Read, specificPath)
  }

  createDataWriteError(specificPath = ''): CLIError {
    return this.createDataDirFsError(DataDirErrorType.Write, specificPath)
  }

  createDataDirInitError(specificPath = ''): CLIError {
    return this.createDataDirFsError(DataDirErrorType.Init, specificPath)
  }

  private initStateFs(): void {
    if (!fs.existsSync(this.getAppDataPath())) {
      fs.mkdirSync(this.getAppDataPath())
    }
    if (!fs.existsSync(this.getStateFilePath())) {
      fs.writeFileSync(this.getStateFilePath(), JSON.stringify(DEFAULT_STATE, null, 4))
    }
  }

  getPreservedState(): StateObject {
    let preservedState: StateObject
    try {
      // Use readFileSync instead of "require" in order to always get a "fresh" state
      preservedState = JSON.parse(fs.readFileSync(this.getStateFilePath()).toString()) as StateObject
    } catch (e) {
      throw this.createDataReadError()
    }
    // The state preserved in a file may be missing some required values ie.
    // if the user previously used the older version of the software.
    // That's why we combine it with default state before returing.
    return { ...DEFAULT_STATE, ...preservedState }
  }

  // Modifies preserved state. Uses file lock in order to avoid updating an older state.
  // (which could potentialy change between read and write operation)
  async setPreservedState(modifiedState: Partial<StateObject>): Promise<void> {
    const stateFilePath = this.getStateFilePath()
    const unlock = await lockFile.lock(stateFilePath)
    const oldState: StateObject = this.getPreservedState()
    const newState: StateObject = { ...oldState, ...modifiedState }
    try {
      fs.writeFileSync(stateFilePath, JSON.stringify(newState, null, 4))
    } catch (e) {
      await unlock()
      throw this.createDataWriteError()
    }
    await unlock()
  }

  async init() {
    await super.init()
    try {
      await this.initStateFs()
    } catch (e) {
      throw this.createDataDirInitError()
    }
  }
}
