<template>
<el-carousel
    height="10rem"
    style="width: 15rem;"
    direction="vertical"
    type="card"
    :autoplay="false"
  >
    <el-carousel-item v-for="(item, key) in items" :key="key">
      <h5 text="small" @dblclick="">{{ key }}</h5>
    </el-carousel-item>
  </el-carousel>
</template>

<script setup lang="ts">
defineProps({
  items: {
    type: Object,
    required: true
  }
});
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