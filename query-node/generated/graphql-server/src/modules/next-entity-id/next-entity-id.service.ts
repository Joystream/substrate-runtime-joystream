import { Service } from 'typedi';
import { Repository } from 'typeorm';
import { InjectRepository } from 'typeorm-typedi-extensions';
import { BaseService, WhereInput } from 'warthog';

import { NextEntityId } from './next-entity-id.model';

@Service('NextEntityIdService')
export class NextEntityIdService extends BaseService<NextEntityId> {
  constructor(@InjectRepository(NextEntityId) protected readonly repository: Repository<NextEntityId>) {
    super(NextEntityId, repository);
  }

  async find<W extends WhereInput>(
    where?: any,
    orderBy?: string,
    limit?: number,
    offset?: number,
    fields?: string[]
  ): Promise<NextEntityId[]> {
    let f = fields;
    if (f == undefined) {
      f = [];
    }

    return super.find<W>(where, orderBy, limit, offset, f);
  }
}
