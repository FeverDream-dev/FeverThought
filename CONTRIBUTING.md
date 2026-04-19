# Contributing to FeverThoth IDE

## Setup

1. Install prerequisites: Node.js 20+, pnpm 9+, Rust stable, Tauri v2 system deps
2. Clone and install: `git clone ... && cd FeverThoth && pnpm install`
3. Start dev: `pnpm dev`

## Code Style

- **Rust**: `cargo fmt` is law. `cargo clippy` must pass with no warnings.
- **TypeScript**: Prettier + ESLint configured in the repo. Run `pnpm lint` before pushing.
- **CSS**: BEM-like naming with `ft-` prefix for design system tokens.

## Pull Requests

- Target `main` branch
- Keep PRs focused — one concern per PR
- Include a test plan in the PR description
- CI must pass (Rust fmt/clippy/test + frontend lint/typecheck)
- At least one review required for merge

## Commit Messages

Use conventional commits:
- `feat:` new features
- `fix:` bug fixes
- `refactor:` code reorganization
- `docs:` documentation changes
- `chore:` build/CI/tooling

## Architecture

- `crates/` — Rust workspace crates (core, providers, lsp, agents, workspace, settings, security, mcp, git_tools, terminal)
- `apps/desktop/` — Tauri v2 desktop app (Rust backend + React frontend)
- `packages/` — Shared packages
- See `docs/architecture.md` for the full breakdown

## Reporting Issues

Use the GitHub issue templates. Include:
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, version)
