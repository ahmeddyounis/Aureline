# Large-file path — prototype notes

These notes pin down what the
[`aureline-largefile-proto`](../../crates/aureline-largefile-proto/) crate
models, what it does NOT model, and the exact handoff a later
large-file UX / save-pipeline / overlay-edit workstream picks up.

The hook vocabulary, switch-trigger order, and capability split
are normative and lifted verbatim from
[ADR 0003 — Buffer, undo, and large-file mode](../adr/0003-buffer-undo-large-file.md).
The prototype MUST NOT introduce synonyms; later work consumes the
same names without renaming.

The prototype lives behind its own crate so the editable
piece-tree path in
[`aureline-buffer`](../../crates/aureline-buffer/) is never
implicated by a large-file rule. A file routed to large-file mode
never builds a piece-tree at all; an editable transaction is the
contract this mode reduces or denies.

## Switch trigger evaluation

Classification runs once at open time on a bounded sniff prefix.
Triggers are evaluated in the ADR-frozen order; the first match
wins, and later triggers do NOT promote past an earlier hit:

1. `size_threshold` — on-disk size exceeds the policy threshold
   (default `100 MiB`).
2. `resource_pressure` — estimated buffer footprint
   (`2 * file_size`) exceeds the soft RSS budget. The prototype
   models pressure with a fixed budget; production swaps in a
   real resource-manager probe behind the same trigger id.
3. `classification` — sniff finds a NUL byte, an above-threshold
   non-printable per-mille ratio, an above-threshold longest line
   (minified heuristic), or the path matches a workspace
   "large-file pack" suffix (`.min.js`, `.min.css`, `.min.map`,
   `.bundle.js`, `.wasm` by default).
4. `decode_posture` — a previous decode failed and the user chose
   "open in large-file mode" from the decode-recovery surface.
5. `operator_override` — the user explicitly opened the file in
   large-file mode.

Every decision rides with a [`ClassificationDecision`] that carries
the trigger, a human-readable reason string, and a
[`SniffSummary`] of counts (bytes inspected, NUL presence,
longest line, BOM, non-printable ratio, heuristic conclusions). A
support bundle can replay why the mode applied without re-reading
the file.

## Bounded-memory backing store

The reader is page-based: the file is sliced into fixed-size
pages, an LRU keeps at most `max_resident_pages` resident, and
older pages drop out as the cursor moves through the file.

| Knob | Default | What it controls |
|---|---|---|
| `page_size` | 64 KiB | Stride per page read. |
| `max_resident_pages` | 4 | Hard cap on page bodies in the LRU. |

Resident bytes are bounded at `page_size * max_resident_pages`
regardless of file length. The bench harness asserts this
invariant against the high-water mark every scenario records.

The ADR specifies "an mmap-backed paged reader". The prototype
implements the read half with stdlib `File` + `Seek` + `Read` so
the workspace's `unsafe_code = "deny"` lint stays honest;
production swaps the reader for an mmap-backed implementation
behind the same `read_range` / `find_first` surface without
changing consumers. `read_range` walks pages one at a time and
copies only the requested slice from each page; `find_first`
streams the file with an overlap window equal to `needle.len() - 1`
so a match spanning two pages is caught.

## What stays, what is reduced, what is denied

The capability split is one reviewable constant per mode in
[`crates/aureline-largefile-proto/src/capabilities.rs`](../../crates/aureline-largefile-proto/src/capabilities.rs).
Both tables MUST cover the same identifier set so a diff between
the two columns is the contract.

| Capability id | Normal mode | Limited (large-file) mode |
|---|---|---|
| `view` | allowed | allowed (read-only viewport rendering) |
| `search_viewport` | allowed | allowed (viewport-bounded) |
| `search_whole_file` | allowed | downgraded (streaming-search variant only) |
| `copy` | allowed | allowed (selection copy remains) |
| `diagnostics_viewport` | allowed | allowed |
| `diagnostics_whole_file` | allowed | denied |
| `multi_cursor_viewport` | allowed | allowed |
| `multi_cursor_whole_file` | allowed | denied |
| `full_file_format_on_save` | allowed | denied |
| `range_format_on_save` | allowed | allowed |
| `full_file_reflow` | allowed | denied (viewport-bounded reflow remains) |
| `viewport_reflow` | allowed | allowed |
| `whole_file_syntax_parse` | allowed | denied (viewport-bounded variants remain) |
| `viewport_syntax_parse` | allowed | allowed |
| `indexing` | allowed | denied |
| `background_analysis` | allowed | denied |
| `cursor_local_lookup` | allowed | allowed |
| `whole_file_load_into_ram` | allowed | denied (paged reader only) |
| `rich_refactor_single_file` | allowed | denied |
| `rich_refactor_multi_file` | allowed | denied |
| `ai_apply_range` | allowed | downgraded (range-only) |
| `ai_apply_whole_file` | allowed | denied |
| `save_participant_range_only` | allowed | allowed |
| `save_participant_whole_file` | allowed | denied |
| `undo_redo_history_full` | allowed | downgraded (tighter coalescing + per-buffer inverse cap) |
| `accessibility_tree_viewport` | allowed | allowed |

Adding or moving a row is an ADR amendment, not a silent code
change.

## Save fallback

`LargeFileBuffer::attempt_save` is the lane's pre-check before it
kicks off the save pipeline:

| Save shape | Normal mode | Limited mode |
|---|---|---|
| `EditedRangeOnly` | accepted | accepted |
| `WithRangeOnlyParticipants` | accepted | accepted |
| `WithWholeFileParticipants` | accepted | denied |

The denial is explicit — the buffer returns
`SaveOutcome::Denied { reason }`; the lane MUST drop whole-file
participants or fall back to a range-only save. There is no
silent demotion.

## Hook vocabulary

The prototype counts the ADR-frozen large-file hooks. Production
replaces the counter struct with a real telemetry seam behind the
same names; lanes that observe these hooks today never need to
rename when production lands.

| Hook | When it fires |
|---|---|
| `buffer_open` | One per successful open. |
| `large_file_mode_enter` | When the buffer enters large-file mode. |
| `large_file_mode_exit` | When the buffer leaves large-file mode (operator downgrade). |
| `save_participant_rebase` | When a save participant rebases or aborts because the on-disk file changed mid-flight. |
| `journal_inverse_rejected` | When the journal refuses a transaction whose stored inverse exceeds the per-buffer cap. |
| `paged_read` | One per page touched by a paged read. Observability only. |
| `classification_recorded` | Once per recorded classification decision. Observability only. |

## Edit-attempt vocabulary

`LargeFileBuffer::attempt_edit(EditRequest)` consults the
capability table and returns one of:

- `EditOutcome::Accepted { capability_id }` — the lane may
  proceed.
- `EditOutcome::Denied { capability_id, reason }` — the lane MUST
  abort the request and surface `reason` (the same string the UX
  banner reads).
- `EditOutcome::Downgraded { capability_id, reason }` — the lane
  routes through the narrower variant named in `reason`.

The prototype does not commit edits even for accepted requests;
the write half belongs to the production overlay. The point of
the surface is that lanes can pre-check an action against the
same table the UX banner reads.

## Smoke harness and metrics seed

[`aureline-largefile-proto`](../../crates/aureline-largefile-proto/)
ships a smoke harness with one scenario per switch trigger plus
one normal-mode control. Each scenario:

- writes its embedded fixture to a scratch directory,
- opens it under a config that tightens the trigger of interest,
- walks the file through the paged reader,
- exercises the limited-mode capability matrix,
- records the resulting structural snapshot (counts only).

The bench binary emits the aggregate as JSON to
[`artifacts/bench/large_file_proto_metrics.json`](../../artifacts/bench/large_file_proto_metrics.json).
Counts only — no wall-clock data — so the seed is byte-stable
across hosts.

| Scenario label | Fixture | Trigger | Outcome |
|---|---|---|---|
| `normal_mode_clean_small_text` | `clean_small_text.txt` | none | NORMAL mode; full capability surface. |
| `trigger_size_threshold` | `above_threshold_text.txt` | `size_threshold` | LIMITED via tightened threshold. |
| `trigger_classification_null_byte` | `null_byte_blob.bin` | `classification` (binary) | LIMITED via NUL in sniff. |
| `trigger_classification_minified_long_line` | `minified_long_line.js` | `classification` (minified) | LIMITED via above-threshold longest line. |
| `trigger_classification_pack_suffix` | `pack_suffix_clean.min.js` | `classification` (pack rule) | LIMITED via path-suffix rule. |
| `trigger_decode_posture` | `decode_recovery_target.txt` | `decode_posture` | LIMITED via decode-recovery surface. |
| `trigger_operator_override` | `operator_override_target.txt` | `operator_override` | LIMITED via explicit user choice. |

Run via the wrapper script:

```
./tools/largefile_proto.sh
```

Defaults:

- emit: `artifacts/bench/large_file_proto_metrics.json`

The wrapper pins the reproducibility posture (`SOURCE_DATE_EPOCH`,
`TZ=UTC`, `LC_ALL=C`) so reruns are byte-identical.

## What stays out of the prototype

Recorded here so they are visible carry-forward items, not
silent capabilities of the prototype.

- **Real mmap.** The reader uses `File::seek` + `Read`; the
  workspace forbids `unsafe_code` so the prototype cannot call
  `mmap` directly. The ADR allows "mmap or paged reads"; the
  paged reader wins for the prototype and the `read_range` /
  `find_first` surface is the seam an mmap-backed swap-in
  consumes.
- **Editable overlay.** `attempt_edit` consults the capability
  table; it does not commit. The production write overlay (the
  paged-rope-class write buffer the ADR names) lands separately
  with its own journal-cap behaviour.
- **Durable mutation journal.** `journal_inverse_rejected` is
  named here as a counter; the production journal owns durability,
  rotation, and replay against the parent snapshot.
- **Decode-recovery UX.** The `decode_posture` trigger reads a
  policy flag set by the harness. The decode-recovery surface
  itself (re-decode, choose encoding, choose "open in large-file
  mode") lands with the editor / status-bar work.
- **Resource-pressure probe.** `resource_pressure` is modelled as
  `2 * file_size > soft_rss_budget` against a fixed budget.
  Production replaces the constant with a real probe behind the
  same trigger id.
- **`large_file_mode_exit` semantics.** Counted but never fired
  by the prototype; leaving large-file mode is an operator-
  downgrade flow that lands with the editor.
- **Multi-GiB inputs.** The prototype's switch conditions fire
  via tightened thresholds, not via real 100+ MiB files, so the
  repo stays small. The recipe — "any file above the workspace
  policy threshold" — is named in the corresponding scenario.

## Handoff to later workstreams

A later large-file UX / save-pipeline / overlay-edit workstream
picks up against the frozen surface this prototype establishes:

- **UX banner content.** The capability table's `note` field is
  the banner text. Keep the table the source of truth; do not
  inline the strings in the editor.
- **Edit gating.** Every editor lane that mutates a buffer routes
  through `attempt_edit` (or its production successor) before
  starting work. Adding a new edit kind requires adding a row to
  the capability table in the same change.
- **Save pipeline.** The save pipeline calls `attempt_save` first
  and either honours an `Accepted` outcome or drops whole-file
  participants on `Denied`.
- **Mutation journal.** The journal owns the `journal_inverse_rejected`
  hook today as a counter; the production journal makes it
  durable and ties it to the per-buffer inverse cap.
- **Mmap swap-in.** The mmap-backed reader replaces `paged.rs` in
  place; the `LargeFileBuffer` surface does not change. The
  bounded-memory invariants the prototype tests carry over.
- **Decode-recovery surface.** The status-bar / banner work that
  routes a decode failure to "open in large-file mode" sets the
  policy flag the prototype already consults; no new trigger id
  is needed.
