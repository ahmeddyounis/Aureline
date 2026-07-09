# M5 Package-Management Component Surface Certification

Closing certification capstone for the B115 package-management component lane. It
certifies that the eight shared package-management components frozen in the
[component matrix](../../../schemas/ui/m5-package-management-component-matrix.schema.json)
— package-explorer-row, manifest-scope-switcher, install-review-sheet,
registry-or-mirror-row, script-risk-notice, lockfile-impact-card,
grouped-update-planner, and rollback-checkpoint-strip — present the same controlled
component truth on every claimed M5 package-management surface, with no hidden
manifest-scope, registry-source, auth, script-risk, or lockfile-churn drift.

Module:
`crates/aureline-deps/src/certify_package_management_component_truth_on_every_claimed_m5_package_management_surface`.

Boundary schema:
[`schemas/ui/m5-package-management-component-certification.schema.json`](../../../schemas/ui/m5-package-management-component-certification.schema.json).

## Certified surfaces

Eight claimed surfaces are certified: package explorer, dependency search / detail,
install-review sheet, help, support export, exported package-review packet, headless
CLI, and diagnostics. Each surface row scores six certification axes:

- **visual / keyboard / screen_reader / cli_export** — always-on parity axes: a
  claimed component must carry the same controlled truth in every form.
- **degraded_state** — narrows a claim honestly when manifest scope, registry
  freshness, auth state, lockfile impact, or rollback truth weakens.
- **scope_and_source_provenance** — the certification-specific separation axis: it
  keeps the manifest-scope, registry-source, script-risk, and lockfile-churn
  distinction explicit so a certified surface never implies its scope is full, its
  registry is fresh, that no scripts run, or that lockfile churn is small.

## Status

A surface earns `certified_parity` (green) only when its certified claim equals its
claimed claim, no axis narrows, and component truth is preserved. It narrows to
`narrowed_parity` (yellow) the moment an axis narrows or the certified claim drops
below the claimed one, and fails to `parity_blocked` (red) whenever the target
manifest, direct/transitive relation, registry source, auth posture,
script/native-build risk, lockfile churn, grouped-update reason, or
rollback/checkpoint identity is flattened out of the export. That last rule is the
delta of this capstone: certification may narrow a claim (AC2 — never overstate) but
may never drop the component's meaning.

## Claim tiers

Reuses `PackageComponentClaimTier` from the accessibility / auto-narrowing lane:
`full_reviewable_management` (6) > `manifest_range_scoped` (5) >
`mirror_or_offline_sourced` (4) > `auth_required_read_only` (3) >
`lockfile_impact_unknown` (2) > `rollback_unavailable_manual_recovery` (1). A
certified claim may never exceed the claim it certifies.

## Auto-narrowing

`apply_downgrade_automation` implements the AC2 release automation: a surface reported
with a flattened component truth blocks (red); a still-green surface whose manifest
scope / registry source went stale narrows its full-management claim to a disclosed
mirror-or-offline source, marks the `scope_and_source_provenance` axis narrowed, and
discloses the `registry_freshness_stale` trigger.

## Evidence

- Release proof:
  `artifacts/release/m5-package-management-certification-proof/{support_export.json,matrix.csv,report.md}`.
- Fixtures:
  `fixtures/ui/m5-package-management-component-certification/{registry_freshness_stale_auto_narrowed,search_detail_and_cli_narrowed}.json`.

Regenerate with
`GEN_PACKAGE_COMPONENT_CERTIFICATION_ARTIFACTS=1 cargo test -p aureline-deps --lib regenerate_package_component_certification_artifacts`.
