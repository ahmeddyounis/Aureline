# Post-import decision sheet

This document freezes the post-import decision sheet shown after an import apply
commits durable state and post-import validation outcomes exist. The decision
sheet exists to keep rollback, keep-state, bundle adoption, unsupported-item
review, and export actions **distinct** with explicit consequences and required
linkage fields.

This is a contract document. It does not implement UI.

Companion artifacts:

- [`/docs/migration/post_import_validation_contract.md`](./post_import_validation_contract.md)
  — runner contract that produces the validation outcomes this sheet reacts to.
- [`/schemas/migration/migration_report.schema.json`](../../schemas/migration/migration_report.schema.json)
  — machine-readable report that MUST include next-step state and available
  actions.
- [`/schemas/workspace/entry_and_restore_result.schema.json`](../../schemas/workspace/entry_and_restore_result.schema.json)
  — shared `next_step_decision_hook` vocabulary used by entry/migration surfaces.

## Actions (required)

The sheet MUST keep these actions distinct:

| Action | Hook id (when applicable) | What it does | Required linkage | Consequences |
|---|---|---|---|---|
| Roll back import | `roll_back_import` | Restore the pre-apply checkpoint. | rollback checkpoint ref(s) + restore linkage | Imported state is reverted; the migration report and validation records remain preserved for support/export. |
| Keep imported state | `keep_imported_state` | Explicitly accept the imported state as the current baseline. | migration report ref + outcome packet ref | No rollback occurs; checkpoint retention follows policy (may expire later) but the report remains addressable. |
| Adopt recommended bundle | `adopt_recommended_bundle` | Apply a recommended workflow bundle **as a separate reviewed mutation**. | `recommended_bundle_ref` + a new bundle-adoption checkpoint | Bundle adoption MUST NOT overwrite the original import provenance; it creates a new checkpoint and preserves the original report refs. |
| Review unsupported items | `review_unsupported_items` | Inspect unsupported/bridge-required/manual-review items without mutating state. | outcome packet ref + docs/help/support refs | No mutation; the surface provides follow-up links and export paths without guessing at fixes. |
| Export report | (report/export action) | Export the machine-readable migration report and its referenced validation runs. | `migration_report_id` + export destination handle | Exports MUST apply redaction policy and MUST include exact build identity refs and schema/version fields. |

## Rules

1. **Rollback requires linkage.** If rollback is offered, the decision sheet MUST
   cite a rollback checkpoint ref; “rollback” without the checkpoint is
   non-conforming.
2. **Keep-state is explicit.** A migration may not silently “keep imported state”
   without emitting a report record whose next-step state makes the keep-state
   posture explicit.
3. **Bundle adoption is not rollback.** Bundle adoption is a new reviewed change
   set with its own checkpoint. It MUST preserve:
   - the original migration report ref,
   - the original post-import validation run refs, and
   - a pointer to the original import checkpoint lineage.
4. **Unsupported-item review is non-mutating.** “Review unsupported items” is an
   inspection action; it does not re-run import apply or silently install
   packages.
5. **Export is explicit.** Exporting a report MUST be presented as a distinct
   action; it cannot be hidden behind “Done” or “Continue” buttons.

## Machine-readable mapping

The canonical machine-readable carrier for this decision sheet is
`migration_report_record.next_step` in
[`schemas/migration/migration_report.schema.json`](../../schemas/migration/migration_report.schema.json).

At minimum, the report MUST record:

- `next_step.state` (explicit next-step state; never implied by UI copy),
- `next_step.available_actions` (distinct actions),
- `next_step.rollback_checkpoint_ref` when rollback is offered, and
- `next_step.recommended_bundle_ref` when bundle adoption is offered.

