# FeverThoth IDE

**AI-first, Rust-powered desktop IDE** — built by [FeverDream](https://github.com/FeverDream).

A cross-platform coding environment designed for vibe-coding: plan-first AI workflows, privacy-aware architecture, and a retro-futuristic Frutiger Aero aesthetic that feels like a polished Windows Vista-era dream.

---

## Features

- **AI-First Architecture** — 8-agent pipeline with 7-step state machine, local Ollama + cloud providers
- **Rust Core** — Tauri v2 with 10 workspace crates, native performance, not Electron bloat
- **Monaco Editor** — VS Code-grade editing with multi-tab, split panes, and Intellisense
- **LSP Integration** — Rust Analyzer, TypeScript, Python, Go, C/C++ out of the box
- **Integrated Terminal** — Multi-session terminal with shell detection
- **Git Integration** — Branch management, diff preview, commit, status via CLI
- **Frutiger Aero UI** — Glassy translucent panels, glossy buttons, nostalgic optimism
- **Privacy-First AI** — Local Ollama by default, screenshots never leave your machine
- **Clarification UX** — The AI asks before guessing, with structured follow-up options
- **Permission System** — 8 permission classes, 3 risk tiers, user review for sensitive actions
- **Model Routing** — Automatic provider selection based on task type, privacy, and cost
- **Cross-Platform** — Windows, macOS, Linux from a single codebase

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Shell | Tauri v2 |
| Backend | Rust (10 workspace crates) |
| Frontend | React + TypeScript + Vite |
| Editor | Monaco Editor |
| State Management | Zustand |
| Language Intelligence | LSP (rust-analyzer, typescript-language-server, etc.) |
| Terminal | tokio process + xterm.js |
| Git | CLI-based via tokio::process |
| Local AI | Ollama (qwen2.5-vl, llama3.2) |
| Cloud AI | OpenAI, Gemini, OpenRouter, Z.AI |
| MCP | Chrome DevTools, Playwright, custom |
| TLS | rustls (no OpenSSL dependency) |

---

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 20+
- [pnpm](https://pnpm.io/) 9+
- [Rust](https://rustup.rs/) (stable)
- [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/)
- [Ollama](https://ollama.com/) (optional, for local AI)

### Install

```bash
git clone https://github.com/FeverDream/FeverThoth.git
cd FeverThoth
./scripts/setup.sh
```

### Development

```bash
pnpm dev
```

### Build

```bash
pnpm build
pnpm --filter @feverthoth/desktop tauri build
```

---

## Project Structure

```
FeverThoth/
├── apps/desktop/              # Tauri v2 desktop app
│   ├── src-tauri/             # Rust backend
│   │   ├── src/
│   │   │   ├── lib.rs         # App entry + Tauri builder
│   │   │   ├── main.rs        # Desktop entry point
│   │   │   └── commands/      # Tauri IPC commands
│   │   ├── Cargo.toml
│   │   └── tauri.conf.json
│   └── src/                   # React frontend
│       ├── components/        # ActivityBar, Sidebar, Editor, Panel, RightPanel, etc.
│       ├── panels/            # FileTree, AiPanel, Search, Git, Settings, DiffCard, etc.
│       ├── editor/            # Monaco editor wrapper, tabs, welcome screen
│       ├── ai/                # ClarificationWidget, OnboardingWizard, ProviderSetupWizard
│       ├── stores/            # Zustand state management
│       └── styles/            # Frutiger Aero design system (tokens.css + main.css)
├── crates/                    # Rust workspace crates (10)
│   ├── core/                  # Shared types, error definitions, app state
│   ├── providers/             # AI provider abstraction (Ollama, OpenAI, Gemini, etc.)
│   ├── lsp/                   # LSP client host + JSON-RPC transport
│   ├── agents/                # 8-role agent pipeline, context engineering, routing, permissions
│   ├── workspace/             # Project/file management
│   ├── settings/              # Settings persistence (11 sections)
│   ├── security/              # Audit log, privacy controls
│   ├── mcp/                   # MCP integration
│   ├── git_tools/             # Git operations via CLI
│   └── terminal/              # Terminal session management
├── packages/
│   └── shared-types/          # Shared TypeScript type definitions
├── .github/
│   ├── workflows/             # ci.yml, release.yml, nightly.yml, security.yml
│   ├── ISSUE_TEMPLATE/        # Bug report + feature request forms
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── config.yml
├── docs/                      # Architecture + setup guides
├── scripts/                   # setup.sh, check.sh
├── Cargo.toml                 # Rust workspace root
├── package.json               # pnpm workspace root
└── turbo.json                 # Turborepo config
```

---

## AI Architecture

The AI system is built around a **7-step pipeline** with **8 agent roles**:

### Pipeline Steps

1. **Intake** — Parse request, detect ambiguity, scope, risk level
2. **Clarify** — Ask focused questions with multiple-choice options if ambiguous
3. **GatherContext** — Build relevant files list, architecture notes, change surface
4. **Plan** — Generate structured plan with assumptions, steps, validation
5. **Execute** — Implementer applies approved changes
6. **Review** — Check correctness, drift, edge cases, unsafe commands
7. **Summarize** — Generate commit message and change summary

### Agent Roles

| Role | Purpose |
|------|---------|
| Intent Clarifier | Prevents incorrect work caused by ambiguity |
| Planner | Creates concise, high-quality implementation plans |
| Repo Cartographer | Maps relevant files, dependencies, architecture |
| Tool Router | Selects smallest reliable toolset for each task |
| Implementer | Makes only the changes required by the approved plan |
| Reviewer | Inspects for correctness, drift, missed edge cases |
| Browser/UI Agent | Visual debugging via MCP browser tooling |
| Git Summarizer | Produces commit messages and change summaries |

### Provider System

```
AiProvider trait
├── OllamaProvider     (local, vision-capable)
├── OpenAiProvider     (cloud, GPT-4o)
├── GeminiProvider     (cloud, Gemini 2.0)
├── OpenRouterProvider (cloud, multi-model)
└── ZaiCodingProvider  (cloud, coding-specialized)
```

Each provider implements the same `AiProvider` trait with capability flags for text, vision, tools, reasoning, JSON mode, streaming, and MCP bridging.

### Model Routing

Tasks are automatically routed to the best provider based on:
- Task type (vision, coding, review, chat)
- Privacy requirements (local-first default)
- User-pinned preferences
- Latency/quality tradeoff mode

### Permission System

| Permission | Default Policy |
|-----------|---------------|
| Read workspace files | Allowed |
| Write workspace files | Requires approval |
| Run shell commands | Requires review |
| Browser MCP tools | Requires approval |
| Send to cloud provider | Requires approval |
| Upload raw screenshots | Denied |
| Commit changes | Requires approval |
| Git push | Requires explicit action |

Risk tiers: **Low** (single-file, local) → **Medium** (multi-file, shell) → **High** (deletions, deps, git, external)

---

## Design System

The UI uses a custom **Frutiger Aero** design system:

- **Colors**: Sky blue backgrounds (#dff4ff, #f6fdff), navy text (#11324a), blue/cyan/green accents
- **Radii**: 6/10/14/18/24/999px for xs through pill
- **Shadows**: Tinted shadows (rgba(44,112,150,…)) with inset gloss highlights
- **Materials**: Glass panels with blur (8/14/24px) and translucent borders
- **Motion**: 120/180/260ms transitions with spring constants
- Light and dark themes with CSS custom properties
- Based on XP/Vista era optimism, not modern minimalism

---

## CI/CD

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `ci.yml` | PR / push to main | Lint, clippy, test, typecheck |
| `release.yml` | Version tags | Multi-platform build + publish |
| `nightly.yml` | Schedule / manual | Pre-release preview builds |
| `security.yml` | Schedule / PR | Cargo audit, npm audit, CodeQL |

Artifact naming: `feverthoth-{version}-{platform}-{arch}.{ext}`

---

## License

MIT © FeverDream
