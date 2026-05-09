# Piece-tree buffer contract

This document is the reviewer-facing contract for Aureline’s canonical editor
buffer: a piece-tree text core with revisioned snapshots and a grouped
undo/redo journal.

Normative design sources:

- `docs/adr/0003-buffer-undo-large-file.md` (buffer model, snapshot semantics,
  undo taxonomy, source-fidelity constraints).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §11 (text model) and
  §12.4.2/§12.4.3 (checkpoint + mutation lineage posture).

## Scope

In scope:

- the stable public surface for a piece-tree buffer (`Buffer`, `Snapshot`,
  transactions, undo classes, hook counters);
- revision and snapshot identifiers suitable for save, search, background
  analysis, and history consumers;
- newline-aware line indexing and grapheme-aware coordinate translation over a
  snapshot.

Out of scope:

- large-file backing store and mmap/streaming readers;
- durable recovery journal persistence and rotation;
- save encoding/newline preservation (handled at the IO boundary);
- incremental line-index maintenance and a balanced piece index.

## Canonical surface

The canonical implementation lives under:

- `crates/aureline-buffer/src/piece_tree/`

The crate root re-exports the core types so downstream crates depend on
`aureline-buffer` rather than internal module paths:

- `crates/aureline-buffer/src/lib.rs`

## Revision and snapshot semantics

- Every committed mutation advances a per-buffer monotonic revision counter.
  The current revision is surfaced via `Buffer::revision_id()` and
  `Buffer::version()`.
- `Snapshot` is a value, not a lock. Holding a snapshot does not block future
  edits.
- `SnapshotId` is a monotonic identifier allocated when a snapshot value is
  created (by an explicit `Buffer::snapshot()` call or by commit/undo/redo
  producing a new head snapshot).
- A snapshot carries both:
  - `Snapshot::id()` — snapshot identity; and
  - `Snapshot::version()` / `Snapshot::revision_id()` — the buffer revision
    that the snapshot represents.

Undo/redo:

- `undo()` and `redo()` advance the revision counter and produce a new head
  snapshot id.

## Line indexing and coordinate translation

`Snapshot` owns an immutable line index computed over its bytes:

- `Snapshot::line_index()`
- `Snapshot::line_count()`
- `Snapshot::line_span(line)`
- `Snapshot::line_str(line)` (UTF-8 only)

Newline handling:

- `\n` (LF), `\r\n` (CRLF), and `\r` (CR) terminate lines.
- Line spans exclude terminator bytes.
- A trailing terminator produces a final empty line.

Coordinate translation is snapshot-scoped:

- `Snapshot::byte_offset_for_line_grapheme(line, grapheme)` maps a
  grapheme-aware `(line, column)` to a byte offset (clamped to line end when the
  requested column exceeds the line length).
- `Snapshot::line_grapheme_for_byte_offset(offset)` maps a byte offset to
  `(line, grapheme)` coordinates (clamped to the preceding line when the offset
  falls on a line terminator).

Downstream consumers should prefer these APIs to re-deriving line starts and
grapheme boundaries independently.

## Consumer wiring (protected path)

The native shell’s editor session uses the piece-tree buffer snapshot and line
index to keep the keystroke→mutation path truthful:

- `crates/aureline-shell/src/bootstrap/native_shell.rs`

The structural bench harness continues to exercise the same public surface:

- `crates/aureline-bench/src/buffer.rs`

