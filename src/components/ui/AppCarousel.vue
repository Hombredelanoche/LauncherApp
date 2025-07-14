<template>
<el-carousel
    height="10rem"
    style="width: 15rem;"
    direction="vertical"
    type="card"
    :autoplay="false"
  >
    <el-carousel-item v-for="(item, key) in props.items" :key="key">
      <h5 text="small" @dblclick="openExe(item.path)">{{ key }}</h5>
    </el-carousel-item>
  </el-carousel>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const props = defineProps({
  items: {
    type: Object,
    required: true
  }
});

const openExe = async (path: string) => {
  try {
    const filename = path.split('\\').pop();
    if (!filename) {
      console.error('Invalid file path:', path);
      return;
    } else {
      const result = await invoke('open_executable', { path });
      console.log(result);
    }
  } catch (error) {
    console.error('Error opening executable:', error);  
  }
}
</script>

<style scoped>
.el-carousel__item  {
  border-radius: 0.5rem;
}
.el-carousel__item h5 {
  display: flex;
  height: 100%;
  justify-content: center;
  flex-direction: column;
  text-align: center;
  color: #f6f6f6;
}

.el-carousel__item:nth-child(2n) {
  background-color: #00a8ff;
}

.el-carousel__item:nth-child(2n + 1) {
  background-color: #00a8ff;
}

.el-carousel__indicators--right {
  visibility: hidden;
}

</style>