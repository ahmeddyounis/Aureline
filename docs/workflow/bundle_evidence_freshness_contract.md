# Workflow-bundle evidence-freshness, certification-downgrade, and stale-badge fail-gate contract

This document freezes the cross-surface evidence-freshness,
badge-downgrade, and stale-badge fail-gate model every
**Certified, Managed approved, Community, Imported, Local
draft, and Retest pending** workflow-bundle badge resolves
through before it is rendered, exported, or projected onto a
claim publication. The goal is that bundle-and-support
badges never **outlive their proof** and never **drift away
from current evidence**: every rendered badge is reviewable
back to the bound evidence, the bound proof class, the bound
freshness window, and the bound downgrade rule.

The companion machine-readable artifacts live at:

- [`/artifacts/workflow/bundle_badge_freshness_rows.yaml`](../../artifacts/workflow/bundle_badge_freshness_rows.yaml)
  — one row per badge id binding required evidence link kinds,
  required proof classes, freshness cadence, downgrade target,
  and publish-gate posture.
- [`/artifacts/workflow/stale_badge_fail_rules.yaml`](../../artifacts/workflow/stale_badge_fail_rules.yaml)
  — closed fail-gate rules so stale `Certified` and
  `Managed approved` badges cannot publish without linked
  evidence.

The companion fixtures live under:

- [`/fixtures/workflow/bundle_certification_cases/`](../../fixtures/workflow/bundle_certification_cases/)

This contract is normative for the badge-freshness row, the
badge-downgrade table, and the stale-badge fail gate. Where
it disagrees with the PRD, TAD, TDD, UI / UX spec, design-
system style guide, or milestone document anchors quoted in
§13, those sources win and this document plus its companion
artifacts and fixtures update in the same change. Where a
downstream Start Center bundle card, bundle detail page,
gallery, browse-bundles, project-doctor, support-export,
release-evidence packet, certification sheet, claim-
publication generator, CLI / headless surface, or marketing
copy lane mints a parallel badge, freshness state,
downgrade rule, or fail-gate verb, this contract wins and
the surface is non-conforming.

This contract mints **no** new bundle, identity, source,
status, packaging, signer, channel-relation, dependency-
marker, component-kind, evidence-link, lifecycle, successor,
removal-surface, rollback-surface, change-axis, drift-state,
or claim-narrowing vocabulary. It re-exports — by reference,
never by redefinition — the closed sets frozen by the
upstream contracts listed in §3.0.

## Who reads this contract

- **Bundle badge authors** rendering badges on the Start
  Center bundle card, the bundle detail page, gallery /
  browse-bundles, update-detail, project-doctor,
  certification sheet, support-export packet, claim-
  publication binding, CLI / headless `aureline bundle list`,
  and any later API. Every rendered badge resolves through
  one row in
  [`bundle_badge_freshness_rows.yaml`](../../artifacts/workflow/bundle_badge_freshness_rows.yaml).
- **Reviewers** tracing a `Certified`, `Managed approved`,
  `Community`, `Imported`, `Local draft`, or `Retest pending`
  badge back to the bound evidence link, the bound proof
  class, the bound freshness window, and the bound downgrade
  rule. They expect to read the badge's freshness state,
  downgrade target, and publish-gate posture from one record
  rather than reconstructing it from prose.
- **Claim-publication automation authors**
  ([`/docs/release/claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md))
  consuming the badge-freshness row and the stale-badge
  fail-gate rules to decide whether a workflow-bundle or
  certification badge MAY publish on docs / Help / About /
  service-health / release-notes / CLI-help / evaluation /
  marketplace / known-limits destinations. The fail gate is
  the same gate this contract pins.
- **Support-export, project-doctor, and certification-sheet
  authors** projecting a bundle's freshness posture and any
  pending retest hooks. The badge they render is the badge
  this contract resolves.
- **Marketing-copy authors** who must distinguish badge
  vocabulary from marketing copy and from workflow-bundle
  ownership. The badge ids are stable, audit-attributable
  proof markers, not promotional language; the separation
  rules live in §11.

## 1. Scope

- Freeze one `bundle_badge_freshness_row_record` per
  upstream `bundle_evidence_badge_id` so every rendered
  badge resolves through a closed row binding badge id,
  backing manifest fields, required evidence link kinds,
  required proof classes, freshness cadence, downgrade
  target, and publish-gate posture.
- Freeze the **freshness-state taxonomy** (§3.1) covering
  `evidence_fresh_within_cadence`,
  `evidence_aging_within_window`,
  `evidence_stale_past_window`,
  `evidence_age_unknown_offline_or_unreachable`,
  `evidence_missing_required_link`, and
  `evidence_signature_or_continuity_broken` so reviewers
  see one closed vocabulary rather than ad-hoc "out of
  date" copy.
- Freeze the **downgrade-trigger taxonomy** (§3.2) covering
  evidence aging out, mirror / source changes, compatibility
  narrowing, archetype proof going stale, signer continuity
  breaking, channel demotion, and policy-pinned recall so
  every downgrade has a typed cause.
- Freeze the **downgrade-target taxonomy** (§3.3) so a
  `badge.certified` row downgrades to a closed target
  (`badge.retest_pending`, `badge.deprecated`,
  `badge.status_unknown`, etc.) and never to a free-text
  marketing label.
- Freeze the **publish-gate taxonomy** (§3.4) covering
  `publish_allowed`, `publish_allowed_with_narrowing`,
  `publish_fail_closed_warn`, and
  `publish_fail_closed_block` so the claim-publication
  contract can plug into a stable gate verb.
- Freeze the **stale-badge blocker taxonomy** (§3.5) so a
  `publish_fail_closed_block` row cites a closed reason.
- Freeze the **badge-freshness evaluation contract** (§4)
  every reviewer / automation reads to decide whether a
  badge stays as-is, downgrades, or fail-closes.
- Freeze the **per-badge row table** (§5) — Certified,
  Managed approved, Community, Imported, Local draft, and
  Retest pending — so the upstream `bundle_evidence_badge_id`
  vocabulary is bound to required evidence and downgrade
  rules in one place.
- Freeze the **stale-badge fail-gate** (§6) so stale
  `Certified` or `Managed approved` badges cannot publish
  without linked evidence: the gate verdict is one of four
  closed values and every fail-closed verdict cites a
  blocker class plus the missing or stale evidence link
  kind.
- Freeze the **archetype-proof-staleness rule** (§7) so a
  certification target whose `archetype_row_id` /
  `archetype_revision` proof has aged out of its cadence
  downgrades the row's badge before any claim publishes
  certified wording on the archetype.
- Freeze the **mirror-or-source-change rule** (§8) so a
  rotation, demotion, mirror mismatch, or signer continuity
  break downgrades the badge before any surface re-renders
  the prior badge.
- Freeze the **compatibility-narrowing rule** (§9) so a
  bundle whose `compatible_aureline_range` narrows below
  the installed runtime, or whose extension-set bridge
  parity narrows past the cutline cadence, downgrades the
  badge.
- Freeze the cross-cutting **disclosure invariants** (§10)
  every surface family (Start Center bundle card, bundle
  detail page, gallery, project-doctor, support-export,
  certification sheet, claim publication, CLI / headless,
  marketing copy lane) MUST satisfy.
- Freeze the **badge-vs-marketing-vs-ownership separation
  rules** (§11) so the badge vocabulary stays a proof
  marker and never bleeds into marketing copy or workflow-
  bundle ownership semantics.

## 2. Out of scope

- The bundle **manifest** itself.
  [`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  owns identity, component inventory, source / status /
  class linkage, lifecycle, successor recommendation, and
  the `evidence_age_class`, `retest_needed_posture`, and
  `lifecycle_state_class` vocabularies this contract reads.
- The bundle **change-preview** record.
  [`/docs/workflow/bundle_change_review_contract.md`](./bundle_change_review_contract.md)
  owns the install / update / downgrade review sheet and
  the rollback checkpoint linkage; this contract pins only
  the badge freshness state and the publish gate.
- The bundle **drift / remove** record.
  [`/docs/workflow/bundle_drift_and_removal_contract.md`](./bundle_drift_and_removal_contract.md)
  owns post-install drift rows; this contract resolves the
  badge a drift row inherits when its
  `claim_narrowing_class` activates but it does not mint a
  parallel drift state.
- The bundle **installer**, **registry transport**,
  **mirror engine**, or **signer rotation engine**. This
  contract pins only the disclosure shape and the publish
  gate; runtime mechanics live with the registry, mirror,
  and signature contracts.
- **Recommendation ranking**, **search ordering**, or
  **gallery curation**. This contract names which badges
  may render in which freshness state; it does not rank
  rows.
- The **claim-publication generator**.
  [`/docs/release/claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md)
  consumes the gate verdict; this contract pins only the
  verdict shape and the closed blocker set.
- Final user-facing **copy / microcopy**. The shell-
  interaction-safety contract and the UX style guide own
  the exact strings; this contract pins the closed sets the
  copy resolves against.
- **Telemetry wire format**. Badge-render and badge-
  downgrade measurement is owned by the support-export and
  onboarding-measurement plans; this contract only tags
  records with the row id this contract names.

## 3. Frozen vocabulary (re-exported) and new closed sets

### 3.0 Re-exported vocabulary

This contract re-exports — by reference, never by
redefinition — the following closed sets:

- `bundle_class`, `bundle_source_class`,
  `bundle_status_class`, `bundle_signer_source_class`,
  `signer_continuity_class`, `signature_class`,
  `support_class`, `bundle_channel_relation_class`,
  `mirror_or_offline_packaging_posture`,
  `bundle_dependency_marker_kind`, `bundle_component_kind`,
  `evidence_link_kind`, `lifecycle_state_class`,
  `successor_recommendation_class`, `evidence_age_class`,
  `retest_needed_posture`, `removal_surface_kind`,
  `rollback_surface_kind`, `bundle_id`, `bundle_revision`,
  `archetype_row_id`, `archetype_revision`,
  `cutline_ref` —
  [`workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  §3.1–§3.15 and
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).
- `bundle_evidence_badge_id`, `bundle_card_freshness_overlay`,
  `bundle_card_surface_family`,
  `bundle_detail_page_section_id` —
  [`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md).
- `proof_class_id`, `freshness_cadence`,
  `downgrade_trigger`, `scoreboard_family_id`,
  `public_proof_packet_shape`, `workflow_bundle_id`,
  `stale_propagation_profile_id` —
  [`artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  and
  [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml).
- `disabled_reason_code`, `change_axis`,
  `compatibility_state_class` —
  [`bundle_change_review_contract.md`](./bundle_change_review_contract.md)
  §3.3, §3.6, §3.8 and
  [`/schemas/workflow/bundle_change_preview.schema.json`](../../schemas/workflow/bundle_change_preview.schema.json).
- `claim_narrowing_class`,
  `successor_bundle_suggestion_class`,
  `drift_state_class` —
  [`bundle_drift_and_removal_contract.md`](./bundle_drift_and_removal_contract.md)
  §3.1, §3.12, §3.13 and
  [`/schemas/workflow/bundle_drift_row.schema.json`](../../schemas/workflow/bundle_drift_row.schema.json).
- `policy_notice_class`,
  `availability_narrowing_class`, `bypass_path_id` —
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- `claim_row_ref`, `effective_claim_posture`,
  `publication_destination_class` —
  [`/docs/release/claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md)
  and
  [`/schemas/governance/claim_publication_binding.schema.json`](../../schemas/governance/claim_publication_binding.schema.json).

This contract introduces **eight** small vocabularies scoped
to badge freshness, downgrade, fail-gate, and ownership
separation. Each is closed; adding a value is additive-minor
and bumps `bundle_badge_freshness_rows_schema_version`,
repurposing is breaking and opens a decision row in
[`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

### 3.1 `badge_freshness_state_class`

What freshness posture the row carries against the bound
evidence. The set is closed:

- `evidence_fresh_within_cadence` — every required
  evidence link kind resolves to a packet whose freshness
  metadata (`captured_at` + `stale_after`) is within the
  bound `proof_class_id`'s `max_stale_after`. The badge
  renders verbatim.
- `evidence_aging_within_window` — at least one required
  evidence link kind has crossed the proof class's first
  warning threshold but is still within
  `max_stale_after`. The badge renders, but
  `bundle_card_freshness_overlay = freshness.aging_within_window`
  MUST render and a typed retest hook MUST be carried on
  the row.
- `evidence_stale_past_window` — at least one required
  evidence link kind is past `max_stale_after`. The badge
  MUST downgrade per §3.3 and the publish gate (§3.4)
  MUST resolve to `publish_allowed_with_narrowing`,
  `publish_fail_closed_warn`, or
  `publish_fail_closed_block` per the per-badge row.
- `evidence_age_unknown_offline_or_unreachable` —
  freshness could not be determined (registry offline,
  mirror unreachable, packet metadata missing). The badge
  MUST downgrade to `badge.status_unknown` and the
  publish gate MUST resolve to
  `publish_fail_closed_warn` or `publish_fail_closed_block`
  per the per-badge row, with
  `disabled_reason_code = network_unreachable`,
  `mirror_only_cached_subset`, or
  `evidence_aged_review_required`.
- `evidence_missing_required_link` — a required evidence
  link kind is absent from the manifest's
  `evidence_link_refs[]`. The badge MUST downgrade and
  the publish gate MUST resolve to
  `publish_fail_closed_block` with
  `stale_badge_blocker_class =
  required_evidence_link_missing`.
- `evidence_signature_or_continuity_broken` — the bundle's
  `signer_continuity_class` is `lost_continuity_*` or the
  `signature_class` is invalidated since apply. The badge
  MUST downgrade and the publish gate MUST resolve to
  `publish_fail_closed_block` with
  `stale_badge_blocker_class =
  signer_continuity_or_signature_broken`.

Rules (frozen):

1. Every badge-freshness row evaluation produces exactly
   one `badge_freshness_state_class`. A surface that
   emits a seventh state (`partially_fresh`,
   `auto_repair_in_progress`, etc.) is non-conforming.
2. `evidence_fresh_within_cadence` is the only state
   that admits the row's "current" badge id verbatim
   without a downgrade target.
3. The mapping from `evidence_age_class` (manifest field)
   onto `badge_freshness_state_class` is closed and
   one-to-one:
   - `fresh_current` → `evidence_fresh_within_cadence`,
   - `aging_within_window` → `evidence_aging_within_window`,
   - `stale_past_window` → `evidence_stale_past_window`,
   - `age_unknown` → `evidence_age_unknown_offline_or_unreachable`.
   `evidence_missing_required_link` and
   `evidence_signature_or_continuity_broken` are
   evaluation outcomes derived from the manifest's
   `evidence_link_refs[]` and signer continuity, not from
   `evidence_age_class` alone.

### 3.2 `badge_downgrade_trigger_class`

Why the badge would downgrade. The set is closed and
re-exports the upstream
`downgrade_trigger_vocabulary` from
[`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml)
into bundle-badge scope plus six bundle-specific
triggers:

- `evidence_aged_out_past_window` — at least one required
  evidence link past its proof class's `max_stale_after`.
- `evidence_link_missing_from_manifest` — a required
  evidence link kind is not declared on the manifest's
  `evidence_link_refs[]`.
- `mirror_or_source_signature_changed` — signer rotated
  unexpectedly, signature review window expired, key
  revocation observed (mirrors
  `bundle_drift_and_removal_contract.md#3-1` →
  `signer_continuity_break`).
- `mirror_or_source_canonical_revision_changed` —
  the bundle's declared channel advanced past the
  applied revision (mirrors `bundle_version_drift`).
- `mirror_or_source_mirror_mismatch` — the workspace's
  mirror copy diverges from the canonical mirror at the
  bound revision (mirrors `mirror_mismatch`).
- `compatibility_narrowed_below_runtime` — the bundle's
  `compatible_aureline_range` narrows below the
  installed runtime (mirrors `compatible_narrowed` /
  `incompatible_blocking`).
- `archetype_proof_stale_past_cadence` — the bound
  `archetype_row_id` / `archetype_revision` proof has
  aged out of its scoreboard family's freshness cadence.
- `channel_demoted_or_branch_forked` — the bundle's
  `bundle_channel_relation_class` shifted to
  `cross_channel_demotion` or `branch_or_fork_revision`
  since apply.
- `policy_pinned_revision_recall` — an organisation or
  fleet policy has recalled the applied revision.
- `lifecycle_deprecation_or_removal` — the manifest's
  `lifecycle_state_class` shifted to
  `soft_deprecated_with_successor`,
  `hard_deprecated_remove_recommended`,
  `removed_unavailable`, or `rollback_recommended`.
- `evidence_signature_invalidated_post_apply` — the
  evidence link's own signature has been revoked /
  re-signed.
- `freshness_unknown_review_required` — freshness could
  not be determined; row defers downgrade with a typed
  `disabled_reason_code`.

Rules (frozen):

1. Every downgrade emits at least one
   `badge_downgrade_trigger_class`. A surface that
   downgrades a badge without naming a trigger is
   non-conforming.
2. Multiple triggers MAY apply to one downgrade
   (for example `evidence_aged_out_past_window`
   plus `mirror_or_source_mirror_mismatch`); the row
   carries the full list and the publish gate
   resolves to the strictest verdict any trigger
   demands.
3. `archetype_proof_stale_past_cadence` MUST cite the
   `archetype_row_id`, the `archetype_revision`, the
   `scoreboard_family_id`, and the `proof_class_id`
   whose cadence was missed.
4. `mirror_or_source_*` triggers MUST cite the
   manifest's `mirror_or_offline_packaging_posture` so
   the reviewer sees whether the mismatch is on a
   live, mirror, or signed-offline posture.

### 3.3 `badge_downgrade_target_class`

What the badge becomes after a downgrade. The set is
closed and binds upstream `bundle_evidence_badge_id`
values plus the typed "no badge" outcomes:

- `target_retest_pending_badge` — downgrade to
  `badge.retest_pending`. Default outcome when the
  manifest's `bundle_status_class` shifts to
  `certified_retest_pending` or `retest_needed`, or
  when `evidence_age_class` is `aging_within_window`
  or `stale_past_window` and the bundle remains
  installable.
- `target_deprecated_badge` — downgrade to
  `badge.deprecated`. Outcome when the manifest's
  `lifecycle_state_class` shifts to
  `soft_deprecated_with_successor`,
  `hard_deprecated_remove_recommended`,
  `removed_unavailable`, or `rollback_recommended`,
  or when `bundle_status_class = deprecated_or_archived`.
- `target_status_unknown_badge` — downgrade to
  `badge.status_unknown`. Outcome when freshness is
  `evidence_age_unknown_offline_or_unreachable`,
  `bundle_status_class = status_unknown`, or
  `mirror_or_offline_packaging_posture =
  packaging_posture_unknown`.
- `target_imported_pending_review_badge` — downgrade
  to `badge.imported`. Outcome when drift narrows an
  imported-user bundle row back to
  `imported_pending_review` (mirrors
  `claim_narrowing_class =
  narrows_to_imported_pending_review`).
- `target_community_unreviewed_badge` — downgrade to
  `badge.community`. Outcome when a bundle's
  `bundle_status_class` narrows from
  `community_reviewed` to `community_unreviewed`.
- `target_publish_blocked_no_badge` — the badge MUST
  NOT render at all because the publish gate is
  `publish_fail_closed_block`. Surfaces still render
  the row's freshness state and the blocker class on
  the bundle detail panel and support-export, but the
  badge slot is empty (NOT a fallback to a "lower"
  badge — emptiness is verbatim).

Rules (frozen):

1. Every downgrade row emits exactly one
   `badge_downgrade_target_class`. A surface that
   downgrades to a different target than the row
   names is non-conforming.
2. The downgrade target inherits the same Start
   Center / detail-page surface invariants the upstream
   `bundle_evidence_badge_id` carries; downgrading from
   `badge.certified` to `badge.retest_pending` does not
   widen any disclosure constraint.
3. `target_publish_blocked_no_badge` MUST cite a
   `stale_badge_blocker_class` (§3.5).
4. A downgrade target MUST never widen authority. A row
   that downgrades from `badge.community` to
   `badge.certified` is non-conforming. The legal
   downgrade lattice is enumerated in §5.

### 3.4 `badge_publish_gate_class`

What the publish gate verdict is for the row at the time
of evaluation. The set is closed:

- `publish_allowed` — every required evidence link is
  fresh, every required proof class is current, and
  the badge renders verbatim. Claim publication
  proceeds.
- `publish_allowed_with_narrowing` — the badge
  downgrades per §3.3 and the publication binding
  carries the narrowed `support_class` /
  `effective_claim_posture` consistent with
  [`claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md).
  Claim publication proceeds at the narrowed
  posture; widened wording fail-closes.
- `publish_fail_closed_warn` — claim publication
  proceeds only when paired with a typed waiver
  register entry from
  [`artifacts/governance/waiver_register.yaml`](../../artifacts/governance/waiver_register.yaml);
  publication without the waiver fail-closes.
- `publish_fail_closed_block` — claim publication
  cannot proceed. The row cites a
  `stale_badge_blocker_class` (§3.5) and the
  `evidence_link_kind` of the missing or stale
  evidence.

Rules (frozen):

1. Every row evaluation produces exactly one
   `badge_publish_gate_class`. A surface that emits
   a fifth verdict (`publish_pending`,
   `publish_with_grace_period`) is non-conforming.
2. `publish_fail_closed_block` is the only verdict
   that may resolve `target_publish_blocked_no_badge`.
3. `publish_allowed_with_narrowing` MUST cite the
   `claim_narrowing_class` (re-exported from drift
   contract §3.12) the narrowing maps to so the
   publication binding inherits the narrowing
   verbatim.
4. The strictest applicable verdict wins. If multiple
   triggers apply (for example
   `evidence_aged_out_past_window` plus
   `evidence_link_missing_from_manifest`), the row
   resolves to whichever verdict is strictest in the
   ordering
   `publish_allowed < publish_allowed_with_narrowing
   < publish_fail_closed_warn <
   publish_fail_closed_block`.

### 3.5 `stale_badge_blocker_class`

Closed set of blocker reasons a `publish_fail_closed_warn`
or `publish_fail_closed_block` verdict cites:

- `required_evidence_link_missing` — a required
  evidence link kind is not declared on the manifest.
- `required_evidence_link_stale_past_window` — a
  required evidence link is past its proof class's
  `max_stale_after`.
- `archetype_proof_stale_past_cadence_blocker` —
  the bound archetype proof scoreboard row is past
  cadence.
- `signer_continuity_or_signature_broken` — signer
  continuity has lost continuity or the signature
  has been revoked since apply.
- `mirror_or_source_revision_recalled` — the
  applied revision has been recalled by the
  signer's origin or the org policy.
- `compatibility_narrowed_below_supported_floor` —
  `compatible_aureline_range` narrows below the
  bundle's `support_class_floor` from the
  workflow-bundle id register.
- `manifest_status_class_does_not_back_badge` —
  the manifest's `bundle_status_class` does not
  back the rendered badge per §5 (for example a
  `community_unreviewed` manifest cannot back
  `badge.certified`).
- `policy_pinned_revision_recall_blocker` — an
  org / fleet policy recall blocks the row from
  publishing.
- `evidence_freshness_unknown_blocker` — freshness
  cannot be determined (offline, mirror unreachable,
  packet metadata missing); the row defers publish
  rather than risking overclaim.

Rules (frozen):

1. Every `publish_fail_closed_warn` or
   `publish_fail_closed_block` verdict cites at
   least one `stale_badge_blocker_class`. A
   `publish_fail_closed_*` row without a blocker
   class is non-conforming.
2. Each blocker class pairs with a typed
   `disabled_reason_code` re-exported from
   [`bundle_change_review_contract.md`](./bundle_change_review_contract.md)
   §3.8 (`signature_review_required`,
   `evidence_aged_review_required`,
   `network_unreachable`,
   `mirror_only_cached_subset`,
   `policy_narrowed_admin`, `policy_narrowed_fleet`,
   `target_runtime_unavailable`).
3. A surface that emits a free-text blocker reason is
   non-conforming.

### 3.6 `badge_evidence_link_requirement_kind`

How required the evidence link is for a badge row. The
set is closed:

- `required_for_render` — the badge cannot render
  without this evidence link kind. Missing it
  produces `evidence_missing_required_link` and
  `publish_fail_closed_block`.
- `required_for_freshness_evaluation` — the badge
  may render without this link, but freshness
  evaluation needs it. Missing it produces
  `evidence_age_unknown_offline_or_unreachable`.
- `recommended_for_audit_only` — the link is
  audit-attributable but does not gate the badge.
  Missing it does not produce a downgrade.

Rules (frozen):

1. Every entry on a row's `required_evidence_links[]`
   names exactly one
   `badge_evidence_link_requirement_kind`.
2. A row with **no** `required_for_render` entry is
   non-conforming for the `Certified` and
   `Managed approved` badges; both badges always
   require at least one
   `evidence_link_kind = certification_sheet` or
   `scoreboard_packet` `required_for_render`.

### 3.7 `badge_disclosure_surface_class`

Closed set of surfaces a rendered badge MAY project
onto. Re-exports and unifies the surface-family
vocabularies of the upstream contracts:

- `start_center_bundle_card`
- `bundle_detail_page`
- `gallery_or_browse_bundles`
- `update_detail_entry`
- `project_doctor_recommended_action`
- `support_export_packet`
- `certification_sheet_artifact`
- `release_evidence_packet`
- `claim_publication_destination`
- `cli_or_headless_listing`
- `marketing_copy_lane`

Rules (frozen):

1. Every row enumerates the surfaces it MAY render
   on through `disclosure_surfaces[]`; surfaces
   not listed cannot render the row's badge.
2. `marketing_copy_lane` is the only surface where
   §11 separation rules apply: a row MAY allow the
   badge text to appear in marketing copy as long as
   the marketing copy resolves through the typed
   badge id (no parallel marketing label).

### 3.8 `badge_marketing_separation_rule_class`

Closed set of separation rules between badge
vocabulary, marketing copy, and workflow-bundle
ownership:

- `badge_is_proof_marker_not_marketing_label` —
  the badge id renders as a proof marker; marketing
  copy that paraphrases the marker as
  "industry-leading", "popular", "recommended",
  "best-in-class", "fastest", or any other
  promotional phrase is non-conforming.
- `badge_decoupled_from_workflow_bundle_ownership` —
  the badge resolves through bundle source class
  and bundle status class; it does NOT resolve
  through the bundle's organisation, signer entity
  name, or marketing tier. A `badge.certified` does
  not imply Aureline brand ownership; a
  `badge.managed_approved` does not imply the
  organisation's brand ownership beyond the typed
  `bundle_signer_source_class`.
- `badge_decoupled_from_recommendation_ranking` —
  the badge does NOT imply ranking. A
  `badge.certified` row may render below a
  `badge.community` row in a gallery; ranking is
  owned by the recommendation contract.
- `badge_decoupled_from_install_authority` — the
  badge does not imply install authority. Render
  in any surface MUST NOT trigger install or
  activation; install rides through the
  environment-starter summary contract.

Rules (frozen):

1. Every row carries
   `marketing_separation_rules[]` with all four
   classes listed (the set is the same for every
   row; it is the contract's separation invariant
   surface, not a per-row choice). Missing any
   class is non-conforming.
2. Marketing copy that violates any class
   resolves through the
   [`claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md)
   forbidden-generic-copy gate, not through this
   contract; this contract pins only the
   separation rules a publication binding inherits.

## 4. Badge-freshness evaluation contract

The evaluation function the freshness gate runs on every
manifest the bundle surfaces project onto a badge:

```
fn evaluate_badge_freshness(
    manifest: WorkflowBundleManifestRecord,
    row: BundleBadgeFreshnessRowRecord,
    evidence_packet_index: EvidencePacketIndex,
    now_utc: Timestamp,
) -> BadgeFreshnessEvaluation
```

### 4.1 Inputs

- One `WorkflowBundleManifestRecord` from
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).
- One row from
  [`bundle_badge_freshness_rows.yaml`](../../artifacts/workflow/bundle_badge_freshness_rows.yaml)
  matching the candidate `bundle_evidence_badge_id`.
- The evidence-packet index resolving every
  `evidence_link_ref.evidence_ref` to a record carrying
  the `captured_at`, `stale_after`, `source_revision`,
  `trigger_revision`, and `proof_class_id` required by
  [`evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml).
- The current UTC time.

### 4.2 Decision order

1. **Status backing.** If
   `manifest.bundle_status_class` is not in the row's
   `backing_bundle_status_classes[]`, downgrade
   immediately with trigger
   `manifest_status_class_does_not_back_badge`.
2. **Source backing.** If
   `manifest.bundle_source_class` is not in the row's
   `backing_bundle_source_classes[]`, downgrade
   immediately with trigger
   `manifest_status_class_does_not_back_badge`.
3. **Required evidence link presence.** For every
   `required_evidence_links[]` entry whose
   `requirement_kind = required_for_render`, the
   manifest's `evidence_link_refs[]` MUST carry at
   least one entry of the matching `evidence_link_kind`
   (and matching `scoreboard_family_id` /
   `public_proof_packet_shape` when the row pins them).
   Missing any required link → state
   `evidence_missing_required_link`, target
   `target_publish_blocked_no_badge`, gate
   `publish_fail_closed_block`, blocker
   `required_evidence_link_missing`.
4. **Signer continuity.** If
   `manifest.signer_continuity_class` resolves to a
   `lost_continuity_*` value or `manifest.signature_class`
   is invalidated → state
   `evidence_signature_or_continuity_broken`, target
   per §5, gate `publish_fail_closed_block`, blocker
   `signer_continuity_or_signature_broken`.
5. **Freshness evaluation.** For every required
   evidence packet, evaluate `now_utc` against
   `captured_at + stale_after`. The proof class's
   `max_stale_after` is the ceiling; a stricter packet-
   local `stale_after` always wins per the upstream
   [`evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
   §metadata_evaluation_contract.
   - All packets within cadence → state
     `evidence_fresh_within_cadence`, target
     `none` (badge renders verbatim), gate
     `publish_allowed`.
   - At least one within first warning threshold but
     under `max_stale_after` → state
     `evidence_aging_within_window`, target per row's
     `aging_downgrade_target`, gate per row's
     `aging_publish_gate`.
   - At least one past `max_stale_after` → state
     `evidence_stale_past_window`, target per row's
     `stale_downgrade_target`, gate per row's
     `stale_publish_gate`.
6. **Mirror / source posture.** If
   `manifest.mirror_or_offline_packaging_posture =
   packaging_posture_unknown` or any required mirror
   freshness ref is missing → state
   `evidence_age_unknown_offline_or_unreachable`,
   target per §5, gate
   `publish_fail_closed_warn` or
   `publish_fail_closed_block` per row.
7. **Compatibility narrowing.** If the manifest's
   `compatible_aureline_range` narrows below the
   installed runtime, or an evidence packet's
   `compatibility_state_class` is `compatible_narrowed`
   or `incompatible_blocking`, fold the trigger
   `compatibility_narrowed_below_runtime` into the
   row's evaluation per §9.
8. **Lifecycle.** If the manifest's
   `lifecycle_state_class` resolves to a deprecation
   or removal class, fold the trigger
   `lifecycle_deprecation_or_removal` into the
   evaluation; downgrade target is
   `target_deprecated_badge`.

### 4.3 Output

A `bundle_badge_freshness_evaluation_record` carrying:

- `evaluated_badge_id` (`bundle_evidence_badge_id`).
- `bundle_id` and `bundle_revision`.
- `badge_freshness_state_class` (§3.1).
- `triggered_downgrade_triggers[]`
  (`badge_downgrade_trigger_class`, §3.2).
- `resulting_badge_id` (`bundle_evidence_badge_id` or
  `null` when target is
  `target_publish_blocked_no_badge`).
- `downgrade_target_class` (§3.3).
- `publish_gate_class` (§3.4).
- `stale_badge_blocker_classes[]` (§3.5) — non-empty
  when gate is `publish_fail_closed_*`.
- `disabled_reason_codes[]` re-exported from the
  change-preview schema.
- `referenced_evidence_packet_refs[]` — the evidence
  packets the evaluation read.
- `referenced_proof_class_ids[]`.
- `claim_narrowing_class` re-exported from drift
  contract §3.12 (`no_narrowing_informational` when
  state is `evidence_fresh_within_cadence`).
- `bundle_card_freshness_overlay` re-exported from
  Start Center bundle surfaces.
- `evaluated_at` — monotonic timestamp.
- `provenance_record_class` re-exported from drift
  contract §3.14 (the evaluation is later auditable).
- `keyboard_reachable = true`.

### 4.4 Evaluation rules (frozen)

1. **One row, one verdict per evaluation.** A row
   evaluation produces exactly one record. A surface
   that mints two parallel verdicts on one badge is
   non-conforming.
2. **Strictest verdict wins.** When multiple
   triggers apply, the gate resolves to the
   strictest verdict per §3.4 rule 4.
3. **Fail-closed by default.** Missing freshness
   metadata, missing proof class binding, or an
   unresolved manifest contradiction MUST resolve to
   `publish_fail_closed_block` rather than
   `publish_allowed`. This contract inherits the
   upstream
   [`evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
   §metadata_evaluation_contract `Missing freshness
   metadata fails closed for scorecard, signoff,
   claim, and promotion use` rule verbatim.
4. **No silent badge promotion.** An evaluation
   never widens authority. A `badge.community` row
   that produces a `evidence_fresh_within_cadence`
   state stays `badge.community`; it does not
   promote to `badge.managed_approved` or
   `badge.certified` because freshness alone is not
   the basis for promotion. Promotion rides through
   a new manifest revision.

## 5. Per-badge row table

The closed pairings between
`bundle_evidence_badge_id`, the manifest fields that back
the badge, and the downgrade lattice are below. Each
pairing is one row in
[`bundle_badge_freshness_rows.yaml`](../../artifacts/workflow/bundle_badge_freshness_rows.yaml).

### 5.1 `badge.certified`

- `backing_bundle_status_classes`: `certified_current`.
- `backing_bundle_source_classes`: `certified`.
- `backing_bundle_classes`: `launch_bundle`,
  `org_approved_bundle` (when the org-approved row's
  manifest carries certified scoreboard evidence).
- `required_evidence_links` (`required_for_render`):
  `certification_sheet`, `scoreboard_packet`
  (one per required scoreboard family on the bound
  cutline), `docs_version_match`.
- `required_proof_classes`:
  `compatibility_report_proof`,
  `benchmark_publication_proof`,
  `docs_claim_truth_proof`,
  `support_scenario_quality_proof` per the bound
  scoreboard families (rows enumerate the required
  pairing).
- `aging_downgrade_target`:
  `target_retest_pending_badge`.
- `stale_downgrade_target`:
  `target_retest_pending_badge`.
- `aging_publish_gate`:
  `publish_allowed_with_narrowing`.
- `stale_publish_gate`:
  `publish_fail_closed_block`.
- `signature_break_gate`:
  `publish_fail_closed_block`.
- `mirror_unknown_gate`:
  `publish_fail_closed_warn`.

### 5.2 `badge.managed_approved`

- `backing_bundle_status_classes`:
  `managed_approved_current`, `certified_current`
  (when the bundle is org-approved and certified).
- `backing_bundle_source_classes`:
  `managed_approved`, `certified`.
- `backing_bundle_classes`: `org_approved_bundle`.
- `required_evidence_links` (`required_for_render`):
  `certification_sheet` OR `scoreboard_packet`
  (managed-approved bundles MUST carry at least one
  of the two; the row enumerates the required-or
  pairing) plus `docs_version_match`.
- `required_proof_classes`:
  `compatibility_report_proof`,
  `docs_claim_truth_proof`.
- `aging_downgrade_target`:
  `target_retest_pending_badge`.
- `stale_downgrade_target`:
  `target_retest_pending_badge`.
- `aging_publish_gate`:
  `publish_allowed_with_narrowing`.
- `stale_publish_gate`:
  `publish_fail_closed_block`.
- `signature_break_gate`:
  `publish_fail_closed_block`.
- `mirror_unknown_gate`:
  `publish_fail_closed_warn`.

### 5.3 `badge.community`

- `backing_bundle_status_classes`:
  `community_reviewed`, `community_unreviewed`.
- `backing_bundle_source_classes`: `community`.
- `backing_bundle_classes`:
  `design_partner_bundle` (when the partner row's
  signer is community), `imported_user_bundle`
  (when promoted to community on review).
- `required_evidence_links`
  (`required_for_freshness_evaluation` only):
  `docs_version_match` is `recommended_for_audit_only`;
  no link is `required_for_render`. The row renders
  even with a thin evidence inventory.
- `required_proof_classes`:
  `docs_claim_truth_proof` (informational).
- `aging_downgrade_target`:
  `target_community_unreviewed_badge` (when row is
  `community_reviewed`) or `none` (when row is
  already `community_unreviewed`).
- `stale_downgrade_target`:
  `target_community_unreviewed_badge`.
- `aging_publish_gate`: `publish_allowed`.
- `stale_publish_gate`:
  `publish_allowed_with_narrowing`.
- `signature_break_gate`:
  `publish_fail_closed_warn`.
- `mirror_unknown_gate`: `publish_allowed`.

### 5.4 `badge.imported`

- `backing_bundle_status_classes`:
  `imported_pending_review`, `community_reviewed`
  (when the imported row was reviewed without
  promotion to managed-approved or certified),
  `community_unreviewed`.
- `backing_bundle_source_classes`: `imported`.
- `backing_bundle_classes`: `imported_user_bundle`.
- `required_evidence_links`: none required for
  render; ingress review reference is
  `recommended_for_audit_only`.
- `required_proof_classes`: none required.
- `aging_downgrade_target`: `none`.
- `stale_downgrade_target`:
  `target_imported_pending_review_badge`.
- `aging_publish_gate`: `publish_allowed`.
- `stale_publish_gate`:
  `publish_allowed_with_narrowing`.
- `signature_break_gate`:
  `publish_fail_closed_warn`.
- `mirror_unknown_gate`: `publish_allowed`.

### 5.5 `badge.local_draft`

- `backing_bundle_status_classes`: `local_draft`.
- `backing_bundle_source_classes`: `local_draft`.
- `backing_bundle_classes`: `local_draft_bundle`.
- `required_evidence_links`: none.
- `required_proof_classes`: none.
- `aging_downgrade_target`: `none`.
- `stale_downgrade_target`: `none`.
- `aging_publish_gate`:
  `publish_fail_closed_block` (a local-draft badge
  never publishes through the claim-publication
  generator on docs / Help / About / service-health /
  release-notes / CLI-help / evaluation /
  certification-sheet / marketplace destinations).
- `stale_publish_gate`:
  `publish_fail_closed_block`.
- `signature_break_gate`:
  `publish_fail_closed_block`.
- `mirror_unknown_gate`:
  `publish_fail_closed_block`.

### 5.6 `badge.retest_pending`

- `backing_bundle_status_classes`:
  `certified_retest_pending`, `retest_needed`.
- `backing_bundle_source_classes`: `certified`,
  `managed_approved`.
- `backing_bundle_classes`: `launch_bundle`,
  `org_approved_bundle`.
- `required_evidence_links`
  (`required_for_freshness_evaluation`):
  `certification_sheet` (must be present even if
  stale, so the row can resolve the retest hook).
- `required_proof_classes`:
  `compatibility_report_proof` /
  `benchmark_publication_proof` per the bound
  cutline (informational; the row's purpose is to
  carry the retest pressure, not to assert
  freshness).
- `aging_downgrade_target`: `none` (the badge is
  already the downgraded state).
- `stale_downgrade_target`:
  `target_status_unknown_badge` (a retest-pending
  row whose evidence ages further unknown is
  promoted to status-unknown).
- `aging_publish_gate`:
  `publish_allowed_with_narrowing`.
- `stale_publish_gate`:
  `publish_fail_closed_warn`.
- `signature_break_gate`:
  `publish_fail_closed_block`.
- `mirror_unknown_gate`:
  `publish_fail_closed_warn`.

Rules (frozen):

1. Any pairing of `bundle_evidence_badge_id`,
   `bundle_status_class`, and
   `bundle_source_class` not in the table is
   non-conforming. The table re-exports the
   `bundle_evidence_badge_id` →
   `bundle_status_class` mapping from
   [`start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md)
   §3.2 and binds it to evidence requirements.
2. Adding a row, adding a backing status, or
   shifting a downgrade target is additive-minor and
   bumps the schema version.
3. Repurposing an existing row's
   `stale_publish_gate` from `publish_allowed*` to
   `publish_fail_closed_*` (or vice versa) is
   breaking and opens a decision row.

## 6. Stale-badge fail gate

Every claim-publication binding pairing a workflow-bundle
or certification badge with a destination
(docs / Help / About / service-health / release-notes /
CLI-help / evaluation / marketplace / known-limits)
reads
[`stale_badge_fail_rules.yaml`](../../artifacts/workflow/stale_badge_fail_rules.yaml)
to decide whether the badge MAY publish. The gate is the
same gate the
[`claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md)
fail-gate plugs into.

### 6.1 Gate inputs

- One `bundle_badge_freshness_evaluation_record` (§4.3).
- One `claim_publication_binding_record` (re-exported
  from the claim-publication contract) carrying the
  `claim_row_ref`, the `publication_destination_class`,
  and the `effective_claim_posture`.
- The current `policy_context` (epoch, deployment
  profile).

### 6.2 Gate verdicts

The gate emits one
`bundle_badge_publish_gate_verdict_record` carrying:

- `verdict_class`: `badge_publish_gate_class` (§3.4).
- `evaluated_badge_id`,
  `resulting_badge_id`,
  `badge_freshness_state_class`,
  `downgrade_triggers[]`,
  `stale_badge_blocker_classes[]` — re-exported from
  the freshness evaluation.
- `claim_row_ref`,
  `publication_destination_class`,
  `effective_claim_posture` — re-exported from the
  publication binding.
- `gate_decision_summary_ref`.
- `waiver_register_ref` — required when verdict is
  `publish_fail_closed_warn`.
- `evaluated_at`.

### 6.3 Fail-gate rules (frozen)

1. **Stale `Certified` cannot publish without linked
   evidence.** A `badge.certified` row whose
   evaluation produces `evidence_stale_past_window`
   resolves to `publish_fail_closed_block` with
   blocker `required_evidence_link_stale_past_window`.
   Publishing certified wording on the destination
   without a refreshed certification sheet,
   scoreboard packet, or docs version match is
   non-conforming.
2. **Stale `Managed approved` cannot publish without
   linked evidence.** A `badge.managed_approved` row
   whose evaluation produces
   `evidence_stale_past_window` resolves to
   `publish_fail_closed_block` with blocker
   `required_evidence_link_stale_past_window`.
3. **Missing required evidence link blocks closed.**
   `required_evidence_link_missing` always resolves
   to `publish_fail_closed_block`. There is no
   waiver path for a missing certification sheet on
   a `badge.certified` row.
4. **Signer or signature break blocks closed.**
   `signer_continuity_or_signature_broken` always
   resolves to `publish_fail_closed_block`.
5. **Status-unknown gates closed-warn or closed-
   block.** A row whose state is
   `evidence_age_unknown_offline_or_unreachable`
   resolves to `publish_fail_closed_warn` (with a
   waiver path) or `publish_fail_closed_block` per
   the per-row `mirror_unknown_gate`.
6. **Local-draft never publishes through the
   automation gate.** `badge.local_draft` always
   resolves to `publish_fail_closed_block` on every
   gate path; local-draft bundles render only on
   user-scope surfaces and never on
   docs / Help / About / service-health / release-
   notes / CLI-help / evaluation /
   certification-sheet / marketplace destinations.
7. **Waiver-paired warn requires a register entry.**
   A `publish_fail_closed_warn` verdict that
   actually permits publication at all MUST cite a
   `waiver_register_ref` from
   [`artifacts/governance/waiver_register.yaml`](../../artifacts/governance/waiver_register.yaml).
   A warn verdict without a waiver MUST be treated
   by the publication generator as
   `publish_fail_closed_block`.
8. **Strictest gate wins.** When a row evaluates to
   multiple gate paths (for example
   `stale_publish_gate` and `signature_break_gate`),
   the gate verdict is the strictest of the
   applicable verdicts.

## 7. Archetype-proof staleness

A bundle row whose `archetype_bindings[]` cite an
`archetype_row_id` and `archetype_revision` whose
proof scoreboard family
(`workflow_bundle_or_archetype_proof_scoreboard` from
[`workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml))
has aged past its `freshness_cadence` MUST downgrade
the badge before any claim publishes certified wording
on the archetype.

Rules (frozen):

1. Every `badge.certified` evaluation reads at least
   one `evidence_link_kind = scoreboard_packet`
   bound to the
   `workflow_bundle_or_archetype_proof_scoreboard`
   family. Missing that link →
   `evidence_missing_required_link` and
   `publish_fail_closed_block`.
2. The proof packet's `captured_at + stale_after`
   evaluation runs against the proof class's
   `max_stale_after` from
   [`evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
   (`compatibility_report_proof` or
   `benchmark_publication_proof`).
3. Aged-out archetype proof folds
   `archetype_proof_stale_past_cadence` into the
   downgrade trigger list and routes the verdict
   through the row's `stale_publish_gate`.

## 8. Mirror-or-source-change rule

Mirror rotation, source signature change, channel
demotion, or mirror mismatch downgrades the badge
before any surface re-renders the prior badge.

Rules (frozen):

1. The manifest's
   `mirror_or_offline_packaging_posture` is the
   only authoritative source for the mirror /
   air-gap posture (per
   [`workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
   §6 rule 6).
2. A mirror mismatch
   (`mirror_or_source_mirror_mismatch`) downgrades
   the badge with target per the row's
   `mirror_unknown_gate` and trigger
   `mirror_or_source_mirror_mismatch`.
3. A signer rotation or signature revocation
   produces `evidence_signature_or_continuity_broken`
   and resolves to `publish_fail_closed_block` on
   `Certified` and `Managed approved` rows.
4. A `cross_channel_demotion` or
   `branch_or_fork_revision` shift folds
   `channel_demoted_or_branch_forked` into the
   downgrade trigger list; downgrade target is
   `target_deprecated_badge` when paired with a
   `lifecycle_state_class` of
   `soft_deprecated_with_successor` or
   `hard_deprecated_remove_recommended`.

## 9. Compatibility-narrowing rule

A bundle whose `compatible_aureline_range` narrows
below the installed runtime, or whose
extension-set / package-bridge / template /
archetype compatibility narrows past the cutline
cadence, downgrades the badge.

Rules (frozen):

1. The freshness evaluation reads the manifest's
   `compatible_aureline_range` and any
   `compatibility_report` evidence packets bound to
   the row's `required_proof_classes`.
2. A `compatibility_state_class` of
   `compatible_narrowed`,
   `incompatible_blocking`, or
   `incompatible_review_required` folds
   `compatibility_narrowed_below_runtime` into the
   downgrade trigger list.
3. `incompatible_blocking` resolves to
   `publish_fail_closed_block` on `Certified` and
   `Managed approved` rows; `compatible_narrowed`
   resolves to `publish_allowed_with_narrowing` per
   the row's `aging_publish_gate`.

## 10. Surface invariants (cross-cutting)

1. **One row, many surfaces.** Start Center bundle
   card, bundle detail page, gallery, update detail,
   project-doctor, support-export, certification
   sheet, claim publication, CLI / headless, and
   marketing copy lane all read the same
   `bundle_badge_freshness_row_record` and the same
   `bundle_badge_freshness_evaluation_record`. A
   surface that mints a parallel row or evaluation
   id is non-conforming.
2. **Badge resolves to manifest fields.** Every
   rendered badge cites the manifest's backing
   `bundle_status_class`, `bundle_source_class`,
   `bundle_class`, `signer_continuity_class`,
   `signature_class`, and `mirror_or_offline_
   packaging_posture` per §5 verbatim. A badge
   whose backing fields contradict the manifest is
   non-conforming.
3. **Freshness chip cannot be hidden.** A surface
   whose row evaluates to
   `evidence_aging_within_window`,
   `evidence_stale_past_window`, or
   `evidence_age_unknown_offline_or_unreachable`
   MUST render the matching
   `bundle_card_freshness_overlay` per
   [`start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md)
   §3.6. Suppressing the chip for UI density is
   non-conforming.
4. **No silent badge promotion.** A surface that
   widens a row from `badge.community` to
   `badge.managed_approved` or from
   `badge.retest_pending` back to
   `badge.certified` without a new manifest
   revision is non-conforming.
5. **No silent install authority on render.**
   Rendering a badge does not commit any
   inventory; install rides through the
   environment-starter summary contract per
   [`workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
   §6 rule 3.
6. **Audit-attributable.** Every evaluation
   record carries at least one
   `provenance_record_class` entry per drift
   contract §3.14 so the row is later auditable
   as a typed lifecycle event rather than only as
   a transient diff list.
7. **One id, every surface.** The
   `bundle_evidence_badge_id` rendered on Start
   Center is the same id rendered on the
   certification sheet, the support export, and
   the claim publication binding. A surface that
   mints `featured_badge`, `team_badge`,
   `pro_badge`, or `verified_badge` parallel ids
   is non-conforming.

## 11. Badge vs marketing copy vs workflow-bundle ownership

The badge vocabulary is a **proof marker**, not
marketing copy and not workflow-bundle ownership.
The separation is enforced by §3.8 rules:

- **Badge ≠ marketing copy.** A badge id renders
  through its typed value (`badge.certified`,
  `badge.managed_approved`, `badge.community`,
  `badge.imported`, `badge.local_draft`,
  `badge.retest_pending`, plus the packaging-
  posture badges). Marketing copy that paraphrases
  the badge as "industry-leading", "best-in-class",
  "premium", "officially endorsed", "approved",
  "trusted", "recommended", or "popular" is
  forbidden by the
  [`claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md)
  forbidden-generic-copy gate. The publication
  binding inherits the gate verbatim; this contract
  pins only that the badge id resolves through the
  typed value, never through promotional language.
- **Badge ≠ workflow-bundle ownership.** A
  `badge.certified` row identifies the bundle as
  Aureline-signed; it does NOT identify Aureline
  as the bundle's owner for branding purposes.
  Bundle ownership for branding rides through
  the manifest's `bundle_signer_source_class` and
  the workflow-bundle id register, NOT through
  the badge. A `badge.managed_approved` row
  identifies the org as the signer; it does NOT
  promote the org's brand into Aureline-equivalent
  authority.
- **Badge ≠ recommendation ranking.** The badge
  does not imply ranking. A `badge.certified` row
  may render below a `badge.community` row in a
  gallery; ranking is owned by the recommendation
  contract (out of scope here per the spec).
- **Badge ≠ install authority.** Rendering the
  badge MUST NOT trigger install or activation;
  install rides through the environment-starter
  summary contract.

Rules (frozen):

1. Every row in
   [`bundle_badge_freshness_rows.yaml`](../../artifacts/workflow/bundle_badge_freshness_rows.yaml)
   carries `marketing_separation_rules[]` with all
   four classes per §3.8 rule 1. Missing any class
   on any row is non-conforming.
2. The publication binding's
   `forbidden_marketing_phrase_class` set
   (re-exported from the claim-publication
   contract) is the authoritative blocker for
   marketing-copy violations on the publish gate.
3. A surface that re-labels a `badge.community`
   row as `badge.officially_supported` to make the
   gallery look denser is non-conforming.

## 12. Acceptance mapping

- **Reviewers can trace any bundle / support badge
  back to current evidence and freshness state.**
  §4 (badge-freshness evaluation contract), §5 (per-
  badge row table), §3.6 (`badge_evidence_link_
  requirement_kind`), and §10.1 (one row, many
  surfaces) together freeze the trace contract.
  Every fixture cites the evidence link refs the
  row read and the resulting state, target, and
  gate.
- **Badge downgrade rules are machine-usable
  enough to plug into claim-publication
  automation later.** §3.2 (downgrade triggers),
  §3.3 (downgrade targets), §3.4 (publish gates),
  §3.5 (stale-badge blockers), §6 (stale-badge
  fail gate), and §4.3 (evaluation record output)
  are read directly by the
  [`claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md)
  binding generator. The verdict record shape is
  the exact shape that contract reads.
- **The contract distinguishes badge vocabulary
  from marketing copy and from workflow-bundle
  ownership.** §3.8
  (`badge_marketing_separation_rule_class`) and
  §11 freeze the four separation rules. Every row
  carries the four classes verbatim.

## 13. Source anchors

- `.t2/docs/Aureline_PRD.md:254` — devcontainer
  compatibility, workspace templates, and optional
  prebuild snapshots are part of the remote story
  from day one (badges inherit the same disclosure
  posture).
- `.t2/docs/Aureline_PRD.md:1259` — remote
  workspaces should accept repo-defined
  devcontainer metadata and optional prebuild
  snapshots so environment setup is reproducible
  and accelerable (badges inherit freshness off the
  same metadata).
- `.t2/docs/Aureline_PRD.md:2328` — intelligent
  project scaffolding and generation: starter
  templates and agentic setup for new services /
  apps / modules using team standards (badge
  vocabulary stays a proof marker, not a marketing
  label).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:802` —
  §6.9 templates, starters, and prebuilds: source
  class, support class, runtime / toolchain,
  freshness, setup actions, always-available
  bypass path, side-effect envelope (badge
  freshness mirrors the same axes).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:6346`
  — §17.7 scaffolding, generation, and template
  health: signal classes (`live`, `cached`,
  `policy_evaluated`, `not_checked`) — badge
  freshness mirrors the same evidence-freshness
  signals.
- `.t2/docs/Aureline_Milestones_Document.md:3787`
  — environment-capsule schema draft, workspace-
  template seed, and prebuild-metadata baseline
  (badges read the same capsule identity).

## 14. Linked artifacts

- Workflow-bundle manifest, component inventory,
  and source-class contract (source of truth for
  `bundle_class`, `bundle_source_class`,
  `bundle_status_class`, `bundle_signer_source_class`,
  `signer_continuity_class`, `signature_class`,
  `mirror_or_offline_packaging_posture`,
  `evidence_link_kind`, `evidence_age_class`,
  `lifecycle_state_class`, `successor_recommendation_class`,
  `retest_needed_posture`, and `archetype_bindings[]`):
  [`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  and
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).
- Bundle install / update review, change preview,
  and rollback-checkpoint contract (source of truth
  for `disabled_reason_code`, `change_axis`,
  `compatibility_state_class`):
  [`/docs/workflow/bundle_change_review_contract.md`](./bundle_change_review_contract.md)
  and
  [`/schemas/workflow/bundle_change_preview.schema.json`](../../schemas/workflow/bundle_change_preview.schema.json).
- Bundle drift, local-override merge, and remove-
  bundle safety contract (source of truth for
  `claim_narrowing_class`,
  `successor_bundle_suggestion_class`,
  `drift_state_class`, and provenance-record
  classes):
  [`/docs/workflow/bundle_drift_and_removal_contract.md`](./bundle_drift_and_removal_contract.md)
  and
  [`/schemas/workflow/bundle_drift_row.schema.json`](../../schemas/workflow/bundle_drift_row.schema.json).
- Start Center bundle card, bundle detail page,
  and evidence-badge contract (source of truth for
  `bundle_evidence_badge_id`,
  `bundle_card_freshness_overlay`,
  `bundle_card_surface_family`,
  `bundle_detail_page_section_id`):
  [`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md).
- Public-proof scoreboards and versioned
  workflow-bundle / archetype IDs (source of truth
  for `proof_class_id`, `freshness_cadence`,
  `downgrade_trigger`, `scoreboard_family_id`,
  `public_proof_packet_shape`,
  `workflow_bundle_id`, `archetype_row_id`,
  `archetype_revision`):
  [`/docs/qe/public_proof_scoreboards.md`](../qe/public_proof_scoreboards.md)
  and
  [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml).
- Evidence freshness SLOs (source of truth for
  proof-class freshness cadences, max stale-after
  windows, stale-propagation profiles, and the
  fail-closed-by-default rule):
  [`artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  and
  [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md).
- Claim-publication automation contract (consumer
  of this contract's gate verdict; source of truth
  for `claim_row_ref`, `effective_claim_posture`,
  `publication_destination_class`, and the
  forbidden-generic-copy / marketing-copy gate):
  [`/docs/release/claim_publication_automation_contract.md`](../release/claim_publication_automation_contract.md)
  and
  [`/schemas/governance/claim_publication_binding.schema.json`](../../schemas/governance/claim_publication_binding.schema.json).
- Template-and-prebuild contract (source of truth
  for `policy_notice_class`,
  `availability_narrowing_class`,
  `bypass_path_id`):
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- Waiver register (consumer of
  `publish_fail_closed_warn` verdicts):
  [`/docs/governance/waiver_register_contract.md`](../governance/waiver_register_contract.md)
  and
  [`artifacts/governance/waiver_register.yaml`](../../artifacts/governance/waiver_register.yaml).
- Bundle badge freshness rows (machine-readable
  companion to this contract):
  [`/artifacts/workflow/bundle_badge_freshness_rows.yaml`](../../artifacts/workflow/bundle_badge_freshness_rows.yaml).
- Stale-badge fail rules (machine-readable
  companion to this contract):
  [`/artifacts/workflow/stale_badge_fail_rules.yaml`](../../artifacts/workflow/stale_badge_fail_rules.yaml).
- Worked-example fixtures:
  [`/fixtures/workflow/bundle_certification_cases/`](../../fixtures/workflow/bundle_certification_cases/).
