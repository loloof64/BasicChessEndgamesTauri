import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHashHistory } from 'vue-router';
import App from './App.vue'


import BalmUI from 'balm-ui'; // Official Google Material Components
import BalmUIPlus from 'balm-ui-plus'; // BalmJS Team Material Components
import 'balm-ui-css';

import GamePage from './components/pages/GamePage.vue';
import ChessBoard from '@loloof64/chessboard-component/dist';

const routes = [
    { path: '/', name: 'game', component: GamePage },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

const pinia = createPinia()
const app = createApp(App)

app.use(router)
app.use(pinia)

app.use(BalmUI);
app.use(BalmUIPlus);

app.mount('#app')
