# M5 breakpoint specs and mapping-state pills

This contract materializes the **breakpoint spec** — one of the governed debugger
object families that the [M5 debug-contracts matrix](./m5_debug_contracts.md) names —
as concrete, typed, serde-serializable [`BreakpointSpec`] records, each carrying one
canonical [`BreakpointPill`]. It is the canonical M5 source every debugger-capable
surface reads to show *what a breakpoint requested, where it actually bound, and
whether its source mapping is still trustworthy*. Gutters, session headers,
breakpoint lists, notebook cells, replay timelines, and support exports consume these
specs directly instead of re-deriving breakpoint truth into rendered chrome.

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
it keeps the *reviewed breakpoint model* that producer emits and every surface reads.

## The breakpoint spec

The module
[`crates/aureline-debug/src/m5_breakpoint_specs/mod.rs`](../../crates/aureline-debug/src/m5_breakpoint_specs/mod.rs)
owns the typed record. A `BreakpointSpec` carries:

- a stable `breakpoint_id` and a `BreakpointKindClass` (`line`, `conditional`,
  `logpoint`, `function`, `data`, `exception`);
- the `BreakpointEnablement` (`enabled` / `disabled`);
- a `BreakpointSourceAnchor` — the **stable `logical_source_ref`** that survives a
  rename, an opaque `physical_path_hint`, and the `line` / span;
- an optional `NotebookCellAnchor` (notebook scope) and an optional `ReplayFrameAnchor`
  (replay scope), so cell and frame identity are never collapsed;
- a `BreakpointPayload` — condition / log-message presence flags plus opaque digests,
  and a structured `hit_condition`, never the raw expression bodies;
- the `BreakpointScopeClass` (`workspace_source`, `session_local`, `notebook_cell`,
  `replay_timeline`, `exception_category`);
- the `BreakpointMappingProvenance` — how the current mapping was derived; and
- the canonical `BreakpointPill` that pins one verification state and one mapping
  state.

## The pill — one verification + mapping vocabulary

Every breakpoint carries exactly one [`BreakpointPill`]. The pill is the single thing
a gutter, a list, a session header, a notebook cell, a replay timeline, and an export
packet all render, so a breakpoint shown anywhere traces back to one spec and one
state vocabulary. It pins:

- **Verification state** (`BreakpointVerificationState`) — `pending`, `verified`,
  `unbound`, `unsupported`, `policy_blocked`. Only `verified` is a confirmed binding.
- **Mapping state** (`BreakpointMappingState`) — `exact`, `misaligned`, `needs_remap`,
  `unmapped`. Only `exact` preserves the requested location; `needs_remap` is the
  explicit degrade-rather-than-vanish state.

The pill's flags are **derived**, never asserted:

```
shows_clean_confirmed =
    verification.is_bound()                 // verified
    && mapping.preserves_exact_location()   // exact
    && !scope.is_replay_only()              // not a replay timeline

requires_disclosure   = !shows_clean_confirmed
needs_explicit_remap  = mapping == needs_remap
is_replay_only        = scope == replay_timeline
```

So the unqualified green confirmed-stop icon (`shows_clean_confirmed`) renders only for
a verified, exact, non-replay breakpoint. An unbound, misaligned, replay-only, or
policy-blocked breakpoint always discloses — a green gutter icon can never hide that
reality.

## Identity through rename / reformat / import

`BreakpointMappingProvenance` names how the current mapping was derived so a textual
guess is never replayed as a semantic one:

- `stable_source_id` — a stable logical source identity survived a rename or move.
- `re_resolved_after_reformat` — re-resolved over the same source id after a reformat.
- `imported_source_map` — derived from an imported source map or session.
- `lexical_fallback` — only a textual match was available; **never** an exact mapping.
- `source_identity_lost` — stable identity could not be recovered.

The contract binds provenance to mapping state:

- `source_identity_lost` ⟺ `needs_remap`. A lost identity forces an explicit
  needs-remap, and the breakpoint stays visible rather than silently disappearing; and
  a needs-remap state only ever comes from a lost identity.
- `lexical_fallback` ⟹ mapping is never `exact`, and `mapping_provenance_is_semantic`
  is false — a grep-style match is disclosed, not presented as certainty.

## Notebook and replay views

- A `notebook_cell`-scoped breakpoint must carry a `NotebookCellAnchor` (stable
  `notebook_ref` + `cell_id`), so cell identity is preserved through cell shifts and
  re-execution. A remapped notebook breakpoint is flagged `needs_remap`, never drawn
  as exact.
- A `replay_timeline`-scoped breakpoint must carry a `ReplayFrameAnchor` (stable
  `capture_ref` + `timeline_ref`), stays `is_replay_only`, and never renders a live
  confirmed stop.

## Contract rules (frozen invariants)

The canonical set computes each invariant's `holds` flag from the built specs; an
inconsistent edit flips an invariant and fails the freeze gate.

- **`breakpoints.one_canonical_pill_vocabulary`** — every breakpoint carries one pill
  whose tokens come from the frozen vocabulary and whose flags equal their derivation.
- **`breakpoints.verification_vocabulary_complete`** — pending, verified, unbound,
  unsupported, and policy-blocked are all materialized.
- **`breakpoints.mapping_vocabulary_complete`** — exact, misaligned, needs-remap, and
  unmapped are all materialized.
- **`breakpoints.green_never_hides_unverified_misaligned_replay_or_blocked`** — the
  confirmed-stop icon renders only for a verified, exact, non-replay breakpoint.
- **`breakpoints.lost_identity_degrades_to_needs_remap`** — a lost source identity
  stays visible as needs-remap, and a needs-remap state only comes from a lost
  identity.
- **`breakpoints.lexical_fallback_never_presented_as_exact`** — a lexical fallback is
  never shown as an exact semantic mapping.
- **`breakpoints.notebook_preserves_stable_cell_identity`** — every notebook
  breakpoint keeps a stable cell anchor and is never drawn exact while remapped.
- **`breakpoints.replay_preserves_stable_frame_identity_and_stays_replay_only`** —
  every replay breakpoint keeps a stable frame anchor and stays replay-only.
- **`breakpoints.export_retains_verification_and_mapping_state`** — every breakpoint
  retains its verification and mapping state as typed pill fields and cites an
  export-safe proof packet.

## First consumers

- core debugger gutter, breakpoint list, and session header;
- notebook debug surface (per-cell breakpoint affordances);
- profiler / trace / replay workspace;
- incident / crash review;
- support export / escalation packets; and
- AI context and tool-call evidence.

## Checked-in artifacts

- Spec module:
  [`crates/aureline-debug/src/m5_breakpoint_specs/mod.rs`](../../crates/aureline-debug/src/m5_breakpoint_specs/mod.rs)
- Boundary schema:
  [`schemas/debug/m5_breakpoint_specs.schema.json`](../../schemas/debug/m5_breakpoint_specs.schema.json)
- Published fixture:
  [`fixtures/debug/m5_breakpoint_specs/canonical_set.json`](../../fixtures/debug/m5_breakpoint_specs/canonical_set.json)
- Evidence note:
  [`artifacts/debug/m5_breakpoint_specs.md`](../../artifacts/debug/m5_breakpoint_specs.md)
- Freeze gate:
  [`crates/aureline-debug/tests/m5_breakpoint_specs.rs`](../../crates/aureline-debug/tests/m5_breakpoint_specs.rs)

## Regenerating

```sh
cargo run -p aureline-debug --example dump_m5_breakpoint_specs \
  > fixtures/debug/m5_breakpoint_specs/canonical_set.json
cargo test -p aureline-debug
```

[`BreakpointSpec`]: ../../crates/aureline-debug/src/m5_breakpoint_specs/mod.rs
[`BreakpointPill`]: ../../crates/aureline-debug/src/m5_breakpoint_specs/mod.rs
