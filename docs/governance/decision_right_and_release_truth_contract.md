# Decision-right card, forum-reason grammar, and release-truth summary contract

This contract freezes one shared vocabulary for the decision-right card
and the release-truth summary. It exists so reviewers can tell who
owns the next decision, why that forum is required, and whether the
current release moment is ship or no-ship — without reading meeting
notes, without scanning chat, and without hunting through nested
release packets.

The decision-right card and the release-truth summary are one object
family. Every release-truth summary cites at least one decision-right
card. Every decision-right card whose reason is a release-bearing
class (stable promotion, support window or LTS, release-blocking
waiver, protected release artifact change) cites at least one
release-truth summary back. The link is machine-readable and
exportable so a contested decision can be reconstructed from packet
ids without inventing parallel status fields.

The contract is pre-implementation. It defines the reusable record
shapes, the closed vocabularies, the projection rules, the export-
parity floor, and the fixture corpus. It does not convene review
councils, execute promotions, or implement a release-center UI.

## Companion artifacts

- [`/schemas/governance/decision_right_card.schema.json`](../../schemas/governance/decision_right_card.schema.json)
  — boundary schema for one `decision_right_card_record`.
- [`/schemas/governance/release_truth_summary.schema.json`](../../schemas/governance/release_truth_summary.schema.json)
  — boundary schema for one `release_truth_summary_record`.
- [`/fixtures/governance/decision_right_cases/`](../../fixtures/governance/decision_right_cases/)
  — worked records covering pending security review, waived blocker
  with expiry, stale evidence holding ship, and a fully satisfied
  promotion path.
- [`/docs/governance/decision_rights_and_signoff_matrix.md`](./decision_rights_and_signoff_matrix.md),
  [`/artifacts/governance/signoff_matrix.yaml`](../../artifacts/governance/signoff_matrix.yaml),
  and
  [`/artifacts/governance/promotion_decision_rows.yaml`](../../artifacts/governance/promotion_decision_rows.yaml)
  — role-based decision-rights matrix. The card carries the same
  `accountable_owner_role_ref`, `concurrence_role_refs`, and degraded-
  state vocabulary so launch-bearing decisions and ownership status
  agree.
- [`/docs/governance/decision_register.md`](./decision_register.md) and
  [`/artifacts/governance/decision_register.yaml`](../../artifacts/governance/decision_register.yaml)
  — launch decision register. Cards whose reason is a launch-bearing
  decision cite the `LR-NNNN` row id in `linked_evidence_refs`.
- [`/docs/governance/forum_charters.md`](./forum_charters.md) and
  [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
  — forum register. Every `decision_forum_ref` resolves into a forum
  here; `required_forum_class` mirrors the forum vocabulary verbatim
  except for the `no_forum_named_degraded` refusal sentinel.
- [`/docs/governance/service_ownership_card_contract.md`](./service_ownership_card_contract.md)
  and
  [`/schemas/governance/service_ownership_card.schema.json`](../../schemas/governance/service_ownership_card.schema.json)
  — sibling vocabulary. The decision-right card reuses
  `decision_subject_class` (mirrors `surface_identity_class`),
  `escalation_path_class`, `linked_evidence_kind_class`, and
  `role_export_class` verbatim and links each card back to the parent
  `service_ownership_card_record` through
  `linked_service_ownership_card_ref`.
- [`/schemas/governance/fitness_tile.schema.json`](../../schemas/governance/fitness_tile.schema.json),
  [`/schemas/governance/nightly_report_row.schema.json`](../../schemas/governance/nightly_report_row.schema.json),
  and
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  — sibling vocabularies. The release-truth summary reuses the
  blocker-kind, mitigation, expiry-proximity, and waiver-authority
  enums frozen there so a live tile, a nightly row, a waiver-expiry
  queue item, and a release-truth summary do not invent two different
  "blocked" or "waived" tokens.
- [`/schemas/release/release_candidate_card.schema.json`](../../schemas/release/release_candidate_card.schema.json),
  [`/schemas/release/publish_target.schema.json`](../../schemas/release/publish_target.schema.json),
  and
  [`/schemas/release/rollback_revocation_panel.schema.json`](../../schemas/release/rollback_revocation_panel.schema.json)
  — release-center object family. The release-truth summary points at
  these records through `release_candidate_ref`, `publish_target_ref`,
  and `rollback_or_revocation_panel_ref` so the ship-or-no-ship
  posture and the publication action share one object model.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` — accountability, signoff, and release-
  decision requirements (M0-bearing requirement IDs, RFC 2119
  MUST / SHOULD language).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.9 release
  governance and shiproom cadence.
- `.t2/docs/Aureline_Technical_Design_Document.md` §8.36 release
  evidence and §8.41 supportability evidence.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` decision-visibility rules
  on the release-status, About / update center, and support-export
  surfaces.

If this contract disagrees with those sources, those sources win and
this contract, the schemas, and the fixtures update in the same
change.

## 1. Why this contract exists

1. **At-risk without a named forum is the failure mode.** Without one
   shared decision-right card, a release-blocking waiver lands on
   "shiproom will figure it out", a contested security egress class
   sits on whoever happens to be in chat, and a compatibility-claim
   narrowing decision drifts between forum charters. The card exists
   so the dashboard, the scorecard, the release-truth summary, the
   release packet, the support bundle, the claim manifest, and the
   compatibility-window report all quote the **same** decision row
   with the **same** stable IDs and the **same** typed reason.
2. **Ship or no-ship is one object family.** Without one shared
   release-truth summary, the dashboard says "blockers: 2", the
   scorecard says "at risk", the release packet says "go", and the
   support bundle says nothing. The summary exists so every consuming
   surface renders the **same** posture token, the **same** open
   blockers, the **same** waived blockers (with expiry), the **same**
   stale evidence count, and the **same** export-packet action.
3. **Plain-language reason grammar, not free text.** The
   `reason_for_review_class` enumerates the launch-critical reasons
   (public CLI schema change, security-sensitive egress class,
   protected release artifact, compatibility claim narrowing,
   release-blocking waiver, protected metric threshold change,
   public-truth publication, stable promotion, claim or archetype
   publication, support window or LTS, architecture freeze, discipline
   waiver, workflow-bundle certification, temporary-handoff or
   ownership resolution) plus the explicit
   `decision_route_unresolved_degraded` refusal sentinel. Surfaces
   render the typed class plus a bounded reviewable reason summary,
   never free-form prose.
4. **Machine-readable linkage back to decision IDs.** The release-
   truth summary's `linked_decision_right_card_refs` array is
   non-empty by schema. Every blocker, waived blocker, and stale-
   evidence entry on the summary cites a `decision_right_card_ref`.
   Every decision-right card on a release-bearing reason cites at
   least one `linked_release_truth_summary_ref` back. The link is
   reconstructable from packet ids; meeting notes are not the source
   of truth.
5. **Degraded states are explicit.** Unresolved ownership renders as
   `degraded_owner_unresolved`; missing decision route renders as
   `degraded_decision_route_missing`; failed concurrence renders as
   `degraded_concurrence_failed_pending_escalation`; stale evidence
   renders as `degraded_evidence_stale`; expired waiver renders as
   `degraded_waiver_expired`; past deadline with no decision renders
   as `degraded_past_deadline_no_decision`. None of these can collapse
   into a clean satisfied chip on any consuming surface.

## 2. Decision-right card shape

A `decision_right_card_record` carries:

- `decision_right_card_id` — stable, machine-readable id quoted by
  every consuming surface.
- `decision_subject` — typed `decision_subject_class`, `subject_ref`,
  `lane_ref`, `protected_path_ref`, and a bounded reviewable
  `subject_summary`. The first nine `decision_subject_class` values
  mirror `surface_identity_class` on
  [`service_ownership_card.schema.json`](../../schemas/governance/service_ownership_card.schema.json)
  verbatim; the four release-bearing values
  (`release_candidate_or_promotion_target`,
  `lts_line_or_support_window`,
  `workflow_bundle_or_extension_certification`,
  `temporary_handoff_or_ownership_resolution`) are kept distinct so
  release-bearing decisions can be reconstructed without fanning into
  the surface vocabulary.
- `reason_for_review` — typed `reason_for_review_class` (§4) plus a
  bounded reviewable `reason_summary`.
- `required_forum` — typed `required_forum_class` (§5),
  `decision_forum_ref`, and a bounded reviewable `forum_summary`. A
  card whose `required_forum_class = no_forum_named_degraded` MUST
  carry a null `decision_forum_ref` and the parent state MUST be
  `blocked_no_decision_route_named` plus
  `degraded_decision_route_missing`.
- `accountable_ownership` — `accountable_owner_role_ref`,
  `concurrence_role_refs`, `co_owner_role_refs`, the active
  `linked_waiver_ref` (when the decision rides on a waiver), the
  `linked_service_ownership_card_ref` (parent ownership card), the
  typed `concurrence_outcome_class` (§7), and a bounded reviewable
  `ownership_summary`.
- `decision_state` — typed `decision_right_state_class` (§6), typed
  `deadline_proximity_class` (§8), `target_milestone_ref`,
  `target_due_at`, `decided_at`, and a bounded reviewable
  `state_summary`.
- `escalation_route` — typed `escalation_path_class` (mirrors
  [`waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  and
  [`service_ownership_card.schema.json`](../../schemas/governance/service_ownership_card.schema.json)),
  `decision_forum_ref`, an ordered `escalation_chain_role_refs`
  array, and a bounded reviewable `escalation_summary`.
- `linked_evidence_refs` — typed refs into release-evidence packets,
  governance packets, signoff rows, certified-archetype reports,
  compatibility rows, claim-manifest rows, scorecard lanes,
  fitness-function catalog rows, protected-metrics rows, waiver
  records, decision-index rows, decision-register rows
  (`LR-NNNN`), the parent service-ownership card, support-export
  packets, and release-truth summaries.
- `linked_release_truth_summary_refs` — refs into the
  release-truth summaries that consume this card. Required to be
  non-empty when `reason_for_review_class` is in
  `{ stable_promotion_decision, support_window_or_lts_decision,
   release_blocking_waiver, protected_release_artifact_change }`.
- `current_decision_right_state` — typed
  `decision_right_state_class` copied from `decision_state` for
  export-parity convenience; surfaces MUST render this value
  verbatim.
- `degraded_state_label_class` — typed degraded label (§9). A card
  whose state is anything other than the four `satisfied_*` /
  `pending_review_within_window` / `pending_review_nearing_deadline`
  classes MUST carry the matching degraded label.
- `role_export_rules` — typed `role_export_class` per audience
  (dashboard, scorecard, release packet, support export, governance
  packet, claim manifest, enterprise review; §10).
- `reuse_class` — typed `reuse_class` (§11) naming whether the card
  is required, recommended, or not-applicable for the subject.
- `export_fields` — typed booleans for dashboard, scorecard, release
  packet, release-truth summary, support export, governance packet,
  claim manifest, and enterprise review (§12).

## 3. Release-truth summary shape

A `release_truth_summary_record` carries:

- `release_truth_summary_id` — stable, machine-readable id quoted by
  every consuming surface.
- `release_target` — typed `release_target_class` (§13),
  `release_candidate_ref`, `publish_target_ref`, `channel_ref`,
  `lts_line_ref`, `headline_label`, and `release_target_summary`.
- `ship_or_no_ship_posture_class` — typed posture (§14).
- `headline_label` — bounded reviewable headline label.
- `blockers` — array of typed blocker entries (§15). Each entry
  carries the `blocker_kind_class`, the `decision_right_card_ref`,
  the `linked_evidence_ref`, and the bounded `blocker_summary`.
- `waived_blockers` — array of typed waived-blocker entries (§16).
  Each entry carries the `blocker_kind_class`, the
  `decision_right_card_ref`, the `waiver_ref`, the
  `waiver_authority_class`, the `expiry_proximity_class`, the
  `waiver_expires_on`, the `linked_evidence_ref`, and the bounded
  `waiver_summary`.
- `stale_evidence_count` and `stale_evidence_summary_class` (§17)
  plus the `stale_evidence_entries` array. Length matches count up
  to the cap.
- `mitigations` — array of typed mitigation entries (§18). Each
  entry carries the `mitigation_class`, `what_users_should_do`,
  `what_operators_should_do`, and the bounded `mitigation_summary`.
- `export_packet_action` — typed `export_packet_action_class` (§19)
  plus the `release_packet_ref`, `publish_target_ref`,
  `rollback_or_revocation_panel_ref`, `support_export_packet_ref`,
  and bounded `action_summary`.
- `linked_decision_right_card_refs` — non-empty array of refs into
  the decision-right cards that gate the posture. Every
  `blocker_entry`, `waived_blocker_entry`, and `stale_evidence_entry`
  `decision_right_card_ref` MUST appear here.
- `degraded_state_label_class` and `degraded_state_summary` (§9 +
  rollback / yank extension).
- `export_fields` — typed booleans (§12).

## 4. Reason-for-review vocabulary

Closed fifteen-class vocabulary:

| Class | Plain-language meaning |
| --- | --- |
| `public_cli_schema_change` | A public CLI schema row, command-action schema row, or workflow-bundle schema row is changing in a way that affects external consumers. |
| `security_sensitive_egress_class_change` | A security-sensitive egress / transport posture class is changing (network egress, signed-pack publication, code-signing chain, transport-trust posture, AI-broker outbound class, support-export upload class). |
| `protected_release_artifact_change` | A protected release artifact (signed pack, mirror feed, registry / marketplace listing, certified-archetype bundle) is being added, modified, or revoked. |
| `compatibility_claim_narrowing` | A compatibility row, certified-archetype row, or claim-manifest row is narrowing (reducing supported scope, deprecating a profile, downgrading a class). |
| `release_blocking_waiver` | A waiver is being requested or renewed that would hold a release-bearing failure open. The waiver ref MUST be cited. |
| `protected_metric_threshold_change` | A protected fitness-function or protected-metric threshold is changing. |
| `public_truth_publication` | A public-truth row (docs, README, AGENTS.md, known-limits matrix, support-window statement, migration guide) is being added, modified, or revoked. |
| `stable_promotion_decision` | A candidate is being promoted to stable on a release channel. The required forum is `release_council` or `shiproom_executive_scope_review`. |
| `claim_or_archetype_publication` | A claim manifest row or certified-archetype row is being published or republished. |
| `support_window_or_lts_decision` | A support window or LTS line is being created, renewed, or retired. |
| `architecture_freeze_or_unfreeze` | An architecture freeze is being declared or relaxed. |
| `discipline_waiver_request` | A discipline waiver (security, reliability, accessibility, trust) is being requested or renewed. |
| `workflow_bundle_certification_change` | A workflow-bundle certification class is being assigned, renewed, downgraded, or retired. |
| `temporary_handoff_or_ownership_resolution` | A temporary handoff is being opened, renewed, closed, or escalated; or an ownership-unresolved row is being resolved. |
| `decision_route_unresolved_degraded` | The accountable owner cannot resolve and the required forum cannot be named. The card MUST render the refusal sentinel; the parent state MUST be `blocked_no_decision_route_named` plus `degraded_decision_route_missing`. |

A card whose reason cannot be typed denies with
`reason_for_review_class_unresolved` rather than defaulting.

## 5. Required-forum vocabulary

Closed eleven-class vocabulary mirroring
[`forum_charters.md`](./forum_charters.md) verbatim, plus the explicit
refusal sentinel:

| Class | Meaning |
| --- | --- |
| `architecture_council` | Architecture freeze, protected-path promotion, host-class scoping, ADR / RFC closure. |
| `performance_council` | Protected-metric thresholds, fitness-function catalog changes, performance waivers. |
| `security_trust_review` | Security-sensitive egress class changes, code-signing chain rotations, transport-trust posture, AI-broker outbound class changes. |
| `accessibility_review` | Accessibility certification rows, accessibility waivers. |
| `compatibility_ecosystem_review` | Compatibility-claim narrowing, certified-archetype publication, workflow-bundle certification. |
| `product_scope_review` | Cutline changes, scope narrowing, claim publication scope. |
| `open_community_sync` | Public community-truth changes, open-source policy changes. |
| `benchmark_council` | Benchmark catalog changes, fitness-function row provisioning. |
| `release_council` | Stable promotion, release-channel posture, release-blocking waivers, claim publication. |
| `shiproom_executive_scope_review` | LTS-line creation, sponsor-scope decisions, contested release-bearing decisions. |
| `no_forum_named_degraded` | Refusal sentinel: no forum can be named. The card MUST render `degraded_decision_route_missing`. |

Schema-enforced pairings:

- `required_forum_class = no_forum_named_degraded` requires
  `decision_forum_ref = null`, `decision_right_state_class =
  blocked_no_decision_route_named`, and
  `degraded_state_label_class = degraded_decision_route_missing`.
- Every other class requires a non-null `decision_forum_ref`.

## 6. Decision-right state vocabulary

Closed eleven-class vocabulary:

| Class | Meaning |
| --- | --- |
| `pending_review_within_window` | The decision is pending and the deadline window is open; surfaces SHOULD render the typed reason and the deadline proximity. |
| `pending_review_nearing_deadline` | The decision is pending and the deadline is within thirty days; surfaces MUST render the deadline-proximity chip. |
| `pending_review_past_deadline` | The decision is pending past its deadline. The card MUST render `degraded_past_deadline_no_decision` and the parent release-truth summary MUST treat this as a blocker. |
| `satisfied_concurrence_complete` | All required concurrence is recorded; the decision is approved. The `decided_at` timestamp MUST be non-null. |
| `satisfied_with_named_narrowing` | The decision is approved with a named narrowing; the parent release-truth summary MUST cite at least one mitigation entry naming the narrowing. |
| `satisfied_with_active_waiver` | The decision is approved under an active waiver inside its expiry window. The `linked_waiver_ref` MUST be non-null. |
| `blocked_concurrence_failed` | Required concurrence has failed; escalation is in flight. The `escalation_path_class` MUST be in `{ escalate_to_decision_forum, escalate_to_release_council, escalate_to_shiproom }`. |
| `blocked_owner_unresolved` | The accountable owner cannot resolve through the ownership matrix. The card MUST render `degraded_owner_unresolved`. |
| `blocked_no_decision_route_named` | The required forum cannot be named. The card MUST render the refusal sentinel and `degraded_decision_route_missing`. |
| `superseded_by_later_decision` | A later decision-right card supersedes this one. The `decided_at` timestamp MUST be non-null. |
| `withdrawn_no_longer_required` | The decision is withdrawn; the affected scope no longer requires the decision. The `decided_at` timestamp MUST be non-null. |

A card whose state cannot be typed denies with
`decision_right_state_class_unresolved` rather than defaulting.

## 7. Concurrence-outcome vocabulary

Closed seven-class vocabulary kept distinct from the parent
decision-right state so a partially-acked decision cannot collapse
into "complete" on a release packet:

| Class | Meaning |
| --- | --- |
| `concurrence_complete` | All required concurrence roles have signed off. |
| `concurrence_partial_pending_remaining_roles` | Some concurrence roles have signed off; remaining roles are pending. |
| `concurrence_pending_within_window` | Concurrence has not started or is still inside the review window. |
| `concurrence_pending_past_deadline` | Concurrence is past its deadline; surfaces MUST render the past-deadline chip. |
| `concurrence_failed_escalation_in_flight` | Required concurrence has failed; escalation is in flight. |
| `concurrence_not_required_for_class` | The decision class does not require concurrence (rare; admissible only for closed-vocabulary administrative classes). The `concurrence_role_refs` array MUST be empty. |
| `concurrence_not_applicable_withdrawn_or_superseded` | The decision is withdrawn or superseded; concurrence is no longer applicable. |

## 8. Deadline-proximity vocabulary

Closed six-class vocabulary mirroring
[`waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)#expiry_proximity_class
verbatim:

| Class | Meaning |
| --- | --- |
| `expired_past_due` | Deadline is in the past. Forces `decision_right_state_class = pending_review_past_deadline` and `degraded_past_deadline_no_decision`. |
| `due_today_or_within_24h` | Deadline is within the next 24 hours. |
| `nearing_deadline_within_seven_days` | Deadline is within the next seven days. |
| `nearing_deadline_within_thirty_days` | Deadline is within the next thirty days. |
| `not_yet_due_window_open` | Deadline is more than thirty days away. |
| `no_deadline_pinned` | No deadline is set. Forces `target_due_at = null` and `target_milestone_ref = null`. |

## 9. Degraded-state-label vocabulary

Closed seven-class vocabulary on the decision-right card
(eight-class on the release-truth summary, with the rollback / yank
extension):

| Class | Meaning |
| --- | --- |
| `not_degraded_satisfied_or_pending` | Card / summary is satisfied or pending inside its window with no degraded posture. |
| `degraded_owner_unresolved` | The accountable role does not resolve to an active owner. |
| `degraded_decision_route_missing` | No forum can be named. The refusal sentinel pairs with `no_forum_named_degraded` and `blocked_no_decision_route_named`. |
| `degraded_concurrence_failed_pending_escalation` | Required concurrence has failed; escalation is in flight. |
| `degraded_evidence_stale` | The underlying evidence packet has aged out (time, trigger, or missing). |
| `degraded_waiver_expired` | A waiver covering the card or summary has expired. |
| `degraded_past_deadline_no_decision` | Decision is past its deadline with no closure. |
| `degraded_rollback_or_yank_in_flight` | (Release-truth summary only.) A previously published artifact is rolling back, yanking, revoking, or repinning. |

Schema-enforced pairings (decision-right card):

- `decision_right_state_class = pending_review_past_deadline` requires
  `degraded_past_deadline_no_decision`.
- `decision_right_state_class = satisfied_*` requires
  `not_degraded_satisfied_or_pending`.
- `decision_right_state_class = blocked_concurrence_failed` requires
  `degraded_concurrence_failed_pending_escalation`,
  `concurrence_outcome_class = concurrence_failed_escalation_in_flight`,
  and `escalation_path_class` in `{ escalate_to_decision_forum,
  escalate_to_release_council, escalate_to_shiproom }`.
- `decision_right_state_class = blocked_owner_unresolved` requires
  `degraded_owner_unresolved`.
- `decision_right_state_class = blocked_no_decision_route_named`
  requires `degraded_decision_route_missing`,
  `required_forum_class = no_forum_named_degraded`,
  `decision_forum_ref = null`, and
  `reason_for_review_class = decision_route_unresolved_degraded`.

## 10. Role-based export rules

Closed five-class `role_export_class` vocabulary, applied per
audience:

| Class | Meaning |
| --- | --- |
| `role_id_only` | Only the role id, the role label, and the lane scope are exposed. |
| `role_id_with_decision_forum` | Role id plus the linked decision forum and the escalation path. Used for shiproom and release packets. |
| `role_id_with_concurrence_set` | Role id plus the named concurrence roles. Used when the audience needs the full concurrence set. |
| `role_id_with_deadline_summary` | Role id plus the deadline proximity and the typed state. Used for the support bundle and the waiver-expiry queue. |
| `redacted_role_class_only_no_role_id` | Only the role class is exposed. Used for public claim manifests and external compatibility reports. |

Schema-enforced ceilings:

- `claim_manifest_export_class` MUST be
  `redacted_role_class_only_no_role_id`.
- `support_export_class` and `enterprise_review_export_class` MUST
  be in `{ role_id_only, role_id_with_deadline_summary,
  role_id_with_concurrence_set }`.
- `release_packet_export_class` MUST be in
  `{ role_id_only, role_id_with_decision_forum,
  role_id_with_concurrence_set, role_id_with_deadline_summary }`.

Personal contact details, private schedules, on-call calendars, raw
email addresses, raw phone numbers, raw chat-room URLs, and raw
meeting refs MUST NOT appear on any surface. The card carries
opaque role IDs only; resolution from role to active person flows
through
[`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).

## 11. Reuse-class vocabulary

Closed three-class vocabulary:

| Class | Meaning |
| --- | --- |
| `required_named_decision_right` | Subject MUST carry a card. Promotion is blocked if the card is missing, degraded, or expired. Required for protected metrics, compatibility / certified-archetype claims, security-sensitive egress classes, public-truth areas, packet families, supportability families, runtime host classes, review families, release candidates, LTS lines, and workflow-bundle certifications. |
| `recommended_named_decision_right` | Subject SHOULD carry a card; absence is a signal but not a blocker. |
| `not_applicable_internal_tooling` | Subject is internal tooling not subject to the decision-right card contract. |

Schema-enforced pairings:

- `decision_subject_class` in
  `{ protected_metric_or_fitness_function,
   compatibility_or_certified_archetype_claim,
   security_sensitive_egress_class, public_truth_area,
   packet_family, supportability_family, runtime_host_class,
   review_family, release_candidate_or_promotion_target,
   lts_line_or_support_window,
   workflow_bundle_or_extension_certification }` requires
  `reuse_class = required_named_decision_right`.
- `reuse_class = required_named_decision_right` requires
  `linked_evidence_refs` non-empty.
- `reason_for_review_class = stable_promotion_decision` requires
  `reuse_class = required_named_decision_right` and
  `required_forum_class` in `{ release_council,
  shiproom_executive_scope_review }`.

## 12. Export parity

Every consuming surface that renders the card or the summary MUST
render the same typed fields. The parity floor is enforced by the
schema's `allOf` block.

Required on every consuming surface (decision-right card):

- `decision_right_card_id`,
  `decision_subject.decision_subject_class`,
  `decision_subject.headline_label`;
- `reason_for_review.reason_for_review_class`,
  `reason_for_review.reason_summary`;
- `required_forum.required_forum_class`,
  `required_forum.decision_forum_ref`;
- `accountable_ownership.accountable_owner_role_ref`,
  `accountable_ownership.concurrence_outcome_class`;
- `decision_state.decision_right_state_class`,
  `decision_state.deadline_proximity_class`,
  `decision_state.target_due_at`;
- `escalation_route.escalation_path_class`,
  `escalation_route.decision_forum_ref`;
- `current_decision_right_state`;
- `degraded_state_label_class`, `degraded_state_summary`;
- `reuse_class`.

Required on every consuming surface (release-truth summary):

- `release_truth_summary_id`,
  `release_target.release_target_class`,
  `release_target.headline_label`;
- `ship_or_no_ship_posture_class`, `headline_label`;
- `blockers[]` (each entry's `blocker_kind_class`,
  `decision_right_card_ref`, `blocker_summary`);
- `waived_blockers[]` (each entry's `blocker_kind_class`,
  `waiver_authority_class`, `expiry_proximity_class`,
  `waiver_expires_on`, `decision_right_card_ref`,
  `waiver_summary`);
- `stale_evidence_count`, `stale_evidence_summary_class`;
- `mitigations[]` (each entry's `mitigation_class`,
  `mitigation_summary`);
- `export_packet_action.export_packet_action_class`,
  `export_packet_action.action_summary`;
- `linked_decision_right_card_refs`;
- `degraded_state_label_class`, `degraded_state_summary`.

Forbidden collapses on release-packet, support-packet, claim-
manifest, governance-packet, and enterprise-review surfaces:

- rendering a `pending_review_past_deadline` card or a
  `no_ship_decision_route_unresolved` summary as a clean satisfied /
  go chip to unblock publication;
- dropping `escalation_route` so a contested decision reads as
  ambiguous "review pending";
- dropping `degraded_state_label` so chronic decision drift looks
  like an isolated incident;
- collapsing two distinct refusal states (`degraded_owner_unresolved`,
  `degraded_decision_route_missing`,
  `degraded_concurrence_failed_pending_escalation`) into one bare
  "needs review" chip;
- exposing personal contact details, private schedules, on-call
  calendars, raw email addresses, raw phone numbers, raw chat-room
  URLs, or raw meeting refs;
- emitting the role id at a `role_export_class` higher than the
  audience's permitted ceiling (§10);
- omitting `linked_decision_right_card_refs` on a release-truth
  summary (the array is non-empty by schema and the linkage MUST be
  reconstructable from packet ids).

## 13. Release-target vocabulary

Closed nine-class vocabulary on the release-truth summary:

| Class | Meaning |
| --- | --- |
| `stable_release_candidate` | A release candidate is being evaluated for stable promotion. Requires non-null `release_candidate_ref`, `publish_target_ref`, and `channel_ref`. |
| `lts_line_creation_or_renewal` | An LTS line is being created or renewed. Requires non-null `lts_line_ref`. |
| `preview_or_beta_promotion` | A preview / beta candidate is being promoted. |
| `workflow_bundle_certification` | A workflow-bundle certification is being assigned, renewed, downgraded, or retired. |
| `claim_or_archetype_publication` | A claim manifest row or certified-archetype row is being published. |
| `public_truth_publication` | A public-truth row (docs, README, AGENTS.md, known-limits matrix, support-window statement, migration guide) is being published. |
| `support_window_or_end_of_support_notice` | A support-window or end-of-support notice is being published. |
| `compatibility_window_publication` | A compatibility-window report is being published. |
| `rollback_or_yank_in_flight` | A previously-published artifact is rolling back, yanking, revoking, or repinning. |

## 14. Ship-or-no-ship posture vocabulary

Closed seven-class vocabulary:

| Class | Meaning |
| --- | --- |
| `ship_satisfied_no_blockers` | All decision-right cards are satisfied; no blockers, no waived blockers, zero stale evidence. The summary renders the not-degraded label. |
| `ship_with_named_narrowing` | Ship is approved with at least one named narrowing recorded as a `satisfied_with_named_narrowing` decision-right card and at least one mitigation entry. |
| `hold_blockers_open` | At least one open blocker is gating ship; the summary MUST NOT render as "go" on any consuming surface. |
| `hold_waived_blockers_within_expiry` | At least one waived blocker is gating ship under an active waiver inside its expiry window. The waiver-holds-release-until-expiry mitigation MUST be cited. |
| `no_ship_blockers_or_unresolved` | Ship is refused on at least one open blocker or unresolved owner. |
| `no_ship_decision_route_unresolved` | At least one decision-right card is in `blocked_no_decision_route_named`. The summary MUST render `degraded_decision_route_missing` rather than collapsing into "at risk". |
| `rollback_or_yank_in_flight` | A previously-published artifact is rolling back, yanking, revoking, or repinning. |

Schema-enforced pairings: see §1 of the schema's `allOf` block on
[`release_truth_summary.schema.json`](../../schemas/governance/release_truth_summary.schema.json).

## 15. Blocker-kind vocabulary

Closed twelve-class vocabulary:

| Class | Meaning |
| --- | --- |
| `protected_metric_failure` | A protected fitness function or protected metric is failing. |
| `compatibility_claim_failure` | A compatibility row, certified-archetype row, or claim-manifest row is failing. |
| `security_egress_class_failure` | A security-sensitive egress / transport posture class is failing. |
| `protected_release_artifact_failure` | A protected release artifact is failing. |
| `public_cli_schema_break` | A public CLI schema row is breaking. |
| `migration_helper_failure` | A migration helper handoff is failing. |
| `support_drill_failure` | A supportability drill is failing. |
| `nightly_governance_failure` | A nightly governance row is failing. |
| `ownership_unresolved_failure` | A surface is missing accountable ownership. |
| `decision_route_unresolved_failure` | A decision-right card is in `blocked_no_decision_route_named`. |
| `evidence_stale_failure` | A protected evidence packet has aged out. |
| `waiver_expired_failure` | A waiver covering a release-bearing failure has expired without renewal. |

## 16. Waived-blocker entry shape

A `waived_blocker_entry` carries:

- `blocker_kind_class` — the underlying blocker kind.
- `decision_right_card_ref` — opaque ref into the
  `satisfied_with_active_waiver` decision-right card. Required.
- `waiver_ref` — opaque ref into the waiver record in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
  Required.
- `waiver_authority_class` — typed waiver authority
  (`performance_council`, `architecture_council`, `release_council`,
  `shiproom_executive_scope_review`, `security_trust_review`,
  `compatibility_ecosystem_review`, `product_scope_review`,
  `not_applicable_no_active_waiver`).
- `expiry_proximity_class` — typed proximity
  (mirrors
  [`waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)).
- `waiver_expires_on` — copied from the waiver record. Required to be
  non-null whenever `expiry_proximity_class` is anything other than
  `no_expiry_pinned`.
- `linked_evidence_ref` — opaque ref into the underlying evidence.
- `waiver_summary` — bounded reviewable summary; raw waiver
  justification text MUST NOT appear.

A waived-blocker entry whose `expiry_proximity_class =
expired_past_due` forces the parent posture out of
`hold_waived_blockers_within_expiry` and into
`no_ship_blockers_or_unresolved`.

## 17. Stale-evidence count and summary

The `stale_evidence_count` is a non-negative integer. The
`stale_evidence_summary_class` is one of:

| Class | Meaning |
| --- | --- |
| `no_stale_evidence` | Zero stale rows. |
| `one_stale_evidence_row_named` | Exactly one stale row; the row is named in `stale_evidence_entries`. |
| `multiple_stale_evidence_rows_named` | Multiple stale rows; all are named in `stale_evidence_entries`. |
| `many_stale_evidence_rows_capped` | More stale rows than the cap; the cap label is rendered. |

Schema-enforced pairings:

- `stale_evidence_count = 0` forces `stale_evidence_summary_class =
  no_stale_evidence` and `stale_evidence_entries` empty.
- `stale_evidence_count >= 1` forces `stale_evidence_summary_class`
  out of `no_stale_evidence` and `stale_evidence_entries` non-empty.

A `ship_satisfied_no_blockers` summary forces
`stale_evidence_count = 0` so a non-zero stale count cannot collapse
into a clean ship posture.

## 18. Mitigation vocabulary

Closed ten-class vocabulary:

| Class | Meaning |
| --- | --- |
| `no_mitigation_required_satisfied` | The summary is satisfied; no mitigation is required. Admissible only when `ship_or_no_ship_posture_class = ship_satisfied_no_blockers`. |
| `narrower_scope_until_concurrence` | Scope is narrowed (one OS, one ring, one workspace) until concurrence completes. |
| `slower_path_active_until_concurrence` | A slower path is active until concurrence completes. |
| `less_portable_temporarily` | The release is less portable temporarily. |
| `temporarily_blocked_pending_decision` | Temporarily blocked pending a council / shiproom decision. |
| `waiver_holds_release_until_expiry` | A waiver holds the release open until expiry. |
| `partial_profile_result_pending_full_capture` | The result is partial (single profile, subset of profiles, subset of corpus rows) pending full capture. |
| `early_signal_drift_owner_inspecting` | Drift is in the warning band; the lane DRI is inspecting. |
| `escalated_pending_council_decision` | Escalated to a council / shiproom and pending the decision. |
| `rollback_in_flight_pending_revalidation` | A rollback / yank is in flight pending revalidation. |

## 19. Export-packet-action vocabulary

Closed nine-class vocabulary mirroring the publish-target /
break-glass / rollback / yank / revoke / repin actions on
[`/schemas/release/publish_target.schema.json`](../../schemas/release/publish_target.schema.json)
and
[`/schemas/release/rollback_revocation_panel.schema.json`](../../schemas/release/rollback_revocation_panel.schema.json):

| Class | Meaning |
| --- | --- |
| `publish_to_target_after_concurrence` | Publish to the named target once the gating concurrence completes. |
| `hold_publish_pending_blocker` | Hold publication pending an open blocker. |
| `hold_publish_pending_waiver_renewal` | Hold publication pending a waiver renewal. |
| `rollback_to_prior_stable` | Roll back to the prior stable artifact. |
| `yank_or_revoke_active_artifact` | Yank or revoke the currently-published artifact. |
| `repin_supported_floor` | Re-pin the supported floor (e.g. update center repin). |
| `mirror_only_emergency_push` | Mirror-only emergency push without a public stable publish. |
| `no_action_required_already_at_target` | No action; already at the target. |
| `escalate_for_break_glass_review` | Escalate for break-glass review (sponsor / shiproom). |

## 20. Authoring rules

When a new launch-bearing decision is opened:

1. Mint a `decision_right_card_record` with the typed
   `decision_subject_class`, the `subject_ref`, the
   `protected_path_ref` (when the subject sits on a protected path),
   and the typed `reason_for_review_class`.
2. Resolve `required_forum` against the forum register; the
   `decision_forum_ref` MUST be a forum id from
   [`forum_charters.md`](./forum_charters.md). When no forum can be
   named, render the refusal sentinel.
3. Resolve `accountable_ownership` against the ownership matrix and
   the parent `service_ownership_card_record`. If the role cannot
   resolve, the card is degraded and the same change MUST land
   either a closing correction or an ownership-resolution decision.
4. Resolve `decision_state` against the live evaluation time;
   surfaces MUST recompute `decision_right_state_class` and
   `deadline_proximity_class` every time the card is rendered.
5. Resolve `linked_evidence_refs` against the underlying packet,
   scorecard lane, signoff row, certified-archetype report,
   compatibility row, claim-manifest row, fitness-function row,
   protected-metric row, waiver record, decision-index row,
   decision-register row, parent service-ownership card,
   support-export packet, or release-truth summary.
6. Set `role_export_rules` per audience using the §10 ceilings.

When a release-truth summary is opened:

1. Mint a `release_truth_summary_record` with the typed
   `release_target_class` and the `release_candidate_ref` /
   `publish_target_ref` / `channel_ref` / `lts_line_ref` as
   applicable.
2. Compute `ship_or_no_ship_posture_class` from the live state of
   the gating decision-right cards; surfaces MUST recompute the
   posture every time the summary is rendered.
3. For each open blocker, mint a `blocker_entry` and cite its
   `decision_right_card_ref` in `linked_decision_right_card_refs`.
4. For each waived blocker, mint a `waived_blocker_entry` and cite
   its `decision_right_card_ref` in
   `linked_decision_right_card_refs`.
5. For each stale evidence row, mint a `stale_evidence_entry` and
   cite its `decision_right_card_ref` (when the stale row is gated
   by a decision) in `linked_decision_right_card_refs`.
6. For each typed mitigation, mint a `mitigation_entry` naming what
   users should do, what operators should do, and the meanwhile
   posture.
7. Resolve `export_packet_action` against the publish-target row,
   the rollback / revocation panel, and the support-export packet
   the action would publish, hold, or roll back.

When a decision-right card transitions into a degraded state:

1. Update `decision_right_state_class` to the matching blocked
   class, recompute `degraded_state_label_class`, and update
   `current_decision_right_state`.
2. Update every release-truth summary citing the card to recompute
   `ship_or_no_ship_posture_class` and `degraded_state_label_class`.
3. Route the escalation per §6 / §10.

## 21. Out of scope

This contract does not implement:

- a decision-tracking workflow tool, a calendar / availability
  surface, or a meeting scheduler;
- the council decision rights themselves (those live in
  [`./decision_rights_and_signoff_matrix.md`](./decision_rights_and_signoff_matrix.md)
  and the signoff matrix);
- the underlying waiver records (those live in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  and on
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json));
- the release-channel identity, support-window, or publish-target
  records (those live in `/schemas/release/`);
- the convening of any review forum or the execution of any
  promotion.

This contract is the projection vocabulary that those upstream
artifacts flow through when the decision-right of a launch-bearing
moment and the live ship-or-no-ship posture have to be visible and
exportable on the dashboard, the milestone scorecard, the release-
evidence shiproom packet, the support bundle, the claim manifest,
the compatibility-window report, and the enterprise / managed-
tenant review.
