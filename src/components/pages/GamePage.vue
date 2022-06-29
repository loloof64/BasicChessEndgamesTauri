<script setup>
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useStore } from '../../stores/game';

defineProps({
  
});

const { t } = useI18n();

const gameStore = useStore();
const board = ref();
const snackBarOpen = ref(false);
const snackBarMessage = ref('');

function startNewGame() {
  gameStore.resetToDefaultGame();
  board.value.newGame(gameStore.startPosition);
  snackBarMessage.value = t('pages.game.status.new-game');
  snackBarOpen.value = true;
}

function handleCheckmate({detail: {whiteTurnBeforeMove}}) {
  const side = whiteTurnBeforeMove ? t('chess.white') : t('chess.black');
  let message = t('pages.game.status.checkmate', {side});
  message = message.charAt(0).toUpperCase() + message.substring(1);
  snackBarMessage.value = message;
  snackBarOpen.value = true;
}

function handleStalemate() {
  snackBarMessage.value = t('pages.game.status.stalemate');
  snackBarOpen.value = true;
}

function handleThreeFoldRepetition() {
  snackBarMessage.value = t('pages.game.status.three-fold-repetition');
  snackBarOpen.value = true;
}

function handleMissingMaterialDraw() {
  snackBarMessage.value = t('pages.game.status.missing-material');
  snackBarOpen.value = true;
}

function handleFiftyMovesDraw() {
  snackBarMessage.value = t('pages.game.status.fifty-moves')
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
    <ui-button raised @click="startNewGame()">{{t('pages.game.buttons.new-game')}}</ui-button>
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
