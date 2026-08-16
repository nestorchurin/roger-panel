# Roger Panel — Roadmap

## Phase 1: Foundation

- [ ] Initialize project (Cargo.toml, package.json)
- [ ] Database schema (SQLite migrations): servers, users, sessions, settings
- [ ] Base Axum server with auth middleware
- [ ] JWT authentication (login, register)
- [ ] React layout with routing

## Phase 2: Server Management

- [ ] Sandbox module: namespace isolation (unshare), cgroups v2
- [ ] Start / stop / restart Minecraft servers
- [ ] Real-time console via WebSocket (xterm.js)
- [ ] Server CRUD API

### Sandbox resource limits

- [ ] **CPU** — `cpu.max` (quota/period), `cpuset.cpus` (core pinning)
- [ ] **RAM** — `memory.max`, `memory.high`
- [ ] **IOPS** — `io.max` (read/write IOPS + bandwidth), `io.stat`
- [ ] **Network speed** — traffic shaping via `tc` (tbf/htb) on veth pair
- [ ] **Network usage** — per-server traffic accounting (bytes in/out) via nftables or veth stats
- [ ] **PIDs** — `pids.max`

## Phase 3: Files & Backups

- [ ] File manager (view, edit, download)
- [ ] Backup system (tar archives)
- [ ] SFTP server

## Phase 4: Monitoring & Admin

- [ ] CPU / RAM / TPS / player monitoring (cgroup stats + console parsing)
- [ ] Network traffic stats (live + historical)
- [ ] IOPS stats (live + historical)
- [ ] Live charts (recharts)
- [ ] User management & roles (admin, operator, viewer)
- [ ] Task scheduler (auto-start/stop, scheduled backups)

## Phase 5: Minecraft-specific

- [ ] Auto-download server versions (Paper API, Fabric, Forge, Vanilla)
- [ ] Plugin / mod installation from panel
- [ ] Server core updates
- [ ] EULA & first-run setup wizard
