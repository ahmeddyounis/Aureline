# Artifact: Finalize marketplace catalog truth — provenance, compatibility labels, activation budgets, and support class

**Task:** Promote the marketplace catalog row into the stable line — bind the mechanically-sourced provenance, the surface boundary (runtime class, host boundary, hosted-surface / browser-handoff implications, reduced accessibility / theming parity), the discoverability posture (ranked / penalized / quarantined) kept separate from trust, the machine-readable compatibility scorecards (parity band, freshness window, evidence source, downgrade state), the activation-budget instrumentation, the support class (with profile-limited and verified-runtime-profile truth), the publisher-continuity binding, and the cross-view alignment into one validated packet, and derive the stability qualification with automatic narrowing below Stable so users, admins, and reviewers can all inspect and trust the row.
**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds the catalog row identity (catalog descriptor ref, package identity, pinned catalog-truth version, publisher trust tier, lifecycle state), the provenance posture (`verified_publisher` / `official_pack` / `enterprise_approved` / `community` / `under_review`, mechanically sourced from one registry/status model), the surface boundary (runtime class, host boundary, runtime-class-verified flag, hosted-surface and browser-handoff implications, reduced accessibility / theming parity), the discoverability posture (`ranked_normally` / `penalized_*` / `quarantined`, explainable without raw install count) kept separate from provenance and support-class truth, the machine-readable compatibility scorecards (each with a parity band, freshness window, evidence source, and downgrade state), the aggregate compatibility summary (derived from the scorecards), the worst-case activation-budget instrumentation, the support class (with a profile-limited flag and the runtime profile it was verified on), the publisher-continuity binding, and the cross-view alignment into one validated packet, and derives the stability qualification it may claim. A `stable` catalog-truth claim is only allowed when the row pins the published catalog-truth version, is evidence-backed, keeps its publisher trust tier out of quarantine, stays on an installable lifecycle, keeps its provenance out of under-review, is ranked normally, carries a verified runtime class, discloses any hosted-surface or browser-handoff boundary, ships at least one compatibility scorecard with no inherited / unsupported / not-yet-run / stale / parity-limited evidence, keeps its activation cost bounded and within budget, keeps a stable-grade non-profile-limited support class, keeps its publisher continuity current, keeps its truth aligned across all views, and is fully attributed. A quarantined / under-review row, an unverified runtime class, an unsupported scorecard, an unbounded activation cost, or a revoked continuity withdraws the row; a penalized ranking, a stale or parity-limited scorecard, an over-budget activation cost, a limited or profile-limited support class, or a stale continuity narrows to `beta`; a catalog-asserted basis, a quarantined trust tier, an undisclosed hosted surface, an inherited-parity or not-yet-run scorecard, a not-measured budget, a missing continuity, unaligned views, an experimental / unsupported support class, or an incomplete attribution narrows to `preview`. When any condition fails the visible tier is automatically narrowed below Stable with machine-readable reasons. The checked-in packet is canonical: the marketplace result / detail / compare rows, install review, diagnostics, support exports, the CLI inspector, and mirror packets ingest it instead of cloning badge text or collapsing catalog truth into ratings or install counts.

## What changed

- New Rust module: `crates/aureline-extensions/src/finalize_marketplace_catalog_truth_provenance_compatibility_labels_activation/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_marketplace_catalog_truth.schema.json`
- New fixtures: `fixtures/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation/`
  - `verified_publisher_stable_current.json` — a verified-publisher wasm-sandbox row, ranked normally, runtime class verified, two fresh evidence-backed scorecards, activation within budget, certified support, continuity current, aligned across all views; it holds Stable.
  - `scorecard_freshness_stale_narrows_to_beta.json` — an official pack whose workflow-bundle scorecard freshness window is stale; it narrows to `beta` without a hard block.
  - `catalog_asserted_narrows_to_preview.json` — a community theme pack claiming Stable on catalog assertion alone (`catalog_asserted_only`); it narrows to `preview`.
  - `quarantined_from_discovery_withdrawn.json` — an enterprise-approved adapter quarantined from prominent discovery; the row is `withdrawn`, a row-review banner is raised, and the quarantine reason is surfaced.
  - `unsupported_scorecard_withdrawn.json` — a legacy bridge whose imported-extension scorecard is unsupported and whose support is limited and profile-limited; the row is `withdrawn`, a banner is raised, and the unsupported scorecard count is surfaced.
- New dump example: `crates/aureline-extensions/examples/dump_stable_marketplace_catalog_truth_records.rs`
- New docs: `docs/extensions/m4/finalize-marketplace-catalog-truth-provenance-compatibility-labels-activation.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_marketplace_catalog_truth.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`scorecard_freshness_stale_narrows_to_beta.json`, `catalog_asserted_narrows_to_preview.json`, `quarantined_from_discovery_withdrawn.json`, `unsupported_scorecard_withdrawn.json`, `catalog_version_mismatch_narrows_below_stable`)
- [x] Users and admins can inspect permissions / runtime boundary (surface boundary), compatibility range (compatibility scorecards with parity band / freshness window / evidence source / downgrade state), activation cost (activation budget), lifecycle label, publisher provenance (provenance + continuity), and rollback/revocation state (continuity `revoked` / lifecycle) for the touched ecosystem row. (`stable_marketplace_catalog_truth_inspection`, scorecards, `stable_marketplace_catalog_activation_budget`, `stable_marketplace_catalog_provenance`, `stable_marketplace_catalog_publisher_continuity`)
- [x] Conformance fixtures, activation-budget instrumentation, and a publisher-continuity packet make the ecosystem claims supportable and mirrorable. (compatibility summary derived from scorecards, `stable_marketplace_catalog_activation_budget`, `stable_marketplace_catalog_publisher_continuity`, all five fixtures)
- [x] Marketplace fact-grid audits prove runtime class, hosted-surface boundary, and profile-limited support labels remain visible before install, after install, and in exported support or mirror packets. (`stable_marketplace_catalog_surface_boundary`, `support_class.profile_limited`, `stable_marketplace_catalog_truth_support_export` preserving runtime class / host boundary / profile-limited support, `support_export_preserves_runtime_class_truth`)
- [x] Ranking state, support class, provenance, and quarantine/under-review posture can be explained without relying on raw install count or manual operator notes. (`ranking_explained_without_install_count` enforced, `ranking_rationale_ref`, separate provenance / support-class / discoverability records)
- [x] Compatibility labels expose parity band, freshness window, evidence source, and downgrade state; stable discoverability may not inherit parity from adjacent bridge or bundle claims. (`stable_marketplace_catalog_compatibility_scorecard`, `inherited_parity_scorecard_narrows_below_stable`)
- [x] Compatibility labels, support class, provenance, activation budgets, and scorecard links remain aligned across public registry, mirror, and side-load views. (`stable_marketplace_catalog_view_alignment`, `unaligned_views_narrow_below_stable`)

## Guardrails honored

- No ambient extension privilege / catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_narrows_to_preview.json`, `no_catalog_only_stable_claim`); `allows_catalog_only_trust` is pinned false.
- No unbounded activation cost: an `unbounded` budget withdraws the row (`unbounded_activation_cost_withdraws_the_row`); `over_budget` narrows to `beta`; `allows_unbounded_activation_cost` is pinned false.
- No parity inheritance: an `inherited_from_adjacent` scorecard narrows below Stable (`inherited_parity_scorecard_narrows_below_stable`); `allows_inherited_parity_stable_claim` is pinned false.
- No discovery-prominence-implied trust: ranking lives in its own record, must be explainable without install count, and never implies support-class or provenance truth; `allows_ranking_implied_trust` is pinned false. A quarantined row is withdrawn (`quarantined_from_discovery_withdraws_the_row`).
- Provenance is mechanically sourced from one registry/status model (`non_mechanically_sourced_provenance_is_rejected`), never copied badge text.
- Runtime-class truth cannot be implied everywhere: an unverified runtime class withdraws the row (`unverified_runtime_class_withdraws_the_row`) and the support export preserves the runtime class, host boundary, and profile-limited support.
- A narrower stable claim is published rather than papered over: the compatibility summary is re-derived from the scorecards and the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions finalize_marketplace
cargo run -q -p aureline-extensions --example dump_stable_marketplace_catalog_truth_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_marketplace_catalog_truth.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The provenance class, trust tier, and ranking state are producer-supplied closed strings; when the registry/status model exposes typed enums, the `provenance_class` and `ranking_state_class` should be sourced from that model directly rather than re-declared as strings here.
- The activation budget is summarized as a single worst-case surface posture; a later revision should carry a per-surface activation row so a reviewer can see which surface drove the worst case.
- Trust-tier, lifecycle, runtime-class, activation-budget, stability-tier, and claim-basis vocabularies are closed string sets shared with the SDK-author-lane, manifest-hardening, runtime-ABI, external-host, and Wasm-host-governance stable lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The compatibility scorecards carry refs to the machine-readable scorecard payloads; a later revision should resolve each bound scorecard's own freshness rather than accepting a producer-supplied `freshness_window_class`.
- The view alignment records which views the truth is asserted to align across; a later revision should diff the actual per-view rendered truth rather than accept a producer-supplied `runtime_class_preserved_across_views` flag.
