import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import CreateClassSchema from '@joystream/cd-schemas/schemas/extrinsics/CreateClass.schema.json'
import chalk from 'chalk'
import { JsonSchemaCustomPrompts, JsonSchemaPrompter } from '../../helpers/JsonSchemaPrompt'
import { CreateClass } from '@joystream/cd-schemas/types/extrinsics/CreateClass'
import { JSONSchema } from '@apidevtools/json-schema-ref-parser'

export default class UpdateClassPermissionsCommand extends ContentDirectoryCommandBase {
  static description = 'Update permissions in given class.'
  static args = [
    {
      name: 'className',
      required: false,
      description: 'Name or ID of the class (ie. Video)',
    },
  ]

  async run() {
    const account = await this.getRequiredSelectedAccount()
    await this.requireLead()

    let { className } = this.parse(UpdateClassPermissionsCommand).args

    if (className === undefined) {
      className = (await this.promptForClass()).name.toString()
    }

    const [classId, aClass] = await this.classEntryByNameOrId(className)
    const currentPermissions = aClass.class_permissions

    const customPrompts: JsonSchemaCustomPrompts = [
      ['class_permissions.maintainers', () => this.promptForCuratorGroups('Select class maintainers')],
    ]

    const prompter = new JsonSchemaPrompter<CreateClass>(
      CreateClassSchema as JSONSchema,
      { class_permissions: currentPermissions.toJSON() as CreateClass['class_permissions'] },
      customPrompts
    )

    const newPermissions = await prompter.promptSingleProp('class_permissions')

    await this.requestAccountDecoding(account)
    await this.sendAndFollowNamedTx(account, 'contentDirectory', 'updateClassPermissions', [
      classId,
      this.createType('Option<bool>', newPermissions.any_member || null),
      this.createType('Option<bool>', newPermissions.entity_creation_blocked || null),
      this.createType('Option<bool>', newPermissions.all_entity_property_values_locked || null),
      this.createType('Option<bool>', newPermissions.maintainers || null),
    ])

    console.log(chalk.green(`${chalk.white(className)} class permissions updated to:`))
    this.jsonPrettyPrint(JSON.stringify(newPermissions))
  }
}
