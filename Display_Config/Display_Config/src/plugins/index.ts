// Plugins
import vuetify from './vuetify.js'
import { createPinia } from 'pinia'
import type { App } from 'vue'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'

export function registerPlugins(app: App) {
  const pinia = createPinia();
  pinia.use(piniaPluginPersistedstate)
  app.use(pinia)
  app.use(vuetify)
}