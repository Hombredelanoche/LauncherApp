<template>
  <div class="common-layout">
    <el-container>
      <el-header>
        <AppHeader />
      </el-header>
      <el-main style="height: 90vh;">
        <AppBody />
      </el-main>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import AppHeader from './layout/AppHeader.vue';
import AppBody from './layout/AppBody.vue';
</script>

<style scoped>
.el-header {
  padding: 0px;
}


</style>