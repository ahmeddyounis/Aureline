# M5 Managed Auth-and-Recovery

**Artifact ref:** `artifacts/auth/m5_auth_and_recovery.md`  
**Contract ref:** `auth:m5_auth_and_recovery:v1`  
**Schema version:** 1  
**As of:** 2026-06-12

## Purpose

This artifact certifies that M5 managed identity events — sign-in, step-up,
re-auth, session revocation, deprovision, and account recovery — narrow only the
requested managed capability and never threaten local work, across the desktop
shell, companion, and browser-adjacent paths. Managed sign-in and recovery flows
disclose owner/provider, reason, paused capabilities, and local-only continuity;
loss of managed identity never implies loss of unsaved local work, local files,
or local-only workflows.

## Certification Scope

The canonical record binds:

1. Six managed event disclosures — `managed_sign_in`, `step_up`, `reauth`,
   `session_revocation`, `deprovision`, and `account_recovery` — spread across
   the `desktop`, `companion`, and `browser_adjacent` surfaces, each disclosing
   provider/owner, the system-browser handoff method and typed reason, the
   passkey posture, and a keyboard-complete fallback.
2. Live degraded conditions and standalone drills for `passkey_unavailable`,
   `browser_handoff_failure`, `offline_identity`, `policy_forced_sign_out`, and
   `deprovision_on_active_local_work`, each pausing only managed capabilities,
   preserving local work, and exercised as `security`, `accessibility`, and
   `recovery` lenses.
3. A credential-store account proving refresh tokens, delegated handles, and
   session-broker references live in OS-backed or approved enterprise stores and
   stay excluded from portable-state, sync, and support export.
4. A surface-parity record showing the desktop shell, CLI inspect, docs/help,
   and support exports consume the same truth.
5. A fail-closed `trust_qualification` that withholds the calm
   `local_first_managed_safe` claim whenever a managed capability is paused and
   reports `unsupported` when a structural pillar fails.

## Guarantees

- **Provider and reason are disclosed.** Every managed sign-in and recovery flow
  names its owner/provider and the typed handoff reason before any browser
  handoff; the record cannot be built with an undisclosed owner.
- **Local work is never threatened.** Every event preserves local editing,
  files, history, and local-only workflows. A local-work impact can only encode
  with a named governed policy row; an ungoverned threat is a build error.
- **Passkey-first, with explicit, keyboard-complete fallbacks.** Flows prefer
  system-browser passkeys; where passkeys are unavailable the fallback posture is
  explicit and keyboard-complete. Embedded password-first or CAPTCHA-only
  recovery can never be required on a claimed stable profile.
- **Credentials never leave the device.** Refresh credentials and delegated
  handles stay in OS-backed or approved enterprise stores and are excluded from
  portable-state, sync, and support export by default.
- **Degraded states are drilled honestly.** The passkey-unavailable,
  browser-handoff-failure, offline-identity, policy-forced-sign-out, and
  deprovision-on-active-local-work drills each keep local work preserved,
  labeled, keyboard-complete, and recoverable.
- **Surfaces and exports agree.** Support exports and docs/help reconstruct the
  same auth-boundary and recovery state visible to the user, carrying typed
  states and opaque refs only.

## Evidence

- Typed model: `crates/aureline-auth/src/m5_auth_and_recovery/`
- Schema: `schemas/auth/m5_auth_and_recovery.schema.json`
- Fixture corpus: `fixtures/auth/m5_auth_and_recovery/`
- Companion doc: `docs/auth/m5_auth_and_recovery.md`
- Emitter: `dump_m5_auth_and_recovery_fixtures`

The fixtures pin one calm managed baseline plus seven degraded drills (step-up
passkey-unavailable, sign-in browser-handoff failure, re-auth offline identity,
policy-forced sign-out, deprovision on active local work, passkey-first account
recovery, and a companion browser-handoff failure), each replayed by the typed
model and validated against the schema so a regression in the model, the
fail-closed gate, or the records is caught against frozen evidence.
