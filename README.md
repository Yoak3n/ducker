# Ducker

A desktop task management application built with Tauri v2 and React, featuring Live2D character display, multi-panel dashboard, and periodic task scheduling.

## Features

- **Task Management** - Create, edit, delete tasks with due dates, reminders, and hierarchical parent-child relationships
- **Periodic Tasks** - Schedule recurring tasks with configurable intervals (on startup, daily, weekly, monthly)
- **Action System** - Associate executable actions with tasks (command, file, directory, URL, notification, group); triggered on task completion or auto-execution when due
- **Multi-Panel Dashboard** - View tasks organized by Today, Weekly, and Monthly panels
- **Live2D Character** - Interactive desktop companion powered by pixi-live2d-display
- **MCP Server** - Optional `ducker-mcp` binary exposes tasks and actions over Model Context Protocol so AI assistants can read and manage your task library
- **System Tray** - Quick access from the system tray with task count display
- **Auto Start** - Launch on system boot
- **Global Shortcuts** - Keyboard shortcuts for quick access
- **Notifications** - Desktop notifications for task reminders
- **Sound Effects** - Audio feedback for task events
- **Internationalization** - Supports 13 languages (English, Chinese, Japanese, Korean, Arabic, German, Spanish, French, Indonesian, Russian, Turkish, Tatar, Traditional Chinese)

> **Platform note:** Action management IPC (create/execute actions) is currently compiled for Windows only. Tasks, Live2D, tray, notifications, and other core features work cross-platform via Tauri.

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust, Tauri v2, Tokio |
| Frontend | React 19, TypeScript, Vite, React Router |
| Styling | TailwindCSS, shadcn/ui, lucide-react |
| State | Zustand, SWR |
| Forms | React Hook Form, Zod |
| Database | SQLite (rusqlite) |
| Live2D | pixi.js, pixi-live2d-display |
| i18n | i18next, react-i18next |
| Audio | rodio |

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

## MCP Integration

Ducker ships a companion binary `ducker-mcp` that speaks the Model Context Protocol over stdio. After enabling **MCP Service** in Settings, AI assistants can list, create, update, and complete tasks (and manage actions) against the same SQLite database the GUI uses.

Example MCP client config (after registering the install directory to PATH):

```json
{
  "mcpServers": {
    "ducker": {
      "command": "ducker-mcp",
      "args": [],
      "env": {}
    }
  }
}
```

You can also call a single tool from the CLI:

```bash
ducker-mcp call task_list '{"completed": false}'
```

## Project Structure

```
ducker/
├── src/                    # Frontend source
│   ├── api/                # Tauri IPC API wrappers
│   ├── components/         # React components
│   │   ├── Action/         # Action management UI
│   │   ├── Date/           # Date/time pickers
│   │   ├── Layout/         # App layout & header
│   │   ├── Live2D/         # Live2D character display
│   │   ├── Panel/          # Dashboard panels (Today/Weekly/Monthly)
│   │   ├── Setting/        # Settings page
│   │   ├── Task/           # Task management UI
│   │   └── ui/             # shadcn/ui base components
│   ├── hooks/              # Custom React hooks
│   ├── lib/                # Shared utilities (e.g. cn)
│   ├── locales/            # i18n translation files
│   ├── mocks/              # Mock data for development
│   ├── pages/              # Route pages
│   ├── router/             # React Router configuration
│   ├── services/           # i18n & command services
│   ├── store/              # Zustand state stores
│   ├── types/              # TypeScript type definitions
│   └── utils/              # Utility functions
├── src-tauri/              # Rust backend source
│   ├── src/
│   │   ├── bin/            # ducker-mcp binary entry
│   │   ├── config/         # App configuration
│   │   ├── core/           # Core logic, Tauri commands, tray, windows
│   │   ├── feat/           # Feature modules
│   │   ├── mcp/            # MCP server implementation
│   │   ├── module/         # Lightweight mode, auto-launch
│   │   ├── process/        # Async process handling
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
