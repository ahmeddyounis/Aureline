# Start Center bundle card, bundle detail page, and evidence-badge contract

This document freezes the cross-surface disclosure contract every
**Start Center bundle card**, **gallery / browse-bundles row**,
**update / what's-changed entry**, **export-facing bundle summary**,
and **bundle detail page** inherits before workflow-bundle install,
review, and update surfaces are implemented. The goal is that
workflow bundles become **inspectable artifacts** at project entry —
not opaque promises behind one-click setup — so certification,
support, and offline posture are legible **before** any side effect
runs.

The companion machine-readable schemas live at:

- [`/schemas/ux/bundle_card.schema.json`](../../schemas/ux/bundle_card.schema.json)
- [`/schemas/ux/bundle_detail_page.schema.json`](../../schemas/ux/bundle_detail_page.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/start_center_bundle_cases/`](../../fixtures/ux/start_center_bundle_cases/)

This contract is normative for the disclosure posture. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, or milestone document
anchors quoted in §15, those sources win and this document plus its
companion schemas and fixtures update in the same change. Where a
downstream Start Center bundle row, gallery row, update entry,
export summary, or detail surface mints a parallel vocabulary, this
contract wins and the surface is non-conforming.

This contract mints **no** new bundle, identity, source, status,
packaging, signer, channel-relation, dependency-marker,
component-kind, evidence-link, lifecycle, successor, evidence-age,
retest-posture, removal-surface, or rollback-surface vocabulary.
Every axis on a card, detail page, or badge resolves through the
closed sets frozen in
[`/docs/workflow/workflow_bundle_object_model.md`](../workflow/workflow_bundle_object_model.md)
(§3.1–§3.15, §4–§9). It also re-exports — by reference — the
Start Center disclosure vocabulary frozen in
[`/docs/ux/start_center_contract.md`](./start_center_contract.md)
(§3.2 zones, §3.3 account posture, §3.4 freshness / absence,
§3.5 privacy reduction, §3.7 primary actions, §3.9 disclosure
classes) and the template-and-prebuild vocabulary frozen in
[`/docs/ux/template_and_prebuild_contract.md`](./template_and_prebuild_contract.md)
(§3.3 support class, §3.8 availability narrowing, §3.14
policy-notice class).

## Who reads this contract

- **Start Center, gallery, browse-bundles, and update-detail
  surface authors** rendering bundle rows or update entries that
  let a user inspect a bundle before install, after install, or
  during retest. Every card, row, badge, and section resolves
  through a record shape defined here.
- **Designers** sizing bundle cards, badge chips, detail-page
  sections, certification copy, and successor banners so
  source / status / freshness / offline posture / support class
  land same-weight with the primary `Review bundle` action.
- **Docs, support, compatibility, certification, and audit
  authors** attributing bundle-row evidence to the same record
  kinds the shell renders, so a later certification sheet,
  lifecycle audit, support export, and CLI / headless bundle
  listing read the same truth.

## 1. Scope

- Freeze one `start_center_bundle_card_record` per bundle row
  on the Start Center, gallery, browse-bundles, update-detail,
  and export-facing summary surfaces. The card is the
  **inspectable summary** every entry surface renders before
  the user opens the detail page.
- Freeze one `start_center_bundle_detail_page_record` per
  bundle detail-page render. The detail page enumerates the
  eleven component-kind sections (extension set, profile
  presets, surface presets, task / launch / debug recipes,
  template / scaffold, docs / tour / glossary packs, migration
  maps, certification targets, evidence links) plus mirror /
  offline posture, changelog, known-limits, successor /
  lifecycle, and the typed `Review bundle` and removal /
  rollback affordances.
- Freeze the **badge vocabulary** (§3) every card, detail
  page, gallery row, update-detail entry, and export summary
  uses. Badges resolve to canonical bundle or archetype
  objects from
  [`/docs/workflow/workflow_bundle_object_model.md`](../workflow/workflow_bundle_object_model.md);
  marketing labels are non-conforming.
- Freeze the **downgrade rules** that bind badge state to the
  manifest's `bundle_status_class`, `evidence_age_class`,
  `retest_needed_posture`, `lifecycle_state_class`, and
  `successor_recommendation_class`. A card whose badge says
  `Certified` while the manifest is `certified_retest_pending`
  is non-conforming.
- Freeze the **cross-surface projection rules** (§9) so
  bundle cards, detail pages, gallery / update entries, and
  export summaries render the same source / status / signer /
  packaging / lifecycle truth regardless of family.
- Reserve the **Review bundle** primary action (§7), the
  removal / rollback affordances (§8), and the successor
  banner (§9.3) so install authority, removal authority, and
  upgrade pressure are typed verbs — not free-form copy.

## 2. Out of scope

- The bundle **manifest** itself.
  [`/docs/workflow/workflow_bundle_object_model.md`](../workflow/workflow_bundle_object_model.md)
  owns identity, component inventory, source / status / class
  linkage, and lifecycle truth. This contract reserves
  disclosure slots that resolve through that manifest.
- Final visuals (exact icons, badge geometry, card padding,
  detail-page rail widths). The style guide and shell-zone
  contract own those.
- The actual install, removal, rollback, retest, or update
  **execution engine**. Commit rides through the
  environment-starter summary contract; rollback / removal
  rides through the lifecycle surface kinds frozen in
  [`/docs/workflow/workflow_bundle_object_model.md`](../workflow/workflow_bundle_object_model.md)
  §3.14–§3.15. This contract pins only the disclosure shape.
- Bundle **recommendation ranking** (relative ordering across
  rows, A/B placement, "for you" sorting). Out of scope for
  M00-442 by spec.
- Final user-facing **copy / microcopy**. The shell-interaction-
  safety contract and the UX style guide own the exact
  strings; this contract pins the closed sets the copy
  resolves against.
- **Telemetry wire format**. This contract only tags records
  with a stable `bundle_id` + `bundle_revision` so support /
  audit / measurement plans can attribute evidence to the
  same id the manifest cites.

## 3. Frozen vocabulary (re-exported) and surface-local sets

### 3.0 Re-exported vocabulary

This contract re-exports — by reference, never by redefinition
— the following closed sets:

- `bundle_class`, `bundle_source_class`, `bundle_status_class`,
  `bundle_signer_source_class`, `signer_continuity_class`,
  `signature_class`, `support_class`,
  `bundle_channel_relation_class`,
  `mirror_or_offline_packaging_posture`,
  `bundle_dependency_marker_kind`, `bundle_component_kind`,
  `evidence_link_kind`, `lifecycle_state_class`,
  `successor_recommendation_class`, `evidence_age_class`,
  `retest_needed_posture`, `removal_surface_kind`,
  `rollback_surface_kind`, `pilot_expiry_or_review_class`,
  `disabled_reason_code`, `scoreboard_family_id`,
  `public_proof_packet_shape`, `runtime_compatibility_range`,
  `archetype_row_id`, `archetype_revision`, `bundle_id`,
  `bundle_revision`, `policy_notice_class` — re-exported
  from
  [`/docs/workflow/workflow_bundle_object_model.md`](../workflow/workflow_bundle_object_model.md)
  §3, §4, §5, §8 and the companion schema at
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).
- `start_center_zone`, `start_center_surface_family`,
  `account_opt_in_posture`, `freshness_class`, `absence_class`,
  `privacy_reduction_mode`, `disclosure_class`,
  `primary_action_id`, `next_step_decision_hook`,
  `keyboard_reachability_posture`, `navigation_route_ref`,
  `entry_verb`, `target_kind`, `resulting_mode`,
  `side_effect_envelope`, `startup_state_token` —
  re-exported from
  [`/docs/ux/start_center_contract.md`](./start_center_contract.md)
  §3 and the companion schema at
  [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json).
- `availability_narrowing_class`, `policy_notice_class`,
  `bypass_path_id` — re-exported from
  [`/docs/ux/template_and_prebuild_contract.md`](./template_and_prebuild_contract.md)
  §3.

This contract introduces **six** small vocabularies scoped to
bundle cards, badges, detail pages, and update / export
projections. Each is closed; adding a value is additive-minor
and bumps the schema version, repurposing is breaking and opens
a decision row.

### 3.1 `bundle_card_surface_family`

Which entry-surface family is rendering the card. The set is
closed:

- `start_center_bundle_row` — bundle row on the Start Center
  surface (e.g. recommended launch bundle, recently installed
  bundle row).
- `gallery_browse_bundles_row` — bundle row in the
  browse-bundles gallery (Start Center secondary entry,
  in-session gallery).
- `workspace_switcher_bundle_row` — bundle row re-advertised
  on the workspace-switcher palette / menu / dedicated view.
- `update_detail_entry` — bundle row inside the
  update / what's-changed surface (composes with the
  release-channel update-detail contract).
- `export_facing_bundle_summary` — bundle row inside a
  support export, claim manifest, certified-archetype report,
  or CLI / headless bundle listing.

Rules (frozen):

1. A surface that mints a sixth family (`recommended_for_you`,
   `team_picks`, `marketplace_hero`, etc.) is non-conforming.
2. `update_detail_entry` and `export_facing_bundle_summary`
   render the same card record shape as
   `start_center_bundle_row`; only `bundle_card_surface_family`
   changes.

### 3.2 `bundle_evidence_badge_id`

Closed set of evidence-and-status badge ids cards and detail
pages render. Every badge id resolves to a canonical bundle or
archetype object — never a free-text marketing label.

- `badge.certified` — backed by
  `bundle_status_class = certified_current` plus at least one
  `evidence_link_kind = certification_sheet` and one
  `evidence_link_kind = scoreboard_packet`.
- `badge.managed_approved` — backed by
  `bundle_status_class = managed_approved_current` plus
  `bundle_signer_source_class ∈ {org_policy_signing_root,
  org_mirror_signing_root}`.
- `badge.community` — backed by
  `bundle_status_class ∈ {community_reviewed,
  community_unreviewed}` (further qualified by §4.2 chip
  rules).
- `badge.imported` — backed by
  `bundle_class = imported_user_bundle` and
  `bundle_status_class ∈ {imported_pending_review,
  community_unreviewed, community_reviewed}`.
- `badge.local_draft` — backed by
  `bundle_class = local_draft_bundle` and
  `bundle_status_class = local_draft`.
- `badge.retest_pending` — backed by
  `bundle_status_class ∈ {certified_retest_pending,
  retest_needed}` or
  `evidence_age_class ∈ {aging_within_window,
  stale_past_window, age_unknown}` or
  `retest_needed_posture ∈ {retest_recommended,
  retest_required, retest_blocked}`.
- `badge.deprecated` — backed by
  `bundle_status_class = deprecated_or_archived` or
  `lifecycle_state_class ∈
  {soft_deprecated_with_successor,
  hard_deprecated_remove_recommended,
  removed_unavailable, rollback_recommended}`.
- `badge.status_unknown` — backed by
  `bundle_status_class = status_unknown`,
  `mirror_or_offline_packaging_posture =
  packaging_posture_unknown`, or `lifecycle_state_class =
  lifecycle_unknown`.
- `badge.signed_offline_bundle` — backed by
  `mirror_or_offline_packaging_posture =
  signed_offline_bundle` (offline / air-gap pinned freshness).
- `badge.mirror_only` — backed by
  `mirror_or_offline_packaging_posture = mirror_only`.
- `badge.live_or_mirror` — backed by
  `mirror_or_offline_packaging_posture = live_or_mirror`.
- `badge.live_origin_only` — backed by
  `mirror_or_offline_packaging_posture = live_origin_only`.
- `badge.offline_no_bundle` — backed by
  `mirror_or_offline_packaging_posture = offline_no_bundle`.

Rules (frozen):

1. **Badges resolve to manifest fields.** Every rendered
   badge cites a manifest field listed above. A badge whose
   backing field would be missing or contradictory in the
   manifest is non-conforming. A `badge.certified` rendered
   while the manifest's `bundle_status_class` is
   `community_unreviewed` is non-conforming.
2. **No marketing-only badges.** A surface that mints
   `badge.popular`, `badge.recommended`, `badge.trending`,
   `badge.fast_setup`, `badge.zero_config`, or any badge not
   listed above is non-conforming.
3. **Status badges and packaging badges co-exist.** A card
   MAY render both a status-class badge (e.g.
   `badge.certified` or `badge.retest_pending`) and a
   packaging-posture badge (e.g. `badge.mirror_only` or
   `badge.signed_offline_bundle`); the two axes are
   independent.
4. **Persona / stack tags are not badges.** Persona, stack,
   archetype-binding tags render through `archetype_tag_refs[]`
   (§4.1), not through `bundle_evidence_badge_id`. A surface
   that promotes a persona tag into the badge slot is
   non-conforming.

### 3.3 `bundle_card_action_id`

Closed set of typed primary and secondary affordances on a
bundle card. Free-form action labels are non-conforming.

- `bundle_card.review_bundle` — open the bundle detail page;
  required on every non-takeover card.
- `bundle_card.compare_with_installed` — open a side-by-side
  diff between this bundle revision and the currently
  installed revision; required when `previous_revision_ref`
  is non-null and the family is `update_detail_entry`.
- `bundle_card.view_certification_sheet` — open the linked
  `certification_sheet` evidence; required when the card
  renders `badge.certified` or `badge.retest_pending` on a
  certified-class bundle.
- `bundle_card.view_changelog` — open the changelog section
  of the detail page; required when the card is rendered on
  the `update_detail_entry` family.
- `bundle_card.view_known_limits` — open the known-limits
  section of the detail page; required when an
  `evidence_link_kind = known_limit_note` is present on the
  manifest.
- `bundle_card.view_successor` — navigate to the successor
  bundle row; required when `successor_recommendation_class
  ∈ {successor_recommended_same_class,
  successor_recommended_different_class,
  successor_recommended_first_party_native}`.
- `bundle_card.export_summary_row` — emit this card as a
  support-export row; available on every family.
- `bundle_card.dismiss` — dismiss the card from the current
  surface (does not affect install state).

Rules (frozen):

1. **Review bundle stays primary.** A card that promotes
   any other action above `bundle_card.review_bundle` is
   non-conforming. The primary action MUST be `review_bundle`
   on every non-takeover card.
2. **No silent install.** A card MUST NOT mint an
   `Install`, `Apply`, `Add`, or `Get started` affordance as
   the primary action. Install authority rides through the
   `environment_starter_summary_record` after the user
   reaches the detail page and reviews the side-effect
   envelope.
3. **No silent install authority on selection.** Clicking a
   card MUST resolve to `bundle_card.review_bundle` (open
   the detail page) — never to a commit. A surface that
   commits inventory on first click is non-conforming.

### 3.4 `bundle_detail_page_section_id`

Closed set of detail-page section ids. Every section MUST
render as a key in the page record (an empty array is allowed,
but the key MUST NOT be missing) so reviewers see absence.

- `section.identity_and_class` — `bundle_id`,
  `bundle_revision`, `bundle_class`, `bundle_source_class`,
  `bundle_status_class`, `support_class`, signer continuity,
  signature class, channel relation,
  `compatible_aureline_range`, archetype bindings,
  `cutline_refs`, `previous_revision_ref`.
- `section.extension_set` — `extension_sets[]` from the
  manifest's component inventory.
- `section.presets` — combines `profile_presets[]` and
  `surface_presets[]` so reviewers see the full preset
  surface a bundle would project.
- `section.task_recipes` — `task_recipes[]`.
- `section.launch_recipes` — `launch_recipes[]`.
- `section.debug_recipes` — `debug_recipes[]`.
- `section.template_or_scaffold` — `template_or_scaffold_refs[]`.
- `section.docs_pack` — `docs_packs[]`.
- `section.tour_pack` — `tour_packs[]`.
- `section.glossary_pack` — `glossary_packs[]`.
- `section.migration_mapping` — `migration_mappings[]`.
- `section.certification_targets` — `certification_targets[]`.
- `section.evidence_links` — `evidence_links[]`.
- `section.mirror_or_offline_posture` —
  `mirror_or_offline_packaging_posture`,
  `mirror_freshness_ref`, `offline_bundle_freshness_ref`,
  applicable `availability_narrowing_class`, and applicable
  `policy_notice_class` chip refs.
- `section.changelog` — typed changelog row refs scoped to
  the bundle's previous → current revision pair (uses the
  release-detail vocabulary; this contract reserves the slot
  but does not redefine the changelog row shape).
- `section.known_limits` — known-limit-note refs from the
  manifest's `evidence_link_refs`.
- `section.lifecycle_and_successor` —
  `lifecycle_state_class`, `evidence_age_class`,
  `retest_needed_posture`, `successor_recommendation` block,
  `certification_sheet_refs`, `removal_surface_kind`,
  `rollback_surface_kind`,
  `disclosed_freshness_window_class`.
- `section.dependency_markers` — `dependency_markers[]` from
  the manifest.
- `section.review_actions` — typed `Review bundle` /
  `Compare with installed` / `Remove` / `Rollback` /
  `Retest now` affordances per §7 and §8.

Rules (frozen):

1. **Sections are required slots.** Every section id is
   required on every detail page; missing keys are
   non-conforming. An empty array is allowed where the
   manifest emits an empty inventory slot.
2. **Sections never invent components.** A section MUST
   read its rows from the manifest's component inventory
   (§3.8 of the workflow-bundle object model). A surface
   that fabricates a row not present in the manifest is
   non-conforming.
3. **Sections render absence visibly.** An empty
   `section.certification_targets` on a `certified_current`
   manifest is impossible (the manifest would downgrade);
   on a `community_unreviewed` manifest it MUST render
   verbatim (empty slot, with a chip explaining "no
   certification targets bound").

### 3.5 `bundle_card_zone`

Where a bundle card may render relative to the host surface's
zone vocabulary. The set is closed:

- `start_center.primary_work_resume` — only when the card
  is a recommended launch bundle paired with a primary
  action; never used for promotional placement.
- `start_center.secondary_entry` — bundle rows in the
  Start Center's secondary-entry zone (browse bundles entry,
  recommended bundles for the active archetype).
- `gallery.body` — gallery / browse-bundles surface body.
- `update_detail.body` — update / what's-changed surface
  body.
- `export_summary.body` — support export, claim manifest,
  CLI / headless listing body.

Rules (frozen):

1. `start_center.primary_work_resume` MUST NOT host a
   marketing-only or unreviewed bundle row. Only bundles
   whose `bundle_status_class ∈ {certified_current,
   managed_approved_current}` and whose
   `mirror_or_offline_packaging_posture` resolves cleanly
   in the current envelope MAY render here.
2. Bundle rows MUST NOT render in any zone above
   `start_center.primary_work_resume` (per
   start-center-contract §3.2 zone rules). A bundle row in
   `disclosure_band` is non-conforming.
3. Persona / stack tags, badge chips, and packaging chips
   render inline on the card; promoting them into a separate
   higher zone is non-conforming.

### 3.6 `bundle_card_freshness_overlay`

Closed set of freshness overlays the card renders on top of
the manifest's `evidence_age_class` to make freshness legible
on every entry surface.

- `freshness.fresh_current` — manifest's
  `evidence_age_class = fresh_current` (chip hidden or muted
  is permitted; presence is asserted on the record).
- `freshness.aging_within_window` — manifest's
  `evidence_age_class = aging_within_window`; chip MUST
  render.
- `freshness.stale_past_window` — manifest's
  `evidence_age_class = stale_past_window`; chip MUST render
  with a typed retest hook.
- `freshness.age_unknown` — manifest's
  `evidence_age_class = age_unknown`; chip MUST render with
  the `disabled_reason_code` that explains why age could not
  be determined.

Rules (frozen):

1. **Freshness chip cannot be hidden.** A card whose
   manifest is `aging_within_window`, `stale_past_window`,
   or `age_unknown` MUST render the freshness chip
   verbatim. Suppressing the chip for UI density is
   non-conforming.
2. **No overclaim across overlays.** A card whose freshness
   overlay reads `freshness.fresh_current` while the
   manifest emits `evidence_age_class = stale_past_window`
   is non-conforming.

## 4. Bundle card record

Every Start Center, gallery, browse-bundles, update-detail, and
export-facing summary surface emits exactly one
`start_center_bundle_card_record` per bundle row. The card is
the inspectable summary every entry surface renders before the
detail page is reached.

### 4.1 Required fields

- `record_kind = start_center_bundle_card_record`.
- `card_id` — opaque, stable per render.
- `bundle_id` — opaque ref; resolves through
  [`/docs/workflow/workflow_bundle_object_model.md`](../workflow/workflow_bundle_object_model.md)
  §5.
- `bundle_revision` — integer ≥ 1 from the manifest.
- `bundle_revision_semver` — optional semver string.
- `bundle_card_surface_family` (§3.1).
- `bundle_card_zone` (§3.5).
- `bundle_class`, `bundle_source_class`,
  `bundle_status_class`, `bundle_signer_source_class`,
  `signer_continuity_class`, `signature_class`,
  `support_class`, `bundle_channel_relation_class`,
  `mirror_or_offline_packaging_posture` — re-exported
  verbatim from the manifest. A card that emits a different
  value than the manifest is non-conforming.
- `archetype_tag_refs[]` — at least one when
  `bundle_class ∈ {launch_bundle, org_approved_bundle,
  design_partner_bundle}`. Each entry names an
  `archetype_row_id` and `archetype_revision`.
- `persona_or_stack_tag_refs[]` — opaque tag refs (e.g.
  `persona_tag:typescript_web_app_developer`,
  `stack_tag:rust_systems`). Resolves to canonical
  archetype / launch-wedge tags; free-text persona strings
  are non-conforming.
- `compatible_aureline_range` — `runtime_compatibility_range`
  block re-exported from the manifest.
- `evidence_badge_ids[]` — ordered list of
  `bundle_evidence_badge_id` (§3.2). Each id MUST be backed
  by the manifest field listed in §3.2.
- `freshness_overlay` (§3.6).
- `availability_narrowing_class` — re-exported from
  template-and-prebuild §3.8 when the bundle is narrowed in
  the current envelope (`mirror_only_subset`,
  `signed_offline_bundle_subset`, `policy_blocked`,
  `signature_review_required`, `target_runtime_unavailable`).
  `null` when not narrowed.
- `policy_notice_ref` — opaque ref to a
  `starter_policy_notice_record` when a notice applies;
  `null` otherwise.
- `local_offline_availability_class` — closed set drawn
  from the manifest's
  `mirror_or_offline_packaging_posture`:
  - `available_live`
  - `available_via_mirror`
  - `available_offline_signed_bundle`
  - `unavailable_offline_or_mirror_only`
  - `availability_unknown`
- `disabled_reason_code` — optional; required when the
  card renders visible-but-disabled (`policy_narrowed_fleet`,
  `mirror_only_cached_subset`, `signature_review_required`,
  `uncertified_excluded`, `target_runtime_unavailable`,
  `network_unreachable`).
- `card_actions[]` — at least one entry; MUST include
  `bundle_card.review_bundle` on every non-takeover card.
- `successor_recommendation_class` — re-exported from the
  manifest; required to be present (`no_successor` is
  allowed but never silent).
- `lifecycle_state_class`, `evidence_age_class`,
  `retest_needed_posture` — re-exported from the manifest.
- `bundle_detail_page_ref` — opaque ref to the companion
  `start_center_bundle_detail_page_record` the card opens.
- `keyboard_reachable = true` — non-conforming if `false`.
- `minted_at` — monotonic timestamp.

### 4.2 Disclosure rules

1. **Source-class chip verbatim.** The card's source-class
   chip cites `bundle_source_class` verbatim
   (`certified`, `managed_approved`, `community`,
   `imported`, `local_draft`). Aliases (`semi_certified`,
   `team_pick`, `community_premium`) are non-conforming.
2. **Status-class chip verbatim.** The card's status chip
   cites `bundle_status_class` verbatim. A community bundle
   surfacing in an org gallery as `managed_approved_current`
   is non-conforming.
3. **Packaging chip verbatim.** The card cites
   `mirror_or_offline_packaging_posture` verbatim through
   the matching `badge.live_or_mirror` /
   `badge.mirror_only` / `badge.signed_offline_bundle` /
   `badge.live_origin_only` / `badge.offline_no_bundle`.
4. **Compatibility chip canonical.** The compatibility chip
   reads from `compatible_aureline_range`. A card that
   widens or narrows compatibility beyond the manifest is
   non-conforming.
5. **Persona / stack tags from canonical archetype rows.**
   Every persona / stack tag resolves through
   `archetype_row_id` or a typed launch-wedge persona id;
   free-text persona strings are non-conforming.
6. **No marketing bypass.** The card MUST NOT promote a
   "Try now", "Quick start", "One-click apply", or
   "Get going" affordance. The primary action is
   `bundle_card.review_bundle` (§3.3 rule 1).
7. **Retest pressure flows visibly.** When
   `retest_needed_posture ∈ {retest_recommended,
   retest_required, retest_blocked}` the card MUST render
   `badge.retest_pending` AND the freshness overlay AND a
   typed `bundle_card.view_certification_sheet` action when
   the bundle is certified-class. A card that hides retest
   pressure to keep the row "clean" is non-conforming.
8. **Successor recommendation visible.** When
   `successor_recommendation_class ∈
   {successor_recommended_same_class,
   successor_recommended_different_class,
   successor_recommended_first_party_native}`, the card
   MUST render `bundle_card.view_successor` and MUST NOT
   widen the current bundle's claim language to match the
   successor.
9. **Badge / chip resolution.** Every chip and badge cites
   the manifest field that backs it. A surface that mints a
   badge without a manifest source is non-conforming.

### 4.3 Badge downgrade rules

1. `badge.certified` is forbidden when
   `bundle_status_class != certified_current` OR
   `evidence_age_class != fresh_current` OR
   `retest_needed_posture != not_required`. In any of those
   cases the card downgrades the certified badge to
   `badge.retest_pending` and continues to render the
   compatibility, packaging, and freshness chips verbatim.
2. `badge.managed_approved` is forbidden when
   `bundle_status_class != managed_approved_current`.
3. `badge.community` is rendered as
   `badge.community` + chip text `reviewed` when
   `bundle_status_class = community_reviewed`, and as
   `badge.community` + chip text `unreviewed` when
   `bundle_status_class = community_unreviewed`. A surface
   that drops the `unreviewed` chip is non-conforming.
4. `badge.imported` always pairs with the manifest's
   `bundle_status_class` chip (`imported_pending_review`,
   `community_unreviewed`, `community_reviewed`,
   `retest_needed`, `deprecated_or_archived`,
   `status_unknown`). Imported never silently inherits
   certified or managed-approved language.
5. `badge.local_draft` MUST render alongside the local-only
   signer source (`local_user_trust_only`,
   `unsigned_user_review_required`,
   `repo_local_workspace_trust`); promoting a local-draft
   row to a community / managed / certified badge requires
   an explicit publish flow that produces a new manifest
   revision in a new class.
6. `badge.deprecated` and `badge.retest_pending` MAY co-exist
   when the manifest is in
   `lifecycle_state_class = soft_deprecated_with_successor`
   and `evidence_age_class = aging_within_window`. The card
   renders both chips and renders
   `bundle_card.view_successor` and
   `bundle_card.view_certification_sheet` together.
7. `badge.status_unknown` MUST render the
   `disabled_reason_code` chip explaining why the status
   could not be determined; a `status_unknown` row without
   a typed reason is non-conforming.

## 5. Bundle detail page record

Every bundle detail-page render (opened from
`bundle_card.review_bundle` on any family) emits exactly one
`start_center_bundle_detail_page_record`.

### 5.1 Required fields

- `record_kind = start_center_bundle_detail_page_record`.
- `detail_page_id` — opaque, stable per render.
- `bundle_id`, `bundle_revision` — ref to the underlying
  manifest.
- `bundle_card_ref` — opaque ref to the
  `start_center_bundle_card_record` the page was opened
  from.
- `bundle_card_surface_family` — the family the page was
  opened from. The detail page renders the same record
  shape regardless of family; the field is reserved so
  audit can attribute the render.
- `sections[]` — array of `bundle_detail_page_section_record`
  entries (§5.2). Every entry in §3.4 MUST be present as a
  section.
- `evidence_badge_ids[]` — re-exported verbatim from the
  card; the detail page MUST NOT render a badge the card
  does not.
- `review_actions[]` — typed actions covering
  `bundle_card.review_bundle` (active), and the typed
  removal / rollback / retest actions (§7, §8) when they
  apply.
- `mirror_or_offline_posture_chip` — re-exported verbatim
  from the manifest; required.
- `successor_recommendation` — block re-exported from the
  manifest's lifecycle (§3.11 / §8.1 of the workflow-bundle
  object model). A `no_successor` block is allowed but
  never elided.
- `certification_sheet_refs[]` — re-exported from the
  manifest; required when the badge set contains
  `badge.certified` or `badge.retest_pending` on a
  certified-class bundle.
- `removal_surface_kind`, `rollback_surface_kind` —
  re-exported from the manifest.
- `keyboard_reachable = true`.
- `minted_at` — monotonic timestamp.

### 5.2 Section record shape

Each `bundle_detail_page_section_record`:

- `section_id` (§3.4).
- `section_summary_ref` — opaque ref to a reviewable
  sentence describing what the section discloses.
- `component_kind` — one of `bundle_component_kind` for
  inventory sections; `null` for `section.identity_and_class`,
  `section.mirror_or_offline_posture`, `section.changelog`,
  `section.known_limits`, `section.lifecycle_and_successor`,
  `section.dependency_markers`, and `section.review_actions`.
- `component_refs[]` — opaque refs to the manifest's
  inventory entries (`component_id`s) the section renders.
  Empty array is allowed when the manifest emits an empty
  slot.
- `evidence_link_refs[]` — opaque evidence-link refs the
  section attaches.
- `policy_notice_refs[]` — in-place narrowing notices
  bound to this section.
- `disclosed_in_zone` — closed set: `detail_page.body`,
  `detail_page.review_actions_rail`,
  `detail_page.lifecycle_rail`,
  `detail_page.evidence_rail`. The zone bounds where the
  section may render but does not change the inventory it
  reads.
- `keyboard_reachable = true`.

### 5.3 Detail-page rules

1. **Eleven-component-kind parity.** The detail page MUST
   render one section per component kind frozen in
   [`/docs/workflow/workflow_bundle_object_model.md`](../workflow/workflow_bundle_object_model.md)
   §3.8 (`extension_set`, `profile_preset`, `surface_preset`,
   `task_recipe`, `launch_recipe`, `debug_recipe`,
   `template_or_scaffold_ref`, `docs_pack`, `tour_pack`,
   `glossary_pack`, `migration_mapping`,
   `certification_target`, `evidence_link`). A page that
   collapses presets into a single "settings" section or
   merges task / launch / debug into "run" is non-conforming.
2. **Mirror / offline section first-class.** A page that
   buries `section.mirror_or_offline_posture` under a
   "more info" toggle is non-conforming. The mirror /
   offline posture renders alongside identity.
3. **Changelog scoped to revision pair.** The
   `section.changelog` shows the diff between the bundle's
   `previous_revision_ref` and the current revision. A page
   that flattens multi-revision history into one running
   list without revision boundaries is non-conforming.
4. **Known-limits visible by default.** A page MUST render
   `section.known_limits` non-collapsed when the manifest
   emits any `evidence_link_kind = known_limit_note`.
5. **Successor and lifecycle visible.** The page renders
   `section.lifecycle_and_successor` regardless of
   lifecycle state; even an `active_supported` bundle
   renders the `no_successor` block with the freshness
   window.
6. **Removal and rollback in review-actions.** Removal and
   rollback affordances render in
   `section.review_actions` (or
   `detail_page.review_actions_rail`); they are typed
   verbs, not free-form copy (§8).
7. **Same-weight bypass.** The detail page MUST render the
   bypass affordance (`Open without this bundle`,
   `bypass.open_folder_without_starter`,
   `bypass.continue_without_starter`) at the same focus
   weight as `Review bundle`. The page composes with
   template-and-prebuild §4.2 rule 1.

## 6. Cross-surface badge contract

The badge vocabulary in §3.2 and the downgrade rules in §4.3
hold across every surface family in §3.1. Concretely:

1. **Stable across surfaces.** The same `bundle_id` plus
   `bundle_revision` renders the same badge set on
   `start_center_bundle_row`, `gallery_browse_bundles_row`,
   `workspace_switcher_bundle_row`, `update_detail_entry`,
   and `export_facing_bundle_summary`. A surface that mints
   a different badge for the same revision is non-conforming.
2. **Stable across post-install.** The badge set survives
   install. A bundle that renders `badge.certified` before
   install MUST continue to render `badge.certified` (or
   downgrade to `badge.retest_pending` per §4.3 rule 1)
   after install — never elide.
3. **Stable across export.** The export-facing summary
   carries the same badge set the Start Center carried.
   `export_facing_bundle_summary` family is the source of
   truth that `support_export`, `claim_manifest`,
   `certified_archetype_report`, and CLI / headless
   listings render against.
4. **Compatibility badges canonical.** A badge that asserts
   compatibility with an Aureline runtime version MUST
   read from `compatible_aureline_range`. A surface that
   asserts compatibility outside the manifest's range is
   non-conforming.
5. **Evidence badges canonical.** Every `badge.certified`,
   `badge.managed_approved`, `badge.community`,
   `badge.imported`, `badge.local_draft`,
   `badge.retest_pending`, `badge.deprecated`, and
   `badge.status_unknown` resolves through the manifest's
   `evidence_link_refs[]`. A surface that resolves a badge
   to a marketing label or a per-surface `tagline` field is
   non-conforming.

## 7. Review bundle action

Every non-takeover bundle card and detail page reserves a
typed `bundle_card.review_bundle` action.

### 7.1 Card-level rules

1. The card's primary action is `bundle_card.review_bundle`.
   On click it opens the
   `start_center_bundle_detail_page_record` referenced by
   `bundle_detail_page_ref`.
2. The action MUST be keyboard-reachable. A card that
   gates `Review bundle` behind a hover-only affordance is
   non-conforming.
3. The action MUST NOT mutate state. Opening the detail
   page is read-only; commit rides through the
   environment-starter summary contract on the detail
   page's review-actions rail.

### 7.2 Detail-page review-actions

The detail page renders a typed review-actions rail with at
most one of:

- `Review bundle` — opens the
  `environment_starter_summary_record` for install commit.
  Available when `bundle_status_class ∉
  {community_unreviewed, imported_pending_review,
  status_unknown, deprecated_or_archived}` and
  `disabled_reason_code` is null. Otherwise rendered
  visible-but-disabled with the typed reason.
- `Compare with installed` — when an installed revision
  exists; emits the diff between the installed revision
  and this one.
- `View certification sheet` — when
  `certification_sheet_refs[]` is non-empty.
- `View successor` — when `successor_recommendation_class
  ∈ {successor_recommended_*}`.
- `Retest now` — when
  `retest_needed_posture ∈ {retest_recommended,
  retest_required}` and `retest_blocked` is not active.
- `Open without this bundle` — typed bypass (composes with
  template-and-prebuild §4.2 rule 1); always visible.

## 8. Removal, rollback, and retest affordances

The detail page projects the manifest's
`removal_surface_kind` (workflow-bundle §3.14) and
`rollback_surface_kind` (§3.15) into typed, named affordances:

- `Remove` — visible when `removal_surface_kind ∈
  {bundle_detail_panel,
  project_doctor_recommended_action,
  support_export_recommendation, cli_or_headless_remove,
  org_policy_recall}`. Disabled (visible) on
  `not_applicable_local_draft`.
- `Rollback to previous revision` — visible when
  `rollback_surface_kind ∈
  {bundle_detail_panel,
  project_doctor_recommended_action,
  support_export_recommendation,
  cli_or_headless_rollback,
  org_policy_pinned_revision}`. Disabled (visible) on
  `not_applicable_no_prior_revision`.
- `Retest now` — visible when
  `retest_needed_posture ∈ {retest_recommended,
  retest_required}` and not blocked.

Rules (frozen):

1. Removal and rollback affordances cite the typed surface
   kinds verbatim. A `Remove` button without a backing
   `removal_surface_kind` is non-conforming.
2. A `Rollback` action MUST resolve to the
   `previous_revision_ref` on the manifest; rollback
   targets that the manifest does not name are
   non-conforming.
3. `Retest now` is a typed verb; surfaces that render
   "Refresh" or "Re-run checks" without binding to the
   manifest's retest scoreboard families are non-conforming.

## 9. Cross-surface invariants

1. **Inspectable before commit.** Every entry surface
   (`start_center_bundle_row`,
   `gallery_browse_bundles_row`,
   `workspace_switcher_bundle_row`,
   `update_detail_entry`,
   `export_facing_bundle_summary`) renders the bundle
   card's source class, status class, support class,
   signer continuity, packaging posture, freshness, and
   compatibility chips before any commit affordance. A
   surface that hides any of these axes behind a "more
   info" toggle is non-conforming.
2. **Side effects gated by detail page.** Install,
   removal, rollback, and retest commits ride through the
   detail page's review-actions rail (§7.2, §8). A card
   that commits any of these on first click is
   non-conforming.
3. **Successor recommendation flows across surfaces.**
   When the manifest emits a non-`no_successor` /
   non-`successor_unknown` recommendation, every entry
   surface (card + detail page + update entry + export
   summary) renders `bundle_card.view_successor` /
   `View successor`. The current bundle's claim language
   MUST NOT widen to match the successor.
4. **Certification expiry flows visibly.** When the
   manifest's `evidence_age_class` is
   `aging_within_window`, `stale_past_window`, or
   `age_unknown`, every entry surface renders the
   freshness overlay (§3.6) and downgrades the badge per
   §4.3 rule 1. Hiding expiry to keep a row "clean" is
   non-conforming.
5. **Retest-needed downgrades flow visibly.** When the
   manifest's `retest_needed_posture` is
   `retest_recommended`, `retest_required`, or
   `retest_blocked`, every entry surface renders
   `badge.retest_pending` AND narrows the support-class
   chip per workflow-bundle §3.13 rule 2 AND surfaces the
   retest-now affordance on the detail page. A surface
   that keeps a `Certified` chip while the manifest is in
   retest-required is non-conforming.
6. **Lifecycle events flow visibly.** When the manifest's
   `lifecycle_state_class` is one of
   `soft_deprecated_with_successor`,
   `hard_deprecated_remove_recommended`, `evidence_aged`,
   `removed_unavailable`, or `rollback_recommended`, the
   card renders `badge.deprecated` (and additionally
   `badge.retest_pending` per §4.3 rule 6 when applicable)
   and the detail page renders the lifecycle banner with
   the typed successor / removal / rollback affordances.
7. **One identity, many surfaces.** The same
   `bundle_id` + `bundle_revision` renders the same card
   record shape on every family in §3.1. Per §6 rule 1,
   the badge set is stable across surfaces.
8. **Keyboard reachability.** Every card action and every
   detail-page section MUST be keyboard-reachable. A
   surface that requires hover for `Review bundle`,
   `View certification sheet`, `Compare with installed`,
   `View successor`, `Remove`, `Rollback to previous
   revision`, or `Retest now` is non-conforming.
9. **No new vocabulary downstream.** A bundle row,
   card, detail page, gallery row, update entry, or
   export summary MUST NOT mint bundle, identity, source,
   status, packaging, signer, channel-relation, dependency-
   marker, component-kind, evidence-link, lifecycle,
   successor, evidence-age, retest-posture,
   removal-surface, or rollback-surface vocabulary that
   shadows the workflow-bundle object model. Every axis
   resolves through the manifest.

## 10. Worked examples

Each example has a companion fixture under
[`/fixtures/ux/start_center_bundle_cases/`](../../fixtures/ux/start_center_bundle_cases/).
Every fixture validates against
[`/schemas/ux/bundle_card.schema.json`](../../schemas/ux/bundle_card.schema.json)
or
[`/schemas/ux/bundle_detail_page.schema.json`](../../schemas/ux/bundle_detail_page.schema.json).

### 10.1 Certified launch bundle (TypeScript web app), Start Center row

A certified launch bundle on the Start Center secondary-entry
zone. `bundle_card_surface_family = start_center_bundle_row`,
`bundle_class = launch_bundle`,
`bundle_source_class = certified`,
`bundle_status_class = certified_current`,
`badge.certified` + `badge.live_or_mirror`,
`freshness_overlay = freshness.fresh_current`,
`successor_recommendation_class = no_successor`. Card
renders `bundle_card.review_bundle`,
`bundle_card.view_certification_sheet`, and
`bundle_card.export_summary_row`. See
[`bundle_card_certified_launch_typescript.json`](../../fixtures/ux/start_center_bundle_cases/bundle_card_certified_launch_typescript.json).

### 10.2 Org-approved bundle in mirror-only envelope

An org-approved bundle in a mirror-only envelope.
`bundle_card_surface_family = gallery_browse_bundles_row`,
`bundle_class = org_approved_bundle`,
`bundle_source_class = managed_approved`,
`bundle_status_class = managed_approved_current`,
`badge.managed_approved` + `badge.mirror_only`,
`local_offline_availability_class = available_via_mirror`. The
card cites `compatible_aureline_range` covering the org's
pinned channel. See
[`bundle_card_org_approved_mirror_only.json`](../../fixtures/ux/start_center_bundle_cases/bundle_card_org_approved_mirror_only.json).

### 10.3 Community-unreviewed bundle, never silently certified

A community bundle whose status is `community_unreviewed`.
`badge.community` + chip text `unreviewed`,
`bundle_card.review_bundle` is the only commit-adjacent
affordance and the detail page's `Review bundle` action is
disabled with `disabled_reason_code = signature_review_required`.
The card MUST NOT render `badge.certified` even though the
gallery is curated. See
[`bundle_card_community_unreviewed.json`](../../fixtures/ux/start_center_bundle_cases/bundle_card_community_unreviewed.json).

### 10.4 Imported-user bundle pending review

An imported-user bundle. `bundle_class = imported_user_bundle`,
`bundle_source_class = imported`,
`bundle_status_class = imported_pending_review`,
`badge.imported` + chip text `pending_review`,
`retest_needed_posture = retest_required`,
`badge.retest_pending` co-renders. Card cites the
imported source object and renders
`bundle_card.view_certification_sheet` only when the
ingress review completes. See
[`bundle_card_imported_pending_review.json`](../../fixtures/ux/start_center_bundle_cases/bundle_card_imported_pending_review.json).

### 10.5 Local-draft bundle on the workspace-switcher

A user-authored local-draft bundle re-advertised on the
workspace-switcher. `bundle_card_surface_family =
workspace_switcher_bundle_row`,
`bundle_class = local_draft_bundle`,
`bundle_source_class = local_draft`,
`bundle_status_class = local_draft`,
`badge.local_draft`. Removal surface is
`not_applicable_local_draft`; the card cannot promote into
managed / certified language. See
[`bundle_card_local_draft_workspace_switcher.json`](../../fixtures/ux/start_center_bundle_cases/bundle_card_local_draft_workspace_switcher.json).

### 10.6 Retest-pending certified bundle (evidence aging)

A certified bundle whose evidence age has crossed the first
warning threshold. `bundle_status_class =
certified_retest_pending`,
`evidence_age_class = aging_within_window`,
`retest_needed_posture = retest_recommended`. Badges:
`badge.retest_pending` + `badge.live_or_mirror`. The card
downgrades the certified badge per §4.3 rule 1; the detail
page surfaces `Retest now` and `View certification sheet`.
See
[`bundle_card_retest_pending_certified.json`](../../fixtures/ux/start_center_bundle_cases/bundle_card_retest_pending_certified.json).

### 10.7 Deprecated bundle with first-party successor

A deprecated launch bundle with a recommended first-party
successor. `bundle_card_surface_family = update_detail_entry`,
`bundle_status_class = deprecated_or_archived`,
`lifecycle_state_class = soft_deprecated_with_successor`,
`successor_recommendation_class =
successor_recommended_first_party_native`,
`badge.deprecated`. Card renders
`bundle_card.view_successor` and
`bundle_card.view_changelog`; the current card's claim
language never widens to the successor's certified
language. See
[`bundle_card_deprecated_with_successor.json`](../../fixtures/ux/start_center_bundle_cases/bundle_card_deprecated_with_successor.json).

### 10.8 Status-unknown bundle in offline registry

A bundle whose status could not be determined.
`bundle_status_class = status_unknown`,
`mirror_or_offline_packaging_posture =
packaging_posture_unknown`,
`evidence_age_class = age_unknown`,
`retest_needed_posture = retest_blocked` with
`disabled_reason_code = network_unreachable`. Badges:
`badge.status_unknown`. The card cites the
`disabled_reason_code` chip; never silently becomes
`badge.certified`. See
[`bundle_card_status_unknown_offline.json`](../../fixtures/ux/start_center_bundle_cases/bundle_card_status_unknown_offline.json).

### 10.9 Detail page for the certified launch bundle

The detail page opened from §10.1's card. Renders all
sixteen sections in §3.4 (eleven inventory sections plus
identity, mirror / offline posture, changelog, known
limits, lifecycle / successor, dependency markers, review
actions). Reviewer can read the badge set, the
`compatible_aureline_range`, the certification-sheet
ref, the changelog scoped to the revision pair, the
known-limits, and the typed `Review bundle` /
`View certification sheet` / `Open without this bundle`
actions. See
[`bundle_detail_page_certified_launch_typescript.json`](../../fixtures/ux/start_center_bundle_cases/bundle_detail_page_certified_launch_typescript.json).

A `manifest.json` index lives alongside the fixtures and
maps every fixture file to its surface family, the closed
sets it exercises, and the rules it validates.

## 11. Acceptance mapping

- **Reviewer can tell from the card / detail model whether a
  bundle is certified, experimental, community-supplied,
  mirrored, or locally drafted.** §3.2 (`bundle_evidence_badge_id`),
  §4 (`start_center_bundle_card_record`), §4.3 (badge
  downgrade rules), §5 (`start_center_bundle_detail_page_record`)
  freeze the closed badges and the manifest fields each
  badge cites. Fixtures §10.1–§10.5 exercise certified,
  managed-approved, community-unreviewed, imported, and
  local-draft rows; §10.8 exercises status-unknown.
- **Entry surfaces do not hide side effects, freshness, or
  support boundaries behind decorative copy.** §4.1
  (required fields), §4.2 (disclosure rules), §4.3 (badge
  downgrade rules), §6 (cross-surface badge contract), §9
  (cross-surface invariants), and §7 / §8 (typed Review
  bundle / removal / rollback / retest actions) remove the
  one-click commit path; install rides through the
  detail-page review-actions rail and the
  environment-starter summary contract.
- **Evidence and compatibility badges resolve to canonical
  bundle or archetype objects.** §3.2 rule 1 (badges resolve
  to manifest fields), §3.2 rule 2 (no marketing-only
  badges), §4.2 rule 4 (compatibility canonical), §4.2 rule
  5 (persona / stack tags from canonical archetype rows), §6
  rules 4 and 5 (compatibility and evidence canonical). The
  manifest at
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json)
  is the source of truth for every badge.
- **Badge and detail-page contracts are sufficient to drive
  a later certification sheet and lifecycle audit, not just
  the happy path.** §3.4 (sixteen detail-page sections), §5.3
  (eleven-component-kind parity), §7.2 (review-actions rail),
  §8 (removal / rollback / retest), §9 (cross-surface
  invariants including post-install and export-facing
  parity). Fixtures §10.6, §10.7, and §10.8 exercise
  retest-pending, deprecated-with-successor, and
  status-unknown rows; §10.9 exercises the full detail-page
  parity.
- **Schema coverage.** The schemas at
  [`/schemas/ux/bundle_card.schema.json`](../../schemas/ux/bundle_card.schema.json)
  and
  [`/schemas/ux/bundle_detail_page.schema.json`](../../schemas/ux/bundle_detail_page.schema.json)
  validate the worked-example fixtures under
  [`/fixtures/ux/start_center_bundle_cases/`](../../fixtures/ux/start_center_bundle_cases/).

## 12. Changing this contract

- **Additive-minor** changes (new
  `bundle_card_surface_family`, new
  `bundle_evidence_badge_id`, new `bundle_card_action_id`,
  new `bundle_detail_page_section_id`, new
  `bundle_card_zone`, new `bundle_card_freshness_overlay`,
  new `local_offline_availability_class`) land here, in
  the companion schemas, and in at least one fixture under
  [`/fixtures/ux/start_center_bundle_cases/`](../../fixtures/ux/start_center_bundle_cases/)
  in the same change. Adding a value bumps the schema
  version.
- **Repurposing** an existing surface family, badge id,
  action id, section id, zone, or overlay is breaking and
  opens a decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (workflow-bundle object
  model, start-center contract, template-and-prebuild
  contract) happen at source and this contract re-exports
  by reference; it MUST NOT shadow the change.
- The PRD / TAD / TDD / UI-UX spec wins on any disagreement
  with the quotations in §15; this contract and its schemas
  plus fixtures update in the same change.

## 13. Surface invariants summary

| Invariant | Source | Enforced on |
|---|---|---|
| Source class verbatim | §4.2 rule 1, §6 rule 1 | Card, detail page, gallery, update entry, export summary |
| Status class verbatim | §4.2 rule 2 | Card, detail page, gallery, update entry, export summary |
| Packaging posture verbatim | §4.2 rule 3 | Card, detail page, gallery, update entry, export summary |
| Compatibility canonical | §4.2 rule 4, §6 rule 4 | Card, detail page |
| Persona / stack tags from canonical archetype rows | §4.2 rule 5 | Card, detail page |
| Review bundle stays primary | §3.3 rule 1, §7 | Card, detail page |
| No silent install authority | §3.3 rule 3, §9 rule 2 | Card, detail page |
| Retest pressure visible | §4.2 rule 7, §9 rule 5 | Card, detail page |
| Successor visible | §4.2 rule 8, §9 rule 3 | Card, detail page, update entry |
| Lifecycle events visible | §9 rule 6 | Card, detail page |
| Eleven-component-kind parity | §5.3 rule 1 | Detail page |
| Mirror / offline section first-class | §5.3 rule 2 | Detail page |
| Same-weight bypass | §5.3 rule 7 | Detail page |
| Stable badge set across surfaces | §6 rule 1 | All five surface families |
| Stable badge set post-install | §6 rule 2 | All five surface families |
| Stable badge set across export | §6 rule 3 | Export-facing summary |
| Keyboard reachability | §4.1, §5.1, §9 rule 8 | Card, detail page |
| No new vocabulary downstream | §9 rule 9 | All five surface families |

## 14. Linked artifacts

- Workflow-bundle object model, component inventory, and
  source-class contract (source of truth for every bundle
  manifest field this contract re-exports):
  [`/docs/workflow/workflow_bundle_object_model.md`](../workflow/workflow_bundle_object_model.md)
  and
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).
- Start Center, workspace-switcher, open-flow, restore-card,
  and recent-work disclosure contract (source of truth for
  zones, account-opt-in posture, freshness / absence,
  privacy reduction, primary actions, and disclosure
  banners):
  [`/docs/ux/start_center_contract.md`](./start_center_contract.md)
  and
  [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json).
- Template gallery, prebuild / warm-start, resume-live, and
  open-without-starter disclosure contract (source of truth
  for support class, availability narrowing, policy notice,
  bypass path, and same-weight bypass):
  [`/docs/ux/template_and_prebuild_contract.md`](./template_and_prebuild_contract.md).
- Workflow-bundle id register (machine-readable upstream for
  `bundle_id`, `archetype_row_id`, `archetype_revision`,
  `scoreboard_family_id`, `public_proof_packet_shape`,
  `cutline_ref`):
  [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml).
- Bundle card schema (machine-readable companion):
  [`/schemas/ux/bundle_card.schema.json`](../../schemas/ux/bundle_card.schema.json).
- Bundle detail-page schema (machine-readable companion):
  [`/schemas/ux/bundle_detail_page.schema.json`](../../schemas/ux/bundle_detail_page.schema.json).
- Worked-example fixtures:
  [`/fixtures/ux/start_center_bundle_cases/`](../../fixtures/ux/start_center_bundle_cases/).

## 15. Source anchors

- `.t2/docs/Aureline_PRD.md:284` — first-run onboarding is a
  launch risk, not polish.
- `.t2/docs/Aureline_PRD.md:2328` — intelligent project
  scaffolding and generation: starter templates and
  agentic setup for new services / apps / modules using
  team standards.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:762` — Start
  Center primary actions; bundles render as inspectable
  rows that route into review, never one-click apply.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:802` — §6.9
  templates, starters, and prebuilds: source class,
  support class, runtime / toolchain, freshness, setup
  actions, always-available bypass path, side-effect
  envelope; bundles inherit these axes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:6346` — §17.7
  scaffolding, generation, and template health: signal
  classes (`live`, `cached`, `policy_evaluated`,
  `not_checked`); bundle freshness composes with this.
- `.t2/docs/Aureline_Milestones_Document.md:1023` — Start
  Center keeps `Open`, `Clone`, `Import`, `Restore`, and
  `Recent work` distinct with a no-account local path;
  bundle rows render same-weight in the secondary-entry
  zone.
- `.t2/docs/Aureline_Milestones_Document.md:3787` —
  environment-capsule schema draft, workspace-template
  seed, and prebuild-metadata baseline; bundle cards
  carry the inspection axes those artifacts expose.
