<script setup>
import {ref} from 'vue';

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
 * - number: String? (can be undefined) - the move number text
 * - fan: String? (can be undefined) - the move text without the number
 * - fen: String? (can be undefined) - the position value resulting from move in Forstyh-Edwards Notation
 *  
 */
function addNode(parameters) {
  const  {number, fan, fen} = parameters;
  nodes.value = [...nodes.value, {number, fan, fen}];
}

/**
 * Scroll, so that the last child is visible.
 */
function scrollToLastElement() {
  const lastChild = root.value.lastElementChild;
  lastChild.scrollIntoView();
}

defineExpose({
  reset,
  addNode,
  scrollToLastElement,
});

</script>

<template>
    <div class="root" ref="root">
        <span v-for="(node, index) in nodes" :key="index">
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