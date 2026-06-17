# Review artifact — search action bindings, parity, and no-wrong-target fallbacks

Packet id: `search.m5.action_bindings.v1`

This artifact is the reviewer-facing summary of the search action-binding layer
for the M5 search, docs, graph, history, and support flows. It is produced from
the seeded packet and is metadata-only.

## What this lane delivers

- Preview, open-in-place, split, peek, and external-handoff actions bound to
  canonical result refs and relation kinds across search results, docs results,
  graph-backed results, history/back-forward replay, and support handoff replay.
  Each binding embeds the canonical `SearchActionBinding` verbatim and reuses the
  navigation `RelationKind`.
- Wrong-target-safe fallbacks recorded as `WrongTargetFallback` rows when the
  original target no longer resolves under narrowed scope, trust, or freshness —
  each carrying a visible reason, a recovery action, and `recoverable = true`.
- A desktop navigation-target projection
  (`crates/aureline-shell/src/navigation_targets/mod.rs`) that reuses the same
  binding objects, preserves relation kinds and return anchors, and renders a
  fallback as a visible, recoverable cue instead of a silent open.

## Acceptance evidence

| Acceptance criterion | Evidence |
| --- | --- |
| Search actions no longer silently degrade from definition to declaration or from local docs to browser handoff without a visible reason | The matrix realizes one `definition → declaration` degrade (freshness-stale split) and one local-to-browser handoff (scope-narrowed docs), each with `relation_kind_changed`/`crosses_to_external_handoff` and a non-empty `visible_reason`. |
| Split, peek, and open-in-place preserve attributable target refs and return anchors | Every binding keeps a `return_anchor_ref` distinct from its `open_target_ref`, and the canonical `open_target_ref` is preserved. |
| Support/export packets can replay or inspect the action binding used without guessing from UI text | The support-handoff flow replays the search and docs fallback bindings verbatim, and the three consumer projections set `reuses_same_binding_objects/preserves_relation_kinds/preserves_return_anchors/preserves_fallback_reasons = true`. |

## Guardrails enforced (fail-closed)

- A relation-kind degrade must carry an explicit, recoverable wrong-target
  fallback whose `fallback_mode` matches the canonical action-binding mode.
- A fallback that crosses to an external handoff must use the `external_handoff`
  action and keep a visible reason.
- A direct action (`trigger = none`) must keep a `direct` fallback mode and carry
  no fallback; a non-direct trigger must carry an explicit, recoverable fallback.
- Convenience routing must not widen authority; every binding asserts
  `authority_not_widened`, and no consumer projection sets `widens_authority`.
- The packet stays metadata-only: no raw query text, source bodies, secrets, or
  private rank weights, and sessions are referenced hash-only.

## Degraded variant

`scope_trust_narrowed.json` proves that under further-narrowed scope, trust, and
freshness a search-results action that was previously direct now takes an
explicit, recoverable fallback (strictly more fallbacks than the canonical
packet), while the action-kind, relation-kind, and fallback-trigger vocabulary,
the result identity, and the reused binding objects are preserved. History
replay reads local material and is unchanged.

## Sources

- Contract doc: `docs/search/action-bindings.md`
- Schema: `schemas/search/action-binding.schema.json`
- Fixtures: `fixtures/search/m5/navigation-targets/`
- Model + tests: `crates/aureline-search/src/action_bindings/`
- Desktop consumer + tests: `crates/aureline-shell/src/navigation_targets/`
