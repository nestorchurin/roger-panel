# Roger Panel — План розробки

Аналог Crafty Panel, зациклений суто на Minecraft серверах.
Підтримка різних ядер: Paper, Spigot, Purpur, Fabric, Forge, Vanilla, Mohist та ін.

## Стек

| Компонент | Технологія |
|-----------|-----------|
| Backend | Rust (Axum) |
| Frontend | React + Vite + TypeScript |
| База даних | SQLite (SQLx) |
| Ізоляція | Linux namespaces + cgroups v2 (напряму, без Docker) |
| Консоль | WebSocket + xterm.js |
| Auth | JWT + argon2 |

## Архітектура

```
┌─────────────────────────────────────────┐
│              Roger Panel                 │
│                                         │
│  ┌──────────┐     ┌──────────────────┐  │
│  │ Frontend │────▶│ Backend (Axum)   │  │
│  │ React +  │ WS  │ Rust             │  │
│  │ Vite     │◀────│                  │  │
│  └──────────┘     │  ┌────────────┐  │  │
│                   │  │  Sandbox   │  │  │
│                   │  │ namespaces │  │  │
│                   │  │ cgroups v2 │  │  │
│                   │  └──────┬─────┘  │  │
│                   └─────────┼────────┘  │
│                             │           │
│                    ┌────────▼────────┐  │
│                    │ Minecraft Server│  │
│                    │  (isolated)     │  │
│                    └─────────────────┘  │
└─────────────────────────────────────────┘
```

## Структура проекту

```
RogerPanel/
├── backend/
│   ├── Cargo.toml
│   ├── migrations/
│   │   └── 001_initial.sql
│   └── src/
│       ├── main.rs
│       ├── config.rs
│       ├── api/
│       │   ├── mod.rs
│       │   ├── auth.rs
│       │   ├── servers.rs
│       │   ├── console.rs
│       │   ├── files.rs
│       │   ├── backups.rs
│       │   ├── users.rs
│       │   └── admin.rs
│       ├── db/
│       │   ├── mod.rs
│       │   ├── models.rs
│       │   └── queries.rs
│       ├── auth/
│       │   ├── mod.rs
│       │   ├── jwt.rs
│       │   └── middleware.rs
│       ├── sandbox/
│       │   ├── mod.rs
│       │   ├── process.rs
│       │   ├── namespace.rs
│       │   ├── cgroup.rs
│       │   ├── fs.rs
│       │   └── monitor.rs
│       └── minecraft/
│           ├── mod.rs
│           ├── versions.rs
│           ├── parser.rs
│           └── eula.rs
├── frontend/
│   ├── package.json
│   ├── vite.config.ts
│   ├── tsconfig.json
│   └── src/
│       ├── main.tsx
│       ├── App.tsx
│       ├── api/
│       ├── components/
│       │   ├── Layout/
│       │   ├── Console/
│       │   ├── FileManager/
│       │   ├── ServerStatus/
│       │   ├── Charts/
│       │   └── Auth/
│       ├── pages/
│       │   ├── Dashboard/
│       │   ├── Servers/
│       │   ├── ServerDetail/
│       │   ├── Backups/
│       │   ├── Users/
│       │   └── Settings/
│       ├── hooks/
│       └── stores/
└── PLAN.md
```

## Фази розробки

### Фаза 1 — Фундамент
- [ ] Ініціалізація проєкту (Cargo.toml, package.json)
- [ ] Схема БД (SQLite міграції): servers, users, sessions, settings
- [ ] Базовий Axum сервер з auth middleware
- [ ] JWT аутентифікація (login, register)
- [ ] React layout з routing

### Фаза 2 — Керування серверами
- [ ] Sandbox модуль: namespace isolation (unshare), cgroups v2 (CPU, RAM)
- [ ] Запуск/зупинка/рестарт Minecraft серверів
- [ ] Реалтайм консоль через WebSocket (xterm.js)
- [ ] CRUD серверів через API

### Фаза 3 — Файли та бекапи
- [ ] Файловий менеджер (перегляд, редагування, завантаження)
- [ ] Система бекапів (tar архіви)
- [ ] SFTP сервер

### Фаза 4 — Моніторинг та адмін
- [ ] Моніторинг CPU/RAM/TPS/гравців (cgroup stats + парсинг консолі)
- [ ] Графіки (recharts)
- [ ] Система користувачів та ролей (admin, operator, viewer)
- [ ] Планувальник завдань

### Фаза 5 — Minecraft-специфічне
- [ ] Автозавантаження версій (Paper API, Fabric, Forge, Vanilla)
- [ ] Встановлення плагінів/модів
- [ ] Оновлення ядер
- [ ] EULA та首次 setup wizard

## Sandbox: Linux Isolation

### Namespace types
- **PID namespace** — ізоляція процесів
- **NET namespace** — ізоляція мережі (опційно)
- **MNT namespace** — ізоляція ФС (mount + pivot_root)
- **UTS namespace** — ізоляція hostname

### Cgroups v2
- **CPU**: `cpu.max` (quota + period)
- **Memory**: `memory.max`, `memory.high`
- **PIDs**: `pids.max`
- **IO**: `io.max` (bandwidth)

### Кожен сервер отримує:
- Свою директорію (`data/servers/<id>/`)
- Свій cgroup
- Свій namespace set
- Обмежений rootfs (server files + java + lib)

## Залежності

### Backend (Rust)
- `axum` — HTTP framework
- `sqlx` — SQLite (compile-time checked queries)
- `jsonwebtoken` — JWT auth
- `tokio` — async runtime
- `tower-http` — CORS, static files
- `uuid` — IDs
- `serde` / `serde_json` — serialization
- `tracing` — logging
- `sha2` / `argon2` — password hashing

### Frontend
- React 18 + TypeScript
- Vite
- React Router v6
- Zustand — state management
- xterm.js — terminal emulator
- recharts — charts
- axios — HTTP client
