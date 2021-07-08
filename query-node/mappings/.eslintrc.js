module.exports = {
  extends: ['@joystream/eslint-config'],
  env: {
    node: true,
  },
  rules: {
    '@typescript-eslint/naming-convention': 'off',
    '@typescript-eslint/ban-types': ["error",
      {
        "types": {
          // enable usage of `Object` data type in TS; it has it's meaning(!) and it's disabled
          // by default only beacuse people tend to misuse it
          "Object": false,
        },
        "extendDefaults": true
      }
    ]
  },
}
