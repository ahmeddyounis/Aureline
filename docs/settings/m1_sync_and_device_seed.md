# Device-registry and settings-sync seed (M1 conflict-review vocabulary)

This document is the reviewer-facing landing page for the canonical
M1 **device-registry and settings-sync seed**: the typed model that
later UI, support, migration, and review surfaces consume so device
participation state, sync session state, conflict-review class,
offered resolution paths, scope-broadening verdict, non-widening
posture, local-vs-managed ownership, and per-data-class portability
do not need to be reinvented at every surface.

This seed is not a sync runtime. It freezes the **vocabulary** and
the **typed profile rows** so the eventual managed sync service, the
profile importer / exporter, the CLI `settings sync` command, the
support-bundle exporter, the mutation-journal renderer, the migration
center, and the conflict review surface all point to the same
contract.

If this document and the row / envelope schemas disagree, the schemas
win and this document must be updated in the same change. The
upstream optional-sync contract at
[`docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md)
freezes the wire-level packet shapes (device record, scope bundle,
session envelope, conflict packet); the M1 seed projects against that
contract and freezes the higher-level **profile rows** every M1
surface reads.

## Canonical sources

- Seed YAML: [`artifacts/settings/m1_device_registry_and_sync_seed.yaml`](../../artifacts/settings/m1_device_registry_and_sync_seed.yaml).
- Envelope schema: [`schemas/settings/device_registry.schema.json`](../../schemas/settings/device_registry.schema.json).
- Row schema: [`schemas/settings/settings_sync_state.schema.json`](../../schemas/settings/settings_sync_state.schema.json).
- Upstream optional-sync contract:
  [`docs/settings/sync_and_device_registry_seed.md`](./sync_and_device_registry_seed.md).
- Companion settings schema-registry seed:
  [`docs/settings/schema_registry_seed.md`](./schema_registry_seed.md).
- Validation lane: [`tests/governance/m1_device_registry_and_sync_seed_lane/run_m1_device_registry_and_sync_seed_lane.py`](../../tests/governance/m1_device_registry_and_sync_seed_lane/run_m1_device_registry_and_sync_seed_lane.py).
- Proof packet:
  [`artifacts/milestones/m1/proof_packets/device_registry_and_sync_seed.md`](../../artifacts/milestones/m1/proof_packets/device_registry_and_sync_seed.md).

## Why a typed device-and-sync seed now

The upstream sync-and-device-registry contract freezes the wire
shapes (device record, scope bundle, session envelope, conflict
packet). Without a higher-level **profile-row seed** every M1 surface
that wants to *render* sync state would invent its own status
taxonomy: the settings UI would label "paused" three different ways,
the CLI would print "conflict" without a vocabulary the user could
search, the support exporter would render "session degraded" with no
typed reason, and the conflict review surface would fall back to
document-level last-writer-wins because the offered resolution paths
were not enumerated anywhere.

This seed closes that gap before any sync surface ships:

- The conflict-review vocabulary is **closed**. Surfaces consume it
  verbatim; widening requires a new decision row.
- The offered-resolution-path set is **closed**. `keep_local`,
  `keep_synced`, `merge_preview`, `rollback_friendly_review`, and
  `decline` are the only paths a surface may offer.
- The non-widening posture, scope-broadening verdict, and
  local-ownership marker are **typed and reusable** so later import /
  sync surfaces do not invent their own widening checks.
- The portability model (portable / machine_local / excluded /
  policy_owned) is **declared per-data-class**, so machine-local
  exclusions and policy-owned ownership stay distinct from
  local-authoritative state that could in principle ride sync.

## Closed vocabularies

The seed envelope freezes these vocabularies. The row schema's
`$defs` is the canonical source; the envelope vocabulary MUST agree.

### Device participation state

| Token | Meaning |
| --- | --- |
| `active` | Normal participating state. |
| `paused` | User- or admin-initiated freeze; lineage cursor retained; emits no bundles, consumes no arrivals. |
| `revoked` | Durable refusal; rejects all sync traffic; device record retained for audit. |
| `forgotten` | Terminal state after revoke retention; only `device_id` and revocation stamps remain. |

### Sync session state

| Token | Meaning |
| --- | --- |
| `open` | Live, non-degraded session. |
| `local_authoritative_degraded` | Floor whenever transport is unavailable, payload stale, capability missing, or policy blocks. Local values remain authoritative. |
| `refused` | Durable refusal: revoked device, encryption failure, policy block, schema incompatibility. |
| `paused` | User- or admin-initiated session pause. |
| `manual_continuity_in_progress` | Typed export/import path users carry by hand. |

### Conflict-review class (the M1 conflict-review vocabulary)

Every conflict-aware surface presents one of these tokens verbatim:

| Token | Meaning |
| --- | --- |
| `no_conflict` | No divergence detected. |
| `value_equal_no_op` | Local and synced values are equal; no write happens. |
| `keep_local_pending_review` | User opted to retain the local value; pending acknowledgement. |
| `keep_synced_pending_review` | User opted to adopt the synced value; pending acknowledgement. |
| `merge_preview_pending_review` | A field-aware merge preview is offered; pending acknowledgement. |
| `rollback_friendly_review_pending` | Apply requires an ADR-0006 rollback checkpoint or an ADR-0007 approval. |
| `scope_broadening_refusal` | Synced entry would widen trust / AI egress / network egress / extension permissions / managed entitlement; refused on apply. |
| `allowed_scope_mismatch` | Synced entry targets a non-syncable scope (`machine_specific`, `workspace`, etc); refused. |
| `stale_payload` | Bundle epoch is not strictly greater than the last accepted epoch from the same device. |
| `encryption_failure` | Transport-level encryption check failed or keys are missing. |
| `policy_block` | Admin policy prohibits optional sync for this identity mode or policy epoch. |
| `device_revoked` | Producer device is revoked; arrivals refused. |
| `device_paused` | Producer device is paused; arrivals not consumed as values. |
| `identity_mode_mismatch` | Producer and receiver `identity_mode` disagree in a way policy forbids. |
| `schema_version_incompatible` | Bundle `schema_version` cannot be migrated in place. |
| `declined` | Terminal refusal of the packet by the user. |

### Offered resolution path

| Token | When offered |
| --- | --- |
| `keep_local` | Always (so the user always has a safe out). |
| `keep_synced` | Only when the entry did not widen trust and does not violate allowed-scope rules. |
| `merge_preview` | Only when the delta is merge-safe (`array_append_only`, `object_field_add`, `redacted_structural`). |
| `rollback_friendly_review` | Default whenever the setting requires a rollback checkpoint or an approval ticket. |
| `decline` | Always available. |

### Resolution state lifecycle

`pending` → (`previewed`) → (`acknowledged`) → `resolved` /
`declined` / `expired` / `withdrawn`. `not_applicable` covers
`no_conflict` and `value_equal_no_op` rows that never enter the
lifecycle.

### Scope-broadening verdict

| Token | Meaning |
| --- | --- |
| `would_not_widen_trust` | Normal case; the entry passes the widening check. |
| `would_widen_trust_refused` | Entry would widen trust; refused on apply (paired with `conflict_review_class = scope_broadening_refusal`). |
| `narrowing_only` | Entry only narrows the local value; flows through write-intent / preview / apply. |
| `not_applicable` | Row does not exchange entries (paused / revoked / forgotten). |

### Non-widening posture

| Token | Meaning |
| --- | --- |
| `non_widening_affirmed` | Producer set `non_widening_affirmed = true` on the outgoing bundle. |
| `non_widening_not_required` | Row does not emit bundles (paused / revoked / forgotten or manual-continuity-in-progress with no bundle on the wire). |
| `non_widening_affirmation_missing` | Invariant-violation state used by the failure drill; protected rows MUST NOT publish it. |

### Local-vs-managed ownership marker

| Token | Meaning |
| --- | --- |
| `local_authoritative` | Device retains final say over its local values regardless of session state. |
| `managed_authoritative` | A managed sync service (M1: not built) would own the row when it lands. |
| `policy_authoritative` | Admin policy owns the row; sync cannot override. |
| `machine_local_excluded` | Data class is machine-local-only and never crosses sync (distinct from `local_authoritative` state that could in principle ride sync). |

### Data-portability class (per data class)

| Token | Meaning |
| --- | --- |
| `portable` | Rides sync subject to scope-broadening and allowed-scope rules. |
| `machine_local` | Bound to one device; never syncs. |
| `excluded` | On the explicit omitted-classes denylist (secret bytes, credentials, trust grants, etc). |
| `policy_owned` | Owned by admin policy; cannot be moved by user sync. |

## Seeded sync-state profile rows

The seed lands seven typed rows covering the protected M1 surfaces:

| `sync_state_profile_id` | Participation | Session | Conflict | Resolution paths |
| --- | --- | --- | --- | --- |
| `active.local_authoritative.no_conflict` | `active` | `open` | `no_conflict` | `keep_local` |
| `active.scope_broadening_refusal` | `active` | `open` | `scope_broadening_refusal` | `keep_local`, `decline` |
| `active.rollback_friendly_review_pending` | `active` | `open` | `rollback_friendly_review_pending` | `keep_local`, `rollback_friendly_review`, `decline` |
| `paused.local_authoritative_degraded` | `paused` | `local_authoritative_degraded` | `device_paused` | `keep_local` |
| `revoked.refused` | `revoked` | `refused` | `device_revoked` | `keep_local`, `decline` |
| `manual_continuity.keep_synced_pending_review` | `active` | `manual_continuity_in_progress` | `keep_synced_pending_review` | `keep_local`, `keep_synced`, `decline` |
| `stale_payload.local_authoritative_degraded` | `active` | `local_authoritative_degraded` | `stale_payload` | `keep_local`, `decline` |

## Failure drills

Every row carries a named failure drill the validation lane
reproduces under
`--force-drill <sync_state_profile_id>:<drill_id>`:

| Row | Drill | Expected check id |
| --- | --- | --- |
| `active.local_authoritative.no_conflict` | `device_sync_state_drill.active_local_authoritative_non_widening_affirmation_dropped` | `settings_sync_state.non_widening_affirmation_missing_blocked_on_active_row` |
| `active.scope_broadening_refusal` | `device_sync_state_drill.active_scope_broadening_verdict_relaxed_to_widen_trust` | `settings_sync_state.scope_broadening_refusal_requires_would_widen_trust_refused_verdict` |
| `active.rollback_friendly_review_pending` | `device_sync_state_drill.active_keep_local_resolution_path_dropped` | `settings_sync_state.keep_local_resolution_path_required_for_active_conflict` |
| `paused.local_authoritative_degraded` | `device_sync_state_drill.paused_session_state_widened_to_open` | `settings_sync_state.paused_device_session_state_must_be_degraded_or_paused` |
| `revoked.refused` | `device_sync_state_drill.revoked_session_state_widened_to_open` | `settings_sync_state.revoked_device_session_state_must_be_refused` |
| `manual_continuity.keep_synced_pending_review` | `device_sync_state_drill.manual_continuity_data_class_drifted_from_excluded_to_portable` | `settings_sync_state.excluded_data_class_widening_blocked` |
| `stale_payload.local_authoritative_degraded` | `device_sync_state_drill.stale_payload_conflict_review_class_drifted_to_no_conflict` | `settings_sync_state.stale_payload_must_not_drift_to_no_conflict` |

## Consumer checklist

Before a sync-aware surface ships, it confirms:

1. It consumes one and only one `conflict_review_class` from the
   closed vocabulary above. Surfaces MUST NOT invent their own
   conflict status names.
2. It honours the offered-resolution-path set on the row; in
   particular, every conflict row that lives in
   `active.{...}` MUST always expose `keep_local`.
3. It quotes the `non_widening_posture_class`,
   `scope_broadening_verdict_class`, and
   `local_ownership_marker_class` verbatim.
4. It reads `data_class_portabilities[]` and renders
   `machine_local` / `excluded` / `policy_owned` rows distinctly
   from `portable` rows; a surface that quietly widens an
   `excluded` data class is a bug.
5. It reads the device-participation state and the sync-session
   state independently: a paused device's session is
   `local_authoritative_degraded` or `paused`, NEVER `open`.

## Change management

- Adding a new participation state, session state, conflict-review
  class, resolution path, resolution state, scope-broadening verdict,
  non-widening posture, local-ownership marker, or data-portability
  class is additive-minor: bump the seed's `schema_version`, extend
  the row schema's `$defs`, extend the envelope vocabularies, and
  refresh the validation capture.
- Repurposing any existing value is breaking and requires a new
  decision row.
- Removing a row is breaking. Narrow by editing the row, not by
  deletion.
- Publishing a new release of the seed MUST refresh the validation
  capture; the refresh trigger and freshness rule are pinned in the
  proof packet.
