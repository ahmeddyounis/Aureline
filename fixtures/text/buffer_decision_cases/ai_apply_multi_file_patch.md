# Fixture: AI apply produces an attributed multi-file patch with checkpoint

## Scenario

The user accepts an AI-suggested patch that touches four files
(rename a public field plus update three call sites). The AI flow
emits the patch as a preview; the user inspects and applies it.
Later, the user reviews the audit stream and undoes the apply.

## Hooks exercised

- `undo_group_open` — fires once per buffer the apply touches.
- `checkpoint_create` — fires once before the apply.
- `transaction_apply` — fires once per per-buffer rewrite, each
  carrying `class_id = machine_generated_change`, the `agent_id`,
  the `invocation_id`, the optional `prompt_or_input_handle`, the
  `preview_handle`, and the shared `workspace_group_id`.
- `undo_group_close` — fires once per buffer when the apply
  commits.
- `undo_apply` — undo reverts the workspace group across all four
  buffers atomically.

## Undo classes emitted

- `machine_generated_change`

## Stack elements stressed

- Attribution invariants: every machine-emitted transaction MUST
  carry `agent_id` and `invocation_id`.
- Preview-then-apply path: the apply group references the preview
  the user inspected so the audit stream can replay the decision.
- Workspace-level group across multiple buffers (same workspace
  semantics as multi-file refactor).
- Only-revertible posture: a divergent edit in any member buffer
  drops the redo stack for the apply group.

## Expected observable outcomes

- The journal records four per-buffer groups under one
  `workspace_group_id`, each with non-null `agent_id`,
  `invocation_id`, and `preview_handle`.
- The audit stream distinguishes the `machine_generated_change`
  apply from a human refactor with the same shape; assistive
  technology can announce "AI applied four-file patch" rather
  than "rename refactor".
- Undo reverts the entire apply atomically; partial-buffer revert
  is a contract violation.
- An apply path that lands without `agent_id` or `invocation_id`
  triggers the `agent_attribution_lost` reopen trigger and is
  rejected by the journal.

## ADR sections motivating this fixture

- Undo-class taxonomy — `machine_generated_change` row, including
  the `agent_attribution_lost` and `ai_apply_budget_breach` reopen
  triggers.
- Snapshot and checkpoint semantics — checkpoint at the start of
  any multi-file mutation.
