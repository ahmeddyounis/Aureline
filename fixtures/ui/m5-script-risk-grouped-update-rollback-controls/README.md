# M5 script-risk, grouped-update, and rollback control fixtures

Protected fixtures for the `script_risk_notice`, `grouped_update_planner`, and
`rollback_checkpoint_strip` components implemented in
`aureline_deps::implement_script_risk_notices_grouped_update_planners_and_rollback_checkpoint_strips`.

Each fixture is an export-safe `ScriptRiskGroupedUpdateRollbackControlsPacket`
that validates against
[`schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json`](../../../schemas/ui/m5-script-risk-grouped-update-rollback-controls.schema.json)
and passes `ScriptRiskGroupedUpdateRollbackControlsPacket::validate`.

- `untrusted_script_broad_convergence.json` — an untrusted post-install script
  paired with a broad convergence plan; neither the derived risk class nor the
  derived plan class can read as benign.
- `remove_blocked_recovery.json` — a remove-blocked revert answered from an
  offline snapshot; the recovery posture stays visible and cannot claim a clean
  automatic rollback.

Regenerate with:

```
GEN_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  implement_script_risk_notices_grouped_update_planners_and_rollback_checkpoint_strips::tests::generate_artifacts
```
