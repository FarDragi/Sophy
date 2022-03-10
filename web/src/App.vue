<template>
  <metainfo>
      <template v-slot:title="{ content }">{{ content ? `${content} | Sophy` : `Sophy` }}</template>
  </metainfo>
  <MenuBar/>
  <router-view/>
</template>

<script setup lang="ts">
import { useMeta } from 'vue-meta'
import MenuBar from './components/MenuBar.vue'

useMeta({
  title: ''
})
</script>

<style lang="scss">
* {
  margin: 0;
  padding: 0;
}

#app {
  min-height: 100%;
  background-color: black;
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: #2c3e50;
}
</style>
