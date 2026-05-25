# Finalize Piece-Tree Buffer Recovery, Undo/Redo Grouping, And Local-History Actor Lineage — proof packet

Reviewer-facing proof packet for the finalized durable-state recovery lineage:
piece-tree buffer recovery, grouped undo/redo, and local-history actor lineage
composed into one governed, export-safe record per recovery posture. This packet
is the stable-line anchor for this lane: dashboards, docs, Help/About surfaces,
and support exports should ingest the typed sources below rather than cloning
this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-editor/src/recovery_state_lineage/`](../../../crates/aureline-editor/src/recovery_state_lineage/)
- Piece-tree buffer + grouped undo/redo journal:
  [`/crates/aureline-buffer/src/piece_tree/`](../../../crates/aureline-buffer/src/piece_tree/)
- Dirty-buffer crash recovery journal:
  [`/crates/aureline-recovery/src/crash_journal/`](../../../crates/aureline-recovery/src/crash_journal/)
- Local-history actor-lineage packet:
  [`/crates/aureline-history/src/local_history/`](../../../crates/aureline-history/src/local_history/)
- Schema:
  [`/schemas/editor/recovery_state_lineage.schema.json`](../../../schemas/editor/recovery_state_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-editor/src/bin/aureline_recovery_state_lineage.rs`](../../../crates/aureline-editor/src/bin/aureline_recovery_state_lineage.rs)
- Fixtures:
  [`/fixtures/editor/m4/recovery_state_lineage/`](../../../fixtures/editor/m4/recovery_state_lineage/)
- Replay gate:
  [`/crates/aureline-editor/tests/recovery_state_lineage_replay.rs`](../../../crates/aureline-editor/tests/recovery_state_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/finalize-piece-tree-buffer-recovery-undo-redo-grouping.md`](../../../docs/workspace/m4/finalize-piece-tree-buffer-recovery-undo-redo-grouping.md)
- Typed consumer: `aureline_editor::project_recovery_state_lineage`

## What this packet proves

1. **Source fidelity survives restore.** The captured encoding, newline mode,
   final-newline posture, and decoder posture are pinned, and the record reports
   `round_trip_provable`. A capture whose encoding or newline mode cannot be
   determined surfaces `round_trip_provable = false` and narrows below Stable
   with `source_fidelity_unprovable` instead of guessing. Worked example:
   [`unknown_encoding_unprovable_narrowed.json`](../../../fixtures/editor/m4/recovery_state_lineage/unknown_encoding_unprovable_narrowed.json).

2. **Restore writes to the canonical target.** The record carries the captured
   object's `identity_relation`, the base-on-disk token, and the
   `compare_before_write_required` guard, and reports `wrong_target_write_guarded`.
   Identity drift with no guard narrows below Stable with `canonical_path_unproven`;
   the same drift behind a compare-before-write guard stays Stable. Worked
   examples:
   [`wrong_target_unguarded_narrowed.json`](../../../fixtures/editor/m4/recovery_state_lineage/wrong_target_unguarded_narrowed.json)
   (narrowed) and
   [`compare_guard_protective_stable.json`](../../../fixtures/editor/m4/recovery_state_lineage/compare_guard_protective_stable.json)
   (protective, Stable).

3. **Restore is no-rerun.** A restore applies stored bytes under a recovery
   checkpoint; it never silently re-runs the actions that produced them. A
   recommended restore with no faithful body narrows below Stable with
   `restore_would_rerun`; a restore that creates no recovery checkpoint narrows
   with `destructive_restore_no_recovery_path`. Worked example:
   [`generated_rerun_narrowed.json`](../../../fixtures/editor/m4/recovery_state_lineage/generated_rerun_narrowed.json).

4. **Undo/redo grouping is contract-bound.** Each group carries its frozen class,
   compensation posture, originator, label, and operation count, plus
   `redo_survives_divergence`, a `recovery_action_class` (`inverse_replay` or
   `snapshot_restore`), and `grouping_integrity_ok`. A named group missing its
   label narrows below Stable with `undo_grouping_contract_violation`. Worked
   example:
   [`unlabeled_named_group_narrowed.json`](../../../fixtures/editor/m4/recovery_state_lineage/unlabeled_named_group_narrowed.json).

5. **Lineage and export stay honest.** Every actor-lineage row carries an
   `export_safe` verdict; a row that leaks raw body refs or a packet that enables
   raw body export narrows below Stable with `actor_lineage_export_unsafe`. Every
   record sets `raw_payload_excluded = true` and excludes raw source, raw
   snapshot bodies, raw patches, and content-addressed body refs.

6. **The record is replay-gated.** The replay gate re-projects each fixture and
   asserts it equals the checked-in `expected`, so the projection cannot drift
   without failing CI.

## Fixture corpus

| Fixture                               | Replay posture            | Qualification           | Proves                                  |
| ------------------------------------- | ------------------------- | ----------------------- | --------------------------------------- |
| `clean_restore_stable`                | `restore_allowed`         | `stable`                | All three pillars proven                |
| `compare_guard_protective_stable`     | `restore_requires_review` | `stable`                | Identity drift guarded = protective     |
| `unknown_encoding_unprovable_narrowed`| `open_without_replay_default` | `narrowed_below_stable` | Source fidelity unprovable          |
| `wrong_target_unguarded_narrowed`     | `restore_allowed`         | `narrowed_below_stable` | Wrong-target write risk                 |
| `generated_rerun_narrowed`            | `restore_requires_review` | `narrowed_below_stable` | Restore would rerun                     |
| `unlabeled_named_group_narrowed`      | `restore_allowed`         | `narrowed_below_stable` | Undo grouping contract violation        |

## How to verify

```sh
# Unit + replay gate for the recovery-state lineage projection.
cargo test -p aureline-editor recovery_state_lineage

# Truth sources (piece-tree buffer + undo/redo, crash recovery, local history).
cargo test -p aureline-buffer
cargo test -p aureline-recovery
cargo test -p aureline-history

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-editor --bin aureline_recovery_state_lineage -- --lines \
  fixtures/editor/m4/recovery_state_lineage/generated_rerun_narrowed.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and replay gate
above. The lineage record self-describes its stable qualification: surfaces that
cannot prove the contract carry `stable_qualification.level =
narrowed_below_stable` with a named reason, so they never inherit an adjacent
green row. No public scope is widened from this row.
