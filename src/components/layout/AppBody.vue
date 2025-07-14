<template>
  <el-row :gutter="20">
    <AppListApplication/>
  </el-row>
</template>

<script setup lang="ts">
import AppListApplication from '../subComponents/AppListApplication.vue';


</script>

<style scoped>
.el-row {
  display: flex;
  justify-content: center;
  flex-direction: row;
  height: 100%;
  gap: 1rem;
}

</style>