import ContentDirectoryCommandBase from '../../base/ContentDirectoryCommandBase'
import { VideoEntity } from '@joystream/cd-schemas/types/entities'
import { InputParser, ExtrinsicsHelper } from '@joystream/cd-schemas'
import { FlattenRelations } from '@joystream/cd-schemas/types/utility'
import { flags } from '@oclif/command'
import { createType } from '@joystream/types'

export default class SetFeaturedVideosCommand extends ContentDirectoryCommandBase {
  static description = 'Set currently featured videos (requires lead/maintainer access).'
  static args = [
    {
      name: 'videoIds',
      required: true,
      description: 'Comma-separated video ids',
    },
  ]

  static flags = {
    add: flags.boolean({
      description: 'If provided - currently featured videos will not be removed.',
      required: false,
    }),
  }

  async run() {
    const account = await this.getRequiredSelectedAccount()
    let actor = createType('Actor', { Lead: null })
    try {
      await this.getRequiredLeadContext()
    } catch (e) {
      actor = await this.getCuratorContext(['FeaturedVideo'])
    }

    await this.requestAccountDecoding(account)

    const {
      args: { videoIds },
      flags: { add },
    } = this.parse(SetFeaturedVideosCommand)

    const ids: number[] = videoIds.split(',').map((id: string) => parseInt(id))

    const videos: [number, FlattenRelations<VideoEntity>][] = (
      await Promise.all(ids.map((id) => this.getAndParseKnownEntity<VideoEntity>(id, 'Video')))
    ).map((video, index) => [ids[index], video])

    this.log(
      `Featured videos that will ${add ? 'be added to' : 'replace'} existing ones:`,
      videos.map(([id, { title }]) => ({ id, title }))
    )

    await this.requireConfirmation('Do you confirm the provided input?')

    if (!add) {
      const currentlyFeaturedIds = (await this.entitiesByClassAndOwner('FeaturedVideo')).map(([id]) => id.toNumber())
      const removeTxs = currentlyFeaturedIds.map((id) =>
        this.getOriginalApi().tx.contentDirectory.removeEntity(actor, id)
      )

      if (currentlyFeaturedIds.length) {
        this.log(`Removing existing FeaturedVideo entities (${currentlyFeaturedIds.join(', ')})...`)

        const txHelper = new ExtrinsicsHelper(this.getOriginalApi())
        await txHelper.sendAndCheck(account, removeTxs, 'The removal of existing FeaturedVideo entities failed')
      }
    }

    this.log('Adding new FeaturedVideo entities...')
    const featuredVideoEntries = videos.map(([id]) => ({ video: id }))
    const inputParser = InputParser.createWithKnownSchemas(this.getOriginalApi(), [
      {
        className: 'FeaturedVideo',
        entries: featuredVideoEntries,
      },
    ])
    const operations = await inputParser.getEntityBatchOperations()
    await this.sendAndFollowNamedTx(account, 'contentDirectory', 'transaction', [actor, operations])
  }
}
