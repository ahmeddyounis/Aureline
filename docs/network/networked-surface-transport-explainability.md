# Networked-surface transport-explainability

This explainability layer is the **product-grade surface** that sits on top of
the [networked-surface transport-decision log](./networked-surface-transport-decision.md).
The decision log emits one inspectable transport decision per network-capable
action; this layer projects that decision stream into the three views users and
admins actually reach for, so they get real transport explainability instead of
a generic could-not-connect dialog or a private support script.

The runtime owner is
`aureline_remote::networked_surface_transport_explainability`; the boundary
schema is
`schemas/network/networked_surface_transport_explainability.schema.json`.

Every view is projected from the same decision snapshot, so the posture
inspectors, the event ledger, and the explain sheets can never diverge from the
decisions they explain. No raw endpoint URLs, raw hostnames, raw ports, raw
credentials, raw bearer or session tokens, raw cookie jars, raw private
certificate bytes, raw SSH private material, or raw PAC bodies cross the
boundary — only closed-vocabulary tokens, opaque refs, and plain-language
summary sentences.

## The three views

### Current transport-posture inspector

For each surface, [`TransportPostureInspector`] answers the posture questions a
user or admin needs before trusting a networked surface, at a glance:

- the **effective proxy mode** — the proxy-resolution tier that actually selects
  the route (PAC → manual → system precedence), recorded as
  `effective_proxy_mode`,
- the **trust source** — the trust input anchoring host proof (`trust_source`),
  with the trust-proof ref and its freshness,
- the **mirror/offline state** — what the surface does when its primary route is
  unavailable (`mirror_offline_state`),

plus the effective egress class and route choice, and the guardrail flags that
prove the posture resolved through the shared governance layer (`no_bypass`),
permits no silent public fall-through (`no_silent_public_fallback`), and keeps
local-core editing alive (`local_core_continuity_preserved`).

### Recent network-event ledger

[`NetworkEventLedger`] holds one [`NetworkEventLedgerEntry`] per recent decision
and exposes the filters product, CLI, and support surfaces share:

- `filter_by_endpoint_class` — by endpoint class,
- `filter_by_origin_scope` — by origin scope,
- `filter_by_disposition` / `allowed_events` / `denied_events` — by allow/deny
  outcome.

Each entry carries a coarse `disposition` (`allowed`, `denied`,
`served_without_egress`, `deferred`, `unavailable`) derived from the fine-grained
transport outcome, plus the typed denial reason when refused. Filtering never
reveals raw secrets or payloads because entries carry only closed-vocabulary
tokens and opaque refs.

### Per-action explain sheet

[`ActionExplainSheet`] is the canonical answer to "why did this action route and
resolve the way it did?". It binds the route choice, trust source, outcome, and
(when refused) the typed denial explanation, and renders them through one stable
field catalog ([`EXPLAIN_FIELD_NAMES`]) so the product UI, CLI/headless output,
and support exports all quote identical decision codes and field names.

The single renderer is `explain_fields()`. CLI output (`render_cli_lines`,
`key=value`) and support output (`render_support_lines`, `key: value`) both read
from it, and `fields_at_parity()` verifies the rendered field names match the
catalog in order, so parity holds by construction rather than by convention.

## Stable field catalog

The per-action explain sheet renders exactly these fields, in this order, on
every surface:

`surface`, `origin_scope`, `endpoint_class`, `egress_class`, `route_choice`,
`proxy_resolution_source`, `auth_posture`, `trust_material`,
`mirror_offline_behavior`, `outcome`, `denial_reason`.

## Stability conditions

The page qualifies `stable` only when **all** of the following hold for every
covered surface:

1. Every required surface has a posture inspector and an explain sheet.
2. No raw private material is present on any record.
3. Every projected decision resolved through the shared governance layer
   (`no_bypass: true`).
4. No decision permits a silent fall-through to the public internet from a
   confined egress class.
5. Any offline-deferred decision queues only an idempotent action.
6. Every decision preserves local-core continuity.
7. Every denied event carries a typed denial explanation.
8. Every posture inspector carries a non-empty trust-proof ref.
9. Every decision's trust proof is fresh (or stale only within a grace window).
10. Every explain sheet renders at field-catalog parity.

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false`
  (`raw_private_material_exposed`),
- a decision with `no_bypass: false` (`bypassed_shared_governance`),
- a policy with `no_silent_public_fallback: false`
  (`silent_public_fallback_resolved`),
- an offline-deferred decision that queues a non-idempotent action
  (`non_idempotent_replay_queued`).

A missing required surface narrows the packet to `preview`. A denial without a
typed reason, a stale-beyond-window proof, a broken explain-field parity, or any
remaining condition gap narrows the affected row to `beta`, which lets release
and support tooling detect and automatically narrow stale or under-qualified
rows before publication.

## CLI / support / product parity

Because product, CLI/headless, and support exports all render explain sheets
through `explain_fields()` over the single `EXPLAIN_FIELD_NAMES` catalog, the
decision codes and field names a user reads in the UI are byte-for-byte the same
tokens CLI output and support packets quote. The
`dump_networked_surface_transport_explainability_fixtures` example's
`explain-cli` subcommand is the headless rendering of this catalog.

## Truth paths

- Doc: `docs/network/networked-surface-transport-explainability.md`
- Artifact: `artifacts/network/networked-surface-transport-explainability.md`
- Schema:
  `schemas/network/networked_surface_transport_explainability.schema.json`
- Fixtures: `fixtures/network/networked_surface_transport_explainability/`
- Contract ref: `remote:networked_surface_transport_explainability:v1`

[`TransportPostureInspector`]: ../../crates/aureline-remote/src/networked_surface_transport_explainability/mod.rs
[`NetworkEventLedger`]: ../../crates/aureline-remote/src/networked_surface_transport_explainability/mod.rs
[`NetworkEventLedgerEntry`]: ../../crates/aureline-remote/src/networked_surface_transport_explainability/mod.rs
[`ActionExplainSheet`]: ../../crates/aureline-remote/src/networked_surface_transport_explainability/mod.rs
[`EXPLAIN_FIELD_NAMES`]: ../../crates/aureline-remote/src/networked_surface_transport_explainability/mod.rs
