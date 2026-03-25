# AGENTS.md

## 1. Project Overview

This repository contains a full-stack web application for displaying photos of other people.
- **Frontend:** Svelte 5, TypeScript, TailwindCSS, ESLint, Prettier, Vite
- **Backend:** Rust (edition 2024), Rocket web framework
- **Database:** MongoDB (migrated from PostgreSQL)
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

| Action | Command |
| ------ | ------- |
| Install dependencies (pnpm preferred) | `pnpm install` or `npm install` |
| Development server | `npm run dev` |
| Production build | `npm run build` |
| Preview build | `npm run preview` |
| Lint | `npm run lint` (prettier & eslint) |
| Format | `npm run format` |
| Type-check | `npm run check` |
| Type-check (watch) | `npm run check:watch` |
| Single test | _No frontend test script currently – if added, document usage here_ |

### Backend (Rust + Rocket + MongoDB)

| Action | Command |
| ------ | ------- |
| Install dependencies | `cargo fetch` |
| Build | `cargo build` |
| Run server | `cargo run` |
| Run all tests | `cargo test` |
| Run a single test | `cargo test <test_name>` |

#### MongoDB local test instance
- If integration tests require a running MongoDB, ensure your test runner spins up a local or in-memory instance, or set the `MONGODB_URI` appropriately.
- Document any test fixtures/utilities required for MongoDB. Update this file to reflect new commands/utilities as they are added.

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

### Backend (Rust, Rocket, MongoDB)

#### Imports & Module Org
- Use idiomatic modules (`use ...;`).
- Group: std/core/alloc first, external (including `mongodb`/BSON types), then crate imports. Separate groups with blank lines.
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
- MongoDB collections: use clear, consistent names (plural, snake_case preferred).
- Data models: ensure BSON (`bson::Document`) usage is type-checked and schemas are documented.

#### Documentation & Comments
- All public APIs must have rustdoc comments (`/// ...`) + example.
- Complex/important internal logic should be commented.
- Document MongoDB connection pool usage and configuration if customized.

#### Error Handling
- Use `Result<T, E>` everywhere possible.
- Use the `?` operator for error propagation; document contexts for MongoDB driver errors.
- Always match and handle connection, query, and conversion errors from MongoDB operations.
- Custom error types should use enums, with variants for database/connection errors.
- Always handle/document panics.

#### Testing
- Write failing tests before fixing/adding features.
- Add doctests/unit/integration test for each new feature/bugfix.
- When testing MongoDB code, use test isolation (unique db/collections, or mock/integration with teardown).

#### Recommended references
- [Rocket CONTRIBUTING.md](https://github.com/SergioBenitez/Rocket/blob/master/CONTRIBUTING.md)
- [Rust Style Guide](https://doc.rust-lang.org/1.70.0/style-guide/index.html)
- [mongodb Rust driver](https://www.mongodb.com/docs/drivers/rust/)
- [BSON crate docs](https://docs.rs/bson/latest/bson/)

---

## 5. Linting, Formatting, and Automation

- **Prettier:** Run before each commit/PR.
- **ESLint:** Lints JS/TS/Svelte. Type-aware, disables redundant TS rules.
- **Svelte Linting:** Enabled via `eslint-plugin-svelte`.
- **Type checking:** Always run `npm run check`.
- **Rust:** Format (manual review) and test (`cargo test`) before changes.
- **Engine strict:** Enforced via `.npmrc`.
- **MongoDB:** Prefer documenting local dev/test db setup & test utilities with any config changes here.

---

## 6. Cursor, Copilot, Agentic Rules

- **No Cursor or Copilot rule files detected.** If `.cursor/rules/`, `.cursorrules`, or `.github/copilot-instructions.md` are added, incorporate their rules here.
- In absence, **err on the side of explicitness, citation, and verbosity.**

---

## 7. Notes for Agentic Contributors

- Always check MCP/Context7 before performing web searches or manual troubleshooting.
- Always lint, type-check, and format before sending code or making a commit/PR.
- Update this file if you add new tools, migrate libraries (including database), or add test helpers/configs (e.g., new DB URI, CI secrets, etc.).
- Reference this file in PRs/code reviews—this is canonical.
- If unsure, consult upstream (SvelteKit, Prettier, MongoDB, Rocket) docs/best practices.

---

_End of AGENTS.md_

---
