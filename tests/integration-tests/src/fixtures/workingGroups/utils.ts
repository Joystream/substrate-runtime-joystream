import { IOpeningMetadata, OpeningMetadata } from '@joystream/metadata-protobuf'
import { assert } from 'chai'
import { OpeningMetadataFieldsFragment } from '../../graphql/generated/queries'

import { ApplicationFormQuestionType } from '../../graphql/generated/schema'

export const queryNodeQuestionTypeToMetadataQuestionType = (
  type: ApplicationFormQuestionType
): OpeningMetadata.ApplicationFormQuestion.InputType => {
  if (type === ApplicationFormQuestionType.Text) {
    return OpeningMetadata.ApplicationFormQuestion.InputType.TEXT
  }

  return OpeningMetadata.ApplicationFormQuestion.InputType.TEXTAREA
}

export const assertQueriedOpeningMetadataIsValid = (
  qOpeningMeta: OpeningMetadataFieldsFragment,
  expectedMetadata?: IOpeningMetadata | null
): void => {
  const {
    shortDescription,
    description,
    expectedEndingTimestamp,
    hiringLimit,
    applicationDetails,
    applicationFormQuestions,
  } = expectedMetadata || {}
  assert.equal(qOpeningMeta.shortDescription, shortDescription || null)
  assert.equal(qOpeningMeta.description, description || null)
  assert.equal(
    qOpeningMeta.expectedEnding ? new Date(qOpeningMeta.expectedEnding).getTime() : qOpeningMeta.expectedEnding,
    expectedEndingTimestamp || null
  )
  assert.equal(qOpeningMeta.hiringLimit, hiringLimit || null)
  assert.equal(qOpeningMeta.applicationDetails, applicationDetails || null)
  assert.deepEqual(
    qOpeningMeta.applicationFormQuestions
      .sort((a, b) => a.index - b.index)
      .map(({ question, type }) => ({
        question,
        type: queryNodeQuestionTypeToMetadataQuestionType(type),
      })),
    (applicationFormQuestions || []).map(({ question, type }) => ({
      question: question || null,
      type: type || 0,
    }))
  )
}
