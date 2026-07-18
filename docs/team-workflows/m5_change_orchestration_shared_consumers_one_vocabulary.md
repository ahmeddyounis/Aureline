# M5 Change-Orchestration Shared Consumers: One Vocabulary Across Surfaces

This lane is the B154 consumer-adoption capstone. It binds the six governed change-orchestration objects
frozen by the [`m5_change_object_patch_stack_and_landing_matrix`](../git/m5-change-orchestration-ops.md) — the
**change object**, the **patch stack / queue**, the **stack-edit / review sheet**, the **landing-candidate
sheet**, the **portable shelf / bundle**, and the **worktree cleanup preview** — to the concrete consumers that
render them, and proves, by fixtures rather than screenshots, that the same seeded change subject carries one
identical vocabulary wherever Aureline selects a change, orders a stack, reviews a stack edit, packages a
landing candidate, exports a shelf, or previews a worktree cleanup.

- Rust module: `aureline_ui::m5_change_orchestration_shared_consumers_one_vocabulary_across_surfaces`
- Boundary schema: [`schemas/teamwork/m5-change-orchestration-shared-consumers.schema.json`](../../schemas/teamwork/m5-change-orchestration-shared-consumers.schema.json)
- Proof bundle: `artifacts/release/m5-change-orchestration-shared-consumers-proof/` (`support_export.json`, `matrix.csv`, `summary.md`)
- Fixtures: `fixtures/teamwork/m5-change-orchestration-shared-consumers/` (`compact_remote_narrowed.json`, `exported_redaction_narrowed.json`)
- Emitter: `cargo run -p aureline-ui --example dump_m5_change_orchestration_shared_consumers -- <support-export|report|csv|fixture-compact-remote-narrowed|fixture-exported-redaction-narrowed|validate>`

## Consumers

Nine shared consumer surfaces adopt the change-orchestration vocabulary: change-object detail, the
patch-stack-queue, the stack-edit / review sheet, review detail, the provider merge queue, the portable shelf,
the worktree cleanup preview, the support / export packet, and the help / docs surface. Each of the six objects
is adopted by at least two distinct consumers, so an object is proven to be shared change-orchestration
infrastructure rather than a one-surface fork that invents its own stack / landing labels.

## One vocabulary, no drift

For a given seeded change subject, every consumer surface must present identical
`ChangeOrchestrationSharedStateFacetValues`: the same change-orchestration-role word, object word,
registry-reference word, landing-state word, surface-context word, and membership-source word. The
change-orchestration-role word must be a token from the frozen `M5ChangeOrchestrationRole` vocabulary
(`selected_change_object_disclosure`, `worktree_binding_disclosure`, `stack_membership_disclosure`,
`landing_state_disclosure`, `validation_freshness_disclosure`, `rollback_export_fallback_disclosure`,
`cleanup_safety_disclosure`), so no surface invents an alternate label for the selected change object, the
worktree binding, stack membership, or landing state.

A role that carries selected-change-object, worktree-binding, stack-membership, or landing-state meaning is a
**gate role**: it must pair its surface presentation with a real
`membership_source_disclosed_and_worktree_binding_bound` continuity and never collapse to a masquerade sentinel
(`membership_inferred_from_branch_name_alone`, `ambient_branch_state_shown_as_reviewed_landing_candidate`,
`stale_member_shown_as_queue_eligible`, `cross_worktree_write_shown_as_selected_change`).

## Narrowing is disclosed

A surface may narrow *how much* it renders across the desktop-full, compact, remote-projected, and
exported-redacted representations, but never reword the vocabulary. Every narrowed representation carries an
explicit `ChangeOrchestrationSharedNarrowNote` naming the reason, the preserved vocabulary, and the next action;
remote and exported forms additionally name their remote-source and export-safe-detail boundaries.

## Map back to one object

Support / export consumers point at the canonical per-domain schema and the frozen matrix by id, so an exported
packet — and every copy / export / open-in-provider action — maps back to one shared contract object rather
than diverging into a surface-local payload or collapsing stable membership / landing labels to generic prose.

## Guardrails

Each binding re-asserts the batch's five hard invariants (all MUST be `false`): it never treats ambient branch
state as a reviewed landing candidate, never mutates another worktree without an explicit selected change object
and worktree binding, never infers stack membership from branch names alone, never silently reorders, collapses,
or retargets stack members, and never deletes orphaned worktrees or stale members without previewing running
tasks, open editors, uncommitted changes, recovery checkpoints, and export-safe evidence.

## Acceptance criteria mapping

1. **Shared consumers use the same labels, states, and recovery language for stack / landing / shelf truth** —
   enforced by the per-subject facet identity and the `change_orchestration_vocabulary_drift_across_surfaces`
   violation over nine shared consumer surfaces, with the frozen-role-token gate
   (`change_orchestration_role_word_outside_vocabulary`) keeping the role vocabulary controlled.
2. **Cross-worktree mutation attempts are blocked or rerouted through explicit selected-change review rather
   than executing implicitly** — enforced by the five guardrail row-invariants (led by
   `mutates_another_worktree_without_a_selected_change_object_and_worktree_binding` and
   `treats_ambient_branch_state_as_a_reviewed_landing_candidate`), the gate-role
   `membership_source_missing_for_gate_role` check over the shared membership-source words, and
   `points_at_canonical_contracts` / `support_export_reference_missing` so exported packets map back to one
   contract object.
