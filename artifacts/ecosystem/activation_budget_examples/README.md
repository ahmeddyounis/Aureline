# Activation-budget example rows

Reviewer-facing activation-budget rows the install-review sheet,
permission inspector, support-export, and release-evidence packets
quote when a managed-workspace binding is in scope. Each file carries
one `activation_budget_example_record` binding one
`activation_budget_class` token to the frozen
`activation_budget_summary_record` slice accounting, the managed-
workspace lifecycle state, the host-boundary cue, and the rollback /
quarantine posture interactions the install-review packet requires.

These examples do not redefine the summary shape. They cite the
vocabulary frozen in:

- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](../../../docs/runtime/target_discovery_and_install_review_taxonomy.md)
  — `activation_budget_summary_record` example shape.
- [`/artifacts/runtime/managed_workspace_lifecycle.yaml`](../../runtime/managed_workspace_lifecycle.yaml)
  — `activation_budget_slice_vocabulary` and lifecycle-state matrix.
- [`/docs/verification/install_review_packet.md`](../../../docs/verification/install_review_packet.md)
  — `activation_budget_class` vocabulary and projection rules.

Required fields across every example:

- `activation_budget_class`
- `managed_workspace_instance_ref`
- `managed_workspace_lifecycle_state`
- `managed_workspace_reviewer_label`
- `budget_window`
- `budget_slices`
- `degradation_markers`
- `threshold_breach_markers`
- `host_boundary_cue`
- `freshness_class`
- `redaction_class`
- `rollback_posture_interaction`
- `quarantine_posture_interaction`
- `install_review_disposition_projection`

Examples:

- [`not_applicable_local_host.yaml`](./not_applicable_local_host.yaml)
  — baseline for an install whose subject does not bind a managed
  workspace; every budget-slice field is null.
- [`healthy_under_budget_ready.yaml`](./healthy_under_budget_ready.yaml)
  — ready instance, every slice strictly under budget.
- [`approaching_slice_ceiling_warming.yaml`](./approaching_slice_ceiling_warming.yaml)
  — warming slice crossed the warning threshold; review admits
  approval under disclosure.
- [`slice_exceeded_with_degradation_marker_warming.yaml`](./slice_exceeded_with_degradation_marker_warming.yaml)
  — warming slice crossed the budget; control plane emitted a
  degradation marker; review defers to admin confirmation.
- [`multi_slice_exhausted_admin_confirmation.yaml`](./multi_slice_exhausted_admin_confirmation.yaml)
  — ready and idle_suspend slices both exhausted; admin
  intervention required.
- [`budget_frozen_on_quarantine.yaml`](./budget_frozen_on_quarantine.yaml)
  — instance quarantined; slices frozen at last-observed values;
  install denied.
- [`budget_frozen_on_retiring_or_retired.yaml`](./budget_frozen_on_retiring_or_retired.yaml)
  — instance retiring with drain window; slices frozen; migration
  hint required.
- [`budget_unknown_pending_refresh.yaml`](./budget_unknown_pending_refresh.yaml)
  — control plane unreachable; freshness below authoritative_live;
  review defers commit.
