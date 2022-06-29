<script setup>

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

function knightText(white) {
    return white ? '\u2658' : '\u265e';
}

function bishopText(white) {
    return white ? '\u2657' : '\u265d';
}

function rookText(white) {
    return white ? '\u2656' : '\u265c';
}

function queenText(white) {
    return white ? '\u2655' : '\u265b';
}

function kingText(white) {
    return white ? '\u2654' : '\u265a';
}

</script>

<template>
    <div class="root">
        1. e4 e5 2.{{knightText(false)}}c6 {{bishopText(true)}}c5
        1. e4 e5 2.{{knightText(false)}}c6 {{bishopText(true)}}c5
        1. e4 e5 2.{{knightText(false)}}c6 {{bishopText(true)}}c5
        1. e4 e5 2.{{knightText(false)}}c6 {{bishopText(true)}}c5
        1. e4 e5 2.{{knightText(false)}}c6 {{bishopText(true)}}c5
        1. e4 e5 2.{{knightText(false)}}c6 {{bishopText(true)}}c5
    </div>
</template>

<style scoped>
@font-face {
  font-family: "Free Serif";
  src: url("../../assets/fonts/FreeSerif.ttf");
}

.root {
    background-color: azure;
    width: v-bind(width);
    height: v-bind(height);
    font-family: 'Free Serif';
    font-size: xx-large;
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    align-items: flex-start;
}
</style>