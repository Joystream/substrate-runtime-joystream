import { cli, Table } from 'cli-ux'
import chalk from 'chalk'
import { MemberDetails, NameValueObj } from '../Types'
import { AccountId } from '@polkadot/types/interfaces'

export function displayHeader(caption: string, placeholderSign = '_', size = 50) {
  const singsPerSide: number = Math.floor((size - (caption.length + 2)) / 2)
  let finalStr = ''
  for (let i = 0; i < singsPerSide; ++i) finalStr += placeholderSign
  finalStr += ` ${caption} `
  while (finalStr.length < size) finalStr += placeholderSign

  process.stdout.write('\n' + chalk.bold.blueBright(finalStr) + '\n\n')
}

export function displayNameValueTable(rows: NameValueObj[]) {
  cli.table(
    rows,
    {
      name: { minWidth: 30, get: (row) => chalk.bold.white(row.name) },
      value: { get: (row) => chalk.white(row.value) },
    },
    { 'no-header': true }
  )
}

export function displayCollapsedRow(row: { [k: string]: string | number }) {
  const collapsedRow: NameValueObj[] = Object.keys(row).map((name) => ({
    name,
    value: typeof row[name] === 'string' ? (row[name] as string) : row[name].toString(),
  }))

  displayNameValueTable(collapsedRow)
}

export function displayCollapsedTable(rows: { [k: string]: string | number }[]) {
  for (const row of rows) displayCollapsedRow(row)
}

export function displayTable(rows: { [k: string]: string | number }[], cellHorizontalPadding = 0) {
  if (!rows.length) {
    return
  }
  const maxLength = (columnName: string) =>
    rows.reduce((maxLength, row) => {
      const val = row[columnName]
      const valLength = typeof val === 'string' ? val.length : val !== undefined ? val.toString().length : 0
      return Math.max(maxLength, valLength)
    }, columnName.length)
  const columnDef = (columnName: string) => ({
    header: columnName,
    get: (row: typeof rows[number]) => chalk.white(`${row[columnName]}`),
    minWidth: maxLength(columnName) + cellHorizontalPadding,
  })
  const columns: Table.table.Columns<{ [k: string]: string }> = {}
  Object.keys(rows[0]).forEach((columnName) => (columns[columnName] = columnDef(columnName)))
  cli.table(rows, columns)
}

export function toFixedLength(text: string, length: number, spacesOnLeft = false): string {
  if (text.length > length && length > 3) {
    return text.slice(0, length - 3) + '...'
  }
  while (text.length < length) {
    spacesOnLeft ? (text = ' ' + text) : (text += ' ')
  }

  return text
}

export function shortAddress(address: AccountId | string): string {
  return address.toString().substr(0, 6) + '...' + address.toString().substr(-6)
}

export function memberHandle(member: MemberDetails): string {
  return member.handle ? member.handle : member.membership.handle_hash.toHex().substr(0, 10) + '... (hash)'
}
