# Admin certification — evidence companion

Human-readable companion to
[`/fixtures/admin/m5-admin-certification/canonical_certification.json`](../../fixtures/admin/m5-admin-certification/canonical_certification.json)
and its boundary schema
[`/schemas/admin/m5-admin-certification.schema.json`](../../schemas/admin/m5-admin-certification.schema.json).
It gives reviewers the per-profile qualification rows without reading the JSON. The
contract narrative lives in
[`/docs/admin/m5-admin-certification.md`](../../docs/admin/m5-admin-certification.md),
and the frozen object model it binds back to lives in
[`/artifacts/admin/m5-admin-plane.md`](./m5-admin-plane.md).

- Bundle id: `m5-admin-certification:bundle:0001`
- Record kind: `m5_admin_certification_bundle`
- Binds matrix: `m5-admin-plane:matrix:0001`
- Profiles: 4 · Families per profile: 6 · Release-evidence rows: 5 · Invariants: 16

## Families and the proof lanes they certify

| Family | Certified surfaces | Proof lane |
| --- | --- | --- |
| `policy_explainability` | effective_policy_view, policy_diff, locked_state_explanation | `m5_admin_render_bundle` |
| `decision_history` | decision_history_timeline | `m5_decision_history_bundle` |
| `endpoint_posture` | endpoint_posture_card | `m5_admin_render_bundle` |
| `retention_delete` | retention_deletion_matrix | `m5_retention_deletion_bundle` |
| `offboarding` | offboarding_wizard | `m5_offboarding_bundle` |
| `procurement_admin_packet` | procurement_verification_packet | `m5_procurement_bundle` |

Each family reads its freshness and pass/fail result from the upstream lane's
boundary schema, worked fixture, and freeze gate; the capstone does not re-derive
the truth.

## Profiles and managed-claim auto-narrowing

| Profile | Deployment | Proof freshness | Claim state | Narrow reasons |
| --- | --- | --- | --- | --- |
| `managed_cloud` | managed_cloud | fresh | active_enforced | — |
| `self_hosted` | self_hosted | recent | unconfirmed_stale | family_proof_failing |
| `sovereign_air_gapped` | sovereign_air_gapped | stale | unconfirmed_stale | family_evidence_stale |
| `mirrored_offline` | managed_cloud | stale | unconfirmed_stale | mirror_evidence_stale |

The managed claim reads `active_enforced` only when every claimed family is proven
fresh and passing. The self-hosted profile auto-narrows because its audit-history
proof is failing; the sovereign profile auto-narrows because its endpoint-posture
and procurement proof is stale; the mirrored profile auto-narrows because its
mirror-backed retention and offboarding proof is stale. No-console-required
explainability holds on the narrowed rows.

## Per-family qualification

| Profile | Family | Qualification | Claim state | Proof freshness | Narrow reason |
| --- | --- | --- | --- | --- | --- |
| `managed_cloud` | policy_explainability | qualified | active_enforced | fresh | — |
| `managed_cloud` | decision_history | qualified | active_enforced | fresh | — |
| `managed_cloud` | endpoint_posture | qualified | active_enforced | fresh | — |
| `managed_cloud` | retention_delete | qualified | active_enforced | fresh | — |
| `managed_cloud` | offboarding | qualified | active_enforced | fresh | — |
| `managed_cloud` | procurement_admin_packet | qualified | active_enforced | fresh | — |
| `self_hosted` | decision_history | narrowed_failing_proof | unconfirmed_stale | fresh | family_proof_failing |
| `self_hosted` | (other five) | qualified | active_enforced | recent | — |
| `sovereign_air_gapped` | endpoint_posture | narrowed_stale_evidence | unconfirmed_stale | stale | family_evidence_stale |
| `sovereign_air_gapped` | procurement_admin_packet | narrowed_stale_evidence | unconfirmed_stale | stale | family_evidence_stale |
| `sovereign_air_gapped` | (other four) | qualified | active_enforced | recent | — |
| `mirrored_offline` | retention_delete | narrowed_stale_evidence | unconfirmed_stale | stale | mirror_evidence_stale |
| `mirrored_offline` | offboarding | narrowed_stale_evidence | unconfirmed_stale | stale | mirror_evidence_stale |
| `mirrored_offline` | (other four) | qualified | active_enforced | recent | — |

A row reads `qualified` (→ `active_enforced`) only when its proof is bound, fresh,
and passing. A failing proof narrows even when fresh in age (`self_hosted`'s
audit history), proving the failing axis is independent of freshness.

## Release-evidence rows (worst case across profiles)

| Dimension | Families | Worst qualification | Claim state |
| --- | --- | --- | --- |
| `policy_source_verification` | policy_explainability, endpoint_posture | narrowed_stale_evidence | unconfirmed_stale |
| `audit_history` | decision_history | narrowed_failing_proof | unconfirmed_stale |
| `delete_export_honesty` | retention_delete | narrowed_stale_evidence | unconfirmed_stale |
| `offboarding_continuity` | offboarding | narrowed_stale_evidence | unconfirmed_stale |
| `procurement_support_admin_packet` | procurement_admin_packet | narrowed_stale_evidence | unconfirmed_stale |

Each release-evidence row carries the most-narrowed qualification across **every**
profile for its bound families, so release automation downgrades the affected
managed claim and never reads rosier than the underlying admin plane.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `admin_cert.families_covered` | Every certified family has exactly one row on every profile. |
| `admin_cert.profiles_covered` | Managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline are all certified. |
| `admin_cert.bound_surfaces_in_matrix` | Every certified surface is present, locally explainable, and typed in the frozen matrix. |
| `admin_cert.claim_states_in_vocabulary` | Every claim state is one the matrix's unified vocabulary defines. |
| `admin_cert.no_green_on_stale_or_failing` | A row qualifies only when its proof is bound, fresh, and passing; otherwise it narrows and names a reason. |
| `admin_cert.qualified_requires_proven_lane` | A qualified row cites a real, export-safe proof lane. |
| `admin_cert.proof_lane_bound` | Every row cites a non-empty, export-safe proof lane. |
| `admin_cert.profile_claim_auto_narrows` | A profile is confirmed only when every claimed family qualifies; otherwise it downgrades and names reasons. |
| `admin_cert.proof_freshness_is_worst_case` | The reported proof freshness is the stalest claimed family. |
| `admin_cert.release_evidence_rows_present` | One release-evidence row per named dimension, each bound to at least one family. |
| `admin_cert.release_evidence_reflects_worst` | Each release-evidence row carries the worst qualification across all profiles. |
| `admin_cert.local_explainability` | Every family is locally explainable, never portal-only. |
| `admin_cert.consumer_parity` | One typed packet serves shell, CLI/headless, Help/About, support export, commercial/procurement, and release evidence. |
| `admin_cert.stable_ids_unique` | Profile, row, and release-evidence ids are unique within scope. |
| `admin_cert.export_safe` | Every stable id is an opaque token and every proof-lane ref is repo-relative. |
| `admin_cert.binds_admin_plane_matrix` | The bundle binds the frozen matrix by id and cites its canonical fixture. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_admin_certification > \
  fixtures/admin/m5-admin-certification/canonical_certification.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_admin_certification

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_admin_certification -- --lines
```
