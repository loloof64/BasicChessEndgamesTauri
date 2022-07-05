import { defineStore } from "pinia";

export const useStore = defineStore("options", {
  state: () => {
    return {
      selectedEnginePath: undefined,
    };
  },
  getters: {},
  actions: {
    setSelectedEnginePath(newPath) {
      this.selectedEnginePath = newPath;
    },
  },
});
