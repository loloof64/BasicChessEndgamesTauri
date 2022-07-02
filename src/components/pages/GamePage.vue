<script setup>
import { ref, nextTick } from 'vue';
import { useI18n } from 'vue-i18n';
import { useStore } from '../../stores/game';
import { promiseTimeout } from '@vueuse/core';

import SimpleChessHistoryVue from '../elements/SimpleChessHistory.vue';

const { t } = useI18n();

const gameStore = useStore();
const board = ref();
const history = ref();
const snackBarOpen = ref(false);
const snackBarMessage = ref('');

function startNewGame() {
  gameStore.resetToDefaultGame();
  history.value.reset(gameStore.startMoveNumber, gameStore.startsAsWhite);
  board.value.newGame(gameStore.startPosition);
  snackBarMessage.value = t('pages.game.status.new-game');
  snackBarOpen.value = true;
}

async function handleCheckmate({detail: {whiteTurnBeforeMove}}) {
  await promiseTimeout(10);
  const side = whiteTurnBeforeMove ? t('chess.white') : t('chess.black');
  history.value.activateNavigationMode();
  let message = t('pages.game.status.checkmate', {side});
  message = message.charAt(0).toUpperCase() + message.substring(1);
  snackBarMessage.value = message;
  snackBarOpen.value = true;
}

async function handleStalemate() {
  await promiseTimeout(10);
  history.value.activateNavigationMode();
  snackBarMessage.value = t('pages.game.status.stalemate');
  snackBarOpen.value = true;
}

async function handleThreeFoldRepetition() {
  await promiseTimeout(10);
  history.value.activateNavigationMode();
  snackBarMessage.value = t('pages.game.status.three-fold-repetition');
  snackBarOpen.value = true;
}

async function handleMissingMaterialDraw() {
  await promiseTimeout(10);
  history.value.activateNavigationMode();
  snackBarMessage.value = t('pages.game.status.missing-material');
  snackBarOpen.value = true;
}

async function handleFiftyMovesDraw() {
  await promiseTimeout(10);
  history.value.activateNavigationMode();
  snackBarMessage.value = t('pages.game.status.fifty-moves')
  snackBarOpen.value = true;
}

async function handleMoveDone({detail: {moveObject: {moveNumber, whiteTurn, moveFan, 
moveSan, fromFileIndex, fromRankIndex, toFileIndex, toRankIndex}}}) {
  const positionFen = board.value.getCurrentPosition();
  history.value.addNode({
    fan: moveFan,
    fromFileIndex,
    fromRankIndex,
    toFileIndex,
    toRankIndex,
    fen: positionFen,
  });
  const blackTurn = whiteTurn == false;
  const gameInProgress = board.value.gameIsInProgress();
  if (blackTurn && gameInProgress) {
    history.value.addNode({
      number: `${parseInt(moveNumber)+1}.`
    });
  }

  await nextTick();
  history.value.scrollToLastElement();
}

function handleHistoryNodeSelectionRequest({
    nodeIndex,
    fen,
    fromFileIndex,
    fromRankIndex,
    toFileIndex,
    toRankIndex,
}) {
  const gameInProgress = board.value.gameIsInProgress();
  if (gameInProgress) return;

  const updateSuccess = board.value.setPositionAndLastMove({
    positionFen: fen,
    fromFileIndex,
    fromRankIndex,
    toFileIndex,
    toRankIndex,
  });

  if (updateSuccess) {
    history.value.setSelectedNode(nodeIndex);
  }
}

function handleStartPositionRequested() {
  const gameInProgress = board.value.gameIsInProgress();
  if (gameInProgress) return;

  const updateSuccess = board.value.setPositionAndLastMove({
    positionFen: gameStore.startPosition,
    fromFileIndex: undefined,
    fromRankIndex: undefined,
    toFileIndex: undefined,
    toRankIndex: undefined,
  });
  if (updateSuccess) {
    history.value.setSelectedNode(-1);
  }
}
</script>

<template>
  <p>
    <ui-tooltip-anchor>
      <ui-button
        @click="startNewGame()"
        data-tooltip-id="newGameButton"
        raised
      ><img src="@/assets/images/start-line.svg" class="btn-img" />
      </ui-button>
      <ui-tooltip id="newGameButton">{{t('pages.game.buttons.new-game-tooltip')}}</ui-tooltip>
    </ui-tooltip-anchor>
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
    @move-done="handleMoveDone"
    >
    </loloof64-chessboard>
    <simple-chess-history-vue
      ref="history"
      @requestNodeSelected="handleHistoryNodeSelectionRequest"
      @requestStartPosition="handleStartPositionRequested"
    />
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

.btn-img {
  width: 30px;
  height: 30px;
}
</style>
