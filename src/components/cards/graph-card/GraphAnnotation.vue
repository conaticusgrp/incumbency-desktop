<script setup lang="ts">
  const props = defineProps<{ data: number[] }>();
  const first = props.data[0];
  const last = props.data[props.data.length - 1];
  const diff = last - first;
  const isPositive = diff > 0;
  const styleClass = isPositive ? 'positive' : 'negative';
  const sign = isPositive ? '+' : '-';
  const formatted = Math.abs(diff).toLocaleString();
  const percent = Math.abs(diff / first * 100).toFixed(2);
</script>

<template>
  <div v-if="diff !== 0">
    <h1 :class=styleClass>{{sign}}${{formatted}} / {{sign}}{{percent}}%</h1>
  </div>
</template>

<style scoped>
div {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  font-size: 0.35rem;
}

.positive {
  color: #6D9B14;
}

.negative {
  color: #C82525;
}
</style>
