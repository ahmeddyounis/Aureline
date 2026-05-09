# Buffer prototype

This prototype validates the buffer-engine contract frozen in
[`docs/adr/0003-buffer-undo-large-file.md`](../../docs/adr/0003-buffer-undo-large-file.md)
and the machine-readable undo-class rows under
[`artifacts/architecture/undo_class_rows.yaml`](../../artifacts/architecture/undo_class_rows.yaml)
early enough that later editor, save-pipeline, mutation-journal, and
benchmark work can instrument against concrete hook names and a
closed class vocabulary rather than against a moving target.

It is a **prototype**, not the production buffer engine. The goal is
contract correctness (piece representation, snapshot semantics,
grouped undo/redo, class-keyed compensation postures, hook firing),
not production-grade performance.

## Where the code lives

| Piece | Path |
|---|---|
| Prototype piece-tree buffer, transactions, journal, undo/redo | [`crates/aureline-buffer/src/piece_tree/buffer.rs`](../../crates/aureline-buffer/src/piece_tree/buffer.rs) |
| Undo-class enum, compensation-posture split, class-id freeze | [`crates/aureline-buffer/src/piece_tree/class.rs`](../../crates/aureline-buffer/src/piece_tree/class.rs) |
| Protected-hot-path hook counters | [`crates/aureline-buffer/src/piece_tree/hooks.rs`](../../crates/aureline-buffer/src/piece_tree/hooks.rs) |
| Public re-exports (crate surface) | [`crates/aureline-buffer/src/lib.rs`](../../crates/aureline-buffer/src/lib.rs) |
| Bench harness (named scenarios → structural metrics) | [`crates/aureline-bench/src/buffer.rs`](../../crates/aureline-bench/src/buffer.rs) |
| Bench binary | [`crates/aureline-bench/src/bin/bench_buffer.rs`](../../crates/aureline-bench/src/bin/bench_buffer.rs) |
| Committed metrics seed | [`artifacts/buffer/buffer_metrics_seed.json`](../../artifacts/buffer/buffer_metrics_seed.json) |
| Committed undo-example traces | [`artifacts/buffer/undo_examples/`](../../artifacts/buffer/undo_examples/) |
| Bench wrapper | [`tools/bench_buffer.sh`](../../tools/bench_buffer.sh) |

## What the prototype models

- **Piece-tree representation.** An immutable original-bytes buffer,
  an append-only edit buffer, and an ordered `Vec<Piece>` describing
  the current logical sequence. The original bytes are never copied
  when the buffer is edited; the prototype swaps the `Vec<Piece>` for
  a balanced index in the production engine without changing the
  public API.
- **Snapshots as values, not locks.** `Buffer::snapshot` materialises
  the current contents into an `Arc<Vec<u8>>` and hands the caller a
  `Snapshot` carrying `(id, version, content, line_index)`. Taking a snapshot does not
  mutate observable buffer state beyond firing `snapshot_create`;
  callers can hold a snapshot while edits continue.
- **Grouped transactions.** `Buffer::begin` opens one undo group with
  a frozen `UndoClass`, a stable `originator`, and an optional
  human-readable label (required for named groups). Multiple
  `insert`/`delete`/`replace` operations inside one transaction
  commit as one journal entry with one `undo_group_id`; dropping the
  handle without calling `commit` auto-aborts and restores the
  pre-transaction piece list, total length, and append buffer.
- **Compensation postures.**
  - `Compensatable` (inverse is a forward op; redo survives a
    divergent edit): `text_edit`, `multi_cursor_text_edit`,
    `structural_edit`, `refactor_single_file`, `formatter_run`.
  - `OnlyRevertible` (inverse depends on a stored snapshot of the
    parent piece list; redo pins to snapshot lineage and is dropped
    on a divergent commit): `refactor_multi_file`,
    `save_participant_group`, `imported_change`,
    `machine_generated_change`, `migration_change`,
    `external_reload`, `decode_recovery_change`.
- **Protected-hot-path hooks.** `HookCounters` names exactly the
  hooks the ADR freezes: `buffer_open`, `text_edit_apply`,
  `transaction_apply`, `snapshot_create`, `checkpoint_create`,
  `undo_group_open`, `undo_group_close`, `undo_apply`, `redo_apply`,
  `journal_inverse_rejected`. The prototype increments; the
  production build replaces the struct with a telemetry seam behind
  the same names.
- **Inverse-cap enforcement.** `BufferConfig::inverse_cap_bytes`
  caps the stored inverse (operation bytes plus, for only-revertible
  groups, the parent-snapshot byte count). Exceeding the cap fires
  `journal_inverse_rejected`, rolls the transaction back in-place,
  and returns `BufferError::InverseTooLarge`. Matches the ADR's
  large-file rule that the journal MAY reject rather than silently
  truncate.
- **Structural metrics only.** The bench harness records counts
  (final buffer length, version, journal/redo lengths, per-hook
  counters) and no wall-clock times, so the committed seed under
  [`artifacts/buffer/buffer_metrics_seed.json`](../../artifacts/buffer/buffer_metrics_seed.json)
  is byte-stable across hosts. The benchmark lab layers timing on
  top of these counters when it scores against protected-hot-path
  budgets.

## How to run

From the repo root:

```
./tools/bench_buffer.sh
```

Defaults:

- emit: `artifacts/buffer/buffer_metrics_seed.json`
- emit-undo-examples: `artifacts/buffer/undo_examples/`

Flags:

- `--release` — build/run the release profile.
- `--emit PATH` — write the metrics JSON to a different file (pass
  `/dev/stdout` to print).
- `--emit-undo-examples DIR` — write the undo-example traces to a
  different directory; pass an empty string to skip them.

The Rust harness has its own tests (`cargo test -p aureline-bench`
and `cargo test -p aureline-buffer`); in particular, the buffer
crate tests cover single-op edits, multi-op transaction rollback on
drop, snapshot stability across edits, undo/redo across both
compensation postures, divergent-redo dropping for only-revertible
groups, repeated open/close cycles, and the inverse-cap rejection
path.

## Known holes — carried forward, not hidden in comments

These are recorded here rather than left implicit in source. Every
item below is a visible carry-forward task; none is a silent
capability of the prototype.

1. **`Vec<Piece>` is linear, not balanced.** The piece list is a
   flat vector, so `locate`, `split_at`, and `extract_range` are
   O(pieces). Fine for the smoke scenarios; the production engine
   replaces this with a balanced index (piece tree / red-black /
   rope) behind the same public API.
2. **No on-disk or memory-mapped original.** `Buffer::from_bytes`
   copies the caller's slice into an `Arc<Vec<u8>>`. Mapping a real
   file, honouring the large-file threshold, the chunked-original
   mode, and the read-through scratch cache from the ADR is
   deferred.
3. **`Snapshot::content` is a full `Arc<Vec<u8>>`.** Snapshots
   materialise the whole buffer on demand. Structural sharing (a
   snapshot referencing the piece list at a moment in time without
   copying bytes) and the COW read paths the ADR describes are
   deferred to the production snapshot store.
4. **No persistent journal.** `journal` and `redo_stack` are
   in-memory `Vec`s. Durable recovery-journal writes, rotation, and
   replay against the parent snapshot at startup are the
   mutation-journal workstream's responsibility; this prototype only
   fires `checkpoint_create` on request.
5. **No encoding negotiation.** The prototype works on raw bytes.
   BOM detection, decode negotiation, and the `decode_recovery_change`
   production path that reaches back through the IO boundary are
   modelled only as an undo-class id that the harness can label a
   transaction with.
6. **Line indexing is snapshot-scoped only.** Snapshots carry a
   newline-aware line index and grapheme-aware coordinate translation, but the
   buffer does not incrementally maintain a line index across edits, and it does
   not yet enforce an encoding boundary or preserve dominant newline mode at the
   save boundary.
7. **No multi-cursor or grapheme-safe offset validation.** The
   transaction API accepts any in-range byte offset; it does not
   reject an offset that falls inside a multi-byte UTF-8 sequence
   or inside an emoji grapheme cluster. The editor layer is
   expected to clamp offsets to grapheme boundaries using the text
   stack; the buffer trusts its caller.
8. **No coalescing policy.** The ADR names coalescing windows and
   cursor-motion boundary rules for `text_edit` typing streams. The
   prototype commits one transaction per `insert`/`delete`/`replace`
   convenience call and otherwise groups exactly what the caller
   groups with `begin`/`commit`. Coalescing is an editor-layer
   decision layered on top.
9. **Redo-after-divergence for compensatable groups is best-effort.**
   The ADR permits compensatable redo to survive a divergent edit;
   the prototype preserves the redo entry and replays the recorded
   operation at the recorded offset. Exact byte-by-byte
   reconstruction of the pre-undo state is not promised when the
   divergent edit has shifted content.
10. **No structural-share large-file mode.** The ADR's large-file
    mode (chunked original, lazy piece loading, spill-to-disk
    append buffer) is not implemented. The prototype simply reads
    all bytes into RAM.
11. **`checkpoint_create` is a counter, not durability.** The
    production build pairs checkpoint creation with a durable write
    and a pre-apply fsync; the prototype fires the hook and returns
    a monotonic handle.
12. **No accessibility / IME integration.** The buffer exposes
    nothing about active compositions. The shell spike owns IME
    composition state; wiring it to a buffer-owned composition
    region is a follow-up.
13. **`Snapshot::as_str` is all-or-nothing.** It returns `None` on
    any non-UTF-8 byte. The production snapshot reader reports a
    precise decode result and honours the active encoding.
14. **Metrics are counts only.** No wall-clock, no memory-use
    samples, no journal-write byte totals in the committed seed.
    The benchmark lab adds those in its own reproducibility pack.

## Carry-forward items (what the next wave of work picks up)

- Replace `Vec<Piece>` with a balanced piece index; keep the
  existing `Buffer` / `Snapshot` / `Transaction` surface.
- Back the original bytes with a memory-mapped file and add the
  chunked-original large-file mode.
- Wire a real mutation journal behind the `transaction_apply` /
  `checkpoint_create` hooks, with durable writes and replay.
- Add the save pipeline (pre-save participants, CRLF/EOL
  preservation, encoding round-trip), committing under
  `save_participant_group`.
- Integrate the text stack so the editor layer can present
  grapheme-safe cursor positions without the buffer needing to know
  about clusters.
- Grow the scenario table with large-file mode, encoding round-trip,
  and external-reload-with-unsaved-edits cases so the benchmark lab
  has the coverage to score against the ADR's protected budgets.
