import { createApp } from "vue";
import App from "./App.vue";
import { registerPlugins } from "./plugins";

import "./styles/global.css"
import { timings } from "./services/timings";

const totalStartupTimer = timings.getTimer('totalStartup');
const createAppTimer = timings.getTimer('createApp');
const app = createApp(App);
timings.endTimer(createAppTimer);

const registerPluginsTimer = timings.getTimer('registerPlugins');
registerPlugins(app);
timings.endTimer(registerPluginsTimer);

const mountAppTimer = timings.getTimer('mountApp');
app.mount('#app')
timings.endTimer(mountAppTimer);
timings.endTimer(totalStartupTimer);
