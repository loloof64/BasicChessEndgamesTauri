<script setup>
import { ref } from 'vue';
import { useStore } from '../../stores/game';

defineProps({
  
})

const gameStore = useStore();
const board = ref();
const snackBarOpen = ref(false);
const snackBarMessage = ref('');

function startNewGame() {
  gameStore.startNewGame('rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2');
  board.value.newGame(gameStore.startPosition);
  snackBarMessage.value = "New game started."
  snackBarOpen.value = true;
}
</script>

<template>
  <loloof64-chessboard
    ref="board"
    :size="300"
  >
  </loloof64-chessboard>
  <p>
    <ui-button raised @click="startNewGame()">New game</ui-button>
  </p>
  <p>Start position is <b>{{ gameStore.startPosition }}</b></p>
  <ui-snackbar
    v-model="snackBarOpen"
    :timeout-ms="4000"
    :message="snackBarMessage"
    :action-type="1"
  ></ui-snackbar>
</template>

<style scoped>
</style>
