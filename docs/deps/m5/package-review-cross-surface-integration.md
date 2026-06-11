# Package-review and dependency-health cross-surface integration

This document describes the canonical packet that carries dependency/package
cards from the desktop dependency workspace into higher-order surfaces —
framework-pack health bundles, review workspaces, incident bundles, and
companion-safe inspect views — without losing package identity, manifest scope,
support class, advisory freshness, or its local-versus-managed source label, and
without smuggling in write authority the host surface does not have. It is the
user-facing companion to the governed artifact at
`artifacts/deps/m5/package-review-cross-surface-integration.json` and the typed
model in the `aureline-deps` crate
(`package_review_cross_surface_integration`).

## What this packet covers

The packet answers, for every dependency/package card and every cross-surface
handoff:

1. **Where is this card shown?** A **surface class** of `desktop_workspace`,
   `framework_pack_health`, `review_workspace`, `incident_bundle`,
   `companion_inspect`, or `browser_handoff`.
2. **What may it do there?** A **write authority** of `inspect_only`,
   `review_stage`, or `full_mutation`.
3. **What package is it?** A stable **package identity** (coordinate, ecosystem,
   manifest path, scope kind) that survives every surface crossing.
4. **What is its support, source, and freshness?** A **support class**, a
   **source label** (`local`, `managed`, `mirrored`, `imported`), and an
   **advisory freshness** that travel with the card.
5. **Is the finding live or imported?** A **finding truth** of `live` (current
   local analysis) or `imported` (ingested from a feed snapshot).
6. **Does the move keep truth?** A **handoff continuity row** that binds a
   transition back to the originating card.

## Write authority never exceeds the host surface

Only the desktop workspace mutates packages. The model pins the highest
authority each surface may carry:

- **`desktop_workspace`** — may carry `full_mutation`.
- **`review_workspace`** — may carry at most `review_stage`; it can request or
  stage a change but cannot apply it.
- **`framework_pack_health`, `incident_bundle`, `companion_inspect`,
  `browser_handoff`** — stay `inspect_only`.

A card whose authority exceeds its surface's ceiling is a validation failure
(`WriteAuthorityOverreach`). This is the guardrail that stops a companion or
browser-adjacent card from implying hidden mutation parity it does not have.

Because an `applied` or `rolled_back` review state implies a real mutation
happened, a card claiming either state without `full_mutation` authority is a
validation failure (`MutationStateWithoutAuthority`).

Companion and browser-adjacent cards must also expose an explicit read-only
inspect entry (`inspect_ref`); omitting it is a validation failure
(`MissingInspectRef`), so an inspect-only surface is always clearly inspect-only.

## Live vs imported finding truth

A `live` finding is computed by current local analysis of the exact build; an
`imported` finding is ingested from a scanner, feed, or snapshot. The model
constrains the claim:

- **`live`** — only permitted when the card's source label is `local` **and**
  advisory freshness is `current`.
- **`imported`** — accepts any source label and freshness.

A card claiming a `live` finding from a non-local or non-current source is a
validation failure (`OverstatedFindingTruth`). This keeps imported, mirror, and
snapshot findings from masquerading as freshly verified across surfaces.

## Handoffs preserve identity, update class, and review state

When a card crosses surfaces, a handoff continuity row records the move:

- **`desktop_reopen`** — must land on `desktop_workspace`.
- **`browser_handoff`** — must land on `browser_handoff`.
- **`companion_followup`** — must land on `companion_inspect`.

A transition whose landing surface disagrees with its kind is a validation
failure (`TransitionSurfaceMismatch`), so a browser or companion handoff can
never quietly land on a mutation-capable surface.

Each handoff references an originating card by id. The handoff must reproduce the
card's package identity, update class, and review state exactly; dropping any of
them is a validation failure (`HandoffDropsTruth`), and referencing a card the
packet does not declare is a validation failure (`HandoffUnknownCard`). This is
how package identity, update class, and review state stay stable across desktop
reopen, browser handoff, and companion follow-up rather than being flattened into
a screenshot or prose.

## Scope and identity are always disclosed

Every card and handoff carries a manifest scope kind — `full_repo`,
`single_manifest`, or `slice` — alongside the coordinate, ecosystem, and manifest
path, so dependency state crossing into review, incident, or companion surfaces
never loses package identity or manifest scope.

This packet introduces no hosted-only mutation plane and grants no new write
authority to browser or companion surfaces. It is metadata-only and carries no
credential bodies or raw provider payloads.
