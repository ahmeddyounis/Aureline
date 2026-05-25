# Finalize Piece-Tree Buffer Recovery, Undo/Redo Grouping, And Local-History Actor Lineage

This contract finalizes how Aureline keeps durable editing state trustworthy
across crashes, restores, and policy-bound workflows. It composes three existing
truth sources — the piece-tree buffer's dirty-buffer recovery journal, the
buffer's grouped undo/redo journal, and the local-history actor-lineage packet —
into one governed, export-safe lineage record per recovery posture. The record
never re-derives an outcome: it ingests each live source verbatim and adds the
proof that recovery preserves source fidelity, writes to the canonical target,
restores without re-running, and keeps lineage export-honest.

## Canonical machine sources

Do not clone status text from this doc — ingest the typed sources:

- Lineage projection and contract types:
  [`crates/aureline-editor/src/recovery_state_lineage/`](../../../crates/aureline-editor/src/recovery_state_lineage/)
- Piece-tree buffer + grouped undo/redo journal:
  [`crates/aureline-buffer/src/piece_tree/`](../../../crates/aureline-buffer/src/piece_tree/)
- Dirty-buffer crash recovery journal:
  [`crates/aureline-recovery/src/crash_journal/`](../../../crates/aureline-recovery/src/crash_journal/)
- Local-history actor-lineage packet:
  [`crates/aureline-history/src/local_history/`](../../../crates/aureline-history/src/local_history/)
- Boundary schema:
  [`schemas/editor/recovery_state_lineage.schema.json`](../../../schemas/editor/recovery_state_lineage.schema.json)
- Headless emitter / CLI:
  [`crates/aureline-editor/src/bin/aureline_recovery_state_lineage.rs`](../../../crates/aureline-editor/src/bin/aureline_recovery_state_lineage.rs)
- Fixtures:
  [`fixtures/editor/m4/recovery_state_lineage/`](../../../fixtures/editor/m4/recovery_state_lineage/)
- Replay gate:
  [`crates/aureline-editor/tests/recovery_state_lineage_replay.rs`](../../../crates/aureline-editor/tests/recovery_state_lineage_replay.rs)
- Proof packet:
  [`artifacts/workspace/m4/finalize-piece-tree-buffer-recovery-undo-redo-grouping.md`](../../../artifacts/workspace/m4/finalize-piece-tree-buffer-recovery-undo-redo-grouping.md)

## Runtime shape

The durable-state path is layered, and the lineage record never re-derives an
outcome — it ingests the live truth sources verbatim:

- The **piece-tree buffer**
  ([`crates/aureline-buffer/src/piece_tree/buffer.rs`](../../../crates/aureline-buffer/src/piece_tree/buffer.rs))
  owns versioned snapshots and a grouped undo/redo journal. Each committed group
  carries a frozen undo class, its compensation posture, an originator lane, and
  the human-readable label a named group must carry.
- The **dirty-buffer recovery journal**
  ([`crates/aureline-recovery/src/crash_journal/`](../../../crates/aureline-recovery/src/crash_journal/))
  captures, per object, the object identity, the base-on-disk token, the
  text-format posture, the capture mode and body availability, the integrity
  state, and the replay posture a crash restore would follow.
- The **local-history actor-lineage packet**
  ([`crates/aureline-history/src/local_history/`](../../../crates/aureline-history/src/local_history/))
  records who made each change and how it reverses, carrying entry, group, and
  checkpoint refs rather than raw snapshot bodies.
- The **lineage projection**
  ([`crates/aureline-editor/src/recovery_state_lineage/`](../../../crates/aureline-editor/src/recovery_state_lineage/))
  consumes all three and projects one governed `RecoveryStateLineageRecord` per
  recovery posture. It adds the source-fidelity proof, the canonical-path-truth
  proof, the restore-no-rerun proof, the undo-grouping contract, and the
  stable-qualification posture without changing any recovery decision.

The projection is read-only: it never re-runs a participant, mutates a buffer,
or widens authority.

## Source-fidelity preservation on restore

The recovery journal pins the open-time encoding, newline mode, final-newline
posture, and decoder posture. The record reports `round_trip_provable`: a restore
is source-faithful only when the encoding and newline mode are known and the
decoder kept a faithful representation (exact decode, lossy-with-raw-preserved,
or binary snapshot). A capture whose encoding or newline mode could not be
determined surfaces `round_trip_provable = false`, and the lineage narrows below
Stable with `source_fidelity_unprovable` rather than guessing.

## Canonical-path truth (no wrong-target write)

The record carries the captured object's `identity_relation` against the current
object plus the base-on-disk token and the `compare_before_write_required`
guard. It reports `wrong_target_write_guarded`:

- Exact identity and virtual-only buffers have no wrong-target risk.
- Any identity drift (alias drift, same-path-different-object, missing object,
  unknown identity) is guarded only when the save path compares before write.

A drifted target with no compare-before-write guard narrows below Stable with
`canonical_path_unproven`. A drifted target that is held for review behind a
compare-before-write guard stays Stable — that is the contract protecting the
user, not a gap.

## Restore is no-rerun

The record proves a restore applies stored bytes and never silently re-runs the
actions that produced them. It reports:

- `restore_recommended` — whether a restore is recommended or allowed at all;
- `byte_restore_faithful` — whether a faithful stored body (content-addressed
  snapshot or journal delta chain) exists to restore byte-for-byte;
- `restore_creates_new_checkpoint` — whether restoring creates a new
  local-history recovery checkpoint so the overwritten live state survives;
- `no_rerun_guaranteed` — the overall verdict.

A restore that is recommended but has no faithful body would have to reconstruct
the content by re-running, so the record narrows with `restore_would_rerun`. A
restore that overwrites live state without creating a recovery checkpoint risks
user-state loss, so it narrows with `destructive_restore_no_recovery_path`. A
posture that correctly declines restore (inspect-only, open-without-replay,
blocked) stays Stable.

## Undo/redo grouping contract

Each undo group is projected with its frozen class id, compensation posture,
originator, label, and operation count, plus:

- `redo_survives_divergence` — true for compensatable groups whose inverse is a
  forward transaction;
- `recovery_action_class` — `inverse_replay` for compensatable groups (replay the
  recorded inverse) or `snapshot_restore` for only-revertible groups (restore the
  pre-transaction snapshot);
- `grouping_integrity_ok` — a named group must carry a non-empty human-readable
  label.

A named group missing its label narrows below Stable with
`undo_grouping_contract_violation`.

## Lineage / export honesty

Every actor-lineage row is summarized with its protected actor-lineage class,
actor/source/reversal/redaction tokens, and `export_safe` verdict. A row that
exposes raw body refs (`obj:`, `raw:`, `secret:`, `token:`) or a packet that
enables raw body export narrows below Stable with `actor_lineage_export_unsafe`.
The record sets `raw_payload_excluded = true` and carries only refs, so support
export is safe by construction.

## Stable qualification (auto-narrow)

A recovery-state lineage record is Stable-qualified only when it can prove the
contract on the captured posture. It auto-narrows below Stable with a named
reason when:

- `source_fidelity_unprovable` — the encoding / newline posture cannot round-trip;
- `canonical_path_unproven` — identity drifted with no compare-before-write guard;
- `restore_would_rerun` — a restore is recommended with no faithful body to restore;
- `destructive_restore_no_recovery_path` — a restore creates no recovery checkpoint;
- `recovery_integrity_inconsistent` — integrity is unverified yet the posture still
  claims an unconditional restore;
- `undo_grouping_contract_violation` — a named undo group is missing its label;
- `actor_lineage_export_unsafe` — a row or packet is not export-safe.

Correct protective postures (a compare-before-write guard in place, a restore
held for review or blocked, an integrity failure that downgrades the replay
posture) stay Stable: the contract working as designed is a pass, not a gap.

## Consuming surfaces

`recovery_state_lineage_lines` is the single human-readable projection shared by
the editor recovery-status surface, the headless CLI emitter
(`aureline_recovery_state_lineage`), Help/About, and support export, so no
surface clones status text from another. The record excludes raw source, raw
snapshot bodies, raw patches, and content-addressed body refs, so support export
is safe by construction (`raw_payload_excluded = true`).
