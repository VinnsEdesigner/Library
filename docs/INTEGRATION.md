# Vyzorix Architecture & Integration

This guide details how to integrate  into the Vyzorix C2 dashboard**TypeScript**, **pnpm**, **Vite**, **React with TanStack SSR**, **Nitro Node.js server (HTML Hydration)**, and a **Go Backend Server with SQLite v3**.

---

## 1. Architecture Design

We have split the integration interface into decoupled, unified micro-clients stored inside `/src/lib/clients/` and coordinate everything through a central switchboard in `/src/lib/config.ts`.

```
                  ┌───────────────────────────────┐
                  │       React Frontend UI       │
                  │   App.tsx  |  SignUpForm      │
                  └───────────────┬───────────────┘
                                  │
                                  ▼
                  ┌───────────────────────────────┐
                  │     Central API Router        │
                  │         src/lib/api.ts        │
                  └───────────────┬───────────────┘
                                  │
         ┌────────────────────────┼────────────────────────┐
         ▼                        ▼                        ▼
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│ Credentials Client│     │    SSO Client    │     │Verification Client│
│  authClient.ts   │     │   ssoClient.ts   │     │verificationClient│
└──────────────────┘     └──────────────────┘     └──────────────────┘
```

---

## 2. Dynamic Integration Swapping (SPA vs SSR)

You can toggle the entire runtime behavior of the frontend with a safe, **one-line change** inside `/src/lib/config.ts`:

```typescript
export const ARCHITECTURE_CONFIG = {
  // 1-LINE TOGGLE: Swap mode between 'SPA' (Static local) and 'SSR' (Production dynamic hydration)
  MODE: 'SPA' as 'SPA' | 'SSR',

  // 1-LINE TOGGLE: Swap connectivity between mock simulation or live Go backend requests
  IS_SIMULATED: false,

  API_BASE_URL: '/api/auth',
  GO_BACKEND_SERVER: 'http://localhost:8080',
};
```

### Server-Side Hydration (SSR) Implementation
When in SSR Mode, States in `/src/App.tsx` are pre-initiated during server render cycles using the `getHydratedState` hook, checking if server variables have been injected into `window.__VYZORIX_PREFETCHED_STATE__`.

---

## 3.Security Single Sign-On (SSO) Integration Setup

Both **Google OAuth 2.0** and **GitHub OAuth App** integration require setting up credentials on their developer consoles. The Go Backend acts as the OAuth callback listener, keeping your private Client Secrets isolated.

### Step-by-Step Configuration

#### A. Google Cloud Platform (OIDC / OAuth 2.0)
1. Go to **APIs & Services** > **Credentials** > **OAuth consent screen** inside the GCP Console.
2. Select **External** and register your App name (`Vyzorix`).
3. Create an **OAuth Client ID** of type *Web Application*.
4. Set the following URL parameters:
   - **Authorized JavaScript Origins**: `https://vyzorix.yourdomain.com` (or `http://localhost:3000`)
   - **Authorized Redirect URIs**: `https://vyzorix.yourdomain.com/api/auth/sso/google/callback`

#### B. GitHub Developer Settings
1. Go to the profile drop-down, choose **Settings** > **Developer Settings** > **OAuth Apps** > **New OAuth App**.
2. Configure:
   - **Homepage URL**: `https://vyzorix.yourdomain.com`
   - **User Authorization Callback URL**: `https://vyzorix.yourdomain.com/api/auth/sso/github/callback`

---

## 4. SQLite v3 Database Schema utilizing UUIDv7 in Go

Your relational data structures are written to **SQLite v3** using UUIDv7 as primary keys. UUIDv7 is structurally sortable, high-performance, and protects corporate entities.

### A. SQLite Table Declarations (`schema.sql`)

```sql
-- 1. Operators Table
CREATE TABLE IF NOT EXISTS operators (
    id TEXT PRIMARY KEY NOT NULL,          -- UUIDv7 (String format, e.g., '018f6f5d-79e5-7977-9d7a-7bf9b4d8ecdc')
    full_name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT,                     -- Nullable for SSO-only operators
    operator_role TEXT NOT NULL DEFAULT 'Operator',
    region TEXT NOT NULL DEFAULT 'Paris, France',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- 2. SS0 Identities (Maps multiple SSO logins to a single Operator account)
CREATE TABLE IF NOT EXISTS sso_identities (
    id TEXT PRIMARY KEY NOT NULL,          -- UUIDv7
    operator_id TEXT NOT NULL,
    provider TEXT NOT NULL,                 -- 'google' or 'github'
    provider_user_id TEXT UNIQUE NOT NULL,  -- Federated unique ID
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY (operator_id) REFERENCES operators(id) ON DELETE CASCADE
);

-- 3. Verification Tokens Table (Poll tracking)
CREATE TABLE IF NOT EXISTS verification_tokens (
    token TEXT PRIMARY KEY NOT NULL,        -- Random verification token string
    operator_id TEXT NOT NULL,
    email TEXT NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    is_used INTEGER DEFAULT 0 NOT NULL,
    FOREIGN KEY (operator_id) REFERENCES operators(id) ON DELETE CASCADE
);
```

### B. Corresponding Go Struct Definitions & UUIDv7 Construction

In Go, utilize the `github.com/google/uuid` module (supporting UUIDv7 beginning with v1.6.0) to insert records:

```go
package database

import (
	"database/sql"
	"time"
	"github.com/google/uuid"
)

type Operator struct {
	ID           string    `json:"id"`
	FullName     string    `json:"fullName"`
	Email        string    `json:"email"`
	Username     string    `json:"username"`
	PasswordHash string    `json:"-"`
	OperatorRole string    `json:"operatorRole"`
	Region       string    `json:"region"`
	CreatedAt    time.Time `json:"createdAt"`
}

// GenerateNewUUIDv7 generates a lexicographically sortable time-ordered UUID
func GenerateNewUUIDv7() (string, error) {
	u7, err := uuid.NewV7()
	if err != nil {
		return "", err
	}
	return u7.String(), nil
}

// CreateNewOperator persists a new validated operator 
func CreateNewOperator(db *sql.DB, fullName, email, username string) (*Operator, error) {
	u7, _ := GenerateNewUUIDv7()
	createdAt := time.Now().UTC()
	
	query := `INSERT INTO operators (id, full_name, email, username, operator_role, region, created_at, updated_at) 
	          VALUES (?, ?, ?, ?, 'Operator', 'Paris, France', ?, ?)`
	
	_, err := db.Exec(query, u7, fullName, email, username, createdAt, createdAt)
	if err != nil {
		return nil, err
	}

	return &Operator{
		ID:           u7,
		FullName:     fullName,
		Email:        email,
		Username:     username,
		OperatorRole: "Operator",
		Region:       "Paris, France",
		CreatedAt:    createdAt,
	}, nil
}
```

---

## 5. UI Fields & Endpoint Contract Mapping

Here is the precise mapping from user interactions to Go Backend endpoints serviced by the 3 Browser-Clients:

| Action Trigger | Frontend Element | Active Browser Client Function | HTTP Request |
| :--- | :--- | :--- | :--- |
| **New operator submit** | "Create Account" button inside `SignUpForm` | `registerOperator(payload)` in `authClient.ts` | `POST /api/auth/register` |
| **Operator log in** | "Sign In" button inside `LoginForm` | `loginOperator(identity, password)` in `authClient.ts` | `POST /api/auth/login` |
| **Trigger GitHub login** | GitHub Icon selector | `initiateSSO('GitHub')` in `ssoClient.ts` | `GET /api/auth/sso/github` |
| **Trigger Google login** | Google Icon selector | `initiateSSO('Google')` in `ssoClient.ts` | `GET /api/auth/sso/google` |
| **Dispatched reset email** | "Dispatch Link" inside `ForgotPasswordForm` | `requestPasswordReset(email)` in `authClient.ts` | `POST /api/auth/forgot-password` |
| **Awaiting verification** | Interval sequence ticker inside `WaitingVerification` | `pollVerificationStatus(token)` in `verificationClient.ts` | `GET /api/auth/poll-verification?token={T}` |
| **Resend link** | "Resend Link" inside `WaitingVerification` | `triggerTokenResend(email)` in `verificationClient.ts` | `POST /api/auth/resend-token` |
| **Cancel registration** | "Abort Route" inside `WaitingVerification` | `cancelVerificationSession(email)` in `verificationClient.ts` | `POST /api/auth/cancel-verification` |
