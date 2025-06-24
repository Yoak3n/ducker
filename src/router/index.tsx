import { createBrowserRouter } from 'react-router-dom';
import { lazy } from 'react';


import Layout from '@/pages/Layout';

const Dashboard = lazy(() => import('@/pages/Dashboard'));
const About = lazy(() => import('@/pages/About'));
const NotFound = lazy(() => import('@/pages/NotFound'));
const Home = lazy(() => import('@/pages/Home'));
const TaskDemo = lazy(() => import('@/pages/TaskDemo'));
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
        path: "dashboard",
        element: <Dashboard />,
      },{
        path: "about",
        element: <About />
      },{
        path: "task-demo",
        element: <TaskDemo />
      }
    ],
  },
  {
    path: '*',
    element: <NotFound />,
  },
]);

export default router;