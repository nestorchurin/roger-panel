import { Navigate, Outlet, Link, useLocation } from 'react-router-dom';
import { useAuthStore } from '../../stores';
import './Layout.css';

const navItems = [
  { path: '/', label: 'Dashboard', icon: 'dashboard' },
  { path: '/servers', label: 'Servers', icon: 'dns' },
  { path: '/backups', label: 'Backups', icon: 'backup' },
  { path: '/users', label: 'Users', icon: 'people' },
  { path: '/settings', label: 'Settings', icon: 'settings' },
];

export default function Layout() {
  const { isAuthenticated, logout } = useAuthStore();
  const location = useLocation();

  if (!isAuthenticated) {
    return <Navigate to="/login" replace />;
  }

  return (
    <div className="layout">
      <nav className="sidebar">
        <div className="sidebar-header">
          <span className="logo-text">Roger Panel</span>
        </div>
        <div className="nav-items">
          {navItems.map((item) => (
            <Link
              key={item.path}
              to={item.path}
              className={`nav-item ${location.pathname === item.path ? 'active' : ''}`}
            >
              <span className="material-symbols-outlined">{item.icon}</span>
              <span className="nav-label">{item.label}</span>
            </Link>
          ))}
        </div>
        <div className="sidebar-footer">
          <button className="nav-item logout-btn" onClick={logout}>
            <span className="material-symbols-outlined">logout</span>
            <span className="nav-label">Logout</span>
          </button>
        </div>
      </nav>
      <main className="main-content">
        <Outlet />
      </main>
    </div>
  );
}
