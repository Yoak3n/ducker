import { lazy } from 'react';
import { createBrowserRouter } from 'react-router-dom';

import Layout from '@/pages/Layout';
import ActionsList from '@/components/Action/ActionsList';
import ActionModify from '@/components/Action/ActionModify';
import Setting from '@/pages/Setting';

const Dashboard = lazy(() => import('@/pages/Dashboard'));
const Action = lazy(() => import('@/pages/Action'));
const About = lazy(() => import('@/pages/About'));
const NotFound = lazy(() => import('@/pages/NotFound'));
const Home = lazy(() => import('@/pages/Home'));



const router = createBrowserRouter([
  {
    path: '/main',
    element: <Home />,
  },{
    path: "/",
    element: <Layout />,
    children: [
      {
        index: true,
        element: <Dashboard />,
      },{
        path: "action",
        element: <Action />,
        children: [
          {
            index: true,
            element: <ActionsList />
          },
          {
            path: "modify/:id",
            element: <ActionModify />
          }
        ]
      },{
        path: "about",
        element: <About />
      },{
        path: "setting",
        element: <Setting />

      }
    ],
  },
  {
    path: '*',
    element: <NotFound />,
  },
]);

export default router;