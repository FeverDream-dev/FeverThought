# Development Setup Guide

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Node.js | 20+ | https://nodejs.org/ |
| pnpm | 9+ | `npm install -g pnpm` |
| Rust | Stable | https://rustup.rs/ |
| Tauri v2 | Latest | https://v2.tauri.app/start/prerequisites/ |
| Ollama | Latest | https://ollama.com/ (optional, for local AI) |

## Quick Start

```bash
git clone https://github.com/FeverDream/FeverThoth.git
cd FeverThoth
pnpm install
pnpm dev
```

## Building

```bash
# Build frontend only
pnpm build

# Build full Tauri app
pnpm --filter @feverthoth/desktop tauri build
```

## Development Workflow

1. Start dev server: `pnpm dev`
2. Make changes — Vite hot-reloads the frontend
3. Rust changes trigger automatic rebuild via Tauri
4. Run checks before pushing: `./scripts/check.sh`

## Rust Crates

Each crate can be built and tested independently:

```bash
cargo check -p feverthoth-agents
cargo test -p feverthoth-providers
```

## Running with Ollama

1. Install and start Ollama: https://ollama.com/
2. Pull models: `ollama pull llama3.2` and `ollama pull qwen2.5-vl`
3. The IDE auto-detects Ollama on localhost:11434
