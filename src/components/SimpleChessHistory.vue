<script setup>
import { ref, computed } from 'vue';

defineProps({
  width: {
    type: String,
    default: "300px"
  },
  height: {
    type: String,
    default: "300px",
  }
});

/**
 * requestNodeSelected notifies that a move node has been clicked.
 * You give it an object with those params :
 * @param {Number} nodeIndex - index of node in history component
 * @param {String} fen - the requested position in Forsyhth-Edwards Notation
 * @param {Number} fromFileIndex - the start file index of matching move.
 * @param {Number} fromRankIndex - the start rank index of matching move.
 * @param {Number} toFileIndex - the end file index of matching move.
 * @param {Number} toRankIndex - the end rank index of matching move.
 * 
 * requestStartPosition notifies that we want to go in the start position of the game.
 * Does not take any parameter.
 */
const emit = defineEmits(['requestNodeSelected', 'requestStartPosition']);

const nodes = ref([]);
const mainContent = ref();
const selectedNodeIndex = ref(-1);
const navigationMode = ref(false);

/**
 * Clears history.
 * @param {Number} startMoveNumber : first move number of the game.
 * @param {Boolean} startsAsWhite : true if white starts the game, false otherwise.
 */
function reset(startMoveNumber, startsAsWhite) {
  const moveNumberText = `${startMoveNumber}.${startsAsWhite ? '' : '..'}`;
  navigationMode.value = false;
  selectedNodeIndex.value = -1;
  nodes.value = [{number: moveNumberText}];
}

/**
 * Add a node to history. You give an object with
 * @param number: String? (can be undefined) - the move number text
 * @param fan: String? (can be undefined) - the move text without the number ands with chess symbols as Unicode
 * @param fen: String? (can be undefined) - the position value resulting from move in Forstyh-Edwards Notation
 * @param fromFileIndex: Number? (can be undefined) - the start file index of the move
 * @param fromRankIndex: Number? (can be undefined) - the start rank index of the move
 * @param toFileIndex: Number? (can be undefined) - the end file index of the move
 * @param toRankIndex: Number? (can be undefined) - the end rank index of the move
 *  
 */
function addNode(parameters) {
  const  {number, fan, fen, fromFileIndex, fromRankIndex, toFileIndex, toRankIndex} = parameters;
  nodes.value = [...nodes.value, {number, fan, fen, fromFileIndex, fromRankIndex, toFileIndex, toRankIndex}];
}

/**
 * Scroll, so that the last child is visible.
 */
function scrollToLastElement() {
  const lastChild = document.querySelector('.main-content span:last-child');
  lastChild.scrollIntoView();
}

/**
 * Sets the selected node :
 * - highlight it
 * - scrolls to it
 * @param {Number} nodeIndex 
 */
function setSelectedNode(nodeIndex) {
  // Needing to be sure that clicking on next move will point to first move node.
  if (nodeIndex < 0) nodeIndex = -1;
  selectedNodeIndex.value = nodeIndex;
  if (nodeIndex > 0) {
    const targetChild = document.querySelector(`.main-content span:nth-child(${nodeIndex})`);
    targetChild.scrollIntoView();
  }
}

/**
 * Activates navigation mode :
 * - shows toolbar
 * - select last move
 */
function activateNavigationMode() {
  navigationMode.value = true;
  selectLastNode();
}

function handleClick(nodeIndex) {
  const {fen, fromFileIndex, fromRankIndex, toFileIndex, toRankIndex} = nodes.value[nodeIndex];

  if (!navigationMode.value) return;
  if (!fen) return;

  emit('requestNodeSelected', {
    nodeIndex,
    fen,
    fromFileIndex,
    fromRankIndex,
    toFileIndex,
    toRankIndex,
  });
}

function isSelectedNode(nodeIndex) {
  return nodeIndex === selectedNodeIndex.value;
}

function scrollToTop() {
  mainContent.value.scroll(0, 0);
}

function requestStartPositionOnBoard() {
  if (!navigationMode.value) return;
  emit('requestStartPosition');
}

function selectStartPosition() {
  if (!navigationMode.value) return;
  requestStartPositionOnBoard();
  scrollToTop();
}

function selectPreviousNode() {
  if (!navigationMode.value) return;
  for (selectedNodeIndex.value -= 1; selectedNodeIndex.value >= 0; selectedNodeIndex.value--) {
    const currentNode = nodes.value[selectedNodeIndex.value];
    const isAMoveNode = currentNode.fen;
    if (isAMoveNode) break;
  }

  if (selectedNodeIndex.value >= 0) {
    const node = nodes.value[selectedNodeIndex.value];
    const {fen,
      fromFileIndex,
      fromRankIndex,
      toFileIndex,
      toRankIndex} = node;
    emit('requestNodeSelected', {
      nodeIndex: selectedNodeIndex.value,
      fen,
      fromFileIndex,
      fromRankIndex,
      toFileIndex,
      toRankIndex,
    });
  }
  else {    
    selectStartPosition();
  }
}

function selectNextNode() {
  if (!navigationMode.value) return;
  for (selectedNodeIndex.value += 1; selectedNodeIndex.value < nodes.value.length; selectedNodeIndex.value++) {
    const currentNode = nodes.value[selectedNodeIndex.value];
    const isAMoveNode = currentNode.fen;
    if (isAMoveNode) break;
  }

  const node = nodes.value[selectedNodeIndex.value];
  if (!(node?.fen)) {
    // cancelling progress in history if we point to a number node
    selectPreviousNode();
  }


  const {fen,
    fromFileIndex,
    fromRankIndex,
    toFileIndex,
    toRankIndex} = node;
  emit('requestNodeSelected', {
    nodeIndex: selectedNodeIndex.value,
    fen,
    fromFileIndex,
    fromRankIndex,
    toFileIndex,
    toRankIndex,
  });
}

function selectLastNode() {
  if (!navigationMode.value) return;
  for (selectedNodeIndex.value += 1; selectedNodeIndex.value < nodes.value.length; selectedNodeIndex.value++) {
  }

  const node = nodes.value[selectedNodeIndex.value];
  if (!(node?.fen)) {
    // cancelling progress in history if we point to a number node
    selectPreviousNode();
  }


  const {fen,
    fromFileIndex,
    fromRankIndex,
    toFileIndex,
    toRankIndex} = node;
  emit('requestNodeSelected', {
    nodeIndex: selectedNodeIndex.value,
    fen,
    fromFileIndex,
    fromRankIndex,
    toFileIndex,
    toRankIndex,
  });
}

defineExpose({
  reset,
  addNode,
  setSelectedNode,
  scrollToLastElement,
  activateNavigationMode,
});

</script>

<template>
    <div class="root">
        <div class="toolbar" v-if="navigationMode">
          <button
            @click="selectStartPosition"
            class="toolbar__button"
          ><font-awesome-icon icon="fa-solid fa-backward-fast" /></button>
          <button
            @click="selectPreviousNode"
            class="toolbar__button"
          ><font-awesome-icon icon="fa-solid fa-backward-step" /></button>
          <button
            @click="selectNextNode"
            class="toolbar__button"
          ><font-awesome-icon icon="fa-solid fa-forward-step" /></button>
          <button
            @click="selectLastNode"
            class="toolbar__button"
          ><font-awesome-icon icon="fa-solid fa-forward-fast" /></button>
        </div>
        <div class="main-content" ref="mainContent">
            <span v-for="(node, index) in nodes" :key="index" :class="{selected: isSelectedNode(index)}" @click="handleClick(index)">
              {{ `${node.number  ?? ''}&nbsp;`}}{{ `${node.fan ?? ''}&nbsp;` }}
            </span>
        </div>
    </div>
</template>

<style scoped>
@font-face {
  font-family: "Free Serif";
  src: url("@/assets/fonts/FreeSerif.ttf");
}

.root {
  width: v-bind(width);
  height: v-bind(height);
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: flex-start;
}

.toolbar {
  background-color: olive;
  width: 100%;
  flex-grow: 1;
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  align-items: center;
}

.toolbar__button {
  font-size: smaller;
}

.main-content {
  width: 100%;
  display: flex;
  flex-grow: 7;
  flex-direction: row;
  justify-content: flex-start;
  flex-wrap: wrap;
  align-content: flex-start;
  box-sizing: border-box;
  background-color: azure;
  font-family: 'Free Serif';
  font-size: xx-large;
  padding: 0.5rem;
  text-align: start;
  overflow-x: visible;
  overflow-y: scroll;
}

.selected {
  background-color: yellowgreen;
  color: blue;
}
</style>