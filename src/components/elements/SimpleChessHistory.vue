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
 * @param nodeIndex: Number - index of node in history component
 * @param fen: String - the requested position in Forsyhth-Edwards Notation
 * @param fromFileIndex: Number - the start file index of matching move.
 * @param fromRankIndex: Number - the start rank index of matching move.
 * @param toFileIndex: Number - the end file index of matching move.
 * @param toRankIndex: Number - the end rank index of matching move.
 */
const emit = defineEmits(['requestNodeSelected']);

const nodes = ref([]);
const root = ref();

/**
 * Clears history.
 * @param startMoveNumber - Number : first move number of the game.
 * @param startsAsWhite - Boolean : true if white starts the game, false otherwise.
 */
function reset(startMoveNumber, startsAsWhite) {
  const moveNumberText = `${startMoveNumber}.${startsAsWhite ? '' : '..'}`;
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

defineExpose({
  reset,
  addNode,
  scrollToLastElement,
});

</script>

<template>
    <div class="root" ref="root">
        <span v-for="(node, index) in nodes" :key="index" @click="() => handleClick(index)">
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
</style>