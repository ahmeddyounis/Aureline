# M5 repair-result-receipt-row and checkpoint-lineage-disclosure controls

This is the guided-repair result lane over the frozen
[M5 workspace-trust / guided-repair component matrix](./m5_workspace_trust_repair_components_contract.md).
It turns the matrix's `repair_result_receipt_row` component — and the checkpoint-lineage disclosure
that traces it — into two resolvers that produce export-safe, honest projections, so **what actually
happened after a repair is preserved** instead of collapsing into a generic success or failure.

- Rust: `crates/aureline-shell/src/implement_the_m5_repair_result_receipt_row_and_checkpoint_lineage_disclosure_applied_versus_skipped_scope_partial_success_compensation_manual_follow_up_and_support_export_linkage_primitive/`
- Schema: `schemas/ui/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls.schema.json`
- Component schema: `schemas/ui/m5-repair-result-receipt-row.schema.json`
- Proof packet: `artifacts/release/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls-proof/`
- Fixtures: `fixtures/ui/m5-repair-result-receipt-row-checkpoint-lineage-disclosure-controls/`

## Goal

Preserve what actually happened after a repair instead of collapsing outcomes into a generic success
or failure. The resolvers reuse the frozen matrix vocabulary directly — the single controlled trust /
repair disposition, the `M5RepairOutcomeClass` repair-outcome, the `M5RepairReversalClass` reversal,
and the `M5RepairCheckpointState` checkpoint vocabularies — so every claimed M5 guided-repair surface
exposes the same receipt and lineage grammar instead of forking its own outcome copy.

## Resolvers

### `resolve_repair_result_receipt_row`

Refuses to read as a clean, attributable receipt unless it:

- names its stable **repair id** and at least one **linked finding id**;
- names its **applied scope** (for any non-failure outcome) alongside the **skipped scope** it left
  behind;
- resolves the **checkpoint state** and names a **linked checkpoint ref** when a checkpoint is present;
- resolves the **reversal class**;
- names the **compensation / manual follow-up state** whenever the outcome (`repair_compensated`,
  `repair_partial_success`, `repair_manual_required`) still needs it;
- keeps a **command-backed support-export path** reachable.

Otherwise it degrades to a typed `M5RepairResultReceiptRowDegradeReason`. It never collapses distinct
outcomes into a generic success (`outcome_collapsed_into_generic_success`) and never presents a
partial success as complete (`partial_success_shown_as_complete`).

### `resolve_checkpoint_lineage_disclosure`

Traces a repair end to end. It refuses to read as a traceable lineage unless every stage — the
**finding**, the **preview** ref, the resolved **checkpoint**, the **apply** ref, and the **result**
(canonical receipt) ref — is named, and it never collapses the four stages into a single opaque
status (`stages_collapsed_into_single_status`). A clean lineage lets users and support trace a repair
from finding to preview to apply to result and cite one canonical receipt object.

## Hard invariants (MUST be false on every clean row)

- `collapses_outcomes_into_generic_success`
- `hides_partial_success_or_follow_up`
- `severs_receipt_from_checkpoint_lineage`
- `requires_feature_local_translation_for_support_export`

## Acceptance criteria, proven by examples

- **Repair outcomes stay attributable and exportable across Doctor, diagnostics, and support
  surfaces, and partial success plus manual follow-up are first-class outcomes.** Clean receipts cover
  an exact success, a partial success, and a manual-required outcome so each keeps its own honest
  word; at least one receipt degrades to an outcome collapsed into a generic success and one to a
  partial success shown as complete; no clean receipt is dishonest; and every clean receipt names its
  repair id, at least one finding, applied scope (for any non-failure outcome), and keeps a
  command-backed support-export path.
- **Support packets can cite one canonical receipt / lineage object for guided repairs.** Clean
  lineages are complete (finding to preview to apply to result) and cover a partial success; at least
  one lineage degrades to a collapsed single status and one to a missing stage; no clean lineage
  severs; and every clean lineage links at least one finding and names a canonical result ref.

The Rust validator in `crates/aureline-shell` is the authoritative gate. Raw secret values and
private endpoints never cross this boundary.
