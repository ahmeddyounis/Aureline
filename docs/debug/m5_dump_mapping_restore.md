# M5 dump/mapping/restore strips

This contract materializes the **dump/core-file/source-map/symbol artifact-strip** family
and the **restore-honesty** family as concrete, typed, serde-serializable
[`DebugArtifactStrip`] and [`RestoredLayoutRecord`] records, each carrying one canonical
pill, and pins the single six-state mapping-fidelity vocabulary every debug surface reads.
It is the canonical M5 source every debugger, notebook, profiler, incident, support, and
AI surface reads to show *which debug artifact it opened* (a core file, crash dump,
inspect-only session, symbol artifact, source map, or replay capture), *how trustworthy
that artifact's source/symbol mapping is*, *which build it belongs to*, and — when a layout
is reopened — *whether the prior process/session is gone, inspect-only, reconnect-required,
or manually relaunchable*. Dump strips, symbolicated stack headers, source-map cards, and
restored panes consume these records directly instead of re-deriving build/mapping truth
into rendered chrome.

Authoritative product anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.10.1–§7.6.10.4 and §9.46 on debug
  launch/session, breakpoints, variables/watches, evaluate side-effect governance,
  chronology capture, replay, and notebook-debug parity.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §14.5 on debug session headers, frame
  mapping, variables/watch panes, evaluate/REPL review, and dump/source-map truth.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` `FR-DEBUG-001` and the debug surface
  rules on stable breakpoints, variables, stack views, chronology cues, and artifact-linked
  evidence.

This lane composes with the [symbolication contract](./symbolication.md) (which pins
symbol/source-map manifests, build-match state, and the four-state user-facing fidelity
label at the *artifact provenance* level) and with the
[frame-mapping and variable/watch snapshots](./m5_frame_variable_snapshots.md) (which pin
the four-state fidelity at the *stack-frame* level). It widens those four-state
vocabularies into one shared six-state [`DebugMappingFidelity`], so frames, breakpoints,
variables, and dump artifacts read one fidelity instead of re-expressing it.

## The shared mapping vocabulary

`DebugMappingFidelity` is the single controlled vocabulary rendered wherever a frame,
breakpoint, variable, or dump artifact invites trust in a source/symbol mapping:

| State | Meaning | Navigable | Exact link |
|---|---|---|---|
| `exact` | authoritative source/symbol against an exact build | yes | yes (with exact build) |
| `approximate` | line-only, drifted, or nearest-span mapping | yes | no |
| `symbol_only` | symbol name resolved, no authoritative source lines | no | no |
| `unresolved` | no mapping could be resolved | no | no |
| `imported` | resolved from an imported / side-loaded artifact, bounded trust | yes | no |
| `mismatched_build` | candidate found but the build does not match; rejected | no | no |

It is a strict superset of the four-state frame-mapping fidelity: `from_frame_fidelity`
widens each frame state, `narrow_to_frame_fidelity` narrows back (imported → approximate,
build-mismatch → unmapped), and `from_symbolication_label` adapts the symbolication label.
The two extra states (`imported`, `mismatched_build`) are the artifact-level degradations
the frame and symbolication vocabularies cannot express.

## The artifact strip

The module
[`crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs`](../../crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs)
owns the typed records. A `DebugArtifactStrip` carries:

- a stable `strip_id`;
- a `DebugArtifactKind` (`core_file`, `crash_dump`, `inspect_only_session`,
  `symbol_artifact`, `source_map`, `replay_capture`);
- a `DebugArtifactEntrypoint` (`open_core_file`, `open_crash_dump`, `open_replay`,
  `open_inspect_only`, `import_symbols_or_source_map`) and a derived
  `opens_inspect_only_session`;
- an optional opaque `build_id` and export-safe `artifact_ref` (at least one is always
  present);
- an optional `debug_format` (PDB, dSYM, DWARF, or a JS/TS/CSS source map), present
  exactly for mapping inputs;
- a `captured_as_of` timestamp; and
- the canonical `DebugArtifactPill` pinning one `DebugMappingFidelity`, one
  `ArtifactBuildMatch`, and one `ArtifactSourceClass`.

The pill's flags are **derived**, never asserted:

```
shows_exact_source_link =
    fidelity.preserves_exact_source()    // exact
    && build_match.proves_exact_build()  // exact-build verified

requires_disclosure      = !shows_exact_source_link
allows_source_navigation = fidelity in {exact, approximate, imported}
is_mirrored_source       = source_class == mirror_supplied
is_imported_source       = source_class == imported_attachment
```

So a strip never flattens exact, approximate, symbol-only, unresolved, imported, and
build-mismatched mappings into one location link: the unqualified precise source link
renders only for an exact mapping backed by an exact-build match. The label always
discloses a non-local source (provider, mirror, imported), a non-exact build match, and an
inspect-only posture.

### Entrypoints stay distinct and visible

Core-file, crash-dump, open-replay, and open-inspect-only are four distinct entrypoints
that each open an inspect-only session; importing a symbol or source-map artifact is a
fifth, non-session entrypoint. An entrypoint never opens a kind it does not accept
(`accepts_kind`), and the strip carries the entrypoint in UI, command, and export paths so
how an artifact was opened is never lost.

### Imported and mismatched-build stay honest

An `imported` strip is always sourced from an `imported_attachment`, and a
`mismatched_build` strip always carries a `mismatched_rejected` build match — neither ever
renders the exact source link. This is the guardrail that keeps a reopened pane,
symbolicated stack, or imported source map from implying a stronger mapping/build guarantee
than the artifact evidence supports.

## The restored layout — restore honesty

A `RestoredLayoutRecord` is what a surface reads when a debug layout is reopened. It
carries a stable `layout_id`, the `restored_strip_ref` of the artifact it reopened, an
opaque `prior_session_ref`, a `restored_as_of` timestamp, an `exact_build_still_verified`
flag, and the canonical `RestorePill`. The pill's flags are **derived**:

```
implies_live_continuity     = false   // always — reopening never reacquires a live process
implies_process_authority   = false   // always
implies_exact_build_mapping = fidelity.preserves_exact_source() && exact_build_still_verified
requires_explicit_action    = posture in {reconnect_required, manually_relaunchable}
requires_disclosure         = true    // always — a restored layout always discloses
```

So a restored layout names one honest `RestorePosture` and never implies live continuity
or reacquired process authority:

- **`process_gone`** — the prior process/session is gone; the layout is historical only.
- **`inspect_only_continuation`** — reopened as inspect-only (a dump or replay); even when
  its exact-build mapping is *still* verified, it never implies a live process.
- **`reconnect_required`** — a live target may be reattachable only after an explicit
  reconnect.
- **`manually_relaunchable`** — no reconnect is possible; the user must relaunch.

An exact-build mapping is shown on restore only when it is still verified, so a reopened
pane never implies exact-build mapping when that is no longer true.

## Contract rules (frozen invariants)

The canonical set computes each invariant's `holds` flag from the built records; an
inconsistent edit flips an invariant and fails the freeze gate.

- **`artifacts.one_canonical_mapping_pill`** — every strip carries one pill whose tokens
  come from the frozen vocabulary and whose flags equal their derivation.
- **`artifacts.mapping_vocabulary_complete`** — exact, approximate, symbol-only,
  unresolved, imported, and mismatched-build are all materialized.
- **`artifacts.artifact_kind_vocabulary_complete`** — all six artifact kinds are
  materialized.
- **`artifacts.source_class_vocabulary_complete`** — workspace, local, provider, mirror,
  and imported source classes are all materialized.
- **`artifacts.exact_link_never_hides_degraded_mapping`** — the precise source link renders
  only for an exact mapping backed by an exact-build match; any degraded strip discloses.
- **`artifacts.imported_and_mismatch_stay_honest`** — an imported mapping is always sourced
  from an import, a build-mismatch always carries a rejected build match, and neither ever
  renders the exact link.
- **`artifacts.entrypoints_distinct_and_visible`** — core-file, crash-dump, open-replay,
  and open-inspect-only are each present and each open an inspect-only session, distinct
  from the import entrypoint, which never opens a session.
- **`artifacts.build_artifact_identity_present`** — every strip carries a build id or
  artifact ref, a capture time, an accepting entrypoint, and a debug format exactly for
  mapping inputs.
- **`artifacts.mirrored_and_imported_sources_disclosed`** — mirrored and imported sources
  are both materialized and disclose their provenance.
- **`restore.posture_vocabulary_complete`** — gone, inspect-only-continuation,
  reconnect-required, and manually-relaunchable are all materialized.
- **`restore.never_implies_live_continuity_or_authority`** — every restored layout never
  implies live continuity or reacquired process authority and always discloses.
- **`restore.exact_build_mapping_only_when_still_verified`** — a restored layout shows an
  exact-build mapping only when the mapping is exact and the build is still verified.
- **`restore.required_action_named`** — reconnect-required and manually-relaunchable name
  an explicit action; gone and inspect-only do not.
- **`set.shared_mapping_vocabulary_supersets_frame_fidelity`** — the shared vocabulary is a
  strict superset of the frame-mapping fidelity, with imported and mismatched-build beyond
  the four frame states.
- **`set.export_retains_artifact_and_restore_state`** — every strip and restore retains its
  typed tokens and cites an export-safe proof packet.

## First consumers

- core debugger dump/core-file/source-map/symbol strips and session headers;
- notebook debug surface (artifact strips for kernel-bridged dumps);
- profiler / trace / replay workspace and replay inspector;
- incident / crash review and symbolicated stacks;
- support export / escalation packets; and
- AI context and tool-call evidence.

## Checked-in artifacts

- Spec module:
  [`crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs`](../../crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs)
- Boundary schema:
  [`schemas/debug/m5_dump_mapping_restore.schema.json`](../../schemas/debug/m5_dump_mapping_restore.schema.json)
- Published fixture:
  [`fixtures/debug/m5_dump_mapping_restore/canonical_set.json`](../../fixtures/debug/m5_dump_mapping_restore/canonical_set.json)
- Evidence note:
  [`artifacts/debug/m5_dump_mapping_restore.md`](../../artifacts/debug/m5_dump_mapping_restore.md)
- Freeze gate:
  [`crates/aureline-debug/tests/m5_dump_mapping_restore.rs`](../../crates/aureline-debug/tests/m5_dump_mapping_restore.rs)

## Regenerating

```sh
cargo run -p aureline-debug --example dump_m5_dump_mapping_restore \
  > fixtures/debug/m5_dump_mapping_restore/canonical_set.json
cargo test -p aureline-debug
```

[`DebugArtifactStrip`]: ../../crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs
[`RestoredLayoutRecord`]: ../../crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs
[`DebugMappingFidelity`]: ../../crates/aureline-debug/src/m5_dump_mapping_restore/mod.rs
