# .

This template should help get you started developing with Vue 3 in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Vue (Official)](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur).

# Tambahkan Ini Ke Database (Frontend)

A modern, high-performance web application built with **Vue 3** and **Tailwind CSS 4**, featuring a brutalist-inspired design.

## 🚀 Tech Stack

- **Framework:** [Vue 3](https://vuejs.org/) (Composition API with `<script setup>`)
- **Build Tool:** [Vite](https://vitejs.dev/)
- **Styling:** [Tailwind CSS 4](https://tailwindcss.com/)
- **State Management:** [Pinia](https://pinia.vuejs.org/)
- **Routing:** [Vue Router](https://router.vuejs.org/)
- **Package Manager:** [Bun](https://bun.sh/)
- **HTTP Client:** [Axios](https://axios-http.com/) / Fetch API
- **Testing:** [Vitest](https://vitest.dev/) (Unit) & [Playwright](https://playwright.dev/) (E2E)

## 🛠️ Getting Started

### Prerequisites

Ensure you have [Bun](https://bun.sh/) installed on your system.

### Installation

```bash
bun install
```

### Development

Start the development server:

```bash
bun dev
```

### Build

Compile and minify for production:

```bash
bun build
```

### Testing

Run unit tests:

```bash
bun test:unit
```

Run end-to-end tests:

```bash
bun test:e2e
```

## 📂 Project Structure

- `src/components/`: Reusable UI components.
- `src/views/`: Page-level components associated with routes.
- `src/services/`: API integration and business logic services.
- `src/stores/`: Pinia state management modules.
- `src/router/`: Navigation and route definitions.
- `src/assets/`: Global styles and static assets.
- `src/config/`: Application configuration (e.g., API base URL).

## 🎨 Design Guidelines

The application follows a **Brutalist Design System**:

- High-contrast color palette (Hyper Lime, Radical Pink, Digital Lavender).
- Heavy borders and bold typography.
- Interactive micro-animations and glassmorphism effects.
- Responsive layout using Tailwind's container-first approach.

## 🔑 Authentication

Authentication is handled via JWT. The token is stored in `localStorage` and managed through the `AuthStore`. All protected API calls include the `Authorization: Bearer <token>` header.
npx playwright install

# When testing on CI, must build the project first

bun run build

# Runs the end-to-end tests

bun test:e2e

# Runs the tests only on Chromium

bun test:e2e --project=chromium

# Runs the tests of a specific file

bun test:e2e tests/example.spec.ts

# Runs the tests in debug mode

bun test:e2e --debug

```

```
