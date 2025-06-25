import { createBrowserRouter } from 'react-router-dom';
import { lazy } from 'react';


import Layout from '@/pages/Layout';

const Dashboard = lazy(() => import('@/pages/Dashboard'));
const About = lazy(() => import('@/pages/About'));
const NotFound = lazy(() => import('@/pages/NotFound'));
const Home = lazy(() => import('@/pages/Home'));
const router = createBrowserRouter([
  {
    path: '/main',
    index: true,
    element: <Home />,
  },{
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <Dashboard />,
      },{
        path: "about",
        element: <About />
      }
    ],
  },
  {
    path: '*',
    element: <NotFound />,
  },
]);

export default router;