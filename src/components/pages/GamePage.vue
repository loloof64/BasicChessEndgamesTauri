<script setup>
import { useStore } from '../../stores/game';

defineProps({
  msg: String
})

const store = useStore();
</script>

<template>
  <h1>{{ msg }}</h1>

  <p>Position is {{ store.positionFen }}</p>
</template>

<style scoped>
a {
  color: #42b983;
}
</style>
