# M5 store-lock and external-root recovery

Generated from the seeded report in
[`crate::m5_store_lock_and_external_root_recovery`](../../crates/aureline-auth/src/m5_store_lock_and_external_root_recovery/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- report-md > \
  artifacts/platform/m5-store-lock-and-external-root-recovery.md
```

- Report id: `auth:m5_store_lock_and_external_root_recovery:report:v1`
- Source schema ref: `schemas/platform/m5-store-lock-and-missing-root.schema.json`
- Claimed platforms: `macos`, `windows`, `linux`
- Registered states: `7`
- Marketed states: `7`
- Active degradations: `6`
- Blocking findings: `0`
- Narrowable marketed states: `0`
- Status: **clean**
- Generated at: `2026-06-16T00:00:00Z`

## Cross-links

| Upstream packet | Ref |
| --------------- | --- |
| `credential_store_ref` | `schemas/auth/credential_state.schema.json` |
| `trust_store_ref` | `artifacts/platform/native_trust_integration_matrix.yaml` |
| `filesystem_identity_ref` | `schemas/workspace/canonical_identity_lineage.schema.json` |
| `deferred_intent_ref` | `docs/m5/durable-progress-and-reopen.md` |
| `auth_recovery_ref` | `artifacts/platform/m5-auth-callback-and-deep-link.md` |
| `help_about_ref` | `docs/help/store_lock_and_external_root_recovery.md` |

## Per-incident-class coverage

| Incident class | Registered states |
| -------------- | ----------------: |
| Credential store locked | 1 |
| Credential store unavailable | 1 |
| Trust-store drift | 1 |
| Removable volume missing | 1 |
| Network share missing | 1 |
| External root missing | 1 |
| Root returned | 1 |

## Per-resource coverage

| Resource | States | Local continuity preserved |
| -------- | -----: | -------------------------: |
| Credential store | 2 | 2 |
| Trust store | 1 | 1 |
| Removable volume | 1 | 1 |
| Network share | 2 | 2 |
| External root | 1 | 1 |

## Recovery index

| State | Incident | Resource | Degraded state | Resume posture | Recovery actions |
| ----- | -------- | -------- | -------------- | -------------- | ---------------: |
| `state:credential_store.locked` | `credential_store_locked` | `credential_store` | `store_locked` | `explicit_resume_required` | 3 |
| `state:credential_store.unavailable` | `credential_store_unavailable` | `credential_store` | `store_unavailable` | `explicit_resume_required` | 2 |
| `state:external_root.missing` | `external_root_missing` | `external_root` | `root_missing` | `explicit_resume_required` | 3 |
| `state:network_share.missing` | `network_share_missing` | `network_share` | `root_missing` | `explicit_resume_required` | 4 |
| `state:network_share.returned` | `root_returned` | `network_share` | `root_returned` | `explicit_resume_required` | 2 |
| `state:removable_volume.missing` | `removable_volume_missing` | `removable_volume` | `root_missing` | `explicit_resume_required` | 4 |
| `state:trust_store.drift` | `trust_store_drift` | `trust_store` | `trust_store_drifted` | `explicit_resume_required` | 2 |

## Findings summary

| Class | Count |
| ----- | ----: |
| _(none)_ | 0 |

## Per-state rows

### `state:credential_store.locked` (credential_store_locked)

- Descriptor revision: `state:credential_store.locked:rev:2026.06.01-01`
- Resource: `credential_store` (degraded state `store_locked`)
- Last-seen identity: `identity:credential_store.os_keychain_login`
- Placeholder: `placeholder:state:credential_store.locked`
- Paused: `provider_authentication`, `managed_sync`
- Local-only: `local_editing`, `local_history`, `local_export`
- Unsaved local state: `preserved_in_place`
- Local continuity preserved: `yes`
- Recovery actions: `unlock_store`, `retry_after_unlock`, `repair_store`
- Repair guidance: `repair:credential_store.unlock`
- Implies plaintext fallback: `no`
- Resume posture: `explicit_resume_required` (silent on recovery: `no`)
- Protected continuations:
  - `continuation:state:credential_store.locked:queued_job` (`queued_job`) -> `explicit_resume_required`
  - `continuation:state:credential_store.locked:remembered_decision` (`remembered_decision`) -> `held_for_review`
- Active profile owner: `profile-owner:state:credential_store.locked`
- Trust checkpoint: `trust:state:credential_store.locked:profile_policy`
- Canonical command: `cmd:identity.store.unlock`
- Surface parity: `desktop`, `cli_headless`, `support`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:store_lock_and_external_root:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: The OS credential store is locked: provider authentication and managed sync are paused, but local editing, local history, and local export are unaffected. Recovery is to unlock the store; a queued provider job and a remembered store-preference decision are held for explicit resume, never re-run automatically, and no secret is ever written to plaintext.
- Degraded-state vocabulary:
  - Your secure store is locked
  - Unlock it to use saved credentials again
  - Your local work is unaffected and nothing is stored in plain text

Findings: none.

### `state:credential_store.unavailable` (credential_store_unavailable)

- Descriptor revision: `state:credential_store.unavailable:rev:2026.06.01-01`
- Resource: `credential_store` (degraded state `store_unavailable`)
- Last-seen identity: `identity:credential_store.os_secret_service`
- Placeholder: `placeholder:state:credential_store.unavailable`
- Paused: `provider_authentication`, `signed_operation`
- Local-only: `local_editing`, `offline_core_tools`, `local_export`
- Unsaved local state: `preserved_in_place`
- Local continuity preserved: `yes`
- Recovery actions: `repair_store`, `retry_after_unlock`
- Repair guidance: `repair:credential_store.reconnect_backend`
- Implies plaintext fallback: `no`
- Resume posture: `explicit_resume_required` (silent on recovery: `no`)
- Protected continuations:
  - `continuation:state:credential_store.unavailable:running_session` (`running_session`) -> `explicit_resume_required`
- Active profile owner: `profile-owner:state:credential_store.unavailable`
- Trust checkpoint: `trust:state:credential_store.unavailable:profile_policy`
- Canonical command: `cmd:identity.store.repair`
- Surface parity: `desktop`, `cli_headless`, `support`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:store_lock_and_external_root:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: The OS credential store backend is unreachable: provider authentication and signing are paused while offline core tooling, local editing, and local export continue. Recovery is to repair the backend handle; a running provider session is held for explicit resume and never silently widened, and no plaintext-secret fallback is offered.
- Degraded-state vocabulary:
  - Your secure store is unavailable right now
  - Repair the connection to use saved credentials again
  - Local editing keeps working and no secret is exposed in plain text

Findings: none.

### `state:external_root.missing` (external_root_missing)

- Descriptor revision: `state:external_root.missing:rev:2026.06.01-01`
- Resource: `external_root` (degraded state `root_missing`)
- Last-seen identity: `identity:external_root.last_seen_path_alias`
- Placeholder: `placeholder:state:external_root.missing`
- Paused: `external_root_access`
- Local-only: `local_editing`, `cached_context_browse`, `local_history`
- Unsaved local state: `preserved_pending_recovery`
- Local continuity preserved: `yes`
- Recovery actions: `locate_root`, `open_cached_context`, `close_placeholder`
- Repair guidance: `repair:external_root.locate`
- Implies plaintext fallback: `no`
- Resume posture: `explicit_resume_required` (silent on recovery: `no`)
- Protected continuations:
  - `continuation:state:external_root.missing:running_session` (`running_session`) -> `held_for_review`
- Active profile owner: `profile-owner:state:external_root.missing`
- Trust checkpoint: `trust:state:external_root.missing:profile_policy`
- Canonical command: `cmd:workspace.root.recover`
- Surface parity: `desktop`, `cli_headless`, `support`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:store_lock_and_external_root:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: An external root went missing: access to its files is paused, the placeholder names the last-seen path alias, unsaved edits are preserved pending recovery, and local history stays available. Recovery offers Locate, Open cached context, and Close; a session bound to the root is held for review and never silently rejoined when the root reappears.
- Degraded-state vocabulary:
  - This external location is no longer available
  - Locate it, open the cached copy, or close this placeholder
  - Your local history and unsaved work are intact

Findings: none.

### `state:network_share.missing` (network_share_missing)

- Descriptor revision: `state:network_share.missing:rev:2026.06.01-01`
- Resource: `network_share` (degraded state `root_missing`)
- Last-seen identity: `identity:network_share.last_seen_mount`
- Placeholder: `placeholder:state:network_share.missing`
- Paused: `external_root_access`, `managed_sync`
- Local-only: `local_editing`, `cached_context_browse`
- Unsaved local state: `preserved_pending_recovery`
- Local continuity preserved: `yes`
- Recovery actions: `reconnect_network_share`, `locate_root`, `open_cached_context`, `close_placeholder`
- Repair guidance: `repair:network_share.reconnect`
- Implies plaintext fallback: `no`
- Resume posture: `explicit_resume_required` (silent on recovery: `no`)
- Protected continuations:
  - `continuation:state:network_share.missing:queued_job` (`queued_job`) -> `explicit_resume_required`
- Active profile owner: `profile-owner:state:network_share.missing`
- Trust checkpoint: `trust:state:network_share.missing:profile_policy`
- Canonical command: `cmd:workspace.root.recover`
- Surface parity: `desktop`, `cli_headless`, `support`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:store_lock_and_external_root:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A network share disconnected: access to files on that mount is paused while local editing and cached-context browsing continue. The placeholder names the last-seen mount and preserves unsaved edits pending recovery; recovery offers Reconnect, Locate, Open cached context, and Close, and a queued write is held for explicit resume rather than replayed when the share returns.
- Degraded-state vocabulary:
  - The network share is disconnected
  - Reconnect it, open the cached copy, or close this placeholder
  - Your unsaved work is preserved and not silently re-sent

Findings: none.

### `state:network_share.returned` (root_returned)

- Descriptor revision: `state:network_share.returned:rev:2026.06.01-01`
- Resource: `network_share` (degraded state `root_returned`)
- Last-seen identity: `identity:network_share.returned_mount`
- Placeholder: `placeholder:state:network_share.returned`
- Paused: _(nothing paused)_
- Local-only: `local_editing`, `local_history`
- Unsaved local state: `preserved_pending_recovery`
- Local continuity preserved: `yes`
- Recovery actions: `confirm_explicit_resume`, `open_cached_context`
- Repair guidance: `repair:network_share.confirm_resume`
- Implies plaintext fallback: `no`
- Resume posture: `explicit_resume_required` (silent on recovery: `no`)
- Protected continuations:
  - `continuation:state:network_share.returned:queued_job` (`queued_job`) -> `explicit_resume_required`
  - `continuation:state:network_share.returned:running_session` (`running_session`) -> `explicit_resume_required`
  - `continuation:state:network_share.returned:remembered_decision` (`remembered_decision`) -> `held_for_review`
- Active profile owner: `profile-owner:state:network_share.returned`
- Trust checkpoint: `trust:state:network_share.returned:profile_policy`
- Canonical command: `cmd:workspace.root.confirm_resume`
- Surface parity: `desktop`, `cli_headless`, `support`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:store_lock_and_external_root:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A previously missing network share has returned, but nothing resumes on its own: the held write, the bound session, and the remembered decision all require explicit confirmation before they continue. The placeholder is reconciled to the returned mount and the user confirms what to resume, so a returned root never auto-rejoins a session or replays a deferred write.
- Degraded-state vocabulary:
  - The network share is back
  - Review what was waiting before it continues
  - Nothing was resumed automatically

Findings: none.

### `state:removable_volume.missing` (removable_volume_missing)

- Descriptor revision: `state:removable_volume.missing:rev:2026.06.01-01`
- Resource: `removable_volume` (degraded state `root_missing`)
- Last-seen identity: `identity:removable_volume.last_seen_label`
- Placeholder: `placeholder:state:removable_volume.missing`
- Paused: `external_root_access`
- Local-only: `local_editing`, `cached_context_browse`, `local_export`
- Unsaved local state: `preserved_pending_recovery`
- Local continuity preserved: `yes`
- Recovery actions: `remount_volume`, `locate_root`, `open_cached_context`, `close_placeholder`
- Repair guidance: `repair:removable_volume.remount`
- Implies plaintext fallback: `no`
- Resume posture: `explicit_resume_required` (silent on recovery: `no`)
- Protected continuations:
  - `continuation:state:removable_volume.missing:queued_job` (`queued_job`) -> `explicit_resume_required`
- Active profile owner: `profile-owner:state:removable_volume.missing`
- Trust checkpoint: `trust:state:removable_volume.missing:profile_policy`
- Canonical command: `cmd:workspace.root.recover`
- Surface parity: `desktop`, `cli_headless`, `support`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:store_lock_and_external_root:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A removable volume was ejected: access to files on that root is paused, but the placeholder names the last-seen volume label, unsaved local edits are preserved pending recovery, and cached context stays browsable. Recovery offers Remount, Locate, Open cached context, and Close; a write queued to the volume is held for explicit resume, never replayed automatically when the volume returns.
- Degraded-state vocabulary:
  - The removable volume is no longer connected
  - Locate it, open the cached copy, or close this placeholder
  - Your unsaved work is kept and nothing is rewritten without you

Findings: none.

### `state:trust_store.drift` (trust_store_drift)

- Descriptor revision: `state:trust_store.drift:rev:2026.06.01-01`
- Resource: `trust_store` (degraded state `trust_store_drifted`)
- Last-seen identity: `identity:trust_store.system_roots`
- Placeholder: `placeholder:state:trust_store.drift`
- Paused: `certificate_validation`, `managed_sync`
- Local-only: `local_editing`, `local_history`
- Unsaved local state: `preserved_in_place`
- Local continuity preserved: `yes`
- Recovery actions: `review_trust_change`, `re_evaluate_trust`
- Repair guidance: `repair:trust_store.review_drift`
- Implies plaintext fallback: `no`
- Resume posture: `explicit_resume_required` (silent on recovery: `no`)
- Protected continuations:
  - `continuation:state:trust_store.drift:remembered_decision` (`remembered_decision`) -> `held_for_review`
- Active profile owner: `profile-owner:state:trust_store.drift`
- Trust checkpoint: `trust:state:trust_store.drift:profile_policy`
- Canonical command: `cmd:identity.trust.review`
- Surface parity: `desktop`, `cli_headless`, `support`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:store_lock_and_external_root:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: The trust store drifted from the anchors a remembered decision was made against: certificate validation and managed sync are paused while local editing and local history continue. Recovery is to review the change and re-evaluate trust; the remembered trust acceptance is held for review and never silently re-applied to the new anchors.
- Degraded-state vocabulary:
  - The trust store changed since you last accepted it
  - Review the change before connections resume
  - Your earlier trust decision is held for review, not re-applied automatically

Findings: none.

## Verification

```sh
cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- validate
cargo test -p aureline-auth --test m5_store_lock_and_external_root_recovery_fixtures
python3 tools/ci/m5/store_lock_and_external_root_check.py
```
