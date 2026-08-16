# Roger Panel

Self-hosted Minecraft server management panel with Linux-level isolation.

## Features

- **Server Management** — create, start, stop, restart Minecraft servers
- **Real-time Console** — WebSocket-powered terminal with xterm.js
- **Linux Isolation** — per-server namespaces (PID, MNT, UTS) + cgroups v2 (CPU, RAM, IOPS, PIDs) + network traffic shaping (tc)
- **File Manager** — browse, edit, upload server files directly from the panel
- **Backups** — scheduled and on-demand server backups
- **Monitoring** — CPU, RAM, TPS, player count, IOPS, network traffic with live graphs
- **Multi-user** — role-based access (admin, operator, viewer)
- **Plugin/Mod Support** — install plugins and mods from the panel
- **Multi-core** — Paper, Spigot, Purpur, Fabric, Forge, Vanilla, Mohist

## Design

**Material 3 Expressive** — Google's latest design system with emotion-driven UX, spring-based motion, dynamic color theming, and expressive shapes. Implemented via custom CSS tokens (no heavy UI library dependency).

Key specs: 45 color roles, 30 typography styles (15 baseline + 15 expressive), 10 shape tokens, motion physics (expressive + standard schemes), 6-level elevation system.

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Backend | Rust (Axum) |
| Frontend | React + Vite + TypeScript |
| Design System | Material 3 Expressive (custom CSS tokens) |
| Database | SQLite (SQLx) |
| Isolation | Linux namespaces + cgroups v2 |
| Console | WebSocket + xterm.js |
| Auth | JWT + argon2 |

## Getting Started

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Linux (namespaces/cgroups v2 required)

### Backend

```bash
cd backend
cargo run
```

### Frontend

```bash
cd frontend
npm install
npm run dev
```

### Requirements

The panel must run as **root** (or with `CAP_SYS_ADMIN`) for Linux namespace/cgroup isolation.

## References

- [Material 3 Expressive](https://m3.material.io/)
- [M3 Expressive Blog](https://m3.material.io/blog/building-with-m3-expressive)
- [Motion Physics](https://m3.material.io/styles/motion/overview)
- [Shape Library](https://m3.material.io/styles/shape/overview-principles)
- [Color System](https://m3.material.io/styles/color/system/overview)

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the full development roadmap.

## Made with AI

This project was designed and developed in collaboration with AI (OpenCode / big-pickle).

## License

MIT
