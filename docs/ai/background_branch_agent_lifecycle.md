# AI background branch-agent lifecycle, dispatch-review, and branch-state contract

This document is the **product-wide contract** for how a longer-running
AI workflow is dispatched into a side branch, side worktree, isolated
ephemeral workspace, or managed remote workspace, how its lifecycle
moves between dispatch, checkpoint, pause, cancel, and final review,
and how the resulting review packet binds back to the originating
composer session, request workspace, plan, pack, route receipt, and
spend receipt without inventing any new origin-class, scope-class,
trust-state, or charge-state vocabulary.

It freezes one **branch-agent session** shape covering queued, paused,
running, checkpoint-review, completed, cancelled, and policy-blocked
states; one **branch-agent review packet** shape carrying the required
budget, secret-scope, mutation-surface, test-plan, and landing-
constraint fields a reviewer reads before deciding whether the work
may land; one **execution-locus vocabulary** distinguishing
current-worktree assist, isolated side-worktree, side-branch,
ephemeral workspace, and managed workspace so the same "agent" label
cannot hide different risk classes; and one set of const audit-event
ids every dispatch / checkpoint / cancel / publish / land event fires
on.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source wins
and this document MUST be updated in the same change. Where this
document disagrees with a downstream AI / composer / review / patch
/ support / replay surface's mint of its own copy, this document wins
and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ai/branch_agent_session.schema.json`](../../schemas/ai/branch_agent_session.schema.json)
  — boundary schema for the `branch_agent_session_record`,
  `branch_agent_checkpoint_record`,
  `branch_agent_validation_plan_record`, and
  `branch_agent_session_audit_event_record` shapes.
- [`/schemas/ai/branch_agent_review_packet.schema.json`](../../schemas/ai/branch_agent_review_packet.schema.json)
  — boundary schema for the `branch_agent_review_packet_record`,
  `branch_agent_landing_constraint_record`, and
  `branch_agent_review_packet_audit_event_record` shapes.
- [`/fixtures/ai/branch_agent_cases/`](../../fixtures/ai/branch_agent_cases/)
  — worked-example corpus covering a queued agent, a running agent,
  a paused-checkpoint review, a policy-blocked dispatch, and a
  completed agent that produces a review packet but cannot self-land.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/prompt_composer_contract.md`](./prompt_composer_contract.md) —
  the composer-session, turn-draft, request-workspace, prompt-pack,
  tool-pack, and `request_workspace_ref` typed-reference vocabulary.
  Every branch-agent session cites exactly one composer session, one
  turn draft, one request workspace, one plan, and one pack version
  by ref; the contract here does not re-mint any of those classes.
- [`/docs/ai/context_assembly_contract.md`](./context_assembly_contract.md) —
  the assembly id, scope-filter, dispatch-target,
  tainted-usage-constraint, redaction-class, and freshness-class
  vocabulary. Every per-hop assembly the branch-agent run reads is
  one `ai_context_assembly_record`; the dispatch packet the
  composer mints (`ai_branch_agent_dispatch_record`) is the
  cross-schema link this contract resolves through.
- [`/docs/ai/spend_and_route_receipt_contract.md`](./spend_and_route_receipt_contract.md) —
  per-hop `provider_route_receipt_record` and `spend_receipt_record`
  rows plus the `branch_agent_route_rollup_record` and
  `branch_agent_spend_rollup_record` cumulative readouts. Every
  branch-agent session cites one `branch_agent_chain_id`; the
  rollups carry the per-hop receipts under that id; the review
  packet quotes the rollup ids verbatim and never re-mints cost,
  origin, or charge state.
- [`/docs/ai/model_graduation_and_budget_contract.md`](./model_graduation_and_budget_contract.md) —
  rollout state, agent ceiling (single-invocation / bounded-recursion
  / bounded-count / unbounded-admin-only), budget scope, exhaustion
  state, and approval-ticket coupling. The session's
  `agent_ceiling_class` re-exports this vocabulary; a session that
  exceeds its bound denies through the route-rollup denial
  `agent_chain_exceeds_bounded_count` rather than minting its own.
- [`/docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md) —
  provider entry, model entry, execution locus, region posture,
  retention stance, quota family, and tainted-return posture. The
  session's per-hop receipts re-export these classes verbatim.
- [`/docs/ai/evidence_replayability_contract.md`](./evidence_replayability_contract.md) —
  evidence packets quote the branch-agent session id, the per-hop
  assembly ids, and the route / spend receipt ids on every chain so
  a replay packet can reconstruct the run.
- [`/docs/ai/review_assist_publish_contract.md`](./review_assist_publish_contract.md) —
  finding-row / scope-selection / publish-to-review vocabulary. A
  branch-agent review packet that carries findings cites the
  finding records by ref; the publish action that lands the
  packet's patch into a hosted review thread rides the existing
  `publish_to_review_sheet_record` shape.
- [`/docs/vcs/review_workspace_contract.md`](../vcs/review_workspace_contract.md) —
  review-workspace, review-anchor, source-class, and the
  no-self-merge / no-direct-protected-push invariants the review
  pack already encodes. The branch-agent landing constraints cite
  these by class verbatim.
- [`/docs/vcs/git_state_and_worktree_contract.md`](../vcs/git_state_and_worktree_contract.md) —
  worktree row, branch row, and isolation-posture vocabulary the
  side-branch / side-worktree dispatch loci read.
- [`/docs/vcs/review_pack_contract.md`](../vcs/review_pack_contract.md) —
  must-run / advisory / provider-required / local-lint check
  classes the branch-agent validation plan re-exports.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state, deployment-profile class, policy epoch.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md) —
  the broker-owned redaction pass; raw secrets, raw tokens, raw
  credential bodies, and raw URLs never cross this boundary.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md) —
  admin policy MAY narrow which dispatch loci, mutation surfaces,
  agent ceilings, and landing constraints a deployment profile
  admits; policy MAY NOT silently widen any axis.
- [`/docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md) —
  scope-filter / execution-context vocabulary re-exported without
  modification.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  approval-ticket vocabulary for unbounded-admin-only agent ceilings
  and policy-pinned dispatch loci.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  redaction class, freshness class, client scope.

If this document disagrees with those sources, those sources win and
this document plus the schemas are updated in the same change.

This document does **not** ship a branch-agent runtime, a merge-bot,
a worktree-allocator, or a protected-branch policy engine. It freezes
the contract those implementations will read and write. The eventual
branch-agent / orchestration crate's Rust types are the schema of
record; the JSON Schema exports are the cross-tool boundary every
non-owning surface reads.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` — background-AI / branch-agent /
  long-running-AI / no-self-merge requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  branch-agent dispatch architecture, execution-locus separation,
  worktree isolation.
- `.t2/docs/Aureline_Technical_Design_Document.md` — branch-agent
  session lifecycle, checkpoint and validation-plan design,
  landing-constraint design.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — branch-agent
  surface UX, dispatch review card, paused-checkpoint review.

If this contract disagrees with those sources, those sources win.

## Why freeze this now

Without one frozen contract, longer-running AI work is free to
appear under whatever surface mints it: a draft patch in the
current worktree gets the same "agent" label as a side-worktree
spawn that mutates files outside the user's view, a managed-cloud
workspace acquires the same "AI ran" chip as an in-process
completion, and the same review thread can quote a self-merged
agent run without any landing constraint. The consequences are
concrete:

1. *An agent silently mutates the active worktree because the
   composer assumed assist-style execution but the runtime
   escalated to side-worktree.* The user's uncommitted edits are
   overwritten and no audit row records the escalation.
2. *Five chained AI hops chase a refactor across an ephemeral
   workspace, the chain hits its budget cap on hop 3, and the
   chain disappears from history.* Reviewers cannot tell what
   ran, what mutation surface was opened, or whether a checkpoint
   was admitted.
3. *A branch agent finishes a patch and self-merges into the
   user's protected default branch because nothing forced it
   through review.* The no-self-merge invariant the product
   already promises is violated by a surface that was never
   asked to honour it.
4. *A managed-cloud branch-agent run produces a review packet
   that cites no budget band, no secret scope, no test plan,
   and no landing constraint.* Reviewers cannot decide whether
   to land, defer, or cancel the work.

This contract closes that gap with **one branch-agent session
shape, one review packet shape, one execution-locus vocabulary,
one checkpoint-and-validation-plan shape, one set of landing
constraints, and one set of const audit-event ids** every
branch-agent surface reads.

## Who reads this document

- **Branch-agent / orchestration / dispatch authors** minting one
  `branch_agent_session_record` per dispatched run, advancing it
  through queued / running / paused / checkpoint-review / completed
  / cancelled / policy-blocked, and emitting checkpoints,
  validation-plan rows, and a review packet on terminal completion.
- **Composer / review-handoff authors** minting the
  `ai_branch_agent_dispatch_record` (already on the assembly
  schema) before a session opens, and resolving the session id back
  to the dispatch ref on every audit event.
- **Review / support / parity-audit / export authors** quoting a
  session by ref, reading the review packet to explain why a chain
  ran, what mutation surface it opened, and what landing
  constraints govern its merge — without reading implementation
  code or provider logs.
- **Admin / policy / settings surface authors** narrowing which
  dispatch loci, agent ceilings, mutation surfaces, secret scopes,
  and landing constraints a deployment profile admits.
- **Replay / audit / claim-manifest authors** reconstructing the
  chain by walking the per-hop receipts under the session's
  `branch_agent_chain_id` and binding the per-hop assembly ids back
  to the session.

## 1. The branch-agent session

### 1.1 Minimum payload

A `branch_agent_session_record` MUST carry at minimum:

| Field                                  | Purpose                                                                     |
|----------------------------------------|-----------------------------------------------------------------------------|
| `branch_agent_session_id`              | Stable opaque id; the cross-tool readout every other surface cites.          |
| `dispatch_origin_class`                | One of `dispatched_from_composer_turn`, `dispatched_from_slash_command`, `dispatched_from_review_handoff`, `dispatched_from_support_replay`, `dispatched_from_admin_console`, `dispatched_from_extension_provided`, `dispatched_from_mocked_test`, `dispatched_no_origin_disabled`. |
| `originating_composer_session_ref`     | Opaque ref to the `prompt_composer_session_descriptor`. |
| `originating_turn_draft_ref`           | Opaque ref to the `prompt_composer_turn_draft_descriptor` whose dispatch_target_class was `background_branch_agent`. |
| `originating_request_workspace_ref`    | Typed `request_workspace_ref_record` echoing flow / scope / isolation / privacy / retention / lifecycle. |
| `originating_assembly_id_ref`          | Opaque ref to the originating `ai_context_assembly_record`. |
| `originating_branch_agent_dispatch_ref` | Opaque ref to the `ai_branch_agent_dispatch_record` minted on the assembly. |
| `target_execution_locus_class`         | One of `current_worktree_assist`, `isolated_side_worktree`, `side_branch_no_worktree`, `ephemeral_workspace`, `managed_remote_workspace`, `mocked_test_locus`, `disabled_no_locus`. |
| `target_branch_identity_ref`           | Opaque ref to the branch row / branch identity the session targets. Empty string only on `current_worktree_assist` (which mutates the active branch under user view) and `disabled_no_locus`. |
| `target_worktree_identity_ref`         | Opaque ref to the worktree row the session targets. Empty string on `side_branch_no_worktree`, `managed_remote_workspace`, and `disabled_no_locus`. |
| `mutation_surface_class`               | One of `no_mutation_read_only`, `proposes_diff_only_no_apply`, `mutates_isolated_worktree_only`, `mutates_active_worktree_under_user_view`, `mutates_managed_remote_workspace_only`, `mutates_through_review_pipeline_only`, `mutation_unknown_unverified`, `mutation_disabled`. |
| `secret_scope_class`                   | One of `no_secrets_admitted`, `read_only_redacted_handles`, `bounded_user_scoped_handles`, `bounded_admin_scoped_handles`, `enterprise_broker_scoped_handles`, `secret_scope_unknown_unverified`, `secret_scope_policy_blocked`. |
| `agent_ceiling_class`                  | Re-exported from the budget contract.                                       |
| `branch_agent_chain_id`                | Stable opaque id binding every per-hop receipt and assembly to one chain.   |
| `session_state_class`                  | Run-state vocabulary (section 1.2).                                         |
| `pause_reason_class`                   | Required (non-empty) when `session_state_class = paused_for_checkpoint_review` / `paused_by_user` / `paused_by_policy`. |
| `cancel_outcome_class`                 | Required (non-empty) when `session_state_class = cancelled_by_user` / `cancelled_by_policy` / `cancelled_by_budget` / `cancelled_by_route` / `cancelled_by_validation_failure`. |
| `route_rollup_ref` / `spend_rollup_ref` | Refs to the cumulative `branch_agent_route_rollup_record` and `branch_agent_spend_rollup_record`. Empty pre-dispatch; populated as soon as the first hop emits. |
| `validation_plan_refs`                 | Opaque refs to the validation-plan rows the session committed to run.       |
| `checkpoint_refs`                      | Opaque refs to the checkpoints the session emitted (ordered).               |
| `review_packet_ref`                    | Empty until terminal completion; populated with the review packet id on `session_completed_review_only`. |
| `landing_posture_class`                | One of `no_self_land_review_required`, `no_self_land_protected_branch`, `landed_via_review_pipeline_only`, `not_eligible_to_land_read_only`, `not_eligible_to_land_blocked`, `landing_disabled`. |
| `originating_approval_ticket_ref`      | Required (non-empty) when `agent_ceiling_class = agent_invocation_unbounded_admin_only` or when policy pinned a non-default dispatch locus; empty otherwise. |
| `policy_context`                       | Policy epoch, trust state, deployment profile, execution context.           |
| `redaction_class`                      | ADR-0011 redaction posture.                                                 |
| `client_scopes`                        | ADR-0011 client scope set.                                                  |
| `dispatched_at` / `last_state_change_at` | Monotonic timestamps.                                                     |

### 1.2 Session-state vocabulary

The `session_state_class` enum names where in the lifecycle the
session sits:

| Session state                          | Meaning                                                                 |
|----------------------------------------|-------------------------------------------------------------------------|
| `queued_pre_dispatch`                  | Minted from the composer; awaiting dispatch.                             |
| `dispatching`                          | Dispatch is running but the first hop has not yet emitted a receipt.     |
| `running`                              | The chain has at least one in-flight or completed hop.                   |
| `paused_for_checkpoint_review`         | A checkpoint requires reviewer acknowledgement before continuing.        |
| `paused_by_user`                       | The user paused; resumable.                                              |
| `paused_by_policy`                     | A policy / safety check paused; resumable only after re-evaluation.      |
| `cancelled_by_user`                    | The user cancelled; terminal.                                            |
| `cancelled_by_policy`                  | A policy cancelled; terminal.                                            |
| `cancelled_by_budget`                  | A budget cap cancelled; terminal.                                        |
| `cancelled_by_route`                   | All admitted routes blocked / exhausted; terminal.                       |
| `cancelled_by_validation_failure`      | A must-run validation-plan row failed; terminal.                         |
| `session_completed_review_only`        | The chain finished and produced a review packet; landing pending review.  |
| `session_completed_landed_via_review`  | The packet was landed through the review pipeline; terminal.             |
| `policy_blocked_pre_dispatch`          | Dispatch was refused before any hop ran; terminal.                       |
| `disabled_no_dispatch`                 | The session row exists but MUST NOT be cited; terminal.                  |

A session MUST advance through monotonic transitions only:
queued → dispatching → running → (optionally) paused_* / cancelled_*
/ session_completed_review_only → (optionally)
session_completed_landed_via_review. Re-opening a terminal session
is forbidden; a follow-up dispatch mints a fresh session id.

### 1.3 Dispatch-origin vocabulary

The `dispatch_origin_class` enum is the single answer to "where did
this branch-agent run come from":

| Dispatch origin                          | Meaning                                                              |
|------------------------------------------|----------------------------------------------------------------------|
| `dispatched_from_composer_turn`          | A turn-draft with `dispatch_target_class = background_branch_agent`. |
| `dispatched_from_slash_command`          | A `background_branch_agent_slash_command` invocation.                 |
| `dispatched_from_review_handoff`         | A review-handoff request needed an agent.                            |
| `dispatched_from_support_replay`         | A replay packet re-runs a past chain.                                |
| `dispatched_from_admin_console`          | An operator triggered the run from an admin surface.                  |
| `dispatched_from_extension_provided`     | An extension-author flow dispatched into an admitted branch agent.    |
| `dispatched_from_mocked_test`            | Parity / record-replay only.                                          |
| `dispatched_no_origin_disabled`          | The row exists but no dispatch origin admitted; canonical disabled.   |

A session whose `dispatch_origin_class = dispatched_from_review_handoff`
MUST cite `originating_review_handoff_ref`; a session whose
`dispatch_origin_class = dispatched_from_support_replay` MUST cite
`originating_support_replay_ref`; missing the matching ref denies
with `dispatch_origin_missing_supporting_ref`.

### 1.4 Target-execution-locus vocabulary

The `target_execution_locus_class` enum is the canonical answer to
"where did the work actually run":

| Target locus                              | Meaning                                                                 |
|-------------------------------------------|-------------------------------------------------------------------------|
| `current_worktree_assist`                 | The agent operates against the user's active worktree under direct view; mutation surface MUST be `proposes_diff_only_no_apply` or `no_mutation_read_only`. |
| `isolated_side_worktree`                  | A side worktree on a side branch on the user's device; mutation surface MUST be one of the `mutates_isolated_worktree_only` / `proposes_diff_only_no_apply` / `no_mutation_read_only` set. |
| `side_branch_no_worktree`                 | A side branch is created but no worktree is checked out; the agent emits commits / patches against the branch ref only. |
| `ephemeral_workspace`                     | A short-lived workspace local to the device with its own filesystem; closes on terminal session state. |
| `managed_remote_workspace`                | A first-party-managed or enterprise-gateway-brokered remote workspace; mutation surface MUST be one of `mutates_managed_remote_workspace_only` / `mutates_through_review_pipeline_only` / `proposes_diff_only_no_apply` / `no_mutation_read_only`. |
| `mocked_test_locus`                       | Record-replay / parity locus.                                           |
| `disabled_no_locus`                       | Canonical disabled state.                                               |

The five non-mock loci correspond to **five different risk classes**.
The schema's allOf gates pair locus to mutation-surface class so the
same "agent" label cannot hide an escalation. The `mutation_disabled`
and `mutation_unknown_unverified` values are admitted on every
non-mock locus so a pre-dispatch / policy-blocked / not-yet-verified
session row can still record the locus that was requested without
declaring an active mutation surface; every other mutation surface
is gated:

- `current_worktree_assist` ⇒ active mutation surface ∈
  `{no_mutation_read_only, proposes_diff_only_no_apply}`.
- `isolated_side_worktree` ⇒ active mutation surface ∈
  `{no_mutation_read_only, proposes_diff_only_no_apply,
   mutates_isolated_worktree_only}`.
- `side_branch_no_worktree` ⇒ active mutation surface ∈
  `{no_mutation_read_only, proposes_diff_only_no_apply,
   mutates_through_review_pipeline_only}`.
- `ephemeral_workspace` ⇒ active mutation surface ∈
  `{no_mutation_read_only, proposes_diff_only_no_apply,
   mutates_isolated_worktree_only,
   mutates_through_review_pipeline_only}`.
- `managed_remote_workspace` ⇒ active mutation surface ∈
  `{no_mutation_read_only, proposes_diff_only_no_apply,
   mutates_managed_remote_workspace_only,
   mutates_through_review_pipeline_only}`.

A session whose mutation surface falls outside the admitted set
denies with `mutation_surface_disagrees_with_target_locus`. A
session whose `target_execution_locus_class = current_worktree_assist`
but whose `mutation_surface_class = mutates_active_worktree_under_user_view`
denies with the same reason — assist-style execution may propose
diffs, never silently apply them.

### 1.5 Mutation-surface vocabulary

`mutation_surface_class` answers "what file system / branch / remote
state can this session change":

| Mutation surface                                  | Meaning                                                          |
|---------------------------------------------------|------------------------------------------------------------------|
| `no_mutation_read_only`                           | No filesystem, branch, or remote state mutated.                  |
| `proposes_diff_only_no_apply`                     | Diff(s) proposed; nothing applied. Default for assist flows.     |
| `mutates_isolated_worktree_only`                  | Writes to a side worktree on a side branch only.                 |
| `mutates_active_worktree_under_user_view`         | Mutates the active worktree under direct user view; only admitted with explicit user assent. |
| `mutates_managed_remote_workspace_only`           | Writes to the managed-cloud workspace only.                      |
| `mutates_through_review_pipeline_only`            | Mutations land only via the review pipeline (no self-merge).     |
| `mutation_unknown_unverified`                     | Mutation surface not yet inspected.                              |
| `mutation_disabled`                               | Mutation explicitly disabled.                                    |

A session whose `landing_posture_class = no_self_land_protected_branch`
or `no_self_land_review_required` MUST carry a mutation surface
that lands only via the review pipeline or proposes diffs only;
disagreement denies with
`landing_posture_disagrees_with_mutation_surface`.

### 1.6 Secret-scope vocabulary

`secret_scope_class` is the typed answer to "which credentials /
tokens / secret handles can this session reach":

| Secret scope                                     | Meaning                                                           |
|--------------------------------------------------|-------------------------------------------------------------------|
| `no_secrets_admitted`                            | The session has no secret handles admitted.                        |
| `read_only_redacted_handles`                     | Redacted handles only; ADR-0007 broker pass applies.               |
| `bounded_user_scoped_handles`                    | The user's BYOK / personal handles, bounded.                       |
| `bounded_admin_scoped_handles`                   | Admin-scoped handles, bounded.                                     |
| `enterprise_broker_scoped_handles`               | Enterprise-broker-scoped handles, bounded.                         |
| `secret_scope_unknown_unverified`                | Posture not yet inspected.                                         |
| `secret_scope_policy_blocked`                    | Admin policy refuses any secret handle for this session.            |

The session's `secret_scope_class` is narrower than or equal to the
originating workspace's admitted data-class allowlist; widening
denies with `secret_scope_widens_originating_workspace`.

### 1.7 Pause / cancel vocabulary

`pause_reason_class` enumerates why a session is paused:

`paused_for_checkpoint_review`, `paused_by_user`,
`paused_by_policy_epoch_roll`, `paused_by_route_circuit_open`,
`paused_by_validation_pending`, `paused_by_budget_warning`,
`paused_unknown`.

`cancel_outcome_class` enumerates why a session is cancelled
(terminal):

`cancelled_user_explicit`, `cancelled_user_session_closed`,
`cancelled_policy_safety`, `cancelled_policy_trust_revocation`,
`cancelled_policy_pack_quarantined`, `cancelled_budget_per_request`,
`cancelled_budget_per_session`, `cancelled_budget_per_agent`,
`cancelled_budget_per_workflow`, `cancelled_budget_per_user`,
`cancelled_budget_per_organisation`, `cancelled_route_no_admitted`,
`cancelled_route_all_fallbacks_exhausted`,
`cancelled_validation_must_run_failed`,
`cancelled_validation_check_blocked`, `cancelled_unknown`.

A cancelled session MUST cite a `cancel_outcome_class`; missing it
denies with `cancel_outcome_class_required_on_cancelled_session`.

## 2. Checkpoints

### 2.1 The checkpoint record

A `branch_agent_checkpoint_record` carries:

| Field                                  | Purpose                                                                |
|----------------------------------------|------------------------------------------------------------------------|
| `checkpoint_id`                        | Stable opaque id.                                                       |
| `branch_agent_session_ref`             | Opaque ref to the owning session.                                       |
| `checkpoint_index`                     | Zero-based monotonic index inside the session.                          |
| `checkpoint_class`                     | One of `pre_dispatch_plan_checkpoint`, `mid_run_progress_checkpoint`, `pre_mutation_dry_run_checkpoint`, `post_mutation_validation_checkpoint`, `pre_publish_review_checkpoint`, `recovery_resume_checkpoint`. |
| `triggering_class`                     | One of `triggered_by_validation_plan`, `triggered_by_user_pause`, `triggered_by_policy`, `triggered_by_budget_warning`, `triggered_by_route_circuit_open`, `triggered_by_extension_request`, `triggered_by_mutation_surface_change`, `triggered_by_taint_posture_change`. |
| `outcome_class`                        | One of `checkpoint_acknowledged_resume`, `checkpoint_paused_pending_user`, `checkpoint_paused_pending_policy`, `checkpoint_cancelled`, `checkpoint_superseded_by_later`. |
| `validation_plan_row_refs`             | Refs to the `branch_agent_validation_plan_record` rows the checkpoint covered. |
| `per_hop_route_receipt_refs`           | Refs to the per-hop route receipts under the chain at checkpoint time.  |
| `per_hop_spend_receipt_refs`           | Refs to the per-hop spend receipts under the chain at checkpoint time.  |
| `mutation_surface_class_at_checkpoint` | The mutation surface posture observed at checkpoint time.               |
| `policy_context`                       | Policy context observed.                                                |
| `redaction_class`                      | ADR-0011 redaction posture.                                             |
| `notes_summary`                        | Reviewable sentence describing the checkpoint.                           |
| `minted_at`                            | Monotonic timestamp.                                                    |

Checkpoints are append-only; a checkpoint never mutates a prior
checkpoint. A checkpoint that supersedes an earlier one cites the
prior id under `supersedes_checkpoint_refs`.

### 2.2 Checkpoint ordering invariants

- The first checkpoint on a session MUST be of class
  `pre_dispatch_plan_checkpoint`. A session whose first checkpoint
  is anything else denies with
  `first_checkpoint_must_be_pre_dispatch_plan`.
- A session that emits a `pre_publish_review_checkpoint` MUST be in
  state `paused_for_checkpoint_review` or `running` at the moment
  the checkpoint mints; otherwise denies with
  `pre_publish_checkpoint_state_disagreement`.
- A session that admits a mutation-surface change after dispatch
  MUST emit a `pre_mutation_dry_run_checkpoint` before the first
  hop with the new mutation surface class; missing it denies with
  `mutation_surface_change_missing_dry_run_checkpoint`.

## 3. Validation plan

### 3.1 The validation-plan row

A `branch_agent_validation_plan_record` carries:

| Field                                  | Purpose                                                                 |
|----------------------------------------|-------------------------------------------------------------------------|
| `validation_plan_row_id`               | Stable opaque id.                                                        |
| `branch_agent_session_ref`             | Opaque ref to the owning session.                                        |
| `validation_check_class`               | One of `must_run_test_suite`, `must_run_lint_suite`, `must_run_type_check`, `must_run_format_check`, `must_run_repo_custom_check_script`, `must_run_provider_required_check`, `must_run_review_pack_required_check`, `advisory_test_suite`, `advisory_review_check`, `advisory_local_lint`. |
| `referenced_check_bundle_ref`          | Opaque ref to the underlying check bundle row (re-export from review-pack contract). |
| `expected_outcome_class`               | One of `expected_outcome_pass_required`, `expected_outcome_pass_advisory`, `expected_outcome_pass_or_fail_advisory`, `expected_outcome_skipped_admitted`. |
| `observed_outcome_class`               | Empty pre-run; one of `outcome_passed`, `outcome_failed`, `outcome_skipped_admitted`, `outcome_blocked_by_policy`, `outcome_blocked_by_route`, `outcome_unknown_unverified` post-run. |
| `landing_constraint_refs`              | Refs to the landing-constraint rows that consume this check (so reviewers can trace which constraint each check satisfies). |
| `policy_context`                       | Policy context observed.                                                 |
| `redaction_class`                      | ADR-0011 redaction posture.                                              |
| `minted_at` / `last_updated_at`        | Monotonic timestamps.                                                    |

A `must_run_*` row whose `observed_outcome_class = outcome_failed`
forces the session into `cancelled_by_validation_failure`. An
advisory row that fails does not force cancellation; the failure
rides the review packet for reviewer judgement.

## 4. The branch-agent review packet

### 4.1 Minimum payload

A `branch_agent_review_packet_record` is minted on every terminal
session that produced inspectable output (`session_completed_review_only`,
`session_completed_landed_via_review`, every cancelled state, and
every paused-checkpoint-review). It MUST carry at minimum:

| Field                                  | Purpose                                                                |
|----------------------------------------|------------------------------------------------------------------------|
| `review_packet_id`                     | Stable opaque id.                                                       |
| `branch_agent_session_ref`             | Opaque ref to the owning session.                                       |
| `review_packet_state_class`            | One of `packet_draft_pending_completion`, `packet_ready_for_review`, `packet_in_review`, `packet_acknowledged_no_land`, `packet_landed_via_review`, `packet_cancelled`, `packet_superseded_by_replacement`. |
| `dispatch_origin_class`                | Re-exported from the session.                                           |
| `target_execution_locus_class`         | Re-exported from the session.                                           |
| `mutation_surface_class`               | Re-exported from the session.                                           |
| `secret_scope_class`                   | Re-exported from the session.                                           |
| `agent_ceiling_class`                  | Re-exported from the budget contract.                                   |
| `landing_posture_class`                | Re-exported from the session.                                           |
| `landing_constraint_refs`              | Opaque refs to the typed `branch_agent_landing_constraint_record` rows the packet binds. MUST include every constraint required by the matching landing posture (section 5). |
| `cost_envelope_class`                  | Coarse cost-band readout re-exported from the chain's spend rollup.     |
| `was_charged_to_user_class`            | Re-exported from the chain's spend rollup.                              |
| `route_rollup_ref` / `spend_rollup_ref` | Refs to the cumulative receipts.                                       |
| `per_hop_route_receipt_refs`           | Per-hop route-receipt ids under the chain.                              |
| `per_hop_spend_receipt_refs`           | Per-hop spend-receipt ids under the chain.                              |
| `validation_plan_row_refs`             | Refs to the validation-plan rows on the session.                         |
| `must_run_summary_class`               | One of `must_run_all_passed`, `must_run_some_failed`, `must_run_pending`, `must_run_skipped_admitted`, `must_run_blocked_by_policy`. |
| `test_plan_summary_class`              | One of `test_plan_full_suite_required_passed`, `test_plan_subset_required_passed`, `test_plan_subset_admitted_advisory`, `test_plan_skipped_admitted`, `test_plan_failed_blocking`. |
| `mutation_diff_summary_class`          | One of `no_diff_proposed`, `diff_proposed_no_apply`, `diff_applied_isolated`, `diff_applied_managed_remote`, `diff_applied_via_review_only`, `diff_blocked_no_apply`. |
| `change_stack_ref` / `patch_stack_ref` | Opaque refs to the change-stack / patch-stack rows the packet's diffs bind to. Empty when no diff was produced. |
| `review_finding_refs`                  | Opaque refs to the `review_finding_record` rows the packet surfaces (re-exported from the review-assist contract). |
| `originating_composer_session_ref`     | Same as the session's.                                                  |
| `originating_turn_draft_ref`           | Same as the session's.                                                  |
| `originating_request_workspace_ref`    | Typed `request_workspace_ref_record` snapshot at packet mint time.      |
| `originating_assembly_id_ref`          | Same as the session's.                                                  |
| `composer_plan_ref` / `prompt_pack_manifest_ref` / `tool_pack_manifest_ref` | Plan + pack identity at dispatch time.        |
| `redaction_class`                      | ADR-0011 redaction posture.                                             |
| `policy_context`                       | Policy context observed.                                                |
| `minted_at` / `last_updated_at`        | Monotonic timestamps.                                                   |

### 4.2 Required review fields

Every review packet renders five typed sections verbatim — not as
free-form prose:

1. **Budget band.** `cost_envelope_class` +
   `was_charged_to_user_class` re-exported from the chain's
   `branch_agent_spend_rollup_record`. Reviewers see the bucket; raw
   amounts never appear.
2. **Secret / credential scope.** `secret_scope_class` re-exported
   from the session; reviewers know whether the chain could reach
   credentials and at what scope.
3. **Mutation surface.** `mutation_surface_class` re-exported from
   the session; reviewers know whether the chain mutated nothing,
   proposed a diff, or applied diffs (and where).
4. **Test plan.** `validation_plan_row_refs` plus
   `must_run_summary_class` and `test_plan_summary_class`;
   reviewers see which checks ran, which were required, and which
   passed.
5. **Landing constraints.** `landing_constraint_refs`; reviewers see
   the typed constraints below verbatim. Every required constraint
   for the session's landing posture MUST appear.

A packet that omits any of the five sections denies with
`review_packet_required_section_missing`.

### 4.3 No-self-merge / no-direct-protected-push

Every review packet binds at minimum two landing-constraint rows
that re-encode the product's invariants:

- `landing_constraint_class = no_self_land_self_merge_forbidden`
  with `applies_to_session_state_class` covering every
  `session_completed_*` state.
- `landing_constraint_class = no_direct_protected_branch_push`
  with `applies_to_session_state_class` covering every
  `session_completed_*` state.

A packet missing either denies with
`landing_constraint_no_self_merge_required` or
`landing_constraint_no_protected_push_required`. These constraints
are the mechanical close of the "branch agent cannot self-land into
a protected branch" acceptance criterion.

## 5. Landing constraints

### 5.1 The landing-constraint row

A `branch_agent_landing_constraint_record` carries:

| Field                                  | Purpose                                                                 |
|----------------------------------------|-------------------------------------------------------------------------|
| `landing_constraint_id`                | Stable opaque id.                                                        |
| `branch_agent_session_ref`             | Opaque ref to the session this constraint applies to.                    |
| `landing_constraint_class`             | One of `no_self_land_self_merge_forbidden`, `no_direct_protected_branch_push`, `requires_human_review_acknowledgement`, `requires_review_pack_must_run_pass`, `requires_provider_required_check_pass`, `requires_admin_approval_ticket`, `requires_workspace_trust_trusted`, `requires_clean_review_findings`, `requires_no_pending_validation`, `requires_no_paused_checkpoint`, `requires_managed_remote_workspace_only`, `requires_published_via_review_thread`. |
| `applies_to_session_state_class`       | Array of `session_state_class` values the constraint covers (for example `["session_completed_review_only", "session_completed_landed_via_review"]`). |
| `enforcement_class`                    | One of `enforced_blocking_must_pass`, `enforced_blocking_admin_only_override`, `enforced_advisory_review_only`. |
| `referenced_review_pack_check_ref`     | Opaque ref to the review-pack check row when `landing_constraint_class = requires_review_pack_must_run_pass` or `requires_provider_required_check_pass`. Empty otherwise. |
| `originating_approval_ticket_ref`      | Required (non-empty) when `landing_constraint_class = requires_admin_approval_ticket`. |
| `policy_context`                       | Policy context observed.                                                 |
| `redaction_class`                      | ADR-0011 redaction posture.                                              |
| `notes_summary`                        | Reviewable sentence.                                                    |
| `minted_at`                            | Monotonic timestamp.                                                    |

The two no-self-merge / no-direct-protected-push constraints carry
`enforcement_class = enforced_blocking_must_pass`; admin override is
admitted only via a separate
`enforced_blocking_admin_only_override` row that MUST cite an
approval ticket.

### 5.2 Landing posture ↔ landing constraints

The schema's allOf gates pair `landing_posture_class` (on the
session and re-exported on the packet) to the minimum required
`landing_constraint_class` set:

- `no_self_land_review_required` ⇒ at minimum
  `no_self_land_self_merge_forbidden`,
  `no_direct_protected_branch_push`,
  `requires_human_review_acknowledgement`,
  `requires_review_pack_must_run_pass`.
- `no_self_land_protected_branch` ⇒ at minimum
  `no_self_land_self_merge_forbidden`,
  `no_direct_protected_branch_push`.
- `landed_via_review_pipeline_only` ⇒ at minimum
  `no_self_land_self_merge_forbidden`,
  `no_direct_protected_branch_push`,
  `requires_published_via_review_thread`.
- `not_eligible_to_land_read_only` / `not_eligible_to_land_blocked`
  / `landing_disabled` ⇒ at minimum
  `no_self_land_self_merge_forbidden`,
  `no_direct_protected_branch_push`. Read-only / blocked / disabled
  postures still emit the invariants so reviewers cannot interpret
  the missing constraints as silent permission.

Missing any required constraint denies with
`landing_constraint_required_for_posture_missing`.

## 6. Lineage back to composer / receipts

### 6.1 Lineage requirements

Every session, every checkpoint, and every review packet MUST
re-export the lineage spine the composer-session contract already
freezes:

- `originating_composer_session_ref` (composer session id),
- `originating_turn_draft_ref` (turn-draft id whose
  `dispatch_target_class` was `background_branch_agent`),
- `originating_request_workspace_ref` (typed
  `request_workspace_ref_record`),
- `originating_assembly_id_ref` (assembly id),
- `originating_branch_agent_dispatch_ref` (the
  `ai_branch_agent_dispatch_record` minted on the assembly),
- `composer_plan_ref` + `prompt_pack_manifest_ref` +
  `tool_pack_manifest_ref` at dispatch time,
- `route_rollup_ref` (the chain's
  `branch_agent_route_rollup_record`) and `spend_rollup_ref` (the
  chain's `branch_agent_spend_rollup_record`),
- per-hop `route_receipt_refs` and per-hop `spend_receipt_refs`.

Reviewers reconstruct *what was asked, what was assembled, what
plan / pack governed it, what each hop ran, and what each hop
cost* from these refs alone — no free-form prose required. A
record missing any required lineage ref denies with
`lineage_ref_missing_required`.

### 6.2 No silent escalation from assist to isolated automation

A session whose `target_execution_locus_class = current_worktree_assist`
but whose later checkpoint or per-hop receipt observes a different
target locus MUST mint a fresh session id (the prior session
becomes terminal under
`session_state_class = cancelled_by_policy` with
`cancel_outcome_class = cancelled_policy_safety` and a typed
denial reason `escalation_from_assist_to_isolated_forbidden`); the
escalation never happens silently.

The same rule applies to a session whose
`target_execution_locus_class` advertised `isolated_side_worktree`
but whose runtime tried to mutate the active worktree: the session
denies with `escalation_to_active_worktree_forbidden`. The
schema's allOf gates encode both denials.

## 7. Pre-dispatch / post-dispatch disclosure interaction

### 7.1 Pre-dispatch

The pre-dispatch disclosure (already governed in
`docs/ai/prompt_composer_contract.md` §14) carries
`branch_agent_dispatch_intent_disclosed` whenever the draft's
`dispatch_target_class = background_branch_agent`. The disclosure
record cites a `branch_agent_dispatch_placeholder_ref`; that
placeholder resolves to the
`ai_branch_agent_dispatch_record` on the assembly and (after
session mint) to the `branch_agent_session_id` on this contract.

### 7.2 Post-dispatch

Every post-dispatch event on the session MUST emit one
`branch_agent_session_audit_event_record` on the
`ai_branch_agent_session` audit stream. Generic "AI ran" copy for a
chain is forbidden; surfaces that want to render a single chip MUST
resolve through the session id and render at minimum:

- the `dispatch_origin_class` band,
- the `target_execution_locus_class` band,
- the `mutation_surface_class` band,
- the `cost_envelope_class` band (re-exported from the chain rollup),
- the `landing_posture_class` band.

A surface that hides any of those bands behind a generic chip
denies with `originless_branch_agent_chip_forbidden_resolve_through_session`.

## 8. Schema gates and denial reasons

### 8.1 Session ↔ locus / mutation / landing gates

The schema freezes:

- `target_execution_locus_class` ↔ `mutation_surface_class`
  (section 1.4).
- `landing_posture_class` ↔ `mutation_surface_class`
  (section 1.5).
- `landing_posture_class` ↔ minimum `landing_constraint_class` set
  (section 5.2).
- `agent_ceiling_class = agent_invocation_unbounded_admin_only` ⇒
  non-empty `originating_approval_ticket_ref` (section 1.1 + budget
  contract).
- `dispatch_origin_class = dispatched_from_review_handoff` ⇒
  non-empty `originating_review_handoff_ref` (section 1.3).

### 8.2 Frozen denial-reason set

The schemas freeze (beyond the receipt and budget contracts'
existing denials) at minimum:

- `dispatch_origin_class_unresolved`
- `dispatch_origin_missing_supporting_ref`
- `target_execution_locus_unresolved`
- `mutation_surface_class_unresolved`
- `secret_scope_class_unresolved`
- `mutation_surface_disagrees_with_target_locus`
- `landing_posture_disagrees_with_mutation_surface`
- `secret_scope_widens_originating_workspace`
- `escalation_from_assist_to_isolated_forbidden`
- `escalation_to_active_worktree_forbidden`
- `cancel_outcome_class_required_on_cancelled_session`
- `pause_reason_class_required_on_paused_session`
- `first_checkpoint_must_be_pre_dispatch_plan`
- `pre_publish_checkpoint_state_disagreement`
- `mutation_surface_change_missing_dry_run_checkpoint`
- `validation_must_run_failed_blocks_completion`
- `review_packet_required_section_missing`
- `landing_constraint_required_for_posture_missing`
- `landing_constraint_no_self_merge_required`
- `landing_constraint_no_protected_push_required`
- `lineage_ref_missing_required`
- `originless_branch_agent_chip_forbidden_resolve_through_session`
- `policy_epoch_rolled_invalidations`
- `branch_agent_session_schema_version_lagging`
- `branch_agent_review_packet_schema_version_lagging`

Denials fail closed; silent downgrade to a best-effort session, a
best-effort review packet, or a generic 'AI agent ran' chip is
forbidden.

## 9. Audit-event reuse

Every session mint / dispatch / pause / resume / checkpoint /
cancel / completion / review-packet emit fires on the
`ai_branch_agent_session` audit stream with const ids authored on
the schemas:

- `branch_agent_session_queued`
- `branch_agent_session_dispatched`
- `branch_agent_session_running`
- `branch_agent_session_paused`
- `branch_agent_session_resumed`
- `branch_agent_session_checkpoint_emitted`
- `branch_agent_session_cancelled`
- `branch_agent_session_completed_review_only`
- `branch_agent_session_completed_landed_via_review`
- `branch_agent_session_policy_blocked`
- `branch_agent_session_validation_plan_row_observed`
- `branch_agent_session_denial_emitted`
- `branch_agent_session_schema_version_bumped`

And on the `ai_branch_agent_review_packet` audit stream:

- `branch_agent_review_packet_minted`
- `branch_agent_review_packet_ready_for_review`
- `branch_agent_review_packet_in_review`
- `branch_agent_review_packet_acknowledged_no_land`
- `branch_agent_review_packet_landed_via_review`
- `branch_agent_review_packet_cancelled`
- `branch_agent_review_packet_superseded_by_replacement`
- `branch_agent_review_packet_denial_emitted`
- `branch_agent_review_packet_schema_version_bumped`

No new audit-event id is introduced on the
`ai_route_receipt`, `ai_spend_receipt`, `ai_context`,
`prompt_composer_session`, or `request_workspace` streams; those
streams keep their existing ids and the session / review packet
cross-reference them by ref.

## 10. Redaction posture

Every session, checkpoint, validation-plan row, review packet, and
landing-constraint row declares a `redaction_class` from the
ADR-0011 set. Raw URLs, raw paths, raw credential bodies, raw
filesystem patches in any specific encoding, raw worktree paths,
raw remote workspace URIs, and raw token / cost values never cross
this boundary. Exports, support bundles, claim manifests, evidence
packets, and replay captures carry opaque refs and structured
fields only.

Narrowing is permitted: admin policy MAY remove a
`target_execution_locus_class`, a `mutation_surface_class`, a
`secret_scope_class`, an `agent_ceiling_class`, a
`landing_posture_class`, or a `landing_constraint_class` from a
deployment profile. Widening beyond the frozen rules is forbidden.

## 11. Acceptance-criteria cross-walk

| Acceptance criterion                                                                                                                                       | Where enforced                                                                                                                                                                                                                          |
|------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Any branch-agent surface can show where the work ran, what it could touch, and how it must land using one shared lifecycle vocabulary.                     | Sections 1, 4, 5. Schemas: `target_execution_locus_class`, `mutation_surface_class`, `secret_scope_class`, `landing_posture_class`, and `landing_constraint_class` re-exported on every session, packet, and audit event.                |
| Checkpoint, cancellation, and final review packets preserve lineage back to the originating composer/request and any route/spend receipts.                | Sections 1.1, 6. Schemas: required `originating_composer_session_ref`, `originating_turn_draft_ref`, `originating_request_workspace_ref` (typed), `originating_assembly_id_ref`, `route_rollup_ref`, `spend_rollup_ref` on every record. |
| No fixture permits silent mutation of the active worktree or silent escalation from assist to isolated automation.                                         | Sections 1.4, 6.2, 8.1. Denials `mutation_surface_disagrees_with_target_locus`, `escalation_from_assist_to_isolated_forbidden`, `escalation_to_active_worktree_forbidden`.                                                                |
| Worked examples exist for queued agent, running agent, paused checkpoint review, policy-blocked dispatch, and completed agent that produces a review packet but cannot self-land. | Fixtures under `/fixtures/ai/branch_agent_cases/`.                                                                                                                                                                                       |

## 12. Schema-of-record posture

Rust types in the eventual branch-agent / orchestration crate are
the source of truth. The JSON Schema exports at
`schemas/ai/branch_agent_session.schema.json` and
`schemas/ai/branch_agent_review_packet.schema.json` are the
cross-tool boundary every non-owning surface reads.

Adding a new `dispatch_origin_class`, `target_execution_locus_class`,
`mutation_surface_class`, `secret_scope_class`,
`session_state_class`, `pause_reason_class`, `cancel_outcome_class`,
`landing_posture_class`, `landing_constraint_class`,
`enforcement_class`, `checkpoint_class`, `triggering_class`,
`outcome_class`, `validation_check_class`, `expected_outcome_class`,
`observed_outcome_class`, `review_packet_state_class`,
`must_run_summary_class`, `test_plan_summary_class`,
`mutation_diff_summary_class`, `audit_event_id`, or `denial_reason`
value is additive-minor and requires a
`branch_agent_session_schema_version` or
`branch_agent_review_packet_schema_version` bump; repurposing an
existing value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
revision; this mirrors the AI provider / model registry contract,
the AI graduation / budget contract, the AI prompt-composer
contract, the AI spend / route-receipt contract, and the ADRs
cited above.

## 13. Out of scope at this revision

- Implementing branch-agent runtimes, dispatch runners, or
  worktree allocators.
- Implementing merge-bots, auto-rebasers, or merge-queue
  arbitration runtimes.
- Implementing protected-branch policy engines.
- Authoring concrete check scripts, validation runners, or
  test orchestrators.
- Authoring concrete review-pack contents.
- Live cost estimation, live budget enforcement, or live secret
  broker integration.
- Provider-specific patch rendering or final review-packet UI
  chrome.

The contract freezes the shape those implementations will read and
write; the implementations themselves land in later milestones.
