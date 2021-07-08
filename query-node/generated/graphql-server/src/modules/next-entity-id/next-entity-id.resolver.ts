import {
  Arg,
  Args,
  Mutation,
  Query,
  Root,
  Resolver,
  FieldResolver,
  ObjectType,
  Field,
  Int,
  ArgsType,
  Info,
} from 'type-graphql';
import graphqlFields from 'graphql-fields';
import { Inject } from 'typedi';
import { Min } from 'class-validator';
import { Fields, StandardDeleteResponse, UserId, PageInfo, RawFields } from 'warthog';

import {
  NextEntityIdCreateInput,
  NextEntityIdCreateManyArgs,
  NextEntityIdUpdateArgs,
  NextEntityIdWhereArgs,
  NextEntityIdWhereInput,
  NextEntityIdWhereUniqueInput,
  NextEntityIdOrderByEnum,
} from '../../../generated';

import { NextEntityId } from './next-entity-id.model';
import { NextEntityIdService } from './next-entity-id.service';

@ObjectType()
export class NextEntityIdEdge {
  @Field(() => NextEntityId, { nullable: false })
  node!: NextEntityId;

  @Field(() => String, { nullable: false })
  cursor!: string;
}

@ObjectType()
export class NextEntityIdConnection {
  @Field(() => Int, { nullable: false })
  totalCount!: number;

  @Field(() => [NextEntityIdEdge], { nullable: false })
  edges!: NextEntityIdEdge[];

  @Field(() => PageInfo, { nullable: false })
  pageInfo!: PageInfo;
}

@ArgsType()
export class ConnectionPageInputOptions {
  @Field(() => Int, { nullable: true })
  @Min(0)
  first?: number;

  @Field(() => String, { nullable: true })
  after?: string; // V3: TODO: should we make a RelayCursor scalar?

  @Field(() => Int, { nullable: true })
  @Min(0)
  last?: number;

  @Field(() => String, { nullable: true })
  before?: string;
}

@ArgsType()
export class NextEntityIdConnectionWhereArgs extends ConnectionPageInputOptions {
  @Field(() => NextEntityIdWhereInput, { nullable: true })
  where?: NextEntityIdWhereInput;

  @Field(() => NextEntityIdOrderByEnum, { nullable: true })
  orderBy?: NextEntityIdOrderByEnum;
}

@Resolver(NextEntityId)
export class NextEntityIdResolver {
  constructor(@Inject('NextEntityIdService') public readonly service: NextEntityIdService) {}

  @Query(() => [NextEntityId])
  async nextEntityIds(
    @Args() { where, orderBy, limit, offset }: NextEntityIdWhereArgs,
    @Fields() fields: string[]
  ): Promise<NextEntityId[]> {
    return this.service.find<NextEntityIdWhereInput>(where, orderBy, limit, offset, fields);
  }

  @Query(() => NextEntityId, { nullable: true })
  async nextEntityIdByUniqueInput(
    @Arg('where') where: NextEntityIdWhereUniqueInput,
    @Fields() fields: string[]
  ): Promise<NextEntityId | null> {
    const result = await this.service.find(where, undefined, 1, 0, fields);
    return result && result.length >= 1 ? result[0] : null;
  }

  @Query(() => NextEntityIdConnection)
  async nextEntityIdsConnection(
    @Args() { where, orderBy, ...pageOptions }: NextEntityIdConnectionWhereArgs,
    @Info() info: any
  ): Promise<NextEntityIdConnection> {
    const rawFields = graphqlFields(info, {}, { excludedFields: ['__typename'] });

    let result: any = {
      totalCount: 0,
      edges: [],
      pageInfo: {
        hasNextPage: false,
        hasPreviousPage: false,
      },
    };
    // If the related database table does not have any records then an error is thrown to the client
    // by warthog
    try {
      result = await this.service.findConnection<NextEntityIdWhereInput>(where, orderBy, pageOptions, rawFields);
    } catch (err) {
      console.log(err);
      // TODO: should continue to return this on `Error: Items is empty` or throw the error
      if (!(err.message as string).includes('Items is empty')) throw err;
    }

    return result as Promise<NextEntityIdConnection>;
  }
}
