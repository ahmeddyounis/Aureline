# M5 managed auth-and-recovery fixtures

Fixture corpus for the `m5_auth_and_recovery` record. These fixtures pin the
seeded M5 managed identity events, their provider/owner disclosure, system-browser
handoff and passkey posture, the managed capabilities a live degraded condition
pauses, the local work that stays usable through it, the credential-store
account, and the degraded-state drills so a change to the typed model, the
fail-closed gate, or the records is caught against frozen evidence.

- Typed model: `crates/aureline-auth/src/m5_auth_and_recovery/`
- Schema: `schemas/auth/m5_auth_and_recovery.schema.json`
- Companion doc: `docs/auth/m5_auth_and_recovery.md`
- Emitter: `dump_m5_auth_and_recovery_fixtures`

## Files

- `corpus_manifest.json` — indexes the scenarios and what each proves (claim
  class and continuity ceiling).
- `calm-managed-baseline.json` — every M5 managed event is disclosed across
  desktop, companion, and browser-adjacent surfaces with provider/owner labels,
  passkey-capable system-browser handoff, the full credential-store account, and
  the complete security/accessibility/recovery drill set; managed identity is
  calm and local-first.
- `step-up-passkey-unavailable.json` — the step-up finds no passkey; an explicit
  keyboard-complete security-key fallback is offered and the gated managed
  actions pause while local work continues.
- `sign-in-browser-handoff-failure.json` — the sign-in system-browser handoff
  fails to return; managed sync and hosted AI pause behind a device-code fallback
  while local editing, files, and local-only workflows continue.
- `reauth-offline-identity.json` — re-auth cannot reach the identity authority
  while offline; managed capabilities pause and the row is labeled offline while
  local durable work keeps running.
- `policy-forced-sign-out.json` — policy forces the active session to sign out;
  every managed capability pauses with an explicit re-sign-in path while unsaved
  local work, files, and history stay intact.
- `deprovision-active-local-work.json` — the seat is deprovisioned while local
  work is active; all managed capabilities are removed, yet unsaved local work,
  files, history, and local-only workflows are explicitly preserved.
- `account-recovery-passkey-first.json` — account recovery runs passkey-first and
  falls back to a system-browser security key, never embedded password-first or
  CAPTCHA-only collection on a stable profile.
- `companion-browser-handoff-failure.json` — a companion re-auth handoff fails;
  companion control and managed sync pause behind a keyboard-complete device-code
  fallback while the companion's local view stays usable.

## What the corpus proves

- **Managed identity is disclosed, not opaque.** Every event names its
  provider/owner, its system-browser handoff method and typed reason, its passkey
  posture, the managed capabilities a degraded condition pauses, and the local
  work that stays usable.
- **Local work is never threatened.** Every event preserves local editing,
  files, history, and local-only workflows; a local-work impact can only encode
  with a named governed policy row, never silently.
- **Passkey-first, with explicit fallbacks.** Flows prefer system-browser
  passkeys; where passkeys are unavailable the fallback posture is explicit and
  keyboard-complete, and embedded password-first or CAPTCHA-only recovery is never
  required on a stable profile.
- **Credentials never leave the device.** Refresh tokens, delegated handles, and
  session-broker references live in OS-backed or approved enterprise stores and
  are excluded from portable-state, sync, and support export.
- **Degraded states are drilled honestly.** Passkey-unavailable, browser-handoff
  failure, offline identity, policy-forced sign-out, and deprovision-on-active-
  local-work each keep local work preserved, labeled, keyboard-complete, and
  recoverable, and are exercised as security, accessibility, and recovery drills.
- **Surfaces agree.** Desktop shell, CLI inspect, docs/help, and support exports
  all consume the same record.

The fixtures carry typed states and opaque refs only — no tokens, handles,
secrets, raw provider payloads, hostnames, serials, or workspace contents.
