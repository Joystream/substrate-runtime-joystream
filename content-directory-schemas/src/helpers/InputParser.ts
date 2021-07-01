import { AddClassSchema, Property } from '../../types/extrinsics/AddClassSchema'
import { createType } from '@joystream/types'
import {
  InputEntityValuesMap,
  ClassId,
  OperationType,
  ParametrizedPropertyValue,
  PropertyId,
  PropertyType,
  EntityId,
  Entity,
  ParametrizedClassPropertyValue,
  InputPropertyValue,
} from '@joystream/types/content-directory'
import { blake2AsHex } from '@polkadot/util-crypto'
import { isSingle, isReference } from './propertyType'
import { ApiPromise } from '@polkadot/api'
import { JoyBTreeSet } from '@joystream/types/common'
import { CreateClass } from '../../types/extrinsics/CreateClass'
import { EntityBatch } from '../../types/EntityBatch'
import { getInitializationInputs, getInputs } from './inputs'

type SimpleEntityValue = string | boolean | number | string[] | boolean[] | number[] | undefined | null
// Input without "new" or "extising" keywords
type SimpleEntityInput = { [K: string]: SimpleEntityValue }

export class InputParser {
  private api: ApiPromise
  private classInputs: CreateClass[]
  private schemaInputs: AddClassSchema[]
  private batchInputs: EntityBatch[]
  private createEntityOperations: OperationType[] = []
  private addSchemaToEntityOprations: OperationType[] = []
  private updateEntityPropertyValuesOperations: OperationType[] = []
  private entityIndexByUniqueQueryMap = new Map<string, number>()
  private entityIdByUniqueQueryMap = new Map<string, number>()
  private entityByUniqueQueryCurrentIndex = 0
  private classIdByNameMap = new Map<string, number>()

  static createWithInitialInputs(api: ApiPromise): InputParser {
    const { classInputs, schemaInputs, entityBatchInputs } = getInitializationInputs()
    return new InputParser(api, classInputs, schemaInputs, entityBatchInputs)
  }

  static createWithKnownSchemas(api: ApiPromise, entityBatches?: EntityBatch[]): InputParser {
    return new InputParser(
      api,
      [],
      getInputs<AddClassSchema>('schemas').map(({ data }) => data),
      entityBatches
    )
  }

  constructor(
    api: ApiPromise,
    classInputs?: CreateClass[],
    schemaInputs?: AddClassSchema[],
    batchInputs?: EntityBatch[]
  ) {
    this.api = api
    this.classInputs = classInputs || []
    this.schemaInputs = schemaInputs || []
    this.batchInputs = batchInputs || []
  }

  private async loadClassMap() {
    this.classIdByNameMap = new Map<string, number>()

    const classEntries = await this.api.query.contentDirectory.classById.entries()
    classEntries.forEach(([key, aClass]) => {
      this.classIdByNameMap.set(aClass.name.toString(), (key.args[0] as ClassId).toNumber())
    })
  }

  private async loadEntityIdByUniqueQueryMap() {
    this.entityIdByUniqueQueryMap = new Map<string, number>()

    // Get entity entries
    const entityEntries: [EntityId, Entity][] = (
      await this.api.query.contentDirectory.entityById.entries()
    ).map(([storageKey, entity]) => [storageKey.args[0] as EntityId, entity])

    // Since we use classMap directly we need to make sure it's loaded first
    if (!this.classIdByNameMap.size) {
      await this.loadClassMap()
    }

    entityEntries.forEach(([entityId, entity]) => {
      const classId = entity.class_id.toNumber()
      const className = Array.from(this.classIdByNameMap.entries()).find(([, id]) => id === classId)?.[0]
      if (!className) {
        // Class not found - skip
        return
      }
      let schema: AddClassSchema
      try {
        schema = this.schemaByClassName(className)
      } catch (e) {
        // Input schema not found - skip
        return
      }
      const valuesEntries = Array.from(entity.getField('values').entries())
      schema.newProperties.forEach(({ name, unique }, index) => {
        if (!unique) {
          return // Skip non-unique properties
        }
        const storedValue = valuesEntries.find(([propertyId]) => propertyId.toNumber() === index)?.[1]
        if (
          storedValue === undefined ||
          // If unique value is Bool, it's almost definitely empty, so we skip it
          (storedValue.isOfType('Single') && storedValue.asType('Single').isOfType('Bool'))
        ) {
          // Skip empty values (not all unique properties are required)
          return
        }
        const simpleValue = storedValue.getValue().toJSON()
        const hash = this.getUniqueQueryHash({ [name]: simpleValue }, schema.className)
        this.entityIdByUniqueQueryMap.set(hash, entityId.toNumber())
      })
    })
  }

  private schemaByClassName(className: string) {
    const foundSchema = this.schemaInputs.find((data) => data.className === className)
    if (!foundSchema) {
      throw new Error(`Schema not found by class name: ${className}`)
    }

    return foundSchema
  }

  private getUniqueQueryHash(uniquePropVal: Record<string, any>, className: string) {
    return blake2AsHex(JSON.stringify([className, uniquePropVal]))
  }

  private findEntityIndexByUniqueQuery(uniquePropVal: Record<string, any>, className: string) {
    const hash = this.getUniqueQueryHash(uniquePropVal, className)
    const foundIndex = this.entityIndexByUniqueQueryMap.get(hash)
    if (foundIndex === undefined) {
      throw new Error(
        `findEntityIndexByUniqueQuery failed for class ${className} and query: ${JSON.stringify(uniquePropVal)}`
      )
    }

    return foundIndex
  }

  // Seatch for entity by { [uniquePropName]: [uniquePropVal] } on chain
  async findEntityIdByUniqueQuery(uniquePropVal: Record<string, any>, className: string): Promise<number> {
    const hash = this.getUniqueQueryHash(uniquePropVal, className)
    let foundId = this.entityIdByUniqueQueryMap.get(hash)
    if (foundId === undefined) {
      // Try to re-load the map and find again
      await this.loadEntityIdByUniqueQueryMap()
      foundId = this.entityIdByUniqueQueryMap.get(hash)
      if (foundId === undefined) {
        // If still not found - throw
        throw new Error(
          `findEntityIdByUniqueQuery failed for class ${className} and query: ${JSON.stringify(uniquePropVal)}`
        )
      }
    }
    return foundId
  }

  async getClassIdByName(className: string): Promise<number> {
    let classId = this.classIdByNameMap.get(className)
    if (classId === undefined) {
      // Try to re-load the map
      await this.loadClassMap()
      classId = this.classIdByNameMap.get(className)
      if (classId === undefined) {
        // If still not found - throw
        throw new Error(`Could not find class id by name: "${className}"!`)
      }
    }
    return classId
  }

  private async parsePropertyType(propertyType: Property['property_type']): Promise<PropertyType> {
    if (isSingle(propertyType) && isReference(propertyType.Single)) {
      const { className, sameOwner } = propertyType.Single.Reference
      const classId = await this.getClassIdByName(className)
      return createType('PropertyType', { Single: { Reference: [classId, sameOwner] } })
    }
    // Types other than reference are fully compatible
    return createType('PropertyType', propertyType as any)
  }

  private includeEntityInputInUniqueQueryMap(entityInput: Record<string, any>, schema: AddClassSchema) {
    Object.entries(entityInput)
      .filter(([, pValue]) => pValue !== undefined)
      .forEach(([propertyName, propertyValue]) => {
        const schemaPropertyType = schema.newProperties.find((p) => p.name === propertyName)!.property_type
        // Handle entities "nested" via "new"
        if (isSingle(schemaPropertyType) && isReference(schemaPropertyType.Single)) {
          if (propertyValue !== null && Object.keys(propertyValue).includes('new')) {
            const refEntitySchema = this.schemaByClassName(schemaPropertyType.Single.Reference.className)
            this.includeEntityInputInUniqueQueryMap(propertyValue.new, refEntitySchema)
          }
        }
      })
    // Add entries to entityIndexByUniqueQueryMap
    schema.newProperties
      .filter((p) => p.unique)
      .forEach(({ name }) => {
        if (entityInput[name] === undefined) {
          // Skip empty values (not all unique properties are required)
          return
        }
        const hash = this.getUniqueQueryHash({ [name]: entityInput[name] }, schema.className)
        this.entityIndexByUniqueQueryMap.set(hash, this.entityByUniqueQueryCurrentIndex)
      })
    ++this.entityByUniqueQueryCurrentIndex
  }

  private async createParametrizedPropertyValues(
    entityInput: Record<string, any>,
    schema: AddClassSchema,
    customHandler?: (property: Property, value: any) => Promise<ParametrizedPropertyValue | undefined>
  ): Promise<ParametrizedClassPropertyValue[]> {
    const filteredInput = Object.entries(entityInput).filter(([, pValue]) => pValue !== undefined)
    const parametrizedClassPropValues: ParametrizedClassPropertyValue[] = []

    for (const [propertyName, propertyValue] of filteredInput) {
      const schemaPropertyIndex = schema.newProperties.findIndex((p) => p.name === propertyName)
      const schemaProperty = schema.newProperties[schemaPropertyIndex]

      let value = customHandler && (await customHandler(schemaProperty, propertyValue))
      if (value === undefined) {
        if (propertyValue === null) {
          // Optional values: (can be cleared by setting them to Bool(false)):
          value = createType('ParametrizedPropertyValue', { InputPropertyValue: { Single: { Bool: false } } })
        } else {
          value = createType('ParametrizedPropertyValue', {
            InputPropertyValue: (await this.parsePropertyType(schemaProperty.property_type)).toInputPropertyValue(
              propertyValue
            ),
          })
        }
      }

      parametrizedClassPropValues.push(
        createType('ParametrizedClassPropertyValue', {
          in_class_index: schemaPropertyIndex,
          value,
        })
      )
    }

    return parametrizedClassPropValues
  }

  private async existingEntityQueryToParametrizedPropertyValue(className: string, uniquePropVal: Record<string, any>) {
    try {
      // First - try to find in existing batches
      const entityIndex = this.findEntityIndexByUniqueQuery(uniquePropVal, className)
      return createType('ParametrizedPropertyValue', { InternalEntityJustAdded: entityIndex })
    } catch (e) {
      // If not found - fallback to chain search
      const entityId = await this.findEntityIdByUniqueQuery(uniquePropVal, className)
      return createType('ParametrizedPropertyValue', {
        InputPropertyValue: { Single: { Reference: entityId } },
      })
    }
  }

  // parseEntityInput Overloads
  private parseEntityInput(entityInput: Record<string, any>, schema: AddClassSchema): Promise<number>
  private parseEntityInput(
    entityInput: Record<string, any>,
    schema: AddClassSchema,
    updatedEntityId: number
  ): Promise<void>

  // Parse entity input. Speficy "updatedEntityId" only if want to parse into update operation!
  private async parseEntityInput(
    entityInput: Record<string, any>,
    schema: AddClassSchema,
    updatedEntityId?: number
  ): Promise<void | number> {
    const parametrizedPropertyValues = await this.createParametrizedPropertyValues(
      entityInput,
      schema,
      async (property, value) => {
        // Custom handler for references
        const { property_type: propertyType } = property
        if (isSingle(propertyType) && isReference(propertyType.Single)) {
          const refEntitySchema = this.schemaByClassName(propertyType.Single.Reference.className)
          if (value !== null && Object.keys(value).includes('new')) {
            const entityIndex = await this.parseEntityInput(value.new, refEntitySchema)
            return createType('ParametrizedPropertyValue', { InternalEntityJustAdded: entityIndex })
          } else if (value !== null && Object.keys(value).includes('existing')) {
            return this.existingEntityQueryToParametrizedPropertyValue(refEntitySchema.className, value.existing)
          }
        }
        return undefined
      }
    )

    if (updatedEntityId) {
      // Update operation
      this.updateEntityPropertyValuesOperations.push(
        createType('OperationType', {
          UpdatePropertyValues: {
            entity_id: { ExistingEntity: updatedEntityId },
            new_parametrized_property_values: parametrizedPropertyValues,
          },
        })
      )
    } else {
      // Add operations (createEntity, AddSchemaSupportToEntity)
      const createEntityOperationIndex = this.createEntityOperations.length
      const classId = await this.getClassIdByName(schema.className)
      this.createEntityOperations.push(createType('OperationType', { CreateEntity: { class_id: classId } }))
      this.addSchemaToEntityOprations.push(
        createType('OperationType', {
          AddSchemaSupportToEntity: {
            schema_id: 0,
            entity_id: { InternalEntityJustAdded: createEntityOperationIndex },
            parametrized_property_values: parametrizedPropertyValues,
          },
        })
      )

      // Return CreateEntity operation index
      return createEntityOperationIndex
    }
  }

  private reset() {
    this.entityIndexByUniqueQueryMap = new Map<string, number>()
    this.classIdByNameMap = new Map<string, number>()

    this.createEntityOperations = []
    this.addSchemaToEntityOprations = []
    this.updateEntityPropertyValuesOperations = []

    this.entityByUniqueQueryCurrentIndex = 0
  }

  public async getEntityBatchOperations() {
    // First - create entityUniqueQueryMap to allow referencing any entity at any point
    this.batchInputs.forEach((batch) => {
      const entitySchema = this.schemaByClassName(batch.className)
      batch.entries.forEach((entityInput) => this.includeEntityInputInUniqueQueryMap(entityInput, entitySchema))
    })
    // Then - parse into actual operations
    for (const batch of this.batchInputs) {
      const entitySchema = this.schemaByClassName(batch.className)
      for (const entityInput of batch.entries) {
        await this.parseEntityInput(entityInput, entitySchema)
      }
    }

    const operations = [...this.createEntityOperations, ...this.addSchemaToEntityOprations]
    this.reset()

    return operations
  }

  public async getEntityUpdateOperations(
    input: Record<string, any>,
    className: string,
    entityId: number
  ): Promise<OperationType[]> {
    const schema = this.schemaByClassName(className)
    await this.parseEntityInput(input, schema, entityId)
    const operations = [
      ...this.createEntityOperations,
      ...this.addSchemaToEntityOprations,
      ...this.updateEntityPropertyValuesOperations,
    ]
    this.reset()

    return operations
  }

  public async parseAddClassSchemaExtrinsic(inputData: AddClassSchema) {
    const classId = await this.getClassIdByName(inputData.className)
    const newProperties = await Promise.all(
      inputData.newProperties.map(async (p) => ({
        ...p,
        // Parse different format for Reference (and potentially other propTypes in the future)
        property_type: (await this.parsePropertyType(p.property_type)).toJSON(),
      }))
    )
    return this.api.tx.contentDirectory.addClassSchema(
      classId,
      new (JoyBTreeSet(PropertyId))(this.api.registry, inputData.existingProperties),
      newProperties
    )
  }

  public parseCreateClassExtrinsic(inputData: CreateClass) {
    return this.api.tx.contentDirectory.createClass(
      inputData.name,
      inputData.description,
      inputData.class_permissions || {},
      inputData.maximum_entities_count,
      inputData.default_entity_creation_voucher_upper_bound
    )
  }

  public async getAddSchemaExtrinsics() {
    return await Promise.all(this.schemaInputs.map((data) => this.parseAddClassSchemaExtrinsic(data)))
  }

  public getCreateClassExntrinsics() {
    return this.classInputs.map((data) => this.parseCreateClassExtrinsic(data))
  }

  // Helper parser for "standalone" extrinsics like addSchemaSupportToEntity / updateEntityPropertyValues
  public async parseToInputEntityValuesMap(
    inputData: SimpleEntityInput,
    className: string
  ): Promise<InputEntityValuesMap> {
    await this.parseEntityInput(inputData, this.schemaByClassName(className))
    const inputPropValMap = new Map<PropertyId, InputPropertyValue>()

    const [operation] = this.addSchemaToEntityOprations
    operation
      .asType('AddSchemaSupportToEntity')
      .parametrized_property_values /* First we need to sort by propertyId, since otherwise there will be issues
      when encoding the BTreeMap (similar to BTreeSet) */
      .sort((a, b) => a.in_class_index.toNumber() - b.in_class_index.toNumber())
      .map((pcpv) => {
        inputPropValMap.set(pcpv.in_class_index, pcpv.value.asType('InputPropertyValue'))
      })

    this.reset()

    return createType('InputEntityValuesMap', inputPropValMap)
  }
}
