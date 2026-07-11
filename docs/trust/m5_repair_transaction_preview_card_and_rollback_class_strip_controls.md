# M5 repair-transaction-preview-card and rollback-class-strip controls

The guided-repair implement lane over the frozen [M5 workspace-trust / guided-repair component matrix](m5_workspace_trust_repair_components_contract.md). It turns the two guided-repair-facing components — the **repair-transaction preview card** and the **rollback-class strip** — into resolvers that produce export-safe, honest projections, so a repair preview reads as a typed transaction review rather than a folklore "Fix it" shortcut. A user never has to infer what a repair will mutate, whether a restore checkpoint exists, or how reversal actually works before applying it.

- Controls packet schema: `schemas/ui/m5-repair-transaction-preview-card-rollback-class-strip-controls.schema.json`
- Support export: `artifacts/release/m5-repair-transaction-preview-card-rollback-class-strip-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-repair-transaction-preview-card-rollback-class-strip-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-repair-transaction-preview-card-rollback-class-strip-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-repair-transaction-preview-card-rollback-class-strip-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_repair_transaction_preview_card_and_rollback_class_strip_...`)

## Reused, not re-minted

The lane binds directly to the frozen workspace-trust / guided-repair object model so every claimed M5 guided-repair surface exposes one transaction-preview grammar and one reversal vocabulary instead of forking its own "Fix it" copy:

- **Repair disposition** reuses the single controlled `M5WorkspaceTrustRepairDisposition` vocabulary from the matrix (trusted, restricted, mixed_root, policy_blocked, reduced_mode, preview_ready, checkpoint_missing, exact_reversal, compensate, regenerate, manual_follow_up, audit_only).
- **Reversal class** reuses `M5RepairReversalClass` (exact_reversal, compensating_reversal, regenerate_reversal, manual_follow_up, audit_only, reversal_unknown).
- **Checkpoint state** reuses `M5RepairCheckpointState`, and **preview state** reuses `M5RepairPreviewState`.
- **Target class** (`M5RepairTargetClass`: local_workspace, remote_host, managed_workspace, mixed_target, external_target, target_unknown) is minted by this lane so a preview never hides whether a repair touches the local workspace, a remote host, or a managed workspace.

## Repair-transaction preview card resolver

`resolve_repair_transaction_preview_card` degrades first rather than ever letting a folklore "Fix it" card read as a clean, ready-to-apply transaction:

| Condition | Degrade reason |
| --- | --- |
| Stable repair id unstated | `repair_id_unstated` |
| No linked finding id named | `linked_findings_unstated` |
| Prerequisites not stated | `prerequisites_unstated` |
| Checkpoint state cannot be resolved | `checkpoint_state_unresolved` |
| Checkpoint absent but absence hidden before apply | `checkpoint_absence_hidden` |
| Impact scope unstated | `impact_scope_unstated` |
| Local / remote / managed target class cannot be resolved | `target_class_unresolved` |
| Preview incomplete yet reads as ready to apply | `preview_not_ready` |
| Target class collapsed into a generic target | `target_collapsed_into_generic` |
| No command-backed review path | `review_path_missing` |
| Proof stale | `proof_stale` |

A clean card names its repair id, at least one linked finding id, its prerequisites, its checkpoint state, its impact scope, and a resolved local / remote / managed target class, and reports `transaction_reviewable = true`. **Checkpoint presence or absence is visible before apply** — a card whose checkpoint is missing or expired must disclose that absence or it degrades.

## Rollback-class strip resolver

`resolve_rollback_class_strip` keeps reversal truth honest so a repair UI never implies reversibility when only compensation or manual follow-up is available:

| Condition | Degrade reason |
| --- | --- |
| Stable repair id unstated | `repair_id_unstated` |
| Reversal class cannot be resolved | `reversal_class_unresolved` |
| Checkpoint state cannot be resolved | `checkpoint_state_unresolved` |
| Checkpoint absent but absence hidden before apply | `checkpoint_absence_hidden` |
| Reads as reversible without an exact or regenerate reversal | `reversibility_overclaimed` |
| Non-exact reversal leaves its limit undisclosed | `reversal_limit_hidden` |
| Distinct reversal classes collapsed into a generic undo | `collapsed_into_generic_undo` |
| No command-backed review path | `review_path_missing` |
| Proof stale | `proof_stale` |

Only an exact or regenerate reversal permits an honest "reversible" claim; compensating, manual-follow-up, and audit-only reversals never do. A non-exact reversal always discloses its limit.

## Acceptance criteria, proven by examples

- **Transaction-preview grammar** — clean cards cover the local, remote, and managed target classes and both a present and an absent (disclosed) checkpoint; at least one card degrades to `checkpoint_absence_hidden`; no clean card is dishonest; and every clean card names its repair id, at least one linked finding, prerequisites, and impact scope. Repair previews remain truthful about target class in desktop, CLI, and exported evidence.
- **Reversal truth** — clean strips cover an exact reversal and at least one non-reversible class (compensate / manual / audit-only); at least one strip degrades to `reversibility_overclaimed` and one to `reversal_limit_hidden`; no clean strip is dishonest; and every clean non-reversible strip discloses its limit and never claims reversibility.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- hides checkpoint absence or reversal limits;
- collapses distinct exact / compensate / regenerate / manual / audit-only reversal classes into a generic success;
- implies reversibility without an exact or regenerate reversal;
- hides the local / remote / managed target class or the impact scope.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
