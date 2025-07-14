<template>
  <el-col :span="7" style="padding-left: 0%; padding-right: 0%;">
      <AppTitle size="extraSmall" bgColor="bg-primary" color="primary">
        Welcome to the Launcher App
      </AppTitle>
      <AppSearch />
      <AppCarousel :items="apps" />
    </el-col>
</template>

<script setup lang="ts">
import AppSearch from '../ui/AppSearch.vue';
import AppTitle from '../ui/AppTitle.vue';
import AppCarousel from '../ui/AppCarousel.vue';
import { onMounted, ref } from 'vue';

const apps = ref([]);
console.log('Apps:', apps.value);


onMounted(async () => {
  fetch('/data/data.json')
    .then(response => response.json())
    .then(data => { 
      apps.value = data;
      console.log('Games fetched successfully:', apps.value);
    })
    .catch(error => {
      console.error('Error fetching games:', error);
    });
});
</script>

<style scoped>
.el-col {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1.2rem;
  height: fit-content;
  padding-bottom: 1rem;
  border-radius: 0.8rem;
  box-shadow: rgba(0, 0, 0, 0.16) 0px 1px 4px;
  background-color: white;
}
</style>