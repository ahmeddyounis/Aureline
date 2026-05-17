# Alpha Declarative Recipes with Reviewability, Approval Fences, and Command-Graph Parity

This document is the reviewer-facing landing page for the declarative
recipe alpha record family. Recipes give Aureline a safer automation
lane by expressing repeatable workflows as ordered sequences of typed
steps where each step resolves to a stable command id and approval
class on the shared command graph. Reviewers, support exports, and the
activity history read one truth: definitions are diffable, mutating or
provider-facing steps preserve preview and approval behavior at run
time, and every run resolves to exactly one typed disposition.

The machine-readable boundary lives at
[`/schemas/commands/recipe.schema.json`](../../../schemas/commands/recipe.schema.json).
The Rust implementation lives at
[`/crates/aureline-runtime/src/recipes/`](../../../crates/aureline-runtime/src/recipes/mod.rs).
The protected fixture lives at
[`/fixtures/runtime/recipe_alpha/page.json`](../../../fixtures/runtime/recipe_alpha/page.json).

## The alpha promise

- Every recipe definition reuses the same stable `command_id`,
  `command_revision_ref`, and `approval_class` taxonomy that direct
  user actions use on the command graph. Bespoke automation shortcuts
  are refused.
- Every step resolves to exactly one
  [`StepCommandLineageClass`](../../../crates/aureline-runtime/src/recipes/mod.rs)
  (`core_command`, `imported_command`, `extension_command`,
  `ai_tool_handle`, `cli_verb`, `provider_action`). The
  `unmapped_command_denied` lane is the closed refusal class for steps
  whose `command_id` does not resolve on the command graph and is
  refused on an admitted definition at validate time.
- Mutating steps (`editor_buffer_mutation`, `editor_multi_file_mutation`,
  `branch_mutation`, `worktree_mutation`, `provider_mutation`,
  `settings_mutation`, `network_mutation`, `process_mutation`) MUST
  carry a `preview_required_*` `preview_requirement` and a non
  `no_approval_required` `approval_class`. Provider-facing steps MUST
  carry `preview_required_provider_mutation` (or
  `preview_required_before_apply`) and `single_step_approval_required`,
  `recipe_approval_required`, or `admin_signed_approval_required`.
  Branch and worktree mutations MUST carry their scoped
  `preview_required_branch_mutation` / `preview_required_worktree_mutation`
  (or `preview_required_before_apply`).
- Every recipe run resolves to exactly one
  [`RecipeRunDispositionClass`](../../../crates/aureline-runtime/src/recipes/mod.rs):
  `proceed_local_editor_only`, `proceed_after_recipe_approval`,
  `preview_required_before_apply`,
  `downgraded_to_observer_no_mutation`, `promoted_to_full_recipe_run`,
  or `denied_unsafe_recipe`. Each disposition pins the required ref:
  approval-tickets, preview-tickets, promoted-run refs, downgrade-target
  labels, or denial-reason labels.
- Every definition and run mints attribution rows on
  `support_export` and `activity_history`; admin-signed approvals also
  mint an `admin_audit_export` attribution. The recipe artifact is
  diffable (definitions carry `declared_write_classes` and
  `declared_approval_classes` as the union of every step's writes and
  approval class), exportable (the
  [`RecipeAlphaSupportExport`](../../../crates/aureline-runtime/src/recipes/mod.rs)
  projection strips raw payload fields and silent-authority guards),
  and support-visible.

## Run-disposition vocabulary

| Disposition | Required ref |
| --- | --- |
| `proceed_local_editor_only` | none |
| `proceed_after_recipe_approval` | `approval_ticket_ref` |
| `preview_required_before_apply` | `preview_ticket_ref` |
| `downgraded_to_observer_no_mutation` | `downgrade_target_label` |
| `promoted_to_full_recipe_run` | `promoted_run_ref` |
| `denied_unsafe_recipe` | `denial_reason` + `denial_reason_label` |

## Step-disposition vocabulary

| Disposition | Meaning |
| --- | --- |
| `proceed_no_approval` | Step proceeded without an approval gate (read-only steps) |
| `proceed_after_preview` | Step proceeded after preview confirmation |
| `proceed_after_approval` | Step proceeded after an approval ticket was minted |
| `preview_required_before_apply` | Step held at preview surface |
| `downgraded_to_observer_no_mutation` | Step rendered as observer-only, no mutation applied |
| `denied_unsafe_step` | Step denied; MUST cite a `denial_reason` |

## Audit-event vocabulary

| Event class | When it fires | Required field |
| --- | --- | --- |
| `recipe_admitted` | Recipe run admitted | `definition_ref` |
| `recipe_denied` | Recipe run denied | `denial_reason_label` |
| `recipe_step_preview_minted` | Preview surface minted for a step | `run_ref` |
| `recipe_step_approval_minted` | Approval ticket minted for a step | `run_ref` |
| `recipe_step_admitted` | Step admitted | `run_ref` |
| `recipe_step_denied` | Step denied | `denial_reason_label` |
| `recipe_run_started` | Run lifecycle started | `run_ref` |
| `recipe_run_completed` | Run lifecycle completed | `run_ref` |
| `recipe_run_aborted` | Run aborted (e.g. downgrade or hard refusal) | `run_ref` |
| `recipe_run_promoted_to_approved_run` | Run promoted to a full approved run | `run_ref` |
| `attribution_minted` | Attribution row minted | `attribution_ref` |
| `audit_denial_emitted` | A denial was emitted | `denial_reason_label` |

## Guardrails (closed)

The validator refuses any of the following:

- `raw_branch_mutation_present`, `raw_worktree_mutation_present`,
  `raw_provider_payload_present`, or `raw_credential_present` set to
  `true` on a definition.
- `silent_authority_widening_taken` set to `true` on a definition or
  run.
- `remote_attach_degraded_state_masked` set to `true` on a definition
  or run — recipes never mask remote-attach degraded state.
- `trust_gate` or `trust_gate_observed` set to `managed_only_denied`.
- A step declaring `credential_mutation_denied` in `write_classes`,
  `terminal_mode_denied` as its `mode_requirement`, or
  `unmapped_command_denied` as its `step_command_lineage` — these are
  closed refusal lanes.
- A mutating step without a `preview_required_*` preview requirement.
- A mutating step without an approval class other than
  `no_approval_required`.
- A provider-facing step without provider-preview or approval.
- A branch- or worktree-mutating step without the matching scoped
  `preview_required_*` preview requirement.
- A `proceed_after_recipe_approval` run without an
  `approval_ticket_ref`; a `preview_required_before_apply` run without
  a `preview_ticket_ref`; a `promoted_to_full_recipe_run` run without a
  `promoted_run_ref`; a `downgraded_to_observer_no_mutation` run
  without a `downgrade_target_label`; a `denied_unsafe_recipe` run
  without a `denial_reason` and `denial_reason_label`.
- A run whose `definition_ref`, step disposition's `step_ref`, audit
  event's `run_ref` or `definition_ref`, or attribution's
  `definition_ref`/`run_ref` does not resolve to a record in the page.
- A page missing coverage of `support_export` and `activity_history`
  attribution surfaces; missing coverage of
  `proceed_local_editor_only`, `preview_required_before_apply`, and
  `denied_unsafe_recipe` run dispositions; or missing coverage of
  `recipe_run_started`, `recipe_run_completed`, and
  `attribution_minted` audit events.

## What review must enforce

Reviewers approving a change in this lane MUST confirm:

- Recipe steps reference stable `command_id` values that already exist
  on the command graph (no inlined ad-hoc shell fragments, no bespoke
  automation shortcuts).
- Mutating and provider-facing steps preserve preview and approval
  behavior when executed from a recipe — the validator's
  `recipe_alpha.step_mutation_missing_preview`,
  `recipe_alpha.step_mutation_missing_approval`,
  `recipe_alpha.step_provider_mutation_missing_preview`, and
  `recipe_alpha.step_provider_mutation_missing_approval` checks pin
  these invariants.
- `declared_write_classes` and `declared_approval_classes` on a
  definition equal the union of every step's writes and approval
  classes; the validator refuses underdeclared or overdeclared totals
  so the diffable surface stays truthful.
- Recipe runs never silently widen authority, never mask remote-attach
  degraded state, and never silently proceed without preview or
  approval when the recipe is mutating or provider-facing.
- Every definition and run mints attribution rows on
  `support_export` and `activity_history`; the support-export
  projection at
  [`RecipeAlphaSupportExport`](../../../crates/aureline-runtime/src/recipes/mod.rs)
  is the canonical export shape and strips raw payload fields and
  silent-authority guards.
