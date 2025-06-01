// @ts-check

import js from "@eslint/js";
import tseslint from "typescript-eslint";
import stylistic from "@stylistic/eslint-plugin";
import pluginVue from 'eslint-plugin-vue'
import { globalIgnores } from "eslint/config";
import globals from 'globals'

export default tseslint.config(
  js.configs.recommended,
  ...tseslint.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  {
    files: ['*.vue', '**/*.vue'],
    languageOptions: {
      parserOptions: {
        parser: '@typescript-eslint/parser'
      }
    }
  },
  {
    plugins: {
      "@stylistic": stylistic,
    },
    languageOptions: {
      sourceType: 'module',
      parserOptions: { projectService: true, extraFileExtensions: ['.vue'] },
      globals: {
        ...globals.browser
      }
    },
    rules: {
      "@stylistic/indent": ["error", 2],
      "@typescript-eslint/no-floating-promises": "error",
    },
  },
  globalIgnores(["src-tauri/", "dist/"])
);
