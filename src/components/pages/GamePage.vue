<script setup>
import { useStore } from '../../stores/game';

defineProps({
  
})

const store = useStore();
</script>

<template>
  <p>Position is <b>{{ store.positionFen }}</b></p>
</template>

<style scoped>
</style>
