# M5 AI-message-card and evidence-timeline controls

This is the fourth and final **implement lane** over the frozen
[M5 editor-inline component matrix](../../schemas/ui/m5-editor-inline-component-matrix.schema.json)
(see the [component contract](m5_editor_inline_components_contract.md)), after the
[editor-tab / gutter lane](m5_editor_tab_and_gutter_controls.md), the
[diagnostic-decoration / code-action-chip lane](m5_diagnostic_decoration_and_code_action_chip_controls.md),
and the [diff-view / review-thread lane](m5_diff_view_and_review_thread_controls.md).
It turns the two inline *AI-evidence* components — the **AI message card** and the **evidence
timeline** — into resolvers that produce export-safe, honest projections across the claimed M5 editor,
review, notebook, AI, support, and product surfaces. The goal is to turn AI output and evidence trails
into inspectable product objects instead of opaque chat bubbles or hidden logs.

- Rust source: `crates/aureline-editor/src/m5_ai_message_card_and_evidence_timeline_source_confidence_and_evidence_lineage/`
- Combined schema: [`schemas/ui/m5-ai-message-card-evidence-timeline-controls.schema.json`](../../schemas/ui/m5-ai-message-card-evidence-timeline-controls.schema.json)
- Per-component schemas: [`m5-ai-message-card.schema.json`](../../schemas/ui/m5-ai-message-card.schema.json),
  [`m5-evidence-timeline.schema.json`](../../schemas/ui/m5-evidence-timeline.schema.json)
- Proof packet: `artifacts/release/m5-ai-message-card-evidence-timeline-controls-proof/`
- Narrowed fixtures: `fixtures/ui/m5-ai-message-card-evidence-timeline-controls/`

The Rust validator in `crates/aureline-editor` is the authoritative gate; this doc and the schema
document the shape.

## What the resolvers guarantee

### `resolve_ai_message_card`

An AI message card reads as a clean, legible state only when it names:

- the **message identity / label**, never unstated;
- the **lifecycle state** (draft, streaming, review-required, blocked-by-policy, applied, reverted,
  failed, or stale-evidence) using **one shared vocabulary**, never collapsed into one generic completed
  message;
- the **approval state**, so a review-required or blocked message never reads as already applied;
- the **confidence / uncertainty class**, never left implicit;
- the **source context** (grounded-in-workspace, grounded-in-docs, model-prior-only,
  retrieved-external, or no-source-cited), never presenting a non-workspace answer as workspace-grounded;
- the **route / provider locality** (local-model, hosted-provider, mirrored-cache, byo-key-provider, or
  offline-replay), keeping the local-versus-hosted-provider distinction explicit;
- the **spend / cost posture** where claimed (no-cost, metered-local, metered-hosted, budget-capped, or
  over-budget), never presenting a metered / over-budget message as free;
- the **available safe actions**, never leaving a user without a route to trust or apply.

It degrades — never silently passes — when the identity is unstated, the state is unresolved or generic,
approval is hidden, confidence is unstated, the source context is unresolved or undisclosed, the route
locality is unresolved or implicit, the spend posture is unresolved or undisclosed, no safe actions are
offered, or no command-backed detail path is reachable.

### `resolve_evidence_timeline`

An evidence timeline reads as a clean, legible state only when it names:

- the **evidence entry identity / label**, never unstated;
- a **timestamp**, never missing;
- the **evidence kind** (tool-invocation, validation-run, retrieval, user-edit, or external-reference),
  never flattened into a generic log line;
- the **tool / validation lineage** (tool, validation, run, change, or resource lineage), never left
  implicit;
- a **related run / change / resource**, never missing;
- the **disclosure state**, disclosing that a redacted or partially-loaded trail is incomplete rather
  than reading as complete;
- an **inspectable, non-opaque structure**, never hidden in an opaque log;
- **open / replay / export actions**, never omitted.

It degrades when the identity is unstated, the timestamp is missing, the evidence kind or lineage is
unresolved, the lineage is unstated, no related resource is named, the disclosure state is unresolved, a
redacted / partial trail is not disclosed, the trail is an opaque log, no replay / export action is
offered, or no command-backed detail path is reachable.

## Hard invariants

Every controls row carries four hard invariants that must stay `false`:

- `ai_message_state_or_source_context_silently_generic`
- `ai_route_or_spend_posture_silently_drifts`
- `evidence_timeline_hidden_in_opaque_log`
- `evidence_lineage_or_redaction_truth_silently_drifts`

## Acceptance criteria, proven by examples

The packet's `validate()` proves each acceptance criterion against the resolved examples rather than
merely asserting a governance bool:

1. **AI surfaces across claimed M5 lanes expose the same message and evidence vocabulary rather than
   per-feature chat chrome.** Clean cards cover at least two distinct message states and span
   local-model and hosted-provider route localities, clean evidence covers at least two distinct
   evidence kinds, a generically-encoded state example degrades, and no clean card is generic.
2. **Users can inspect source context, approval state, and supporting evidence before treating an AI
   output as ready to trust or apply.** At least one clean card discloses a non-workspace source, an
   approval-hidden example degrades, no clean card hides approval, and a clean card and clean evidence
   both expose a command-backed detail entrypoint.
3. **Timeline and export consumers preserve lineage and redaction truth instead of flattening AI history
   into unstructured logs.** Clean evidence covers at least two distinct lineage classes, an opaque-log
   example degrades, a redaction-hidden example degrades, no clean evidence is opaque, and no clean
   evidence hides a redacted / partial trail.

## Regenerating the artifacts

The headless emitter is the only mint-from-truth path:

```text
cargo run -p aureline-editor --example dump_m5_ai_evidence_controls -- support-export
cargo run -p aureline-editor --example dump_m5_ai_evidence_controls -- report
cargo run -p aureline-editor --example dump_m5_ai_evidence_controls -- csv
cargo run -p aureline-editor --example dump_m5_ai_evidence_controls -- fixture-ai-ui-beta-narrowed
cargo run -p aureline-editor --example dump_m5_ai_evidence_controls -- fixture-support-export-preview-narrowed
```
