# Ducker

A desktop task management application built with Tauri v2 and React, featuring Live2D character display, multi-panel dashboard, and periodic task scheduling.

## Features

- **Task Management** - Create, edit, delete tasks with due dates, reminders, and hierarchical parent-child relationships
- **Periodic Tasks** - Schedule recurring tasks with configurable intervals
- **Action System** - Associate executable actions with tasks, triggered on task completion
- **Multi-Panel Dashboard** - View tasks organized by Today, Weekly, and Monthly panels
- **Live2D Character** - Interactive desktop companion powered by pixi-live2d-display
- **System Tray** - Quick access from the system tray with task count display
- **Auto Start** - Launch on system boot
- **Global Shortcuts** - Keyboard shortcuts for quick access
- **Notifications** - Desktop notifications for task reminders
- **Sound Effects** - Audio feedback for task events
- **Internationalization** - Supports 13 languages (English, Chinese, Japanese, Korean, Arabic, German, Spanish, French, Indonesian, Russian, Turkish, Tatar, Traditional Chinese)

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust, Tauri v2 |
| Frontend | React 19, TypeScript, Vite |
| Styling | TailwindCSS, shadcn/ui |
| State | Zustand |
| Forms | React Hook Form, Zod |
| Database | SQLite (rusqlite) |
| i18n | i18next, react-i18next |

## Prerequisites

- [Node.js](https://nodejs.org/) (LTS)
- [pnpm](https://pnpm.io/)
- [Rust](https://www.rust-lang.org/tools/install) (stable, >= 1.77.2)
- [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/)

## Getting Started

```bash
# Clone the repository
git clone https://github.com/Yoaken/ducker.git
cd ducker

# Install dependencies
pnpm install

# Start development server
pnpm tauri dev
```

## Build

```bash
# Build for production
pnpm tauri build
```

The built installer (NSIS) will be located in `src-tauri/target/release/bundle/`.

## Project Structure

```
ducker/
├── src/                    # Frontend source
│   ├── api/                # Tauri IPC API wrappers
│   ├── components/         # React components
│   │   ├── Action/         # Action management UI
│   │   ├── Layout/         # App layout & header
│   │   ├── Live2D/         # Live2D character display
│   │   ├── Panel/          # Dashboard panels (Today/Weekly/Monthly)
│   │   ├── Setting/        # Settings page
│   │   ├── Task/           # Task management UI
│   │   └── ui/             # shadcn/ui base components
│   ├── hooks/              # Custom React hooks
│   ├── locales/            # i18n translation files
│   ├── pages/              # Route pages
│   ├── router/             # React Router configuration
│   ├── services/           # i18n & command services
│   ├── store/              # Zustand state stores
│   ├── types/              # TypeScript type definitions
│   └── utils/              # Utility functions
├── src-tauri/              # Rust backend source
│   ├── src/
│   │   ├── core/           # Core business logic & Tauri commands
│   │   ├── feat/           # Feature modules
│   │   ├── schema/         # Data models & schemas
│   │   ├── service/        # Task scheduling & execution
│   │   ├── store/          # SQLite database layer
│   │   └── utils/          # Utility modules
│   └── resources/          # App resources (sounds, etc.)
├── public/                 # Static assets (Live2D models)
└── .github/workflows/      # CI/CD (GitHub Actions)
```

## License

MIT
