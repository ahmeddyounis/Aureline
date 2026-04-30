# Service-ownership card, escalation route, and temporary-handoff expiry contract

This contract freezes one shared vocabulary for the service-ownership
card and the temporary-handoff record. It exists so accountable
ownership stays visible and exportable on every surface that has to
quote "who owns this" — the governance dashboard, the milestone
scorecard, the release-evidence shiproom packet, the support bundle,
the enterprise / managed-tenant review, the weekly governance review,
the claim manifest, and the compatibility-window report — without
relying on meeting memory or personal contact lists.

The contract is pre-implementation. It defines the reusable record
shapes, the closed vocabularies, the projection rules, the export-
parity floor, and the fixture corpus. It does not implement an
org-chart tool, a paging system, an on-call rotation engine, or a
calendar / availability surface.

## Companion artifacts

- [`/schemas/governance/service_ownership_card.schema.json`](../../schemas/governance/service_ownership_card.schema.json)
  — boundary schema for one `service_ownership_card_record`.
- [`/schemas/governance/temporary_handoff_record.schema.json`](../../schemas/governance/temporary_handoff_record.schema.json)
  — boundary schema for one `temporary_handoff_record`.
- [`/fixtures/governance/service_ownership_cases/`](../../fixtures/governance/service_ownership_cases/)
  — worked records covering an actively-owned protected lane, an
  active temporary handoff, an expired temporary handoff, and an
  escalated blocked surface.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — DRI, backup-owner, decision-forum, and waiver register. Every
  `owner_role_ref`, `backup_owner_role_ref`, `decision_forum_ref`, and
  `linked_waiver_ref` resolves into this matrix.
- [`/docs/governance/dri_map.md`](./dri_map.md) — narrative companion
  for ownership, blocker aging, and escalation. The card cites lane
  scope frozen there.
- [`/docs/governance/decision_rights_and_signoff_matrix.md`](./decision_rights_and_signoff_matrix.md),
  [`/artifacts/governance/signoff_matrix.yaml`](../../artifacts/governance/signoff_matrix.yaml),
  and
  [`/artifacts/governance/promotion_decision_rows.yaml`](../../artifacts/governance/promotion_decision_rows.yaml)
  — role-based decision-rights matrix. The card carries the same
  `accountable_owner_role_ids`, `concurrence_role_ids`, and degraded-
  state vocabulary so launch-bearing decisions and ownership status
  agree.
- [`/docs/governance/forum_charters.md`](./forum_charters.md) and
  [`/artifacts/governance/forum_matrix.yaml`](../../artifacts/governance/forum_matrix.yaml)
  — forum register. Every `decision_forum_ref` and every
  `escalation_path_class` resolves into a forum here.
- [`/artifacts/architecture/service_ownership_matrix.yaml`](../../artifacts/architecture/service_ownership_matrix.yaml)
  — runtime host-class and service-plane ownership. Cards whose
  `surface_identity_class` is `subsystem` or `runtime_host_class`
  resolve into this artifact.
- [`/schemas/governance/fitness_tile.schema.json`](../../schemas/governance/fitness_tile.schema.json),
  [`/schemas/governance/nightly_report_row.schema.json`](../../schemas/governance/nightly_report_row.schema.json),
  and
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  — sibling vocabularies. The card reuses the evidence-freshness,
  escalation-path, and waiver-authority enums frozen there so the
  ownership lane and the protected-fitness lane do not invent two
  different "expired" or "blocked" tokens.
- [`/schemas/release/support_window_badge.schema.json`](../../schemas/release/support_window_badge.schema.json)
  — release-channel-and-support-class badge. The card re-exports the
  eight-class `support_class` vocabulary verbatim so a "supported"
  surface and a "supported" ownership card mean the same thing.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` ownership, accountability, and release-
  decision requirements (M0-bearing requirement IDs RFC 2119 MUST /
  SHOULD language).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.9 release
  governance and ownership cadence.
- `.t2/docs/Aureline_Technical_Design_Document.md` §8.36 release
  evidence and §8.41 supportability evidence.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` ownership-visibility
  rules on About, update center, support-export, and release-status
  surfaces.

If this contract disagrees with those sources, those sources win and
this contract, the schemas, and the fixtures update in the same
change.

## 1. Why this contract exists

1. **Silent ownership drift is the failure mode.** Without one shared
   card shape, protected lanes drift toward "shared responsibility,"
   release-blocking decisions sit on whoever happens to be in the
   shiproom that week, and a support escalation has no DRI to route
   to. The card exists so the dashboard, the scorecard, the release
   packet, the support bundle, the claim manifest, and the
   compatibility-window report all quote the **same** ownership row
   with the **same** stable IDs.
2. **Temporary handoffs cannot collapse into permanent silence.**
   Coverage gaps happen — vacation, leave, parent-leave, recused
   reviewer, lab outage. The temporary-handoff record exists so the
   gap is recorded with a start date, a named expiry, the linked
   evidence or waiver, and the closure or renewal path. When the
   handoff lapses, the surface MUST render as degraded; it MUST NOT
   continue rendering as actively owned.
3. **Role-based, not personal.** Cards quote roles (`@chief_architect`,
   `@release_owner`, `@security_lead`, `@enterprise_support_owner`,
   etc.) and resolve them to active people through the ownership
   matrix. The schema forbids private contact details, calendar
   availability, and personal schedules so the export is safe for
   shiproom, support, and enterprise review.
4. **Protected paths and public truth must name accountable
   ownership.** Once a surface is promoted into a protected path, a
   public-truth area, a security-sensitive egress class, or a
   compatibility claim, it cannot run without a card. The schema's
   `requires_named_accountable_ownership` invariant blocks promotion
   when the card is missing or degraded.
5. **Export parity keeps screenshots out of the workflow.** Every
   consuming surface renders the same role IDs, the same support
   class, the same accountability status, and the same escalation
   path. The schema's `export_fields` block enforces a parity floor
   so the release packet cannot drop the escalation path, the support
   bundle cannot drop the freshness label, and the enterprise review
   cannot drop the temporary-handoff expiry.

## 2. Service-ownership card shape

A `service_ownership_card_record` carries:

- `service_ownership_card_id` — stable, machine-readable id quoted by
  every consuming surface.
- `surface_identity` — typed `surface_identity_class`,
  `surface_ref`, `lane_ref`, `protected_path_ref`, and a bounded
  reviewable `surface_summary`. The pair of `surface_identity_class`
  and `surface_ref` is the join key against the ownership matrix.
- `accountable_ownership` — `primary_owner_role_ref`,
  `backup_owner_role_ref`, `co_owner_role_refs`, `owning_team_label`,
  `concurrence_role_refs`, the active waiver ref (if a single-
  maintainer or other backup waiver is in force), and the typed
  `accountability_status_class` (§4).
- `support_posture` — typed eight-class `support_class` (mirrors
  [`support_window_badge.schema.json`](../../schemas/release/support_window_badge.schema.json)),
  the `support_window_badge_ref`, and a bounded reviewable
  `support_posture_summary`. A card whose surface is a public-truth,
  release, or claim-bearing row MUST cite a non-null
  `support_window_badge_ref`.
- `escalation_route` — typed `escalation_path_class` (mirrors
  [`waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)),
  the `decision_forum_ref`, an ordered `escalation_chain_role_refs`
  array of the next-up roles, and a bounded reviewable
  `escalation_summary`.
- `freshness` — typed `ownership_freshness_class` (§5),
  `last_reaffirmed_at`, `stale_after`, the computed `expires_at`, and
  a bounded reviewable `freshness_summary`. The freshness vocabulary
  reuses the evidence-freshness shape from
  [`fitness_tile.schema.json`](../../schemas/governance/fitness_tile.schema.json)
  but graded against an ownership reaffirmation rather than an
  evidence-packet capture.
- `linked_evidence_refs` — typed refs into release evidence packets,
  governance packets, signoff rows, certified-archetype reports,
  compatibility rows, claim-manifest rows, scorecard lanes,
  fitness-function catalog rows, protected-metrics rows, and waiver
  records. A card on a protected, public-truth, release-bearing, or
  claim-bearing surface that names no linked evidence is
  non-conforming.
- `temporary_handoff_refs` — refs into the open temporary-handoff
  records that gate the card's accountability status (zero, one, or
  many).
- `current_accountability_status` — typed `accountability_status_class`
  copied from `accountable_ownership` for export-parity convenience;
  surfaces MUST render this value verbatim.
- `degraded_state_label` — typed `degraded_state_label_class` (§7).
  A card whose `accountability_status_class` is anything other than
  `active_owner_no_handoff` MUST carry the matching degraded-state
  label.
- `role_export_rules` — typed `role_export_class` per audience
  (shiproom, support, enterprise review, public claim manifest;
  §8). The schema enforces that personal contact details, private
  schedules, and on-call calendars never enter the record.
- `reuse_class` — typed `reuse_class` (§9) naming whether the card
  is required, recommended, or not-applicable for the surface kind
  (protected path, public surface, security-sensitive egress class,
  compatibility claim, internal tooling lane).
- `export_fields` — typed booleans for dashboard, scorecard, release
  packet, support export, governance packet, claim manifest, and
  enterprise review (§10).

## 3. Stable IDs and human-readable copy

Cards carry both:

- machine-stable ids — `service_ownership_card_id`, `surface_ref`,
  `lane_ref`, `protected_path_ref`, `primary_owner_role_ref`,
  `backup_owner_role_ref`, `co_owner_role_refs`,
  `concurrence_role_refs`, `decision_forum_ref`,
  `escalation_chain_role_refs`, `linked_evidence_refs`,
  `temporary_handoff_refs`, `support_window_badge_ref`,
  `linked_waiver_ref`; and
- bounded reviewable copy — `headline_label`, `surface_summary`,
  `support_posture_summary`, `escalation_summary`,
  `freshness_summary`, `accountability_summary`,
  `degraded_state_summary`.

Surfaces render the copy verbatim and quote the IDs as refs. Tooling
consumes the IDs and the typed class fields without parsing the
copy. Personal handles, raw email addresses, raw phone numbers, raw
calendar URLs, raw chat-room URLs, raw absolute paths, and raw
secret material MUST NOT appear; the record carries opaque refs,
typed vocabulary, and bounded reviewable summaries only.

Roles are quoted as opaque role IDs (e.g. `role:chief_architect`,
`role:release_owner`, `role:enterprise_support_owner`,
`role:security_lead`, `role:certification_qe_owner`). Resolution
from role to active person flows through the ownership matrix so
the card stays role-based and exportable.

## 4. Accountability-status vocabulary

The closed seven-class accountability-status vocabulary:

| Class | Meaning |
| --- | --- |
| `active_owner_no_handoff` | A named primary DRI is active; no temporary handoff is in force. The card MUST cite either a named backup or an active backup waiver. |
| `active_owner_with_backup_waiver` | A named primary DRI is active; backup coverage is held by a named backup-owner waiver (e.g. the standing single-maintainer-backup waiver) with a recorded expiry and escalation path. |
| `temporary_handoff_active` | A named temporary handoff is in force inside its declared window; the card cites the temporary-handoff record. |
| `temporary_handoff_nearing_expiry` | The temporary handoff is in force but `expires_at` is within the next thirty days; surfaces SHOULD render a near-expiry chip and route renewal or closure work. |
| `temporary_handoff_expired` | The temporary handoff lapsed without renewal; the card MUST render as degraded with `degraded_handoff_expired`. |
| `owner_unresolved_degraded` | The accountable role does not resolve to an active owner through the ownership matrix; the card MUST render as degraded with `degraded_owner_unresolved`. |
| `blocked_escalated` | Ownership is contested or blocked pending a council / shiproom decision; the card MUST cite the relevant `decision_forum_ref` and route to one of the council escalation paths. |

A card whose accountability class cannot be typed denies with
`accountability_status_class_unresolved` rather than defaulting to
`active_owner_no_handoff`.

## 5. Ownership-freshness vocabulary

Closed five-class vocabulary:

| Class | Meaning |
| --- | --- |
| `fresh` | Ownership was reaffirmed inside the declared review window; the card is current. |
| `near_expiry` | Ownership reaffirmation is within the next review window; surfaces SHOULD prompt the lane DRI to confirm. |
| `stale_by_time` | Ownership has not been reaffirmed inside the declared review window; the card is degraded and MUST NOT be projected as fresh on any consuming surface. |
| `stale_by_trigger` | A named rerun trigger fired (lane re-scope, protected-path promotion, support-class change, security egress class change, decision-rights matrix update) and the card has not been reaffirmed against the new state. |
| `not_applicable_provisional` | The lane is provisional (e.g. a deferred host-class anchor); ownership is provisionally claimed but not yet reaffirmed against a frozen lane definition. |

`stale_by_time` and `stale_by_trigger` MUST resolve to one of the
degraded-state labels in §7. A card cannot be `fresh` and degraded
simultaneously.

## 6. Surface-identity vocabulary

The closed nine-class surface-identity vocabulary names what kind of
surface the card grades. Each class carries different reuse rules
(§9):

| Class | Meaning | Reuse class floor |
| --- | --- | --- |
| `subsystem` | A code subsystem / crate (renderer, buffer, VFS, RPC transport, telemetry foundation, shell / command system). | required when the subsystem is on a protected path. |
| `runtime_host_class` | A runtime host class from `service_ownership_matrix.yaml` (desktop_shell, local_supervisor, index_workers, ...). | required. |
| `public_truth_area` | A public-truth area (docs, README, AGENTS.md, known-limits matrix, support-window statements). | required. |
| `supportability_family` | A supportability artifact family (support bundles, doctor probes, recovery ladder, support-export schema). | required. |
| `packet_family` | A governance / release packet family (architecture-freeze packet, claim-publication packet, stable-promotion packet, ...). | required. |
| `review_family` | A standing review or decision forum family (architecture council, performance council, release council, shiproom). | required. |
| `protected_metric_or_fitness_function` | A row in the protected fitness-function catalog or protected-metrics register. | required. |
| `compatibility_or_certified_archetype_claim` | A compatibility row, certified-archetype row, or claim-manifest row. | required. |
| `security_sensitive_egress_class` | A security-sensitive egress / transport posture class (network egress, signed-pack publication, code-signing chain, transport-trust posture, AI-broker outbound class). | required. |

A card whose surface class is `subsystem` and whose
`protected_path_ref` resolves to a non-protected lane MAY downgrade
its reuse class to `recommended`; every other class is required.

## 7. Degraded-state-label vocabulary

The closed five-class degraded-state-label vocabulary mirrors the
two named degraded states from
[`decision_rights_and_signoff_matrix.md` §103](./decision_rights_and_signoff_matrix.md)
and adds the freshness-degradation states:

| Class | Meaning |
| --- | --- |
| `not_degraded_active` | Card is actively owned and fresh; no degraded state. Required when `accountability_status_class = active_owner_no_handoff` or `active_owner_with_backup_waiver` and `ownership_freshness_class = fresh`. |
| `degraded_handoff_expired` | A temporary handoff lapsed without renewal; the card MUST NOT render as actively owned. |
| `degraded_owner_unresolved` | The accountable role does not resolve to an active owner through the ownership matrix. |
| `degraded_freshness_stale` | Ownership has not been reaffirmed inside the declared window or a rerun trigger fired; the card MUST be reviewed before it appears on any release- or claim-bearing surface. |
| `degraded_blocked_escalated` | Ownership is contested or blocked pending a council / shiproom decision; surfaces MUST render the escalation chip and the linked decision forum. |

Schema-enforced pairings:

- `accountability_status_class = active_owner_no_handoff` and
  `ownership_freshness_class = fresh` requires
  `degraded_state_label_class = not_degraded_active`.
- `accountability_status_class = temporary_handoff_expired` requires
  `degraded_state_label_class = degraded_handoff_expired`.
- `accountability_status_class = owner_unresolved_degraded` requires
  `degraded_state_label_class = degraded_owner_unresolved`.
- `accountability_status_class = blocked_escalated` requires
  `degraded_state_label_class = degraded_blocked_escalated`.
- `ownership_freshness_class` in
  `{stale_by_time, stale_by_trigger}` requires
  `degraded_state_label_class = degraded_freshness_stale` (or one of
  the more-specific degraded labels above when both apply).

## 8. Role-based export rules

Closed five-class `role_export_class` vocabulary, applied per
audience:

| Class | Meaning |
| --- | --- |
| `role_id_only` | Only the role id, the role label, and the lane scope are exposed. Personal contact details, private schedules, and on-call calendars MUST NOT appear. |
| `role_id_with_decision_forum` | Role id plus the linked decision forum and the escalation path. Used for shiproom and release packets. |
| `role_id_with_named_backup_or_waiver` | Role id, named backup id, and the active backup waiver ref. Used for the support bundle and enterprise review so coverage is verifiable without exposing schedules. |
| `role_id_with_temporary_handoff_summary` | Role id plus the open temporary-handoff record's start date, expiry, closure path, and degraded-state label. Used when the card is in `temporary_handoff_active`, `temporary_handoff_nearing_expiry`, or `temporary_handoff_expired`. |
| `redacted_role_class_only_no_role_id` | Only the role class (e.g. "an architecture-council reviewer") is exposed. Used for public claim manifests and external compatibility reports where individual role IDs are out of scope. |

The card carries one `role_export_class` value per audience. The
schema enforces:

- the public claim manifest MUST NOT exceed
  `redacted_role_class_only_no_role_id`;
- the support bundle MUST NOT exceed
  `role_id_with_named_backup_or_waiver` or, when a temporary handoff
  is active, `role_id_with_temporary_handoff_summary`;
- the shiproom and release packet MUST NOT exceed
  `role_id_with_decision_forum` or, when a temporary handoff is
  active, `role_id_with_temporary_handoff_summary`;
- the enterprise / managed-tenant review MUST NOT exceed
  `role_id_with_named_backup_or_waiver` or, when a temporary handoff
  is active, `role_id_with_temporary_handoff_summary`;
- raw email addresses, raw phone numbers, raw calendar URLs, raw
  chat-room URLs, raw on-call rotation entries, and raw private
  meeting refs MUST NOT appear on any surface.

## 9. Reuse-class vocabulary

Closed three-class `reuse_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `required_named_accountable_ownership` | Surface MUST carry a card. Promotion is blocked if the card is missing, degraded, or expired. Required for protected paths, public-truth areas, supportability families, packet families, review families, protected-metric / fitness-function rows, compatibility / certified-archetype claims, security-sensitive egress classes, and runtime host classes. |
| `recommended_named_accountable_ownership` | Surface SHOULD carry a card; absence is a signal but not a blocker. Used for non-protected subsystems and internal-tooling lanes. |
| `not_applicable_internal_tooling` | Surface is internal tooling not subject to the ownership-card contract (throwaway spike, scratch fixture, build-only helper). |

Schema-enforced pairings:

- `surface_identity_class` in
  `{runtime_host_class, public_truth_area, supportability_family,
   packet_family, review_family, protected_metric_or_fitness_function,
   compatibility_or_certified_archetype_claim,
   security_sensitive_egress_class}` requires
  `reuse_class = required_named_accountable_ownership`.
- `surface_identity_class = subsystem` with a non-null
  `protected_path_ref` requires
  `reuse_class = required_named_accountable_ownership`.
- `reuse_class = required_named_accountable_ownership` requires a
  non-empty `linked_evidence_refs` array, a non-null
  `accountable_ownership.primary_owner_role_ref`, and a non-null
  `escalation_route.decision_forum_ref`.
- `reuse_class = required_named_accountable_ownership` forbids
  `accountability_status_class = owner_unresolved_degraded` and
  `degraded_state_label_class = degraded_owner_unresolved` from
  rendering as a clean / non-degraded card on any consuming surface.

## 10. Export parity

Every surface that renders the card MUST render the same fields. The
parity floor is enforced by the schema's `allOf` block.

Required on every consuming surface:

- `service_ownership_card_id`,
  `surface_identity.surface_identity_class`,
  `surface_identity.headline_label`;
- `accountable_ownership.primary_owner_role_ref`,
  `accountable_ownership.accountability_status_class`,
  `current_accountability_status`;
- `support_posture.support_class`,
  `support_posture.support_posture_summary`;
- `escalation_route.escalation_path_class`,
  `escalation_route.decision_forum_ref`;
- `freshness.ownership_freshness_class`,
  `freshness.expires_at`;
- `degraded_state_label_class`,
  `degraded_state_summary`;
- `reuse_class`.

Per-state required extras:

- `temporary_handoff_active`, `temporary_handoff_nearing_expiry`, and
  `temporary_handoff_expired` — at least one entry in
  `temporary_handoff_refs`.
- `active_owner_with_backup_waiver` —
  `accountable_ownership.linked_waiver_ref` MUST be non-null.
- `blocked_escalated` —
  `escalation_route.decision_forum_ref` MUST be non-null and the
  `escalation_path_class` MUST be one of `escalate_to_decision_forum`,
  `escalate_to_release_council`, or `escalate_to_shiproom`.
- ownership-freshness in `{stale_by_time, stale_by_trigger}` —
  `freshness.rerun_trigger_ref` MUST be non-null when the class is
  `stale_by_trigger`.

Forbidden collapses on release-packet, support-packet, claim-
manifest, and enterprise-review surfaces:

- rendering `temporary_handoff_expired` or `owner_unresolved_degraded`
  as a clean active-owner card to unblock publication;
- dropping `escalation_route` so a contested ownership row reads as
  ambiguous "review pending";
- dropping `degraded_state_label` so chronic ownership drift looks
  like an isolated incident;
- exposing personal contact details, private schedules, on-call
  calendars, raw email addresses, raw phone numbers, raw chat-room
  URLs, or raw meeting refs;
- emitting the role id at a `role_export_class` higher than the
  audience's permitted ceiling (§8).

## 11. Temporary-handoff record shape

A `temporary_handoff_record` carries:

- `temporary_handoff_id` — stable, machine-readable id quoted by
  every consuming surface and by the parent ownership card.
- `service_ownership_card_ref` — opaque ref into the parent
  `service_ownership_card_record`. Required.
- `primary_owner_role_ref` — role id of the primary DRI the handoff
  is replacing. Required.
- `temporary_owner_role_ref` — role id of the temporary owner the
  handoff routes to. Required.
- `handoff_chronology` — `started_at`, `intended_expires_at`, the
  optional `closed_at`, and the `evaluated_at` projection timestamp.
  `intended_expires_at` is required and MUST be in the future at
  `started_at`.
- `temporary_handoff_status_class` — typed status (§12).
- `linked_evidence_refs` — refs into the supporting evidence
  (council decision row, leave-coverage waiver, recused-reviewer
  decision packet, lab-outage incident packet). At least one entry
  is required.
- `linked_waiver_refs` — refs into waivers in the ownership matrix
  that gate the handoff (zero, one, or many). Required to be
  non-empty when `temporary_handoff_status_class` is
  `handoff_active_under_waiver` or
  `handoff_nearing_expiry_under_waiver`.
- `closure_or_renewal_path_class` — typed closure / renewal path
  (§13).
- `escalation_path_class` — typed escalation path mirroring
  [`waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  (`escalate_to_dri`, `escalate_to_backup_owner`,
  `escalate_to_decision_forum`, `escalate_to_release_council`,
  `escalate_to_shiproom`, `escalate_to_support`,
  `no_escalation_required`). Required.
- `degraded_state_label_class` — typed degraded label mirroring §7.
  Required.
- `handoff_summary` — bounded reviewable sentence naming the
  primary owner role, the temporary owner role, the start date, the
  intended expiry, and the closure / renewal path. Personal contact
  details, private schedules, and calendar URLs MUST NOT appear.
- `export_fields` — typed booleans for dashboard, scorecard, release
  packet, support export, governance packet, and enterprise review.

## 12. Temporary-handoff status vocabulary

The closed seven-class temporary-handoff status vocabulary:

| Class | Meaning |
| --- | --- |
| `handoff_active_inside_window` | The handoff is in force inside its declared window; `intended_expires_at` is more than thirty days away. |
| `handoff_nearing_expiry_within_thirty_days` | The handoff is in force but `intended_expires_at` is within the next thirty days. |
| `handoff_nearing_expiry_within_seven_days` | The handoff is in force but `intended_expires_at` is within the next seven days; surfaces MUST render the near-expiry chip prominently. |
| `handoff_active_under_waiver` | The handoff is in force and gated by an explicit waiver in the ownership matrix; `linked_waiver_refs` MUST be non-empty. |
| `handoff_expired_past_due` | The handoff lapsed without renewal; the parent card MUST render as degraded with `degraded_handoff_expired`. |
| `handoff_closed_with_renewal` | The handoff closed via renewal; a successor `temporary_handoff_record` carries the new window. |
| `handoff_closed_returned_to_primary` | The handoff closed by the primary DRI returning; the parent card returns to `active_owner_no_handoff` or `active_owner_with_backup_waiver`. |

A handoff whose status cannot be typed denies with
`temporary_handoff_status_class_unresolved` rather than defaulting.

Schema-enforced pairings:

- `handoff_active_inside_window`,
  `handoff_nearing_expiry_within_thirty_days`,
  `handoff_nearing_expiry_within_seven_days`, and
  `handoff_active_under_waiver` require `closed_at` to be null.
- `handoff_expired_past_due` requires
  `intended_expires_at` strictly in the past relative to
  `evaluated_at` and `closed_at` to be null.
- `handoff_closed_with_renewal` and
  `handoff_closed_returned_to_primary` require a non-null
  `closed_at`.
- `handoff_active_under_waiver` requires `linked_waiver_refs` to be
  non-empty.

## 13. Closure-or-renewal-path vocabulary

The closed six-class `closure_or_renewal_path_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `return_to_primary_dri` | The handoff closes when the primary DRI returns. Default for vacation, leave, and short-window handoffs. |
| `renew_handoff_with_named_expiry` | The handoff renews into a successor record with a new `intended_expires_at`. |
| `escalate_to_decision_forum` | Closure routes through the lane's decision forum (architecture council, performance council, security and trust review, accessibility review, ecosystem and compatibility review, milestone scope review, open community sync). |
| `escalate_to_release_council` | Closure routes through the release council; used for release-bearing surfaces. |
| `escalate_to_shiproom` | Closure routes through shiproom / executive scope review; used when the handoff blocks a stable promotion or LTS-line decision. |
| `retire_via_correction_program` | Closure retires the lane (or the surface) via a named correction program; the card transitions to `not_applicable_internal_tooling` or is removed in the same change. |

Schema-enforced pairings:

- `escalate_to_decision_forum`, `escalate_to_release_council`, and
  `escalate_to_shiproom` require the parent card to cite a non-null
  `decision_forum_ref` and an `escalation_path_class` from the same
  council family.
- `renew_handoff_with_named_expiry` requires a successor
  `temporary_handoff_id` to be cited in the closing record's
  `successor_handoff_ref` (admissible only on records whose
  `temporary_handoff_status_class = handoff_closed_with_renewal`).

## 14. Reuse rules for protected paths, public surfaces, security-sensitive egress, and compatibility claims

The card's `reuse_class` floor (§9) requires
`required_named_accountable_ownership` for all of the following:

1. **Protected paths** — every crate marked `protected_path: true`
   in [`/artifacts/governance/package_inventory.yaml`](../../artifacts/governance/package_inventory.yaml),
   plus the non-code lanes named protected in
   [`./dri_map.md`](./dri_map.md). Promotion of a non-protected lane
   into protected status MUST land an ownership card in the same
   change.
2. **Public-truth surfaces** — every public-truth area (`/docs/`,
   `/README.md`, `/AGENTS.md`, `/CLAUDE.md`, known-limits matrix,
   support-window statements, migration guides) and every public
   claim row. Cards on public-truth surfaces MUST cite a non-null
   `support_window_badge_ref` and the `role_export_class` for the
   public claim manifest MUST be
   `redacted_role_class_only_no_role_id`.
3. **Security-sensitive egress classes** — every named security-
   sensitive egress / transport posture class (network egress,
   signed-pack publication, code-signing chain, transport-trust
   posture, AI-broker outbound class, support-export upload class).
   Cards on egress classes MUST cite the security-and-trust review
   forum, MUST resolve to a `support_class` of `certified` or
   `supported` for the positive-claim case, and MUST render
   degraded under any other support class.
4. **Compatibility claims** — every compatibility row in
   [`/artifacts/release/`](../../artifacts/release/) and every
   certified-archetype row. Cards on compatibility claims MUST cite
   the compatibility / ecosystem review forum and MUST link the
   underlying compatibility report or certified-archetype report in
   `linked_evidence_refs`.

Cards on internal-tooling lanes (the shell spike, throwaway scratch
fixtures, build-only helpers) are admissible at
`reuse_class = not_applicable_internal_tooling`. The schema does
not require a card on those lanes; the schema does require that
**no lane downgrades from `required_named_accountable_ownership` to
`not_applicable_internal_tooling` without an architecture-council
decision row** so a protected path cannot silently lose its card.

## 15. Authoring rules

When a new lane, surface, or claim row is promoted into a state that
requires a card:

1. Mint a `service_ownership_card_record` with the typed
   `surface_identity_class`, the `surface_ref`, and the
   `protected_path_ref` (when the surface sits on a protected path).
2. Resolve `accountable_ownership` against the ownership matrix; the
   role id MUST resolve to an active owner. If it cannot, the card
   is degraded and the same change MUST land either a closing
   correction or a temporary handoff record.
3. Resolve `support_posture` against the
   [`support_window_badge.schema.json`](../../schemas/release/support_window_badge.schema.json)
   record for the surface. Public-truth, release, and claim-bearing
   surfaces require a non-null `support_window_badge_ref`.
4. Resolve `escalation_route` against the forum register; the
   `decision_forum_ref` MUST be a forum id from
   [`forum_charters.md`](./forum_charters.md).
5. Wire `freshness` against the live evaluation time; surfaces
   MUST recompute `ownership_freshness_class` every time the card
   is rendered.
6. Resolve `linked_evidence_refs` against the underlying packet,
   scorecard lane, signoff row, certified-archetype report,
   compatibility row, claim-manifest row, fitness-function row,
   protected-metric row, or waiver record.
7. Set `role_export_rules` per audience using the §8 ceilings.

When a temporary handoff is opened:

1. Mint a `temporary_handoff_record` with the
   `service_ownership_card_ref`, the `primary_owner_role_ref`, the
   `temporary_owner_role_ref`, the `started_at`, and the
   `intended_expires_at`.
2. Update the parent card's `accountability_status_class` to
   `temporary_handoff_active` and add the handoff id to
   `temporary_handoff_refs`.
3. Resolve `closure_or_renewal_path_class` against the council /
   shiproom routing.
4. Resolve `degraded_state_label_class` for the handoff; while the
   handoff is active and inside its window the parent card stays
   non-degraded, but the handoff record's degraded-label class is
   `not_degraded_active` only when the handoff is freshly minted.

When a temporary handoff lapses:

1. Update the handoff's
   `temporary_handoff_status_class` to
   `handoff_expired_past_due` and recompute the
   `degraded_state_label_class` to `degraded_handoff_expired`.
2. Update the parent card's
   `accountability_status_class` to `temporary_handoff_expired`,
   recompute `degraded_state_label_class` to
   `degraded_handoff_expired`, and route the closure / renewal path
   per §13.
3. The card MUST NOT render as actively owned on any consuming
   surface until the handoff renews (new record), the primary DRI
   returns (`handoff_closed_returned_to_primary`), or the lane is
   retired (`retire_via_correction_program`).

## 16. Out of scope

This contract does not implement:

- an org-chart tool, a paging or on-call rotation system, a
  calendar / availability surface, or an HR roster;
- the council decision rights themselves (those live in
  [`./decision_rights_and_signoff_matrix.md`](./decision_rights_and_signoff_matrix.md)
  and the signoff matrix);
- the underlying waiver records (those live in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  and on
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json));
- the runtime host-class ownership matrix (that lives in
  [`/artifacts/architecture/service_ownership_matrix.yaml`](../../artifacts/architecture/service_ownership_matrix.yaml));
- the support-class admission map (that lives in
  [`/docs/release/channel_support_window_contract.md`](../release/channel_support_window_contract.md)
  and
  [`/schemas/release/support_window_badge.schema.json`](../../schemas/release/support_window_badge.schema.json)).

This contract is the projection vocabulary that those upstream
artifacts flow through when ownership has to be visible and
exportable on the dashboard, the milestone scorecard, the release-
evidence shiproom packet, the support bundle, the claim manifest,
the compatibility-window report, and the enterprise / managed-
tenant review.
