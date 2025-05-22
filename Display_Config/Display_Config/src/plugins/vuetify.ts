/**
* plugins/vuetify.ts
*
* Framework documentation: https://vuetifyjs.com`
*/

// Styles
import 'vuetify/styles'

// Composables
import { mdi, aliases } from 'vuetify/iconsets/mdi-svg'
import { createVuetify } from 'vuetify'

// https://vuetifyjs.com/en/introduction/why-vuetify/#feature-guides
export default createVuetify({
  theme: {
    defaultTheme: 'dark',
  },
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: { mdi },
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
      hideSpinButtons: true,
      persistentPlaceholder: true,
    }
  }
})