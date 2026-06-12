# GitHub OAuth Complete Implementation & Flow Architecture

Implementing secure **GitHub Single Sign-On (SSO)** into your high-performance stack (React TanStack SSR + Go Backend + SQLite v3) utilizes standard OAuth 2.0. 

Because GitHub does not natively focus on OIDC (OpenID Connect) token standards, the authentication server trades the code for an access token, then dispatches an authenticated HTTP callback to fetch the user's primary emails and profile metadata with 2 separate GET requests.

This document details the configuration flows and Go + SQLite v3 requirements.

---

## 1. End-to-End SSO Sequence Flow

```
┌──────────┐          ┌──────────────┐          ┌─────────────┐          ┌──────────────┐
│ Browser  │          │ Nitro NodeJS │          │ Go Backend  │          │ GitHub Auth  │
│ (React)  │          │ (Hydration)  │          │  (SQLite)   │          │  API V3      │
└────┬─────┘          └──────┬───────┘          └──────┬──────┘          └──────┬───────┘
     │                       │                         │                        │
     │  1. Click GitHub SSO  │                         │                        │
     ├──────────────────────►│                         │                        │
     │                       │  2. Redirect Proxy      │                        │
     │                       ├────────────────────────►│                        │
     │                       │                         │ 3. Build Redirect URI  │
     │                       │                         │    with Client ID      │
     │                       │                         ├───────────────────────►│
     │                       │  4. Location Redirection│                        │
     │                       │◄────────────────────────┤                        │
     │  5. Redirection to Authorize application        │                        │
     ├─────────────────────────────────────────────────┼───────────────────────►│
     │                                                 │                        │
     │  6. User Approves Permission Request            │                        │
     │◄────────────────────────────────────────────────┼────────────────────────┤
     │                                                 │                        │
     │  7. Redirection to Callback with ?code=Y        │                        │
     ├─────────────────────────────────────────────────┼───────────────────────►│
     │                                                 │                        │
     │                       │  8. Callback proxy      │                        │
     │                       ├────────────────────────►│                        │
     │                       │                         │ 9. POST to exchange    │
     │                       │                         │    ?code=Y for Token   │
     │                       │                         ├───────────────────────►│
     │                       │                         │                        │
     │                       │                         │ 10. GET User Profile   │
     │                       │                         ├───────────────────────►│
     │                       │                         │                        │
     │                       │                         │ 11. GET Private Emails │
     │                       │                         ├───────────────────────►│
     │                       │                         │                        │
     │                       │                         │ 12. Persist to SQLite, │
     │                       │                         │     Create Session     │
     │                       │                         │◄───────────────────────┤
     │                       │  13. Hydrate UI         │                        │
     │                       │◄────────────────────────┤                        │
     │ 14. Render Success Verification Screen          │                        │
     │◄──────────────────────┘                         │                        │
```

---

## 2. Integrated Implementation Layers

To successfully map GitHub OAuth authorizations into the workspace portals:

| File Path | Layer | Role |
| :--- | :--- | :--- |
| **`src/lib/config.ts`** | Frontend (React) | Configure endpoint routes. Toggles simulation parameters globally. |
| **`src/lib/clients/ssoClient.ts`** | Frontend (React) | Directs browser to initiate the GitHub login routing logic. |
| **`main.go`** | Backend (Go) | Exposes authentication loops, fetches profile JSON metadata, and maps standard GitHub roles. |
| **`database.go`** | Backend (Go) | Creates user record indexes mapped by UUIDv7 keys in SQLite. |

---

## 3. GitHub OAuth API Endpoints & Go Handler Schema

### A. Step 1: Redirecting the Client to GitHub
GitHub requires requesting specific read permission scopes inside url string parameters. To capture verify-ready user profiles, request `read:user` and `user:email` scopes.

* **GitHub Authorization Root**: `https://github.com/login/oauth/authorize`

```go
package handlers

import (
	"crypto/rand"
	"encoding/hex"
	"net/http"
	"net/url"
)

func HandleGitHubLogin(w http.ResponseWriter, r *http.Request) {
	// 1. Generate standard security check state
	b := make([]byte, 16)
	rand.Read(b)
	state := hex.EncodeToString(b)
	
	// Save checking state inside HttpOnly session cookie...

	// 2. Build Authorize Target URI
	params := url.Values{}
	params.Set("client_id", "YOUR_GITHUB_CLIENT_ID")
	params.Set("redirect_uri", "https://yourdomain.com/api/auth/sso/github/callback")
	params.Set("scope", "read:user user:email")
	params.Set("state", state)

	redirectURL := "https://github.com/login/oauth/authorize?" + params.Encode()
	http.Redirect(w, r, redirectURL, http.StatusTemporaryRedirect)
}
```

### B. Step 2: The Github Token & Profile Handshake
The Go Server accepts the transient callback authorization Code, performs a POST request to obtain an Access Token, retrieves profile parameters, and fetches additional user private emails to construct database indexes.

* **Exchange Gateway**: `https://github.com/login/oauth/access_token`
* **Profile Metadata API**: `https://api.github.com/user`
* **Private Emails API**: `https://api.github.com/user/emails`

```go
package handlers

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/url"
)

type GitHubTokenResponse struct {
	AccessToken string `json:"access_token"`
	TokenType   string `json:"token_type"`
	Scope       string `json:"scope"`
}

type GitHubUser struct {
	ID        int64  `json:"id"`       // Numeric constant identifier from GitHub
	Login     string `json:"login"`    // username handle
	Name      string `json:"name"`     // full name string
}

type GitHubEmail struct {
	Email    string `json:"email"`
	Primary  bool   `json:"primary"`
	Verified bool   `json:"verified"`
}

func HandleGitHubCallback(w http.ResponseWriter, r *http.Request) {
	code := r.URL.Query().Get("code")

	// 1. Trade authorization code for standard OAuth Access Token
	resp, _ := http.PostForm("https://github.com/login/oauth/access_token", url.Values{
		"client_id":     {"YOUR_GITHUB_CLIENT_ID"},
		"client_secret": {"YOUR_GITHUB_SECRET_KEY"},
		"code":          {code},
		"redirect_uri":  {"https://yourdomain.com/api/auth/sso/github/callback"},
	})
	defer resp.Body.Close()

	// Parse payload headers with content negotiations matching "application/json"
	var tokenResp GitHubTokenResponse
	json.NewDecoder(resp.Body).Decode(&tokenResp)

	// 2. Query GitHub Core Profile information
	req, _ := http.NewRequest("GET", "https://api.github.com/user", nil)
	req.Header.Set("Authorization", "Bearer "+tokenResp.AccessToken)
	
	client := &http.Client{}
	userResp, _ := client.Do(req)
	defer userResp.Body.Close()

	var ghUser GitHubUser
	json.NewDecoder(userResp.Body).Decode(&ghUser)

	// 3. Obtain primary email from authenticated resource API
	emailReq, _ := http.NewRequest("GET", "https://api.github.com/user/emails", nil)
	emailReq.Header.Set("Authorization", "Bearer "+tokenResp.AccessToken)
	
	emailResp, _ := client.Do(emailReq)
	defer emailResp.Body.Close()

	var ghEmails []GitHubEmail
	json.NewDecoder(emailResp.Body).Decode(&ghEmails)

	primaryEmail := ""
	for _, emailRecord := range ghEmails {
		if emailRecord.Primary && emailRecord.Verified {
			primaryEmail = emailRecord.Email
			break
		}
	}
	
	// If fields are empty, map login values
	if primaryEmail == "" && len(ghEmails) > 0 {
		primaryEmail = ghEmails[0].Email
	}

	// 4. Resolve/Upsert Operator record inside SQLite DB maps with UUIDv7
	// 5. Build HTTP session cookie & redirect user back to authenticated screen
}
```

---

## 4. SQLite User Link Model

Ensure to index the SSO federated ID matching parameters efficiently to avoid duplicating accounts. When user logs in via GitHub, verify if the `provider_user_id` matches, or if an account under `primaryEmail` already exists to map them contextually:

```sql
-- Checks if GitHub profile is already registered in identities table
SELECT operator_id FROM sso_identities 
WHERE provider = 'github' AND provider_user_id = ?;

-- Checks if account email is already registered via credentials
SELECT id FROM operators WHERE email = ?;
```
