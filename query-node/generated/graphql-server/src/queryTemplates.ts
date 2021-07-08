import { IQueryTemplate, queryTemplateUtils } from '@apollographql/graphql-playground-react/lib/components/Playground/QueryTemplates/templateUtils'

// fields that will be ignored by autofill
const commonIgnoredFields = [
  'deletedAt',
  'createdById',
  'updatedById',
  'deletedById',
  'version',
]

const dataObjectIgnoredFields = [
  ...commonIgnoredFields,

  // dataObject's `owner` is problematic because it's variant and will need some special handling
  'owner',
]

const exampleDate = `"2018-01-31 23:59"`

export const queryTemplates: IQueryTemplate[] = [
  ...queryTemplateUtils.getOneGetAllTemplates('video', 'videos', 'videos', commonIgnoredFields),
  {
    title: 'Featured videos',
    description: 'Get all featured videos.',
    ignoredFields: commonIgnoredFields,
    query: `query {
      ${queryTemplateUtils.descriptionMarker}
      videos(where: { isFeatured_eq: true }) { ${queryTemplateUtils.allPropsMarker} }
    }`,
  }, {
    title: 'All recent videos',
    description: 'Get all videos after created or updated after the given date.',
    ignoredFields: commonIgnoredFields,
    query: `query {
      ${queryTemplateUtils.descriptionMarker}
      videos(where: {
        createdAt_gt: ${exampleDate},
        updatedAt_gt: ${exampleDate},
      }) { ${queryTemplateUtils.allPropsMarker} }
    }`,
  },

  ...queryTemplateUtils.getOneGetAllTemplates('video category', 'video categories', 'videoCategories', commonIgnoredFields),
  {
    title: `All videos in category`,
    description: `Get all videos associated with the given video category.`,
    ignoredFields: commonIgnoredFields,
    query: `query {
      ${queryTemplateUtils.descriptionMarker}
      videos(where: { categoryId_eq: 1 }) { ${queryTemplateUtils.allPropsMarker} }
    }`,
  },
  ...queryTemplateUtils.getOneGetAllTemplates('channel', 'channels', 'channels', commonIgnoredFields),
  ...queryTemplateUtils.getOneGetAllTemplates('channel category', 'channels categories', 'channelCategories', commonIgnoredFields),

  {
    title: `Channel's videos`,
    description: `Get all videos associated with the given channel.`,
    ignoredFields: commonIgnoredFields,
    query: `query {
      ${queryTemplateUtils.descriptionMarker}
      videos(where: { channelId_eq: 1 }) { ${queryTemplateUtils.allPropsMarker} }
    }`,
  },

  ...queryTemplateUtils.getOneGetAllTemplates('asset', 'assets', 'dataObjects', dataObjectIgnoredFields),
  ...queryTemplateUtils.getOneGetAllTemplates('membership', 'memberships', 'memberships', commonIgnoredFields),

  ...queryTemplateUtils.getOneGetAllTemplates('curator group', 'curator groups', 'curatorGroups', commonIgnoredFields),
  ...queryTemplateUtils.getOneGetAllTemplates('worker', 'workers', 'workers', commonIgnoredFields),
].map(queryTemplateUtils.formatQuery)
