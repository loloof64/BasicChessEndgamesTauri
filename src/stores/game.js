import { defineStore } from "pinia";
import { DEFAULT_FEN, EMPTY_FEN } from "../constants";

export const useStore = defineStore('game', {
    state: () => {
        return {
            startPosition: EMPTY_FEN,
        }
    },
     actions: {
        resetToDefaultGame() {
            this.startPosition = DEFAULT_FEN;
        },
        startNewGame(customStartPosition) {
            this.startPosition = customStartPosition;
        }
     }
});