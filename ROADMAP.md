# Roger Panel — Roadmap

## Design System: Material 3 Expressive

Custom CSS tokens implementation — no heavy UI library. Full control over motion, color, shapes.

### Color (45 roles)
- 3 accent groups (Primary, Secondary, Tertiary) × 4 roles each (color, on, container, on-container)
- Error group × 4 roles
- 10 surface roles (surface, surface-dim, surface-bright, 5 container levels, on-surface, on-surface-variant)
- Inverse group (surface, on-surface, primary)
- Outline group (outline, outline-variant)
- Utility (shadow, scrim)
- Dynamic color: seed → 5 tonal palettes → light/dark schemes

### Typography (30 styles)
- 5 categories: Display, Headline, Title, Body, Label
- 3 sizes each: Large, Medium, Small
- Baseline + Emphasized variants (M3 Expressive)
- Token naming: `--md-sys-typescale-{category}-{size}-{property}`

### Shape (10 corner tokens + 35 decorative shapes)
- Tokens: none(0) → extra-small(4) → small(8) → medium(12) → large(16) → large-increased(20) → extra-large(28) → extra-large-increased(32) → extra-extra-large(48) → full(9999px)
- Shape morph motion for transitions

### Motion (2 schemes)
- **Expressive**: bouncy, elastic springs for hero moments
- **Standard**: minimal bounce, utilitarian
- Easing: emphasized, emphasized-decelerate, emphasized-accelerate, standard variants
- Duration: short1-4 (50-200ms), medium1-4 (250-400ms), long1-4 (450-600ms), extra-long1-4 (700-1000ms)
- Spring tokens: `cubic-bezier(...)` values per scheme

### Elevation
- 6 levels (0-5dp): 0, 1, 3, 6, 8, 12dp
- Tonal surface color communicates elevation (not just shadows)

## Phase 1: Foundation

- [ ] Initialize project (Cargo.toml, package.json)
- [ ] M3 Expressive design tokens (CSS custom properties: 45 color roles, 30 typography styles, 10 shape tokens, motion easing/duration tokens, 6-level elevation)
- [ ] Database schema (SQLite migrations): servers, users, sessions, settings
- [ ] Base Axum server with auth middleware
- [ ] JWT authentication (login, register)
- [ ] React layout with routing + base M3 components (buttons, cards, inputs, nav)

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
