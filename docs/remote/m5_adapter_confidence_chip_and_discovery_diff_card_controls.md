# M5 adapter-confidence-chip and discovery-diff-card controls

This is the first implement lane over the frozen
[M5 build/remote-boundary component matrix](m5_build_remote_boundary_components_contract.md). It
turns the two build-intelligence confidence components the matrix names — the **adapter-confidence
chip** and the **discovery-diff card** — into resolvers that produce export-safe, honest
projections instead of feature-local confidence copy.

The authoritative gate is the Rust validator in `crates/aureline-remote`
(`implement_the_m5_adapter_confidence_chip_and_discovery_diff_card_..._primitive`). The checked-in
support export under
`artifacts/release/m5-adapter-confidence-chip-discovery-diff-card-controls-proof/` and the narrowed
fixtures under `fixtures/ui/m5-adapter-confidence-chip-discovery-diff-card-controls/` are minted only
by the seed builders through the headless emitter
`cargo run -p aureline-remote --example dump_m5_adapter_discovery_controls`.

## Goal

Make target discovery confidence explicit before a user runs, debugs, previews, or hands off work on
the wrong host or adapter path.

## Components

### Adapter-confidence chip

`resolve_adapter_confidence_chip` renders every chip with:

- the adapter / source class (bound from `TargetDiscoveryClass`),
- the confidence band (bound from `AdapterConfidence`),
- the heuristic-vs-structured-vs-imported discovery mode (bound from `DiscoveryConfidence`), and
- the current downgrade reason (bound from `NarrowingReason`) whenever the resolved certainty is
  genuinely reduced.

The chip resolves to one of six controlled certainties — `exact`, `compatible`, `heuristic`,
`imported`, `downgraded`, or `stale` — so a user can read the target's certainty before invoking the
action. A chip that leaves its source class, confidence band, or discovery mode unstated degrades
rather than reading as a clean pass, and a downgraded or stale target that carries no attributed
reason degrades too.

### Discovery-diff card

`resolve_discovery_diff_card` renders whenever the resolved target changes materially, naming:

- the previous and current target identity,
- the previous and current discovery confidence and the changed certainty, and
- a review-before-switch affordance.

A material change presented without an attributable review state degrades to
`silent_relabel_without_review` (AC2). A weaker discovery result that would replace a
higher-confidence resolved target without an explicit review state degrades to
`lower_confidence_overwrote_resolved` — the no-higher-confidence-overwrite guardrail — so stale or
weaker discovery can never silently overwrite a stronger resolved target.

## Acceptance criteria

- **AC1** — users can see when a target is exact, compatible, heuristic, imported, downgraded, or
  stale before invoking the action.
- **AC2** — material discovery drift produces an attributable review state instead of a silent
  relabel.
- **AC3** — target discovery language stays consistent across run/test/debug, preview, AI tool
  routing, and support/export surfaces.

Each criterion is proven by the resolved examples the packet carries, not merely asserted by
governance flags.

## Hard invariants (every controls row)

- never relabel a materially changed target without an attributable review state,
- never let lower-confidence discovery overwrite a higher-confidence resolved target without review,
- never hide the adapter confidence, source class, or discovery mode, and
- never conceal a downgrade or drift behind generic status wording.

Raw secret values and private endpoints never cross this boundary.
