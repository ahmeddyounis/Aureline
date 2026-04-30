# Service-ownership card and temporary-handoff fixtures

Worked fixtures for the service-ownership card, escalation route,
and temporary-handoff expiry contract frozen in
[`/docs/governance/service_ownership_card_contract.md`](../../../docs/governance/service_ownership_card_contract.md)
and its boundary schemas
[`/schemas/governance/service_ownership_card.schema.json`](../../../schemas/governance/service_ownership_card.schema.json)
and
[`/schemas/governance/temporary_handoff_record.schema.json`](../../../schemas/governance/temporary_handoff_record.schema.json).

Each fixture renders one concrete card or handoff record: the live
state, the accountability status, the support posture, the
escalation route, the ownership-freshness envelope, the linked
evidence refs, the typed degraded-state label, and the role-export
ceilings per audience. The corpus exists so consuming surfaces
(governance dashboard, milestone scorecard, weekly governance
review, release-evidence shiproom packet, support bundle, public
claim manifest, compatibility-window report, enterprise / managed-
tenant review) can be validated against one shared set of records
rather than inventing local fixtures.

## Index

| Fixture | Status | Surface | Notes |
| --- | --- | --- | --- |
| `active_owner_renderer_lane.yaml` | `active_owner_with_backup_waiver` | `subsystem` (renderer) | Active owner with backup coverage held by the standing single-maintainer-backup waiver; ownership freshness inside the declared window. |
| `temporary_handoff_buffer_lane.yaml` | `temporary_handoff_active` | `subsystem` (buffer / editor core) | Active temporary handoff inside its declared window; closure routes to `return_to_primary_dri`. |
| `temporary_handoff_buffer_lane_record.yaml` | `handoff_active_inside_window` | temporary_handoff_record | Companion handoff record for the buffer-lane card. |
| `expired_handoff_release_evidence_lane.yaml` | `temporary_handoff_expired` | `packet_family` (release evidence) | Lapsed handoff; degraded with `degraded_handoff_expired`; closure routes through the release council. |
| `expired_handoff_release_evidence_record.yaml` | `handoff_expired_past_due` | temporary_handoff_record | Companion handoff record for the lapsed release-evidence card. |
| `escalated_blocked_security_egress.yaml` | `blocked_escalated` | `security_sensitive_egress_class` (signed-pack publication) | Contested ownership pending a signing-chain rotation decision; refusal support class; escalation routes through shiproom. |

## Intended usage

- **Accountability-status grammar conformance.** Every card fixture
  renders one of the seven accountability-status tokens; every
  handoff fixture renders one of the seven temporary-handoff status
  tokens. A surface that renders a different token, or a generic
  "needs review" chip, is non-conforming.
- **Degraded-state conformance.** Every card whose accountability is
  not `active_owner_no_handoff` or `active_owner_with_backup_waiver`
  with `fresh` ownership renders one of the four degraded labels.
  A surface that renders a lapsed handoff or an unresolved owner as
  a clean active-owner card is non-conforming.
- **Reuse-class conformance.** Every card whose surface identity is
  `runtime_host_class`, `public_truth_area`, `supportability_family`,
  `packet_family`, `review_family`, `protected_metric_or_fitness_function`,
  `compatibility_or_certified_archetype_claim`, or
  `security_sensitive_egress_class` MUST render
  `reuse_class = required_named_accountable_ownership`.
- **Role-export conformance.** Every card MUST stay inside the
  audience-specific role-export ceilings: claim manifest at
  `redacted_role_class_only_no_role_id`, support and enterprise
  review at most `role_id_with_named_backup_or_waiver` (or
  `role_id_with_temporary_handoff_summary` when a handoff is in
  force), release packet at most `role_id_with_decision_forum` (or
  `role_id_with_temporary_handoff_summary`).
- **Export-parity conformance.** Every consuming surface MUST render
  the same `service_ownership_card_id`, `surface_identity`,
  `accountable_ownership`, `support_posture`, `escalation_route`,
  `freshness`, `degraded_state_label_class`, and `reuse_class`. A
  surface that drops any required field is non-conforming.

## Acceptance coverage

The four required acceptance cases from
[`.plans/M00-477.md`](../../../.plans/M00-477.md) are covered as
follows:

- **Active owner** — `active_owner_renderer_lane.yaml` (renderer
  subsystem on a protected path, backup coverage via standing
  waiver, no handoff, fresh ownership).
- **Temporary handoff** — `temporary_handoff_buffer_lane.yaml` plus
  the companion `temporary_handoff_buffer_lane_record.yaml`
  (buffer / editor-core subsystem, primary DRI on declared leave,
  handoff inside its declared window).
- **Expired handoff** — `expired_handoff_release_evidence_lane.yaml`
  plus the companion `expired_handoff_release_evidence_record.yaml`
  (release-evidence packet family, handoff lapsed without renewal,
  degraded with `degraded_handoff_expired`, closure routes through
  the release council).
- **Escalated blocked surface** —
  `escalated_blocked_security_egress.yaml` (signed-pack publication
  egress class, contested ownership, refusal support class,
  escalation routes through shiproom).
