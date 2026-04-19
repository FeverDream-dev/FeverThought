# Architecture Overview

## Monorepo Structure

```
FeverThoth/
├── apps/desktop/              # Tauri v2 desktop application
│   ├── src-tauri/             #   Rust backend (IPC commands, Tauri builder)
│   └── src/                   #   React + TypeScript frontend
├── crates/                    # Rust workspace crates
│   ├── core/                  #   Shared types, error definitions, app state
│   ├── providers/             #   AI provider abstraction (Ollama, OpenAI, Gemini, etc.)
│   ├── lsp/                   #   Language Server Protocol client host
│   ├── agents/                #   Multi-agent AI pipeline (8 roles, 7 steps)
│   ├── workspace/             #   Project/file management, file watching
│   ├── settings/              #   Settings persistence (11 sections)
│   ├── security/              #   Audit log, privacy controls, permissioning
│   ├── mcp/                   #   Model Context Protocol integration
│   ├── git_tools/             #   Git operations via CLI
│   └── terminal/              #   Terminal session management
├── packages/                  # Shared frontend packages
├── .github/workflows/         # CI/CD (ci, release, nightly, security)
└── docs/                      # Documentation
```

## Data Flow

```
User Request
    ↓
Frontend (React)
    ↓ Tauri IPC
Rust Backend
    ↓
Agent Pipeline (7 steps):
  Intake → Clarify → GatherContext → Plan → Execute → Review → Summarize
    ↓
AI Providers (Ollama / OpenAI / Gemini / OpenRouter / Z.AI)
    ↓
Workspace / Git / Terminal / LSP / MCP
```

## Key Design Decisions

- **Tauri v2** instead of Electron for native performance and smaller bundles
- **rustls** instead of native-tls/OpenSSL for pure-Rust TLS
- **ollama-rs** for local AI with vision support (qwen2.5-vl)
- **Monaco Editor** for VS Code-grade editing experience
- **Frutiger Aero** design system with glassy translucent panels
