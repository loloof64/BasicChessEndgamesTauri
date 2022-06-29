<script setup>
import { ref } from 'vue';
import { useStore } from '../../stores/game';

defineProps({
  
})

const gameStore = useStore();
const board = ref();

function startNewGame() {
  gameStore.startNewGame('rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2');
  board.value.newGame(gameStore.startPosition);
}
</script>

<template>
  <loloof64-chessboard
    ref="board"
    :size="300"
  >
  </loloof64-chessboard>
  <p>
    <button @click="startNewGame()">New game</button>
  </p>
  <p>Start position is <b>{{ gameStore.startPosition }}</b></p>
</template>

<style scoped>
</style>
