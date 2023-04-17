// TODO(altf02): possibly use https://github.com/hannoeru/vite-plugin-pages

import { createRouter, createWebHashHistory } from "vue-router"
import LoadingGameVue from "./views/LoadingGame.vue";
import MainMenu from "./views/MainMenu.vue";
import SinglePlayer from "./views/SinglePlayer.vue";
import SettingsMenu from "./views/SettingsMenu.vue";
import NewGame from "./views/NewGame.vue";


const routes = [
    { path: '/', redirect: 'singleplayer' },
    { path: '/loading', component: LoadingGameVue, name: 'loading' },
    { path: '/newgame', component: NewGame, name: 'new-game' },
    // TODO(altf02): create loadgame component
    { path: '/loadgame', component: MainMenu, name: 'load-game' },
    { path: '/menu', component: MainMenu, name: 'main-menu' },
    { path: '/singleplayer', component: SinglePlayer, name: 'singleplayer' },
    { path: '/settings', component: SettingsMenu, name:  'settings' }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes,
})

export default router
