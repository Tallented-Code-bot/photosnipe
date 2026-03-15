# AGENTS.md

## 1. Project Overview

This repository contains a web application to display photos of other people.  
It is a full-stack project with a Svelte (TypeScript-based) frontend and a Rust (Rocket framework) backend.  
- **Frontend:** Svelte 5, TypeScript, TailwindCSS, ESLint, Prettier, Vite
- **Backend:** Rust (edition 2024), Rocket web framework
- **Database:** PostgreSQL
- **Authentication:** Discord via Auth0

---

## 2. Build, Lint, and Test Commands

### Frontend (Svelte & TypeScript)

**Install dependencies:**
- With npm:
  ```
  npm install
  ```
- With pnpm (preferred):
  ```
  pnpm install
  ```

**Start dev server:**
```
npm run dev
# Or (open in browser)
npm run dev -- --open
```

**Build:**
```
npm run build
```

**Preview (production build):**
```
npm run preview
```

**Lint (Prettier & ESLint):**
```
npm run lint
# (runs: prettier --check . && eslint .)
```

**Format:**
```
npm run format
# (runs: prettier --write .)
```

**Type-check (strict):**
```
npm run check
# (runs: svelte-kit sync && svelte-check --tsconfig ./tsconfig.json)
```

**Watch mode type-check:**
```
npm run check:watch
```

**Running a Single Test:**
- _No test script is defined for the frontend. If tests are later defined (e.g., Vitest or Playwright), add instructions here._

---

### Backend (Rust + Rocket)

**Install dependencies:**
```
cargo fetch
```
**Build:**
```
cargo build
```
**Run server:**
```
cargo run
```
**Run all tests:**
```
cargo test
```
**Run a single test:**
```
cargo test <test_name>
# Replace <test_name> with the actual test function name.
```
---

## 3. Code Style Guidelines

### General
- Write clear, maintainable, well-documented code.
- Use meaningful names and provide comments, especially for complex logic.
- Prefer pure functions and minimal side effects.
- Prioritize type safety throughout (TypeScript strict for frontend, idiomatic Rust for backend).

---

### Frontend (Svelte & TypeScript)

#### Imports
- Use ES module imports everywhere.
- Group imports: external first, then internal, then relative.
- Omit file extensions for TS/JS/Svelte unless necessary.
- Use path aliases via SvelteKit config where possible (e.g. `$lib`).

#### Formatting (Enforced by Prettier)
- Use **tabs** for indentation.
- **Single quotes** for strings.
- **Trailing commas:** none.
- **Line width:** 100 characters.
- **Prettier plugins:** includes plugin-svelte and TailwindCSS (sorts classes).
- Respect `.prettierignore` (see file).

#### TypeScript types
- All code must be strongly typed (`"strict": true` in tsconfig).
- Prefer interfaces/types for all complex objects.
- Avoid `any`, prefer explicit union types.
- Functions and variables must have explicit types where not inferred.
- Use nullish/undefined coalescing thoughtfully, avoid “!” assertions.

#### Naming Conventions
- **Variables/functions:** `camelCase`
- **Types/interfaces:** `PascalCase`
- **Files:** `kebab-case` or `camelCase`. Svelte components: `PascalCase.svelte`.
- Exported Svelte components: default exports, capitalized.

#### Svelte Specifics
- Use `<script lang="ts">` in all Svelte files.
- Keep script and markup organized: logic at the top, template below.
- For reactivity, use Svelte 5 runes appropriately (`$state`).
- CSS: co-locate critical styles in components, global with Tailwind.

#### Error Handling
- Propagate errors up to a boundary (SvelteKit error pages or try/catch).
- Use result/unwrapped patterns in TS where available.
- Log errors with descriptive context.

---

### Backend (Rust: Rocket)

#### Imports & Organization
- Use idiomatic module imports (use …;).
- Group std, 3rd-party, then local modules.
- Organize files with `mod.rs` if project grows.

#### Naming Conventions
- **Variables/functions:** `snake_case`
- **Types/structs/enums:** `CamelCase`
- **Constants/statics:** `SCREAMING_SNAKE_CASE`

#### Error Handling
- Use `Result<T, E>` pattern throughout for error propagation.
- Use the `?` operator for propagating errors upward.
- Implement custom error types using enums when appropriate.
- Always handle or document panics.

#### Commenting/Docs
- Document all public APIs with triple-slash comments.
- Write concise inline comments for complex logic.

#### Formatting
- Enforce standard Rust style via `cargo fmt`.
- Use `cargo clippy` for linting (if/when set up; add if not present).

---

## 4. Linting, Formatting, and Automation

- **Prettier**: Canonical code formatter. Run before each commit/PR.
- **ESLint**: Lints JS/TS/Svelte code, disables certain rules for TypeScript as needed (e.g., `no-undef: off`).
- **Svelte Linting**: Enabled via `eslint-plugin-svelte`.
- **Type checking**: Always run `npm run check`.
- **Rust**: Always run `cargo fmt` and `cargo test` before submitting changes.
- **Engine Strict**: package manager must enforce Node engine compatibility via `.npmrc`.

---

## 5. Cursor / Copilot / Agent-Specific Rules

- **No Cursor or Copilot rule files found** in this repo at this time.
- If `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` are added, incorporate their rules into this guide.
- Agentic contributors **should err on the side of verbosity and citation** when these files are missing.

---

## 6. Additional Notes for Agentic Developers

- Always lint, type-check, and format before sending code or making a commit/PR.
- If you add new tools (e.g., tests, clippy, additional formatters), update this file.
- Cross-reference this file in PRs and code review—this is the canonical agentic ruleset.
- When in doubt, follow SvelteKit, Prettier, TypeScript, and Rocket official best practices by default.

---

_End of AGENTS.md_

---
