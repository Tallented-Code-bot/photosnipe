# AGENTS.md

## 1. Project Overview

This repository contains a full-stack web application for displaying photos of other people.
- **Frontend:** Svelte 5, TypeScript, TailwindCSS, ESLint, Prettier, Vite
- **Backend:** Rust (edition 2024), Rocket web framework
- **Database:** PostgreSQL
- **Authentication:** Discord via Auth0

---

## 2. MCP/Context7 Tooling Usage

### Svelte MCP Server (Mandatory)
- For all Svelte or SvelteKit documentation lookup, code analysis, autofixing, and code review tasks, use the Svelte MCP server tools (list-sections, get-documentation, svelte-autofixer, etc).
- Always use the MCP server for any Svelte, SvelteKit, or Svelte-related questions before searching the web. Proactively use its autofix/code validation tools when writing or editing `.svelte` files.
- Leverage MCP for best practices, up-to-date Svelte features (e.g. Runes, reactivity), and error troubleshooting.
- Only use web search as a fallback if the MCP server yields no relevant or sufficient answer.

### Context7 API (Mandatory)
- For any question about modern JavaScript, TypeScript, or Rust third-party libraries or frameworks, use the Context7 API (libraryId resolution and docs query tools) for code examples, usage, and documentation.
- Prioritize using Context7 for package usage, install, troubleshooting, and configuration over web searches.
- If both MCP and Context7 are relevant for a Svelte-related library task, consult each as appropriate.
- Use web search only if Context7 does not return sufficient/authoritative information.

---

## 3. Build, Lint, and Test Commands

### Frontend (SvelteKit, TypeScript, Vite)

**Install dependencies:**
- With npm:
  ```
  npm install
  ```
- Or with pnpm (preferred):
  ```
  pnpm install
  ```

**Development server:**
```
npm run dev
# Or: npm run dev -- --open
```

**Production build:**
```
npm run build
```

**Preview (serve prod build):**
```
npm run preview
```

**Lint (Prettier & ESLint):**
```
npm run lint
# (prettier --check . && eslint .)
```

**Format:**
```
npm run format
# (prettier --write .)
```

**Type-check (strict):**
```
npm run check
# (svelte-kit sync && svelte-check --tsconfig ./tsconfig.json)
```

**Type-check in watch mode:**
```
npm run check:watch
```

**Running a Single Test:**
_There is currently no test script for the frontend. If you add Vitest or Playwright, document single-test usage here._

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
# Replace <test_name> with the test fn name.
```

---

## 4. Code Style Guidelines

### General
- Write clear, maintainable, and well-documented code.
- Use meaningful names, especially for complex logic.
- Prefer pure functions, minimal side effects.
- Maximal type safety (TS strict, idiomatic Rust).
- Always prefer MCP/Context7 tools for Svelte/libraries; update code per their guidance before using web search.

---

### Frontend (Svelte 5, TypeScript)

#### Imports
- Use ES modules everywhere.
- Group imports: external, then internal/library, then relative.
- Omit file extensions unless required.
- Use SvelteKit path aliases where possible (`$lib`).

#### Formatting (enforced by Prettier)
- **Tabs** for indentation.
- **Single quotes** for strings.
- **No trailing commas**.
- **Line width:** 100 chars.
- **Prettier plugins:** Use `prettier-plugin-svelte`, `prettier-plugin-tailwindcss`.
- Respect `.prettierignore`.

#### TypeScript
- All code must be strongly typed (`strict: true` in tsconfig).
- Prefer interfaces/types for all objects.
- Avoid `any`; prefer union, `unknown`, or narrowing.
- Explicit types unless trivial or contextually inferred.
- Use nullish/undefined coalescing thoughtfully.

#### Naming
- **Variables, functions:** `camelCase`
- **Types, interfaces:** `PascalCase`
- **Files:** `kebab-case` or `camelCase`; Svelte: `PascalCase.svelte`
- Exported Svelte components must be default capitalized.

#### Svelte Specific
- Always use `<script lang="ts">` in Svelte files.
- Organize: logic first, then markup.
- Use Runes/reactivity idioms as per Svelte 5 (`$state`/`$derived`).
- Co-locate component styles; use global Tailwind for layout/theme.
- For any Svelte file creation, analysis, or fixing, use Svelte MCP server tooling first.

#### Error Handling
- Propagate errors to error boundaries (SvelteKit error pages, or via `try/catch`).
- Use `Result`/unwrapped patterns in TS. Log errors with context.
- Use Svelte MCP/autofixer for error fixes before resorting to alternative sources.

#### Recommended references
- [Svelte Styleguide](https://github.com/sveltejs/style-guide)
- [SvelteKit docs](https://kit.svelte.dev/docs/project-structure)
- [Svelte MCP Docs](https://kit.svelte.dev/docs)

---

### Backend (Rust, Rocket)

#### Imports & Module Org
- Use idiomatic modules (`use ...;`).
- Group: std/core/alloc first, external, then crate imports. Separate groups with blank lines.
- Do **not** use multi-line imports. One per line.

#### Formatting
- One blank line between all items.
- Follow [Rust Style Guide](https://doc.rust-lang.org/1.70.0/style-guide/index.html) _except_:
  - Prefer where-clauses over block-indented generics.
  - Block-align long `where` clauses as per Rocket project `CONTRIBUTING.md`.
  - Do not use automated `cargo fmt`; format manually to match project idioms.

#### Naming
- **Variables/functions:** `snake_case`
- **Types/structs/enums:** `CamelCase`
- **Constants/statics:** `SCREAMING_SNAKE_CASE`

#### Documentation & Comments
- All public APIs must have rustdoc comments (`/// ...`) + example.
- Complex/important internal logic should be commented.

#### Error Handling
- Use `Result<T, E>` everywhere possible.
- Use the `?` operator for error propagation.
- Custom error types should use enums.
- Always handle/document panics.

#### Testing
- Write failing tests before fixing/adding features.
- Add doctests/unit/integration test for each new feature/bugfix.

#### Recommended references
- [Rocket CONTRIBUTING.md](https://github.com/SergioBenitez/Rocket/blob/master/CONTRIBUTING.md)
- [Rust Style Guide](https://doc.rust-lang.org/1.70.0/style-guide/index.html)

---

## 5. Linting, Formatting, and Automation

- **Prettier:** Run before each commit/PR.
- **ESLint:** Lints JS/TS/Svelte. Type-aware, disables redundant TS rules.
- **Svelte Linting:** Enabled via `eslint-plugin-svelte`.
- **Type checking:** Always run `npm run check`.
- **Rust:** Format (manual review) and test (`cargo test`) before changes.
- **Engine strict:** Enforced via `.npmrc`.

---

## 6. Cursor, Copilot, Agentic Rules

- **No Cursor or Copilot rule files detected.** If `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` are added, incorporate their rules here.
- In absence, **err on the side of explicitness, citation, and verbosity.**

---

## 7. Notes for Agentic Contributors

- Always check MCP/Context7 before performing web searches or manual troubleshooting.
- Always lint, type-check, and format before sending code or making a commit/PR.
- Update this file if you add new tools (test, clippy, formatters, etc).
- Reference this file in PRs/code reviews—this is canonical.
- If unsure, consult upstream (SvelteKit, Prettier, Rocket) docs/best practices.

---

_End of AGENTS.md_

---
