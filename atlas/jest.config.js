module.exports = {
  preset: "ts-jest",
  setupFiles: [
    "<rootDir>/enzyme.config.js"
  ],
  moduleFileExtensions: [
    "js",
    "ts",
    "tsx"
  ],
  transform: {
    "^.+\\.tsx?$": "ts-jest",
    "^.+\\.jsx?$": "babel-jest"
  },
  snapshotSerializers: [
    "enzyme-to-json/serializer"
  ]
}