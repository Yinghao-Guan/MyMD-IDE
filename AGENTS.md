# Repository Guidelines

## Project Structure & Module Organization
- `src/` contains the React frontend (entry `main.jsx`, UI in `App.jsx`, styles in `App.css`).
- `src/assets/` and `public/` hold static assets used by the UI.
- `src-tauri/` contains the Rust backend, Tauri configuration (`tauri.conf.json`), and build scripts (`build.rs`).
- Generated artifacts live in `node_modules/` (frontend) and `src-tauri/target/` (Rust); avoid editing these.

## Build, Test, and Development Commands
- `npm run dev`: start the Vite dev server for the frontend.
- `npm run build`: create a production build of the frontend.
- `npm run preview`: serve the production build locally.
- `npm run tauri dev`: run the full Tauri app in dev mode (frontend + Rust backend).
- `npm run tauri build`: build a Tauri production bundle.

## Coding Style & Naming Conventions
- Use the existing 4-space indentation style in `src/` and keep double quotes for strings.
- Prefer small, single-purpose React components; keep UI logic in `src/` and backend logic in `src-tauri/src/`.
- Match current naming: `PascalCase` for components (e.g., `App`), `camelCase` for functions and variables.
- No formatter or linter is configured; run changes through manual review for consistency.

## Testing Guidelines
- No automated test framework is currently configured.
- If you add tests, document the framework and commands here, and keep naming consistent (e.g., `App.test.jsx`).

## Commit & Pull Request Guidelines
- Git history is not available in this repo, so no commit convention is defined.
- Use concise, imperative commit messages (e.g., "Add PDF preview loading state").
- PRs should include: a clear description, manual test steps, and screenshots or recordings for UI changes.

## Configuration & Security Notes
- Tauri permissions and app metadata live in `src-tauri/tauri.conf.json` and `src-tauri/capabilities/`.
- When adding external dependencies, note why they are required and keep the bundle size minimal.
