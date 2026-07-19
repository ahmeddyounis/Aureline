# M5 stable-line bundle-refresh-audit and shipping-line-drift-report registries

This lane keeps onboarding and migration promises honest after launch over the frozen
[M5 stable-line-protection matrix](./m5-stable-line-ops.md). It audits whether launch bundles, imported-user
handoff bundles, and org-approved bundles remain current, reversible, and supportable on the active shipping line.
It records the *bundle-refresh-audit* grammar (how a shipping line audits each claimed bundle it still owns — a
launch-bundle freshness audit, a launch-bundle reversibility audit, a missing-artifact audit, an
imported-user-handoff-bundle audit, an org-approved-bundle audit, or an unsupported-drift audit — with its exact
affected rows, freshness / reversibility state, missing-artifact posture, rollback / reversibility target, and
required refresh / narrow decision) and the *shipping-line-drift-report* grammar (the drift report emitted when an
audit finds shipping-line drift, recording whether the bundle went stale, became non-reversible, or drifted into
an unsupported / missing-artifact state, and naming the active drift reason) into registry resolvers that produce
export-safe, honest projections, so start-center, migration / help, release / support, admin / public-proof,
shiproom, executive-steering, and program-governance surfaces resolve one canonical bundle-currentness truth
instead of reading a stale launch bundle as silently supportable. The bundle-currentness audit and the drift
report are separated in runtime and serialized state: the audited bundle, affected rows, freshness / reversibility
state, rollback target, and retest posture live on the bundle-refresh audit, while the resolved line identity,
affected-claim reference, target-train reference, drift-scope state, narrowed-claim state, active drift reason, and
last drift revision live on the shipping-line drift report, and a line's rollback / reversibility posture stays
preserved so onboarding / migration / support language never runs ahead of a current, reversible bundle.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries` (the
  authoritative validator).
- **Combined schema:**
  `schemas/program/m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-supported-line-defect-ledger.schema.json`](../../schemas/program/m5-supported-line-defect-ledger.schema.json)
  (reused from the frozen matrix, the supported-line bundle-currentness / defect ledger) and
  [`schemas/program/m5-shipping-line-drift-report.schema.json`](../../schemas/program/m5-shipping-line-drift-report.schema.json)
  (minted by this lane) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:**
  `fixtures/release/m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries/`
  (`bundle_refresh_audit_beta_narrowed.json`, `shipping_line_drift_report_preview_narrowed.json`).

## Two registries

1. **Bundle refresh audit** (`resolve_bundle_refresh_audit_entry`) — records one typed bundle-currentness audit
   per claimed bundle: the audited-bundle kind and its canonical mode, the affected repo / journey rows, the
   bundle IDs, the install topology, the toolchain envelope, the known limits, the rollback / reversibility
   target, and the diagnostics / retest posture. A clean entry names a canonical registry token, a classified
   bundle-refresh-audit kind, and a stable-line-protection role, covers the canonical / accessible / audit
   resolution forms, publishes a complete object, preserves its rollback / reversibility posture before a claim
   widens, and keeps a public-facing bundle's onboarding / migration / support claim matched to a current,
   reversible bundle. Otherwise it degrades honestly — a line widening its claim while a claimed bundle is stale
   or non-reversible, or a public-facing bundle running its onboarding / migration language ahead of its refreshed
   state, degrades to
   `bundle_refresh_audit_lets_line_widen_without_rollback_or_runs_support_ahead_of_proof`, the structured blocker
   reason a widen-over-stale-bundle attempt must surface.
2. **Shipping-line drift report** (`resolve_shipping_line_drift_report_entry`) — emits the machine-readable drift
   report when an audit finds shipping-line drift. A clean entry names a classified drift scope (stale-bundle,
   non-reversible-bundle, or unsupported-bundle drift) and provides the complete line-identity / affected-claim /
   target-train / drift-scope / narrowed-claim / active-drift-reason / last-drift-revision report object; a report
   that would keep support language ahead of a refreshed bundle, hide the drift, or let a missing-artifact gap
   masquerade as covered degrades to
   `shipping_line_drift_report_runs_support_ahead_of_proof_or_drops_shipping_line_drift_report`.

## Per-entry bundle reference

The audited bundle carries its canonical mode, and the resolver publishes the full audit object, so the registry
— never a launch bundle assumed to have stayed current — is the single source of truth.
`bundle_refresh_audit_object_is_complete` rejects an object missing any audit field,
`line_preserves_rollback_and_diagnostics_before_widening` rejects a claim widening while a claimed bundle is stale
or non-reversible or onboarding / migration language running ahead of a refreshed bundle, and
`shipping_line_drift_report_stays_honest` rejects a report that has kept support language ahead of a refreshed
bundle.

## Acceptance criteria (proven by resolved examples)

- **A bundle refresh audit exists for each claimed launch or org-approved bundle on the active stable line, with
  exact freshness, reversibility, drift, and missing-artifact truth.** Clean bundle-refresh-audit entries cover the
  canonical launch-bundle-freshness / launch-bundle-reversibility / missing-artifact / imported-user-handoff /
  org-approved / unsupported-drift audits and the first release-center / shiproom / executive-steering /
  program-governance / support surfaces, an object-incomplete example degrades, and no clean bundle-refresh-audit
  entry published an incomplete object.
- **Stale, unsupported, or non-reversible bundle states automatically narrow affected onboarding / migration /
  support language until refreshed.** A widen-over-stale-bundle example and an unbound example degrade, a clean
  bundle-refresh-audit entry is present, and no clean entry is unbounded or unbound.
- **At least one product/help/support consumer renders shipping-line bundle drift or retest state from the audit
  packet rather than from hand-authored prose.** Clean shipping-line-drift-report entries cover the stale-bundle /
  non-reversible-bundle / unsupported-bundle drift scopes with full resolution-form coverage while providing the
  complete report object — the resolved line identity and the active drift reason — and a report that would keep
  support language ahead of a refreshed bundle or drop the drift degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh -- support-export
cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh -- csv
cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh -- report
cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh -- bundle-refresh-audit-table
cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh -- fixture-bundle-refresh-audit-beta-narrowed
cargo run -p aureline-ui --example dump_m5_stable_line_bundle_refresh -- fixture-shipping-line-drift-report-preview-narrowed
```
