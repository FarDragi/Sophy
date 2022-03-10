import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createMetaManager } from 'vue-meta'

async function init () {
  const app = createApp(App)
    .use(router)
    .use(createMetaManager())

  await router.isReady()
  app.mount('#app')
}

init()
