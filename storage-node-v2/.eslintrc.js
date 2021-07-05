module.exports = {
  env: {
    mocha: true,
  },
  parserOptions: {
    project: './tsconfig.json'
  },
  extends: ['@joystream/eslint-config'],
  rules: {
    'no-console': 'warn', // use dedicated logger
    'no-unused-vars': 'off', // Required by the typescript rule below
    '@typescript-eslint/no-unused-vars': ['error'],
    '@typescript-eslint/no-floating-promises': 'error',
    'prettier/prettier': 'off', // prettier-eslint conflicts inherited from @joystream/eslint-config
  },
}
