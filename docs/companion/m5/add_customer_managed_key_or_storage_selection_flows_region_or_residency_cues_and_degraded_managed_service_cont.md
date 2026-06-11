# Customer-Managed-Key and Storage Selection Flows, Region/Residency Cues, and Degraded Managed-Service Continuity

This document is the human-readable contract for the residency and offboarding
depth lane: the **key custody selection** flow that records which key-custody
options are offered and which is active, the **storage selection** flow that
records which storage locations are offered and which is active, the
**region/residency cues** that disclose where each managed artifact scope resides
and whether the residency pin is verified, and the **managed-service continuity**
rows that record, per managed-service capability, what stays local and what requires
provider or admin continuity when the managed service degrades. The machine-readable
truth source is the checked-in support export; later desktop companion panel,
CLI/headless, diagnostics, support export, and Help/About surfaces ingest it instead
of cloning status text.

- Record kind: `add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont`
- Schema: `schemas/companion/add-customer-managed-key-or-storage-selection-flows-region-or-residency-cues-and-degraded-managed-service-cont.schema.json`
- Support export: `artifacts/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont/support_export.json`
- Markdown summary: `artifacts/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont.md`
- Fixtures: `fixtures/companion/m5/add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont/`
- Producer crate: `aureline-companion`

## Sections and matrix inheritance

The packet has four sections. The key-custody-selection, storage-selection, and
residency-cue sections inherit their qualification and staged rollout stage from the
frozen M5 companion-matrix `residency_encryption` lane; the
managed-service-continuity section inherits from the `offboarding_continuity` lane
(see `docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`),
so a section never claims more than the matrix qualifies. The key/storage/encryption
vocabulary (`key_custody`, `encryption_posture`, `encrypted_artifact_scope`) is
reused from the managed sync maturity surface rather than re-invented.

| Section | Matrix lane | Scope | Qualification | Rollout stage |
| --- | --- | --- | --- | --- |
| `key_custody_selection` | `residency_encryption` | `read_only` | preview | early_access |
| `storage_selection` | `residency_encryption` | `read_only` | preview | early_access |
| `residency_cue` | `residency_encryption` | `read_only` | preview | early_access |
| `managed_service_continuity` | `offboarding_continuity` | `read_only` | beta | staged_rollout |

## Read-only selection, with the local core authoritative

Every section is read-only. The surface **projects** the selection flow but never
applies a selection: a key-custody, storage-location, or residency change is applied
by the local core, never authored from this surface
(`selection_applied_by_local_core_not_surface`). A local-only key option and a
local-first storage option are always offered as a fallback, so a managed
degradation never strands the user.

- **Key custody selection** records each offered `offered_custody`
  (`customer_managed_key`, `provider_managed_key`, `local_only_no_key_escrow`), its
  `selection_state` (`active`, `available`, `requires_admin_approval`,
  `unavailable`), and the `encryption_posture` the custody option yields. A
  local-only-no-escrow option is always present (`local_only_key_fallback_offered`).
- **Storage selection** records each offered `offered_location` (`local_only`,
  `customer_managed_bucket`, `provider_managed_region`, `hybrid_local_first`), its
  `selection_state`, and its `residency_region_ref`. A local-first option
  (`local_only` or `hybrid_local_first`) is always present
  (`local_first_storage_fallback_offered`).
- **Residency cues** record, per managed `artifact_scope` (`managed_snapshot_store`,
  `sync_transport`, `conflict_history`, `device_registry_store`), the
  `residency_region_ref` and the `pin_state` (`pinned_verified`, `pinned_unverified`,
  `unpinned`).
- **Managed-service continuity** records, per `capability` (`managed_sync`,
  `key_management`, `residency_pinning`, `device_approval`, `managed_audit_log`), a
  `continuity_posture` and a `degraded` flag.

## Provable where claimed

A key-custody option claims a verified encryption posture
(`end_to_end_encrypted_verified`, `encrypted_at_rest_verified`), and a residency cue
claims a verified pin (`pinned_verified`), only when it is backed by evidence
(`claim_verified = true`). An unverifiable claim narrows — encryption to
`claimed_unverified`, residency to `pinned_unverified` — and sets
`proof_label_shown = true` so it is labeled, never shown as proven. The same applies
to a storage option's residency claim.

## Local-first continuity

Each managed-service continuity row records a `continuity_posture` that says what
stays local and what requires provider or admin continuity:

- `local_core_continues_unaffected` — the capability is fully local and a
  managed-service outage has no effect.
- `degraded_local_fallback` — the managed feature degrades to a local fallback that
  keeps the user working.
- `requires_provider_continuity` — the capability is suspended until the provider
  returns; local work is preserved.
- `requires_admin_continuity` — the capability requires managed-tenant admin
  continuity; local work is preserved.

Every row asserts `local_work_preserved = true`: a degraded managed service or
offboarding never strands user-owned local work.

## Stale-state honesty

Every item carries a `freshness` state (`live`, `cached`, `stale`, `unknown`). When
freshness `requires_label` (stale or unknown), `stale_label_shown` is set, and a
degraded item is never shown as live.

## Degraded behavior

`apply_residency_continuity_degradation` narrows sections, narrows selection options
to their local fallback, and downgrades freshness, residency pins, and encryption
claims from a per-observation signal, recording the reasons in `degraded_labels`:

| Observation | Effect |
| --- | --- |
| `managed_service_available = false` | every section narrows one step, every live/cached item goes stale, every non-local continuity capability is marked degraded; labels `managed_service_degraded`, `freshness_downgraded_to_stale` |
| `key_management_available = false` | every non-local key-custody option narrows to `requires_admin_approval`; the key-custody section narrows; labels `key_management_unavailable`, `selection_narrowed_to_local_fallback` |
| `storage_provider_available = false` | every non-local-fallback storage option narrows; the storage section narrows; labels `storage_provider_unavailable`, `selection_narrowed_to_local_fallback` |
| `residency_verified = false` | every verified residency pin (cues and storage options) downgrades to unverified-and-labeled; the residency-cue and storage sections narrow; labels `residency_unverified`, `residency_claim_downgraded` |
| `encryption_verified = false` | every verified encryption claim downgrades to `claimed_unverified` and is labeled; the key-custody section narrows; labels `encryption_unverified`, `encryption_claim_downgraded` |
| `admin_continuity_available = false` | the key-custody, residency-cue, and continuity sections narrow; labels `admin_continuity_unavailable` |
| `proof_fresh = false` / `upstream_matrix_narrowed = true` | every section narrows; labels `proof_stale` / `upstream_matrix_narrowed` |
| `host_session_active = false` | every host-dependent exact handoff narrows to `unresolved`; labels `host_session_inactive`, `handoff_target_unresolved` |

The local-only key and local-first storage options always remain, and local work is
always preserved. Degraded state is labeled, never hidden.

## Boundary and redaction

Credential bodies, raw key material, raw provider payloads, and raw storage record
contents never cross this boundary; the packet carries only redacted summaries and
opaque refs (`evidence_ref`, `record_ref`, `residency_region_ref`,
`deep_link_ref`).

## Regenerating the artifacts

The checked-in support export, the Markdown summary, and the degraded fixtures are
generated deterministically from the first-consumer surface builder:

```text
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- canonical
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- markdown
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- managed_service_degraded
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- key_management_down
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- storage_provider_down
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- residency_unverified
cargo run -p aureline-companion --example dump_key_storage_residency_continuity_surface -- admin_continuity_lost
```
