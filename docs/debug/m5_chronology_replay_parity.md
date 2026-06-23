# M5 chronology capabilities, replay sessions, timeline bookmarks, and notebook-debug parity

This contract materializes three governed debugger object families that the
[M5 debug-contracts matrix](./m5_debug_contracts.md) names — the **chronology
capability**, the **replay session**, and the **notebook-debug parity** record — as
concrete, typed, serde-serializable records, each carrying one canonical support pill. It
is the canonical M5 source every live-debug, replay, notebook, profiler, AI, and support
surface reads to speak about *what time-travel and notebook-debug a backend actually
supports*, *what a replay session reconstructed and from which capture*, *where a timeline
bookmark is pinned*, and *what a restart or reconnect preserved, lost, invalidated, or left
stale* — without re-expressing debugger folklore ad hoc.

Authoritative product anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.10.1–§7.6.10.4 and §9.46 on
  debug launch/session, breakpoints, variables/watches, evaluate side-effect governance,
  chronology capture, replay, and notebook-debug parity.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §14.5 on debug session headers, frame
  mapping, variables/watch panes, evaluate/REPL review, and dump/source-map truth.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` `FR-DEBUG-001` and the debug
  surface rules on stable breakpoints, variables, stack views, chronology cues, and
  artifact-linked evidence.

This lane composes with the chronology/replay support-class truth already frozen in
`crates/aureline-debug/src/qualify_chronology_capture_and_replay_support_classes/` and the
notebook debugger bridge in
`crates/aureline-notebook/src/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/`;
it keeps the *shared chronology/replay/notebook-parity object model* every surface reads.

## One support-class vocabulary

The lane pins one [`DebugSupportClass`] vocabulary — `supported`, `limited`, `unavailable`,
`policy_blocked` — and reuses it across live debug, replay, the notebook bridge,
presentation, and support export. A surface never invents a private support label. The
broader replay-support qualification lane carries finer-grained `view_only` / `unsupported`
distinctions; this lane keeps the four product-facing classes the spec names so the same
chip reads one way everywhere.

## The support pill — derived, never asserted

Every chronology, replay, and notebook-kernel descriptor carries one
[`CapabilitySupportPill`] derived from its **own** support class and [`TimelineState`]:

```
permits_use              = support_class in {supported, limited}
time_travel_available    = permits_use && timeline_state in {recording, recorded_complete, recorded_partial, replay_active}
requires_disclosure      = support_class != supported || timeline_state in {recorded_partial, expired, mismatched, unavailable}
is_inert                 = support_class in {unavailable, policy_blocked}
is_inspect_only_timeline = timeline_state == replay_active
```

Because the pill is derived only from the descriptor's own truth, **an unsupported runtime
never inherits a neighbor's chronology or notebook-debug claim**: an `unavailable` or
`policy_blocked` backend backs zero verbs, grants no time-travel, and records no history.

## The chronology capability descriptor

A [`ChronologyCapabilityDescriptor`] carries a stable `descriptor_id`, a
[`RuntimeBackendFamily`] (`local_native`, `remote_helper`, `container`, `managed_runtime`,
`browser_runtime`, `notebook_kernel`), the support class and timeline state, the support
pill, the [`CapabilityVerb`] set it backs, a [`RecordedScope`] (`full_session`,
`since_attach`, `bounded_window`, `none`), a [`NotebookParityClass`] (`mirrored`,
`divergent`, `unsupported`, `not_applicable`), its session ref, an optional capture ref, and
a proof packet. Time-travel verbs (`reverse_step`, `reverse_continue`, `jump_to_event`,
`set_bookmark`, `jump_to_bookmark`, `inspect_historical_frame`) are backed only when the
pill says time travel is available.

## The replay session

A [`ReplaySession`] is **always inspect-only** and names the [`CaptureIdentity`] it
reconstructs — one `capture_id`, one `session_id`, one `target_id`, and the optional
exact-build artifact ref. It carries the replay verbs it backs, the chronology descriptor it
is sourced from, and an optional restart/reconnect consequence ref. A replay whose capture
no longer matches the rebuilt artifact is disclosed as `mismatched` with no replay verbs
until re-recorded.

## The timeline bookmark

A [`TimelineBookmark`] is bound to exactly one capture/session/target identity, carries an
opaque `position_digest`, a [`BookmarkKind`] (`user_set`, `auto_event`, `error_stop`), and a
reviewable label. It is built to **survive support export and restore review**, and its
capture identity must match the replay session it belongs to, so a bookmark is never
orphaned from the capture it pins.

## The notebook-kernel capability descriptor and cell-frame link

A [`NotebookKernelCapabilityDescriptor`] carries the kernel's support class, timeline state,
debug verbs (gated on support class — notebook debug does not require a recorded timeline),
its kernel ref, and the restart consequence that applies when the kernel restarts. A
[`CellFrameLink`] ties a debugger frame to a notebook cell with a [`CellLinkFidelity`]
(`exact`, `approximate`, `stale`, `unmapped`); `renders_exact_link` is **derived** — true
only when the fidelity is exact and the support class permits use — so an approximate,
stale, or unmapped link is never drawn exact.

## The restart/reconnect consequence record

A [`RestartConsequenceRecord`] names a [`ConsequenceTrigger`] (`session_restart`,
`reconnect`, `kernel_restart`, `transport_lost_reconnect`, `replay_reacquire`) and itemizes,
per [`ConsequenceSubject`], what happened with a [`ConsequenceDisposition`]:

| Subject | Meaning |
|---|---|
| `variables` | variable / scope state |
| `queued_cells` | queued / pending notebook cells |
| `debug_state` | debugger / bridge state |
| `breakpoints` | breakpoints |
| `transient_outputs` | console / stream output |

Each consequence itemizes **all five** subjects exactly once, with a disposition of
`preserved`, `lost`, `invalidated`, or `stale` and a reviewable detail — so a restart or
reconnect is **explained, never flattened into a generic banner**. Consequences exist for
notebook, debug, and replay sessions.

## Contract rules (frozen invariants)

The canonical set computes each invariant's `holds` flag from the built records; an
inconsistent edit flips an invariant and fails the freeze gate.

- **`capability.one_canonical_support_pill`** — every descriptor carries one support pill
  whose tokens come from the frozen vocabulary and whose flags equal their derivation.
- **`capability.support_class_vocabulary_complete`** — supported, limited, unavailable, and
  policy-blocked are all materialized.
- **`capability.one_shared_support_vocabulary`** — live debug, replay, and notebook reuse one
  support-class vocabulary.
- **`capability.no_inherited_claims_across_backends`** — an unavailable or policy-blocked
  backend backs no verbs, grants no time-travel, and records no history.
- **`capability.time_travel_verbs_backed_only_when_replayable`** — a time-travel verb is
  backed only when a recorded/replayable timeline supports it.
- **`replay.inspect_only_and_capture_bound`** — every replay session is inspect-only, bound
  to a full capture identity, and sourced from a chronology descriptor in the set.
- **`bookmark.bound_to_one_capture_and_survives_export`** — every bookmark is bound to one
  capture/session/target identity that matches its replay session and survives export and
  restore review.
- **`consequence.itemized_never_flattened`** — every consequence itemizes the five subjects
  exactly once.
- **`consequence.required_subjects_complete`** — every consequence explains variables, queued
  cells, debug state, breakpoints, and transient outputs.
- **`consequence.disposition_vocabulary_complete`** — preserved, lost, invalidated, and stale
  are all materialized.
- **`consequence.trigger_vocabulary_complete`** — all five triggers are materialized.
- **`consequence.covers_notebook_debug_and_replay`** — consequences exist for notebook,
  debug, and replay sessions, not just one.
- **`link.exact_only_when_exact_and_supported`** — a frame-to-cell link renders exact only
  when its mapping is exact and supported.
- **`link.fidelity_vocabulary_complete`** — exact, approximate, stale, and unmapped are all
  materialized.
- **`set.notebook_linkage_resolves`** — every link resolves to a kernel and every kernel
  resolves its restart-consequence reference.
- **`set.export_retains_capability_state`** — every record retains its typed tokens and cites
  an export-safe proof packet.

## First consumers

- core debugger chronology cues and replay workspace;
- notebook debug surface (kernel bridge, frame-to-cell linkage, restart banner);
- profiler / trace / replay workspace and replay inspector;
- incident / crash review and exported transcripts;
- support export / escalation packets; and
- AI context, composer, and tool-call evidence.

## Checked-in artifacts

- Spec module:
  [`crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs`](../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs)
- Boundary schema:
  [`schemas/debug/m5_chronology_replay_parity.schema.json`](../../schemas/debug/m5_chronology_replay_parity.schema.json)
- Published fixture:
  [`fixtures/debug/m5_chronology_replay_parity/canonical_set.json`](../../fixtures/debug/m5_chronology_replay_parity/canonical_set.json)
- Evidence note:
  [`artifacts/debug/m5_chronology_replay_parity.md`](../../artifacts/debug/m5_chronology_replay_parity.md)
- Freeze gate:
  [`crates/aureline-debug/tests/m5_chronology_replay_parity.rs`](../../crates/aureline-debug/tests/m5_chronology_replay_parity.rs)

## Regenerating

```sh
cargo run -p aureline-debug --example dump_m5_chronology_replay_parity \
  > fixtures/debug/m5_chronology_replay_parity/canonical_set.json
cargo test -p aureline-debug
```

[`DebugSupportClass`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`TimelineState`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`CapabilitySupportPill`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`ChronologyCapabilityDescriptor`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`RuntimeBackendFamily`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`CapabilityVerb`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`RecordedScope`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`NotebookParityClass`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`ReplaySession`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`CaptureIdentity`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`TimelineBookmark`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`BookmarkKind`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`NotebookKernelCapabilityDescriptor`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`CellFrameLink`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`CellLinkFidelity`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`RestartConsequenceRecord`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`ConsequenceTrigger`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`ConsequenceSubject`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
[`ConsequenceDisposition`]: ../../crates/aureline-debug/src/m5_chronology_replay_parity/mod.rs
