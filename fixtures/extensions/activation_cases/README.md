# Extension runtime-budget activation-evidence fixtures

Worked JSON fixtures exercising the activation-evidence packet shape
named in
[`docs/extensions/runtime_budget_packet.md`](../../../docs/extensions/runtime_budget_packet.md).
Every packet cites axis ids from
[`artifacts/extensions/runtime_budget_rows.yaml`](../../../artifacts/extensions/runtime_budget_rows.yaml)
and (where a quarantine rule fires) trigger rule ids from
[`artifacts/extensions/quarantine_rules.yaml`](../../../artifacts/extensions/quarantine_rules.yaml).

Raw wasm-module bytes, raw core dumps, raw log bodies, raw network
payload bodies, and raw memory snapshots MUST NOT appear in any
fixture; every such field is an opaque ref.

## Fixtures

| File                                                              | Observation kind           | Demonstrates                                                                                                          |
|-------------------------------------------------------------------|----------------------------|-----------------------------------------------------------------------------------------------------------------------|
| `host_startup_under_budget.json`                                  | `host_startup`             | Discovery axis nominal; negotiation packet cited; no quarantine trip.                                                 |
| `cold_activation_user_invoked_ready.json`                         | `cold_activation`          | User-invoked activation under medium_bounded budget; cold_activation counter nominal; resource governor nominal.       |
| `warm_activation_warming_after_prior_cold.json`                   | `warm_activation`          | Warm activation cites prior cold-activation packet id; cold counter not re-measured.                                  |
| `idle_polling_throttled_efficiency_aware.json`                    | `idle_poll_sample`         | Idle-polling sustained soft breach trips `discovery_sustained_soft_breach_throttles_background` under efficiency_aware.|
| `memory_hard_cap_disables_session.json`                           | `idle_poll_sample`         | Memory hard-cap breach fires `memory_hard_cap_breach_disables_session` with `disable_until_next_session` response.     |
| `egress_hard_breach_disables_until_reenable.json`                 | `idle_poll_sample`         | Egress hard breach fires `egress_sustained_hard_breach_disables_until_reenable` with maintainer-coverage posture.      |
| `crash_loop_window_quarantines.json`                              | `host_shutdown_crash`      | Crash-loop window breach fires `crash_loop_window_breach_trips_quarantine` with `removed_from_ranking` visibility.     |
| `host_shutdown_clean_release_counters.json`                       | `host_shutdown_clean`      | Clean shutdown records final memory and cumulative egress counters without tripping any rule.                         |
