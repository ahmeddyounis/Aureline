# Artifact: Certify notebook diff, review, export, collaboration, and experiment packets and narrow unqualified rows

## Lane

M05-033 — Notebook diff, review, collaboration, experiment lineage, and reproducibility depth certification.

## Claim

- The M5 notebook depth lanes (diff/review, export, collaboration, experiment,
  narrowing) are bound to canonical qualification states.
- Every lane carries a current sub-packet ref, rollback path state, and
  explicit downgrade reasons when narrowed.
- Unqualified rows are automatically narrowed rather than left asserting a
  claim the evidence no longer backs.
- The checked-in certification packet is referenced by the canonical M5
  evidence index and consumed by docs, help, support, and CI surfaces.

## Evidence

| Evidence kind | Path | State |
|---|---|---|
| Rust implementation | `crates/aureline-notebook/src/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows/` | Landed |
| Schema | `schemas/notebook/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.schema.json` | Landed |
| Checked-in packet | `artifacts/notebook/m5/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.json` | Landed |
| Fixture corpus | `fixtures/notebook/m5/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows/` | Landed |
| Integration tests | `crates/aureline-notebook/tests/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.rs` | Landed |
| Docs | `docs/notebook/m5/certify_notebook_diff_review_export_collaboration_and_experiment_packets_and_narrow_unqualified_rows.md` | Landed |

## Downgrade rules

- If `freshness_expired`, the lane narrows to `narrowed` and triggers
  `automatic_narrowing`.
- If `packet_missing`, the lane narrows to `rule_missing` and triggers
  `manual_hold`.
- If `evidence_stale`, the lane narrows to `stale` and triggers
  `automatic_narrowing`.
- If `rollback_path_missing`, the lane narrows to `narrowed` and triggers
  `manual_hold`.
- If `underqualified_sub_lane`, the lane narrows to `incomplete` and triggers
  `automatic_narrowing`.
- If `policy_blocked`, the lane narrows to `blocked` and triggers
  `emergency_rollback`.

## Rollback path

1. Revert to the previous certified packet revision stored in the canonical
   artifact path.
2. Notify downstream docs, help, support, and CI surfaces via the support
   export pipeline.
3. Update the M5 evidence index to reflect the narrowed claim and effective
   label.
4. Re-run the certification validation gate before any promotion to stable.

## Freshness SLO

- Packet must be refreshed when any sub-packet it certifies is refreshed or
  when a downgrade rule changes.
- Target max age: 30 days.
- Warn window: 7 days before expiry.

## Owner

Notebook subsystem owner (see `CODEOWNERS`).
