# M5 governance-dashboard component fixtures

These fixtures are the checked-in, export-safe evidence for the frozen M5
governance-dashboard component matrix defined in
`crates/aureline-release/src/freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`.

## Narrowed matrix packets

Full matrix packets with one component narrowed below Stable. Each validates
against
[`schemas/ui/m5-governance-dashboard-component-matrix.schema.json`](../../../schemas/ui/m5-governance-dashboard-component-matrix.schema.json)
and every component family stays visible — narrowing never hides a component.

- `service_ownership_card_beta_narrowed.json` — the service-ownership card is held
  at Beta; an unresolved or backup-missing owner still never reads as covered.
- `release_gate_banner_preview_narrowed.json` — the release-gate banner is narrowed
  to Preview; a held or blocked gate still never reads as a generic go.

## Per-component instances

Single-component instances that validate against the per-component schemas. Each
demonstrates a guardrail state that must never read as a clean pass:

- `instance_fitness_dashboard_tile.json` — `evidence_stale` on a sampled corpus.
- `instance_governance_report_row.json` — `blocked` train-scope lane.
- `instance_waiver_expiry_queue_item.json` — `expired_waiver`.
- `instance_release_gate_banner.json` — `blocked_by_owner_or_forum` with a reason.
- `instance_mitigation_note_card.json` — `partially_mitigated`, jargon-free text.
- `instance_service_ownership_card.json` — `owner_unresolved`.
- `instance_on_call_strip.json` — `on_call_gap` with a named escalation route.
- `instance_decision_right_card.json` — `forum_unresolved` (no authorized forum).
- `instance_milestone_dashboard_row.json` — `exit_gate_waived`.

## Regeneration

The narrowed matrix packets, the support export, the matrix CSV, and the summary
are all minted from the seed builders. Regenerate them with:

```sh
GEN_GOVERNANCE_DASHBOARD_COMPONENT_MATRIX_ARTIFACTS=1 cargo test -p aureline-release \
  --lib freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix::tests::generate_artifacts \
  -- --exact --ignored
```
