# M5 Decision / Feedback Shared-Consumer Contract — One Vocabulary Across Surfaces

This is the closing **consumer-adoption** lane for the eight reusable decision / feedback
primitives frozen in
[`m5-decision-feedback-component-matrix.schema.json`](../../schemas/ui/m5-decision-feedback-component-matrix.schema.json)
and implemented across the badge/popover, dialog/consequence, banner/empty-state, and
toast/loading-state lanes. It proves — by fixtures, not screenshots — that one reusable
decision / feedback grammar survives across the first heterogeneous M5 consumers rather
than living in a component demo only.

The shared primitives are **badges / chips / pills, popovers, dialogs / sheets, banners /
inline notices, toasts, empty states, loading states, and consequence blocks**. Each is
bound to the concrete **shell, help, entry, trust/repair, update/advisory,
provider/account, and export/support** consumers that render it.

## Honesty axes

The lane enforces three axes, mirroring the batch acceptance criteria.

1. **Reuse.** Each of the eight shared primitives must be adopted by at least two distinct
   consumers, so a primitive is proven to be shared product infrastructure rather than a
   one-surface feature-local fork (`primitive_reuse_unproven`).
2. **One vocabulary / no drift.** For a given primitive object every consumer surface must
   present identical `state_facets` — the same `disposition_word` (a frozen
   [`M5DecisionFeedbackDisposition`] token), the same `scope_word`, the same
   `severity_word`, the same `rationale_word`, the same `recovery_path_word`, and the same
   `durable_object_word`. A surface may narrow *how much* it shows across desktop, compact,
   remote, and exported representations, but it may never reword the underlying vocabulary
   per surface (`vocabulary_drift_across_surfaces`,
   `disposition_word_outside_vocabulary`).
3. **Map back to one family.** Support and CLI/export consumers must point at the canonical
   per-primitive schema and the frozen matrix schema by id, so an exported packet can always
   map a shell / help / review / settings / updates decision-feedback surface back to one
   shared contract family (`support_export_reference_missing`).

## Narrowing is disclosed, never hidden

A `compact_narrowed`, `remote_projected`, or `exported_redacted` representation carries an
explicit `narrow_note` naming the reason, the preserved vocabulary, and the next action. A
remote representation additionally names its remote source; an exported representation
names its export-safe-detail boundary. A `desktop_full` binding must carry no narrow note.

## Guardrails

Every binding must keep the six B135 hard invariants false: meaning never relies on color
alone; a popover never carries the only critical workflow instruction; a high-risk dialog
never uses generic Yes/No confirmation copy; durable or reviewable work is never
represented as toast-only truth; a useful pane is never blanked during loading; and a
full-screen spinner is never used where partial capability exists.

## Artifacts

- Schema: [`schemas/ui/m5-decision-feedback-shared-consumers.schema.json`](../../schemas/ui/m5-decision-feedback-shared-consumers.schema.json)
- Support export: [`artifacts/release/m5-decision-feedback-shared-consumers-proof/support_export.json`](../../artifacts/release/m5-decision-feedback-shared-consumers-proof/support_export.json)
- Matrix CSV / summary: same proof directory
- Narrowed fixtures: [`fixtures/ui/m5-decision-feedback-shared-consumers/`](../../fixtures/ui/m5-decision-feedback-shared-consumers/)

The support export, CSV, summary, and fixtures are minted only from
`cargo run -p aureline-ui --example dump_m5_decision_feedback_shared_consumers`. The packet
references upstream primitive contracts by id; raw secret values, credentials, and private
endpoints stay outside the support boundary.
