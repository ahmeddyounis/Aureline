# Managed account-exit fixture cases

Worked YAML cases for the managed account-exit packet contract. Each
fixture validates against
[`/schemas/managed/account_exit_packet.schema.json`](../../../schemas/managed/account_exit_packet.schema.json)
under the JSON Schema Draft 2020-12 validator and exercises a subset
of the rules in
[`/docs/managed/account_seat_plan_and_exit_contract.md`](../../../docs/managed/account_seat_plan_and_exit_contract.md).

| Fixture | Account state | Posture origin | What it exercises |
|---|---|---|---|
| `individual_seat_expiry.yaml` | `expired` | `account` | Plan term ended past grace; seat deprovisioned; access-end window open; BYOK local alternative offered. |
| `org_suspension.yaml` | `suspended` | `org` | Org-wide billing lock; per-seat managed actions paused; local-core remains usable. |
| `grace_period_warning.yaml` | `grace` | `plan` | Grace warning window open; scheduled plan downgrade disclosed; export-before-close pathway named. |
| `export_before_suspend.yaml` | `grace` | `plan` | Grace final-warning window; some artifacts already exported with refs; others still exportable. |
| `self_hosted_alternative_recommendation.yaml` | `offboarding_in_progress` | `account` | User-initiated leave; typed self-hosted org / BYOK local AI / signed file-based policy / offline-capture continuations. |

Fixtures are product-contract examples, not implementation traces.
They do not describe a billing engine, an entitlement service, or a
managed-account control plane.
