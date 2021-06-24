import * as Types from './schema'

import gql from 'graphql-tag'
export type MemberMetadataFieldsFragment = { name?: Types.Maybe<string>; about?: Types.Maybe<string> }

export type MembershipFieldsFragment = { id: string; handle: string; metadata: MemberMetadataFieldsFragment }

export type GetMemberByIdQueryVariables = Types.Exact<{
  id: Types.Scalars['ID']
}>

export type GetMemberByIdQuery = { membershipByUniqueInput?: Types.Maybe<MembershipFieldsFragment> }

export const MemberMetadataFields = gql`
  fragment MemberMetadataFields on MemberMetadata {
    name
    about
  }
`
export const MembershipFields = gql`
  fragment MembershipFields on Membership {
    id
    handle
    metadata {
      ...MemberMetadataFields
    }
  }
  ${MemberMetadataFields}
`
export const GetMemberById = gql`
  query getMemberById($id: ID!) {
    membershipByUniqueInput(where: { id: $id }) {
      ...MembershipFields
    }
  }
  ${MembershipFields}
`
