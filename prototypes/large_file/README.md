# Large-file path prototype

This prototype validates the reduced-capability large-file mode
frozen in
[`docs/adr/0003-buffer-undo-large-file.md`](../../docs/adr/0003-buffer-undo-large-file.md)
and the reviewable evaluation order, capability split, and hook
vocabulary it names — early enough that later editor, save-pipeline,
mutation-journal, AI-apply, and benchmark-lab work can instrument
against concrete trigger names and a closed capability table rather
than against a moving target.

It is a **prototype**, not the production large-file backing store.
The goal is contract correctness (trigger order, bounded-memory
reads, capability denials, save fallback, hook firing) — not
production-grade performance, not real mmap, not an editable write
overlay.

## Where the code lives

| Piece | Path |
|---|---|
| Classification (trigger vocabulary, sniff, decision) | [`crates/aureline-largefile-proto/src/classification.rs`](../../crates/aureline-largefile-proto/src/classification.rs) |
| Bounded-memory paged reader (LRU, metrics, streaming search) | [`crates/aureline-largefile-proto/src/paged.rs`](../../crates/aureline-largefile-proto/src/paged.rs) |
| Capability split between normal and limited modes | [`crates/aureline-largefile-proto/src/capabilities.rs`](../../crates/aureline-largefile-proto/src/capabilities.rs) |
| Large-file hook counters | [`crates/aureline-largefile-proto/src/hooks.rs`](../../crates/aureline-largefile-proto/src/hooks.rs) |
| `LargeFileBuffer` surface (open, attempt_edit, attempt_save, snapshot) | [`crates/aureline-largefile-proto/src/buffer.rs`](../../crates/aureline-largefile-proto/src/buffer.rs) |
| Smoke harness (named scenarios → structural metrics) | [`crates/aureline-largefile-proto/src/harness.rs`](../../crates/aureline-largefile-proto/src/harness.rs) |
| Bench binary | [`crates/aureline-largefile-proto/src/bin/largefile_proto.rs`](../../crates/aureline-largefile-proto/src/bin/largefile_proto.rs) |
| Public re-exports (crate surface) | [`crates/aureline-largefile-proto/src/lib.rs`](../../crates/aureline-largefile-proto/src/lib.rs) |
| Embedded fixtures + index | [`fixtures/text/large/`](../../fixtures/text/large/) |
| Committed metrics seed | [`artifacts/bench/large_file_proto_metrics.json`](../../artifacts/bench/large_file_proto_metrics.json) |
| Architecture notes | [`docs/architecture/large_file_prototype_notes.md`](../../docs/architecture/large_file_prototype_notes.md) |
| Smoke wrapper | [`tools/largefile_proto.sh`](../../tools/largefile_proto.sh) |

## What the prototype models

- **At-open classification.** Every file is classified once at
  open time. The first matching trigger in the ADR-frozen order
  wins (`size_threshold` → `resource_pressure` → `classification`
  → `decode_posture` → `operator_override`); the decision rides
  with a human-readable reason and a counts-only sniff summary so
  a support bundle can replay why the mode applied.
- **Reviewable capability split.** One constant table per mode in
  `capabilities.rs`; both tables MUST cover the same identifier
  set, so a diff between the two columns is the contract. Adding
  or moving a row is an ADR amendment, not a silent code change.
- **Bounded-memory reader.** Pages are slices of `page_size`
  bytes; an LRU keeps at most `max_resident_pages` resident.
  Resident bytes are bounded at `page_size * max_resident_pages`
  regardless of file length; the harness asserts this invariant
  per scenario.
- **Streaming search.** `find_first` walks pages with an overlap
  window equal to `needle.len() - 1` so a match spanning two
  pages is caught. Needles bigger than one page are refused; the
  production swap-in raises the bound behind the same surface.
- **Edit-attempt gating.** `LargeFileBuffer::attempt_edit` looks
  up the request's capability id and returns `Accepted`,
  `Denied`, or `Downgraded` with the same `note` text the UX
  banner reads. Every editor lane that mutates a buffer is
  expected to pre-check via this surface (or its production
  successor) before starting work.
- **Save fallback.** `attempt_save` denies whole-file save
  participants in limited mode and accepts edited-range-only or
  range-only-participants saves in any mode. There is no silent
  demotion; the lane MUST honour the denial.
- **Protected-hot-path hooks.** `HookCounters` names exactly the
  hooks the ADR freezes for the large-file path: `buffer_open`,
  `large_file_mode_enter`, `large_file_mode_exit`,
  `save_participant_rebase`, `journal_inverse_rejected`, plus
  observability-only `paged_read` and `classification_recorded`.
  The prototype increments; the production build replaces the
  struct with a telemetry seam behind the same names.
- **Structural metrics only.** The bench harness records counts
  (mode, trigger, page count, hook counters, reader metrics,
  per-step capability outcomes) and no wall-clock times, so the
  committed seed under
  [`artifacts/bench/large_file_proto_metrics.json`](../../artifacts/bench/large_file_proto_metrics.json)
  is byte-stable across hosts. The benchmark lab layers timing on
  top when it scores against the protected-hot-path budgets the
  ADR freezes.

## How to run

From the repo root:

```
./tools/largefile_proto.sh
```

Defaults:

- emit: `artifacts/bench/large_file_proto_metrics.json`

Flags:

- `--release` — build/run the release profile.
- `--emit PATH` — write the metrics JSON to a different file
  (pass `/dev/stdout` to print).
- `--scratch-dir DIR` — materialise the embedded fixtures into
  `DIR` instead of a fresh temp directory; the harness will not
  remove the directory when this is set.

The Rust crate has its own tests: `cargo test
-p aureline-largefile-proto` covers classification trigger order,
sniff heuristics, BOM detection, paged-reader bounded-memory and
cache-hit / spanning-search invariants, capability-table id
alignment, save-fallback split, and full-harness byte-stability
across reruns.

## Known holes — carried forward, not hidden in comments

These are recorded here rather than left implicit in source.
Every item below is a visible carry-forward task; none is a
silent capability of the prototype.

1. **No real mmap.** The reader uses `File::seek` + `Read`; the
   workspace forbids `unsafe_code` so the prototype cannot call
   `mmap` directly. The ADR allows "mmap or paged reads"; the
   paged reader wins for the prototype and the `read_range` /
   `find_first` surface is the seam an mmap-backed swap-in
   consumes.
2. **No editable write overlay.** `attempt_edit` consults the
   capability table but does not commit any edit even for
   accepted requests. The production paged-rope-class write
   buffer the ADR names lands separately, with its own
   journal-cap behaviour.
3. **No durable mutation journal.** `journal_inverse_rejected`
   is named here as a counter; the production journal owns
   durability, rotation, and replay against the parent snapshot.
4. **Resource-pressure trigger is constant-driven.** The
   prototype models pressure as `2 * file_size > soft_rss_budget`
   against a fixed budget. Production replaces the constant with
   a real resource-manager probe behind the same trigger id.
5. **Decode-recovery surface is out of scope.** The
   `decode_posture` trigger reads a policy flag set by the
   harness. The status-bar / banner work that routes a decode
   failure to "open in large-file mode" lands with the editor
   workstream and sets the flag the prototype already consults.
6. **`large_file_mode_exit` is counted but never fired.**
   Leaving large-file mode is an operator-downgrade flow that
   lands with the editor; the hook is named here so lanes do not
   invent synonyms.
7. **`save_participant_rebase` is counted but never fired.**
   The save pipeline owns concurrent-edit rebase; the hook is
   named here for the same reason.
8. **Sniff is a single bounded prefix.** The default 64 KiB
   sniff window catches NULs, BOMs, and most minified single
   lines, but a file with non-pathological content followed by
   binary tail bytes will not trip `classification` until
   production adds a periodic re-sniff (or the file crosses
   `size_threshold` first).
9. **Pack-suffix table is a default list.** The five suffixes
   in `ClassificationPolicy::default()` are the prototype's
   default policy; production reads the workspace policy at
   open time. The trigger id (`classification`) does not change.
10. **No multi-GiB inputs.** The prototype's switch conditions
    fire via tightened thresholds, not via real 100+ MiB files,
    so the repo stays small. The recipe — "any file above the
    workspace policy threshold" — is named in the corresponding
    scenario.
11. **No CRLF / line-ending awareness.** The reader walks bytes
    and the sniff counts LF or CR as line breaks; line/column
    mapping, CRLF preservation, and the ADR's CRLF-preserving
    save rule belong to the save pipeline.
12. **No encoding negotiation beyond BOM detection.** The
    prototype reports `BomKind` but does not transcode. Encoding
    round-trip and the production decode-recovery path land with
    the IO boundary work.
13. **`LargeFileBuffer::attempt_edit` is the only edit surface.**
    There is no transaction handle, no `insert`/`delete`/`replace`,
    no undo group; every mutation goes through capability
    pre-check. Production layers the write overlay on top.
14. **No reflow, syntax parse, indexing, or accessibility tree.**
    The capability table denies these in limited mode; the
    prototype does not implement them at all. Each capability id
    is the seam the corresponding workstream branches on.
15. **Metrics are counts only.** No wall-clock, no memory-use
    samples, no per-page latency in the committed seed. The
    benchmark lab adds those in its own reproducibility pack.

## Carry-forward items (what the next wave of work picks up)

- Replace the paged reader with an mmap-backed implementation
  behind the same `read_range` / `find_first` surface.
- Land the production write overlay (paged-rope-class write
  buffer with the per-buffer inverse cap and `journal_inverse_rejected`
  wired to a real durable journal).
- Wire the decode-recovery UX surface so the `decode_posture`
  trigger fires from a real user choice, not a policy flag set
  by tests.
- Add a real resource-manager probe behind the
  `resource_pressure` trigger id so the trigger reflects live
  conditions.
- Grow the scenario table with cases where a file crosses
  `size_threshold` mid-edit, where the operator downgrades out
  of large-file mode, and where the save participant rebases —
  so the benchmark lab has the coverage to score against the
  ADR's protected budgets across the full mode lifecycle.
- Land the UX banner that quotes the capability table's `note`
  field verbatim.
- Read the workspace policy (size threshold, sniff window,
  pack-suffix table, soft RSS budget) at open time instead of
  taking the prototype's defaults.
