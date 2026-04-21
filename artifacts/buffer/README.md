# `artifacts/buffer/`

Artifacts produced by the buffer prototype's smoke harness
(`crates/aureline-bench/src/buffer.rs`). Two families live here:

| Family | Path | Purpose |
|---|---|---|
| Metrics seed | [`buffer_metrics_seed.json`](buffer_metrics_seed.json) | Byte-stable structural-metrics record per named scenario. Reports the frozen scenario labels, per-scenario hook-counter values, and the aggregate totals. No wall-clock data. |
| Undo examples | [`undo_examples/`](undo_examples/) | Human-readable deterministic traces of specific undo/redo scenarios. One `.txt` per named example; each step records version, length, journal depth, redo depth, contents, and the full hook-counter snapshot at the end. |

Both families are produced by the same binary:

```
cargo run -p aureline-bench --bin bench_buffer -- \
    --emit artifacts/buffer/buffer_metrics_seed.json \
    --emit-undo-examples artifacts/buffer/undo_examples
```

A convenience wrapper lives at
[`tools/bench_buffer.sh`](../../tools/bench_buffer.sh) and honours
the same reproducibility posture as `tools/bench_text_stack.sh`
(pinned `SOURCE_DATE_EPOCH`, `TZ=UTC`, `LC_ALL=C`).

## Why these are checked in

- The **metrics seed** is the contract the later benchmark lab
  diffs against. A change to the scenario table, the prototype
  buffer, or the harness MUST regenerate this file in the same
  commit; a drift-check under CI compares the committed seed
  against a fresh harness run.
- The **undo examples** pin the semantics the ADR freezes so a
  reviewer can read (or diff) a single text file and see that,
  e.g., one undo reverts a multi-cursor transaction in one step,
  that a divergent edit drops only-revertible redo entries, and
  that compensatable redo-after-divergence replays the recorded
  operation rather than restoring byte-identical pre-undo state.
  These files are read by humans and by regression tests; keeping
  them small and text-diffable is the point.

## What stays out

- **No wall-clock times.** The prototype records counts only; the
  benchmark lab layers timing on top of these counters when it
  scores against the protected-hot-path budgets.
- **No host or toolchain fingerprints.** The committed seed is
  byte-stable across hosts so drifts register as real harness
  changes, not environmental noise.
- **No frame-level or GPU data.** Buffer edits and renderer frames
  are two separate journeys; the renderer spike owns its own
  artifact family under `artifacts/render/`.

## Known regeneration triggers

Regenerate this directory when any of the following change:

- `crates/aureline-buffer/src/prototype/buffer.rs`
- `crates/aureline-buffer/src/prototype/class.rs`
- `crates/aureline-buffer/src/prototype/hooks.rs`
- `crates/aureline-bench/src/buffer.rs` (scenario table or
  renderers)
- `crates/aureline-bench/src/bin/bench_buffer.rs` (CLI flags)

Do not hand-edit the `.json` seed or the `.txt` traces. They are
reproducible outputs; regenerate via the bench binary or the
wrapper script.

## Scenario labels (frozen)

The metrics seed reports one entry per label in the order below:

| Label | Undo class | Posture |
|---|---|---|
| `typing_insert_sequence_64` | `text_edit` | compensatable |
| `grouped_text_edit_word_coalesce` | `text_edit` | compensatable |
| `multi_cursor_triple_insert` | `multi_cursor_text_edit` | compensatable |
| `structural_edit_sort_lines` | `structural_edit` | compensatable |
| `refactor_single_file_rename` | `refactor_single_file` | compensatable (named group) |
| `refactor_multi_file_rename` | `refactor_multi_file` | only_revertible (named group) |
| `formatter_run_full_document` | `formatter_run` | compensatable (named group) |
| `save_participant_group_pipeline` | `save_participant_group` | only_revertible (named group) |
| `machine_generated_change_apply` | `machine_generated_change` | only_revertible (named group) |
| `external_reload_clean_buffer` | `external_reload` | only_revertible |
| `decode_recovery_resolve_override` | `decode_recovery_change` | only_revertible |
| `undo_redo_cycle_mixed_classes` | mixed | mixed |

## Undo-example labels (frozen)

| Label | File | What it shows |
|---|---|---|
| `typing_insert_sequence` | `undo_examples/typing_insert_sequence.txt` | Three one-keystroke commits; undo/redo each. |
| `multi_cursor_triple_insert` | `undo_examples/multi_cursor_triple_insert.txt` | Multi-cursor transaction reverts in one undo. |
| `refactor_single_file_rename` | `undo_examples/refactor_single_file_rename.txt` | Compensatable named group reverts and re-applies atomically. |
| `refactor_multi_file_rename` | `undo_examples/refactor_multi_file_rename.txt` | Only-revertible named group; undo restores the parent snapshot. |
| `save_participant_group_pipeline` | `undo_examples/save_participant_group_pipeline.txt` | Save + format + organise imports as one only-revertible named group. |
| `only_revertible_redo_drop` | `undo_examples/only_revertible_redo_drop.txt` | Divergent edit after undo drops the only-revertible redo entry. |
| `compensatable_redo_after_divergence` | `undo_examples/compensatable_redo_after_divergence.txt` | Divergent compensatable commit preserves the redo entry; redo replays the recorded operation. |
