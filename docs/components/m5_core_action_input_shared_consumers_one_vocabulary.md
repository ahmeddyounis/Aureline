# M5 Core Action / Input Shared-Consumer Contract — One Vocabulary Across Surfaces

This is the closing **consumer-adoption** lane for the eight reusable atomic action and
input controls frozen in
[`m5-core-action-input-component-matrix.schema.json`](../../schemas/ui/m5-core-action-input-component-matrix.schema.json)
and implemented across the button/icon-button, split-button/segmented-control,
text-field/search-field, and combobox/toggle-control lanes. It proves — by fixtures, not
screenshots — that one reusable control grammar survives across the first heterogeneous M5
consumers rather than living in a component demo only.

The shared controls are **buttons, icon buttons, split buttons, text fields, search
fields, comboboxes, checkbox/radio/switch toggle controls, and segmented controls**. Each
is bound to the concrete **settings, request/data, package/install, provider/account,
template/starter, admin/policy, repair, and start-center entry** consumers that render it.

## Honesty axes

The lane enforces three axes, mirroring the batch acceptance criteria.

1. **Reuse.** Each of the eight shared controls must be adopted by at least two distinct
   consumers, so a control is proven to be shared product infrastructure rather than a
   one-surface feature-local fork (`control_reuse_unproven`).
2. **One vocabulary / no drift.** For a given control object every consumer surface must
   present identical `state_facets` — the same `state_word` (a frozen
   [`M5CoreControlDisposition`] token), the same `command_binding_word`, the same
   `value_source_word`, the same `validation_word`, and the same `lock_policy_word`. A
   surface may narrow *how much* it shows across desktop, compact, remote, and exported
   representations, but it may never reword the underlying vocabulary per surface
   (`vocabulary_drift_across_surfaces`, `state_word_outside_vocabulary`).
3. **Map back to one family.** Support and CLI/export consumers must point at the canonical
   per-control schema and the frozen matrix schema by id, so an exported packet can always
   map a settings / request / provider / admin / repair / entry control back to one shared
   contract family (`support_export_reference_missing`).

## Narrowing is disclosed, never hidden

A `compact_narrowed`, `remote_projected`, or `exported_redacted` representation carries an
explicit `narrow_note` naming the reason, the preserved vocabulary, and the next action. A
remote representation additionally names its remote source; an exported representation
names its export-safe-detail boundary. A `desktop_full` binding must carry no narrow note.

## Guardrails

Every binding must keep the six B134 hard invariants false: placeholder text never
replaces the label; a loading control never relabels the action or loses attribution; an
icon-only destructive action is never unlabeled; a switch is never blurred with a deferred
checkbox; a split button never defaults to a riskier alternate; and locked or degraded
semantics are never hidden behind generic disabled chrome.

## Artifacts

- Schema: [`schemas/ui/m5-core-action-input-shared-consumers.schema.json`](../../schemas/ui/m5-core-action-input-shared-consumers.schema.json)
- Support export: [`artifacts/release/m5-core-action-input-shared-consumers-proof/support_export.json`](../../artifacts/release/m5-core-action-input-shared-consumers-proof/support_export.json)
- Matrix CSV / summary: same proof directory
- Narrowed fixtures: [`fixtures/ui/m5-core-action-input-shared-consumers/`](../../fixtures/ui/m5-core-action-input-shared-consumers/)

The support export, CSV, summary, and fixtures are minted only from
`cargo run -p aureline-ui --example dump_m5_core_action_input_shared_consumers`. The packet
references upstream control contracts by id; raw secret values, credentials, and private
endpoints stay outside the support boundary.
