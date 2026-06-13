# Google SSO (OpenID Connect) Complete Implementation & Flow Architecture

Implementing secure **Google Single Sign-On (SSO)** in your high-performance stack (React TanStack SSR + Go Backend + SQLite v3) requires exactly **4 integrated files** across the front-end, hydration, and back-end layers. 

This document outlines the end-to-end execution flow, redirect URLs, database logic, and code structures required.

---

## 1. End-to-End SSO Sequence Flow

```
┌──────────┐          ┌──────────────┐          ┌─────────────┐          ┌──────────────┐
│ Browser  │          │ Nitro NodeJS │          │ Go Backend  │          │ Google Auth  │
│ (React)  │          │ (Hydration)  │          │  (SQLite)   │          │  Servers     │
└────┬─────┘          └──────┬───────┘          └──────┬──────┘          └──────┬───────┘
     │                       │                         │                        │
     │  1. Hover & Click SSO │                         │                        │
     ├──────────────────────►│                         │                        │
     │                       │  2. Redirect Proxy      │                        │
     │                       ├────────────────────────►│                        │
     │                       │                         │ 3. Generate Auth URI   │
     │                       │                         │    with State Token    │
     │                       │                         ├───────────────────────►│
     │                       │  4. Redirect Location   │                        │
     │                       │◄────────────────────────┤                        │
     │  5. Temporary Redirection to Google Auth Screen │                        │
     ├─────────────────────────────────────────────────┼───────────────────────►│
     │                                                 │                        │
     │  6. User Approves Authorization & Multi-Factor  │                        │
     │◄────────────────────────────────────────────────┼────────────────────────┤
     │                                                 │                        │
     │  7. Redirection to Callback with ?code=X        │                        │
     ├─────────────────────────────────────────────────┼───────────────────────►│
     │                                                 │                        │
     │                       │  8. Callback proxy      │                        │
     │                       ├────────────────────────►│                        │
     │                       │                         │ 9. Exchange ?code=X    │
     │                       │                         │    for ID Token (JWT)  │
     │                       │                         ├───────────────────────►│
     │                       │                         │                        │
     │                       │                         │ 10. Verify Signature,  │
     │                       │                         │     Extract user data, │
     │                       │                         │     Upsert SQLite DB   │
     │                       │                         │◄───────────────────────┤
     │                       │                         │                        │
     │                       │  11. Render Success     │                        │
     │                       │      with HTTP Session  │                        │
     │                       │◄────────────────────────┤                        │
     │ 12. App Hydrated with Verified User State       │                        │
     │◄──────────────────────┤                         │                        │
```

---

## 2. The 4 Essential Implementation Files

To bring Google SSO live without visual changes or massive refactoring, you must configure the following files:

| File Path | Environment | Purpose |
| :--- | :--- | :--- |
| **`1. src/lib/config.ts`** | Frontend (React) | Standardizes redirection destination. Set `IS_SIMULATED: false` to enable production endpoints. |
| **`2. src/lib/clients/ssoClient.ts`** | Frontend (React) | Coordinates redirecting client browser state via standard `window.location.href` to trigger the flow. |
| **`3. main.go`** (or `auth_handlers.go`) | Go Backend | Performs OAuth handshake exchanges with Google's endpoints via TLS/SSL, extracts user profile information, and signs session cookies. |
| **`4. database.go`** (SQLite Layer) | Go Backend / DB | Inserts Operator records with sortable SQLite UUIDv7 IDs, mapping OAuth identifiers to federated profiles. |

---

## 3. Detailed Google API Specs & Backend Code Blueprint

### A. Step 1: Initiating Redirection (Go Handler)
The Go Backend generates a randomized `state` parameter to prevent Cross-Site Request Forgery (CSRF). It saves the state value in a temporary secure `httpOnly` cookie before redirecting the client to Google's OAuth gateway.

* **Target endpoint**: `GET /api/auth/sso/google`
* **Google base redirect URL**: `https://accounts.google.com/o/oauth2/v2/auth`

```go
package handlers

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
	"net/url"
	"time"
)

func HandleGoogleLogin(w http.ResponseWriter, r *http.Request) {
	// 1. Generate CSRF State Token
	b := make([]byte, 16)
	rand.Read(b)
	state := hex.EncodeToString(b)

	// Save verification cookie
	cookie := &http.Cookie{
		Name:     "oauth_state",
		Value:    state,
		Expires:  time.Now().Add(5 * time.Minute),
		HttpOnly: true,
		Secure:   true,
		Path:     "/",
	}
	http.SetCookie(w, cookie)

	// 2. Build URL parameters
	params := url.Values{}
	params.Set("client_id", "YOUR_GOOGLE_CLIENT_ID.apps.googleusercontent.com")
	params.Set("redirect_uri", "https://yourdomain.com/api/auth/sso/google/callback")
	params.Set("response_type", "code")
	params.Set("scope", "openid email profile")
	params.Set("state", state)

	redirectURL := "https://accounts.google.com/o/oauth2/v2/auth?" + params.Encode()
	http.Redirect(w, r, redirectURL, http.StatusTemporaryRedirect)
}
```

### B. Step 2: The Callback Handshake (Go Handler)
The Go backend validates the `state` matching parameter, trades the short-lived `code` for a cryptographic JSON Web Token (JWT) on Google secure servers (`https://oauth2.googleapis.com/token`), decodes user payload information, and updates SQLite.

* **Target Callback**: `GET /api/auth/sso/google/callback?code=CODE&state=STATE`

```go
package handlers

import (
	"encoding/json"
	"net/http"
	"net/url"
	"strings"
)

type GoogleTokenResponse struct {
	AccessToken string `json:"access_token"`
	IDToken     string `json:"id_token"`
}

type GoogleClaims struct {
	Email         string `json:"email"`
	Name          string `json:"name"`
	Picture       string `json:"picture"`
	VerifiedEmail bool   `json:"email_verified"`
	Sub           string `json:"sub"` // Google's unique federated User ID
}

func HandleGoogleCallback(w http.ResponseWriter, r *http.Request) {
	// 1. Verify CSRF Cookie State matches current r.URL.Query().Get("state")
	state := r.URL.Query().Get("state")
	cookie, err := r.Cookie("oauth_state")
	if err != nil || cookie.Value != state {
		http.Error(w, "Invalid Security Handshake State", http.StatusForbidden)
		return
	}

	// 2. Trade Code for tokens
	code := r.URL.Query().Get("code")
	resp, err := http.PostForm("https://oauth2.googleapis.com/token", url.Values{
		"code":          {code},
		"client_id":     {"YOUR_GOOGLE_CLIENT_ID"},
		"client_secret": {"YOUR_GOOGLE_SECRET_KEY"},
		"redirect_uri":  {"https://yourdomain.com/api/auth/sso/google/callback"},
		"grant_type":    {"authorization_code"},
	})
	if err != nil {
		http.Error(w, "Failed Google authentication handshake", http.StatusInternalServerError)
		return
	}
	defer resp.Body.Close()

	var tokenResp GoogleTokenResponse
	json.NewDecoder(resp.Body).Decode(&tokenResp)

	// Decode claims (OIDC standards allow splitting the IDToken JWT locally for efficiency)
	payloadSegment := strings.Split(tokenResp.IDToken, ".")[1]
	// Decode base64 payload to target GoogleClaims...
	
	// 3. Persist Operator Profile locally into SQLite with UUIDv7
	// 4. Issue a secure session cookie cookie, then Hydrate frontend view
}
```

---

## 4. SQLite Schema Mapping

To protect logins and allow operators to link multiple directories (e.g. log in with Google OR GitHub and reference the identical workspace), map the SQLite queries with a separate identity structure:

```sql
-- Main operator information
CREATE TABLE IF NOT EXISTS operators (
    id TEXT PRIMARY KEY NOT NULL,             -- sortable UUIDv7
    full_name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- SSO Links
CREATE TABLE IF NOT EXISTS sso_identities (
    id TEXT PRIMARY KEY NOT NULL,             -- sortable UUIDv7
    operator_id TEXT NOT NULL,                -- references operators(id)
    provider TEXT NOT NULL,                   -- 'google' or 'github'
    provider_user_id TEXT UNIQUE NOT NULL,    -- maps to IDToken "sub" value
    FOREIGN KEY(operator_id) REFERENCES operators(id) ON DELETE CASCADE
);
```
