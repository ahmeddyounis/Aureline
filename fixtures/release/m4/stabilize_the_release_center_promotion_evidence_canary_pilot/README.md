# Ring Promotion Control Fixtures

This directory contains mutated copies of the canonical ring promotion control
artifact used to exercise validation failures.

## Files

- `valid_artifact.json` — unmodified artifact; should parse and validate.
- `missing_subject_kind.json` — removes all `ai_tool_pack` rows; triggers `SubjectKindMissing`.
- `held_with_active_gap.json` — adds a gap reason to a `qualified` row; triggers `HeldRowWithActiveGap`.
- `narrowing_row_not_narrowed.json` — sets `effective_label` back to `claim_label` on a narrowed row; triggers `EffectiveLabelNotNarrowed`.
- `stale_packet_not_narrowed.json` — leaves a stale row at `stable`; triggers `EffectiveLabelNotNarrowed`.
- `target_narrower_than_current.json` — sets target ring below current; triggers `TargetRingNarrowerThanCurrent`.
- `missing_rule_for_reason.json` — removes the `waiver_expired` rule; triggers `GapReasonWithoutRule`.
- `rollback_target_wider.json` — sets rollback target wider than current; triggers `RollbackTargetWiderThanCurrent`.

## Usage

Run the model tests:

```bash
cargo test -p aureline-release
```

The checked-in fixtures are loaded and rejected by the model tests in
`tests/ring_promotion_control.rs`.
