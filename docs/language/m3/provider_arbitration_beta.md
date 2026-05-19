# Language-intelligence arbitration and provider-health inspector (beta)

This document freezes the inspector contract that protects definition,
references, rename, formatting, organize-imports, and code-action lanes
once language packs claim beta on real workspaces. It binds the
per-provider health-state row read by editor chrome, quick-fix previews,
diagnostics detail, command results, CLI/headless inspect, and support
export together with the per-lane arbitration decision that records
provider order, the chosen winner, the confidence outcome, the
disagreement disclosure, the downgraded-promise copy, the fallback
label, the apply gate, and the back-references into the existing
capability-negotiation, result-provenance, and router-decision records.

Machine-readable companions:

- [`/schemas/language/provider_health_state.schema.json`](../../../schemas/language/provider_health_state.schema.json)
  — `provider_health_state_record`, the row every inspector consumer
  reads when it needs one answer to "how healthy, how fresh, how
  complete, and how reachable is this provider right now, and what retry
  or isolate action may I take?".
- [`/schemas/language/arbitration_decision.schema.json`](../../../schemas/language/arbitration_decision.schema.json)
  — `arbitration_decision_record`, the cross-surface record that binds
  one lane request to its provider order, winner, confidence outcome,
  conflict disclosure, fallback label, and apply gate.
- [`/fixtures/language/m3/provider_arbitration/`](../../../fixtures/language/m3/provider_arbitration/)
  — worked YAML cases that close the required lane, outcome, conflict,
  quarantine, and side-branch matrix.

This contract composes with and does not replace:

- [`/docs/language/provider_graph_and_arbitration_contract.md`](../provider_graph_and_arbitration_contract.md)
  — the underlying provider graph, capability arbitration, and
  result-provenance contract. The inspector projects from those records
  and does not invent a competing vocabulary.
- [`/docs/language/language_router_contract.md`](../language_router_contract.md)
  — the router-decision record. The inspector keeps a back-reference
  into the matching router decision so the same evidence travels across
  surfaces.
- [`/docs/language/refactor_preview_beta.md`](../refactor_preview_beta.md)
  — the refactor preview, validation, and rollback contract that wide-
  scope rename and code-action lanes must continue to satisfy when this
  inspector escalates to a preview or side-branch gate.

If this document disagrees with the PRD, TAD, TDD, or the linked
contracts above, those documents win and this document plus the
companion schemas update in the same change.

## Why freeze this now

Language intelligence on real workspaces fans into many lanes at once.
Without one inspector contract the surfaces can drift apart:

- editor chrome can render an exact badge while a hidden conflict gates
  the actual command result;
- quick-fix preview can hide a partial scope warning that the support
  export still discloses;
- a crash-looped language service can vanish from the badge while still
  appearing as the winner in CLI/headless inspect;
- wide-scope rename can preview as if everything is safe while the graph
  is only partial for the active workset.

The arbitration inspector pins all six lanes to one boundary so every
consumer reads the same record and the same downgraded-promise copy.

## Scope

Frozen at this revision:

- the `provider_health_state_record` health-strip row every inspector
  consumer reads;
- the `arbitration_decision_record` one-lane decision every consumer
  routes from;
- the closed lane vocabulary (`definition`, `references`, `rename`,
  `formatting`, `organize_imports`, `code_action`);
- the closed confidence outcomes (`exact`, `heuristic`, `partial`,
  `stale`, `unavailable`);
- the closed apply-gate vocabulary (`ready_to_apply`, `preview_required`,
  `side_branch_required`, `blocked_for_disagreement`,
  `blocked_for_partial_scope`, `blocked_for_health`, `inspect_only`);
- the closed fallback-label vocabulary (`none`, `text_fallback`,
  `heuristic_fallback`, `syntax_file_local_fallback`,
  `cached_semantic_reuse`, `advisory_ai_assist_only`,
  `unsupported_operation`);
- the closed disagreement visibility vocabulary (`none`,
  `inline_conflict_panel`, `side_panel_disagreement_inspector`,
  `preview_blocked_until_review`);
- the retry, isolate, and recovery-hint affordances that the health
  strip must expose for language-server, framework-pack, notebook
  adapter, generated-source-bridge, syntax, project-graph, and AI-assist
  providers;
- the closed consumer routing vocabulary (`editor_chrome`,
  `quick_fix_preview`, `diagnostics_detail`, `command_result`,
  `cli_headless_inspect`, `support_export`) that must read the same
  record on every surface.

Out of scope:

- live LSP, framework-pack, notebook, generator, project-graph, or AI
  provider integrations;
- final provider-scoring algorithms beyond the eligibility and honesty
  rules frozen in the underlying provider-graph contract;
- concrete pixel layout, iconography, or animation for the badge, panel,
  or strip surfaces.

## 1. Provider health-state row

Every provider that participates in a lane decision projects one
`provider_health_state_record`. The row answers four questions:

1. *What family and role is this provider?*
2. *What is its current health, freshness, scope claim, completeness,
   locality, and last-good timestamp?*
3. *Per supported lane, is it exact, heuristic, partial, stale, or
   unsupported right now?*
4. *Which retry, isolate, and recovery-hint affordances may the user
   trigger from the strip?*

Rules:

- The row carries no raw provider payloads, no raw process arguments, no
  raw hostnames, and no raw secret material. Identity flows as opaque
  ids and reviewable summaries only.
- `health_state: crash_loop_quarantined` requires a `quarantine_ref`, a
  health reason summary, and `freshness_class != authoritative_live`.
  The row's downgraded promise must read
  `crash_loop_excluded`, and the retry/isolate controls must expose at
  least one admissible action (`manual_recovery_required` plus
  `quarantine_for_workspace` counts as admissible).
- `provider_family: ai_assist` is permanently `assist_only`, never
  carries an `exact` lane support class, and emits an AI-flavored
  downgraded promise (`advisory_only_ai_assist`, plus the narrowed,
  cached, policy, or remote variants).
- `locality_class: imported_snapshot` may not claim
  `authoritative_live`.

## 2. Arbitration decision record

Every lane request produces one `arbitration_decision_record`. The
record binds:

- the requested lane, authority floor, scope claim, and subject ref;
- the ordered `provider_order_rows` with rank, family, role, and a
  reference into the matching health-state row;
- the chosen provider id (or `null` when the outcome is `unavailable`);
- the confidence outcome and the negotiated scope, completeness, and
  freshness for the visible answer;
- the disagreement block (`conflict_class` plus
  `disagreement_visibility_class` and a reviewable summary);
- the downgraded-promise block (`downgraded_promise_reason_class` plus a
  reviewable summary);
- the fallback label that travels with the answer;
- the apply gate that the surface must enforce;
- the consumer routing rows that route the record to every consumer;
- the back-references into the existing capability negotiation packet,
  result provenance record, and router decision record;
- the policy context, redaction class, capture timestamp, and
  export-safe summary.

Rules:

- `confidence_outcome_class: exact` requires
  `negotiated_completeness_class: complete_for_claimed_scope`,
  `negotiated_freshness_class: authoritative_live`,
  `fallback_label_class: none`, and a downgraded promise reason of
  `none`. The chosen provider must be honestly authoritative for its
  declared scope.
- `confidence_outcome_class: unavailable` requires
  `chosen_provider_id: null` and a blocked or inspect-only apply gate.
- `confidence_outcome_class: heuristic`, `partial`, and `stale` require
  a non-`none` fallback label so the visible answer never reads as
  exact.
- `confidence_outcome_class: partial` requires partial completeness and
  routes apply through `preview_required`, `side_branch_required`,
  `blocked_for_partial_scope`, `blocked_for_disagreement`, or
  `inspect_only`.
- Wide-scope rename (`active_workset` or `whole_workspace`) with
  partial completeness must enforce one of those preview, side-branch,
  blocked, or inspect-only gates. Rename cannot be ready-to-apply with
  partial truth.
- A non-`none` conflict class blocks `ready_to_apply` and requires a
  visible disagreement panel.
- Provider preference may shift the rank order, but it cannot hide a
  conflict, hide a stale state, or hide a scope warning. The schema
  carries the disclosure regardless of ordering.

## 3. Consumer routing

The same arbitration decision feeds:

- `editor_chrome` — the badge, hover strip, and inline conflict panel.
- `quick_fix_preview` — the preview/side-branch gate around apply.
- `diagnostics_detail` — the per-diagnostic provenance and disagreement
  block.
- `command_result` — the result panel that lists the chosen winner with
  fallback label and alternates.
- `cli_headless_inspect` — the CLI inspector that prints the same
  vocabulary without UI chrome.
- `support_export` — the support evidence packet that captures the
  health-state rows, the decision, and the policy context.

Each decision must route to at least `editor_chrome`, `command_result`,
`cli_headless_inspect`, and `support_export`. Quick-fix preview and
diagnostics detail are required whenever the lane drives a quick-fix or
diagnostic surface. This rule is enforced by the inspector evaluator and
covered by the worked corpus.

## 4. Worked fixture corpus

The companion corpus closes the matrix:

- `lsp_framework_definition_disagreement.yaml` — `definition`, `exact`,
  framework-vs-language disagreement, inline conflict panel.
- `graph_partial_references_scope_narrowed.yaml` — `references`,
  `partial`, narrowed-to-workset disclosure, preview-required gate.
- `notebook_rename_preview_required.yaml` — `rename`, `partial`,
  notebook-boundary disagreement, side-branch-required gate.
- `formatting_fallback_to_text.yaml` — `formatting`, `heuristic`,
  text-fallback label, degraded language-service disclosure.
- `organize_imports_crash_loop_quarantined.yaml` — `organize_imports`,
  `unavailable`, quarantined provider, retry/isolate actions still
  visible.
- `code_action_wide_scope_side_branch.yaml` — `code_action`, `stale`,
  cached-semantic reuse, AI assist staying advisory, side-branch gate.

The inspector evaluator validates that the corpus covers every lane,
every confidence outcome, the required apply gates, the required
consumers, and the quarantined-provider case.

## 5. Acceptance checklist

A reviewer can audit conformance without implementation code:

1. **Provider identity**: Can you tell which provider family and role
   answered each lane, and how it ranked against other providers?
2. **Health honesty**: Can you tell the provider's health, freshness,
   scope claim, completeness, locality, and last-good timestamp without
   reading UI chrome?
3. **Confidence outcome**: Can you tell whether the visible answer is
   exact, heuristic, partial, stale, or unavailable, and which fallback
   label travels with it?
4. **Disagreement disclosure**: When providers diverge, can you see the
   conflict class and the visible disagreement panel?
5. **Apply gate**: Can you tell whether the surface allowed apply,
   required preview, required side-branch review, or blocked outright?
6. **Crash-loop recovery**: Are quarantined providers inspectable with
   retry, isolate, and recovery-hint controls instead of vanishing from
   the strip?
7. **Wide-scope rename guardrail**: Are partial wide-scope rename and
   refactor surfaces structurally unable to read as ready-to-apply?
8. **Consumer parity**: Do editor chrome, quick-fix preview, diagnostics
   detail, command result, CLI/headless inspect, and support export
   read the same arbitration decision?

If any answer above requires inferring hidden provider state from UI
chrome or reading implementation code, the surface is non-conforming.
