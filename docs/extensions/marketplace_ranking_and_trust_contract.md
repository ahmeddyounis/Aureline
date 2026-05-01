# Marketplace ranking, trust badge, and anti-abuse vocabulary contract

This document is the narrative companion to the discovery boundary
schemas at
[`/schemas/extensions/discovery_ranking_reason.schema.json`](../../schemas/extensions/discovery_ranking_reason.schema.json)
and
[`/schemas/extensions/discovery_badge.schema.json`](../../schemas/extensions/discovery_badge.schema.json),
and to the machine-readable anti-abuse register at
[`/artifacts/extensions/anti_abuse_states.yaml`](../../artifacts/extensions/anti_abuse_states.yaml).
It freezes the controlled vocabulary that every marketplace
discovery, detail, and install-review surface MUST project across
public, private, mirror, offline-bundle, and local-archive lanes.
The schemas are authoritative when the narrative and the schemas
disagree; this document MUST be updated in the same change that
lands any schema bump.

The contract is deliberately narrow. It does **not** land a
marketplace UX, a moderation pipeline, a popularity-counter
implementation, a publisher-trust service, or a runtime quarantine
policy implementation. Its job is to freeze the discovery / trust /
anti-abuse vocabulary early enough that ranking, badges, and
warnings remain first-class once the marketplace, install-review
sheet, permission inspector, runtime-status pill, and support
export lanes build against it.

## What this contract freezes

1. A **shared discovery-result row** binding every catalog hit to the
   registry-manifest row that backs it. The discovery row carries
   trust / provenance, compatibility, runtime cost, safety / policy,
   bridge state, and the closed ranking-reason chip set; surfaces
   read these fields rather than mint marketplace-shaped fields.
2. A **closed ranking-reason chip vocabulary** (33 classes, schema
   §`ranking_reason_chip_class`) covering compatibility match,
   verified / managed / organisational / unverified / quarantined
   publisher, maintained / abandoned, low / nominal / elevated /
   unknown / quarantined runtime cost, policy pinned / blocked /
   managed-only, anti-abuse warning raised, suppressed / removed
   from ranking, popularity observed / suppressed-due-to-quarantine,
   provenance projection (approved-mirror, private, offline-bundle,
   local-archive), and bridge-state projection.
3. A **closed badge vocabulary** (35 classes, schema
   §`badge_class`) bound to seven typed axes (trust, provenance,
   compatibility, runtime cost, safety / policy, maintainership,
   bridge state).
4. A **closed warning vocabulary** (19 classes, schema
   §`warning_class`) covering typosquat, impersonation, abandoned
   publisher, publisher transfer, revoked / emergency-disabled
   artifact, transitive-permission concern, policy-blocked install,
   quarantined-runtime / crash-loop, digest-collision, rapid-
   republish, mirror-rewrite, and revocation-snapshot stale or
   missing.
5. A **typed warning-severity, warning-response, repair-affordance,
   and denial-reason vocabulary**. Every warning carries at least
   one repair affordance (silent dead-ends are non-conforming) and
   pairs every blocking severity with a typed denial reason.
6. A **default ranking-floor order** that places safety,
   compatibility, maintainership, and runtime cost ahead of raw
   popularity. The schema gates pin "popularity may never be the
   first floor"; the anti-abuse register pins per-state position
   caps so a quarantined or revoked row can never render in the
   "promoted" position.
7. A **shared anti-abuse register** binding every detector signal
   (registry / mirror / publisher-continuity / runtime quarantine /
   policy / moderation) to the warning class, severity, response,
   repair-affordance set, position cap, forced ranking-reason
   chip(s), forced badge, and typed denial reason. Surfaces read
   this register; minting per-surface anti-abuse vocabulary is
   non-conforming.

## Record kinds

| Record kind                        | Schema                                                                                              | Purpose                                                                                                        |
|------------------------------------|-----------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| `discovery_result_row`             | [`discovery_ranking_reason.schema.json`](../../schemas/extensions/discovery_ranking_reason.schema.json#/$defs/discovery_result_row) | One ranked discovery hit. Carries the trust / provenance / compatibility / runtime-cost / safety / bridge-state class set, the ranking-floor order, the ranking-reason chip set, and the discovery-position class. |
| `ranking_reason_chip_record`       | [`discovery_ranking_reason.schema.json`](../../schemas/extensions/discovery_ranking_reason.schema.json#/$defs/ranking_reason_chip_record) | Standalone chip record. Used when a chip is cited from a journal entry, support export, or claim manifest independently of its parent discovery row. |
| `discovery_badge_record`           | [`discovery_badge.schema.json`](../../schemas/extensions/discovery_badge.schema.json#/$defs/discovery_badge_record) | One badge that renders on a discovery card / detail page / install-review sheet, bound to one of the seven axes. |
| `discovery_warning_record`         | [`discovery_badge.schema.json`](../../schemas/extensions/discovery_badge.schema.json#/$defs/discovery_warning_record) | One first-class warning (typosquat, impersonation, abandoned, transfer, revoked, transitive-permission, policy-blocked, quarantined, …). |

## Source-of-truth bindings

A discovery row is **never** the source of truth. Every row binds
back to the upstream rows via opaque refs:

- `registry_manifest_row_ref` → the
  [`registry_manifest_row`](../../schemas/extensions/registry_manifest.schema.json#/$defs/registry_manifest_row)
  in the registry / mirror / offline-bundle / local-archive seed.
- `publisher_continuity_ref` → the ADR-0012 publisher-continuity row.
- `policy_pack_applicable_refs` → the ADR-0012 policy-pack constraint
  rows that apply.
- `discovery_badge_record_refs` and per-record `evidence_refs` →
  the badge / warning records and the rows that justify them
  (registry-manifest row, mirror-continuity row, publisher-continuity
  row, channel-promotion row, activation-evidence packet,
  policy-pack constraint row, anti-abuse signal entry source,
  runtime quarantine trigger row id).

The discovery row projects these upstream classes into chips, badges,
and warnings; it does not invent new fields.

## Default ranking-floor order

The default order every discovery surface MUST start from is:

1. `ranking_floor_safety_first` — no anti-abuse warning at
   `denial_no_install_path` severity, no active runtime quarantine,
   no `revoked` or `emergency_disabled` approval state, no
   `quarantined_cannot_inherit` trust badge.
2. `ranking_floor_compatibility_first` — `compatibility_claim_class`
   in `compatible_on_all_declared_targets` or
   `compatible_on_subset_of_declared_targets`.
3. `ranking_floor_maintainership_first` — `maintainership_badge_*`
   in `active` or `recent`; not `abandoned`,
   `unknown`, or `publisher_transfer_recent`.
4. `ranking_floor_runtime_cost_first` — `runtime_cost_class` in
   `runtime_cost_low_nominal` or `runtime_cost_nominal`, with
   `runtime_cost_evidence_class` in
   `activation_evidence_packet_present` or
   `benchmark_archive_present`. A
   `self_reported_only_unverified` evidence class never credits the
   `low_nominal` chip.
5. `ranking_floor_policy_pinned_first` — workspace pin / org pin if
   present.
6. `ranking_secondary_popularity` — popularity, only after every
   floor above is satisfied.

Surfaces MAY narrow the order (drop popularity for managed-only
workspaces, drop the policy-pinned floor for unmanaged personal
workspaces) but MUST NOT promote `ranking_secondary_popularity`
above any of the four safety / compatibility / maintainership /
runtime-cost floors. The schema gates pin this:

- `discovery_result_row.ranking_floor_order[0]` MUST be one of the
  four ranking floors `safety_first`, `compatibility_first`,
  `maintainership_first`, or `runtime_cost_first`. A surface that
  starts the order with `ranking_secondary_popularity` is
  non-conforming.
- A surface that violates the order at runtime denies with
  `discovery_ranking_floor_violated`.

## Discovery-position class

Every `discovery_result_row` carries exactly one
`discovery_position_class`:

| Position class                                       | Renders                                                                                       |
|------------------------------------------------------|-----------------------------------------------------------------------------------------------|
| `promoted_safe_maintained_compatible_low_cost`       | Top of the result set. Reserved for rows that satisfy every promotion floor and raise no negative chips. |
| `elevated_compatibility_match`                       | Above the standard band. Compatibility-first hit with at most advisory chips.                  |
| `standard_compatible_match`                          | Standard result band.                                                                          |
| `suppressed_below_compatible_match`                  | Below the standard band. Used for non-`production` channel rows or rows with neutral suppression. |
| `demoted_warning_visible`                            | Visible but visually demoted. Used for advisory or blocking-pending-acknowledgement warnings.   |
| `removed_from_ranking_visible_in_detail_only`        | Removed from the ranked list; reachable from a direct link or a detail page.                   |
| `not_in_ranking_review_only`                         | Removed from ranking entirely. Reachable only from review surfaces (admin moderation, support). |

The schema enforces:

- `approval_state_class ∈ {revoked, emergency_disabled,
  blocked_by_policy}` → position class is at most
  `demoted_warning_visible` (and the anti-abuse register narrows
  this further to `removed_from_ranking_visible_in_detail_only`
  or `not_in_ranking_review_only` for revoked / emergency-disabled
  / policy-blocked rows).
- `trust_badge_inheritance_rule = quarantined_cannot_inherit` →
  position class is at most `standard_compatible_match`, AND the
  row carries a `publisher_quarantined_cannot_inherit` chip.
- `runtime_cost_class =
  runtime_cost_quarantined_under_crash_loop_or_egress_breach` →
  position class is `removed_from_ranking_visible_in_detail_only`
  or `not_in_ranking_review_only`, AND the row carries both a
  `runtime_cost_quarantined` and a `removed_from_ranking` chip.
- `channel_class = quarantine` → position class is at most
  `suppressed_below_compatible_match`, AND the row carries a
  `suppressed_in_ranking` or `removed_from_ranking` chip.

## Ranking-reason chips

Every `discovery_result_row` carries a non-empty
`ranking_reason_chip_entries` array. Each chip carries:

- `chip_class` from the closed 33-class vocabulary.
- `polarity` in `{positive, neutral, negative, suppress_only}`.
- `ranking_floor_class` naming the floor the chip contributes to.
- `rendered_label` (human-legible) and optional `explanation_label`
  (inspectable).
- `evidence_refs` pointing at the upstream rows that justify the
  chip.

Chips are **inspectable**: a surface that hides the explanation when
the user opens the chip is denied with `review_disclosure_incomplete`.
A row with no chips is non-conforming; honesty demands a
`compatibility_unknown_pending_reverification` or a
`runtime_cost_unknown_pending_evidence` chip over an empty list.

The closed chip set:

| Floor                                  | Positive chips                                                                                                          | Negative / suppress chips                                                                            |
|----------------------------------------|-------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `ranking_floor_safety_first`           | `verified_publisher`, `managed_approved`, `organisationally_published`                                                  | `publisher_unverified`, `publisher_quarantined_cannot_inherit`, `anti_abuse_warning_raised`, `policy_blocked`, `removed_from_ranking`, `suppressed_in_ranking` |
| `ranking_floor_compatibility_first`    | `compatibility_match`, `compatibility_subset_match`                                                                     | `compatibility_bridge_required`, `compatibility_unknown_pending_reverification`, `incompatible_blocked_on_policy` |
| `ranking_floor_maintainership_first`   | `maintained_actively`, `maintained_recently`                                                                            | `maintainership_unknown`, `abandoned_publisher`                                                       |
| `ranking_floor_runtime_cost_first`     | `low_runtime_cost`, `nominal_runtime_cost`                                                                              | `elevated_runtime_cost`, `runtime_cost_unknown_pending_evidence`, `runtime_cost_quarantined`          |
| `ranking_floor_policy_pinned_first`    | `policy_pinned`, `managed_only_install`                                                                                 | `policy_blocked`                                                                                      |
| `ranking_secondary_popularity`         | `popularity_signal_observed`                                                                                            | `popularity_signal_suppressed_due_to_quarantine`                                                      |
| `ranking_diagnostic_only_no_score`     | `approved_mirror_provenance`, `private_registry_provenance`, `offline_bundle_provenance`, `local_archive_provenance`, `bridge_state_required`, `bridge_state_capability_world_subset_only`, `bridge_state_host_contract_family_subset_only` | (informational only; never moves a row up or down)                                                    |

## Badges

Every badge carries one of seven `badge_axis_class` values:

| Axis                   | Badge classes                                                                                                                                                                  |
|------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `trust_axis`           | `trust_badge_verified_publisher`, `trust_badge_organisational_publisher`, `trust_badge_managed_approved`, `trust_badge_unverified_publisher`, `trust_badge_quarantined_cannot_inherit` |
| `provenance_axis`      | `provenance_badge_public_registry`, `provenance_badge_approved_mirror`, `provenance_badge_private_registry`, `provenance_badge_offline_bundle`, `provenance_badge_local_archive`, `provenance_badge_quarantined_local_copy` |
| `compatibility_axis`   | `compatibility_badge_compatible_on_all_declared_targets`, `compatibility_badge_compatible_on_subset_of_declared_targets`, `compatibility_badge_compatibility_bridge_required`, `compatibility_badge_compatibility_unknown_pending_reverification`, `compatibility_badge_incompatible_blocked_on_policy` |
| `runtime_cost_axis`    | `runtime_cost_badge_low_nominal`, `runtime_cost_badge_nominal`, `runtime_cost_badge_elevated`, `runtime_cost_badge_unknown_pending_evidence`, `runtime_cost_badge_quarantined` |
| `safety_policy_axis`   | `safety_policy_badge_policy_pinned`, `safety_policy_badge_managed_only_install`, `safety_policy_badge_policy_blocked`, `safety_policy_badge_emergency_disabled`, `safety_policy_badge_revoked` |
| `maintainership_axis`  | `maintainership_badge_active`, `maintainership_badge_recent`, `maintainership_badge_unknown`, `maintainership_badge_abandoned`, `maintainership_badge_publisher_transfer_recent` |
| `bridge_state_axis`    | `bridge_state_badge_no_bridge_required`, `bridge_state_badge_compatibility_bridge_profile`, `bridge_state_badge_capability_world_subset_only`, `bridge_state_badge_host_contract_family_subset_only`, `bridge_state_badge_unsupported_blocked_on_policy` |

The schema gates pin badge → axis: a renamed badge cannot move axes
silently. `trust_badge_quarantined_cannot_inherit` and
`trust_badge_unverified_publisher` are restricted to
`advisory` or `negative` polarity.

## Warnings

The closed warning-class vocabulary (19 classes):

| Warning class                                       | Default severity                                              | Default response                          |
|-----------------------------------------------------|---------------------------------------------------------------|-------------------------------------------|
| `typosquat_candidate`                               | `blocking_install_pending_explicit_acknowledgement`           | `prompt_user_for_explicit_acknowledgement` |
| `impersonation_suspected`                           | `blocking_install_pending_explicit_acknowledgement`           | `prompt_user_for_explicit_acknowledgement` |
| `publisher_namespace_recently_orphaned`             | `advisory`                                                    | `render_disclosure_with_inspectable_explanation` |
| `publisher_namespace_recently_quarantined`          | `denial_no_install_path`                                      | `block_install_pending_review`             |
| `publisher_namespace_recently_succeeded`            | `advisory`                                                    | `render_disclosure_with_inspectable_explanation` |
| `abandoned_publisher_no_recent_activity`            | `advisory`                                                    | `render_disclosure_with_inspectable_explanation` |
| `publisher_transfer_recent`                         | `advisory`                                                    | `render_disclosure_with_inspectable_explanation` |
| `artifact_revoked`                                  | `denial_no_install_path`                                      | `block_install_no_recovery`                |
| `artifact_emergency_disabled`                       | `denial_no_install_path`                                      | `block_install_pending_review`             |
| `transitive_permission_concern`                     | `blocking_install_pending_explicit_acknowledgement`           | `prompt_user_for_explicit_acknowledgement` |
| `transitive_permission_inheritance_widened`         | `blocking_install_pending_admin_policy_exception`             | `prompt_admin_for_policy_exception`        |
| `policy_blocked_install`                            | `denial_no_install_path`                                      | `block_install_pending_review`             |
| `quarantined_runtime_disabled`                      | `denial_no_install_path`                                      | `block_install_pending_review`             |
| `quarantined_at_runtime_crash_loop`                 | `denial_no_install_path`                                      | `block_install_pending_review`             |
| `digest_collision_suspected`                        | `denial_no_install_path`                                      | `block_install_pending_review`             |
| `rapid_version_republish`                           | `advisory`                                                    | `render_disclosure_with_inspectable_explanation` |
| `mirror_rewrite_attempted`                          | `denial_no_install_path`                                      | `block_install_pending_review`             |
| `revocation_snapshot_stale`                         | `blocking_install_pending_explicit_acknowledgement`           | `prompt_user_for_explicit_acknowledgement` |
| `revocation_snapshot_unverified_no_snapshot`        | `denial_no_install_path`                                      | `block_install_pending_review`             |

Schema gates pin:

- `severity_class = denial_no_install_path` →
  `denial_reason_class != no_denial_admissible` AND
  `response_class ∈ {block_install_pending_review,
  block_install_no_recovery}`.
- `severity_class = informational` →
  `denial_reason_class = no_denial_admissible` AND
  `response_class ∈ {render_disclosure_continue,
  render_disclosure_with_inspectable_explanation}`.
- `warning_class ∈ {artifact_revoked, artifact_emergency_disabled,
  policy_blocked_install, quarantined_runtime_disabled,
  quarantined_at_runtime_crash_loop}` →
  `severity_class = denial_no_install_path`.
- `response_class = block_install_no_recovery` →
  `repair_affordance_class_set` contains `no_recovery_review_only`
  (the user / admin still gets a typed next step, even if it is
  review-only).

Every warning carries at least one
`warning_repair_affordance_class`. Silent dead-ends
(raised warning, no typed next step) are non-conforming.

## Anti-abuse register

[`/artifacts/extensions/anti_abuse_states.yaml`](../../artifacts/extensions/anti_abuse_states.yaml)
binds every detector signal to a typed
`(warning_class, severity, response, repair_affordance_set,
discovery_position_class_at_most, forced_ranking_reason_chips,
forced_badge, denial_reason)` tuple.

Detector signals come from one of:

- **Registry / mirror anti-abuse signals** (registry-manifest row's
  `anti_abuse_signal_class`).
- **Publisher-continuity state changes** (the publisher's continuity
  row recorded an orphan, quarantine, or succession event).
- **Extension-runtime quarantine triggers** (a previously installed
  copy tripped a runtime quarantine; see
  [`artifacts/extensions/quarantine_rules.yaml`](../../artifacts/extensions/quarantine_rules.yaml)).
- **Policy-pack anti-abuse constraints** (the workspace policy pack
  raised a transitive-permission concern or an install denial).
- **Moderation review decisions**.

A discovery surface that processes any of these signals MUST cite
the row id from the register; silent demotion or removal without a
cited row is non-conforming. An anti-abuse signal **never** promotes
or demotes the publisher-continuity row on its own — promotion /
demotion is a publisher-continuity decision; the register only
narrows the discovery surface's rendered position and forces the
warning chip and badge.

## Disclosure invariants

- A surface MUST render every raised badge whose polarity is
  `advisory` or `negative` and every raised warning. Hiding either
  is denied with `review_disclosure_incomplete`.
- A surface MUST render the `ranking_reason_chip_entries` set in
  full, with each chip's `evidence_refs` and `explanation_label`
  inspectable on click. Hiding the explanation denies with
  `review_disclosure_incomplete`.
- A surface MUST render the `compatibility_claim_class` projection
  on every result, including `compatibility_unknown_pending_reverification`
  and `compatibility_bridge_required`. Hiding a bridge or unknown
  claim denies with `review_disclosure_incomplete`.
- A surface MUST render the runtime-cost class even when it is
  `runtime_cost_unknown_pending_evidence`. Promoting the chip to
  `low_nominal` on the basis of popularity or self-reported
  counters alone denies with `review_disclosure_incomplete`.
- A surface MUST render the `discovery_position_class`. Silent
  promotion above the anti-abuse register's position cap denies
  with `discovery_ranking_floor_violated`.

## Default behaviour for the four acceptance fixtures

The fixtures under
[`/fixtures/extensions/marketplace_discovery_cases/`](../../fixtures/extensions/marketplace_discovery_cases)
exercise the contract end-to-end:

- `public_package_promoted_first_party.yaml` — a public-registry
  production row resolves to
  `discovery_position_class = promoted_safe_maintained_compatible_low_cost`
  with the full positive chip set.
- `private_registry_org_managed_approved.yaml` — an org-private
  artifact resolves to `discovery_position_class =
  elevated_compatibility_match` with `trust_badge_managed_approved`
  and `safety_policy_badge_managed_only_install`.
- `typosquat_candidate_warning_demoted.yaml` — a typosquat
  candidate resolves to `discovery_position_class =
  demoted_warning_visible` with `typosquat_candidate` warning at
  `blocking_install_pending_explicit_acknowledgement` severity.
- `revoked_artifact_removed_from_ranking.yaml` — a revoked artifact
  resolves to `discovery_position_class =
  not_in_ranking_review_only` with `artifact_revoked` warning at
  `denial_no_install_path` severity and
  `block_install_no_recovery` response.
- `bridge_state_compatibility_bridge_required.yaml` — a row that
  needs a compatibility-bridge profile resolves to
  `bridge_state_class = bridge_required_compatibility_bridge_profile`
  with the `compatibility_bridge_required` chip and the
  `bridge_state_badge_compatibility_bridge_profile` badge; install
  is admitted via the bridge profile.

## Audit events reserved

Discovery surfaces emit events on existing streams; raw artifact
bytes, raw signing-key material, raw URLs, raw repository paths,
and raw popularity counts MUST NOT appear on any event.

- `anti_abuse_signal_raised` (mirrored from the registry seed).
- `mirror_continuity_broken` (mirrored).
- `mirror_narrowing_attempted_widening` (mirrored).
- `channel_promotion_blocked_by_policy` (mirrored).
- `channel_promotion_revoked` (mirrored).
- `revocation_snapshot_stale_detected` (mirrored).
- `extension_quarantine_tripped` (mirrored from runtime-budget /
  quarantine artifacts).
- `extension_quarantined_recovery_attempted` (mirrored).
- `extension_quarantine_cleared` (mirrored).
- `discovery_ranking_floor_violated` (new, raised when a surface
  promotes popularity above safety / compatibility / maintainership
  / runtime-cost or renders a row above its anti-abuse position
  cap).
- `discovery_review_disclosure_incomplete` (new, raised when a
  surface hides a chip, badge, warning, or evidence ref).

## Denial reasons reserved

Discovery surfaces reuse and extend the registry-seed denial
vocabulary. Silent fallback to a generic "install blocked" or
"unavailable" chip is forbidden; every denial emits the corresponding
audit event with a typed reason and a repair-affordance label.

- `discovery_ranking_floor_violated`
- `review_disclosure_incomplete`
- `publisher_namespace_recently_quarantined`
- `artifact_revoked`
- `artifact_emergency_disabled`
- `policy_blocked_install`
- `quarantined_runtime_disabled`
- `quarantined_at_runtime_crash_loop`
- `transitive_permission_concern_user_review_required`
- `typosquat_candidate_user_review_required`
- `impersonation_suspected_user_review_required`
- `abandoned_publisher_user_review_required`
- `publisher_transfer_recent_user_review_required`
- `revocation_snapshot_stale_user_review_required`
- `digest_collision_suspected_user_review_required`

## Consumer expectations

The downstream surfaces below MUST read this contract rather than
invent marketplace-shaped fields:

- **Discovery / search / catalog browse.** Project the
  `discovery_result_row` verbatim. Render every chip, badge, and
  warning. Order rows by `ranking_floor_order`; never promote
  popularity above safety / compatibility / maintainership /
  runtime cost.
- **Detail page.** Project the same `discovery_result_row` and the
  full `discovery_badge_record` and `discovery_warning_record` set.
  Inspectable explanations open the upstream evidence refs.
- **Install / update review sheet.** Reuse the same chip / badge /
  warning vocabulary; the install-review denial vocabulary
  resolves to the same `denial_reason_class` set.
- **Permission inspector.** Read the
  `transitive_permission_concern` and
  `transitive_permission_inheritance_widened` warnings; the
  evidence refs resolve to ADR-0012 effective-permission summary
  rows.
- **Runtime-status pill.** Read the `runtime_cost_quarantined`
  chip, the `runtime_cost_badge_quarantined` badge, and the
  `quarantined_runtime_disabled` /
  `quarantined_at_runtime_crash_loop` warnings. Quarantine
  evidence refs resolve to the runtime-budget / quarantine artifacts.
- **Support export, mutation-journal entry, save manifest, claim
  manifest.** Carry the discovery_result_row id, the chip / badge /
  warning records, and the upstream evidence refs. Raw artifact
  bytes, raw URLs, and raw popularity counts forbidden.
- **Moderation review surface.** Read the anti-abuse register;
  every demotion or removal cites a row id from
  [`anti_abuse_states.yaml`](../../artifacts/extensions/anti_abuse_states.yaml).

## Out of scope

- Operating a marketplace, a trust service, a moderation pipeline, or
  a popularity-counter implementation. This contract freezes the
  vocabulary, not the implementation.
- The exact quantitative thresholds for "recent" vs "active" vs
  "abandoned" maintainership; the runtime-cost class boundaries; the
  popularity-counter aggregation window. These ride on later
  successor decision rows and on the reference-hardware manifest.
- Third-party registry integration beyond the registry-seed source
  classes (`approved_mirror`, `private_registry`).
- The discovery query API itself; the contract pins the row shape
  every API response carries, not the query language.
