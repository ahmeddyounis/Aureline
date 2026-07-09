# Implement script-risk notices, grouped-update planners, and rollback/checkpoint strips

Status: Implemented (M05-976, batch B115)

This contract narrows the last three components frozen in
[`m5-package-management-component-matrix`](freeze_the_m5_package_management_component_matrix.md)
(M05-972) — the `script_risk_notice`, the `grouped_update_planner`, and the
`rollback_checkpoint_strip` — into one implemented, export-safe packet with three
co-equal control vectors. Together they make package mutation side effects and
recovery posture explicit **before** a grouped or risky change runs, and keep
recovery posture visible **after** it runs instead of only when something breaks.

- Boundary schema: [`schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json`](../../../schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json)
- Producer: `aureline_deps::current_script_risk_grouped_update_rollback_export`
- Release proof: [`artifacts/release/m5-script-risk-grouped-update-rollback-proof/`](../../../artifacts/release/m5-script-risk-grouped-update-rollback-proof/)
- Protected fixtures: [`fixtures/ui/m5-script-risk-grouped-update-rollback-controls/`](../../../fixtures/ui/m5-script-risk-grouped-update-rollback-controls/)

## Script-risk notices

Every `ScriptRiskNotice` reuses the frozen `M5PackageComponent` tag (gated to
`script_risk_notice`) and answers, from the notice alone:

- **Execution source** (`execution_source`: `install_lifecycle_script` /
  `native_build_step` / `postinstall_binary_fetch` / `no_scripts_declared`) — a
  native build is never flattened into a generic warning, and a no-scripts
  package can present its reassuring truth. Any executing source requires an
  `execution_source_note` (`script_execution_source_note_missing`).
- **Support and client notes** (`support_note`, `client_note`, both required).
- **Actions** — a policy-block action when policy blocks
  (`script_policy_block_action_missing`), or a review action when review is
  warranted (`script_review_action_missing`).

The risk class is *derived*, never asserted, by
`resolve_script_risk(execution_source, policy_blocks, source_trusted)`: nothing
runs ⇒ `no_execution`; policy blocks the scripts ⇒ `policy_blocked`; trusted code
runs ⇒ `review_recommended`; untrusted code runs ⇒ `unknown_untrusted`. The
notice's `risk_class` must match the derived value
(`script_risk_class_misrepresented`), so a package running an untrusted hook can
never present as benign. The packet requires all four risk classes to be
represented (`script_risk_coverage_missing`). A notice never mutates, so its
rollback posture is constrained to read-only or staged-review
(`script_risk_notice_rollback_posture_inconsistent`).

## Grouped-update planners

Every `GroupedUpdatePlanner` reuses the frozen `M5PackageComponent` tag (gated to
`grouped_update_planner`) and answers, from the planner alone:

- **Update reason** (`update_reason`: `direct_request` / `security_advisory` /
  `routine_refresh` / `dependency_convergence`) with a required `reason_note`.
- **Grouped packages** (`grouped_packages`, required and non-empty) and
  **transitive churn** (`transitive_churn_count`, with a required
  `transitive_churn_note` when there is any churn).

The plan class is *derived* by
`resolve_update_plan_class(update_reason, grouped_package_count,
transitive_churn_count)`: a convergence reason, a large grouped set, or churn past
the broad threshold ⇒ `broad_convergence`; a security advisory ⇒ `security_patch`;
several packages or churn past the grouped threshold ⇒ `grouped_refresh`;
otherwise `direct_bump`. The planner's `plan_class` must match the derived value
(`plan_class_misrepresented`), so a broad convergence can never read as a single
direct bump — this is the AC's "distinguish direct bumps, security patches,
grouped refreshes, and broad convergence plans before execution". A broad plan
requires a `convergence_note` and a security patch requires a `security_note`. The
packet requires all four plan classes to be demonstrated
(`plan_class_coverage_missing`). A planner previews a plan and never writes, so
its rollback posture is constrained to read-only or staged-review.

## Rollback / checkpoint strips

Every `RollbackCheckpointStrip` reuses the frozen `M5PackageComponent` tag (gated
to `rollback_checkpoint_strip`) and answers, from the strip alone:

- **Checkpoint identity** (`checkpoint_label`, `checkpoint_id`, both required) and
  a **mutation summary** (`mutation_summary`, required).
- **Remove-blocked state** (`remove_blocked_state`: `not_a_remove` / `removable`
  / `remove_blocked_policy_pinned` / `remove_blocked_required_by`) with a required
  `remove_blocked_note` whenever removal is blocked.
- **Recovery visibility** (`recovery_visible_now`, which must be `true` —
  `recovery_posture_not_visible_after_mutation`) so recovery posture stays visible
  after mutation instead of only appearing when something breaks.
- **Actions** — `revert_action_label`, `open_diff_action_label`, and
  `export_patch_action_label`, all required (`rollback_actions_missing`).

The recovery posture is *derived* by
`resolve_recovery_posture(remove_blocked_state, regenerated)`: a blocked removal
⇒ `compensating_only`; a regenerating write ⇒ `revert_with_regeneration`;
otherwise `fully_revertible`. The strip's `recovery_posture_class` must match the
derived value (`recovery_posture_misrepresented`), and its `rollback_posture` must
be the posture that recovery class implies
(`rollback_strip_rollback_posture_inconsistent`), so a remove-blocked revert can
never claim a clean automatic rollback. The packet requires all three recovery
postures and at least one remove-blocked state to be demonstrated
(`recovery_posture_coverage_missing`, `remove_blocked_coverage_missing`).

## Coverage and boundary

Beyond the per-vector coverage rules above, the packet carries the batch
guardrail `no_generic_one_click_update_language` (the trust-review invariant that
no generic one-click update conceals manifest scope, script risk, or broad
lockfile regeneration). Raw manifest bodies, raw lockfile bodies, raw script
bodies, registry credentials, private registry URLs, and live registry responses
never cross this boundary; the export is scanned for forbidden material
(`raw_boundary_material_in_export`).

## Regenerating artifacts

```
GEN_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  implement_script_risk_notices_grouped_update_planners_and_rollback_checkpoint_strips::tests::generate_artifacts
```
