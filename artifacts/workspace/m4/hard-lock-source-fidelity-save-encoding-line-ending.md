# Hard-Lock Source-Fidelity Save And Save-Participant Lineage — proof packet

Reviewer-facing proof packet for the hard-locked source-fidelity save path and
the governed save-participant lineage record. This packet is the stable-line
anchor for this lane: dashboards, docs, Help/About surfaces, and support exports
should ingest the typed sources below rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-editor/src/save_fidelity_lineage/`](../../../crates/aureline-editor/src/save_fidelity_lineage/)
- Save pipeline truth source:
  [`/crates/aureline-workspace/src/save/`](../../../crates/aureline-workspace/src/save/)
- Schema:
  [`/schemas/editor/save_fidelity_lineage.schema.json`](../../../schemas/editor/save_fidelity_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-editor/src/bin/aureline_save_fidelity_lineage.rs`](../../../crates/aureline-editor/src/bin/aureline_save_fidelity_lineage.rs)
- Fixtures:
  [`/fixtures/editor/m4/save_fidelity_lineage/`](../../../fixtures/editor/m4/save_fidelity_lineage/)
- Replay gate:
  [`/crates/aureline-editor/tests/save_fidelity_lineage_replay.rs`](../../../crates/aureline-editor/tests/save_fidelity_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/hard-lock-source-fidelity-save-encoding-line-ending.md`](../../../docs/workspace/m4/hard-lock-source-fidelity-save-encoding-line-ending.md)
- Typed consumer: `aureline_editor::project_save_fidelity_lineage`

## What this packet proves

1. **Source fidelity is preserved on save.** Encoding, BOM, newline mode, and
   final-newline posture are pinned at open and re-applied before durable bytes
   are encoded. A participant whose output would change representation posture is
   held for review; a file whose open-time encoding cannot round-trip
   (`unknown_binary_like`) is surfaced as `round_trip_provable = false` and the
   lineage narrows below Stable instead of guessing.

2. **Save participants run under an explicit, provable ordering contract.** Each
   participant is pinned to a canonical hot-save-path stage (`format ->
   organize_imports -> lint -> code_action_apply -> scan ->
   validate_after_apply`). The record reports `participant_order_canonical`; an
   out-of-order pipeline narrows below Stable with
   `participant_ordering_violation`. Worked example:
   [`ordering_violation_narrowed.json`](../../../fixtures/editor/m4/save_fidelity_lineage/ordering_violation_narrowed.json).

3. **Fix actions are classified and gated by checkpoint-plus-preview.** Every
   participant fix is classified `safe_inline`, `preview_required`, `multi_file`,
   `generated_scope`, or `semantically_broad`. Anything past `safe_inline`
   requires preview and checkpoint before durable mutation; the record proves it
   with `preview_and_checkpoint_enforced`. Worked examples:
   [`whole_file_preview_held.json`](../../../fixtures/editor/m4/save_fidelity_lineage/whole_file_preview_held.json)
   (held before mutation) and
   [`generated_scope_regeneration.json`](../../../fixtures/editor/m4/save_fidelity_lineage/generated_scope_regeneration.json)
   (reviewed + committed under a checkpoint).

4. **Recovery is mapped per participant.** Each participant carries a recovery
   action: `exact_undo`, `compensation`, `regeneration`, `checkpoint_restore`, or
   `none_no_write`. The record never reports a recovery path that did not happen:
   a held or failed participant maps to `none_no_write`; a committed generated
   companion maps to `regeneration`.

5. **External-change compare semantics are surfaced, not flattened.** A save that
   races an external change halts for rebase and the record carries the
   `external_change_event_ref` and the `rebase_required` outcome rather than a
   generic success. Worked example:
   [`external_change_rebase.json`](../../../fixtures/editor/m4/save_fidelity_lineage/external_change_rebase.json).

6. **The record is export-safe and replay-gated.** Every record sets
   `raw_payload_excluded = true` and excludes raw source, patches, and tool logs.
   The replay gate re-projects each fixture and asserts it equals the checked-in
   `expected`, so the projection cannot drift without failing CI.

## Fixture corpus

| Fixture                          | Outcome                          | Qualification           | Proves                                  |
| -------------------------------- | -------------------------------- | ----------------------- | --------------------------------------- |
| `safe_inline_committed`          | `committed`                      | `stable`                | Safe-inline pipeline, exact-undo        |
| `whole_file_preview_held`        | `review_required_before_mutation`| `stable`                | Whole-file rewrite held for preview     |
| `generated_scope_regeneration`   | `committed`                      | `stable`                | Generated scope + regeneration recovery |
| `ordering_violation_narrowed`    | `safe_to_run`                    | `narrowed_below_stable` | Out-of-order participant narrowing      |
| `external_change_rebase`         | `rebase_required`                | `stable`                | External-change compare surfaced        |

## How to verify

```sh
# Unit + replay gate for the lineage projection.
cargo test -p aureline-editor save_fidelity_lineage

# Save pipeline truth source (source fidelity, risk review, compare-before-write).
cargo test -p aureline-workspace

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-editor --bin aureline_save_fidelity_lineage -- --lines \
  fixtures/editor/m4/save_fidelity_lineage/whole_file_preview_held.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and replay gate
above. The lineage record self-describes its stable qualification: surfaces that
cannot prove the contract carry `stable_qualification.level =
narrowed_below_stable` with a named reason, so they never inherit an adjacent
green row. No public scope is widened from this row.
