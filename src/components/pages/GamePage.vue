<script setup>
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { useStore } from '../../stores/game';

import SimpleChessHistoryVue from '../elements/SimpleChessHistory.vue';

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
  <p>
    <ui-button raised @click="startNewGame()">{{t('pages.game.buttons.new-game')}}</ui-button>
  </p>
  <div id="mainZone">
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
    <simple-chess-history-vue />
  </div>
  <ui-snackbar
    v-model="snackBarOpen"
    :timeout-ms="4000"
    :message="snackBarMessage"
    :action-type="1"
  ></ui-snackbar>
</template>

<style scoped>
#mainZone {
  display: flex;
  flex-direction: row;
  justify-content: space-evenly;
  align-items: center;
  margin-bottom: 50px;
}
</style>
