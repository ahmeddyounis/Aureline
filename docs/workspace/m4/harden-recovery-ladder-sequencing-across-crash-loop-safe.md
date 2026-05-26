# Recovery-ladder sequencing lineage — contract

This document describes the recovery-ladder sequencing lineage record:
the workspace's governed, export-safe projection that hardens how
Aureline walks through crash-loop safe mode, suspect-extension
quarantine, open-without-restore, cache/index repair, restricted
reopen, and (optionally) typed repair and support-export handoff —
and proves that the recovery sequence never silently replays
privileged work, never drops user state without disclosure, and never
narrows trust into an unreviewable state.

Where the cache / storage-class lineage proves *the storage layer
underneath caches and durable state* and the trust-gating lineage
proves *that privileged surfaces are gated under restricted
workspaces*, this record proves the *ordered recovery sequence* the
user, support, and the IDE itself walk when something goes wrong:
which rungs exist, which trigger reaches each rung, which no-rerun
posture each rung declares, which reversibility class each rung
admits, which user-state preservation posture each rung commits to,
which inspection / repair hooks fire before any destructive rung
commits, and which support-export projection each rung ships.

The record is the single artifact every consuming surface (workspace
recovery status, command-palette recovery actions, Help/About,
support cleanup tool, headless CLI, support export) ingests instead
of cloning status text.

## Input

The projection ingests a live
[`RecoveryLadderInputs`](../../../crates/aureline-workspace/src/recovery_ladder_lineage/mod.rs)
envelope verbatim. The envelope carries one
[`RecoveryRungObservation`](../../../crates/aureline-workspace/src/recovery_ladder_lineage/mod.rs)
per governed recovery rung (crash-loop safe mode, safe-mode
quarantine, open-without-restore, cache/index repair, restricted
reopen, typed repair flow, support export handoff). Each rung row
records the declared step ordinal, the trigger class and disclosure
id, the no-rerun posture, the commit action / disclosure ids, the
user-state preservation posture and export-before-repair disclosure
id, the reversibility class and any rollback checkpoint or
irreversibility disclosure id, and the support-export projection.

For determinism and replay, the projection accepts the same envelope
shape the fixtures and the headless emitter consume.

## What the record proves

- **Rung-coverage truth.** Every governed recovery rung ships a row
  bound to one closed `recovery_rung_kind` (`crash_loop_safe_mode`,
  `safe_mode_quarantine`, `open_without_restore`,
  `cache_index_repair`, `restricted_reopen`, `typed_repair_flow`,
  `support_export_handoff`). The corpus seeds one row per required
  rung so the user never lands on a release where a recovery rung is
  missing.
- **Sequence truth.** Each rung carries its canonical step ordinal so
  the ladder cannot ship out of order or skip past a less-invasive
  rung silently.
- **Trigger-disclosure honesty.** Each rung carries a closed trigger
  class plus a non-empty trigger disclosure id so the user can pivot
  to the reason the rung was reached.
- **No-rerun honesty.** Every rung that touches privileged or
  mutating surfaces declares `explicit_user_action_required` (or
  `terminal_no_further_run` when no further run is possible) and
  references a commit action id and a commit disclosure id — never
  `auto_continue_after_checkpoint`.
- **User-state preservation truth.** Every rung declares one closed
  `user_state_preservation_posture`; lossy postures
  (`preserved_after_export_prompt` and `dropped_with_disclosure`)
  require an `export_before_repair` disclosure id.
- **Reversibility truth.** Every rung declares one closed
  `reversibility_class`. `reversible_with_checkpoint` rungs reference
  a rollback checkpoint id; irreversible rungs reference an
  irreversibility disclosure id.
- **Support-export honesty.** Each rung's support-export projection
  preserves `rung_kind`, `trigger_class`, `no_rerun_posture`,
  `user_state_preservation`, `reversibility`, `step_ordinal`, and
  `disclosure_id`, redacts raw secrets, approval tickets, delegated
  credentials, and live authority handles, and refuses
  local-only postures on credential-touching rungs.
- **Pre-action inspection-hook honesty.** A controlled set of
  pre-action inspection / repair hooks (`inspect_ladder_state`,
  `compare_before_action`, `export_before_repair`,
  `rollback_checkpoint`, `export`, `repair`) is reachable so
  destructive recovery rungs stay reviewable.
- **Producer attribution.** Each record carries a producer ref,
  schema version, capture timestamp, and integrity hash so replay
  and support pipelines can pin the source before applying.
- **Lineage and export honesty.** The record sets
  `raw_payload_excluded = true` and carries only opaque refs to the
  source workspace, corpus, and producer.

## Output record shape

The projection produces a single
[`RecoveryLadderLineageRecord`](../../../crates/aureline-workspace/src/recovery_ladder_lineage/mod.rs)
with the following pillars:

- `rung_sequence_coverage` — per-rung rows plus required-rung and
  canonical-ordering flags.
- `trigger_disclosure` — whether every rung references a trigger
  disclosure id.
- `no_rerun_honesty` — whether every privileged rung is safe and
  every explicit rung carries metadata.
- `user_state_preservation` — count of lossy rungs and whether they
  all reference an export-before-repair disclosure id.
- `reversibility_truth` — checkpointed and irreversible rung counts
  plus their disclosure / checkpoint coverage flags.
- `support_export_honesty` — per-row field preservation and
  redaction flags.
- `inspection_hooks` — the captured pre-action inspection / repair
  hook table.
- `producer_attribution` — opaque producer ref, schema version,
  capture timestamp, and integrity hash.
- `stable_qualification` — whether the record proves the contract on
  the claimed posture, with named narrow reasons when not.
- `summary` — a single-line human-readable summary.

## Stable qualification

A record is `stable` only when every pillar passes. Otherwise it is
`narrowed_below_stable` with one or more named narrow reasons drawn
from the closed
[`RecoveryLadderLineageNarrowReason`](../../../crates/aureline-workspace/src/recovery_ladder_lineage/mod.rs)
vocabulary.

## Consumers

The workspace recovery status surface, the command-palette recovery
actions, Help/About, the support cleanup tool, the headless CLI, and
the support export ingest the same human-readable projection
(`recovery_ladder_lineage_lines`) so no surface clones status text.

## Verification

```sh
cargo test -p aureline-workspace --lib recovery_ladder_lineage
cargo test -p aureline-workspace --test recovery_ladder_lineage_replay
cargo run -p aureline-workspace --bin aureline_recovery_ladder_lineage -- --lines \
  fixtures/workspace/m4/recovery_ladder_lineage/baseline_recovery_ladder_stable.json
```
