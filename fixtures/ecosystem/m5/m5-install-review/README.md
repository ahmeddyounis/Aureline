# Fixtures: M5 install/update review sheets

This directory contains fixture metadata for the `m5_install_review` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-install-review.json`

## Coverage

- Eight review sheets cover the framework-pack, docs-pack, local-model-pack,
  recipe-pack, template, and bridge-backed package families, so one reviewed change
  model is proven across the marketed M5 artifact families that can widen
  permissions, move runtime origin, restart hosts, or affect open work.
- Each sheet carries a `governance_family_ref` that resolves to its row in
  `artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json`.
- Change kind covers `install` (a fresh template install with no current revision)
  and `update`; scope covers `workspace`, `profile`, and `global`; commit disposition
  covers `one_click_allowed`, `unified_review_required`, and `blocked`.
- Every review trigger is exercised by at least one sheet: `permissions_widened`,
  `publisher_discontinuity`, `runtime_origin_changed`, `host_class_changed`,
  `compatibility_floor_regressed`, `compatibility_unsupported`,
  `restart_or_reattach_required`, `open_work_impacted`, and
  `rollback_not_established`. Capability deltas exercise both `direct` and
  `transitive` origins and `required` requirements, including a transitive required
  widening.
- The guardrail is proven: every sheet that widens permissions, transfers its
  publisher, or changes its runtime origin recomputes to at least
  `unified_review_required` rather than `one_click_allowed`, and the
  unsupported-on-target sheet recomputes to `blocked` with its `commit` action
  disabled.
- Scope distinctness is proven: every action on a sheet carries that sheet's scope,
  and rollback checkpoint creation, current-package fallback, scope-specific disable,
  and retry are offered through the same reviewed model.
- Each sheet's `compatibility_floor_change`, `review_triggers`, and
  `commit_disposition` equal the recomputation from its facts; the clean
  framework-pack and docs-pack updates and the fresh template install recompute to
  `one_click_allowed`.
