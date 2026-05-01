# Publisher-lifecycle, revocation / quarantine, and private-registry parity contract

This document is the narrative companion to the publisher-lifecycle and
registry-parity boundary schemas at
[`/schemas/extensions/publisher_lifecycle_event.schema.json`](../../schemas/extensions/publisher_lifecycle_event.schema.json)
and
[`/schemas/extensions/registry_parity.schema.json`](../../schemas/extensions/registry_parity.schema.json),
and to the worked fixtures under
[`/fixtures/extensions/publisher_lifecycle_cases/`](../../fixtures/extensions/publisher_lifecycle_cases).
It freezes the controlled vocabulary that every discovery, detail,
install-review, installed-row, support-export, and moderation review
surface MUST project across public, private, mirror, offline-bundle,
and local-archive lanes for publisher-lifecycle events and registry
parity. The schemas are authoritative when the narrative and the
schemas disagree; this document MUST be updated in the same change
that lands any schema bump.

The contract is deliberately narrow. It does **not** land a publisher-
trust service, a moderation pipeline, a key-management implementation,
a private-registry administration UX, an installed-row updater, or an
abuse-investigation tool. Its job is to freeze the publisher-lifecycle
event vocabulary, the installed-package fan-out vocabulary, and the
registry-parity vocabulary early enough that ranking, badges, and
warnings remain first-class once verification, transfer, suspension,
abandonment, revocation (administrative or emergency), quarantine,
restoration, mirror promotion, artifact-level emergency disable /
revocation, and parity audits build against it.

The contract reuses the closed vocabulary frozen by the
[`marketplace ranking, trust badge, and anti-abuse vocabulary contract`](marketplace_ranking_and_trust_contract.md),
the
[`registry / mirror / offline-bundle / local-archive seed`](registry_and_offline_bundle_seed.md),
and the anti-abuse register at
[`artifacts/extensions/anti_abuse_states.yaml`](../../artifacts/extensions/anti_abuse_states.yaml).
Lifecycle events extend, never replace, those vocabularies.

## What this contract freezes

1. A **closed publisher-lifecycle-event vocabulary** (19 classes,
   schema §`lifecycle_event_class`) covering verification (admitted /
   revalidated / lapsed), transfer (announced / completed),
   suspension (pending review / lifted), abandonment (observed /
   cleared), revocation (emergency / administrative), quarantine
   (engaged / cleared), restoration after review, artifact-level
   emergency disable (engaged / cleared), artifact-level revocation,
   and mirror promotion (admitted / revoked). Every event names a
   typed actor class, a typed reason class, and an effective date.
2. A **closed actor-class vocabulary** (9 classes, schema
   §`lifecycle_actor_class`) covering registry / mirror / private-
   registry operators, workspace admins, moderation review,
   automated continuity / anti-abuse detectors, and publisher /
   successor-publisher self-service.
3. A **closed reason-class vocabulary** (21 classes, schema
   §`lifecycle_reason_class`) covering identity proof, signing-key
   continuity / compromise, publisher-org dissolution, successor
   acknowledgement, inactivity, moderation review outcome, emergency
   safety review, supply-chain compromise, namespace collision,
   mirror continuity, private-registry admin decision, policy-pack
   constraint change, workspace trust, and the honest unknown
   default.
4. A **closed effect-on-trust-badge vocabulary** (8 classes, schema
   §`lifecycle_effect_on_trust_badge_class`) and matching effect-on-
   compatibility-badge (4 classes), effect-on-runtime-cost-badge (4
   classes), and effect-on-anti-abuse-state (5 classes) vocabularies.
   These classes pin how a lifecycle event mutates the rendered
   badges and the engaged anti-abuse register rows on every consumer
   surface.
5. A **closed installed-package notification-disposition vocabulary**
   (10 classes, schema §`installed_package_notification_disposition_class`)
   covering notify-only, optional update, required update, required
   review, install-disabled-pending-review, uninstall-offered,
   uninstall-required-no-recovery, workspace-admin review,
   managed-only continuation, and the honest unknown default.
6. A **closed installed-package repair-affordance vocabulary** (13
   classes, schema §`installed_package_repair_affordance_class`)
   covering run-update / open-review / open-publisher-continuity /
   open-permission-inspector / open-runtime-status-pill / open-
   policy-pack-constraint / consult-admin / request-policy-exception
   / uninstall-now / uninstall-now-no-recovery / acknowledge-
   disclosure / no-recovery-review-only.
7. A **closed lifecycle denial-reason vocabulary** (14 classes,
   schema §`lifecycle_denial_reason_class`) extending the discovery /
   registry denial vocabulary with publisher-lifecycle-shaped
   reasons (publisher-revocation-emergency, publisher-revocation-
   administrative, publisher-quarantine-engaged, publisher-
   suspension-pending-review, publisher-transfer-pending-user-
   acknowledgement, publisher-transfer-pending-admin-policy-
   exception, installed-package-stale-high-trust-badge, lifecycle-
   event-unverified-actor, lifecycle-event-missing-typed-reason,
   lifecycle-event-missing-effective-at, review-disclosure-
   incomplete).
8. A **closed registry-parity vocabulary** (10 axis classes, 12
   parity-state classes, 6 consequence classes, 12 evidence classes,
   schema §`parity_*`) requiring enterprise / private registries,
   approved mirrors, offline bundles, and local archives to use the
   exact same trust, provenance, compatibility, runtime-cost,
   safety-policy, maintainership, bridge-state, anti-abuse, and
   publisher-lifecycle vocabulary as public discovery surfaces, while
   still naming the registry source class directly via the dedicated
   `registry_source_naming_axis`.

## Record kinds

| Record kind                                          | Schema                                                                                                                                  | Purpose                                                                                                                                                  |
|------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `publisher_lifecycle_event_record`                   | [`publisher_lifecycle_event.schema.json`](../../schemas/extensions/publisher_lifecycle_event.schema.json#/$defs/publisher_lifecycle_event_record) | One typed lifecycle event. Carries actor, reason, effective date, prior-event ref, and the projected effect on the trust / compatibility / runtime-cost badges and on the anti-abuse register. |
| `installed_package_lifecycle_notification_record`    | [`publisher_lifecycle_event.schema.json`](../../schemas/extensions/publisher_lifecycle_event.schema.json#/$defs/installed_package_lifecycle_notification_record) | One notification fanned out to one installed copy after a lifecycle event. Carries a typed disposition, repair-affordance set, projected post-event badges, and a typed denial reason. |
| `registry_parity_assertion_record`                   | [`registry_parity.schema.json`](../../schemas/extensions/registry_parity.schema.json#/$defs/registry_parity_assertion_record)                 | One parity assertion that the subject lane (public / mirror / private / offline-bundle / local-archive) projects the same closed vocabulary as public discovery surfaces. Carries one axis assertion per parity_axis_class plus an overall state and consequence. |

## Source-of-truth bindings

A lifecycle event row is **never** the source of truth for publisher
identity, signing keys, attestation bundles, or moderation decisions.
Every event binds back to upstream rows via opaque refs:

- `publisher_continuity_ref` → the ADR-0012 publisher-continuity row
  the event applies to.
- `successor_publisher_continuity_ref` → the successor publisher's
  continuity row on transfer events.
- `subject_registry_endpoint_ref` → the registry / mirror / private-
  registry / offline-bundle endpoint record.
- `subject_registry_manifest_row_ref` → the
  [`registry_manifest_row`](../../schemas/extensions/registry_manifest.schema.json#/$defs/registry_manifest_row)
  for artifact-scoped events.
- `mirror_continuity_row_ref` → the
  [`mirror_continuity_row`](../../schemas/extensions/registry_manifest.schema.json#/$defs/mirror_continuity_row)
  for mirror-promotion events.
- `prior_publisher_lifecycle_event_record_ref` → the previous event
  the current row supersedes (required on every clear / lift /
  restoration / revalidation / mirror-promotion-revoked event).
- `actor_identity_ref` → the actor decision row. Raw user identity,
  raw email addresses, and raw repository paths are never carried.
- `evidence_refs` → signing-continuity row, attestation-bundle ref,
  moderation review row, anti-abuse signal entry source, mirror
  continuity row, policy-pack constraint row.
- `anti_abuse_state_row_refs` → rows from
  [`anti_abuse_states.yaml`](../../artifacts/extensions/anti_abuse_states.yaml)
  the event engages, widens, narrows, or clears.
- `policy_pack_applicable_refs` → the ADR-0012 policy-pack constraint
  rows whose scope covers the publisher / artifact at the moment of
  the event.

A registry-parity record similarly binds back to the registry-manifest
row, the publisher-continuity row, the lifecycle event records inside
the parity window, the anti-abuse state rows engaged on the subject
row, and the discovery-result row the subject row projects onto.

## Lifecycle event classes

The closed 19-class set:

| Lifecycle event class                                | Default actor class                                       | Default reason class                                        | Default effect on trust badge                                          |
|------------------------------------------------------|-----------------------------------------------------------|-------------------------------------------------------------|------------------------------------------------------------------------|
| `publisher_verification_admitted`                    | `registry_operator` / `private_registry_administrator`    | `identity_proof_completed`                                  | `trust_badge_promoted_to_verified` / `_managed_approved` / `_organisational` |
| `publisher_verification_revalidated`                 | `registry_operator`                                       | `identity_proof_completed`                                  | `trust_badge_unchanged` / `_promoted_to_verified`                      |
| `publisher_verification_lapsed`                      | `automated_publisher_continuity_detector`                 | `identity_proof_lapsed`                                     | `trust_badge_demoted_to_organisational` / `_demoted_to_unverified`     |
| `publisher_transfer_announced`                       | `publisher_self_service`                                  | `successor_publisher_acknowledged`                          | `trust_badge_unchanged` (announce only)                                |
| `publisher_transfer_completed`                       | `successor_publisher_self_service` / `registry_operator`  | `successor_publisher_acknowledged`                          | `trust_badge_demoted_to_organisational` / `_demoted_to_unverified`     |
| `publisher_suspension_pending_review`                | `moderation_review` / `automated_anti_abuse_detector`     | `moderation_review_concluded_block` / `emergency_safety_review` | `trust_badge_demoted_to_unverified` / `_forced_quarantined_cannot_inherit` |
| `publisher_suspension_lifted`                        | `moderation_review`                                       | `moderation_review_concluded_admit`                         | restorative; ≠ `trust_badge_unchanged` (cleared)                       |
| `publisher_abandonment_observed`                     | `automated_publisher_continuity_detector`                 | `publisher_inactivity_observed`                             | `trust_badge_unchanged` (advisory)                                     |
| `publisher_abandonment_cleared`                      | `automated_publisher_continuity_detector`                 | `publisher_inactivity_cleared`                              | `trust_badge_unchanged` (advisory cleared)                             |
| `publisher_revocation_emergency`                     | `registry_operator` / `moderation_review`                 | `signing_key_compromised` / `emergency_safety_review`       | `trust_badge_demoted_to_unverified` / `_forced_quarantined_cannot_inherit` |
| `publisher_revocation_administrative`                | `registry_operator` / `private_registry_administrator`    | `publisher_org_dissolution` / `private_registry_admin_decision` | `trust_badge_demoted_to_unverified` / `_forced_quarantined_cannot_inherit` |
| `publisher_quarantine_engaged`                       | `automated_anti_abuse_detector` / `moderation_review`     | `emergency_safety_review` / `signing_key_compromised`       | `trust_badge_forced_quarantined_cannot_inherit`                        |
| `publisher_quarantine_cleared`                       | `moderation_review`                                       | `moderation_review_concluded_admit`                         | restorative                                                            |
| `publisher_restoration_after_review`                 | `moderation_review`                                       | `moderation_review_concluded_admit` / `successor_publisher_review_completed` | `trust_badge_promoted_to_*` / `_cleared_after_restoration`             |
| `artifact_emergency_disable_engaged`                 | `registry_operator` / `private_registry_administrator`    | `artifact_supply_chain_compromise` / `emergency_safety_review` | `trust_badge_demoted_to_unverified` / `_forced_quarantined_cannot_inherit` |
| `artifact_emergency_disable_cleared`                 | `moderation_review`                                       | `moderation_review_concluded_admit`                         | restorative                                                            |
| `artifact_revocation_engaged`                        | `registry_operator` / `private_registry_administrator`    | `signing_key_compromised` / `artifact_supply_chain_compromise` | `trust_badge_demoted_to_unverified`                                    |
| `mirror_promotion_admitted`                          | `mirror_operator`                                         | `mirror_continuity_recertified`                             | `trust_badge_unchanged` / `_demoted_to_unverified`                     |
| `mirror_promotion_revoked`                           | `mirror_operator` / `registry_operator`                   | `mirror_continuity_lost`                                    | `trust_badge_demoted_to_unverified`                                    |

Schema gates pin the constraints below; surfaces that violate them
deny with `lifecycle_event_unverified_actor`,
`lifecycle_event_missing_typed_reason`, or
`installed_package_stale_high_trust_badge` as appropriate.

- `publisher_verification_admitted` MUST promote the trust badge to
  verified, managed_approved, or organisational. It MUST NOT leave
  the badge unchanged or demote it.
- `publisher_quarantine_engaged` MUST force
  `trust_badge_forced_quarantined_cannot_inherit` and engage / widen
  the anti-abuse register.
- `publisher_revocation_*`, `publisher_suspension_pending_review`,
  `artifact_revocation_engaged`, and
  `artifact_emergency_disable_engaged` MUST demote the trust badge
  to `_demoted_to_unverified` or `_forced_quarantined_cannot_inherit`.
  Leaving the badge unchanged is non-conforming.
- `publisher_transfer_completed` MUST cap the rendered trust badge at
  organisational (`_demoted_to_organisational`) or unverified
  (`_demoted_to_unverified`) until the successor publisher row clears
  its first verification review.
- `publisher_restoration_after_review`,
  `publisher_quarantine_cleared`, `publisher_suspension_lifted`,
  `publisher_abandonment_cleared`,
  `artifact_emergency_disable_cleared`, `mirror_promotion_revoked`,
  and `publisher_verification_revalidated` MUST cite a non-null
  `prior_publisher_lifecycle_event_record_ref` so the cleared event
  resolves back to the engaging event.
- `publisher_restoration_after_review` MUST NOT leave the trust badge
  unchanged.
- `publisher_quarantine_cleared`,
  `publisher_suspension_lifted`, and
  `artifact_emergency_disable_cleared` MUST narrow or clear the anti-
  abuse state, never widen or engage it.
- `mirror_promotion_admitted` and `mirror_promotion_revoked` MUST
  cite a non-null `mirror_continuity_row_ref` and have
  `subject_registry_source_class = approved_mirror`.
- `mirror_promotion_admitted` MUST NOT widen the rendered trust badge
  above the origin tier; allowed effect-on-trust-badge classes are
  `trust_badge_unchanged` and `trust_badge_demoted_to_unverified`.
- Artifact-scoped events
  (`artifact_revocation_engaged`,
  `artifact_emergency_disable_engaged`,
  `artifact_emergency_disable_cleared`) MUST cite non-null values for
  `subject_extension_identity`, `subject_extension_version`, and
  `subject_registry_manifest_row_ref`.
- Publisher-transfer events MUST cite a non-null
  `successor_publisher_continuity_ref`.
- Publisher-revocation, publisher-quarantine-engaged, publisher-
  suspension-pending-review, artifact-revocation, artifact-emergency-
  disable-engaged, and publisher-transfer-completed events MUST set
  `installed_package_notification_required = true` so installed
  copies cannot retain stale high-trust badges.
- Publisher-revocation, publisher-quarantine-engaged, publisher-
  suspension-pending-review, artifact-revocation, and artifact-
  emergency-disable-engaged events MUST cite a typed
  `denial_reason_class` other than `no_denial_admissible`.

## Installed-package fan-out

Every lifecycle event whose `installed_package_notification_required`
is true MUST emit one
`installed_package_lifecycle_notification_record` per installed copy.
The notification reads the typed effect classes from the lifecycle
event and projects them as the rendered post-event badges on the
installed-row card. Surfaces MUST NOT compute the post-event badges
independently — installed-row code reads the notification record
verbatim.

| Disposition class                                    | Default repair-affordance set                                                                                            | Default denial reason class                              |
|------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------|
| `notify_only_no_action_required`                     | `acknowledge_disclosure`                                                                                                 | `no_denial_admissible`                                   |
| `notify_with_optional_update_offered`                | `run_offered_update_now`, `acknowledge_disclosure`                                                                       | `no_denial_admissible`                                   |
| `notify_with_required_update_pending`                | `run_required_update_now`, `open_install_review_sheet`                                                                   | `publisher_transfer_pending_user_acknowledgement` / typed |
| `notify_with_required_review_pending`                | `open_install_review_sheet`, `consult_workspace_admin`                                                                   | typed (e.g. `publisher_suspension_pending_review`)       |
| `notify_with_install_disabled_pending_review`        | `open_install_review_sheet`, `consult_workspace_admin`                                                                   | typed (e.g. `publisher_quarantine_engaged`)              |
| `notify_with_uninstall_offered`                      | `uninstall_now`, `consult_workspace_admin`                                                                               | typed                                                    |
| `notify_with_uninstall_required_no_recovery`         | `uninstall_now_no_recovery`, `no_recovery_review_only`, `consult_workspace_admin`                                        | typed (e.g. `artifact_revocation_engaged`)               |
| `notify_with_workspace_admin_review_required`        | `consult_workspace_admin`, `request_admin_policy_exception`, `open_policy_pack_constraint_row`                           | typed                                                    |
| `notify_with_managed_only_continuation`              | `consult_workspace_admin`                                                                                                | `no_denial_admissible`                                   |
| `notify_class_unknown_requires_review`               | `consult_workspace_admin`                                                                                                | `review_disclosure_incomplete`                           |

Schema gates pin:

- `notify_with_uninstall_required_no_recovery` MUST cite both
  `no_recovery_review_only` and `uninstall_now_no_recovery` in the
  repair-affordance set, and MUST cite a typed denial reason.
- `notify_with_install_disabled_pending_review` and
  `notify_with_required_review_pending` MUST cite a typed denial
  reason.
- `notify_only_no_action_required` MUST be paired with
  `denial_reason_class = no_denial_admissible` AND MUST NOT carry
  `trust_badge_after_event_class` of
  `trust_badge_demoted_to_unverified` or
  `trust_badge_forced_quarantined_cannot_inherit`. A revocation /
  quarantine / emergency-disable / suspension event that resolves the
  installed row to `notify_only_no_action_required` is denied with
  `installed_package_stale_high_trust_badge`.
- `trust_badge_after_event_class` of
  `trust_badge_demoted_to_unverified` or
  `trust_badge_forced_quarantined_cannot_inherit` MUST NOT pair with
  `notify_only_no_action_required` or
  `notify_with_optional_update_offered`. The user / admin always
  gets a typed next step.
- Installed copies sourced from `local_archive` or
  `quarantined_local_copy` MUST cite a non-null
  `installed_local_archive_ref`. Installed copies sourced from
  `public_registry`, `approved_mirror`, `private_registry`, or
  `offline_bundle` MUST cite a non-null
  `installed_registry_manifest_row_ref`.

## Registry-parity invariants

Every public, private, mirror, offline-bundle, or local-archive lane
MUST emit a `registry_parity_assertion_record` per registry-manifest
row that the lane projects onto a discovery / install-review surface.
The assertion carries:

- `subject_registry_source_class` — names the lane explicitly.
- `subject_registry_endpoint_ref` — the endpoint record (raw URLs
  forbidden).
- `subject_registry_manifest_row_ref` — the registry-manifest row.
- `registry_source_named` — `true` when the lane explicitly names its
  registry-source class on every consumer surface; `false` is non-
  conforming and MUST resolve `parity_overall_state_class` to
  `parity_violated_registry_source_not_named`.
- `trust_badge_inheritance_rule_carried` — mirrors the inheritance
  rule from the registry-manifest row so support-export / moderation
  surfaces can render it directly without redirecting through the
  registry-manifest row.
- `publisher_continuity_ref` — the publisher-continuity row.
- `publisher_lifecycle_event_record_refs` — every lifecycle event
  whose effective_at falls inside the parity window. Hiding any of
  these denies with `review_disclosure_incomplete`.
- `anti_abuse_state_row_refs` — anti-abuse register rows engaged on
  the subject row.
- `discovery_result_row_ref` — the discovery row the subject projects
  onto, or `null` when the subject is suppressed from discovery.
- `axis_assertion_set` — exactly ten typed axis assertions (one per
  `parity_axis_class`).
- `parity_overall_state_class` and `parity_overall_consequence_class`
  — the rolled-up state and consequence.

The ten parity axes:

| Parity axis class                       | Asserts that the lane uses                                                                                                                                                                          |
|-----------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `trust_vocabulary_axis`                 | the closed `trust_claim_source` and `trust_badge_inheritance_rule` vocabulary from registry_manifest.schema.json and the closed trust-badge classes from discovery_badge.schema.json.                |
| `provenance_vocabulary_axis`            | the closed `registry_source_class` and the closed provenance-badge classes; private / mirror / offline-bundle / local-archive lanes never project as `provenance_badge_public_registry`.            |
| `compatibility_vocabulary_axis`         | the closed `compatibility_claim_class` and the closed compatibility-badge classes; private lanes never mint a renamed `internal_compatible` class.                                                  |
| `runtime_cost_vocabulary_axis`          | the closed `runtime_cost_class` plus the `runtime_cost_evidence_class` gating; self-reported counters alone never credit `runtime_cost_low_nominal`.                                                |
| `safety_policy_vocabulary_axis`         | the closed `approval_state_class` and the closed safety-policy-badge classes; private lanes render `safety_policy_badge_revoked` / `_emergency_disabled` / `_policy_blocked` exactly as public lanes do. |
| `maintainership_vocabulary_axis`        | the closed maintainership-badge classes (`active`, `recent`, `unknown`, `abandoned`, `publisher_transfer_recent`); private lanes never mint a renamed `internal_maintained` class.                  |
| `bridge_state_vocabulary_axis`          | the closed `bridge_state_class` from discovery_ranking_reason.schema.json; bridge-required rows render the bridge chip and badge on every lane.                                                     |
| `anti_abuse_vocabulary_axis`            | the closed anti-abuse register at artifacts/extensions/anti_abuse_states.yaml; private lanes cite the same anti-abuse-state ids as public lanes.                                                    |
| `publisher_lifecycle_vocabulary_axis`   | the closed lifecycle-event vocabulary from this contract; private / mirror / offline-bundle / local-archive lanes cite the same lifecycle event records as public lanes.                            |
| `registry_source_naming_axis`           | an explicit `registry_source_class` chip on every consumer surface; private lanes never present as if they were public, and quarantined-local-copy never presents as `local_archive`.               |

The closed parity-state vocabulary:

| Parity state class                                      | Default consequence class                                |
|---------------------------------------------------------|----------------------------------------------------------|
| `parity_held_full_vocabulary_match`                     | `no_consequence_parity_held`                             |
| `parity_held_with_documented_narrowing`                 | `no_consequence_parity_held`                             |
| `parity_held_with_capped_inheritance`                   | `no_consequence_parity_held`                             |
| `parity_violated_minted_marketplace_field`              | `emit_parity_violation_audit_event` / `block_install_*`  |
| `parity_violated_widened_trust_tier`                    | `block_install_pending_review`                           |
| `parity_violated_hidden_warning`                        | `render_disclosure_with_inspectable_explanation` / `block_install_pending_review` |
| `parity_violated_anti_abuse_state_not_cited`            | `block_install_pending_review`                           |
| `parity_violated_lifecycle_event_hidden`                | `block_install_pending_review`                           |
| `parity_violated_registry_source_not_named`             | `remove_from_ranking_review_only`                        |
| `parity_violated_compatibility_vocabulary_renamed`      | `render_disclosure_with_inspectable_explanation`         |
| `parity_violated_runtime_cost_vocabulary_renamed`       | `render_disclosure_with_inspectable_explanation`         |
| `parity_unknown_pending_review`                         | `render_disclosure_with_inspectable_explanation`         |

Schema gates pin:

- `parity_held_*` states pair with
  `parity_overall_consequence_class = no_consequence_parity_held`.
- `parity_violated_*` states pair with a consequence other than
  `no_consequence_parity_held`.
- `registry_source_named = false` MUST pair with
  `parity_overall_state_class = parity_violated_registry_source_not_named`.
- `local_archive` and `quarantined_local_copy` subjects MUST carry
  `trust_badge_inheritance_rule_carried` ∈
  {`capped_at_unverified_on_local_archive`,
  `quarantined_cannot_inherit`}.
- `approved_mirror` subjects MUST carry
  `trust_badge_inheritance_rule_carried` ∈
  {`inherits_origin_tier`,
  `capped_at_community_on_approved_mirror`,
  `quarantined_cannot_inherit`}.
- `private_registry` subjects MUST carry
  `trust_badge_inheritance_rule_carried` ∈
  {`inherits_origin_tier`,
  `capped_at_organisational_on_private_registry`,
  `quarantined_cannot_inherit`}.
- `axis_assertion_set` MUST contain exactly one assertion per
  `parity_axis_class` (the schema enforces ten contains gates).

## Disclosure invariants

- A discovery / detail / install-review / installed-row surface MUST
  render every lifecycle event whose effective_at falls inside the
  rendered window. Hiding any of them denies with
  `review_disclosure_incomplete`.
- A surface MUST render the publisher-lifecycle effect on the trust
  badge inline with the registry source class. Silent retention of a
  high-trust badge after a revocation / quarantine / suspension event
  denies with `installed_package_stale_high_trust_badge`.
- A surface MUST render the post-event compatibility badge class.
  Promoting an installed row from
  `compatibility_badge_demoted_to_unknown_pending_reverification`
  back to `_compatible_after_reverification` without a fresh
  reverification denies with `review_disclosure_incomplete`.
- A surface MUST render the post-event runtime-cost badge class.
  Clearing `runtime_cost_badge_quarantined` without a fresh
  activation-evidence packet denies with
  `review_disclosure_incomplete`.
- A surface MUST render the parity assertion's overall state and
  consequence on the support-export / moderation review surface.
  Hiding a `parity_violated_*` state denies with
  `review_disclosure_incomplete`.
- A private-registry / mirror / offline-bundle / local-archive lane
  MUST name its `registry_source_class` directly on every consumer
  surface (the parity record's
  `registry_source_naming_axis` assertion). A surface that paints a
  private-registry row as if it were a public-registry row denies
  with `parity_violated_registry_source_not_named`.

## Default behaviour for the five acceptance fixtures

The fixtures under
[`/fixtures/extensions/publisher_lifecycle_cases/`](../../fixtures/extensions/publisher_lifecycle_cases)
exercise the contract end-to-end:

- `verified_publisher_transfer.yaml` — a `publisher_transfer_completed`
  event records the transition from `acme-labs` to `acme-labs-next`.
  The trust badge demotes from verified to organisational on every
  installed copy until the successor publisher's first verification
  review clears. Installed-row notification disposition is
  `notify_with_required_review_pending`. The matching parity record
  resolves `parity_held_full_vocabulary_match`.
- `abandoned_publisher.yaml` — a `publisher_abandonment_observed`
  event from the publisher-continuity detector renders the
  `maintainership_badge_abandoned` advisory across discovery and
  installed rows. Disposition is `notify_with_optional_update_offered`
  (the user is informed but install / activation is not blocked); the
  installed copy keeps its prior trust badge but adds the abandoned
  badge.
- `mirror_promoted_package.yaml` — a `mirror_promotion_admitted`
  event records that an `approved_mirror` lane promoted an artifact
  on the strength of a mirror-continuity recertification. Trust badge
  is unchanged (mirror inherits origin tier); the parity assertion
  resolves `parity_held_with_capped_inheritance` because the
  inheritance rule caps at community on approved mirror unless the
  mirror reverified the origin signature with a fresh revocation
  snapshot. Installed-row notification disposition is
  `notify_only_no_action_required`.
- `quarantine_engaged.yaml` — a `publisher_quarantine_engaged` event
  raised by the automated anti-abuse detector forces
  `trust_badge_forced_quarantined_cannot_inherit` on every installed
  copy and engages the
  `publisher_namespace_recently_quarantined_blocking` row in the
  anti-abuse register. Installed-row disposition is
  `notify_with_install_disabled_pending_review`. The parity assertion
  resolves `parity_held_full_vocabulary_match` for the public-
  registry lane.
- `restored_publisher_after_review.yaml` — a
  `publisher_restoration_after_review` event from a moderation
  review row clears a prior quarantine. The event cites the prior
  quarantine event via `prior_publisher_lifecycle_event_record_ref`
  and promotes the trust badge back to organisational; installed-row
  disposition is `notify_with_required_update_pending` so the user
  / admin runs the post-restoration update before activation
  resumes. The parity assertion resolves
  `parity_held_full_vocabulary_match`.

A private-registry parity case is exercised inline on the
`abandoned_publisher.yaml` fixture's parity record (subject
`registry_source_class = private_registry`), demonstrating that
private lanes cite the same lifecycle, anti-abuse, and badge
vocabulary as public lanes.

## Audit events reserved

Lifecycle and parity surfaces emit events on existing streams; raw
artifact bytes, raw signing-key material, raw URLs, raw repository
paths, raw publisher-private data, and raw popularity counts MUST NOT
appear on any event.

- `publisher_lifecycle_event_recorded`
- `publisher_lifecycle_event_revoked`
- `publisher_lifecycle_event_amended`
- `publisher_restoration_after_review_recorded`
- `installed_package_lifecycle_notification_emitted`
- `installed_package_lifecycle_notification_acknowledged`
- `installed_package_required_update_completed`
- `installed_package_uninstalled_after_lifecycle_event`
- `installed_package_stale_high_trust_badge_detected`
- `mirror_promotion_admitted_recorded`
- `mirror_promotion_revoked_recorded`
- `registry_parity_assertion_recorded`
- `registry_parity_assertion_revoked`
- `registry_parity_violation_detected`
- `registry_parity_violation_repaired`
- `registry_parity_assertion_amended`

## Denial reasons reserved

Lifecycle and parity surfaces extend the existing denial vocabulary.
Silent fallback to a generic 'install blocked' or 'unavailable' chip
is forbidden; every denial emits the corresponding audit event with a
typed reason and a repair-affordance label.

- `publisher_quarantine_engaged`
- `publisher_revocation_emergency`
- `publisher_revocation_administrative`
- `artifact_revocation_engaged`
- `artifact_emergency_disable_engaged`
- `publisher_suspension_pending_review`
- `publisher_transfer_pending_user_acknowledgement`
- `publisher_transfer_pending_admin_policy_exception`
- `installed_package_stale_high_trust_badge`
- `lifecycle_event_unverified_actor`
- `lifecycle_event_missing_typed_reason`
- `lifecycle_event_missing_effective_at`
- `review_disclosure_incomplete`

## Forbidden collapses

Tooling tests fail closed on any record that would do one of these
things:

- silently retain a high-trust badge on an installed copy after a
  revocation, quarantine, suspension, or emergency-disable event;
- collapse a publisher-transfer-completed event into a single
  "publisher updated" chip that hides the demoted trust tier;
- treat a private-registry row as if it were a public-registry row by
  rendering `provenance_badge_public_registry` or by hiding the
  registry-source chip;
- mint a renamed `internal_compatible` / `internal_maintained` /
  `internal_low_runtime_cost` class on a private lane;
- hide an engaged anti-abuse register row from any consumer surface
  on the basis that the lane is internal;
- skip emitting an `installed_package_lifecycle_notification_record`
  on a revocation / quarantine / emergency-disable / suspension /
  publisher-transfer-completed event;
- promote a `mirror_promotion_admitted` event's trust badge above the
  origin tier;
- back-date a lifecycle event by mutating `effective_at` after the
  event has been recorded;
- hide a publisher_lifecycle_event_record from the parity record's
  `publisher_lifecycle_event_record_refs` set;
- promote `parity_unknown_pending_review` to `parity_held_*` without
  a typed amend event;
- render a parity assertion that drops one of the ten parity axes;
- expose raw URLs, raw repository paths, raw publisher email
  addresses, raw signing-key bodies, raw attestation-bundle bytes,
  raw moderation review bodies, raw policy-pack constraint bodies,
  raw popularity counts, or raw user-supplied parameter values
  across either boundary.

## Consumer expectations

The downstream surfaces below MUST read this contract rather than
invent lifecycle-shaped or parity-shaped fields:

- **Discovery / search / catalog browse.** Project the
  `publisher_lifecycle_event_record` set onto the discovery row's
  badges and the anti-abuse register row(s); render every lifecycle
  event inside the rendered window.
- **Detail page.** Project the same lifecycle event records and the
  parity assertion's per-axis state. Inspectable explanations open
  the upstream evidence refs.
- **Install / update review sheet.** Reuse the same disposition /
  repair-affordance / denial vocabulary as the installed-row
  notification.
- **Installed-package list / runtime-status pill.** Read the
  `installed_package_lifecycle_notification_record` verbatim. Render
  the post-event trust / compatibility / runtime-cost badge classes;
  never compute them locally.
- **Permission inspector.** Read the lifecycle event's
  `policy_pack_applicable_refs` so the inspector can render the
  policy-pack constraint that backs the rendered badge.
- **Support export, mutation-journal entry, save manifest, claim
  manifest.** Carry the lifecycle event ids, the installed-row
  notification ids, the parity assertion ids, and the upstream
  evidence refs. Raw artifact bytes, raw URLs, raw publisher-private
  fields, and raw popularity counts forbidden.
- **Moderation review surface.** Read every parity assertion and
  every lifecycle event inside the parity window; surface
  `parity_violated_*` states and `lifecycle_event_unverified_actor`
  / `lifecycle_event_missing_typed_reason` denials inline with the
  responsible registry / mirror / private-registry endpoint.

## Out of scope

- Operating a publisher-trust service, a moderation pipeline, a key-
  management implementation, or a private-registry administration
  UX. This contract freezes the vocabulary, not the implementation.
- Legal policy on how publisher transfers, revocations, or
  quarantines are decided. The contract names the actor / reason /
  effective-date trio so downstream legal / governance flows can
  plug into the same record.
- Live registry operations or abuse-investigation tooling.
- The exact quantitative thresholds for "abandoned" maintainership,
  the recertification cadence for approved mirrors, or the
  quarantine clearance window. These ride on later successor
  decision rows.
- Third-party registry integration beyond the registry-seed source
  classes (`approved_mirror`, `private_registry`).
