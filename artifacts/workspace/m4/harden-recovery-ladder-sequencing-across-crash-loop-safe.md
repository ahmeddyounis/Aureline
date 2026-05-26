# Harden Recovery-Ladder Sequencing Across Crash-Loop Safe Mode, Quarantine, Open-Without-Restore, Cache/Index Repair, and Restricted Reopen — proof packet

Reviewer-facing proof packet for the recovery-ladder sequencing lane:
every required ladder rung (crash-loop safe mode, suspect-extension
quarantine, open-without-restore, cache/index repair, restricted
reopen, plus the optional typed-repair-flow and
support-export-handoff rungs) is bound to one closed trigger class,
one closed no-rerun posture, one closed user-state preservation
posture, and one closed reversibility class. The projection re-derives
each rung's canonical step ordinal so the ladder cannot ship out of
order. Privileged rungs (anything past `crash_loop_safe_mode`) must
declare `explicit_user_action_required` (or
`terminal_no_further_run`) and reference both a commit action id and
a commit disclosure id so resume / reconnect / recovery cannot
silently replay terminals, tasks, debug sessions, or AI apply. Lossy
rungs must reference an `export_before_repair` disclosure id; rungs
declared `reversible_with_checkpoint` must reference a rollback
checkpoint id; irreversible rungs must reference an irreversibility
disclosure id. A destructive rung never fires without the controlled
inspection / repair hook table (`inspect_ladder_state`,
`compare_before_action`, `export_before_repair`,
`rollback_checkpoint`, `export`, `repair`) being reachable; a missing
hook narrows the record below Stable with a named reason. This packet
is the stable-line anchor for this lane; dashboards, docs, Help/About
surfaces, and support exports should ingest the typed sources below
rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/recovery_ladder_lineage/`](../../../crates/aureline-workspace/src/recovery_ladder_lineage/)
- Schema:
  [`/schemas/workspace/recovery_ladder_lineage.schema.json`](../../../schemas/workspace/recovery_ladder_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_recovery_ladder_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_recovery_ladder_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/recovery_ladder_lineage/`](../../../fixtures/workspace/m4/recovery_ladder_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/recovery_ladder_lineage_replay.rs`](../../../crates/aureline-workspace/tests/recovery_ladder_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/harden-recovery-ladder-sequencing-across-crash-loop-safe.md`](../../../docs/workspace/m4/harden-recovery-ladder-sequencing-across-crash-loop-safe.md)
- Typed consumer: `aureline_workspace::project_recovery_ladder_lineage`

## What this packet proves

1. **Rung-coverage truth.** Each record carries one row per governed
   recovery rung declaring one closed `recovery_rung_kind`. A corpus
   missing any of the five required rungs (`crash_loop_safe_mode`,
   `safe_mode_quarantine`, `open_without_restore`,
   `cache_index_repair`, `restricted_reopen`) narrows the record with
   `required_rung_missing`. Worked example:
   [`baseline_recovery_ladder_stable.json`](../../../fixtures/workspace/m4/recovery_ladder_lineage/baseline_recovery_ladder_stable.json).

2. **Sequence truth.** Every rung declares its canonical step ordinal
   (1: `crash_loop_safe_mode`, 2: `safe_mode_quarantine`,
   3: `open_without_restore`, 4: `cache_index_repair`,
   5: `restricted_reopen`, 6: `typed_repair_flow`,
   7: `support_export_handoff`). A mismatch narrows the record with
   `rung_sequence_unordered`.

3. **Trigger-disclosure honesty.** Every rung carries one closed
   `rung_trigger_class` (`repeated_crash_loop`,
   `extension_regression_suspected`, `resume_state_unsafe`,
   `derived_store_inconsistent`, `trust_posture_unverified`,
   `typed_repair_requested`, `support_escalation_initiated`) and a
   non-empty trigger disclosure id. A missing disclosure narrows the
   record with `rung_trigger_missing_disclosure`.

4. **No-rerun honesty.** Every rung that touches privileged or
   mutating surfaces (terminals, tasks, debug, AI apply, privileged
   extensions) must declare `explicit_user_action_required` (or
   `terminal_no_further_run` when no further run is possible) — never
   `auto_continue_after_checkpoint`. Worked example:
   [`cache_repair_auto_continue_narrowed.json`](../../../fixtures/workspace/m4/recovery_ladder_lineage/cache_repair_auto_continue_narrowed.json)
   downgrades cache-index repair to auto-continue, surfacing
   `no_rerun_posture_unsafe`. Every
   `explicit_user_action_required` rung also references a commit
   action id and a commit disclosure id; missing metadata narrows
   with `explicit_action_metadata_missing`.

5. **User-state preservation truth.** Every rung declares one closed
   `user_state_preservation_posture` (`preserved`,
   `preserved_after_export_prompt`, `dropped_with_disclosure`). Any
   posture beyond `preserved` requires an `export_before_repair`
   disclosure id; missing it narrows with
   `user_state_loss_undisclosed`.

6. **Reversibility truth.** Every rung declares one closed
   `reversibility_class` (`reversible`,
   `reversible_with_checkpoint`, `irreversible_with_disclosure`).
   `reversible_with_checkpoint` rungs must reference a rollback
   checkpoint id; irreversible rungs must reference an irreversibility
   disclosure id. Violations narrow with
   `reversibility_checkpoint_missing` or
   `irreversible_rung_missing_disclosure`.

7. **Inspection precedes destructive recovery.** The controlled
   inspection / repair hook table must be available before any
   destructive rung commits. A missing hook narrows with
   `inspection_hook_unavailable`. Worked example:
   [`missing_export_before_repair_hook_narrowed.json`](../../../fixtures/workspace/m4/recovery_ladder_lineage/missing_export_before_repair_hook_narrowed.json)
   demonstrates the narrow path when `export_before_repair` is
   unavailable.

8. **Support-export honesty.** Each rung's support-export projection
   must preserve `rung_kind`, `trigger_class`, `no_rerun_posture`,
   `user_state_preservation`, `reversibility`, `step_ordinal`, and
   `disclosure_id`, and redact raw secrets, approval tickets,
   delegated credentials, and live authority handles. Dropping a
   field narrows with `support_export_fields_dropped`; raising raw
   material narrows with `support_export_redaction_unsafe`.

9. **Producer attribution is pinnable for replay.** Each record
   carries the producer ref, the schema version, the capture
   timestamp, and an integrity hash derived from the input identities
   so replay and support pipelines can pin the source before
   applying. Incomplete attribution narrows with
   `producer_attribution_incomplete`.

10. **Lineage and export stay honest.** Every record sets
    `raw_payload_excluded = true` and carries only opaque refs to the
    source workspace, corpus, and producer. An empty workspace or
    corpus ref narrows with `lineage_export_unsafe`.

11. **The record is replay-gated.** The replay gate re-projects each
    fixture and asserts it equals the checked-in `expected`, so the
    projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                                       | Workspace state covered                              | Qualification           | Proves                                                                                                                            |
| ------------------------------------------------------------- | ---------------------------------------------------- | ----------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| `baseline_recovery_ladder_stable`                             | Five required rungs in canonical order               | `stable`                | A baseline release-branch corpus can prove the full contract                                                                      |
| `extended_with_typed_repair_and_support_handoff_stable`       | Adds typed-repair-flow + support-export-handoff      | `stable`                | The optional terminal rungs ride safely on the same projection                                                                    |
| `cache_repair_auto_continue_narrowed`                         | Cache repair downgraded to auto-continue             | `narrowed_below_stable` | The projection refuses to let a privileged rung skip explicit user action and surfaces `no_rerun_posture_unsafe`                  |
| `missing_export_before_repair_hook_narrowed`                  | `export_before_repair` hook unavailable              | `narrowed_below_stable` | The contract refuses to ship Stable when a required pre-action hook is missing                                                    |

## How to verify

```sh
# Unit + replay gate for the recovery-ladder lineage projection.
cargo test -p aureline-workspace --lib recovery_ladder_lineage
cargo test -p aureline-workspace --test recovery_ladder_lineage_replay

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_recovery_ladder_lineage -- --lines \
  fixtures/workspace/m4/recovery_ladder_lineage/baseline_recovery_ladder_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and
replay gate above. The lineage record self-describes its stable
qualification: postures that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named
reason, so they never inherit an adjacent green row. No public scope
is widened from this row.
