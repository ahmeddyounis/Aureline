# Fixture: workflow block — AI apply blocked by partial_results + policy

## Scenario

The user triggered an AI apply across three files. One file's
apply finished with partial citations (two rows cite durable
anchors; one row is inferred without an anchor). Admin policy
forbids promoting inferred rows to durable truth without
citations. The shell MUST block the commit step, preserve the
partial result for review, and keep the rest of the session
interactive.

## Row bindings

- **Failure-tier row.** `tier.workflow_block`.
- **Lifecycle row.** `lifecycle:ai_action`, state
  `ai.awaiting_review` with `ai.partial_results` on one row.
- **Controlled label.** None on the workspace (the workspace is
  not degraded); the review surface carries typed per-row
  labels.
- **Truth class.** `ai_inferred_truth` on the AI rows;
  `workspace_vcs_truth` on the target files.
- **Degraded-state token.** `Partial` on the apply summary row;
  `PolicyBlocked` on the commit affordance until the inferred
  row is cited or discarded.

## Required axes rendered

- **Cause token.**
  `ai_inferred_truth_promoted_without_citation` on the single
  row that lacks an anchor.
- **Preserved.** Other workspace edits; save pipeline; recovery
  journal; the AI draft itself (visible for review).
- **Blocked.** `Commit` on the affected rows; `Publish`; any
  consequence-bearing export.
- **Next-safe action hooks.** `compare_before_restore` (reveal
  citations), `review_archetype_match` (retry with narrower
  scope or with explicit evidence), `open_minimal` (discard the
  draft), `roll_back_import` (roll back the apply).

## Placement

- **Failure tier.** `tier.workflow_block`.
- **Delivery surface.** `window_attached_sheet` on the AI
  apply review surface (the user's existing work stays visible
  behind the sheet; focus-return on dismiss returns to the
  triggering row).
- **Interruptibility tier.** `tier_blocking_trust` (the commit
  step cannot continue without an approved resolution).

## Recovery affordances (keyboard-reachable)

- **`Reveal citations`** — opens the citation anchor inspector
  for each row; the inferred row is marked verbatim
  ("Inferred — no anchor").
- **`Retry with narrower scope`** — reissues the AI apply with
  a narrower scope (single file, single symbol).
- **`Discard draft`** — discards the uncommitted AI draft
  (`rung.none_required`).
- **`Roll back apply`** — rolls back partial applies on the
  other files that were already committed
  (`next_step_decision_hook = roll_back_import`).
- **`Open support export`** — packages the evidence for
  escalation (`object_issue_handoff`).

## Last-failure reason (mandatory visibility)

- **Keyboard-reachable reveal.** On the AI apply review
  surface, the inferred row carries a reveal affordance
  ("Reveal last failure"). The policy-blocked commit carries
  the policy source label ("Admin policy requires citation for
  durable AI rows").
- **Support-export field.** `recovery_ladder_packet`,
  `object_issue_handoff`. Preserves the AI apply id, the
  inference vs. cited evidence boundary, the policy source ref,
  and the recovery-rung history.
- **Export-safe description.** Redaction pass runs; the raw
  prompt, the raw model output, and any raw external content
  are redacted per the suspicious-content vocabulary.

## Promotion triggers

- **From `tier.contextual_degraded`.** The partial AI result on
  the review surface was contextually degraded; the commit
  action promoted it to a workflow block because the commit
  cannot continue safely.
- **To `tier.session_recovery`.** The user chooses to halt the
  whole AI apply or the subsystem crashes; dialog-modal entry
  to `rung.safe_mode` + `rung.none_required` cleanup.

## Recovery / support / measurement

- **Recovery-ladder rung.** `rung.none_required` on
  discard / narrow-retry; `rung.restricted_mode_fallback` if
  the user continues without AI apply capability for the
  session.
- **Support-packet family.** `recovery_ladder_packet`,
  `object_issue_handoff`.
- **Journey-trace class.** `open_edit_save`
  (consequence-bearing apply).

## Accessibility

- The inferred row's "Inferred — no anchor" label is announced
  by assistive technology.
- The policy source label is addressable separately from the
  commit affordance.
- Focus-return on dismiss returns to the triggering row.

## Forbidden copy and patterns on this path

- "Something went wrong"
- "AI error"
- "Try again"
- Silent commit of the inferred row
- Surprise modal (the sheet is reached through the navigation/
  escalation ladder's monotonic climb)
- Undo verb on `compensating_rollback` /
  `regenerate_from_canonical_source` classes

## Expected observable outcomes

- The commit step is blocked until the inferred row is cited
  or discarded; rolling back surfaces the rollback evidence.
- Last-failure reason and the inference vs. cited evidence
  boundary are preserved on export.
- Focus-return on dismiss is honoured.
- `overclaims_readiness = false` is asserted.

## Fixture fields (seed)

```yaml
__fixture__:
  name: workflow_block_ai_apply_blocked
  taxonomy_rows:
    - tier.workflow_block
    - lifecycle:ai_action
  doc_section: docs/ux/state_and_recovery_taxonomy.md#7.3
running_build_identity_ref: build-identity-seed-state-wf-ai-blocked
overclaims_readiness: false
```
