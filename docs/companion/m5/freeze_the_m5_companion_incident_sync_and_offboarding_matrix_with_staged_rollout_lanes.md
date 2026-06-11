# M5 Companion, Incident, Sync, Residency, and Offboarding Matrix

This document is the human-readable contract for the frozen M5 matrix that locks
the companion, incident, managed-sync, residency, and offboarding lanes with
staged rollout stages. The machine-readable truth source is the checked-in
support export; later companion, incident, support, diagnostics, and Help/About
surfaces ingest it instead of cloning status text.

- Record kind: `freeze_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes`
- Schema: `schemas/companion/freeze-the-m5-companion-incident-sync-and-offboarding-matrix-with-staged-rollout-lanes.schema.json`
- Support export: `artifacts/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes/support_export.json`
- Markdown summary: `artifacts/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`
- Fixtures: `fixtures/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes/`
- Producer crate: `aureline-companion`

## Domains and lanes

The matrix freezes eight lanes across five domains. Each lane row carries a
qualification class, a staged rollout stage, an explicit locality disclosure,
required evidence packet refs, downgrade triggers, a rollback posture, source
contracts, and the consumer surfaces that must project it.

| Domain | Lane | Qualification | Rollout stage |
| --- | --- | --- | --- |
| Companion | `companion_notification` | stable | general_availability |
| Companion | `companion_review` | stable | general_availability |
| Companion | `companion_session_follow` | beta | staged_rollout |
| Companion | `companion_light_edit` | preview | early_access |
| Incident | `incident_workspace` | stable | general_availability |
| Sync | `managed_sync` | beta | staged_rollout |
| Residency | `residency_encryption` | beta | staged_rollout |
| Offboarding | `offboarding_continuity` | stable | general_availability |

## Locality disclosure

Every row states three things explicitly, so no companion or managed surface can
imply a second flagship or a hidden control plane:

- **Stays local** — what the local core computes, owns, and keeps usable offline.
- **Staged** — what is gated behind a rollout cohort or capability flag.
- **Requires provider or admin continuity** — what depends on the companion
  relay, the sync provider, the managed key authority, or admin teardown — and
  which the local core never depends on to keep working.

## Track invariants

- Browser and mobile companions stay narrow: notification and review are
  read-only, session-follow is read plus handoff, and light-edit is a bounded
  touch-up relayed to the host for preview/approval — never a full mobile editor.
- Incident packets stay attributable: incident workspaces bind crash trails,
  evidence spans, and runbook steps to a redacted support bundle preview, and a
  lost attribution narrows the lane (`incident_attribution_missing`).
- Managed sync stays inspectable: the local core is the source of truth, every
  synced record is reconcilable, and inspection loss narrows the lane
  (`sync_inspection_unavailable`).
- Customer-managed-key and end-to-end-encryption residency claims stay provable:
  they are claimed only when verifiable, and an unverified claim narrows the lane
  (`residency_or_encryption_unverified`).
- Offboarding never strands user-owned local work: export preserves the local
  core intact, and any risk of stranding narrows the lane
  (`offboarding_strands_local_work`).

## Staged rollout and downgrade automation

`M5CompanionMatrixPacket::apply_downgrade_automation` narrows lanes from
per-lane observations so release tooling can react before publication:

- Evidence that fails validation holds the lane (`held`) and withholds its
  rollout (`withheld`).
- Stale proof, unavailable provider/admin continuity, an unverified
  residency/encryption claim, or a narrowed upstream dependency narrows the
  qualification one step (e.g. stable → beta) and the rollout stage one step
  (e.g. general_availability → staged_rollout).

Downgrade narrows the claim and stage rather than hiding the lane, and a stale or
underqualified lane blocks promotion.

## Proof freshness

The packet carries a proof-freshness SLO (168 hours) with automatic narrowing on
stale proof. The checked-in export is regenerated deterministically from the
first-consumer builder via:

```text
cargo run -p aureline-companion --example dump_m5_companion_matrix -- canonical
cargo run -p aureline-companion --example dump_m5_companion_matrix -- markdown
cargo run -p aureline-companion --example dump_m5_companion_matrix -- sync_withheld
cargo run -p aureline-companion --example dump_m5_companion_matrix -- residency_narrowed
```

## Boundary

Credential bodies, raw provider payloads, and raw sync record contents never
cross this boundary. The packet is metadata-only and export-safe.
