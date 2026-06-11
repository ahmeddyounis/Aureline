# M5 Companion, Incident, Sync, Residency, Encryption, and Offboarding Lane Certification

This document is the human-readable contract for the certification that locks
whether each frozen M5 companion-matrix lane may keep its public claim on every
**marketed M5 profile**. The machine-readable truth source is the checked-in
support export; later browser/mobile companions, the desktop companion panel,
incident workspaces, diagnostics, support exports, release tooling, and Help/About
surfaces ingest it instead of cloning status text.

- Record kind: `certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile`
- Schema: `schemas/companion/certify-companion-incident-sync-residency-encryption-and-offboarding-lanes-on-every-marketed-m5-profile.schema.json`
- Support export: `artifacts/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/support_export.json`
- Markdown summary: `artifacts/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile.md`
- Fixtures: `fixtures/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/`
- Producer crate: `aureline-companion`

## Truth source and matrix binding

The certification is canonical for claimed M5 support in this lane: **no surface
may stay greener than this packet**. Every lane's headline claim is ceilinged by
the frozen matrix
(`docs/companion/m5/freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes.md`):
the builder reads each lane's qualification straight from the checked-in matrix
export, records it as `matrix_baseline_qualification`, and validation refuses any
`claimed_qualification` that exceeds it (`claim_exceeds_matrix_baseline`) or any
recorded baseline that disagrees with the live matrix (`matrix_baseline_mismatch`).
The certification can only ever *shrink* a claim relative to the matrix, never
widen it.

## Marketed M5 profiles

The eight matrix lanes are certified across six marketed profiles:

| Profile | Managed plane | Companion relay | Notes |
| --- | --- | --- | --- |
| `local_solo` | no | yes | Individual local-first install, no managed control plane |
| `team_managed` | yes | yes | Team on a managed sync and cloud plane |
| `enterprise_managed` | yes | yes | Enterprise tenant with admin continuity, customer-managed keys, region residency |
| `browser_companion` | no | yes | Browser companion channel paired to a desktop host |
| `mobile_companion` | no | yes | Mobile companion channel paired to a desktop host |
| `air_gapped_offline` | no | no | Offline/air-gapped install with no provider or admin continuity |

## Marketability rules

Each `(lane, profile)` cell is certified only where the lane is genuinely
marketable, and the row otherwise records `certified_on_profile: false` with a
plain "not marketed on this profile" disclosure rather than optimistic language:

- **Managed lanes** (`managed_sync`, `residency_encryption`) are certified only on
  profiles that provide a managed plane (`team_managed`, `enterprise_managed`). A
  managed claim on any other profile fails validation
  (`lane_not_marketable_on_profile`), so no managed surface implies a hidden
  control plane.
- **Relay-bound companion lanes** (`companion_notification`, `companion_review`,
  `companion_session_follow`, `companion_light_edit`) are certified on every
  profile that can reach the companion relay — all except `air_gapped_offline`.
- **Local-first lanes** (`incident_workspace`, `offboarding_continuity`) are
  certified on **every** profile, including `air_gapped_offline`, because they run
  entirely on the local core. Their certified rows never require provider or admin
  continuity (`continuity_flag_inconsistent` guards this), proving incident packets
  stay attributable and offboarding never strands user-owned local work even with
  no provider at all.

## Locality and local-core continuity

Every certified row carries an explicit locality disclosure — what stays local,
what is staged, and what requires provider or admin continuity — and validation
requires the disclosure to be complete (`locality_disclosure_incomplete`). Every
row, certified or not, must assert `local_core_continuity_preserved`; a row that
would strand user-owned local work fails (`local_core_continuity_stranded`).

## Freshness honesty

Each row carries a `freshness` state (`fresh`, `stale`, `unknown`). A `stale` or
`unknown` state must set `freshness_label_shown`; an unlabeled degraded row fails
(`freshness_state_not_labeled`), so a degraded row is never shown as live.

## Downgrade rules and automation

Each lane carries a closed set of downgrade rules. Every lane includes the
`proof_stale` rule (`downgrade_rule_missing_proof_stale` guards this), and every
rule must narrow strictly below the matrix baseline
(`downgrade_rule_not_narrowing`). `narrowed_qualification(trigger)` is the
deterministic lookup consumers and release tooling project instead of re-deriving
narrowing locally.

`apply_downgrade_automation(observations)` is the failure/recovery automation that
release and support tooling run against live signals:

- **Invalid evidence** holds the lane (`held`) and withholds every certified row,
  recording `evidence_invalid`.
- A **stale proof**, **unavailable provider/admin plane**, **unverified residency
  or encryption claim**, or **narrowed upstream matrix lane** narrows the headline
  claim and every certified row one step each, and a stale proof additionally
  forces every row to a labeled stale state. The reasons are recorded in
  `degraded_labels` so a stale or underqualified row automatically narrows before
  publication instead of shipping greener than the evidence.

Narrowing always stays at or below the matrix baseline, so automation can only
shrink the certification.

## Consumer projection

The certification is projected by the desktop companion panel, browser companion,
mobile companion, incident workspace, CLI/headless replay, support export,
diagnostics, and Help/About. Unqualified or not-marketed rows are visibly labeled
rather than dressed up as claimed.

## Boundary safety

The packet carries only typed disclosure booleans, class tokens, and review-safe
summaries. Credential bodies, raw provider payloads, and raw sync record contents
never cross this boundary; validation rejects obvious forbidden material
(`raw_boundary_material_in_export`).

## Regenerating the artifacts

The checked-in support export, Markdown summary, and fixtures are generated
deterministically from the first-consumer builder
(`canonical_m5_companion_certification`) via the conformance dump:

```text
cargo run -p aureline-companion --example dump_m5_companion_certification -- canonical              > artifacts/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/support_export.json
cargo run -p aureline-companion --example dump_m5_companion_certification -- markdown               > artifacts/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile.md
cargo run -p aureline-companion --example dump_m5_companion_certification -- proof_stale            > fixtures/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/proof_stale_certification.json
cargo run -p aureline-companion --example dump_m5_companion_certification -- sync_evidence_invalid  > fixtures/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/sync_evidence_invalid_certification.json
cargo run -p aureline-companion --example dump_m5_companion_certification -- residency_unverified   > fixtures/companion/m5/certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile/residency_unverified_certification.json
```

The `checked_support_export_matches_canonical_builder` test fails if the checked-in
support export drifts from the builder.
