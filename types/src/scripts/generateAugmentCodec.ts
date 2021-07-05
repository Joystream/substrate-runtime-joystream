/**
 * Generates" augment files in /augment-codec based on already generated ones in /augment via @polkadot/typegen:
 * 1. Creates augment-codec/all.ts file that exports all @joystream/types separately
 * 2. Copies augment-* files from /augment to /augment-codes. Since those files import types from ./all,
 * the imports in /augment-codec will be overriden with our custom codec/class types
 */

import path from 'path'
import fs from 'fs'
// Types by module:
import common from '../common'
import members from '../members'
import council from '../council'
import roles from '../roles'
import forum from '../forum'
import stake from '../stake'
import mint from '../mint'
import recurringRewards from '../recurring-rewards'
import hiring from '../hiring'
import contentWorkingGroup from '../content-working-group'
import workingGroup from '../working-group'
import discovery from '../discovery'
import media from '../media'
import proposals from '../proposals'
import contentDirectory from '../content-directory'
import storage from '../storage'

const AUGMENT_INTERFACES_PATH = path.join(__dirname, '../../augment')
const AUGMENT_CODEC_PATH = path.join(__dirname, '../../augment-codec')
const EXPORT_ALL_TYPES_FILE_PATH = path.join(AUGMENT_CODEC_PATH, 'all.ts')
const RELATIVE_TYPES_ROOT_PATH = '..' // @joystream/types/index path relative to AUGMENT_CODEC_PATH

const typesByModule = {
  'common': common,
  'members': members,
  'council': council,
  'roles': roles,
  'forum': forum,
  'stake': stake,
  'mint': mint,
  'recurring-rewards': recurringRewards,
  'hiring': hiring,
  'content-working-group': contentWorkingGroup,
  'working-group': workingGroup,
  'discovery': discovery,
  'media': media,
  'proposals': proposals,
  'content-directory': contentDirectory,
  'storage': storage,
}

type Imports = { [moduleName: string]: string[] }

function generateExportAllFile(filePath: string) {
  const imports: Imports = {}
  const exports: string[] = []

  Object.entries(typesByModule).forEach(([moduleName, types]) => {
    Object.entries(types).forEach(([typeName, codecOrName]) => {
      if (typeof codecOrName === 'function') {
        const constructorName = codecOrName.name
        if (!constructorName) {
          throw new Error(`Codec constructor doesn't have a name: ${typeName}`)
        }
        const normalizedTypeName = typeName.replace(/[^A-Za-z0-9_]/g, '_')
        // Add "as" if necessary
        const importStatement =
          constructorName === normalizedTypeName ? normalizedTypeName : `${constructorName} as ${normalizedTypeName}`
        !imports[moduleName] ? (imports[moduleName] = [importStatement]) : imports[moduleName].push(importStatement)
        exports.push(normalizedTypeName)
      } else {
        throw new Error(
          'All types exposed to registry by a module should have a corresponding class!\n' +
            `Class not found for type: ${typeName} in module ${moduleName}`
        )
      }
    })
  })

  const fileLines: string[] = []
  fileLines.push('// This file was automatically generated via generate:augment-codec')
  for (const [module, importStatements] of Object.entries(imports)) {
    fileLines.push(`import { ${importStatements.join(', ')} } from '${RELATIVE_TYPES_ROOT_PATH}/${module}';`)
  }
  fileLines.push('')
  fileLines.push(`export { ${exports.join(', ')} };`)

  fs.writeFileSync(filePath, fileLines.join('\n'))
}

// ACTUAL SCRIPT:
// Generate /augment-codec/all.ts file exporting all types ("augment-*" files will import from it)
generateExportAllFile(EXPORT_ALL_TYPES_FILE_PATH)
console.log(`Generated all types export file in ${EXPORT_ALL_TYPES_FILE_PATH}\n`)
// Copy augment-* files from /augment to /augment-codec
let copiedFilesCounter = 0
fs.readdirSync(AUGMENT_INTERFACES_PATH).forEach((fileName) => {
  if (fileName.startsWith('augment-')) {
    const src = path.join(AUGMENT_INTERFACES_PATH, fileName)
    const dest = path.join(AUGMENT_CODEC_PATH, fileName)
    // Copy the file
    fs.copyFileSync(src, dest)
    console.log(`Copied file!\nFrom: ${src}\nTo: ${dest}\n`)
    ++copiedFilesCounter
  }
})

if (!copiedFilesCounter) {
  console.log('No files were copied! Did you forget to run generate:augment first?')
}
