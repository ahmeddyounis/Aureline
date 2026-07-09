# M5 package-management component matrix (design)

This is the canonical design reference for the reusable M5 package-management
components. Design, release, help, and support packets reference **this one
matrix** instead of scattered dialog definitions, and new package-management
work reuses these governed components rather than inventing local scope, auth, or
risk UI.

- Typed model: `aureline-deps` crate,
  `freeze_the_m5_package_management_component_matrix`.
- Boundary schema: `schemas/ui/m5-package-management-component-matrix.schema.json`.
- Contract doc: `docs/deps/m5/freeze_the_m5_package_management_component_matrix.md`.
- Release evidence: `artifacts/release/m5-package-management-proof/`.
- Narrowed fixtures: `fixtures/ui/m5-package-management-components/`.

## Component family

| Component | Maturity | Canonical source contract |
| --- | --- | --- |
| Package explorer row | Stable | `schemas/ui/m5-dependency-row.schema.json` |
| Manifest-scope switcher | Stable | `schemas/runtime/manifest_scope_alpha.schema.json` |
| Install-review sheet | Stable | `schemas/ecosystem/m5-install-review.schema.json` |
| Registry or mirror row | Stable | `schemas/ui/m5-mirror-offline-artifact-row.schema.json` |
| Script-risk notice | Beta | `schemas/governance/post_install_disclosure.schema.json` |
| Lockfile-impact card | Stable | `schemas/runtime/lockfile_impact_alpha.schema.json` |
| Grouped-update planner | Preview | `schemas/deps/grouped-update-and-rollback-review.schema.json` |
| Rollback/checkpoint strip | Stable | `schemas/ui/m5-bundle-rollback-remove-primitive.schema.json` |

## State model

- **Maturity class** — stable / beta / preview / experimental / unavailable /
  held.
- **Registry-degradation narrowing vocabulary** — resolved_exact /
  manifest_range_only / mirror_backed / offline_snapshot_only /
  auth_required_unsatisfied / unknown_or_stale.
- **Downgrade triggers** — proof_stale, policy_blocked, registry_unreachable,
  auth_required, mirror_backed_only, offline_snapshot_only, lockfile_divergent,
  script_or_native_build_risk, broad_lockfile_regeneration, rollback_unavailable,
  scope_expansion_unqualified, upstream_dependency_narrowed.
- **Rollback posture** — read_only_no_mutation / staged_review_no_write /
  write_back_checkpointed / regenerate_only_no_manual_edit /
  compensating_only_no_clean_revert / not_applicable.
- **Consumer surfaces** — package_workspace, dependency_explorer,
  install_update_review, registry_auth_workspace, rollback_recovery,
  browser_companion, cli_headless, support_export, help_about.

## Design invariants

1. Generic "manage package" or "one-click update" language never conceals
   manifest scope, registry source, auth posture, script/native-build risk, or
   broad lockfile regeneration.
2. Mirror/offline continuity and rollback/checkpoint identity are explicit
   **before** any write.
3. Every component projects the same truth across desktop, CLI/headless, and
   support export; a downgrade narrows the claim rather than hiding the
   component.
4. Raw manifest/lockfile bodies, credentials, private URLs, and live registry
   responses stay outside the export boundary.
