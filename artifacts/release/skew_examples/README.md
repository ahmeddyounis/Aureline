# Skew-smoke reviewer examples

These fixtures are reviewer-facing examples keyed to the seed cases in
[`fixtures/release/skew_cases/`](../../../fixtures/release/skew_cases/)
and the skew-smoke packet at
[`docs/release/skew_smoke_packet.md`](../../../docs/release/skew_smoke_packet.md).

Each example is shaped so five questions have a one-paragraph answer
compatibility reports, support exports, About/Help, and shiproom
reviewers MAY render verbatim:

1. **Which surface is the skew on?** — see `surface_class`.
2. **Which skew state applies?** — see `skew_state_class`.
3. **What does the user experience?** — see `outcome_label_class` and
   `explanation_fields.user_facing_summary`.
4. **Which compatibility row and support-packet family route this
   case?** — see `compatibility_row_ref`,
   `version_skew_register_ref`, and `support_packet_routing_classes`.
5. **What promotion decision does it justify?** — see
   `promotion_decision_class` and
   `explanation_fields.promotion_summary`.

## Index

| Example | Surface | Skew state | Outcome label | Promotion decision |
|---|---|---|---|---|
| [`side_by_side_stable_preview_coexist.yaml`](./side_by_side_stable_preview_coexist.yaml) | `side_by_side_install` | `compatible` | `side_by_side_coexist_ok` | `promote` |
| [`state_migration_old_to_new_additive.yaml`](./state_migration_old_to_new_additive.yaml) | `state_schema_migration` | `repairable` | `upgrade_applied_compatible` | `ship_narrowed_claim` |
| [`state_migration_new_to_old_blocked.yaml`](./state_migration_new_to_old_blocked.yaml) | `state_schema_migration` | `blocked` | `downgrade_blocked_newer_schema` | `no_go` |
| [`helper_agent_attach_skewed_client_degraded.yaml`](./helper_agent_attach_skewed_client_degraded.yaml) | `helper_agent_attach` | `degraded` | `attach_degraded_review_only` | `ship_narrowed_claim` |
| [`rollback_prior_channel_build_compatible.yaml`](./rollback_prior_channel_build_compatible.yaml) | `downgrade_upgrade_rollback` | `compatible` | `rollback_applied` | `promote` |
| [`helper_agent_attach_unknown_probe_required.yaml`](./helper_agent_attach_unknown_probe_required.yaml) | `helper_agent_attach` | `unknown_requires_probe` | `probe_required` | `pending_probe` |

Rows that widen beyond these reviewer-facing examples MUST land a
decision row in `artifacts/governance/decision_index.yaml` per the
packet's promotion rule.
