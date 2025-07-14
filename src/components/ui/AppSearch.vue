<template>
  <el-input
      v-model="search"
      style="width: 240px"
      placeholder="Type something"
      :prefix-icon="Search"
    />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Search } from '@element-plus/icons-vue'

const search = ref('')
</script>

<style scoped>
</style>