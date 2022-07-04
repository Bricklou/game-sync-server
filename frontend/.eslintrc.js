module.exports = {
  env: {
    browser: true,
    es2021: true
  },
  extends: ['prettier', 'plugin:react/recommended', 'plugin:@typescript-eslint/recommended'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaFeatures: {
      jsx: true
    },
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  plugins: ['react', 'react-hooks', '@typescript-eslint', 'prettier'],
  rules: {
    'prettier/prettier': ['error'],
    'react/jsx-filename-extension': [
      'warn',
      {
        extensions: ['.tsx']
      }
    ],
    'accessor-pairs': ['error'],
    '@typescript-eslint/explicit-member-accessibility': ['error'],
    '@typescript-eslint/explicit-function-return-type': [
      'error',
      {
        allowExpressions: true,
        allowConciseArrowFunctionExpressionsStartingWithVoid: true
      }
    ],
    'no-unused-vars': 'off',
    '@typescript-eslint/no-unused-vars': ['error']
  },
  settings: {
    react: {
      version: 'detect'
    }
  }
}
