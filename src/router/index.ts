import { createRouter, createWebHistory } from 'vue-router';
import SeriesList from '../pages/SeriesList.vue';
import EntriesList from '../pages/EntriesList.vue';

const routes = [
  { path: '/', redirect: '/series' },
  { path: '/series', name: 'series', component: SeriesList },
  { path: '/series/:id', name: 'series-detail', component: EntriesList, props: true },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

