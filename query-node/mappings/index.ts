import BN from 'bn.js'

// Workaround for invalid BN variant fields serialization
BN.prototype.toJSON = function () {
  return this.toString()
}

export * from './membership'
export * from './workingGroups'
export * from './proposals'
export * from './proposalsDiscussion'
