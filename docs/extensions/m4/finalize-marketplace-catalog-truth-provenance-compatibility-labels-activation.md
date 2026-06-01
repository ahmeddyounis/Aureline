# Finalize marketplace catalog truth: provenance, compatibility labels, activation budgets, and support class

**Status:** Stable marketplace catalog-truth lane — implemented in `crates/aureline-extensions`.

## Goal

Promote the marketplace catalog row into the **stable line**. Every claimed stable marketplace row carries one canonical, checked-in catalog truth: the mechanically-sourced **provenance** posture, the **surface boundary** (runtime class, host boundary, hosted-surface / browser-handoff implications, and any reduced accessibility or theming parity), the **discoverability posture** (ranked / penalized / quarantined) kept *separate* from provenance and support-class truth, the machine-readable **compatibility scorecards** (each exposing a parity band, freshness window, evidence source, and downgrade state), the **activation-budget** instrumentation, the **support class** (with whether it is profile-limited and which runtime profile it was verified on), the **publisher-continuity** binding, and the cross-view **alignment**. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the evidence can no longer back a `stable` catalog claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. The marketplace result, detail, and compare rows, install review, diagnostics, support exports, the CLI inspector, and mirror packets ingest this packet instead of cloning badge text or collapsing catalog truth into ratings or install counts.

## Design principles

1. **Catalog truth ≠ discovery prominence** — Ranking state (`ranked_normally`, `penalized_staleness`, `penalized_performance`, `penalized_trust`, `quarantined`) lives in its own record, is explainable without raw install count or operator notes, and never implies support-class or provenance truth. A penalized row narrows to `beta`; a quarantined row is `withdrawn`.
2. **Provenance is mechanically sourced** — Verified-publisher / official-pack / enterprise-approved / community / under-review state is sourced from one registry/status model (`mechanically_sourced == true`), never from badge text copied into multiple views. An `under_review` provenance withdraws the stable claim.
3. **Compatibility never collapses into ratings** — Each scorecard (for an imported extension, bridge class, workflow bundle, or certified reference workspace) exposes a parity band, a freshness window, an evidence source, and a downgrade state. The aggregate `stable_marketplace_catalog_compatibility_summary` is re-derived from the scorecards at validation time, so a stored packet cannot hide an inherited, unsupported, or stale scorecard.
4. **No parity inheritance** — A scorecard whose evidence source is `inherited_from_adjacent` narrows below Stable, so stable discoverability never inherits parity from an adjacent bridge or bundle claim.
5. **Bounded activation cost** — The worst-case surface's activation cost carries a `budget_class` (`within_budget` / `over_budget` / `unbounded` / `not_measured`). An `unbounded` cost withdraws the row; `over_budget` narrows to `beta`; `not_measured` narrows to `preview`.
6. **Runtime-class truth travels** — The surface boundary carries the runtime class, the host boundary, and a `runtime_class_verified` flag; an unverified runtime class withdraws the row, and a hosted-surface or browser-handoff boundary must be disclosed. The support export preserves the runtime class, host boundary, and profile-limited support so a package verified on one runtime cannot imply parity everywhere else.
7. **No catalog-only stable claim** — A `stable` tier must be `evidence_backed`; a `catalog_asserted_only` basis narrows below Stable.
8. **Aligned across views** — A stable claim must keep its compatibility labels, support class, provenance, activation budget, and scorecard links aligned across the public registry, approved mirror, and side-load views.
9. **Downgraded-row banner** — A quarantined / under-review row, an unverified runtime class, an undisclosed hosted surface, a missing or unsupported / inherited scorecard, an unbounded activation cost, a revoked / missing continuity, a non-installable lifecycle, or a quarantined trust tier raises a banner that names the reason a reviewer must see before install or enablement.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_marketplace_catalog_truth_packet` | Top-level packet consumed by marketplace result / detail / compare rows, install review, diagnostics, support export, docs/help, release packets, the CLI inspector, and mirror packets. |
| `stable_marketplace_catalog_truth_identity` | Catalog descriptor ref, package identity, pinned catalog-truth version, publisher trust tier, lifecycle state. |
| `stable_marketplace_catalog_provenance` | Provenance class, mechanically-sourced flag, registry/status source ref. |
| `stable_marketplace_catalog_surface_boundary` | Runtime class, host boundary, runtime-class-verified flag, hosted-surface / browser-handoff implications, reduced accessibility / theming parity. |
| `stable_marketplace_catalog_discoverability_posture` | Ranking state and rationale ref, kept separate from provenance and support-class truth. |
| `stable_marketplace_catalog_compatibility_scorecard` | Per-subject parity band, freshness window, evidence source, downgrade state, and scorecard ref. |
| `stable_marketplace_catalog_compatibility_summary` | Inherited / not-evaluated / stale / unsupported / parity-limited counts and present subjects, derived from the scorecards. |
| `stable_marketplace_catalog_activation_budget` | Worst-case surface activation-cost posture, measured-cost and ceiling refs, measured surface count. |
| `stable_marketplace_catalog_support_class` | Support class, profile-limited flag, verified runtime profile, evidence ref. |
| `stable_marketplace_catalog_publisher_continuity` | Publisher-continuity state and continuity-packet ref. |
| `stable_marketplace_catalog_view_alignment` | Views the truth is aligned across and the runtime-class-preserved flag. |
| `stable_marketplace_catalog_truth_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_marketplace_catalog_downgraded_banner` | Whether a row-review banner must display and why. |
| `stable_marketplace_catalog_truth_inspection` | Compact boolean/count projection for CLI and dashboard surfaces. |
| `stable_marketplace_catalog_truth_support_export` | Metadata-safe support / partner / mirror export row that preserves runtime-class and profile-limited support truth. |

## Closed vocabularies

### Provenance classes
`verified_publisher`, `official_pack`, `enterprise_approved`, `community`, `under_review` (only `under_review` blocks a stable claim)

### Ranking states (discoverability, separate from trust)
`ranked_normally`, `penalized_staleness`, `penalized_performance`, `penalized_trust`, `quarantined`

### Runtime classes
`passive_package`, `wasm_capability_sandbox`, `declarative_host_rendered_view`, `external_host`, `compatibility_bridge`, `remote_side_component`

### Host boundary classes
`in_process_passive`, `wasm_sandbox`, `supervised_external_process`, `host_rendered_surface`, `hosted_remote_surface`, `browser_handoff`

### Scorecard subjects
`imported_extension`, `bridge_class`, `workflow_bundle`, `certified_reference_workspace`

### Parity bands
`full_parity`, `high_parity`, `partial_parity`, `limited_parity`, `unsupported`

### Freshness windows
`current`, `aging`, `stale`, `expired`, `not_evaluated`

### Evidence sources
`conformance_suite`, `certified_workspace`, `bridge_matrix`, `vendor_attested`, `inherited_from_adjacent` (the last never backs a stable claim)

### Scorecard downgrade states
`none`, `narrowed`, `downgraded`, `unsupported`

### Activation-budget classes
`within_budget`, `over_budget`, `unbounded`, `not_measured` (only `within_budget` keeps a stable claim)

### Support classes
`certified`, `supported`, `limited`, `community`, `experimental`, `unsupported` (`certified` / `supported` / `community` are stable-grade)

### Publisher-continuity states
`current`, `stale`, `missing`, `revoked` (only `current` keeps a stable claim)

### Catalog views
`public_registry`, `approved_mirror`, `side_load` (a stable claim must align across all three)

### Stability tiers
`stable`, `beta`, `preview`, `withdrawn` (only `stable` is a stable catalog-truth claim)

### Claim basis
`evidence_backed`, `catalog_asserted_only` (only `evidence_backed` keeps a stable claim)

### Narrowing reasons
`catalog_version_not_published`, `catalog_only_trust_not_evidence_backed`, `trust_tier_quarantined`, `lifecycle_not_installable`, `provenance_under_review`, `quarantined_from_discovery`, `ranking_penalized`, `runtime_class_unverified`, `hosted_surface_boundary_undisclosed`, `missing_required_scorecard`, `compatibility_unsupported`, `compatibility_parity_limited`, `scorecard_parity_inherited`, `scorecard_evidence_not_run`, `scorecard_freshness_stale`, `activation_cost_unbounded`, `activation_cost_over_budget`, `activation_cost_not_measured`, `support_class_below_stable_grade`, `support_class_limited`, `support_class_profile_limited`, `publisher_continuity_revoked`, `publisher_continuity_missing`, `publisher_continuity_stale`, `views_not_aligned`, `attribution_incomplete`

Each narrowing reason is partitioned into exactly one severity bucket: `withdrawn` (the row cannot be trusted as stable catalog truth at all), `preview` (a structural / trust / disclosure shortfall), or `beta` (a safe narrowing).

## How to verify

```bash
cargo test -p aureline-extensions finalize_marketplace
cargo run -q -p aureline-extensions --example dump_stable_marketplace_catalog_truth_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_marketplace_catalog_truth.schema.json` (checked with a Draft 2020-12 validator).

## Source of truth

The Rust types in
`crates/aureline-extensions/src/finalize_marketplace_catalog_truth_provenance_compatibility_labels_activation/`
are canonical. The companion schema is
`schemas/extensions/stable_marketplace_catalog_truth.schema.json`; canonical fixtures live under
`fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/`.
