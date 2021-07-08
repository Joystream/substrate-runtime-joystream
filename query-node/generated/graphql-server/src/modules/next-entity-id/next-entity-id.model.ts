import { BaseModel, FloatField, Model, StringField } from 'warthog';

@Model({ api: {} })
export class NextEntityId extends BaseModel {
  @FloatField({
    description: `Next deterministic id for entities without custom id`,
  })
  nextId!: number;

  constructor(init?: Partial<NextEntityId>) {
    super();
    Object.assign(this, init);
  }
}
