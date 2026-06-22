# Editor-assist support / provider-debug packet

One canonical, frozen, export-safe **support packet** that makes editor-assist
decisions — **completion, hint, hover, and peek** — supportable and replayable.
Where the [editor-assist matrix](m5-editor-assist.md) freezes the *contract*
(which channel is offered, narrowed, or blocked on each surface, and the
support-export minimums), this packet carries the *realized records* that answer,
with stable ids and no screenshots:

- *why did this hint appear, and from which provider or fallback path?*
- *why is this completion cached / local-word fallback instead of deterministic
  language intelligence?*
- *why did this hover or peek open on an approximate (wrong) anchor?*
- *why was this hover stale, or this completion pending?*
- *why did this surface narrow or block assist, and what is the next safe action?*

- Schema: [`schemas/editor/m5-assist-support.schema.json`](../../schemas/editor/m5-assist-support.schema.json)
- Canonical fixture: [`fixtures/editor/m5-assist-support/canonical_packet.json`](../../fixtures/editor/m5-assist-support/canonical_packet.json)
- Rust truth source: `crates/aureline-editor/src/m5_assist_support`
- Headless emitter: `cargo run --bin aureline_m5_assist_support`
- Freeze gate: `cargo test -p aureline-editor --test m5_assist_support_replay`

The packet reuses the shared vocabularies rather than forking a second one: the
[assist source-label classes](assist_and_quickfix_beta.md), the matrix surface and
degraded-state classes, the [constrained-file](m5-constrained-assist.md) narrow
reasons and next-safe-action routes, the [hover/peek](m5-hover-peek.md) mapping
quality, and the [completion](m5-completion-rows.md) provider posture and
additional-edit cue.

## What the packet carries

### Decision records

Each [`AssistDecisionRecord`] is the full, redaction-safe provenance of a single
assist outcome. Stable identity comes first: a kind-prefixed `decision_id`
(`completion-decision:`, `hint-decision:`, `hover-decision:`, `peek-decision:`), a
dotted `field_id` (`assist_support.<kind>.<drift_class>`) operators filter and
group by, and a `subject_ref` back to the originating micro-surface record
(`completion-session:`, `hint:`, `hover-peek:`) so an issue correlates to the live
session / card identity.

The provider / source path: `provider_id` (an opaque provider name, never a
credential), `source_label_class`, and `provider_posture`. The degraded / blocked
reason: `degrade_state` plus a `narrow_reason` when narrowed. The freshness state:
`index_freshness` (fresh / building / reindex-pending / not-indexed) and
`content_state` (live / stale / pending / imported snapshot / fallback / policy
limited / suppressed). The anchor quality: `mapping_quality` (exact / approximate /
heuristic / unresolved). Any acceptance side effect: `side_effect_cue`. The single
derived headline category: `drift_class`. And, when drifted, a `next_safe_action`
with its `next_action_command`, plus a bounded `explanation`.

### Drift classes

The one closed explainability vocabulary every consumer filters by:

| Drift class | Meaning |
| --- | --- |
| `no_drift_authoritative` | Authoritative provider, exact anchor, live content — the baseline. |
| `provider_drift` | The preferred provider drifted to a degraded posture; a fallback answered. |
| `cached_local_word_fallback` | A cached / local-word / lexical fallback answered instead of deterministic language. |
| `wrong_anchor_mapping` | The decision mapped to an approximate / heuristic anchor; the target may be wrong. |
| `constrained_file_narrowing` | A constrained-file state narrowed or blocked the decision. |
| `partial_index_pending` | Semantic results are pending while the index builds. |
| `stale_doc_snapshot` | The decision reflects a stale or imported-snapshot doc, not a live read. |

### Project Doctor rollups

Two aggregate views, each with a stable correlation `field_id`:

- **Drift rollups** ([`assist_support.drift.<token>`]) — how many decisions fell
  into each drift class, and which kinds and surfaces they touched.
- **Surface rollups** ([`assist_support.surface.<token>`]) — per surface, how many
  decisions stayed authoritative (`clean_count`) versus drifted (`drifted_count`).

### Support-export contract

The `support_export` block names exactly which per-decision fields cross the
boundary and which evidence classes never do — `source_text`, `prompt_context`,
`provider_payload`, `credential_body`, and `buffer_contents`. Explanation quality
comes from stable metadata and bounded sentences, not raw payloads.

## Invariants

The packet evaluates these over its own data and records the result in
`invariants[].holds`; the freeze gate re-proves every one:

1. `every_decision_carries_stable_ids`
2. `clean_baseline_is_fully_authoritative`
3. `drift_class_matches_evidence`
4. `drifted_decisions_offer_route_and_explanation`
5. `narrowing_iff_degraded`
6. `rollups_reconcile_with_decisions`
7. `corpus_covers_kinds_and_constrained_surfaces`
8. `redaction_safe_excludes_raw_payload`

## Consuming the packet

The packet is a single record. Project Doctor, support export, local diagnostics,
and the CLI should read the canonical fixture (or call `assist_support_packet()`),
filter decisions by `field_id` / `drift_class` / `surface_class`, and render the
provenance and `explanation` directly — never re-deciding why an assist surface
behaved a given way. The human-readable projection (`assist_support_packet_lines`)
is shared by the CLI emitter and support export.

## Regenerating

The fixture is generated, not hand-edited:

```sh
cargo run --bin aureline_m5_assist_support > fixtures/editor/m5-assist-support/canonical_packet.json
```

If you change the packet in code without regenerating, the freeze gate
(`m5_assist_support_replay`) fails.
