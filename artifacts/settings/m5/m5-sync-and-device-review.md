# M5 Sync-and-Device Review

**Artifact ref:** `artifacts/settings/m5/m5-sync-and-device-review.md`  
**Contract ref:** `settings:m5_sync_and_device_review:v1`  
**Schema version:** 1  
**As of:** 2026-06-12

## Purpose

This artifact certifies that the new M5 feature families participate in settings
sync as field-aware scope bundles with explicit device controls and an honest
local-only fallback, instead of opaque last-writer-wins state. Local durable
state stays authoritative; transport failures, stale payloads, missing
capabilities, and policy blocks never overwrite it or hide what still works
locally. A remote value that would widen trust, extension permissions, AI egress,
or a managed entitlement can never be applied silently.

## Certification Scope

The canonical record binds:

1. Five M5 feature families — `notebooks`, `data_api`, `profiler`,
   `extension_bundles`, and `companion` — each shipping one schema-backed scope
   bundle with a `bundle_schema_version`, `redaction_mode`, source
   device/profile, capability dependencies, and local/remote revision sets.
2. Field-aware conflicts classed per key (`same_key_divergent`, `policy_locked`,
   `missing_capability`, `machine_only`, `delete_versus_modify`,
   `stale_remote`) with a disposition that never auto-applies a remote value
   (`local_authoritative_kept`, `awaiting_field_review`, `remote_apply_blocked`,
   `remote_applied_after_review`).
3. A device-action audit trail covering `pause`, `resume`, `revoke`, `forget`,
   and `rotate`, each with a durable audit ref, the resulting participation
   state, and the explicit statement that local durable state remains intact.
4. Degraded-state drills for `offline`, `stale_remote`, `blocked_sync_apply`,
   `e2ee_unavailable`, and `local_only_fallback`, each keeping local
   authoritative and visibly labeled with a recovery path.
5. Surface-parity rows proving desktop settings, CLI inspect, docs/help, and
   support exports render the same record.
6. A fail-closed trust gate: a record cannot be built that claims remote
   authority over local state, applies a trust-widening field without review,
   applies a policy-locked field from the remote side, records a device action
   without an audit ref or that wipes local state, or drills a degraded state
   that hides the local-only fallback.

## Canonical Paths

- Typed model: `crates/aureline-settings/src/m5_sync_and_device_review/`
- Schema: `schemas/settings/m5/m5-sync-and-device-review.schema.json`
- Fixtures: `fixtures/settings/m5/m5-sync-and-device-review/`
- Companion doc: `docs/settings/m5/m5-sync-and-device-review.md`
- Emitter: `aureline_settings_m5_sync_and_device_review`

## Corpus Outcomes

| Scenario | Claim | Trust ceiling |
| --- | --- | --- |
| `fully_synced_baseline` | `fully_synced` | `synced` |
| `same_key_divergent_drill` | `narrowed_local_authoritative` | `local_authoritative` |
| `policy_locked_drill` | `narrowed_local_authoritative` | `review_blocked` |
| `missing_capability_drill` | `narrowed_local_authoritative` | `local_authoritative` |
| `machine_only_drill` | `narrowed_local_authoritative` | `local_authoritative` |
| `delete_versus_modify_drill` | `narrowed_local_authoritative` | `local_authoritative` |
| `stale_remote_drill` | `narrowed_local_authoritative` | `local_authoritative` |
| `trust_widening_blocked_drill` | `narrowed_local_authoritative` | `review_blocked` |

## Guardrails

The record carries typed states and opaque refs only. No secrets, raw provider
payloads, credential bodies, hostnames, serials, or workspace contents are
serialized. Synced M5 state must not silently widen trust, extension
permissions, AI egress, or managed entitlements on another device, and no M5
artifact class claims full sync parity without a schema-backed scope bundle and
a field-aware conflict packet.
