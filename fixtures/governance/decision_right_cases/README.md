# Decision-right card and release-truth summary fixtures

Worked fixtures for the decision-right card, forum-reason grammar,
and release-truth summary contract frozen in
[`/docs/governance/decision_right_and_release_truth_contract.md`](../../../docs/governance/decision_right_and_release_truth_contract.md)
and its boundary schemas
[`/schemas/governance/decision_right_card.schema.json`](../../../schemas/governance/decision_right_card.schema.json)
and
[`/schemas/governance/release_truth_summary.schema.json`](../../../schemas/governance/release_truth_summary.schema.json).

Each case ships as a paired card fixture and summary fixture: the
decision-right card names who can decide the next move and why the
forum is required; the release-truth summary names the live ship-
or-no-ship posture, the open blockers, the waived blockers (with
expiry), the stale-evidence count, the typed mitigations, and the
export-packet action. Every release-truth summary cites at least
one decision-right card through `linked_decision_right_card_refs`.

## Index

| Case | Card fixture | Summary fixture |
| --- | --- | --- |
| Pending security review | `pending_security_review_card.yaml` (`pending_review_nearing_deadline`, `security_sensitive_egress_class_change`, `security_trust_review`) | `pending_security_review_summary.yaml` (`hold_blockers_open`, one `security_egress_class_failure` blocker, `temporarily_blocked_pending_decision`, `hold_publish_pending_blocker`) |
| Waived blocker with expiry | `waived_blocker_with_expiry_card.yaml` (`satisfied_with_active_waiver`, `release_blocking_waiver`, `performance_council`) | `waived_blocker_with_expiry_summary.yaml` (`hold_waived_blockers_within_expiry`, one `protected_metric_failure` waived blocker, `nearing_expiry_within_thirty_days`, `waiver_holds_release_until_expiry`, `hold_publish_pending_waiver_renewal`) |
| Stale evidence holding ship | `stale_evidence_holding_ship_card.yaml` (`pending_review_within_window`, `protected_metric_threshold_change`, `benchmark_council`) | `stale_evidence_holding_ship_summary.yaml` (`hold_blockers_open`, one `evidence_stale_failure` blocker, `stale_evidence_count = 2`, `multiple_stale_evidence_rows_named`, `narrower_scope_until_concurrence`, `degraded_evidence_stale`) |
| Fully satisfied promotion path | `fully_satisfied_promotion_path_card.yaml` (`satisfied_concurrence_complete`, `stable_promotion_decision`, `release_council`) | `fully_satisfied_promotion_path_summary.yaml` (`ship_satisfied_no_blockers`, no blockers, no waived blockers, zero stale evidence, `no_mitigation_required_satisfied`, `publish_to_target_after_concurrence`) |

## Intended usage

- **Reason-for-review grammar conformance.** Every card fixture
  renders one of the closed reason-for-review classes; a surface
  that renders a free-text 'needs review' chip is non-conforming.
- **Required-forum grammar conformance.** Every card fixture
  resolves `required_forum_class` to a forum charter id and a
  non-null `decision_forum_ref`; a card that elides the forum and
  renders 'shiproom will figure it out' is non-conforming.
- **Decision-right state grammar conformance.** Every card fixture
  renders one of the eleven typed decision-right states and one of
  the seven typed degraded labels. A surface that renders a
  blocked card as a clean approved chip is non-conforming.
- **Release-truth posture grammar conformance.** Every summary
  fixture renders one of the seven ship-or-no-ship posture tokens
  and one of the eight degraded labels. A summary that renders a
  hold or no-ship posture as 'go' on a release packet is
  non-conforming.
- **Decision-ID linkage conformance.** Every summary's
  `linked_decision_right_card_refs` is non-empty and every blocker,
  waived blocker, and stale-evidence entry cites a
  `decision_right_card_ref` that appears in the summary's linkage
  array. A surface that drops the linkage is non-conforming.
- **Export-parity conformance.** Every consuming surface MUST
  render the same `decision_right_card_id`, `decision_subject`,
  `reason_for_review`, `required_forum`, `accountable_ownership`,
  `decision_state`, `escalation_route`, `degraded_state_label_class`,
  and `reuse_class` (card) or `release_truth_summary_id`,
  `release_target`, `ship_or_no_ship_posture_class`, `blockers`,
  `waived_blockers`, `stale_evidence_count`,
  `stale_evidence_summary_class`, `mitigations`,
  `export_packet_action`, `linked_decision_right_card_refs`, and
  `degraded_state_label_class` (summary). A surface that drops any
  required field is non-conforming.
- **Audience-export ceiling conformance.** Every card MUST stay
  inside the audience-specific role-export ceilings: claim manifest
  at `redacted_role_class_only_no_role_id`, support and enterprise
  review at most `role_id_with_concurrence_set` or
  `role_id_with_deadline_summary`, release packet at most
  `role_id_with_concurrence_set` or `role_id_with_decision_forum`.

## Acceptance coverage

The four required acceptance cases from
[`.plans/M00-478.md`](../../../.plans/M00-478.md) are covered as
follows:

- **Pending security review** —
  `pending_security_review_card.yaml` plus the companion
  `pending_security_review_summary.yaml` (signed-pack publication
  egress class, code-signing chain rotation pending the
  security-and-trust review, deadline within seven days, parent
  release-truth summary holds the stable publish).
- **Waived blocker with expiry** —
  `waived_blocker_with_expiry_card.yaml` plus the companion
  `waived_blocker_with_expiry_summary.yaml`
  (ff.vfs_save_conflict_handling on the air-gapped lab profile,
  performance-council waiver expiring 2026-06-30, parent
  release-truth summary holds the publish under
  `hold_waived_blockers_within_expiry` with the
  `nearing_expiry_within_thirty_days` proximity class).
- **Stale evidence holding ship** —
  `stale_evidence_holding_ship_card.yaml` plus the companion
  `stale_evidence_holding_ship_summary.yaml`
  (ff.warm_start_to_first_paint and ff.input_to_paint aged out
  against the new startup-corpus revision, parent release-truth
  summary renders `stale_evidence_count = 2` plus
  `degraded_evidence_stale`).
- **Fully satisfied promotion path** —
  `fully_satisfied_promotion_path_card.yaml` plus the companion
  `fully_satisfied_promotion_path_summary.yaml` (stable 2.1.0
  release candidate with all required concurrence recorded, parent
  release-truth summary renders `ship_satisfied_no_blockers` and
  routes `publish_to_target_after_concurrence`).
