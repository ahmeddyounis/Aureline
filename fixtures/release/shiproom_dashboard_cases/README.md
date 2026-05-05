# Shiproom dashboard cases

These fixtures exercise the dashboard panel contract from
[`/docs/release/shiproom_dashboard_contract.md`](../../../docs/release/shiproom_dashboard_contract.md).
Each fixture is a worked panel record (or set of panel records) plus
the alert threshold firings and promotion checklist outcomes that
follow from the panel state. The fixtures are structural examples of
the daily decision surface; they are not live release calls.

Schema companions:

- [`/schemas/release/shiproom_panel.schema.json`](../../../schemas/release/shiproom_panel.schema.json)

Register companions:

- [`/artifacts/release/shiproom_alert_thresholds.yaml`](../../../artifacts/release/shiproom_alert_thresholds.yaml)
- [`/artifacts/release/promotion_checklist.yaml`](../../../artifacts/release/promotion_checklist.yaml)

Cases:

| Fixture | Coverage |
|---|---|
| `beta_widening_green.yaml` | Beta widening proceeds with every required panel green, freshness within SLO, no alert thresholds firing, every beta checklist row at `yes`. |
| `stable_widening_held_by_stale_evidence.yaml` | Stable widening held: `build_identity` panel reports `panel_red_stale_required_source`, `alert.stale_packet.required_release_evidence` fires, `default_action: refresh_evidence_packet`, `checklist.beta.evidence_freshness_within_slo` returns `no`. |
| `milestone_close_blocked_by_repeat_freeze_exception.yaml` | Milestone close blocked: `readiness_scorecard` panel reports `panel_red_repeated_freeze_exception`, `alert.repeated_freeze_exception.same_protected_path` fires, `default_action: rebaseline_decision`, `checklist.milestone_close.no_repeat_freeze_exception` returns `no`. |
| `panel_degraded_by_missing_source.yaml` | `support_center_readiness` panel reports `panel_red_missing_required_source` because a required `support_packet_index` ref cannot be resolved; the dashboard does not render the panel green by omission. |
