# AI background branch-agent worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/background_branch_agent_lifecycle.md`](../../../docs/ai/background_branch_agent_lifecycle.md)
and the schemas at
[`/schemas/ai/branch_agent_session.schema.json`](../../../schemas/ai/branch_agent_session.schema.json)
and
[`/schemas/ai/branch_agent_review_packet.schema.json`](../../../schemas/ai/branch_agent_review_packet.schema.json).

Every file is a multi-document YAML stream. The first document is a
`__fixture__` prelude summarising the scenario, the contract sections
it exercises, and the record kinds it produces. The remaining
documents are individual `branch_agent_session_record`,
`branch_agent_checkpoint_record`,
`branch_agent_validation_plan_record`,
`branch_agent_session_audit_event_record`,
`branch_agent_review_packet_record`,
`branch_agent_landing_constraint_record`, and
`branch_agent_review_packet_audit_event_record` instances that
conform to the two schemas.

No fixture embeds raw URLs, raw filesystem paths, raw worktree paths,
raw branch names, raw remote workspace URIs, raw provider payloads,
raw API keys, raw OAuth tokens, raw mTLS material, raw cost amounts
in any specific currency, raw token counts, raw provider unit prices,
raw user identifiers, or raw billing-account ids. Every such field is
an opaque ref, a structured class label, or a coarse bucket.

## Cases

| Scenario file                                             | Axis exercised                                                                                  | Covered locus / state                                                                                                |
|-----------------------------------------------------------|-------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------|
| `queued_agent_dispatch.yaml`                              | Queued-pre-dispatch session + first checkpoint + validation-plan rows committed                  | `dispatched_from_composer_turn`; `isolated_side_worktree`; `proposes_diff_only_no_apply`; three validation rows       |
| `running_agent_isolated_worktree.yaml`                    | Running session + mid-run / dry-run checkpoints + mutation-surface promotion                     | `dispatched_from_composer_turn`; `isolated_side_worktree`; mutation surface advances to `mutates_isolated_worktree_only` |
| `paused_checkpoint_review.yaml`                           | Paused-for-checkpoint-review session + pre-publish review checkpoint + advisory findings         | `paused_for_checkpoint_review`; pre-publish checkpoint paused pending user; must-run rows passed                      |
| `policy_blocked_dispatch.yaml`                            | Policy-blocked dispatch refusal + typed denial event                                             | `policy_blocked_pre_dispatch`; air-gapped deployment profile refuses `managed_remote_workspace`; typed denial         |
| `completed_review_packet_no_self_land.yaml`               | Completed session + review packet + four landing constraints incl. no-self-merge / no-protected-push | `session_completed_review_only`; review packet ready; cannot self-land; review pipeline required                      |

Every fixture declares its canonical values via the
`exercised_classes` block so later coverage audits can confirm each
vocabulary member is hit at least once.

## Acceptance-criteria coverage

The seeded cases cover every acceptance criterion named in the task:

- **Any branch-agent surface can show where the work ran, what it
  could touch, and how it must land using one shared lifecycle
  vocabulary.** Every fixture renders
  `target_execution_locus_class`, `mutation_surface_class`,
  `secret_scope_class`, `landing_posture_class`, and (on the
  completed case) the typed `landing_constraint_class` rows.
- **Checkpoint, cancellation, and final review packets preserve
  lineage back to the originating composer/request and any
  route/spend receipts.** Every fixture cites
  `originating_composer_session_ref`,
  `originating_turn_draft_ref`,
  `originating_request_workspace_ref` (typed),
  `originating_assembly_id_ref`, and
  `originating_branch_agent_dispatch_ref`. Running, paused, and
  completed cases also cite `route_rollup_ref`, `spend_rollup_ref`,
  per-hop `route_receipt_refs`, and per-hop `spend_receipt_refs`.
- **No fixture permits silent mutation of the active worktree or
  silent escalation from assist to isolated automation.** Every
  fixture's locus / mutation surface combination satisfies the
  schema's allOf gates pairing `target_execution_locus_class` to
  `mutation_surface_class`. Mutation-surface promotion
  (`proposes_diff_only_no_apply` →
  `mutates_isolated_worktree_only`) on the running and paused cases
  is explicit and rides a `pre_mutation_dry_run_checkpoint`.
- **Worked examples exist for queued agent, running agent, paused
  checkpoint review, policy-blocked dispatch, and completed agent
  that produces a review packet but cannot self-land.** The five
  cases above cover each axis explicitly.
