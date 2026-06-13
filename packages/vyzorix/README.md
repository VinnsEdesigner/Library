# @vyzorix/ui

A package containing the exact, premium authentication forms and loader elements designed for Vyzorix Secure Operator Portal platforms.

## 📦 Installation

```bash
npm install @vyzorix/ui
```

Make sure you have peer dependencies pre-installed:
```bash
npm install react react-dom lucide-react
```

---

## 🎨 Design Theme & Tailwind Configuration

All components utilize standard **Tailwind Utility Classes**. Ensure your Tailwind CSS configuration includes appropriate support for colors (rose, slate, and neutral tones) and custom keyframe animations:

```css
@keyframes block-large-spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}
@keyframes block-pulse {
  0%, 100% { transform: scale(0.95); opacity: 0.9; }
  50% { transform: scale(1.05); opacity: 1; }
}
```

---

## 🔌 Export Map

### 1. `LoginForm`
Secure operator credentials validation view with support for OIDC Single Sign-On providers (Google, GitHub) and password recovery.

```tsx
import { LoginForm } from '@vyzorix/ui';

<LoginForm
  onLogin={(identifier, password) => handleLogin(identifier, password)}
  onSSO={(provider) => triggerSSO(provider)}
  onForgotPassword={() => setView('forgot_password')}
  isSubmitting={isLoading}
/>
```

### 2. `SignUpForm`
Alphanumeric operator workspace onboarding form equipped with strict name, email, credentials password match, and legal consent validation logic.

```tsx
import { SignUpForm } from '@vyzorix/ui';

<SignUpForm
  onSignUp={(data) => handleOnboarding(data)}
  onSSO={(provider) => triggerSSO(provider)}
  isSubmitting={isRegistering}
  triggerToast={(msg, type) => showToast(msg, type)}
/>
```

### 3. `ForgotPasswordForm`
Interactive email reset link transmitter with dispatch verification screens and auto-resend throttle timers.

```tsx
import { ForgotPasswordForm } from '@vyzorix/ui';

<ForgotPasswordForm
  onResetSubmit={(email) => requestRecovery(email)}
  onBackToLogin={() => setView('login')}
  isSubmitting={isDispatching}
  resetSent={hasSent}
  setResetSent={(v) => toggleSent(v)}
/>
```

### 4. `WaitingVerification`
Custom secure asynchronous registration polling visual dashboard, featuring the signature double rotating rose spinning block loader, countdown session verification timers, and responsive real-time state logs.

```tsx
import { WaitingVerification } from '@vyzorix/ui';

<WaitingVerification
  email="operator_409@vyzorix.org"
  timeLeft={899}
  formatTime={(sec) => renderTime(sec)}
  onResend={() => triggerResend()}
  onCancel={() => recycleSession()}
  statusText="Synthesizing profile database entry..."
/>
```

### 5. `SuccessView`
Dynamic post-authenticated operations profile card with structured metadata grid containing Operator Roles, sorting UUID IDs, geographical location regions, and system access points.

```tsx
import { SuccessView } from '@vyzorix/ui';

<SuccessView
  successReport={sessionData}
  onProceed={() => window.location.assign('/authorized-core')}
/>
```

### 6. `SpinningBlocksLoader`
Signature dual-block loader layout using clean custom nested rotation arrays.
