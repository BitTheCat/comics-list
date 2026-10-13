import { createRouter, createWebHistory } from 'vue-router';
import type { RouteLocationNormalized } from 'vue-router';
import SeriesList from '../pages/SeriesList.vue';
import EntryList from '../pages/EntryList.vue';

const routes = [
  { path: '/', redirect: '/series' },
  { path: '/series', name: 'series', component: SeriesList },
  {
    path: '/series/:id',
    name: 'series-detail',
    component: EntryList,
    props: (route: RouteLocationNormalized) => ({
      id: route.params.id,
      minYear: Number(route.query.min_year) || 1950,
    }),
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;

