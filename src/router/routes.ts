import { RouteRecordRaw } from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      { path: '', component: () => import('pages/AppPages/Home.vue') },
      { path: '/about', component: () => import('pages/AppPages/About.vue') },
      {
        path: '/about2',
        component: () => import('pages/AppPages/AboutModular.vue'),
      },
      {
        path: '/dynamic',
        component: () => import('pages/AppPages/Dynamic.vue'),
      },
      {
        path: '/props',
        component: () => import('pages/AppPages/PropsPage.vue'),
      },
      {
        path: '/simple_state',
        component: () => import('pages/AppPages/StatePage.vue'),
      },
      {
        path: '/bc_demo',
        component: () => import('pages/AppPages/BCPage.vue'),
      },
      {
        path: '/gh_demo',
        component: () => import('pages/AppPages/GHPage.vue'),
      },
    ],
  },
];

export default routes;
