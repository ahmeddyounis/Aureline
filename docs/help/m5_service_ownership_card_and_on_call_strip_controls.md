# M5 service-ownership card and on-call strip controls

Two reusable M5 governance-dashboard primitives are implemented as **one controls
packet** so support, operator, and release surfaces share a single role-based
ownership/escalation model rather than cloning prose:

- the **service-ownership card** — service or surface identity, owning role/team, support
  class, escalation path, owner freshness, and backup-coverage state; and
- the **on-call strip** — role alias, current availability state, primary/secondary
  distinction, escalation route, and export-safe handoff continuity.

The card and strip narrow the `service_ownership_card` and `on_call_strip` families frozen
in the governance-dashboard component matrix
(`schemas/ui/m5-governance-dashboard-component-matrix.schema.json`,
`docs/help/m5_governance_dashboard_components_contract.md`). The readiness-state
vocabulary, the ownership-coverage states, the on-call-coverage states, and the
escalation-route classes are reused verbatim from that matrix — no surface invents a
second ownership or escalation grammar.

Source of truth: the checked-in seed builder and support export in
`crates/aureline-release`
(`implement_service_ownership_cards_and_on_call_strips_...`). The Rust validator is the
authoritative gate; this doc and the schema
(`schemas/ui/m5-service-ownership-on-call-controls.schema.json`) document the shape.

## Two resolvers

### `resolve_service_ownership_card`

Takes one service's identity, owning role, support class, ownership-coverage state, owner
source, backup owner, escalation route, and owner freshness, and produces the derived
readiness state drawn from the frozen `M5GovernanceReadinessState` vocabulary. The derived
state is computed in a fixed degrade-first order:

1. unknown freshness → `not_evaluated`;
2. an owner that is only an inference from the last interacting team, or an unrecorded
   owner → `owner_unresolved` (**never inherited as a resolved owner**);
3. an unresolved coverage state → `owner_unresolved`;
4. a missing owner record → `blocked`;
5. a stale owner record → `evidence_stale`;
6. policy-hidden ownership → `warning`;
7. a primary owner with **no named backup** → `warning`;
8. an aging owner record → `warning`.

Only a service with an authoritative owner, a named backup (`owned_with_backup`), and a
fresh owner record is a clean pass. **An ownerless or backup-missing protected surface
never reads as covered** (AC-1).

### `resolve_on_call_strip`

Takes one strip's role alias, on-call-coverage state, current availability,
primary/secondary role tier, escalation route, handoff continuity, and roster freshness,
and produces the derived readiness state, an always-explicit escalation route, and the
export-safe handoff continuity. Degrade-first order: unknown freshness → `not_evaluated`;
a missing escalation path → `blocked`; an on-call gap, no coverage, or a missing roster →
`blocked`; no named responder → `owner_unresolved`; a stale roster → `evidence_stale`; an
unknown posture → `not_evaluated`; an escalation-only or off-shift posture → `warning`; a
pending handoff → `warning`. **An on-call gap never reads as covered.**

## Parity matrix

`M5ServiceOwnershipOnCallControlsPacket` binds one row per claimed M5 governance consumer
— the operator board, the release center, the service-health surface, the support export,
and the CLI inspect — to the shared card and strip anatomy, the same vocabulary, degrade
reasons, next actions, actions, export fields, and non-visual accessibility routes, plus
worked resolution cases that must reproduce the resolver output exactly. The operator,
release, and support consumers each carry worked ownership and on-call cases so they can be
proven to **reuse one role-based ownership/escalation model** rather than cloning prose
(AC-2).

## Hard invariants

Every controls row asserts, and the validator enforces, that it never:

- renders an unowned or backup-missing surface as covered;
- inherits the last interacting team as the owner;
- hides an on-call gap or the escalation route; or
- invents an ownership-local status word.

An owner or on-call alias is a **role alias, never a personal contact detail** (an `@` is
rejected). Raw URLs, tokens, credentials, private endpoints, and user text bodies never
cross the export boundary.

## Evidence

- Support export: `artifacts/release/m5-service-ownership-on-call-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-service-ownership-on-call-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-service-ownership-on-call-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-service-ownership-on-call-controls/`

Regenerate the checked artifacts and fixtures from the seed builder with:

```
GEN_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_ARTIFACTS=1 \
  cargo test -p aureline-release --lib generate_artifacts -- --ignored
```
