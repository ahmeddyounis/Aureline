# Editor-assist support / provider-debug packet

## Release evidence

This artifact documents the one canonical, frozen, export-safe assist-support
packet produced by `crates/aureline-editor/src/m5_assist_support/`. It makes
completion, hint, hover, and peek decisions supportable and replayable: each
decision record carries the originating micro-surface identity, the provider /
source path, the degraded / blocked reason, the partial-index and stale-doc state,
the anchor mapping quality, side-effect cues, and a single derived drift class,
with stable ids and correlation field ids. Project Doctor, support export, local
diagnostics, and the CLI consume this packet directly rather than inventing a
second view of assist behavior.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `AssistSupportPacket` | `m5_assist_support_packet` | `schemas/editor/m5-assist-support.schema.json` | 1 |

- Packet id: `m5-assist-support:packet:0001`
- As of: `2026-06-22T00:00:00Z`
- Coverage: 17 decisions across 4 kinds and 7 surfaces, 7 drift classes
- Overall: all 8 invariants hold

## Honesty invariants (all must pass)

1. `every_decision_carries_stable_ids` — every decision has a kind-prefixed id, a correlation field id, a subject ref, and a provider id.
2. `clean_baseline_is_fully_authoritative` — a decision is no-drift iff it has a full-semantic posture, exact anchor, live content, full fidelity, no narrow reason, and no remediation route.
3. `drift_class_matches_evidence` — each drift class is consistent with the decision's posture, source, mapping, content, and constrained-surface evidence.
4. `drifted_decisions_offer_route_and_explanation` — every drifted decision carries a next-safe-action route with a command id and a non-empty explanation.
5. `narrowing_iff_degraded` — a decision carries a narrow reason exactly when its degraded-state class is not full fidelity.
6. `rollups_reconcile_with_decisions` — drift and surface rollups cover exactly the classes and surfaces present and reconcile to the decision total.
7. `corpus_covers_kinds_and_constrained_surfaces` — every kind and every claimed constrained surface is exercised.
8. `redaction_safe_excludes_raw_payload` — every decision is metadata-only and the export contract excludes source text, prompt context, provider payloads, and credential bodies.

## Drift coverage

Generated and pinned in `fixtures/editor/m5-assist-support/canonical_packet.json`.

| Drift class | What it explains |
|---|---|
| no_drift_authoritative | Authoritative provider, exact anchor, live content (baseline). |
| provider_drift | Preferred provider degraded; labeled fallback answered. |
| cached_local_word_fallback | Cached / local-word / lexical fallback instead of deterministic language. |
| wrong_anchor_mapping | Approximate / heuristic anchor; target may be wrong. |
| constrained_file_narrowing | Generated / protected / request / SQL / docs-code / large-file narrowing. |
| partial_index_pending | Semantic results pending while the index builds. |
| stale_doc_snapshot | Stale or imported-snapshot doc rather than a live read. |

## Redaction posture

The `support_export` contract exports identity, provider / source path, degraded
reason, freshness, mapping, side-effect cue, drift class, and the next-safe-action
route only. It never exports `source_text`, `prompt_context`, `provider_payload`,
`credential_body`, or `buffer_contents`. Every decision record sets
`redaction_safe`, and the packet sets `raw_payload_excluded`.

## Verification

Emit the canonical packet:

```sh
cargo run --bin aureline_m5_assist_support
cargo run --bin aureline_m5_assist_support -- --lines
```

Run the freeze gate (rebuilds the packet and asserts it equals the fixture):

```sh
cargo test -p aureline-editor --test m5_assist_support_replay
```

Run the unit contract suite:

```sh
cargo test -p aureline-editor m5_assist_support
```

## Risks and follow-ups

- **The corpus is a seeded, representative proof, not a live capture.** It freezes
  one decision per kind × surface × drift combination worth explaining so the
  shared model and its redaction posture are provable. Wiring live editor surfaces
  to emit these records as real assist decisions happen is incremental follow-up.
- **Project Doctor / support export consume this packet; they do not re-derive
  it.** The rollups and field ids are the source of truth for assist-decision
  explainability across desktop, Support Center, CLI, and support bundles. The
  support crate binds to this packet rather than this lane editing the support
  crate.
- **Shared vocabularies are reused, not re-proved here.** The packet references the
  assist source-label, matrix surface / degraded-state, constrained-file narrow /
  route, hover/peek mapping, and completion provider-posture classes; their own
  contracts remain the source of truth for those vocabularies.
