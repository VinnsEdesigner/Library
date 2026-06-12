# Vyzorix Sign-up pages
 incorporates a modern React client, an Node.js Nitro-HML-based hydration and server proxy layer, and an asynchronous, performant Go API backend powered by SQLite (v3).

This architecture completely decouples persistent states from vulnerable client-side storage technologies like `localStorage`, moving instead to secure, **encrypted, and HttpOnly cookie blobs** managed at the multi-tier boundaries.

---

## 🏗️ System Architecture & Mechanics

```
               [ User Browser / Client-Side Client ]
               │  - React 18 / Tailwind CSS / Lucide
               │  - Strict Cookie-Driven State (Local Storage Deprecated)
               ▼
   [ Node.js Hydration Server ] (Port 3000)
   ├── Asset compilation & Vite streaming dev server
   └── Session Pre-prefetching: Reads cookie state or pending verification state,
       injecting into `window.__VYZORIX_PREFETCHED_STATE__`
               │
               ▼ (API Proxies)
   [ Go Authentication Engine ] (Port 8080)
   ├── SQLite Dynamic Database Transactions (vyzorix.db)
   ├── Cryptographic Token Generator & anti-forgery CSRF validation
   └── Google & GitHub SSO OAuth Handshake Clients
```

---

## 🔒 Cookie-Driven Session Hydration & Storage Strategy

By strict design mandate, **no user state, profile indexes, or active verification sessions are preserved in the browser's `localStorage`**. Instead, state flow is bound to two state-tracking HttpOnly cookies issued by the server:

1. **`vyzorix_session`** (HttpOnly, SameSite=Lax):
   Contains the verified `operator_id` (a lexicographically ordered, time-based sorting **UUIDv7** reference) indicating a fully logged-in and authorized session.

2. **`vyzorix_pending_auth`** (HttpOnly, SameSite=Lax):
   Enables seamless registration and sign-up session restoration across page reloads. It holds a structured pipe-delimited payload: 
   `[verification_token]|[full_name]|[email]|[username]`. 

### The Hydration Flow (`server.ts`):
1. Upon incoming root requests, the Express server accesses the client's cookie list.
2. If `vyzorix_session` exists, the server queries the Go backend's `/api/auth/me` endpoint in the background to resolve the active profile.
3. If `vyzorix_pending_auth` exists, the server decodes the token and operator context.
4. The server synthesizes these pre-initialized structures into `window.__VYZORIX_PREFETCHED_STATE__` and injects them directly into the delivered `index.html` file before streaming it to the browser.
5. In React, `getHydratedState()` reads this initialized state dynamically on mount, preventing flash-of-unstyled-content (FOUC), redirect loops, or structural desyncs.

---

## ⚙️ Layer Structure & Key Files

### 🖥️ Layer 1: React client-side Frontend (SPA client)
*  **`src/App.tsx`**: Governs global portal layout, navigation state machines (Login / SignUp / Recover/ Waiting for Verification / Auth Success), real-time polling engines, and reactive UI transitions.
*  **`src/lib/api.ts`**: High-performance HTTP client that calls native API routes.
*  **`src/lib/config.ts`**: System environment parameters and hydration bridge mappings.
*  **`src/components/`**: Modular layout forms (`LoginForm`, `SignUpForm`, `CredentialsRecoverForm`, `VerificationModal`).

### 📦 Layer 2: Frontend Server Hydration (`server.ts`)
*  **`server.ts`**: Sets up Vite middleware for streaming local assets, proxies incoming `/api/auth/*` traffic straight to the Go backend, and prefetches cookie states to inject the hydrated variable block.

### 📜 Layer 3: Go Backend Server API
*  **`main.go`**: System entry point, triggers SQLite setups and handles cross-origin resource sharing (CORS) header injection rules.
*  **`router.go`**: Outlines REST handlers (Register, Login, Me, Logout, Poll, Reset validation) communicating with SQLite database blocks.
*  **`database/operators.go`**: Controls relational database transactions and table creation. Utilizes UUIDv7 keys for fast indexed indexing.
*  **`services/`**: Formulates external API integrations for OIDC endpoints (Google, GitHub OAuth integrations).

---

## 📊 Database Schema Definitions (SQLite)

The backend maintains persistent records in `vyzorix.db` with the following entities:

1. **`operators`**: Holds standard operator details.
   * `id` **TEXT (UUIDv7)** Primary Key
   * `full_name` **TEXT**
   * `email` **TEXT UNIQUE**
   * `username` **TEXT UNIQUE**
   * `password_hash` **TEXT** (Cryptographically signed with SHA-256)
   * `operator_role` **TEXT** (Defaults to 'Operator')
   * `region` **TEXT** (Defaults to 'Paris, France')

2. **`sso_identities`**: Map table connecting federated oauth identifiers to standard operators.
   * `id` **TEXT** Primary Key
   * `operator_id` **TEXT** Foreign Key -> `operators(id)`
   * `provider` **TEXT** (e.g. 'google', 'github')
   * `provider_user_id` **TEXT UNIQUE**

3. **`verification_tokens`**: Facilitates email-response security validation workflows.
   * `token` **TEXT** Primary Key (UUIDv7)
   * `operator_id` **TEXT** Foreign Key -> `operators(id)`
   * `email` **TEXT**
   * `expires_at` **TIMESTAMP** (15-minute standard lifespan)
   * `is_used` **INTEGER** (0=False, 1=True)
   * `poll_count` **INTEGER** (Tracks polling iterations)

---

## 🔌 API Route Map

| Method | Endpoint | Description | Lifecycle Context |
| :--- | :--- | :--- | :--- |
| `POST` | `/api/auth/register` | Inserts Operator and launches verification token. | Registration Step |
| `POST` | `/api/auth/login` | Compares password hash and issues `vyzorix_session` cookie. | Standard Login |
| `POST` | `/api/auth/logout` | Invalidates authentication and pending progress cookies. | Global Exit |
| `GET` | `/api/auth/me` | Hydrates current profile index according to session cookie. | Session Verification |
| `GET` | `/api/auth/poll-verification` | Checks verification status via token. | Background Polling |
| `POST` | `/api/auth/resend-token` | Cancels old session tokens and issues a fresh one. | Resend Sequence |
| `POST` | `/api/auth/cancel-verification` | Purges unverified operators and resets state. | Cancel Sequence |
| `GET` | `/api/auth/sso/google` | Generates CSRF states and redirects target to Google. | Google Federated Login |
| `GET` | `/api/auth/sso/github` | Generates CSRF states and redirects target to GitHub. | GitHub Federated Login |

---

## 🏃 DEVELOPMENT & BUILD GUIDES

### Prerequisites
* **Node.js** (v18+) and **npm**
* **Go** (1.24+) and standard C compilers (for `go-sqlite3` bindings)

### Installation & Compilation
1. Install client-side node packages:
   ```bash
   npm install
   ```
2. Build client assets and compile TypeScript files for Node production:
   ```bash
   npm run build
   ```
3. Boot the development ecosystem:
   ```bash
   npm run dev
   ```
   *This commands initializes `tsx server.ts` which proxies traffic synchronously, keeping your front-end hot-reloads and API endpoints perfectly aligned**
