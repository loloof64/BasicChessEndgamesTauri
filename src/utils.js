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

export {
    knightText,
    bishopText,
    rookText,
    queenText,
    kingText
};