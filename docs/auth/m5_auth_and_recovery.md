# M5 Managed Auth-and-Recovery

**Doc ref:** `docs/auth/m5_auth_and_recovery.md`  
**Contract ref:** `auth:m5_auth_and_recovery:v1`  
**Schema version:** 1

## Overview

This document defines the shared product truth for how M5 managed identity
events behave across the desktop shell, companion, and browser-adjacent paths.
The canonical record is `M5AuthAndRecovery` in
`crates/aureline-auth/src/m5_auth_and_recovery/`. The desktop shell, CLI inspect,
docs/help, and support exports consume the same record so they describe sign-in,
step-up, re-auth, revocation, deprovision, and recovery identically instead of
cloning auth status text.

The record replaces an opaque "signed-in / signed-out" toggle with one
inspectable, local-first auth-boundary model. Managed identity events narrow only
the requested managed capability and never threaten local work: local editing,
files, history, and local-only workflows stay usable through every event, sign-out,
revocation, and deprovision.

## Managed events

Every required managed event ships one `AuthEventRow`:

- `managed_sign_in`
- `step_up`
- `reauth`
- `session_revocation`
- `deprovision`
- `account_recovery`

Each event is a typed disclosure rather than a chip. It carries:

- `provider_label` / `org_label` — the owner/provider disclosed before any
  handoff (never raw tokens or endpoints).
- `surface` — `desktop`, `companion`, or `browser_adjacent`. The required event
  set is spread across all three surfaces.
- `handoff` — the system-browser `method` (`system_browser_passkey`,
  `system_browser_security_key`, `system_browser_device_code`, or
  `system_browser_federated_password`), the typed `reason`
  (`interactive_sign_in`, `step_up_challenge`, `policy_reauth`, `recovery_flow`),
  an opaque `return_route_ref`, and a `keyboard_complete_fallback` flag.
- `passkey_posture` — `preferred`, `available`, `unavailable_fallback_explicit`,
  or `unavailable_fallback_implicit`. The last value is rejected at build time:
  where passkeys are unavailable, the fallback posture must be explicit.
- `local_continuity` — the local work that stays usable through the event
  (editing, files, history, local-only workflows), each preserved with a
  human-readable statement.
- `active_condition` — an optional live degraded condition (see below). When
  absent, the event is calm and managed identity is healthy.

## Degraded conditions and drills

A live degraded condition (`AuthCondition`) or a standalone drill (`AuthDrill`)
names a `DrillKind`:

- `passkey_unavailable`
- `browser_handoff_failure`
- `offline_identity`
- `policy_forced_sign_out`
- `deprovision_on_active_local_work`

A condition pauses one or more **managed** capabilities (`hosted_ai`,
`managed_sync`, `marketplace_publish`, `companion_control`, `policy_distribution`,
`org_collab`) and names the local capabilities that remain usable. It carries an
explicit `fallback_posture`, a `keyboard_complete_fallback` flag, and a
`local_work_threatened` flag that must be `false` unless a named
`governed_policy_exception_ref` authorizes the impact. The standalone drills are
exercised as `security`, `accessibility`, and `recovery` lenses, and each keeps
local work preserved, labeled, keyboard-complete, and recoverable.

## Credential storage

Refresh credentials and delegated handles are accounted for by
`CredentialStorageRow`. Each protected credential class (`refresh_token`,
`delegated_handle`, `session_broker`) lives in an OS-backed or approved
enterprise store (`os_keychain`, `enterprise_store`, `session_broker_memory`) and
is excluded from portable-state, sync, and support export. The record carries
storage metadata only — never the secret bodies.

## Fail-closed gate

`M5AuthAndRecovery::build` rejects a record that would:

- omit a required managed event kind, or fail to represent desktop, companion,
  and browser-adjacent surfaces;
- present an event without a provider/owner disclosure;
- encode a local-work threat without a named governed policy row;
- leave a passkey-unavailable event without an explicit fallback posture;
- require embedded password-first or CAPTCHA-only recovery on a `stable` profile;
- carry a degraded condition without a keyboard-complete fallback, or one that
  pauses no managed capability;
- let a refresh token, delegated handle, or session-broker reference escape
  export exclusion or sit in an unprotected store;
- ship a drill that loses local authority, label, or keyboard completeness, or
  omit a required drill kind or review category;
- drop a required parity surface.

The build derives a fail-closed `trust_qualification`:

- `local_first_managed_safe` — managed identity is calm; every event is healthy
  and every pillar holds (`effective_continuity_ceiling = local_first_full`).
- `narrowed_managed_degraded` — at least one event has a live degraded condition;
  a managed capability is paused but all local work stays usable
  (`effective_continuity_ceiling = managed_narrowed_local_intact`).
- `unsupported` — a structural pillar failed.

## Surface parity

`SurfaceTruthRow` records that the desktop shell, CLI inspect, docs/help, and
support exports each consume the shared record and show the provider disclosure,
the paused managed capabilities, the local-work continuity, the fallback posture,
and the drills. Support exports reconstruct the same auth-boundary and recovery
state the user sees, carrying typed states and opaque refs only.

## Consuming surfaces

Release-center, Help/About, support exports, admin docs, and diagnostics surfaces
ingest this record instead of cloning auth status text. They must not invent a
local `is_signed_in` boolean, collapse a paused managed capability into "signed
out", present an embedded credential collector as a silent fallback, or imply
that loss of managed identity means loss of unsaved local work.
