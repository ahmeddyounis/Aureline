# M5 Sync-and-Device Review

**Doc ref:** `docs/settings/m5/m5-sync-and-device-review.md`  
**Contract ref:** `settings:m5_sync_and_device_review:v1`  
**Schema version:** 1

## Overview

This document defines the shared product truth for how the new M5 feature
families participate in settings sync. The canonical record is
`M5SyncAndDeviceReview` in `crates/aureline-settings/src/m5_sync_and_device_review/`.
Desktop settings, CLI inspect, docs/help, and support exports consume the same
record so they describe sync, conflicts, and device participation identically
instead of cloning sync status text.

The record replaces opaque, last-writer-wins sync state with field-aware scope
bundles. Local durable state is always authoritative; when sync transport,
encryption, or policy degrades, the affected rows are held local-authoritative
and visibly labeled rather than overwritten, and the user is never misled about
what still works locally.

## Scope bundles

Every M5 feature family ships one `SyncScopeBundle`:

- `notebooks`
- `data_api`
- `profiler`
- `extension_bundles`
- `companion`

Each bundle is a typed packet rather than a blob. It carries:

- `bundle_schema_version` — the payload schema version, so a peer can detect a
  shape mismatch instead of merging incompatible fields.
- `redaction_mode` — `none`, `redact_secrets`, `machine_local_excluded`, or
  `fully_local_only`. Secret-bearing and machine-only fields never leave the
  device in the clear.
- `source_device_ref` / `source_profile_ref` — opaque refs to the device and
  profile that produced the bundle (never hostnames or serials).
- `revisions` — the `local_revision_ref`, an optional `remote_revision_ref`, and
  an optional `last_common_revision_ref` used to classify divergence.
- `capability_dependencies` — capabilities the bundle needs to apply, each marked
  `present_locally` and whether its absence `narrows_apply`.
- `local_authoritative` — always `true`; a bundle can never claim a remote
  payload outranks the local source of truth.
- `remote_synced` — whether local and remote are currently in clean sync.

## Field-aware conflicts

Divergences are classed per field, never resolved by last-writer-wins. Each
`FieldConflict` names a `field_path`, a `class`, a `disposition`, the local (and
optional remote) value refs, an optional `widens_trust` marker, whether it
`requires_explicit_review`, and a human-readable `detail`.

Conflict classes:

- `same_key_divergent` — both sides set the same key to different values.
- `policy_locked` — an admin policy locks the key locally; the remote value
  cannot win.
- `missing_capability` — the remote value needs a capability absent on this
  device.
- `machine_only` — the key is machine-only and never reconciles across devices.
- `delete_versus_modify` — one side deleted the key while the other modified it.
- `stale_remote` — the remote revision predates the local common ancestor.

Dispositions never include a last-writer-wins variant. A remote value can only
land through `remote_applied_after_review`; otherwise the local value is kept
(`local_authoritative_kept`), held (`awaiting_field_review`), or blocked
(`remote_apply_blocked`).

## Trust never widens silently

A `FieldConflict` whose remote value would raise a trust boundary
(`trust_elevation`), grant extension permissions (`extension_permission`), open
AI egress (`ai_egress`), or widen a managed entitlement (`managed_entitlement`)
carries a `widens_trust` marker. Such a field must `requires_explicit_review` and
may never carry an applied disposition — the build gate rejects any record that
would let synced state widen trust on another device.

## Device actions

The record exposes the full participation-control catalog as durable audit
records (`DeviceActionRecord`): `pause`, `resume`, `revoke`, `forget`, and
`rotate`. Each names the target device and class, the `audit_ref`, the actor, the
`participation_after` state (`active`, `paused`, `revoked`, `forgotten`,
`rotating`), and `local_state_intact` — always `true`, because no device action
deletes local durable state.

## Degraded-state drills

Every record demonstrates the degraded states a claimed sync row must survive:
`offline`, `stale_remote`, `blocked_sync_apply`, `e2ee_unavailable`, and
`local_only_fallback`. Each `SyncDrill` keeps `local_authoritative` and
`local_state_labeled` true, names the `transport_state` it models, the
`expected_signal` the user sees, and the `recovery_path` back to clean sync.

## Trust qualification

The record derives a fail-closed verdict:

- Each bundle resolves to a `bundle_sync_trust` of `synced` (clean parity),
  `local_authoritative` (degraded or a field pending review), or `review_blocked`
  (a trust-widening, policy-locked, or blocked field needs review).
- The record publishes the weakest bundle trust as its
  `effective_trust_ceiling`.
- `claim_class` is `fully_synced` only when every structural pillar holds and
  every bundle is `synced`; otherwise it is `narrowed_local_authoritative`, or
  `unsupported` when a structural pillar fails.

## Surfaces

`SurfaceTruthRow` proves every consumer renders the same record:
`desktop_settings`, `cli_inspect`, `docs_help`, and `support_export`. Each shows
the scope bundles, field conflicts, device actions, and local-only fallback.

## Guardrails

Records carry typed states and opaque refs only — no secrets, raw provider
payloads, hostnames, serials, or workspace contents. Synced M5 state must not
silently widen trust, extension permissions, AI egress, or managed entitlements,
and no M5 artifact class claims full sync parity without a schema-backed scope
bundle and a field-aware conflict packet.
