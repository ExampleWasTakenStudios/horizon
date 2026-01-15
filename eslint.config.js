import eslint from '@eslint/js';
import recommendedConfig from 'eslint-plugin-prettier/recommended';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default tseslint.config(
  eslint.configs.recommended,
  ...tseslint.configs.strictTypeChecked,
  ...tseslint.configs.stylisticTypeChecked,
  recommendedConfig,
  {
    name: 'Main Config',
    languageOptions: {
      globals: globals.builtin,
      parserOptions: {
        projectService: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
    rules: {
      'prettier/prettier': ['error', {}, { usePrettierrc: true }],

      /* UNICORN CONFIG START */
      'better-regex': 'error',
      'filename-case': ['error', { cases: { PascalCase: true, kebabCase: true } }],
      'no-abusive-eslint-disable': 'error',
      'no-new-buffer': 'error',
      /* UNICORN CONFIG END */

      /* BAN THROW */
      'no-restricted-syntax': [
        'error',
        {
          selector: 'ThrowStatement',
          message: 'Throwing exceptions is banned. Return a TResult<T, E> instead.',
        },
      ],
      'prefer-promise-reject-errors': 'error',

      'max-lines-per-function': ['error', { max: 60, skipBlankLines: true, skipComments: true }],
      complexity: ['error', { max: 10, variant: 'modified' }],
      'no-await-in-loop': 'error',
      'no-constructor-return': 'error',
      'no-self-compare': 'error',
      'no-unreachable-loop': 'error',
      'no-console': 'warn',

      /* The following block contains ESLint Rules that are extended by @typescript-eslint and are therefore disabled. */
      'no-unused-vars': 'off',
      'init-loop-func': 'off',
      'no-shadow': 'off',
      'no-unused-private-class-members': 'off',
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          varsIgnorePattern: '^_',
          argsIgnorePattern: '^_',
          caughtErrorsIgnorePattern: '^_',
          destructuredArrayIgnorePattern: '^_',
        },
      ],
      '@typescript-eslint/consistent-type-exports': ['error', { fixMixedExportsWithInlineTypeSpecifier: true }],
      '@typescript-eslint/default-param-last': 'error',
      '@typescript-eslint/explicit-function-return-type': [
        'error',
        {
          allowExpressions: true,
          allowHigherOrderFunctions: false,
          allowDirectConstAssertionInArrowFunctions: false,
        },
      ],
      '@typescript-eslint/explicit-member-accessibility': 'error',
      '@typescript-eslint/explicit-module-boundary-types': [
        'error',
        { allowHigherOrderFunctions: false, allowTypedFunctionExpressions: false },
      ],
      // '@typescript-eslint/member-ordering': ['error', ] // TODO: this requires extensive configuration but can help greatly with consistent code style
      '@typescript-eslint/method-signature-style': ['error', 'method'],
      //'@typescript-eslint/naming-convention': ['error'] // TODO: this requires extensive configuration but can help greatly with consistent code style
      '@typescript-eslint/no-import-type-side-effects': 'error',
      '@typescript-eslint/no-loop-func': 'error',
      '@typescript-eslint/no-shadow': 'error',
      '@typescript-eslint/no-unnecessary-qualifier': 'error',
      '@typescript-eslint/no-unsafe-type-assertion': 'error',
      '@typescript-eslint/no-useless-empty-export': 'error',
      '@typescript-eslint/parameter-properties': ['error', { allow: [], prefer: 'class-property' }],
      '@typescript-eslint/prefer-readonly': 'error',
      '@typescript-eslint/require-array-sort-compare': 'error',
      '@typescript-eslint/switch-exhaustiveness-check': ['error', { considerDefaultExhaustiveForUnions: true }],
      '@typescript-eslint/naming-convention': [
        'error',
        // 1. Default: camelCase
        {
          selector: 'default',
          format: ['camelCase'],
          leadingUnderscore: 'allow',
        },

        // 2. Variables: camelCase
        {
          selector: 'variable',
          format: ['camelCase'],
          leadingUnderscore: 'allow',
        },

        // 3. Static Constant Variables (Global Consts): UPPER_CASE
        {
          selector: 'variable',
          modifiers: ['const', 'global'],
          format: ['UPPER_CASE'],
        },
        // (Optional) Class Static Readonly properties often act as constants too
        {
          selector: 'classProperty',
          modifiers: ['static', 'readonly'],
          format: ['UPPER_CASE'],
        },

        // 4. Classes, Types, Interfaces, Enums: PascalCase
        {
          selector: 'typeLike',
          format: ['PascalCase'],
        },

        // 5. Interfaces: No "I" Prefix
        {
          selector: 'interface',
          format: ['PascalCase'],
          custom: {
            regex: '^I[A-Z]',
            match: false,
          },
        },
      ],
      '@typescript-eslint/member-ordering': [
        'error',
        {
          default: [
            // Fields: Private -> Protected -> Public
            'private-static-field',
            'protected-static-field',
            'public-static-field',

            'private-instance-field',
            'protected-instance-field',
            'public-instance-field',

            // Constructor
            'constructor',

            // Methods: Instance (Public -> Protected -> Private)
            'public-instance-method',
            'protected-instance-method',
            'private-instance-method',

            // Methods: Static (Public -> Protected -> Private)
            'public-static-method',
            'protected-static-method',
            'private-static-method',
          ],
        },
      ]
    },
  },
  {
    files: ['**/*.mjs', '**/*.cjs', '**/*.js'],
    extends: [tseslint.configs.disableTypeChecked],
  },
  {
    // DO NOT ADD ANY PROPERTIES TO THIS OBJECT
    ignores: ['node_modules', 'dist', 'assets'],
  }
);
