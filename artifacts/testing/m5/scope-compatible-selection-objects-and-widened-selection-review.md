# M5 Scope-Compatible Selection Objects

- Packet: `portable-selection:stable:0001`
- Label: `M5 Scope-Compatible Selection Objects`
- Selections: 5 across 4 / 4 channels
- Intents present: 5 / 5
- Assessments: 4 (2 opened review)

## Selections

- **selection:ui:checkout-rerun** (ui / rerun_all): policy `reresolve_within_snapshot`
  - Rerun the checkout suite selected from the test tree
  - snapshot `snapshot:framework-pack:checkout` digest `digest:checkout:v1` (4 targets)
    - `framework:case:add-item` [concrete_case] fingerprint=`fingerprint:framework:case:add-item` identity=`stable`
    - `framework:template:totals` [parameterized_template] fingerprint=`fingerprint:framework:template:totals` identity=`stable`
    - `framework:invocation:totals:usd` [concrete_invocation] fingerprint=`fingerprint:framework:invocation:totals:usd` identity=`stable`
    - `framework:invocation:totals:eur` [concrete_invocation] fingerprint=`fingerprint:framework:invocation:totals:eur` identity=`stable`
- **selection:cli:rerun-failed** (cli / rerun_failed): policy `pinned_exact`
  - CLI rerun of only the failed invocations from the last session
  - snapshot `snapshot:framework-pack:checkout` digest `digest:checkout:v1` (1 targets)
    - `framework:invocation:totals:eur` [concrete_invocation] fingerprint=`fingerprint:framework:invocation:totals:eur` identity=`stable`
- **selection:ai:changed-since** (ai / changed_since): policy `allow_widen_with_review`
  - AI test plan proposing the tests changed since the base ref
  - snapshot `snapshot:test-tree:aggregate` digest `digest:aggregate:v3` (2 targets)
    - `tree:case:login` [concrete_case] fingerprint=`fingerprint:tree:case:login` identity=`stable`
    - `tree:notebook:analysis` [notebook_linked_test] fingerprint=`fingerprint:tree:notebook:analysis` identity=`stable`
- **selection:support:snapshot-scoped** (support / snapshot_scoped): policy `reresolve_within_snapshot`
  - Support reconstruction of a snapshot-scoped triage selection
  - snapshot `snapshot:test-tree:aggregate` digest `digest:aggregate:v3` (1 targets)
    - `tree:case:login` [concrete_case] fingerprint=`fingerprint:tree:case:login` identity=`stable`
- **selection:support:imported-ci** (support / explicit_items): policy `frozen_imported_read_only`
  - Imported CI overlay selection retained read-only for triage
  - snapshot `snapshot:imported-ci:smoke` digest `digest:imported:v1` (1 targets)
    - `ci:case:smoke` [concrete_case] fingerprint=`fingerprint:ci:case:smoke` identity=`imported_read_only`

## Assessments

- **assessment:cli:compatible** → `selection:cli:rerun-failed`: class `compatible`, review `not_required` (dispatch allowed)
- **assessment:ai:widened** → `selection:ai:changed-since`: class `widened_needs_review`, review `pending` (dispatch blocked)
  - would add: tree:case:logout
- **assessment:ui:drifted** → `selection:ui:checkout-rerun`: class `snapshot_drifted`, review `pending` (dispatch blocked)
- **assessment:imported:blocked** → `selection:support:imported-ci`: class `imported_not_rerunnable`, review `blocked` (dispatch blocked)
