import { Outlet } from 'react-router-dom';
import Header from '@/components/Layout/Header';

const Layout = () => {
  return (
    <div className="app-container">
      <Header />
      <main>
        <Outlet />
      </main>
    </div>
  );
};

export default Layout;