# Relation-aware continuity contract

How Aureline keeps **symbol navigation and rename evidence relation-aware** across
peek, side-panel, split, history, and drift states — so a return jump or a replay
preserves *which relation kind* was navigated, *where to return*, and *whether the
target still resolves*, instead of degenerating into generic open behavior or silently
retargeting to a nearby guess.

- Builder and corpus: [`crates/aureline-navigation/src/relation_continuity/mod.rs`](../../crates/aureline-navigation/src/relation_continuity/mod.rs)
- Boundary schema: [`schemas/navigation/relation_continuity.schema.json`](../../schemas/navigation/relation_continuity.schema.json)
- Frozen corpus: [`fixtures/navigation/relation_continuity/canonical_continuity.json`](../../fixtures/navigation/relation_continuity/canonical_continuity.json)
- Evidence companion: [`artifacts/navigation/relation_continuity.md`](../../artifacts/navigation/relation_continuity.md)
- Freeze gate: [`crates/aureline-navigation/tests/relation_continuity.rs`](../../crates/aureline-navigation/tests/relation_continuity.rs)

This contract sits on top of the typed
[navigation target model](m3/navigation_target_beta_contract.md) — which freezes
`RelationKind`, `ContinuityState`, `AmbiguityClass`, and the proof/freshness classes —
and complements the generic
[bookmark/history/drift continuity packet](../m4/bookmark-history-and-drift-continuity.md),
whose anchors carry drift state but **no relation semantics**. This corpus adds the
relation-aware layer the generic packet left implicit: a peek/reveal/split/history entry
that remembers its relation kind, origin surface, return anchor, and current-versus-
captured target truth, plus replay-safe rename-preview evidence.

## What a relation-continuity packet preserves

The builder
([`build_relation_continuity_packet`](../../crates/aureline-navigation/src/relation_continuity/mod.rs))
is a pure function over a `RelationContinuityInput`. Given navigation entries and rename
evidence for a session it produces a `RelationContinuityPacket` with five guarantees.

1. **Relation-aware entries.** Each `RelationNavEntryInput` becomes a
   `RelationNavigationEntry` listed in canonical order — peek, temporary reveal,
   open-in-split, back, forward, recent location — that keeps its `RelationNavEntryKind`,
   its `RelationKind`, its origin surface, a `ReturnAnchor` that restores the origin
   selection and viewport, and a `RelationTargetSnapshot` for **both** the captured
   target and the currently-resolved one. The captured and current snapshots always carry
   the **same** relation kind: a remap never relabels it.
2. **Current-versus-captured truth.** A `RelationContinuityCounts` tallies entries by
   kind and by drift state, and separates `current_scope_count` from
   `captured_scope_count` (with `current + captured == total`). An entry is current-scope
   **only** when it is bound with fresh, semantic evidence; a remapped, drifted, imported,
   runtime, framework, lexical, or stale entry is captured-scope only.
3. **No silent jump.** An entry sets `auto_open_allowed: true` only when it is bound with
   live semantic evidence and is unambiguous. A `Remapped` entry cites stable
   `remap_evidence_refs` (with `used_nearby_fallback: false` **always**) and offers an
   explicit `open_remapped_target` recovery action; a `Drifted`, `MissingTarget`,
   `ScopeUnavailable`, or `Archived` entry carries **no** `current_target`, keeps a
   visible `drift_reason` and `RelationRecoveryChoice`s, and routes selection ambiguity
   through a `disambiguation_set_ref` — so the drift state shows before any jump.
4. **Replay-safe support/export.** Every entry and every `RenamePreviewEvidence` row
   names its `RelationContinuityEvidenceClass` (so a lexical/grep fallback never reads as
   semantic) and carries a replay-safe target id derived from its own stable id
   (`aureline://replay/relation-nav/...`, `aureline://replay/rename/...`). Rename evidence
   keeps a definition root relation, its `RenameApplyPosture`, ambiguity, drift state,
   return anchor, and change-versus-held counts.
5. **Consumer parity.** Each packet projects to every `ConsumerSurface` with a
   `RelationContinuityProjection` that preserves relation kind, origin surface, return
   anchor, current-versus-captured truth, drift state, fallback class, ambiguity, and
   replay ids, never silently retargets, and never exports raw code bodies.

## Entry kinds and the auto-open rule

| Entry kind | Token | Keeps origin live? |
| --- | --- | --- |
| Peek | `peek` | yes |
| Temporary reveal | `temporary_reveal` | yes |
| Open in split | `open_in_split` | yes |
| Back | `back_history` | no (return anchor restores) |
| Forward | `forward_history` | no (return anchor restores) |
| Recent location | `recent_location` | no (return anchor restores) |

`auto_open_allowed` is true **iff** the entry is `bound`, its evidence class is
`semantic`, its freshness is not stale/unverified, and it is unambiguous. Everything
else — a remap, any drift, a lexical/runtime/framework/imported entry, or an ambiguous
entry — discloses its state and offers recovery before it can be navigated.

## Drift states stay visible

The drift vocabulary is the closed `ContinuityState` set: `bound`, `remapped`,
`drifted`, `missing_target`, `scope_unavailable`, `archived`. Each non-bound entry keeps
a visible drift reason and at least one recovery choice:

| Drift state | Current target? | Recovery choices |
| --- | --- | --- |
| `bound` | yes (live) | — (auto-opens) |
| `remapped` | yes (moved, via stable evidence) | `open_remapped_target`, `restore_return_anchor_only` |
| `drifted` | no | `inspect_drift`, `restore_return_anchor_only` |
| `missing_target` | no | `inspect_drift`, `restore_return_anchor_only` |
| `scope_unavailable` | no | `widen_scope`, `restore_return_anchor_only` |
| `archived` | no | `keep_archived_reference`, `restore_return_anchor_only` |

A `refresh_provider` choice is added when freshness is degraded/stale/unverified or the
scope is stale, and a `choose_from_disambiguation` choice is added when the entry's
ambiguity needs an explicit selection.

## Evidence honesty

Each entry names a `RelationContinuityEvidenceClass` — `semantic`, `framework_derived`,
`runtime_observed`, `imported_snapshot`, `lexical_fallback`, `syntax_fallback`, or
`unavailable`. A fallback entry always carries a fallback note **and** a downgrade reason
and is never current-scope or auto-open; a runtime, framework, or imported entry stays
captured-scope only. So a grep match is never replayed as semantic certainty.

## Replay and support export

Every packet is metadata-only and serde-serializable, so search, graph, docs/help,
editor, AI, review, support, and CLI surfaces consume the same record. Because each entry
and rename-evidence row carries a stable replay-safe target id, its relation kind, drift
state, fallback class, ambiguity, and return anchor, a support or debug packet can replay
or explain symbol navigation and rename evidence **without silent retargeting** — and
without any source body, raw path, identifier, provider payload, URL, hostname, or
credential. Refs are opaque `aureline://` handles or repo-relative paths only.

## Frozen invariants

The corpus computes each invariant's `holds` flag from the builder's own output, so an
inconsistent change flips an invariant and fails CI:

- `relation_continuity.entry_preserves_relation_context`
- `relation_continuity.current_vs_captured_separated`
- `relation_continuity.no_silent_jump`
- `relation_continuity.drift_states_visible_with_disambiguation`
- `relation_continuity.fallback_class_honest`
- `relation_continuity.replay_ids_stable`
- `relation_continuity.rename_evidence_preserved`
- `relation_continuity.consumers_preserve_truth`
- `relation_continuity.corpus_covers_vocabulary`

## Release-automation binding

The freeze gate
[`crates/aureline-navigation/tests/relation_continuity.rs`](../../crates/aureline-navigation/tests/relation_continuity.rs)
runs under `cargo test --workspace`. It rebuilds the corpus in code and asserts it equals
the checked-in fixture, re-proves that every stored packet equals the builder's own
output, that the corpus is support-export safe, that every entry preserves its relation
kind and return context, that every non-bound entry discloses its drift state and
recovery before any jump, that fallback entries never auto-open, that rename evidence
survives with its relation kind and replay id, and that every frozen invariant holds — so
a claimed search/graph/docs/editor row cannot promote while relation-kind honesty or
drift truth stays implicit.

Regenerate the fixture after any builder change with:

```sh
cargo run -p aureline-navigation --example dump_relation_continuity \
  > fixtures/navigation/relation_continuity/canonical_continuity.json
```
