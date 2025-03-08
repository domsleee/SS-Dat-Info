/**
* plugins/vuetify.ts
*
* Framework documentation: https://vuetifyjs.com`
*/

// Styles
import '@mdi/font/css/materialdesignicons.css'
import 'vuetify/styles'

// Composables
import { createVuetify } from 'vuetify'

// https://vuetifyjs.com/en/introduction/why-vuetify/#feature-guides
export default createVuetify({
  theme: {
    defaultTheme: 'dark',
  },
  defaults: {
    global: {
      transition: 'no',
      density: 'compact',
    },
    VSelect: {
      hideDetails: true,
      persistentPlaceholder: true,
      placeholder: "<choose one>",
    },
    VCombobox: {
      hideDetails: true,
      persistentPlaceholder: true,
      placeholder: "<choose one>",
    },
    VCheckbox: {
      hideDetails: true,
    },
    VTextField: {
      hideDetails: true,
    }
  }
})