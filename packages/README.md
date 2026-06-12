# Vyzorix Development library

Welcome to the development library for the Vyzorix ecosystems. This suite contains modular libraries and CLI commands, infrastructure operations, configuring runtimes, and deploying standardized interfaces.

---

## 📂 Monorepo Structure

```
vyzorix-workspace/
├── packages/
│   ├── vyzorix/                   # 🎨 @vyzorix/ui (In development)
│   │   ├── package.json
│   │   ├── src/
│   │   │   ├── index.ts           # Unified exports
│   │   │   ├── types/             # Shared Types
│   │   │   ├── themes/            # 🎨 Theme Configuration system
│   │   │   │   ├── index.ts
│   │   │   │   ├── colors.ts      # Vyzorix standard color map
│   │   │   │   ├── typography.ts  # Standard heading/label utility maps
│   │   │   │   └── layers.ts      # Structural utility groups (cards, inputs)
│   │   │   └── components/
│   │   │       ├── LoginForm.tsx, SignUpForm.tsx     # Forms
│   │   │       ├── SuccessView.tsx, WaitingVerification.tsx # Views
│   │   │       └── DataCard.tsx, StatusIndicator.tsx # Core Atoms
│   │
│   ├── vyzorix-config/            # ⚙️ @vyzorix/config (In development)
│   │   ├── src/
│   │   │   ├── validator.ts        
│   │   │   └── session.ts         
│   │
│   └── vyzorix-cli/               # ⚡ vyzorix-cli (in development)
│       ├── bin/vyzorix.js         # Executable entry
│       ├── package.json
│       └── src/
│           ├── index.ts
│           ├── commands/
│           │   ├── init.ts        # Scaffolds directory structure and injects auth code
│           │   ├── theme.ts       # Exports @vyzorix/ui theme configurations securely
│           │   └── publish.ts     # Package bundle compiler tool
```

---

## 🎨 Theme Architecture (`@vyzorix/ui/themes`)

To avoid scattered classes and ensure reuse, Vyzorix exposes a structured `themes` module.

```tsx
import { themes } from '@vyzorix/ui';

// Usage in custom components:
export function Panel() {
  return (
    <div className={`${themes.layers.card.base} ${themes.colors.background.surface}`}>
       <h1 className={themes.typography.heading.h2}>Vyzorix Core</h1>
    </div>
  )
}
```

This enforces consistency and guarantees spacing without writing custom PostCSS plugins.

---
 
## ⚡ CLI Design (`vyzorix-cli`) ----(needs rewriting , it's currently rRust)

Vyzorix CLI uses **Commander.js** combined with **Inquirer.js** to bootstrap environments rapidly.

### Command Catalog

1. `vyzorix init`
   * Scaffolds the Next.js/Vite environment.
   * Auto-injects `@vyzorix/ui` dependencies and sets up the Express hydration server structure `server.ts`.
   * Provisions local `vyzorix.db` via CLI.

2. `vyzorix doctor`
   * Asserts the integrity of `vyzorix_session` handling mechanisms.

3. `vyzorix generate:theme`
   * Bootstraps local instances of `colors.ts`, `typography.ts` in the target project, allowing to locally safely override background colors and padding without breaking structural boundaries.

---

## 🛠️ Workspaces Config (`package.json`)

To enable seamless local resolution of packages without complex symlinking, add the following to the root manifest:

```json
{
  "name": "vyzorix-workspace",
  "private": true,
  "workspaces": [
    "packages/*"
  ]
}
```

---

## 📈 Roadmap & Package Sync Mappings

### 1. `@vyzorix/ui'
* **Goal**: Modularize standard operator interface systems.
* **Status**: In development but still ready to publish to public/private registries. 

### 2. `@vyzorix/config`
* **Goal**: Holds core cookie setups, cryptographic signature secrets, and database endpoints mapping profiles.


### 3. `vyzorix-cli` 
* **Goal**: No goal yet-------
