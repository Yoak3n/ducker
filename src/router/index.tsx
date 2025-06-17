import { createBrowserRouter } from 'react-router-dom';
import Home from '@/pages/Home';
import About from '@/pages/About';
import Dashboard from '@/pages/Dashboard';
import NotFound from '@/pages/NotFound';
import Layout from '@/pages/Layout';

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