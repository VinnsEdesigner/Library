# Vyzorix Rust CLI (Infrastructure Distribution)

This package contains the high-performance CLI written in Rust. It utilizes modern paradigms capable of running on raw machines, VMs, and extremely lightweight non-root containers.

## 🚀 Usage Methods

### 1. Direct Binary (Raw Machine)

Because the CLI is compiled ahead-of-time (AOT), you can distribute a single executable file.

```bash
make build
./target/release/vyzorix init
```

### 2. Isolated Docker Container (Non-Root)

Designed perfectly for CI/CD pipelines, Kubernetes, or VMs without polluting the host environment. The `Dockerfile` implements a multi-stage Alpine build, creating an isolated non-root user (`vyzorix`) to ensure maximum security.

Build the image:

```bash
make docker-build
```

Run commands against your local filesystem by mounting a volume:

```bash
# By default, runs "vyzorix --help"
make docker-run

# Run specific commands:
make docker-run CMD="init"
make docker-run CMD="doctor --dev"
```

### Security Posture

- **No-Root Execution:** The Dockerfile permanently drops root privileges.
- **Musl compilation:** Creates a statically linked binary. It doesn't rely on `glibc` floating on the host.
- **Lightweight:** Final compiled images are extremely small (~10-20MB instead of Node's ~200-500MB).

## 🧪 Phase 3: Testing & CI/CD Pipeline

The CLI integrates robust execution and environment testing:

- **Integration Tests**: Handled by the `assert_cmd` and `predicates` crates to test production CLI interactions.
- **CI/CD Pipelines**: Automated via `.github/workflows`:
  - `rust-ci.yml`: Performs formatting (`cargo fmt`), linting (`clippy`), and runs integration test suites on PRs.
  - `docker-publish.yml`: Automatically builds the multi-stage Alpine Docker image and tags/pushes to the remote registry on GitHub Releases.

Run local tests quickly using:

```bash
cargo test
```

## 🎨 Phase 4: Rich Terminal UX (TUI & Spinners)

We elevate the developer experience from basic terminal output to an interactive, polished CLI.

- **Interactive Prompts**: Using `dialoguer` to ask engineers for confirmation and framework selections natively.
- **Dynamic Spinners**: Using `indicatif` to offer smooth real-time progress indicators during threaded I/O operations.
- **Visual Engagement**: Colored indicators and interactive menus make the CLI intuitive, friendly, and robust—feeling like a premium developer tool comparable to the Vercel or Stripe CLIs.

## 🌩️ Phase 5: Auth & Cloud Synchronization

Premium native tools often integrate with their own cloud systems effortlessly. In Phase 5, we introduce real-time authentication and secure token handling:

- **`vyzorix auth`**: Implements a dedicated authentication module handling secure logins (`--login`) and token revocation (`--logout`).
- **Secret Management**: Capable of reading from or writing to the `.env` context or system keychains securely for production operations.
- **Brand Discipline**: Ensures all feedback uses strict Vyzorix branding (Rose-600 `#e11d48`, White, and Black).

## 🌍 Phase 6: Edge Deployment & Intelligent Config

The CLI reaches enterprise maturity with configuration management and cloud deployment:

- **`vyzorix deploy`**: Bundles the application recursively and simulates pushing payload shards to remote distributed edge clusters.
- **`vyzorix.toml` Engine**: Context-aware operations mapping to `vyzorix.toml` in the working directory using `serde` and `toml`. This allows the CLI to dynamically react and build context depending on the workspace.
- **Brand Themed URLs**: Renders success output including production vs preview URL generation with exact Vyzorix-standard coloring logic.

## ⚡ Phase 7: Real-time Development Server (Dev & HMR)

Completes the local developer experience by introducing a watching multiplexer daemon:

- **`vyzorix dev`**: Spawns a multi-threaded asynchronous `tokio` daemon that watches filesystem nodes and simulates HMR (Hot Module Replacement) delivery.
- **Asynchronous Execution & Signals**: Integrates `tokio::select!` for hyper-performant, non-blocking polling and handles system-level interrupt signals (`Ctrl+C`) to guarantee graceful shutdowns.
- **Vyzorix Aesthetic Logs**: Streamlines terminal output by cleanly separating server bounds, watchers, and timestamped HMR events without terminal jitter.

## 🔄 Phase 8: Over-The-Air (OTA) Updates & Self-Healing

We culminate the CLI's autonomy by enabling frictionless binary updates directly via the cloud registry:

- **`vyzorix upgrade`**: Connects to the Vyzorix Registry to pull the latest version manifest, comparing semantic versions automatically.
- **Delta Patching**: Simulates securely downloading, verifying payload integrity via cryptographic signatures, and seamlessly hot-swapping the active binary in memory.
- **Self-Healing Architecture**: By utilizing this OTA method, we bypass system package managers (apt, brew), giving developers guaranteed access to emergency bug fixes without friction.

## 🔒 Phase 9: Secure Environment Vault

Protecting application secrets seamlessly from developer machines to the edge.

- **`vyzorix secrets set/list`**: Provides secure AES-GCM encrypted persistence for configuration variables. No keys are ever echoed locally, maintaining zero-trust architecture.
- **Edge Sync**: Simulates a secure bridge between local configurations and remote serverless edge boundaries.

## 🧩 Phase 10: Extensible Plugin Architecture (WASM)

No CLI should be monolythic. In Phase 10 we introduce modular extensibility natively.

- **`vyzorix plugin add/list`**: Dynamically downloads WebAssembly (WASM) compliant modules to extend local functionality on the fly without heavy binary redeployment.
- **Sandboxed Execution**: Plugins operate within tight permissions ensuring robust supply chain security.

## 🚀 Phase 11: Enterprise Maturity (25+ Module Expansion)

Phase 11 injects heavy artillery into the Vyzorix CLI covering Data, Infrastructure, Analytics, CI/CD, and AI generation features across **25 interconnected modules**:

- **Database Orchestration (`vyzorix db`)**: Full-fledged schema migrations, remote proxy seeding, and atomic rollbacks mapped securely to `db_engine`.
- **Multi-Cloud IaC (`vyzorix infra`)**: First-class terraform-less scaffolding for AWS, GCP, and Azure right from your terminal without locking.
- **Edge Telemetry (`vyzorix analytics`)**: Real-time inspection of distributed logs and alert aggregations powered by the `telemetry` core engine.
- **CI/CD Autobuilder (`vyzorix cicd`)**: Injects enterprise-grade GitHub Actions, GitLab CI, and CircleCI configurations pre-optimized for Rust/Node environments.
- **Vyzorix AI Autopilot (`vyzorix ai`)**: An LLM-backed terminal copilot (`ai_client`) for intelligent code generation, security anomaly auditing, and optimization checks directly within the workspace.

## 🧱 Phase 12: E2E Integration & Subsystem Maturity (25+ Files)

The CLI reaches enterprise production strength here with a sweeping integration of **24+ architectural files** supporting advanced operations:

- **`src/core/net`**: Features a fully configured `.reqwest` client with persistent TLS connection pooling and custom Vyzorix User-Agent bindings.
- **`src/core/fs`**: Leverages `ignore` and `flate2` internally for recursive Git-aware tree scanning and ultra-fast `.tar.gz` bundle compression during edge deployments.
- **`src/core/crypto`**: Secures environment variables with genuine state-of-the-art **AES-256-GCM** encryption and introduces `SHA-256` hashing mechanics to protect memory buffers.
- **`src/core/wasm`**: Lays out the abstraction layer for embedding custom plugins compiled via WebAssembly within a rigid permissions sandbox.
- **`src/types` & `src/services`**: Implements strong Serde mapping types (`edge`, `api`, `auth`) bridging the CLI to remote cloud services gracefully.

## 🤝 Phase 13: Distributed Collaboration & Package Registry

Scaling from single-player development to massive global engineering teams seamlessly:

- **Team Management (`vyzorix team`)**: Handle invites, RBAC (Role-Based Access Control) configuration, and token revocations across your workspace without leaving the terminal.
- **Global Edge Registry (`vyzorix registry`)**: An enterprise-grade, immutably signed package system. Publish extensions (`vyzorix registry publish`), search for team payloads, and orchestrate WASM modules transparently.
- **Service Layers**: Backed by secure network models routing directly to `api.vyzorix.com` using the `reqwest` context.

## 🛡️ Phase 14: Hardened Subsystems & Global Distribution

Bringing real runtime engines to life and deploying them across global development environments.

- **Cross-Platform Matrix (CI/CD)**: Added `.github/workflows/release.yml` with a multi-os matrix covering Apple Silicon, macOS Intel, Linux, and Windows MSVC native targeting.
- **`install.sh`**: Developed a fully-automated, zero-dependency curl one-liner for bootstrapping Vyzorix globally (`curl -fsSL vyzorix.com/install.sh | bash`).
- **Interactive TUI (Terminal UI)**: `vyzorix analytics realtime` now hooks into `ratatui` + `crossterm` providing split pane immersive graph and log rendering.
- **AES Cryptography Wired**: Handled real `aes-gcm` block-cypher implementation for `vyzorix secrets set/list`, logging cryptographic nonces safely to the terminal without compromising payloads.
- **Wasmtime Engine Live**: Woven real `wasmtime::Engine` initialization directly into the core sandbox processor, allowing true plugin memory-block mounting.
