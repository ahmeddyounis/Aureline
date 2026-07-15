# M5 regression-asset and incident-close registries

This lane governs incident-close by attributable regression assets rather than postmortem folklore, over the
frozen [M5 launch-control matrix](./m5_launch_control_contract.md). It turns the *regression-asset* grammar (how
each Sev-1 / Sev-2 incident or launch-bearing failure links a regression asset — an automated test, a fixture
repository, a recovery drill, a protected-corpus case, a schema/policy guard, or a monitoring regression check —
and preserves the exact build, affected row, cohort/ring, and workaround lineage on that asset before the incident
closes) and the *incident-close* grammar (how a severe incident records the resolved incident identity, the linked
regression-asset ledger, the exact build and affected row, the cohort/ring lineage, and the close-lineage
freshness that keeps it queryable) into registry resolvers that produce export-safe, honest projections, so the
shiproom, release-center, executive-steering, program-governance, diagnostics, docs, CLI, support, and
public-proof surfaces resolve one canonical incident-close truth instead of a per-incident, hand-copied mailing
list. The regression-asset requirement and the incident-close record are separated in runtime and serialized
state: the regression asset type, exact build reference, affected row reference, cohort/ring reference, workaround
lineage, regression-asset reference, approved-exception reference, and close-blocker reference live on the
regression-asset entry, while the resolved incident identity, linked regression-asset ledger, exact build and row
reference, cohort/ring lineage state, close-lineage freshness state, workaround lineage reference, and last
incident-close revision live on the incident-close record, and a severe incident's regression-asset and lineage
posture stays visible so an incident never closes without a linked, attributable regression asset.

- **Canonical Rust module:** `crates/aureline-ui/src/m5_regression_asset_and_incident_close_registries` (the
  authoritative validator).
- **Combined schema:** `schemas/program/m5-regression-asset-and-incident-close-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/program/m5-regression-asset.schema.json`](../../schemas/program/m5-regression-asset.schema.json)
  and
  [`schemas/program/m5-incident-close.schema.json`](../../schemas/program/m5-incident-close.schema.json)
  as its canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-regression-asset-and-incident-close-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-regression-asset-and-incident-close-registries/`
  (`regression_asset_beta_narrowed.json`, `incident_close_preview_narrowed.json`).

## Two registries

1. **Regression asset** (`resolve_regression_asset_entry`) — publishes one typed regression-asset object per
   incident: the regression asset type and canonical asset-type mode, the exact build reference, the affected row
   reference, the cohort/ring reference, the workaround lineage, the regression-asset reference, the
   approved-exception reference, and the close-blocker reference. A clean entry names a canonical registry token, a
   classified regression asset type, and a launch-control role, covers the canonical / accessible / audit
   resolution forms, publishes a complete object, keeps its regression asset linked to the lane / cohort / build
   before closure, and — for a severe incident — keeps an attributable asset or an explicit approved exception
   matched to proof. Otherwise it degrades honestly — a severe incident closing without a linked regression asset,
   or an approved exception running ahead of proof, degrades to
   `incident_closes_without_regression_asset_or_runs_claim_ahead_of_proof`, the structured blocker reason a
   close-without-asset attempt must surface.
2. **Incident close** (`resolve_incident_close_entry`) — keeps the incident-close record honest and queryable. A
   clean entry names a classified incident severity (Sev-1, Sev-2, or launch-bearing failure) and provides the
   complete resolved-incident-identity / linked-regression-asset-ledger / exact-build-and-row / cohort-ring-lineage
   / close-lineage-freshness / workaround-lineage / last-incident-close-revision record; a record that would close
   an incident without a linked regression asset, drop the lineage, or let a lineage gap masquerade as covered
   degrades to `incident_close_drops_lineage_or_closes_without_regression_asset`.

## Per-entry regression-asset reference

The regression asset type carries its canonical mode, and the resolver publishes the full lineage object, so the
registry — never a hand-copied per-incident mailing list — is the single source of truth.
`regression_asset_object_is_complete` rejects an object missing any field,
`regression_asset_attributable_before_closure` rejects a severe incident that closes without a linked, attributable
regression asset, and `incident_close_stays_honest` rejects a record that would close an incident while its lineage
is dropped or a lineage gap is unflagged.

| regression asset type | asset-type mode | exact build reference | approved-exception reference | close-blocker reference |
| --- | --- | --- | --- | --- |
| automated test | automated_test_type | `repo.rows.core-team-canary-archetypes` | `rollback.target.canary-previous-stable` | `diagnostics.posture.full-telemetry` |
| fixture repository | fixture_repository_type | `repo.rows.migration-alpha-archetypes` | `rollback.target.migration-previous-toolchain` | `diagnostics.posture.migration-telemetry` |
| monitoring regression check | monitoring_regression_check_type | `repo.rows.certified-archetype-archetypes` | `rollback.target.certified-previous-stable` | `diagnostics.posture.certified-telemetry` |

A severe incident closing without a linked regression asset degrades to
`incident_closes_without_regression_asset_or_runs_claim_ahead_of_proof`, an incomplete object degrades to
`regression_asset_object_incomplete`, and a record that drops the lineage or closes without an asset degrades to
`incident_close_drops_lineage_or_closes_without_regression_asset`, so a close-without-asset attempt, an incomplete
object, or a dropped-lineage close can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Severe incidents cannot close without an attributable regression asset or an explicit approved exception.**
  Clean regression-asset entries cover the canonical automated-test / fixture-repository / recovery-drill /
  protected-corpus-case / schema-policy-guard / monitoring-regression-check asset types and the first
  release-center / shiproom / executive-steering / program-governance / support surfaces, an object-incomplete
  example degrades, and no clean regression-asset entry published an incomplete object.
- **Regression assets remain linked to the lane, cohort, and build that exposed the defect.** A
  close-without-asset example and an unbound example degrade, a clean linked-before-closure regression-asset entry
  is present, and no clean entry hides its lineage or is unbound.
- **Support and engineering can query incident-close lineage without relying on tribal memory.** Clean
  incident-close entries cover the sev-one / sev-two / launch-bearing-failure severities with full resolution-form
  coverage while providing the complete record — the resolved incident identity and the exact build and row
  reference — and a record that would close an incident without a linked regression asset or drop the lineage
  degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- support-export
cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- csv
cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- report
cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- regression-asset-table
cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- fixture-regression-asset-beta-narrowed
cargo run -p aureline-ui --example dump_m5_regression_asset_and_incident_close_registries -- fixture-incident-close-preview-narrowed
```
