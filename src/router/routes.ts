import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/AppPages/Home.vue') },
      { path: '/profile', component: () => import('pages/AppPages/ProfilePage.vue') },
      { path: '/gist', component: () => import('pages/AppPages/GistPage.vue') },
      { path: '/repositories', component: () => import('pages/AppPages/RepositoriyPage.vue') },
    ],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
