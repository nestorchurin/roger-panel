import { useEffect } from 'react';
import { useServersStore } from '../../stores';
import './Dashboard.css';

export default function DashboardPage() {
  const { servers, loading, fetchServers } = useServersStore();

  useEffect(() => {
    fetchServers();
  }, [fetchServers]);

  const running = servers.filter((s) => s.status === 'running').length;
  const stopped = servers.filter((s) => s.status === 'stopped').length;

  return (
    <div className="dashboard">
      <h1 className="page-title">Dashboard</h1>

      <div className="stats-grid">
        <div className="stat-card">
          <span className="material-symbols-outlined stat-icon">dns</span>
          <div className="stat-info">
            <span className="stat-value">{servers.length}</span>
            <span className="stat-label">Total Servers</span>
          </div>
        </div>
        <div className="stat-card">
          <span className="material-symbols-outlined stat-icon running">play_circle</span>
          <div className="stat-info">
            <span className="stat-value">{running}</span>
            <span className="stat-label">Running</span>
          </div>
        </div>
        <div className="stat-card">
          <span className="material-symbols-outlined stat-icon stopped">stop_circle</span>
          <div className="stat-info">
            <span className="stat-value">{stopped}</span>
            <span className="stat-label">Stopped</span>
          </div>
        </div>
      </div>

      {loading && <p className="loading-text">Loading servers...</p>}

      {!loading && servers.length === 0 && (
        <div className="empty-state">
          <span className="material-symbols-outlined empty-icon">dns</span>
          <h2>No servers yet</h2>
          <p>Create your first Minecraft server to get started.</p>
        </div>
      )}
    </div>
  );
}
