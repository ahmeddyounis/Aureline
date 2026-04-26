# Repair-transaction seed cases

These fixtures are the seed cases the repair-transaction packet at
[`docs/support/repair_transaction_contract.md`](../../../docs/support/repair_transaction_contract.md)
defines. Each file validates against the `repair_seed_case_record`
alternate in
[`schemas/support/repair_transaction.schema.json`](../../../schemas/support/repair_transaction.schema.json),
which is itself the boundary schema the packet shares with the live
`repair_transaction_record` shape.

Every case:

- names one stable `repair_transaction_id` (for example
  `repair_transaction:disposable_state_rebuild.cache_index_repair`);
- binds one `repair_class_family` from the closed milestone vocabulary
  (`disposable_state_rebuild`, `extension_isolation`,
  `extension_rollback_reinstall`, `execution_context_reresolve`,
  `remote_runtime_repair`, `policy_entitlement_refresh`,
  `guided_export_escalation`, `observe_only_no_repair`);
- declares one `suggested_repair_class` from the project-doctor
  vocabulary (extended with the three transaction-level tokens
  `rollback_or_reinstall_extension`, `rollback_remote_runtime`, and
  `refresh_policy_entitlement`);
- declares one `transaction_reversal_class` drawn from the closed
  five-token set (`exact`, `compensating`, `regenerate`, `manual`,
  `audit_only`);
- declares one `apply_mode_class` (`dry_run_preview_only`,
  `apply_with_checkpoint`, `apply_with_rollback_on_failure`,
  `apply_observe_only_no_write`, `apply_refused_escalation_only`);
- lists at least one `doctor.finding.*` initiating finding code from
  [`fixtures/support/scenario_matrix.yaml`](../scenario_matrix.yaml);
- lists `impacted_state_classes`, `preserved_state_classes`,
  `lost_capability_classes`, and a `runtime_requirements` block
  declaring the online/offline gate, the trust/policy gate, and the
  user/admin consent gates apply MUST satisfy;
- lists `forbidden_action_assertions` covering at least
  `widen_workspace_trust`, `publish_route`, `run_repo_hook_silently`,
  `mutate_user_authored_files`, and `read_or_rotate_credentials`,
  plus the family-specific assertions named in the contract doc;
- declares `checkpoint_ref_required` (true for `apply_with_checkpoint`
  and `apply_with_rollback_on_failure`; false otherwise);
- declares an `idempotency_key`, an `escalation_route` block (with
  `escalation_required_when`, the default handoff-packet template
  ref, and the default redaction choice), and stable `linkage_refs`
  onto the matched scenario row, the recovery-action id, the support-
  bundle case, the escalation case, the Doctor finding code, the
  checkpoint, the preview record, and the outcome record (or declares
  those refs `null` with an explicit reason in `notes`); and
- declares five reviewer-facing explanation strings (preserved work,
  change summary, capability disablement, escalation, next step) so
  Support and export flows render repair advice verbatim without
  minting prose.

## Case list

- `cache_index_repair.yaml` —
  `repair_transaction:disposable_state_rebuild.cache_index_repair`
- `extension_quarantine.yaml` —
  `repair_transaction:extension_isolation.suspect_host_quarantine`
- `toolchain_reresolve.yaml` —
  `repair_transaction:execution_context_reresolve.toolchain_required_component`
- `remote_agent_rollback.yaml` —
  `repair_transaction:remote_runtime_repair.remote_agent_rollback`
- `policy_refresh.yaml` —
  `repair_transaction:policy_entitlement_refresh.trust_approval_reacquire`
- `escalation_only_packet.yaml` —
  `repair_transaction:guided_export_escalation.no_local_repair_available`

Every case cites its scenario row, recovery action, support-bundle
case, escalation case, and Doctor finding code by stable ref so
support review can pivot in O(1) from one transaction → one preview →
one outcome → one bundle → one escalation packet → one Doctor
finding.
