import love from 'eslint-config-love'

export default [
  {
    ignores: ['**/target/**/*', '**/node_modules/**/*']
  },
  {
    ...love,
    rules: {
      '@typescript-eslint/no-magic-numbers': ['off']
    },
    files: ['**/*.{ts,tsx,cts,mts,d.ts,d.mts,d.cts}']
  }
]
