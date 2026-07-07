# M5 budget-size-strip and tainted-context-warning primitive contract

Task: **M05-887** — Implement budget-size strips, omitted-context drawers, and tainted-context
warnings with token-pressure, truncation, route-change, and review-before-send truth across the
claimed M5 AI lanes.

This lane narrows the `budget_size_strip` and `tainted_context_warning` families from the frozen
[prompt-composer component matrix](./freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix.md)
(M05-884) into two reusable primitives: a budget-or-size strip (with its omitted-context
drawer), a tainted-context-warning resolver, and one shared parity matrix. A user can tell —
from the budget strip or the warning alone — which context classes are included versus omitted,
why context was truncated or withheld, how much token or size pressure the request is under,
whether the route changed and what that means, and, for any untrusted input, its source, its
severity, whether it is treated as data, and the review path that must run before a
side-effectful AI route.

## Primitives

- Budget-or-size strip: `resolve_budget_size_strip(&M5BudgetSizeStripResolutionInput) -> Result<M5ResolvedBudgetSizeStrip, M5BudgetSizeStripResolutionError>`.
- Tainted-context warning: `resolve_tainted_context_warning(&M5TaintedContextWarningResolutionInput) -> Result<M5ResolvedTaintedContextWarning, M5TaintedContextWarningResolutionError>`.
- Parity matrix packet: `M5BudgetTaintPacket`, one row per claimed send-capable composer
  consumer, each carrying worked budget and warning resolution cases.

### Budget-posture ladder (blocking-first)

1. `hard_ceiling_hit` → **hard_blocked** (not sendable; band exhausted).
2. `over_budget` → **over_budget** (band critical; requires review before send).
3. `truncation_pending` → **truncation_pending** (band critical; truncation active).
4. `near_limit` → **near_limit** (band watch).
5. `unmetered_local` → **unmetered_local** (band unmetered).
6. otherwise → **within_budget** (band nominal).

The pressure band is a coarse, non-tokenizer-trivia band derived from the posture
(`unmetered` / `nominal` / `watch` / `critical` / `exhausted`).

### Omitted-context drawer

Every omitted-context drawer entry names its context class, its reason
(`size_truncated` / `budget_capped` / `policy_excluded` / `dedup_collapsed` / `stale_dropped`),
and an opaque detail. A `none_omitted` entry or an empty detail is rejected, so context is never
silently dropped. Whenever anything was omitted, the strip offers an `inspect_omitted_context`
action so the inspect path is preserved before send.

### Route-switch consequence

The strip derives the consequence of a route change from the before / after route classes:
`unchanged`, `locality_changed` (crossing the on-device boundary), `reach_widened`,
`reach_narrowed`, or `provider_changed` (same reach, different provider class). A changed route
offers a `review_route_change` action, and the consequence is exportable rather than inferred
from later evidence.

### Taint-warning-posture ladder (severity-first)

1. severity `none` → **no_taint_trusted** (proceedable).
2. severity `injection_suspected` → **injection_blocked** (blocks send).
3. severity `quarantine_required` → **quarantine_held** (blocks send).
4. severity `elevated` → **elevated_review_required** (or **acknowledged_proceedable** once
   acknowledged); blocks a side-effecting send until reviewed.
5. severity `informational` → **flagged_as_data** (or **acknowledged_proceedable** once
   acknowledged).

Any tainted input must be treated as data rather than instruction (`treats_untrusted_as_data`),
a held or blocked warning must carry its quarantine note, and a tainted warning always offers a
`review_tainted_content` action so the review path is preserved before any side-effectful route
runs.

## Invariants

Each matrix row asserts four hard invariants (all `false`):

- `masks_budget_or_omission_truth`
- `downplays_taint_source_or_severity`
- `invents_private_context_grammar`
- `bypasses_review_before_side_effecting_send`

## Acceptance-criterion coverage

- **Budget or size strips surface omitted or truncated context truth everywhere the user can
  send.** `budget_omission_disclosure_unproven` fires unless a worked strip proves an omission
  disclosed (with reason and detail) that requires review before send, across all five
  send-capable consumers.
- **Tainted-context warnings appear for the defined input classes and preserve a review path
  before any side-effectful AI route runs.** `taint_input_class_coverage_unproven` fires unless
  pasted external text, promoted tool output, and prior model output are each proven;
  `taint_review_path_unproven` fires unless a side-effecting route that blocks send preserves its
  review path; `taint_treated_as_data_unproven` fires unless every tainted worked warning treats
  its untrusted content as data.
- **Route-change and omission states are explicit and exportable.**
  `budget_route_change_coverage_unproven` fires unless a worked strip proves a route change with a
  `review_route_change` action, and the omitted-context drawer and route switch are carried as
  export fields.

## Boundary

Raw prompts, pasted bodies, tool-output bodies, raw paths, raw URLs, credentials, and private
endpoints never cross this boundary; every strip id, warning id, context label, omitted-context
detail, and quarantine note is carried only as an opaque, export-safe representation. The
`raw_material_in_export` violation and the resolver's forbidden-material errors reject obviously
sensitive strings.

## Source contracts

- Boundary schema: `schemas/ai/m5-budget-size-strip-and-tainted-context-warning.schema.json`.
- Frozen component matrix: `schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json`.
- Context assembly: `schemas/ai/context_assembly.schema.json`.
- Tainted context: `schemas/ai/tainted_context.schema.json`.

## Artifacts

- Support export: `artifacts/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/support_export.json`.
- Matrix CSV: same directory, `matrix.csv`.
- Markdown report: `artifacts/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes.md`.
- Narrowed fixtures: `fixtures/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/`.

All are minted from the seed builders by the headless emitter
`aureline_ai_budget_size_strip_tainted_context_warning_primitive`; the inline tests assert the
checked artifacts never drift from the seed.
