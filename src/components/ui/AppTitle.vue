<template>
  <h1
    :style="computedStyle"
    class="title"
  >
    <slot/>
  </h1>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { defineProps } from 'vue';

type Size = 'extraSmall' | 'small' | 'medium' | 'large' | 'extraLarge';
type Color = 'primary' | 'secondary';
type BgColor = 'bg-primary' | 'bg-secondary' | 'bg-tertiary' | 'bg-quaternary';

const props = defineProps<{
  size?: Size,
  color?: Color,
  bgColor?: BgColor,
}>()


const sizes = {
  extraSmall: '1rem',
  small: '1.2rem',
  medium: '1.5rem',
  large: '1.8rem',
  extraLarge: '2rem',
}

const colors = {
  primary: '#f5f6fa',
  secondary: '#353b48',
}

const bgColors = {
  'bg-primary': '#00a8ff',
  'bg-secondary': '#fbc531',
  'bg-tertiary': '#4cd137',
  'bg-quaternary': '#e84118',
}

const computedStyle = computed(() => {
  return {
    fontSize: sizes[props.size || 'medium'],
    color: colors[props.color || 'primary'],
    backgroundColor: bgColors[props.bgColor || 'bg-primary'],
  }
})

</script>

<style scoped>
.title {
  padding: 1rem;
  width: 100%;
  border-top-left-radius: 0.8rem;
  border-top-right-radius: 0.8rem;
  box-shadow: rgba(0, 0, 0, 0.16) 0px 1px 4px;
  text-align: center;
}
</style>