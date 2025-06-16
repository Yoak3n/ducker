import { Link, Outlet } from 'react-router-dom';

const Layout = () => {
  return (
    <div className="app-container">
      <header>
        <nav>
          <ul className="nav-links">
            <li>
              <Link to="/">首页</Link>
            </li>
            <li>
              <Link to="/about">关于</Link>
            </li>
          </ul>
        </nav>
      </header>
      <main>
        <Outlet />
      </main>
      <footer>
        <p>© 2023 Ducker 应用</p>
      </footer>
    </div>
  );
};

export default Layout;