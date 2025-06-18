import { createBrowserRouter } from 'react-router-dom';
import { lazy } from 'react';


import Layout from '@/pages/Layout';

const Dashboard = lazy(() => import('@/pages/Dashboard'));
const About = lazy(() => import('@/pages/About'));
const NotFound = lazy(() => import('@/pages/NotFound'));
const Home = lazy(() => import('@/pages/Home'));
const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: 'about',
        element: <About />,
      },
      {
        path: 'dashboard',
        element: <Dashboard />,
      },
    ],
  },
  {
    path: '*',
    element: <NotFound />,
  },
]);

export default router;