<script setup>
import { ref } from 'vue';

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
 */
const emit = defineEmits(['requestNodeSelected']);

const nodes = ref([]);
const root = ref();
const selectedNodeIndex = ref(-1);

/**
 * Clears history.
 * @param {Number} startMoveNumber : first move number of the game.
 * @param {Boolean} startsAsWhite : true if white starts the game, false otherwise.
 */
function reset(startMoveNumber, startsAsWhite) {
  const moveNumberText = `${startMoveNumber}.${startsAsWhite ? '' : '..'}`;
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
  const lastChild = root.value.lastElementChild;
  lastChild.scrollIntoView();
}

/**
 * Sets the selected node, just in order to highlight it.
 * @param {Number} nodeIndex 
 */
function setSelectedNode(nodeIndex) {
  selectedNodeIndex.value = nodeIndex;
}

function handleClick(nodeIndex) {
  const {fen, fromFileIndex, fromRankIndex, toFileIndex, toRankIndex} = nodes.value[nodeIndex];
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

defineExpose({
  reset,
  addNode,
  setSelectedNode,
  scrollToLastElement,
});

</script>

<template>
    <div class="root" ref="root">
        <span v-for="(node, index) in nodes" :key="index" @click="() => handleClick(index)"
          :class="{selected: isSelectedNode(index)}"
        >
          {{ `${node.number  ?? ''}&nbsp;`}}{{ `${node.fan ?? ''}&nbsp;` }}
        </span>
    </div>
</template>

<style scoped>
@font-face {
  font-family: "Free Serif";
  src: url("../../assets/fonts/FreeSerif.ttf");
}

.root {
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  flex-wrap: wrap;
  align-content: flex-start;
  box-sizing: border-box;
  background-color: azure;
  width: v-bind(width);
  height: v-bind(height);
  font-family: 'Free Serif';
  font-size: 40px;
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