# M5 Governance-Dashboard Component Surface Certification (M05-1059)

Closing capstone for the B125 governance-dashboard-component lane. Where the freeze
matrix
(`freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`)
defines the nine reusable components, the M05-1053..1056 implement lanes narrow each
one, the M05-1057 consumer lane adopts them, and the M05-1058 accessibility lane
proves keyboard / screen-reader / CLI-export parity and per-family auto-narrowing,
this lane **certifies** that the shared component truth holds on every claimed M5
assurance, operator, and shiproom surface — and automatically narrows any surface
that cannot sustain it.

- Rust module: `crates/aureline-release/src/certify_fitness_tile_report_row_waiver_gate_mitigation_ownership_on_call_decision_and_milestone_dashboard_component_truth_on_every_claimed_m5_assurance_operator_and_shiproom_surface/`
- Boundary schema: `schemas/ui/m5-governance-dashboard-component-certification.schema.json`
- Release proof: `artifacts/release/m5-governance-dashboard-component-certification-proof/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- Fixtures: `fixtures/ui/m5-governance-dashboard-component-certification/`

## What is certified

The packet is keyed on the claimed **surface** a user or operator reads readiness,
ownership, or a ship/no-ship decision through — not on the component family it
renders. The eight certified surfaces are:

`assurance_center`, `operator_overview`, `release_center`, `shiproom`,
`service_health`, `support_export`, `docs_help`, `cli_headless`.

Each row certifies its surface across six truth axes — exactly the parity
dimensions the spec requires verifying:

| Axis | Meaning |
| --- | --- |
| `visual` | fitness reading / provenance, readiness state, waiver expiry, owner / backup coverage, on-call / escalation route, decision forum, and blocker / waiver counts are shown on the primary surface |
| `keyboard` | the same fitness / ownership / waiver / decision truth and its actions are reachable without a pointer |
| `screen_reader` | the same truth is announced non-visually, never relying on color or a badge glyph alone |
| `cli_export` | **always-on**: the certified surface state is reconstructable as text / JSON / Markdown for support and automation |
| `degraded_state` | stale evidence or a partial proof honestly downgrades a `GovernedPass` / `GovernedResolved` claim to degraded / provisional rather than reading current |
| `governance_truth` | waiver expiry, owner / backup coverage, on-call gap, mitigation language, and decision-right authority stay explicit and never read as a clean pass when the lane is waived, ownerless, forumless, or jargon-hidden |

Each surface also cites the frozen component families it renders
(`consumed_families`). Across the whole packet every one of the nine families must
be certified on at least one surface (`all_families_covered`), which is how this
capstone proves the full component matrix runs across the claimed consumers.

## Governance-support claim ladder

The claim a surface asserts (and the weakest ceiling it is certified down to) is
the reused M05-1058 `M5GovernanceSupportClaim` ladder, strongest first:

`governed_pass` > `governed_resolved` > `degraded` > `provisional` >
`waiver_gated` > `blocked`.

Certification may only **narrow** a claim, never strengthen it. Only `governed_pass`
is a clean green pass; a waived, stale, ownerless, or forumless lane can never reach
it.

## Verdict derivation (green / yellow / red)

The `derived_status` on every row is always recomputed from the axis outcomes and
claim narrowing — never asserted. The invariant is **a degraded axis must produce
a visible claim narrowing**.

- **Green** — every axis certified and the claimed governance-support claim is
  delivered (`claimed_claim == certified_claim`, no `claim_auto_narrow`).
- **Yellow** — an axis is not current and the surface discloses the reduction by
  narrowing its claim to the weakest supported ceiling. The `claim_auto_narrow`
  block must bind to a non-always-on axis that is `disclosed_narrowed`, carry a
  precise (non-generic) `visible_label`, and its `from_claim`/`to_claim` must
  match the row's `claimed_claim`/`certified_claim`. The narrowed axis outcome
  names a frozen `M5GovernanceDowngradeTrigger`.
- **Red** — any of: an axis is `undisclosed_drift`; the always-on `cli_export`
  axis is not certified (or copy/export is incomplete); the certified claim is
  stronger than the claimed one; a degraded axis is retained behind a full claim
  with no narrowing; or the narrowing block is inconsistent (spurious, wrongly
  bound, generic-labelled, or bound to the always-on axis). Red surfaces block
  the release; gaps are expressed as narrowed (yellow) claims or blocked (red)
  rows, never as hidden exceptions.

Every row cites exactly one canonical governance-proof bundle —
`artifacts/release/m5-governance-dashboard-proof/support_export.json`, the frozen
governance-dashboard component matrix proof — rather than cloning per-surface
evidence, and records the M05-1058 accessibility support export as supporting
evidence. The packet is metadata-only: raw evidence, waiver credentials, owner
contact detail, and escalation secrets never cross this boundary.

## Seed certification

The checked-in packet certifies all eight surfaces: **4 green / 4 yellow / 0 red**.

| Surface | Claimed → Certified | Status | Binding axis |
| --- | --- | --- | --- |
| assurance_center | governed_pass → governed_pass | green | — |
| release_center | governed_pass → governed_pass | green | — |
| support_export | governed_resolved → governed_resolved | green | — |
| docs_help | governed_resolved → governed_resolved | green | — |
| operator_overview | governed_pass → provisional | yellow | degraded_state |
| service_health | governed_pass → degraded | yellow | governance_truth |
| shiproom | governed_pass → degraded | yellow | governance_truth |
| cli_headless | governed_resolved → waiver_gated | yellow | governance_truth |

## Regenerating the proof

The on-disk `support_export.json` is the `include_str!` canonical for the
round-trip test. Regenerate the artifacts and fixtures after any change to the
seeded builder:

```
GEN_GOVERNANCE_DASHBOARD_CERT_ARTIFACTS=1 cargo test -p aureline-release \
  certify_fitness_tile_report_row_waiver_gate_mitigation_ownership_on_call_decision_and_milestone_dashboard_component_truth_on_every_claimed_m5_assurance_operator_and_shiproom_surface::tests::generate_artifacts
```

Then rebuild so the baked-in `include_str!` picks up the new content, and run:

```
cargo test -p aureline-release --lib \
  certify_fitness_tile_report_row_waiver_gate_mitigation_ownership_on_call_decision_and_milestone_dashboard_component_truth_on_every_claimed_m5_assurance_operator_and_shiproom_surface
```
