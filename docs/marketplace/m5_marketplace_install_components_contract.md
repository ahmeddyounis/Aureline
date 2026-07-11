# M5 marketplace-install component matrix contract

This document is the human-readable companion to the frozen **M5 marketplace-result-row,
marketplace-detail-fact-grid, compatibility-label-strip, permission-manifest-summary,
activation-budget-band, install/update/disable/rollback review-sheet, publisher-continuity-row, and
installed-state-diagnostics-card component matrix**.

The authoritative source of truth is the Rust validator and seed builder in
`crates/aureline-shell/src/freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix/`.
The checked-in support export, matrix CSV, design report, and narrowed fixtures are minted from that
seed builder by the `dump_m5_marketplace_install_component_matrix` example; the schemas under
`schemas/ui/` document the shape and the JSON Schemas are meta-valid Draft 2020-12.

## What this freezes

Every extension-marketplace or registry surface M5 claims that still ships its own listing, detail,
compatibility, permission, budget, install, publisher, or diagnostics chrome is named once here and
bound to one shared vocabulary, so compatibility, permission, activation, publisher, and rollback
truth stop drifting across claimed M5 ecosystem surfaces.

### Governed component families

| Component family | Canonical schema |
| --- | --- |
| `marketplace_result_row` | `schemas/ui/m5-marketplace-result-row.schema.json` |
| `marketplace_detail_fact_grid` | `schemas/ui/m5-marketplace-detail-fact-grid.schema.json` |
| `compatibility_label_strip` | `schemas/ui/m5-compatibility-label-strip.schema.json` |
| `permission_manifest_summary` | `schemas/ui/m5-permission-manifest-summary.schema.json` |
| `activation_budget_band` | `schemas/ui/m5-activation-budget-band.schema.json` |
| `install_update_disable_rollback_review_sheet` | `schemas/ui/m5-install-update-disable-rollback-review-sheet.schema.json` |
| `publisher_continuity_row` | `schemas/ui/m5-publisher-continuity-row.schema.json` |
| `installed_state_diagnostics_card` | `schemas/ui/m5-installed-state-diagnostics-card.schema.json` |

## The one controlled disposition vocabulary

Every consumer binds to one marketplace/install-disposition vocabulary and no surface invents a
parallel word for any of these:

`public`, `mirrored`, `enterprise`, `side_load`, `verified`, `transferred`, `deprecated`, `limited`,
`incompatible`, `over_budget`, `throttled`, `quarantined`, `disable_scope`, `rollback_compatibility`.

## Family-specific controlled vocabularies

Each family declares only the vocabularies applicable to it:

- **Registry source class** — `public_registry`, `mirrored_registry`, `enterprise_registry`,
  `side_loaded`, `verified_partner`, `source_unknown` (result row, detail grid, publisher row).
- **Compatibility state** — `compatible`, `compatible_with_warnings`, `incompatible`,
  `degraded_host`, `unsupported_runtime`, `compatibility_unknown` (result row, detail grid,
  compatibility strip, diagnostics card).
- **Host / runtime model** — `in_process`, `sandboxed`, `remote_host`, `web_worker`, `native_host`,
  `host_unknown` (detail grid, compatibility strip).
- **Permission posture** — `minimal`, `standard`, `elevated`, `widened_transitive`,
  `policy_restricted`, `posture_unknown` (detail grid, permission summary).
- **Activation-budget band** — `within_budget`, `near_budget`, `over_budget`, `throttled`,
  `suspended_over_budget`, `budget_unknown` (detail grid, activation band, diagnostics card).
- **Publisher continuity** — `continuous`, `transferred`, `deprecated`, `abandoned`,
  `verified_publisher`, `continuity_unknown` (result row, detail grid, publisher row).
- **Disable scope** — `disable_workspace`, `disable_global`, `disable_profile`, `uninstall_full`,
  `keep_data_disable`, `scope_unknown` (review sheet).
- **Rollback compatibility** — `rollback_exact`, `rollback_compatible`, `rollback_incompatible`,
  `rollback_data_loss`, `no_prior_version`, `rollback_unknown` (review sheet).
- **Quarantine state** — `not_quarantined`, `quarantined_active`, `quarantined_history`,
  `released_from_quarantine`, `quarantine_pending`, `quarantine_unknown` (diagnostics card).

## Hard invariants

Every component row asserts (all `false`):

1. `hides_permission_widening_or_activation_cost`
2. `hides_publisher_transfer_disable_scope_or_rollback_incompatibility`
3. `collapses_registry_source_class_across_public_mirrored_enterprise`
4. `presents_incompatible_or_over_budget_as_ready`

## Non-visual / CLI / export requirements

Every component declares a non-visual accessibility route set (keyboard-focusable,
screen-reader-announced, non-hover-reachable, pointer-optional, high-contrast-safe,
support-exportable) so none of these components becomes marketplace-only chrome, and every component
must be present in the support / export packet.

## Acceptance-criteria mapping

- **Shared matrix** — design, schema, QA, and release owners share this one matrix for extension
  marketplace and install-review primitives.
- **One canonical contract** — every claimed M5 ecosystem consumer points at one canonical
  per-component schema (or the combined matrix schema) instead of rewording listing/detail/install
  state locally.
- **Agreed baseline** — future implementation rows inherit this field/state baseline with no open
  ambiguity about compatibility, permission, or budget labeling.
