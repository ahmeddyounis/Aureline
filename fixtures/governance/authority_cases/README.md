# Runtime authority-case examples

Worked fixtures for the runtime authority-ticket, issuer-rule,
and external-effect lineage vocabulary defined in
[`/docs/governance/runtime_authority_contract.md`](../../../docs/governance/runtime_authority_contract.md).

Schemas consumed:

- [`/schemas/governance/authority_ticket.schema.json`](../../../schemas/governance/authority_ticket.schema.json)
  — `authority_ticket_record`,
  `authority_ticket_audit_event_record`,
  `authority_ticket_invalidation_record`.
- [`/schemas/governance/external_effect_lineage.schema.json`](../../../schemas/governance/external_effect_lineage.schema.json)
  — `external_effect_lineage_record`,
  `external_effect_lineage_outcome_record`.

Each fixture carries a `__fixture__` section summarizing the
scenario, the axes it exercises, and the document sections it
illustrates. The fixture body conforms to the schema it references
so tooling can validate every example as an integration check.

Fixtures:

- [`local_workspace_mutation_destructive.json`](./local_workspace_mutation_destructive.json)
  — `authority_class = local_workspace_mutation`,
  `side_effect_class = local_destructive_edit`, with a required
  rollback checkpoint and preview fingerprint for a project-wide
  replace.
- [`external_provider_publish.json`](./external_provider_publish.json)
  — `authority_class = external_provider_mutation`,
  `side_effect_class = external_irreversible_publish`, admitting
  an inner provider approval ticket (`admitted_inner_tickets`
  non-empty) and forbidding `rememberable_scope`.
- [`credential_projection_build.json`](./credential_projection_build.json)
  — `authority_class = credential_projection`,
  `side_effect_class = credential_handle_projection` with
  `projection_mode_ref` and a bounded-reuse counter for a build
  agent.
- [`debug_attach_privileged.json`](./debug_attach_privileged.json)
  — `authority_class = debug_or_privileged_inspection`,
  `side_effect_class = privileged_inspection_attach` with the
  `attaches_privileged_debugger` high-risk flag.
- [`policy_change_trust_root_rotation.json`](./policy_change_trust_root_rotation.json)
  — `authority_class = policy_or_admin_change`,
  `side_effect_class = policy_or_trust_mutation` issued from the
  supervisor control path for a trust-root rotation.
- [`automation_lineage_admission_recipe.json`](./automation_lineage_admission_recipe.json)
  — `authority_class = automation_lineage_admission`,
  `side_effect_class = automation_admission_only`, admitting a
  recipe plan so later derived tickets inherit the approved
  lineage.
- [`rememberable_decision_local_edit.json`](./rememberable_decision_local_edit.json)
  — `rememberable_scope` active: a narrow reusable rule plus a
  renewable short-lived ticket for a frequent local reversible
  edit, not an unlimited bearer credential.
- [`invalidation_policy_epoch_drift.json`](./invalidation_policy_epoch_drift.json)
  — `authority_ticket_invalidation_record` with
  `invalidation_reason = policy_epoch_drift`, showing the
  pre/post drift fingerprint diff and the fresh superseding
  ticket the runtime minted in response.
- [`external_effect_lineage_publish_succeeded.json`](./external_effect_lineage_publish_succeeded.json)
  — `external_effect_lineage_record` for an external publish plus
  a `succeeded` outcome, carrying the preview fingerprint, the
  approval-refs packet, and every consumer surface the support
  bundle / audit stream / mutation journal reads.
- [`external_effect_lineage_failed_after_side_effect.json`](./external_effect_lineage_failed_after_side_effect.json)
  — `external_effect_lineage_outcome_record` with
  `outcome_class = failed_after_side_effect` and a typed
  `reconciliation_state = pending_admin_action`, demonstrating
  how post-incident reconciliation hooks read lineage directly.
