/** @type { import("eslint").Linter.Config } */
module.exports = {
	root: true,
	extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended', 'prettier'],
	parser: '@typescript-eslint/parser',
	plugins: ['@typescript-eslint'],
	parserOptions: {
		sourceType: 'module',
		ecmaVersion: 2020
	},
	env: {
		es2017: true,
		node: true
	},
	rules: {
		'no-unused-vars': 'off',
		'@typescript-eslint/no-this-alias': 'off',
		'@typescript-eslint/no-unused-vars': [
			'off',
			{
				argsIgnorePattern: '^_',
				varsIgnorePattern: '^_',
				caughtErrorsIgnorePattern: '^_'
			}
		]
	}
};
