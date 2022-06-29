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
  gameStore.resetToDefaultGame();
  board.value.newGame(gameStore.startPosition);
  snackBarMessage.value = "New game started."
  snackBarOpen.value = true;
}

function handleCheckmate({detail: {whiteTurnBeforeMove}}) {
  const side = whiteTurnBeforeMove ? 'White' : 'Black';
  snackBarMessage.value = side + ' has won by checkmate.';
  snackBarOpen.value = true;
}

function handleStalemate() {
  snackBarMessage.value = 'Draw by stalemate.';
  snackBarOpen.value = true;
}

function handleThreeFoldRepetition() {
  snackBarMessage.value = 'Draw by 3-fold repetition.';
  snackBarOpen.value = true;
}

function handleMissingMaterialDraw() {
  snackBarMessage.value = 'Draw by missing material.';
  snackBarOpen.value = true;
}

function handleFiftyMovesDraw() {
  snackBarMessage.value = 'Draw by the 50-moves rule.'
  snackBarOpen.value = true;
}
</script>

<template>
  <loloof64-chessboard
    ref="board"
    :size="300"
    @checkmate="handleCheckmate"
    @stalemate="handleStalemate"
    @perpetual-draw="handleThreeFoldRepetition"
    @missing-material-draw="handleMissingMaterialDraw"
    @fifty-moves-draw="handleFiftyMovesDraw"
  >
  </loloof64-chessboard>
  <p>
    <ui-button raised @click="startNewGame()">New game</ui-button>
  </p>
  <ui-snackbar
    v-model="snackBarOpen"
    :timeout-ms="4000"
    :message="snackBarMessage"
    :action-type="1"
  ></ui-snackbar>
</template>

<style scoped>
</style>
