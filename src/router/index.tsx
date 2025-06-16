import { createBrowserRouter } from 'react-router-dom';
import Home from '@/pages/Home';
import About from '@/pages/About';
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
    ],
  },
  {
    path: '*',
    element: <NotFound />,
  },
]);

export default router;