# M5 Workspace-Trust and Guided-Repair Component Contract

This document is the design/QA/release-facing companion to the frozen M5 workspace-trust and
guided-repair component matrix. The authoritative gate is the Rust validator in
[`crates/aureline-shell`](../../crates/aureline-shell); the schemas under
[`schemas/ui/`](../../schemas/ui) document the shape; the checked-in support export under
[`artifacts/release/m5-workspace-trust-repair-proof/`](../../artifacts/release/m5-workspace-trust-repair-proof)
is the single mint-from-truth artifact.

## Why this matrix exists

The shell already hardens workspace trust, restricted mode, capability sheets, Project Doctor,
guided repair, repair transactions, and runtime-boundary repair cards. But the reusable banners,
grids, sheets, strips, and receipt rows that carry grant lineage, narrowed capability state, per-root
trust, preview / checkpoint / reversal class, and repair outcomes were still lighter than the rest of
the M5 component catalog. This matrix freezes that component family so trust lineage, narrowed
capabilities, repair preview, and reversal truth stop drifting across M5 surfaces. It does **not**
re-architect trust evaluation, entitlement issuance, or Doctor backend logic — it is the shared
component contract layered on top of already-claimed M5 trust and repair systems.

## The eight governed component families

| Component | Names |
| --- | --- |
| `workspace_trust_banner` | Whether the workspace is trusted, restricted, or mixed-root, who granted the trust, and what capability is narrowed. |
| `trust_fact_grid` | Grant source and policy epoch, trusted object and root scope, narrowed capability, and per-root trust together in one place. |
| `trust_elevation_sheet` | Exactly what a trust elevation grants, its grant source, and the scope it changes. |
| `restricted_capability_row` | Exactly which capability is narrowed (reduced mode, blocked task, blocked execution, blocked extension) and why. |
| `root_trust_strip` | Per-root trust so mixed-root trust never collapses into one uniform badge. |
| `repair_transaction_preview_card` | Repair candidate ids, checkpoint availability, and reversal class before anything is applied. |
| `rollback_class_strip` | The reversal class (exact / compensate / regenerate / manual follow-up / audit-only) and checkpoint availability. |
| `repair_result_receipt_row` | The applied outcome (including partial success) and any manual follow-up. |

## The one controlled disposition vocabulary

Every consumer binds to a single controlled trust/repair-disposition vocabulary and never invents a
parallel word:

`trusted`, `restricted`, `mixed_root`, `policy_blocked`, `reduced_mode`, `preview_ready`,
`checkpoint_missing`, `exact_reversal`, `compensate`, `regenerate`, `manual_follow_up`, `audit_only`.

## Family-specific controlled vocabularies

- **Grant source** — `user_explicit`, `inherited_parent`, `policy_managed`, `workspace_config`, `first_party_default`, `grant_source_unknown`.
- **Trust scope** — `trusted_workspace`, `trusted_root`, `restricted_workspace`, `mixed_root`, `policy_blocked`, `scope_unknown`.
- **Narrowed capability** — `full_capability`, `reduced_mode`, `task_blocked`, `execution_blocked`, `extension_blocked`, `capability_unknown`.
- **Per-root trust** — `root_trusted`, `root_restricted`, `root_inherited`, `root_policy_blocked`, `root_mixed_children`, `root_unknown`.
- **Reversal class** — `exact_reversal`, `compensating_reversal`, `regenerate_reversal`, `manual_follow_up`, `audit_only`, `reversal_unknown`.
- **Checkpoint state** — `checkpoint_available`, `checkpoint_partial`, `checkpoint_missing`, `checkpoint_expired`, `checkpoint_external`, `checkpoint_unknown`.
- **Repair outcome** — `repair_applied_exact`, `repair_compensated`, `repair_regenerated`, `repair_partial_success`, `repair_manual_required`, `repair_failed`.
- **Preview state** — `preview_ready`, `preview_incomplete`, `preview_blocked`, `candidate_identified`, `candidate_ambiguous`, `preview_unknown`.

## Hard invariants (all `false` on every row)

1. `implies_blanket_trust_across_roots_or_routes` — a trust surface never implies blanket approval across roots, profiles, or routes.
2. `hides_checkpoint_absence_or_reversal_limits` — a repair preview never hides checkpoint absence or reversal limits.
3. `collapses_reversal_outcomes_into_generic_success` — exact / compensate / regenerate / manual / audit-only outcomes never collapse into a single generic success.
4. `presents_partial_success_as_complete` — a partial success is never shown as a complete success.

## Non-visual / accessibility & export requirements

Every component must offer `keyboard_focusable`, `screen_reader_announced`, `non_hover_reachable`,
`pointer_optional`, `high_contrast_safe`, and `support_exportable` routes. No trust or repair truth
is hover-only, pointer-only, menu-only, or visually encoded alone, and every component appears in the
support / export packet.

## Consumers

Workspace-trust UI, settings, Project Doctor, safe mode, extensions, remote/workspace, AI context,
and support export all point at this one canonical component contract instead of rewording trust or
repair state locally. Future implementation rows inherit the field/state baseline frozen here with no
open ambiguity about trust lineage or reversal labeling.

## Regenerating the artifacts

```text
cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- support-export
cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- report
cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- csv
cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- fixture-trust-elevation-sheet-beta-narrowed
cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- fixture-repair-transaction-preview-card-preview-narrowed
cargo run -p aureline-shell --example dump_m5_workspace_trust_repair_component_matrix -- validate
```
