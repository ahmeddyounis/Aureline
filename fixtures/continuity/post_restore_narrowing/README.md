# Post-restore narrowing cases

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures`.

Every file validates against
`schemas/continuity/restore_identity_summary.schema.json`
(`python3 tools/validate_m5_restore_from_backup_review_fixtures.py`).

## Files

- `page.json` — seeded stable page. It carries one continuity-restore review for
  each managed artifact family (managed records, policy bundles, sync metadata),
  a support/export-record review, and an ordinary local workspace restore. Every
  managed-continuity review distinguishes exact from narrower-than-normal restore
  identity, names the affected slice when narrower, fences its privileged and
  externally mutating lanes, and offers restored-vs-current compare and export.
  Every claimed restored row points to a current clean review.
- `summary.json` — seeded page summary record
- `registry.json` — seeded restore-review registry record (per-claim-row coverage)
- `support_export.json` — support-export wrapper for the seeded page
- `case_full_normal_status_overclaim_withdrawn.json` — a narrower-than-normal
  restore claims full normal status; it fails closed and is withdrawn
- `case_restore_lane_conflated_withdrawn.json` — a managed row presents an
  ordinary workspace restore as continuity restore; it fails closed and is
  withdrawn
- `case_privileged_lane_auto_replayed_withdrawn.json` — a privileged lane is left
  unfenced and would auto-replay; it fails closed and is withdrawn
- `case_affected_slice_unnamed_beta.json` — a narrower-than-normal restore does
  not name the affected slice and narrows to beta
- `case_replay_fence_review_missing_beta.json` — a cleared fence names no explicit
  reviewed step and narrows to beta
- `case_compare_parity_missing_preview.json` — a managed-continuity review cannot
  compare restored-vs-current state and is held at preview
- `case_review_evidence_missing_preview.json` — a claimed restored row carries no
  review and is held at preview

## Regeneration

```sh
DIR=fixtures/continuity/post_restore_narrowing
EX="cargo run -q -p aureline-continuity --example dump_m5_restore_from_backup_review_fixtures --"
$EX page > $DIR/page.json
$EX summary > $DIR/summary.json
$EX registry > $DIR/registry.json
$EX support-export > $DIR/support_export.json
$EX case-full-normal-status-overclaim-withdrawn > $DIR/case_full_normal_status_overclaim_withdrawn.json
$EX case-restore-lane-conflated-withdrawn > $DIR/case_restore_lane_conflated_withdrawn.json
$EX case-privileged-lane-auto-replayed-withdrawn > $DIR/case_privileged_lane_auto_replayed_withdrawn.json
$EX case-affected-slice-unnamed-beta > $DIR/case_affected_slice_unnamed_beta.json
$EX case-replay-fence-review-missing-beta > $DIR/case_replay_fence_review_missing_beta.json
$EX case-compare-parity-missing-preview > $DIR/case_compare_parity_missing_preview.json
$EX case-review-evidence-missing-preview > $DIR/case_review_evidence_missing_preview.json
```

The canonical evidence packets under `artifacts/m5/continuity/restore_reviews/`
are regenerated from the same example (`page`, `registry`, and `support-export`).
