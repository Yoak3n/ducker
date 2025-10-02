import { Outlet } from 'react-router-dom';
import Header from '@/components/Layout/Header';

const Layout = () => {
  return (
    <div className="app-container">
      <Header />
      <main className='max-h-[calc(100vh-4rem)]'>
        <Outlet />
      </main>
    </div>
  );
};

export default Layout;