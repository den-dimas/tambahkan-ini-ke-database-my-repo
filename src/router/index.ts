import { createRouter, createWebHistory } from 'vue-router';
import { useAuthStore } from '../stores/auth';
import LandingPage from '../views/LandingPage.vue';
import ExplorePage from '../views/ExplorePage.vue';
import ProfilePage from '../views/ProfilePage.vue';
import SearchPage from '../views/SearchPage.vue';
import AuthForm from '../components/AuthForm.vue';
import ApprovalPage from '../views/ApprovalPage.vue';
import AddPersonPage from '../views/AddPersonPage.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'landing',
      component: LandingPage,
    },
    {
      path: '/login',
      name: 'login',
      component: AuthForm,
      meta: { guestOnly: true },
    },
    {
      path: '/explore',
      name: 'explore',
      component: ExplorePage,
    },
    {
      path: '/profile',
      name: 'profile',
      component: ProfilePage,
      meta: { requiresAuth: true },
    },
    {
      path: '/search',
      name: 'search',
      component: SearchPage,
    },
    {
      path: '/approvals',
      name: 'approvals',
      component: ApprovalPage,
      meta: { requiresAuth: true },
    },
    {
      path: '/add',
      name: 'add-person',
      component: AddPersonPage,
      meta: { requiresAuth: true },
    },
  ],
});

// Navigation guard
router.beforeEach((to, from, next) => {
  const authStore = useAuthStore();
  const isAuthenticated = authStore.isAuthenticated;

  if (to.meta.requiresAuth && !isAuthenticated) {
    next({ name: 'login' });
  } else if (to.meta.guestOnly && isAuthenticated) {
    next({ name: 'landing' });
  } else {
    next();
  }
});

export default router;
