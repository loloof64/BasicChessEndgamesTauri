import { defineStore } from "pinia";
import { DEFAULT_FEN } from "../constants";

export const useStore = defineStore('game', {
    state: () => {
        return {
            positionFen: DEFAULT_FEN,
        }
    },
     actions: {
        resetToDefaultGame() {
            this.positionFen = DEFAULT_FEN;
        }
     }
});