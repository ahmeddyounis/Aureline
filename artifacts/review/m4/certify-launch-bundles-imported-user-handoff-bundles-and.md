# Artifact: Certify launch bundles, imported-user handoff bundles, and org-approved bundles against archetype reports

**Task:** Turn workflow bundles into reviewable, reversible launch-wedge artifacts whose certification claims resolve to machine-readable compatibility scorecards and downgrade automatically.
**Status:** Implemented
**Verification class:** Automated functional + Conformance / interoperability suite + Design QA / UX validation + Release evidence review

## Summary

This lane certifies launch bundles, imported-user handoff bundles, and org-approved bundles against archetype reports. It binds bundle identity, a machine-readable compatibility scorecard, and the certification claim a bundle may render into one validated packet. A stable claim — `Certified` or `Managed approved` — is only allowed when it resolves to a *current* scorecard row whose freshness, bridge state, downgrade state, and known-gap state still support it. Stale evidence, narrowed bridge parity, a disclosed gap, a missing/mismatched row, a prose-only basis, or a missing reference workspace automatically downgrade the visible badge and record machine-readable reasons. Imported-user handoff bundles preserve their migration report and unsupported-item lists; offline and mirror flows keep the same scorecard vocabulary; and no bundle application may carry hidden setup hooks, secret injection, or silent trust widening.

## What changed

- New Rust module: `crates/aureline-workspace/src/certify_launch_bundles_imported_user_handoff_bundles_and/mod.rs`
- Re-exported from `crates/aureline-workspace/src/lib.rs`
- New schema: `schemas/review/certify-launch-bundles-imported-user-handoff-bundles-and.schema.json`
- New fixtures: `fixtures/review/m4/certify-launch-bundles-imported-user-handoff-bundles-and/`
  - `certified_launch_bundle_current.json` — certified launch bundle resolves to a current row and holds.
  - `org_approved_managed_current.json` — managed/org-approved bundle resolves to a current row and holds, mirror-first distribution.
  - `imported_user_handoff_preserved.json` — imported-user bundle preserves migration report + unsupported items; makes no stable claim.
  - `stale_certification_auto_downgrade.json` — certified claim downgrades to `retest_pending` when its scorecard row expires.
- New tests: `crates/aureline-workspace/tests/certify_launch_bundles_imported_user_handoff_bundles_and_alpha.rs`
- New docs: `docs/review/m4/certify-launch-bundles-imported-user-handoff-bundles-and.md`

## Acceptance criteria

- [x] Certified, Managed approved, Community, Imported, and Local draft bundle states are visible and test-covered on the stable line, and stale certification automatically downgrades the visible claim. (`stale_certification_auto_downgrade.json`, `stale_row_downgrades_automatically`)
- [x] Bundle/archetype claims reference machine-readable compatibility scorecards linking bundle id, imported-extension class, bridge state, supported deployment/profile rows, and certified reference-workspace ids. (`bundle_compatibility_scorecard_row`)
- [x] Imported-user and org-approved badges may not imply parity from prose alone; any claimed stable handoff points to a current scorecard row with freshness, downgrade, and known-gap state. (`no_prose_only_stable_claim`, `prose_only_stable_claim_is_downgraded`)
- [x] Launch-bundle, imported-user, and org-approved claims resolve to current scorecard rows and downgrade automatically when freshness expires or bridge parity narrows. (`narrowed_bridge_downgrades_automatically`, `reference_workspace_missing_downgrades`)
- [x] Imported-user handoff bundles preserve migration reports and unsupported-item lists rather than collapsing into one green banner. (`imported_user_handoff_preserved.json`, `imported_user_bundle_requires_preserved_handoff`)
- [x] Offline and mirror-first flows preserve the same scorecard vocabulary instead of degrading into opaque archive import. (`offline_distribution_keeps_scorecard_vocabulary`, `empty_scorecard_is_rejected`)
- [x] Bundle identity (id, signer/source, archetype class, compatible Aureline range, certification state) is one canonical record consumed by every surface.

## Guardrails honored

- Bundle UX never implies exact parity when the scorecard says approximate or unsupported: an `approximate_bridge`/`unsupported_bridge` row narrows a stable claim to `limited`.
- Bundle application never hides trust changes or managed dependencies behind a convenience path: `allows_hidden_setup_hooks`, `allows_secret_injection`, and `allows_silent_trust_widening` are forced false and validated.
- Public scope is not widened: the module adds one bounded packet family on top of the existing `bundles` review packet without altering it.

## How to verify

```bash
cargo test -p aureline-workspace --test certify_launch_bundles_imported_user_handoff_bundles_and_alpha
cargo test -p aureline-workspace --lib certify
```

Materialized packets for every fixture validate against `schemas/review/certify-launch-bundles-imported-user-handoff-bundles-and.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The scorecard is consumed as input; a later revision should source it directly from the certification pipeline that produces `artifacts/compat/bundle_scorecards/*` instead of accepting a producer-supplied row.
- Freshness expiry is declarative (`stale_past_window` / `downgraded_freshness_expired` supplied by the producer). When a wall-clock evaluation lane lands, the downgrade can be derived from `freshness_expires_at` versus evaluation time.
- Bridge-state and imported-extension classes are closed string vocabularies; when the provider/migration crates stabilize typed enums, these should be narrowed.
