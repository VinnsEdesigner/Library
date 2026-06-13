# Vyzorix React SSR Hydration, Vite, & Nitro server Integration Guide

When executing **Server-Side Rendering (SSR)** paired with **Client-Side Hydration**, React compiles components into a static HTML string on the server. The client browser renders this markup immediately. It then downloads the bundled JavaScript payload to "hydrate" the DOM Nodes—attaching event listeners, loading initial states, and activating runtime logic.

If there is even a subtle difference between the HTML delivered by the server and what the React client expects during initial rendering, React will trigger a **Hydration Mismatch Warning** (or throw an error in strict modes), destroying and rebuilding the DOM tree, causing painful flashes of content.

This comprehensive guide details the exact HTML structure, React mount syntax, and scripts necessary to run a flawless SSR setup using both **Vite Dev Mode** and a production-grade **Nitro Node.js server**.

---

## 1. End-to-End SSR Pipeline Lifecycle

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Phase 1: Server Request                         │
│ Client requests "/" -> Nitro / Go server intercepts request.            │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
                                   ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        Phase 2: State Irrigation                       │
│ Server queries DB (SQLite) -> Extracts authenticated operator payload.  │
│ Prefills a plain JSON string: window.__VYZORIX_PREFETCHED_STATE__      │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
                                   ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     Phase 3: React Static Render                       │
│ React's renderToString() compiles <App /> to HTML using state.         │
│ Inserts compiled HTML inside the <div id="root"> template shell.       │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
                                   ▼
┌────────────────────────────────────────────────────────────────────────┐
│                    Phase 4: Client First Paint                         │
│ Browser receives HTML. Immediate visual paint (very high performance). │
│ At this point, buttons are completely unclickable, forms are inert.    │
└──────────────────────────────────┬─────────────────────────────────────┘
                                   │
                                   ▼
┌────────────────────────────────────────────────────────────────────────┐
│                        Phase 5: Hydration & Mount                      │
│ Browser downloads Vite main script -> executes hydrateRoot().          │
│ Match-merges state data -> Portal becomes fully interactive!           │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Core HTML Structure Template (`index.html`)

Instead of standard lazy SPA templates, your SSR template utilizes **two clear placeholders**:
1. `<!--app-html-->` : Where Nitro or your SSR compiler injects React's server output.
2. `<!--app-state-->`: Where JSON state parameters are irrigated into the window container safely before mounting scripts execute.

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Vyzorix Authorization Workspace</title>
    <!-- Stylesheets / Fonts -->
    <link rel="stylesheet" href="/src/index.css" />
  </head>
  <body class="bg-neutral-950 text-white selection:bg-rose-600 selection:text-white antialiased">
    
    <!-- 1. The Mounting Container -->
    <!-- CRITICAL: Do NOT leave whitespace or linebreaks between these tags. -->
    <!-- Any space inside the div will cause React 18+ to warn about mismatched text nodes on load! -->
    <div id="root"><!--app-html--></div>

    <!-- 2. The Hydrated State Safe Box -->
    <!-- Injected safely before scripts run so they are instantly accessible -->
    <!--app-state-->

    <!-- 3. Client Bundles entry point -->
    <script type="module" src="/src/main.tsx"></script>
    
  </body>
</html>
```

---

## 3. Designing Client/Server React Entry Points

To enable SSR, you must decouple your React compilation code into two entry files:

### A. Client Mounting / Hydration Layer (`src/main.tsx` / `entry-client.tsx`)
In SPA apps, you use `createRoot`. In SSR apps, you **MUST** swap to `hydrateRoot` so React reuses the server-delivered HTML elements.

```typescript
// src/main.tsx (Client Hydration)
import { hydrateRoot } from 'react-dom/client';
import React from 'react';
import App from './App';
import './index.css';

const container = document.getElementById('root');

if (!container) {
  throw new Error('Critical Error: Root mounting container not found inside index.html');
}

// hydrateRoot matches the initial server output node-by-node
hydrateRoot(
  container,
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

### B. Server-Side Builder (`src/entry-server.tsx`)
This script executes solely on the server (running node/Vite or Nitro engines), compiling React code into standard text strings.

```typescript
// src/entry-server.tsx
import ReactDOMServer from 'react-dom/server';
import React from 'react';
import App from './App';

export function render(url: string, prefetchedState: any) {
  // Irrigates state into global variables inside Node/Nitro context
  if (typeof global !== 'undefined') {
    (global as any).__VYZORIX_PREFETCHED_STATE__ = prefetchedState;
  }

  // Generate plain static markup
  const html = ReactDOMServer.renderToString(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );

  return { html };
}
```

---

## 4. Vite Development Server Setup (Middleware Mode)

During local development, Vite compiles TypeScript, processes CSS, and resolves module trees on the fly. To achieve perfect Dev SSR, construct your development server utilizing Vite's SSR compilation middleware:

```typescript
// dev-server.ts
import express from 'express';
import fs from 'fs';
import path from 'path';
import { createServer as createViteServer } from 'vite';

async function startDevServer() {
  const app = express();
  const PORT = 3000;

  // Integrate Vite dev middleware in SSR mode
  const vite = await createViteServer({
    server: { middlewareMode: true },
    appType: 'custom' // Custom indicates we are handling the routing of HTML ourselves
  });

  app.use(vite.middlewares);

  app.get('*', async (req, res) => {
    try {
      const url = req.originalUrl;

      // 1. Read index.html template file
      let template = fs.readFileSync(path.resolve('./index.html'), 'utf-8');

      // 2. Apply Vite HTML transformations (e.g. injects HMR client scripts)
      template = await vite.transformIndexHtml(url, template);

      // 3. Load Server module dynamically through Vite
      const { render } = await vite.ssrLoadModule('/src/entry-server.tsx');

      // 4. Mimic/Mock database prefetching
      const state = {
        view: 'signup',
        profileData: { fullName: '', email: '', username: '' },
        successReport: null
      };

      // 5. Render App to HTML string
      const { html } = render(url, state);

      // 6. Assemble output with state variables
      const stateObject = `<script>window.__VYZORIX_PREFETCHED_STATE__ = ${JSON.stringify(state)};</script>`;
      
      const responseHtml = template
        .replace('<!--app-html-->', html)
        .replace('<!--app-state-->', stateObject);

      // 7. Dispatch payload
      res.status(200).set({ 'Content-Type': 'text/html' }).end(responseHtml);

    } catch (e: any) {
      vite.ssrFixStacktrace(e);
      res.status(500).end(e.stack);
    }
  });

  app.listen(PORT, () => {
    console.log(`Vite SSR Dev Server running at http://localhost:${PORT}`);
  });
}

startDevServer();
```

---

## 5. Nitro Node.js Production Engine Setup

In production mode, Vite does not run. Instead, **Nitro** serves pre-compiled bundles. Code inside Nitro intercepts the static index.html skeleton and hydrates it dynamically targeting your backend JSON contracts.

### Complete Server Entry-point (`server/index.ts` / Nitro Node Route)

```typescript
// production-server.ts (Nitro/Node.js SSR integration)
import express from 'express';
import fs from 'fs';
import path from 'path';

const app = express();
const PORT = process.env.PORT || 3000;

const distPath = path.resolve('./dist');

// Serve compiled static assets before routing logic!
app.use('/assets', express.static(path.join(distPath, 'client/assets')));

app.get('*', async (req, res) => {
  try {
    const url = req.originalUrl;

    // 1. Read our pre-compiled production template
    let template = fs.readFileSync(path.join(distPath, 'client/index.html'), 'utf-8');

    // 2. Fetch data from sqlite/Go server to construct prefetched state
    // (If user has valid Session Cookie, pre-build success screen dashboard state)
    const mockDbReport = {
      fullName: 'Alexis Thorne',
      email: 'alexis@vyzorix.com',
      username: 'alexis_vyzorix',
      memberId: 'VXZ-64981',
      operatorRole: 'Operator',
      region: 'Paris, France',
      createdAt: '2026-06-12 12:29:27 UTC',
      method: 'Standard Email'
    };

    const state = {
      view: 'success',
      profileData: { fullName: 'Alexis Thorne', email: 'alexis@vyzorix.com', username: 'alexis_vyzorix' },
      successReport: mockDbReport
    };

    // 3. Load production-built Server Bundle
    // We import the compiled server.js which outputs render()
    const { render } = require('./dist/server/entry-server.js');

    // 4. Render output
    const { html } = render(url, state);

    // 5. Build dynamic script string
    const stateScript = `<script>
      window.__VYZORIX_PREFETCHED_STATE__ = ${JSON.stringify(state)};
    </script>`;

    // 6. Hydrate and replace templates
    const finalHtml = template
      .replace('<!--app-html-->', html)
      .replace('<!--app-state-->', stateScript);

    res.status(200).set({ 'Content-Type': 'text/html' }).end(finalHtml);

  } catch (error: any) {
    res.status(500).end(`Server Hydration Handshake Failed: ${error.message}`);
  }
});

app.listen(PORT, '0.0.0.0', () => {
  console.log(`Nitro/Go SSR System listening on Port ${PORT}`);
});
```

---

## 6. Complete Server-Served HTML Output (Over-The-Wire Example)

This is the exact raw HTML code that the Go Backend or the Nitro Node.js server transmits over the network to the browser. It features the pre-rendered static components, loaded global dehydrated states, and script assets that hydrate the layout:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Vyzorix Authorization Workspace</title>
    <!-- Server-served optimized CSS link -->
    <link rel="stylesheet" href="/assets/index-38c7f91a.css" />
  </head>
  <body class="bg-neutral-950 text-white selection:bg-rose-600 selection:text-white antialiased">
    
    <!-- 1. The Pre-compiled DOM Tree sent from the server (Immediate First Paint) -->
    <!-- Note: There are absolutely no whitespaces inside the #root wrapper to avoid hydration warnings -->
    <div id="root"><div class="min-h-screen bg-neutral-950 flex flex-col justify-between"><header class="p-6 border-b border-rose-900/20"><span class="text-rose-500 font-mono tracking-widest uppercase">Vyzorix Workspace Portal</span></header><main class="p-8 max-w-4xl mx-auto"><h1 class="text-4xl font-sans tracking-tight">Active Connection Approved</h1><p class="mt-4 font-mono text-neutral-400">Operator token authenticated via Google. Session is fully active.</p></main></div></div>

    <!-- 2. The Dehydrated Initial State safe box -->
    <!-- This initializes state variables inside client's RAM memory immediately before React's hydrateRoot runs -->
    <script id="vyzorix-dehydrated-state">
      window.__VYZORIX_PREFETCHED_STATE__ = {
        "view": "success",
        "profileData": {
          "fullName": "Alexis Thorne",
          "email": "alexis@vyzorix.com",
          "username": "alexis_vyzorix"
        },
        "successReport": {
          "fullName": "Alexis Thorne",
          "email": "alexis@vyzorix.com",
          "username": "alexis_vyzorix",
          "memberId": "VXZ-64981",
          "operatorRole": "Operator",
          "region": "Paris, France",
          "createdAt": "2026-06-12 12:29:27 UTC",
          "method": "Standard Email"
        }
      };
    </script>

    <!-- 3. Client Module Entry Bundle -->
    <!-- Triggers client-side React code to attach event-listeners directly to pre-rendered nodes above -->
    <script type="module" src="/assets/main-7b490f2b.js"></script>
    
  </body>
</html>
```

---

## 7. Resolving Script Mounting Issues & Common Pitfalls

If scripts are failing to bind or your mounting throws errors, verify these three critical conditions:

1. **Incorrect Asset Root Paths**:
   Inside your compiled `dist/client/index.html`, ensure the source injection tag is `<script type="module" src="/assets/main-hash.js"></script>`. Double check that your Go or Nitro route exposes access to `/assets` correctly with absolute paths.
2. **Whitespace matching inside `<div id="root">`**:
   The placeholder inside `index.html` must be exactly `<div id="root"><!--app-html--></div>` on a single line. If you leave spaces (e.g., `<div id="root"> \n <!--app-html--> \n </div>`), React will encounter empty text nodes on load and trigger structural hydration warnings.
3. **Mismatched Client/Server Date & Times**:
   Never execute standard dynamic operations inside React component rendering pathways (e.g., `{new Date().toLocaleTimeString()}`). Because the server time-zone will differ from the client's localized timezone, this will produce mismatch errors. **Always calculate dynamic values inside client-only React `useEffect` callback steps.**
