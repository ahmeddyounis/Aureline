# Search action bindings, parity, and no-wrong-target fallbacks

This document describes the action-binding layer that backs preview, open,
split, peek, and external-handoff actions across the M5 search, docs, graph,
history, and support flows. Where the result-truth packet *owns* the row-level
`SearchActionBinding` and the navigation target model *owns* the semantic
`RelationKind`, this layer binds the two together so a result action preserves
the relation kind, target ref, return anchor, required capability, fallback
mode, and history policy — and can never silently jump to a nearby symbol,
declaration, or docs page.

- Schema: `schemas/search/action-binding.schema.json`
- Packet model: `crates/aureline-search/src/action_bindings/mod.rs`
- Desktop consumer: `crates/aureline-shell/src/navigation_targets/mod.rs`
- Fixtures: `fixtures/search/m5/navigation-targets/`

## Action bindings

Every result row keeps a `ResolvedActionBinding`. The binding embeds the
canonical `SearchActionBinding` verbatim — open target, alternate behaviors,
required surface capabilities, fallback mode, history policy — and adds the
attributes that make navigation trustworthy:

| Field | Meaning |
| --- | --- |
| `action_kind` | `preview`, `open_in_place`, `split`, `peek`, or `external_handoff`. |
| `requested_relation_kind` | The relation the user asked for (e.g., go-to-definition). |
| `resolved_relation_kind` | The relation the action actually resolved to. |
| `return_anchor_ref` | Where focus returns after the action; never equal to the open target. |
| `fallback_trigger` | Why a wrong-target-safe fallback fired (`none` for a direct action). |
| `fallback` | The explicit fallback record, present iff the trigger is not `none`. |

Because the same binding object is reused, the product UI, history/back-forward
replay, and support replay all land on **one** target semantics rather than
reconstructing a near-miss target from rendered row text.

## No-wrong-target fallbacks

When the original target no longer resolves under the current scope, trust, or
freshness posture, the action takes a wrong-target-safe fallback recorded as a
`WrongTargetFallback` instead of silently degrading. Each fallback carries:

| Field | Meaning |
| --- | --- |
| `trigger` | `scope_narrowed`, `trust_policy`, `freshness_stale`, or `target_missing`. |
| `fallback_mode` | Reused verbatim from the canonical action-binding fallback mode. |
| `relation_kind_changed` | `true` when the resolved relation differs from the requested one. |
| `crosses_to_external_handoff` | `true` when a local target fell back to a browser handoff. |
| `visible_reason` | The user-visible reason — never silent. |
| `recovery_action` | How to recover the original target. |
| `recoverable` | Always `true`: fallbacks are explicit and recoverable. |

Two degrades are guaranteed to be visible states, never unrepresented edges:

- **Definition → declaration.** A go-to-definition that can only reach the
  indexed declaration/signature snapshot (e.g., the live definition body is
  stale) records `relation_kind_changed = true` with a visible reason. A
  definition jump may never silently become a declaration jump.
- **Local docs → browser handoff.** A docs action that cannot serve the page
  from the offline pack records `crosses_to_external_handoff = true` with a
  visible reason and uses the `external_handoff` action.

## Parity and history/support reuse

The `flows` cover search results, docs results, graph-backed results,
history/back-forward replay, and support handoff replay. History replay and
support handoff replay reuse the **same** binding objects — including return
anchors and fallback reasons — so back/forward and a support bundle inspect the
exact action that was launched. The three consumer projections (`product_ui`,
`history_back_forward`, `support_replay`) assert
`reuses_same_binding_objects = true` and `widens_authority = false`.

## Guardrails enforced (fail-closed)

- A relation-kind degrade must carry an explicit, recoverable wrong-target
  fallback with a visible reason.
- A fallback that crosses to an external handoff must use the `external_handoff`
  action and keep a visible reason.
- Split, peek, and open-in-place must keep a return anchor distinct from the open
  target.
- Convenience routing must not widen authority; every binding asserts
  `authority_not_widened = true`.
- The packet is metadata-only: no raw query text, source bodies, secrets, or
  private rank weights are admitted, and sessions are referenced hash-only.
