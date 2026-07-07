# M5 issue-report-builder-step primitive

Status: implemented (B106, task M05-902)

This is the second `implement_` lane that narrows the frozen
[M5 support-intake / escalation component matrix](./m5_support_intake_escalation_component_matrix.md)
into one reusable primitive: the **issue-report builder step**. It closes the
gap between the deeper Project Doctor finding, crash-forensics, repair-
transaction, evidence-chronology, and support-bundle redaction systems and the
reusable intake component a user actually assembles a case with — so a support
packet's scope and omissions are made explicit *before* anything is shared,
instead of collapsing into one opaque "report draft" blob.

Truth source (checked in):

- Schema: `schemas/ui/m5-support-issue-report-builder-step.schema.json`
- Support export: `artifacts/release/m5-support-issue-report-builder-step-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-support-issue-report-builder-step-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-support-issue-report-builder-step-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-support-issue-report-builder-step-primitive/`

The single mint-from-truth path is the headless emitter
`aureline_support_issue_report_builder_step_primitive`; the in-code seed
builders, the checked support export, and the fixtures never drift.

## What the primitive implements

The matrix names the issue-report builder step as one governed family and
freezes its controlled vocabulary (builder step kinds, evidence classes, and
redaction states, plus the shared surface families, deployment lines, consumer
surfaces, accessibility routes, qualification classes, and downgrade triggers).
The metadata-only / environment-adjacent / code-adjacent / high-risk data-risk
vocabulary (`schemas/support/data_risk_class.schema.json`) is reused verbatim —
the same one Doctor and support-bundle exports use — so included and excluded
classes never get a parallel sensitivity grammar. This lane implements that
contract as one resolver so a user can tell, from the builder step alone, which
evidence classes will leave the local boundary and which stay excluded, at what
data-risk class, under which redaction posture, without ever losing a same-weight
local-only preview.

### `resolve_issue_report_builder_step`

Takes one builder step's kind, its human-readable summary, its ordered
reproduction steps, the selected and excluded evidence classes, the redaction
posture, a share-requested signal, and a stable step identity. Derives the
**step posture** in a fixed blocking-first order:

1. `share_blocked` — the export is blocked by policy or unavailability
   (redaction `export_blocked`); nothing crosses, only the local-only preview
   remains.
2. `no_evidence_selected` — no evidence class is selected yet; nothing will
   cross the local boundary.
3. `redaction_review_required` — code-adjacent or high-risk evidence is selected
   under a `full_metadata` posture; redaction must be reviewed before anything
   crosses.
4. `local_only_preview` — the draft is being previewed locally only; the
   selected evidence stays on the device until a share is requested.
5. `ready_to_share` — the selected evidence classes are ready to cross the local
   boundary under the chosen redaction posture.

Each decided evidence class gets a **boundary disposition** naming its data-risk
class, whether it is selected, and whether it will cross the local boundary at
the resolved posture. The summary, reproduction steps, and selected / excluded
evidence are carried explicitly and never collapsed into one blob. The step
always offers `reveal_evidence_boundary`, `preview_local_only` (same-weight,
never dropped), `edit_evidence_selection`, and `export_step`; offers
`review_redaction` whenever the selection carries sensitive evidence or a review
is required; and offers `share_report` only when the step is ready to share.

## Acceptance criteria coverage

- **A user can tell exactly which evidence classes will leave the local boundary
  and which remain excluded** — `evidence_dispositions` carries one entry per
  decided class with its `data_class` and `crosses_local_boundary`, and
  `crossing_classes` lists exactly what leaves; excluded classes are always
  `selected = false`, `crosses_local_boundary = false`. Proven by
  `validate_boundary_coverage`.
- **Repro steps and selected evidence survive reopen/export without being
  collapsed into one opaque "report draft" blob** — the resolver preserves
  `summary`, `repro_steps`, `selected_evidence`, and `excluded_evidence`
  verbatim; `M5IssueReportBuilderStepResolutionCase::preserves_report` and
  `validate_report_preservation` enforce it, and the `collapses_report_into_blob`
  row invariant must be `false`.

## Consumers

Five claimed support-intake consumers render the shared step: the support-center
builder, the recovery-center builder, the Doctor handoff builder, the
headless/CLI builder, and the support-packet export. The same evidence /
data-risk / redaction / boundary vocabulary works identically across desktop,
headless/export, and support-packet consumers.
