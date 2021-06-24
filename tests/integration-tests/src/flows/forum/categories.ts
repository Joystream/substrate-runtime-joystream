import { FlowProps } from '../../Flow'
import Debugger from 'debug'
import { FixtureRunner } from '../../Fixture'
import {
  CategoryParams,
  CreateCategoriesFixture,
  CategoryStatusUpdate,
  UpdateCategoriesStatusFixture,
  RemoveCategoriesFixture,
  CategoryModeratorStatusUpdate,
  UpdateCategoryModeratorsFixture,
} from '../../fixtures/forum'
import { HireWorkersFixture } from '../../fixtures/workingGroups/HireWorkersFixture'

export default async function categories({ api, query }: FlowProps): Promise<void> {
  const debug = Debugger(`flow:cateogries`)
  debug('Started')
  api.enableDebugTxLogs()

  // Create root categories
  const categories: CategoryParams[] = [
    { title: 'General', description: 'General stuff' },
    { title: 'Working Groups', description: 'Working groups related discussions' },
  ]

  const createCategoriesFixture = new CreateCategoriesFixture(api, query, categories)
  const createCategoriesRunner = new FixtureRunner(createCategoriesFixture)
  await createCategoriesRunner.run()
  const rootCategoryIds = createCategoriesFixture.getCreatedCategoriesIds()
  const workingGroupsCategoryId = rootCategoryIds[1]

  // Create subcategories
  const workingGroupsSubcategories: CategoryParams[] = [
    {
      title: 'Forum Working Group',
      description: 'Forum Working Group related discussions',
      parentId: workingGroupsCategoryId,
    },
    {
      title: 'Storage Working Group',
      description: 'Storage Working Group related discussions',
      parentId: workingGroupsCategoryId,
    },
    {
      title: 'Membership Working Group',
      description: 'Membership Working Group related discussions',
      parentId: workingGroupsCategoryId,
    },
  ]
  const createSubcategoriesFixture = new CreateCategoriesFixture(api, query, workingGroupsSubcategories)
  const createSubcategoriesRunner = new FixtureRunner(createSubcategoriesFixture)
  await createSubcategoriesRunner.run()
  const subcategoryIds = createSubcategoriesFixture.getCreatedCategoriesIds()

  await Promise.all([createCategoriesRunner.runQueryNodeChecks(), createSubcategoriesRunner.runQueryNodeChecks()])

  // Create moderators and perform status updates
  const createModeratorsFixture = new HireWorkersFixture(api, query, 'forumWorkingGroup', subcategoryIds.length + 1)
  await new FixtureRunner(createModeratorsFixture).run()
  const moderatorIds = createModeratorsFixture.getCreatedWorkerIds()

  const moderatorUpdates: CategoryModeratorStatusUpdate[] = subcategoryIds.reduce(
    (updates, categoryId, i) =>
      (updates = updates.concat([
        { categoryId, moderatorId: moderatorIds[i], canModerate: true },
        { categoryId, moderatorId: moderatorIds[i + 1], canModerate: true },
        { categoryId, moderatorId: moderatorIds[i + 1], canModerate: false },
      ])),
    [] as CategoryModeratorStatusUpdate[]
  )
  const updateCategoryModeratorsFixture = new UpdateCategoryModeratorsFixture(api, query, moderatorUpdates)
  const updateCategoryModeratorsRunner = new FixtureRunner(updateCategoryModeratorsFixture)
  await updateCategoryModeratorsRunner.run()

  // Update archival status
  const categoryUpdates: CategoryStatusUpdate[] = [
    { categoryId: subcategoryIds[0], archived: true },
    { categoryId: subcategoryIds[1], archived: true },
    { categoryId: subcategoryIds[1], archived: false },
  ]

  const categoryUpdatesFixture = new UpdateCategoriesStatusFixture(api, query, categoryUpdates)
  const categoryUpdatesRunner = new FixtureRunner(categoryUpdatesFixture)
  await categoryUpdatesRunner.run()

  // Run compound query node checks
  await Promise.all([updateCategoryModeratorsFixture.runQueryNodeChecks(), categoryUpdatesRunner.runQueryNodeChecks()])

  // Remove categories (make sure subcategories are removed first)
  const removeSubcategoriesFixture = new RemoveCategoriesFixture(
    api,
    query,
    subcategoryIds.map((categoryId) => ({ categoryId }))
  )
  const removeRootCategoriesFixture = new RemoveCategoriesFixture(
    api,
    query,
    rootCategoryIds.map((categoryId) => ({ categoryId }))
  )
  const removeSubcategoriesRunner = new FixtureRunner(removeSubcategoriesFixture)
  const removeRootCategoriesRunner = new FixtureRunner(removeRootCategoriesFixture)
  await removeSubcategoriesRunner.run()
  await removeRootCategoriesRunner.run()
  await Promise.all([removeSubcategoriesRunner.runQueryNodeChecks(), removeRootCategoriesRunner.runQueryNodeChecks()])

  debug('Done')
}
