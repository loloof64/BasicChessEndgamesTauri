import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHashHistory } from 'vue-router';
import { createI18n } from 'vue-i18n'
import App from './App.vue'

import { library } from '@fortawesome/fontawesome-svg-core'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faFastBackward, faFastForward, faBackwardStep, faForwardStep } from '@fortawesome/free-solid-svg-icons'


import BalmUI from 'balm-ui'; // Official Google Material Components
import BalmUIPlus from 'balm-ui-plus'; // BalmJS Team Material Components
import 'balm-ui-css';

import GamePage from '@/pages/GamePage.vue';
import OptionsPage from '@/pages/OptionsPage.vue';
import ChessBoard from '@loloof64/chessboard-component/dist';

import translations from './i18n';

const routes = [
    { path: '/', name: 'game', component: GamePage },
    { path: '/options', name: 'options', component: OptionsPage },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

const pinia = createPinia()

function getLocale() {
    if (navigator.languages != undefined) 
        return navigator.languages[0]; 
    return navigator.language.substring(0, 2);
}
const locale = getLocale();

const i18n = createI18n({
    locale,
    fallbackLocale: 'en',
    messages: translations,
})

library.add(faFastBackward, faFastForward, faBackwardStep, faForwardStep);

const app = createApp(App)

app.component('font-awesome-icon', FontAwesomeIcon)
app.use(router)
app.use(pinia)
app.use(i18n)

app.use(BalmUI);
app.use(BalmUIPlus);

app.mount('#app')
