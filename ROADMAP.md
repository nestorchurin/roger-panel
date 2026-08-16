# Roger Panel — Roadmap

## Phase 1: Foundation

- [ ] Initialize project (Cargo.toml, package.json)
- [ ] Database schema (SQLite migrations): servers, users, sessions, settings
- [ ] Base Axum server with auth middleware
- [ ] JWT authentication (login, register)
- [ ] React layout with routing

## Phase 2: Server Management

- [ ] Sandbox module: namespace isolation (unshare), cgroups v2 (CPU, RAM)
- [ ] Start / stop / restart Minecraft servers
- [ ] Real-time console via WebSocket (xterm.js)
- [ ] Server CRUD API

## Phase 3: Files & Backups

- [ ] File manager (view, edit, download)
- [ ] Backup system (tar archives)
- [ ] SFTP server

## Phase 4: Monitoring & Admin

- [ ] CPU / RAM / TPS / player monitoring (cgroup stats + console parsing)
- [ ] Live charts (recharts)
- [ ] User management & roles (admin, operator, viewer)
- [ ] Task scheduler (auto-start/stop, scheduled backups)

## Phase 5: Minecraft-specific

- [ ] Auto-download server versions (Paper API, Fabric, Forge, Vanilla)
- [ ] Plugin / mod installation from panel
- [ ] Server core updates
- [ ] EULA & first-run setup wizard
