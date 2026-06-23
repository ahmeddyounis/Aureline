# M5 frame mappings and variable/watch snapshots

This contract materializes two governed debugger object families that the
[M5 debug-contracts matrix](./m5_debug_contracts.md) names — the **frame mapping** and
the **variable/watch snapshot** — as concrete, typed, serde-serializable
[`FrameMapping`] and [`ValueSnapshot`] records, each carrying one canonical pill. It is
the canonical M5 source every debugger, notebook, replay, support, and AI surface reads
to show *which source a stack frame maps to and how trustworthy that mapping is*, and
*whether a value is a live read, a captured snapshot, a stale last-known value,
unavailable, or redacted*. Frame stacks, variables/watch panes, notebook variable
explorers, replay inspectors, and exported crashes consume these records directly
instead of re-deriving frame and value truth into rendered chrome.

Authoritative product anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.10.1–§7.6.10.4 and §9.46 on
  debug launch/session, breakpoints, variables/watches, evaluate side-effect
  governance, chronology capture, replay, and notebook-debug parity.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §14.5 on debug session headers, frame
  mapping, variables/watch panes, evaluate/REPL review, and dump/source-map truth.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` `FR-DEBUG-001` and the debug
  surface rules on stable breakpoints, variables, stack views, chronology cues, and
  artifact-linked evidence.

This lane composes with the live breakpoint/call-stack/variables/watch/evaluate truth
already frozen in
`crates/aureline-runtime/src/harden_breakpoint_call_stack_variables_watch_evaluate_and/`;
it keeps the *reviewed frame-mapping and value-snapshot model* every surface reads.

## The frame mapping

The module
[`crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs`](../../crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs)
owns the typed records. A `FrameMapping` carries:

- stable `session_id` / `thread_id` / `frame_id` plus a `frame_index` (0 is the
  innermost frame);
- a `function_label` and an optional `symbol_label`;
- a `FrameSourceLocation` — a stable `logical_source_ref` and line for a mapped frame, or
  an `artifact_ref` only for a symbol-only / unmapped frame;
- a `BuildArtifactIdentity` — the build/artifact the mapping was resolved against and its
  `BuildMatchClass` (`exact_build_verified`, `approximate_candidate`,
  `mismatched_rejected`, `no_candidate`);
- a `FrameMappingProvenance` — how the mapping was derived (`direct_source_line`,
  `source_map`, `symbol_table`, `heuristic_line_only`, `unresolved`);
- a `FrameContinuityClass` (`contiguous`, `async_resumption`, `runtime_gap`) and a
  derived `is_async_boundary`;
- explicit `is_current_frame` / `is_selected_frame` flags; and
- the canonical `FrameMappingPill` pinning one `FrameMappingFidelity` (`exact`,
  `approximate`, `symbol_only`, `unmapped`) and one build-match outcome.

The pill's flags are **derived**, never asserted:

```
shows_exact_source_link =
    fidelity.preserves_exact_source()         // exact
    && build_match.proves_exact_build()       // exact-build verified

requires_disclosure     = !shows_exact_source_link
allows_source_navigation = fidelity in {exact, approximate}
is_async_boundary       = continuity in {async_resumption, runtime_gap}
```

So a frame stack never flattens exact, approximate, symbol-only, and unresolved frames
into one generic location link: the unqualified precise source link renders only for an
exact mapping backed by an exact-build match. The pill label always discloses a
`source-map` provenance, a non-exact build match, and an async/runtime boundary.

### Frame identity, source maps, and async boundaries

- **Current-frame identity is preserved per thread.** Each thread has exactly one
  current frame, and the user-selected frame is tracked distinctly, so the frame where
  execution stopped and the frame a reader is inspecting are never collapsed.
- **A lost mapping degrades to an explicit unmapped frame.** A frame is `unmapped`
  exactly when its provenance is `unresolved`; an unresolvable frame stays visible as an
  explicit unmapped frame rather than a generic guessed location.
- **A source-map mapping always discloses.** A `source_map` provenance always sets
  `mapping_provenance_requires_disclosure` and surfaces `source-map` in the pill label,
  so a generated-source mapping is never flattened into a direct exact link.
- **An async/runtime boundary stays visible.** A frame whose caller is an async
  resumption or a runtime gap carries `is_async_boundary` and discloses it, so a
  reconstructed caller is never drawn as a contiguous native one.

## The value snapshot — one disclosure vocabulary

One `ValueSnapshot` struct materializes both the variable-snapshot and watch-snapshot
families. A `SnapshotEntryKind::Watch` entry carries a `watch_expression_digest` and the
`watch_expression` scope; every other field — scope, type/shape/size, freshness,
truncation, redaction, capture context, timestamp — is shared, so variables and watches
reuse one disclosure vocabulary. Each snapshot carries:

- a stable `snapshot_id`, a `SnapshotEntryKind` (`variable` / `watch`), and a
  `display_name`;
- a `VariableScopeClass` (`local`, `argument`, `closure`, `global`, `register`,
  `watch_expression`);
- a `SnapshotCaptureContext` — session/thread/frame, the `captured_as_of` timestamp, a
  `capture_stop_seq`, and an optional `notebook_cell_ref` or `replay_capture_ref`;
- a `TypeShapeSummary` (type name, `ValueShapeClass`, element count, size summary);
- a `ValueTruncation` (whether and why the representation was truncated);
- an opaque `value_repr_digest` (present only when a value body is present), a
  `lazy_loadable` flag, an optional `VariableUnavailableReason`, and a
  `ValueRedactionClass`; and
- the canonical `SnapshotDisclosurePill`.

The disclosure pill's flags are **derived** from the freshness state and redaction
class:

```
disclosure =
    redacted              if redaction != not_redacted   // redaction dominates
    else map(freshness):  live → live, captured_snapshot → captured,
                          stale → stale, unavailable → unavailable

is_live_read           = disclosure == live
implies_live_authority = is_live_read
requires_disclosure    = disclosure != live
value_body_present     = !is_redacted && freshness in {live, captured, stale}
```

So variables, watches, and variable explorers always say whether they are live reads,
captured snapshots, stale last-known state, unavailable, or redacted:

- a **captured** or **stale** value carries a body but never implies live authority;
- an **unavailable** value names a `VariableUnavailableReason` (`optimized_out`,
  `out_of_scope`, `not_loaded`, `target_resumed`, `evaluation_error`, `unsupported`) and
  carries no value body;
- a **redacted** value withholds its body and discloses the redaction class; and
- a **lazy-loadable** value is a live, expandable handle whose children load on demand.

### Notebook and replay reuse the snapshot vocabulary

A notebook variable explorer sets `notebook_cell_ref`; a replay inspector sets
`replay_capture_ref`. Both draw their disclosure from the same `ValueDisclosure`
vocabulary as a live-session variable, so neither invents a notebook-only or replay-only
truth.

## Contract rules (frozen invariants)

The canonical set computes each invariant's `holds` flag from the built records; an
inconsistent edit flips an invariant and fails the freeze gate.

- **`frames.one_canonical_mapping_pill`** — every frame carries one pill whose tokens
  come from the frozen vocabulary and whose flags equal their derivation.
- **`frames.fidelity_vocabulary_complete`** — exact, approximate, symbol-only, and
  unmapped are all materialized.
- **`frames.exact_link_never_hides_approximate_symbol_only_unmapped_or_mismatch`** — the
  precise source link renders only for an exact mapping backed by an exact-build match.
- **`frames.preserve_current_frame_identity_per_thread`** — each thread has exactly one
  current frame and the selected frame is tracked distinctly.
- **`frames.lost_mapping_degrades_to_explicit_unmapped`** — a frame is unmapped exactly
  when its provenance is unresolved.
- **`frames.source_map_provenance_always_disclosed`** — a source-map mapping always
  discloses and is never flattened into a direct exact link.
- **`frames.async_boundary_stays_visible`** — every frame across an async/runtime
  boundary discloses it; a contiguous frame never falsely claims one.
- **`frames.build_artifact_identity_preserved`** — every frame preserves a build/artifact
  identity, and a precise source link implies an exact-build match.
- **`snapshots.disclosure_vocabulary_complete`** — live, captured, stale, unavailable,
  and redacted are all materialized.
- **`snapshots.one_canonical_disclosure_pill`** — every snapshot carries one disclosure
  pill whose tokens come from the frozen vocabulary and whose flags equal their
  derivation.
- **`snapshots.live_authority_only_when_truly_live`** — a value implies live authority
  only when it is a true live read.
- **`snapshots.unavailable_names_reason_and_withholds_body`** — every unavailable
  snapshot names a reason and carries no value body.
- **`snapshots.redacted_withholds_value_body`** — every redacted snapshot withholds its
  body; redaction dominates freshness.
- **`snapshots.variables_and_watches_share_one_vocabulary`** — variables and watches are
  both materialized and share one disclosure vocabulary.
- **`snapshots.notebook_and_replay_reuse_snapshot_vocabulary`** — notebook explorers and
  replay inspectors reuse the shared vocabulary rather than inventing surface-only truth.
- **`set.export_retains_frame_and_value_state`** — every frame and snapshot retains its
  typed tokens and cites an export-safe proof packet.

## First consumers

- core debugger call stack, frame view, and variables/watch panes;
- notebook debug surface (per-cell frame view and variable explorer);
- profiler / trace / replay workspace and replay inspector;
- incident / crash review and symbolicated stacks;
- support export / escalation packets; and
- AI context and tool-call evidence.

## Checked-in artifacts

- Spec module:
  [`crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs`](../../crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs)
- Boundary schema:
  [`schemas/debug/m5_frame_variable_snapshots.schema.json`](../../schemas/debug/m5_frame_variable_snapshots.schema.json)
- Published fixture:
  [`fixtures/debug/m5_frame_variable_snapshots/canonical_set.json`](../../fixtures/debug/m5_frame_variable_snapshots/canonical_set.json)
- Evidence note:
  [`artifacts/debug/m5_frame_variable_snapshots.md`](../../artifacts/debug/m5_frame_variable_snapshots.md)
- Freeze gate:
  [`crates/aureline-debug/tests/m5_frame_variable_snapshots.rs`](../../crates/aureline-debug/tests/m5_frame_variable_snapshots.rs)

## Regenerating

```sh
cargo run -p aureline-debug --example dump_m5_frame_variable_snapshots \
  > fixtures/debug/m5_frame_variable_snapshots/canonical_set.json
cargo test -p aureline-debug
```

[`FrameMapping`]: ../../crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs
[`ValueSnapshot`]: ../../crates/aureline-debug/src/m5_frame_variable_snapshots/mod.rs
