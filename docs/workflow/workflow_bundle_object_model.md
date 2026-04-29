# Workflow-bundle object model, component inventory, and source-class contract

This document freezes the cross-surface object model every
**workflow bundle** is reviewed, diffed, exported, installed,
removed, projected onto Start Center, projected onto
diagnostics, projected onto claim surfaces, and projected onto
later CLI / headless paths against. The goal is that bundles
become **versioned, reviewable artifacts** — not hand-wavy
"starter experiences" — so M1 / M2 implementation does not
invent late bundle semantics.

The companion machine-readable schema lives at:

- [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json)

The companion fixtures live under:

- [`/fixtures/workflow/bundles/`](../../fixtures/workflow/bundles/)

This contract is normative for the bundle object model and the
source / status taxonomy. Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, or milestone document anchors quoted in §13,
those sources win and this document plus its companion schema
and fixtures update in the same change. Where a downstream
gallery, picker, recommender, diagnostics row, claim row, or
removal flow mints a parallel bundle vocabulary, this contract
wins and the surface is non-conforming.

This contract mints **no** new template, prebuild, archetype,
language-bundle, or claim vocabulary. It re-exports — by
reference — the closed sets frozen by the upstream contracts
listed in §3.0.

## Who reads this contract

- **Bundle authors** (first-party lanes, team-managed admins,
  community contributors, design partners, importers, local
  drafters) writing or curating bundle manifests. Every field
  on the manifest resolves through a closed set defined here.
- **Reviewers** diffing bundle manifests across versions,
  channels, and signers. They expect to read source class,
  status class, support class, signer continuity, evidence-link
  refs, lifecycle posture, and dependency markers without
  external context.
- **Start Center, workspace-switcher, recommender, and Add /
  Browse-bundles surface authors** projecting bundle rows onto
  the disclosure contracts in
  [`/docs/ux/start_center_contract.md`](../ux/start_center_contract.md)
  and
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- **Diagnostics, project-doctor, support-export, and
  removal / rollback authors** rendering bundle truth after
  install — what was changed, what is stale, what should be
  removed, what should be retested.
- **Claim, certified-archetype, compatibility, migration,
  benchmark, and docs authors** who need to cite a bundle by
  stable id and revision rather than by free-text label.
- **CLI / headless / API authors** later in M1 / M2 who must
  read the same bundle truth a UI surface renders. The boundary
  is one record kind across surfaces.

## 1. Scope

- Freeze one `workflow_bundle_manifest_record` that every
  reviewable bundle artifact emits regardless of source class,
  status class, channel, or surface family.
- Freeze the `bundle_component_inventory` block inside that
  record covering the **eleven** component kinds (§4) every
  bundle declares: extension sets, profile presets, surface
  presets, task recipes, launch recipes, debug recipes,
  template / scaffold references, docs / tour / glossary packs,
  migration mappings, certification targets, and evidence-link
  packs.
- Freeze the bundle **identity** block (§5) — stable bundle id,
  revision, source class, signer / source, compatible Aureline
  range, archetype binding, support class, channel relation,
  dependency markers, evidence-link refs.
- Freeze the bundle **source / status** taxonomy (§6) covering
  Certified, Managed approved, Community, Imported, and Local
  draft bundles, plus the mirror / offline packaging posture
  and the evidence-link fields that back any non-Imported
  status above `community_unreviewed`.
- Freeze the bundle **class linkage** rows (§7) reserved for
  `Launch`, `Imported-user`, `Org-approved`, `Design-partner`,
  and `Local-draft` so later Start Center, diagnostics, and
  claim surfaces project the same taxonomy without minting
  parallel labels.
- Freeze the bundle **lifecycle** block (§8) — successor
  recommendation, certification-sheet refs, evidence age,
  retest-needed posture, removal / rollback surface linkage.
- Freeze the cross-cutting **surface invariants** (§9) bundle
  truth must satisfy on every projection (Start Center, docs,
  diagnostics, exports, CLI / headless).
- Name the closed vocabularies (§3) that bound bundle class,
  source class, status class, packaging posture, signer-source
  class, channel-relation class, dependency-marker kind,
  component kind, lifecycle-state class, evidence-age class,
  retest-needed posture, successor-recommendation class,
  evidence-link kind, certification-target kind, claim-surface
  kind, removal-surface kind, and rollback-surface kind.

## 2. Out of scope

- The bundle **registry service**. Discovery, transport,
  channel rotation, key rotation, and signature mechanics live
  with the registry and release-artifact contracts; this
  contract pins only the manifest shape and the closed sets the
  registry resolves against.
- The bundle **execution engine**. Materialisation, extension
  activation, task / launch / debug execution, and template /
  scaffold expansion live with the execution and scaffold
  contracts; this contract reserves the disclosure and
  inventory slots those engines read.
- The bundle **template backend**. Template authoring tools,
  scaffold-hook policy enforcement, and the per-template
  registry record are owned by
  [`/docs/templates/template_registry_and_scaffold_contract.md`](../templates/template_registry_and_scaffold_contract.md);
  this contract only re-exports the relevant ids and reserves
  manifest slots that point at template registry rows.
- Final user-facing **copy / microcopy**. The shell
  interaction-safety contract and the UX style guide own the
  exact strings; this contract pins the closed sets the copy
  resolves against.
- **Telemetry wire format**. Onboarding, install, and removal
  measurement is owned by the support-export and onboarding-
  measurement plans; this contract only tags records with the
  stable bundle id and revision those plans cite.

## 3. Frozen vocabulary (re-exported) and new closed sets

### 3.0 Re-exported vocabulary

This contract re-exports — by reference, never by redefinition
— the following closed sets:

- `template_source_class`, `support_class`,
  `runtime_and_toolchain_scope`, `template_lifecycle_class`,
  `declared_freshness_class`, `starter_setup_cost_class`,
  `availability_narrowing_class`, `bypass_path_id`,
  `template_health_signal_class`,
  `template_health_check_class`, `generation_preflight_axis`,
  `post_create_handoff_axis`, `policy_notice_class` —
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
  §3.1–§3.14.
- `signer_continuity_class`, `acquisition_posture`,
  `transport_class`, `mirror_freshness`,
  `policy_narrowing_class` — source-acquisition seed §1–§2.
- `template_archetype_class`,
  `template_certification_class`,
  `template_trust_source_class`, `signature_class`,
  `template_health_cadence_class`,
  `template_registry_health_state_class`,
  `template_registry_origin_class` —
  [`/schemas/templates/template_registry_entry.schema.json`](../../schemas/templates/template_registry_entry.schema.json).
- `entry_verb`, `target_kind`, `resulting_mode`,
  `admission_class`, `recovery_class`,
  `next_step_decision_hook` — entry-restore object model §1.
- `start_center_zone`, `primary_action_id`,
  `disclosure_class`, `privacy_reduction_mode`,
  `startup_state` — start-center contract §3 plus
  entry-restore truth audit §6.
- `archetype_row_id` (with `archetype_revision`) and the
  `workflow_bundle_id` register —
  [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml).
- `support_claim_class`, `detection_outcome` —
  [`/schemas/workspace/archetype_detection.schema.json`](../../schemas/workspace/archetype_detection.schema.json).
- `scoreboard_family_id`, `public_proof_packet_shape`,
  `proof_class`, `downgrade_trigger` —
  workflow-bundle id register linked above.
- `trust_state` — ADR-0001.

This contract introduces **fifteen** small vocabularies scoped
to bundle manifest, component inventory, and source / status
truth. Each is closed; adding a value is additive-minor and
bumps the schema version, repurposing is breaking and opens a
decision row.

### 3.1 `bundle_class`

Which **product row class** the bundle resolves into across
Start Center, diagnostics, claim, and CLI surfaces. The set is
closed:

- `launch_bundle` — a bundle bound to a P0 persona's launch
  wedge. Resolves through the Launch row in the workflow-bundle
  id register; carries replacement-grade semantics only when
  scoreboard families and the launch-wedge cutline admit them.
- `imported_user_bundle` — a bundle composed from an imported
  user environment (settings, profile, extension set,
  task / launch / debug recipes). The bundle is reviewable and
  diffable; it never silently equals `launch_bundle` by
  position.
- `org_approved_bundle` — a bundle authored or curated by the
  user's organisation, fleet, or admin policy. Resolves through
  the team-managed signer; carries the team's support class.
- `design_partner_bundle` — a bundle authored by or with a
  named design partner under a scoped pilot, beta, or field-
  readiness agreement. Carries an explicit pilot scope;
  promotion to `launch_bundle` requires the launch-wedge
  contract and a decision row.
- `local_draft_bundle` — a bundle authored locally and not yet
  promoted beyond the device. Carries `local_only` source and
  `unsigned_user_review_required` trust; never claims authority
  it has not been granted.

Rules (frozen):

1. Every `workflow_bundle_manifest_record` names exactly one
   `bundle_class`. A surface that emits a sixth class
   (`personal_pack`, `agent_recipe`, etc.) is non-conforming.
2. `launch_bundle` rows MUST cite at least one
   `workflow_bundle_id` from
   [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml)
   and at least one `cutline_ref`. A bundle that claims
   `launch_bundle` without the register row is non-conforming.
3. `design_partner_bundle` rows MUST carry a typed
   `design_partner_pilot_scope` ref and an expiry-or-review
   date class (§3.13). Open-ended design-partner pilots are
   non-conforming.
4. `local_draft_bundle` rows MUST resolve through
   `local_user_trust_only` or
   `unsigned_user_review_required`. Promotion to any other
   class rides through an explicit publish flow that opens a
   new manifest revision in a new class.
5. `imported_user_bundle` rows MUST cite the imported source
   (`source_object_ref` from a migration / import packet) and
   MUST NOT silently inherit a higher source class than
   `community` until promoted by a reviewer.

### 3.2 `bundle_source_class`

Who authored the bundle and under what trust posture. The set
is closed:

- `certified` — authored and signed by Aureline; passes the
  workflow-bundle / archetype proof and benchmark-public-proof
  scoreboard families.
- `managed_approved` — authored or curated by the user's
  organisation or fleet policy; signed by the team-managed
  signer.
- `community` — authored by an external contributor and
  distributed through a governed community channel.
- `imported` — composed at install time from an imported user
  environment; signature is not asserted unless the importer
  produced a signed-offline-bundle.
- `local_draft` — authored locally; never promoted beyond the
  current device until an explicit publish flow runs.

Rules (frozen):

1. Every manifest names exactly one `bundle_source_class`.
2. `bundle_source_class` is **not** a synonym for
   `bundle_class`: a `launch_bundle` may be `certified`; an
   `org_approved_bundle` is typically `managed_approved`; an
   `imported_user_bundle` is `imported`; a
   `design_partner_bundle` may be `managed_approved` or
   `community` depending on signer; a `local_draft_bundle` is
   `local_draft`. The legal pairings are listed in §7.
3. `certified` and `managed_approved` rows MUST carry a
   `signer_continuity_class` and a `signature_class`.
4. `community` and `imported` rows MUST carry a
   `signer_continuity_class`; missing the class is non-
   conforming.

### 3.3 `bundle_status_class`

Operational status of the bundle at the manifest's
`minted_at`. The set is closed:

- `certified_current` — passes every required scoreboard
  family on the bound launch-wedge cutline within the
  scoreboard's freshness cadence.
- `certified_retest_pending` — was certified, but at least one
  scoreboard row is past its cadence; retest is queued.
- `managed_approved_current` — currently approved by the
  organisation's policy bundle; mirrors
  `managed_approved` source.
- `community_reviewed` — community bundle that has passed an
  Aureline-or-org review checkpoint within the last review
  window.
- `community_unreviewed` — community bundle awaiting review;
  cannot project replacement-grade or daily-driver claim
  language.
- `imported_pending_review` — imported bundle whose ingress
  review has not completed; cannot enable mutating side
  effects until reviewed.
- `local_draft` — unpublished local authoring; never carries
  install-or-recommend authority.
- `deprecated_or_archived` — explicitly retired; renders only
  through removal / rollback surfaces.
- `retest_needed` — evidence age tripped; the bundle remains
  installable but renders a retest banner across surfaces.
- `status_unknown` — status could not be determined (offline,
  expired registry, lost signer continuity); rendered with a
  typed review hook.

Rules (frozen):

1. Every manifest names exactly one `bundle_status_class`.
2. `certified_current` requires at least one
   `evidence_link_ref` per required scoreboard family the
   bound launch-wedge cutline cites; missing any link
   downgrades the row to `certified_retest_pending`.
3. `community_unreviewed` and `imported_pending_review` MUST
   render the
   [`policy_notice_class`](../ux/template_and_prebuild_contract.md)
   matching their narrowing on every projection.
4. `deprecated_or_archived` rows MUST carry a
   `successor_recommendation` (§8) — `no_successor` is allowed
   but never silent.
5. `status_unknown` MUST cite the
   `disabled_reason_code` (`network_unreachable`,
   `signature_review_required`, `mirror_only_cached_subset`,
   `policy_narrowed_fleet`, `policy_narrowed_admin`)
   that explains why the status could not be determined.

### 3.4 `mirror_or_offline_packaging_posture`

How the bundle is packaged for delivery and offline use. The
set is closed:

- `live_origin_only` — the bundle is delivered live from the
  signer's origin only; no mirror or offline bundle is
  authorised.
- `live_or_mirror` — the bundle may be delivered live or
  through a mirror; mirror freshness is declared per
  acquisition.
- `mirror_only` — the bundle is delivered through a mirror
  (admin-required or air-gap); live origin is not contacted.
- `signed_offline_bundle` — the bundle is delivered as a
  signed offline artifact; freshness is pinned at packaging.
- `offline_no_bundle` — the bundle is unavailable in this
  posture; the manifest still renders the row with the
  narrowing class.
- `packaging_posture_unknown` — the posture could not be
  determined; rendered with a typed review hook.

Rules (frozen):

1. Every manifest names exactly one
   `mirror_or_offline_packaging_posture`.
2. `mirror_only` and `signed_offline_bundle` MUST cite a
   typed `mirror_freshness_ref` and a typed
   `availability_narrowing_class` from the template
   contract §3.8 when projected onto a gallery.
3. `offline_no_bundle` MUST render a
   `mirror_or_airgap_policy_notice` on every projection.
4. `packaging_posture_unknown` MUST NOT be promoted to
   `live_or_mirror` silently; promotion requires a registry
   refresh and a manifest revision.

### 3.5 `bundle_signer_source_class`

Who signed the bundle. The set is closed and re-exports the
template-trust-source vocabulary into bundle scope:

- `core_signing_root`
- `core_signing_root_via_mirror`
- `org_policy_signing_root`
- `org_mirror_signing_root`
- `community_channel_signature`
- `signed_offline_bundle_root`
- `repo_local_workspace_trust`
- `local_user_trust_only`
- `unsigned_user_review_required`
- `trust_source_unknown_review_required`

Rules (frozen):

1. Every manifest names exactly one
   `bundle_signer_source_class`.
2. `bundle_class = launch_bundle` requires
   `core_signing_root` or `core_signing_root_via_mirror`.
3. `bundle_class = org_approved_bundle` requires
   `org_policy_signing_root` or `org_mirror_signing_root`.
4. `bundle_class = local_draft_bundle` requires
   `local_user_trust_only`,
   `unsigned_user_review_required`, or
   `repo_local_workspace_trust`.

### 3.6 `bundle_channel_relation_class`

How this bundle revision relates to its release channel. The
set is closed:

- `same_channel_revision` — current revision on the same
  channel as the bundle's previous accepted revision.
- `cross_channel_promotion` — the bundle is being promoted
  from a less stable to a more stable channel (for example
  beta → stable).
- `cross_channel_demotion` — the bundle is being demoted
  toward a less stable channel (for example stable → beta or
  legacy).
- `branch_or_fork_revision` — the bundle is published on a
  parallel branch or fork; never silently overwrites the main
  channel.
- `not_applicable_local_draft` — local-draft bundles do not
  resolve a channel relation.

Rules (frozen):

1. Every manifest names exactly one
   `bundle_channel_relation_class`.
2. `cross_channel_promotion` requires a
   `previous_revision_ref` and a typed
   `evidence_link_refs[]` set covering the scoreboard
   families the new channel demands.
3. `cross_channel_demotion` requires a typed
   `lifecycle_state_class` (§3.10) of
   `soft_deprecated_with_successor` or
   `hard_deprecated_remove_recommended` and a
   `successor_recommendation` (§3.11).

### 3.7 `bundle_dependency_marker_kind`

How a bundle declares dependencies, conflicts, and
replacement on other bundles, extensions, runtimes, templates,
or archetypes. The set is closed:

- `requires_runtime_range` — the bundle requires a
  `compatible_aureline_range` (§5).
- `requires_extension_set` — the bundle requires the named
  extension set or a compatible bridge.
- `requires_template_ref` — the bundle requires the named
  template registry entry to be installable.
- `requires_archetype_binding` — the bundle requires the
  named archetype row to be the workspace archetype.
- `requires_companion_bundle` — the bundle works only when
  another named bundle is present.
- `optional_companion_bundle` — the bundle integrates with
  another named bundle when present.
- `replaces_bundle` — the bundle replaces another named
  bundle (typical successor relation).
- `conflicts_with_bundle` — the bundle conflicts with another
  named bundle and cannot be co-installed.
- `recommends_against_bundle` — the bundle recommends the
  user not install another bundle (paired with a typed
  reason).

Rules (frozen):

1. Every dependency entry in the manifest names exactly one
   `bundle_dependency_marker_kind`.
2. `replaces_bundle`, `conflicts_with_bundle`, and
   `recommends_against_bundle` MUST cite the target bundle's
   stable id and revision; free-text strings are non-
   conforming.
3. `requires_archetype_binding` MUST cite the
   `archetype_row_id` and `archetype_revision` from
   [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml).

### 3.8 `bundle_component_kind`

The eleven component kinds a bundle declares in its
inventory. The set is closed:

- `extension_set` — declared extension manifests the bundle
  installs, recommends, or activates.
- `profile_preset` — appearance / editor / language / shell
  profile presets the bundle ships.
- `surface_preset` — Start Center, workspace-switcher, panel
  layout, status-bar, or quick-open presets.
- `task_recipe` — declared task definitions
  (`task_recipe_ref`).
- `launch_recipe` — declared launch / run definitions
  (`launch_recipe_ref`).
- `debug_recipe` — declared debug session definitions
  (`debug_recipe_ref`).
- `template_or_scaffold_ref` — typed reference to one
  template registry entry the bundle ships or recommends.
- `docs_pack` — docs pages, command help refs, and
  feature-readiness anchors the bundle adds or layers.
- `tour_pack` — guided tour or onboarding scenes the bundle
  contributes.
- `glossary_pack` — glossary, vocabulary, or known-limit
  notes the bundle contributes.
- `migration_mapping` — typed mapping rows from imported
  source objects to bundle components, used by the migration
  contract.
- `certification_target` — typed reference to the
  workflow-bundle / archetype proof scoreboard rows the
  bundle binds to.
- `evidence_link` — typed reference to docs, benchmark,
  compatibility, migration, support, or known-limit
  artifacts the bundle leans on.

Rules (frozen):

1. Every component entry in the inventory names exactly one
   `bundle_component_kind` from this set. Free-form
   `component_type` strings are non-conforming.
2. A bundle that emits an inventory without `extension_set`,
   `task_recipe`, `launch_recipe`, `debug_recipe`, or
   `template_or_scaffold_ref` is allowed (a bundle may be
   purely docs or glossary), but the inventory MUST be
   present and the empty kinds MUST render as empty arrays
   (not as missing keys) so reviewers see the absence.
3. `certification_target` and `evidence_link` are required
   on `certified_current`, `certified_retest_pending`,
   `managed_approved_current`, and `community_reviewed`
   manifests; missing them downgrades the status row.

### 3.9 `evidence_link_kind`

Closed set of evidence-link kinds the manifest carries. Every
non-`imported` and non-`local_draft` bundle ships at least one
evidence link per required scoreboard family it cites.

- `certification_sheet`
- `compatibility_report`
- `benchmark_packet`
- `migration_note`
- `docs_version_match`
- `support_export_reference`
- `known_limit_note`
- `scoreboard_packet`
- `claim_manifest_row`
- `decision_row_ref`

Rules (frozen):

1. Every `evidence_link` entry names exactly one
   `evidence_link_kind`.
2. A `certified_current` manifest MUST carry at least one
   entry of each of `certification_sheet`,
   `scoreboard_packet`, and `docs_version_match`.
3. `decision_row_ref` is required when a manifest claims
   cross-channel promotion, cross-channel demotion, or any
   `replaces_bundle` / `conflicts_with_bundle` /
   `recommends_against_bundle` dependency marker.

### 3.10 `lifecycle_state_class`

Operational lifecycle posture of the bundle. The set is
closed:

- `active_supported`
- `soft_deprecated_with_successor`
- `hard_deprecated_remove_recommended`
- `retest_needed`
- `evidence_aged`
- `removed_unavailable`
- `rollback_recommended`
- `lifecycle_unknown`

Rules (frozen):

1. Every manifest names exactly one `lifecycle_state_class`.
2. `soft_deprecated_with_successor` and
   `hard_deprecated_remove_recommended` MUST cite a
   `successor_recommendation` (§3.11) other than
   `successor_unknown`. `no_successor` is permitted but never
   silent.
3. `removed_unavailable` and `rollback_recommended` MUST
   resolve a `removal_surface_kind` (§3.14) and a
   `rollback_surface_kind` (§3.15).
4. `evidence_aged` MUST set the manifest's
   `bundle_status_class` to `retest_needed` or
   `certified_retest_pending`; setting `evidence_aged` while
   keeping `certified_current` is non-conforming.

### 3.11 `successor_recommendation_class`

What the manifest recommends when the bundle is deprecated,
demoted, or removed. The set is closed:

- `no_successor`
- `successor_recommended_same_class`
- `successor_recommended_different_class`
- `successor_recommended_first_party_native`
- `successor_unknown`

Rules (frozen):

1. Every manifest emits exactly one
   `successor_recommendation` block; absent is non-
   conforming.
2. `successor_recommended_*` blocks MUST cite a stable
   `successor_bundle_id` and `successor_bundle_revision`,
   and MUST NOT silently widen claim authority (a
   community successor never inherits certified wording).

### 3.12 `evidence_age_class`

How fresh the evidence backing the bundle's claims is. The
set is closed:

- `fresh_current` — within the bound scoreboard family's
  freshness cadence.
- `aging_within_window` — past the cadence's first warning
  threshold but inside the absolute window.
- `stale_past_window` — past the absolute window; downstream
  claims must be downgraded.
- `age_unknown` — age could not be determined; rendered with
  a typed review hook.

Rules (frozen):

1. Every manifest names exactly one
   `evidence_age_class`.
2. `stale_past_window` MUST set
   `bundle_status_class` to `retest_needed` or
   `certified_retest_pending`, and MUST set
   `lifecycle_state_class` to `retest_needed` or
   `evidence_aged`.

### 3.13 `retest_needed_posture`

How the manifest projects retest pressure onto surfaces. The
set is closed:

- `not_required`
- `retest_recommended`
- `retest_required`
- `retest_blocked`

Rules (frozen):

1. Every manifest names exactly one
   `retest_needed_posture`.
2. `retest_required` MUST narrow the `support_class`
   projection on Start Center, diagnostics, and claim
   surfaces by one rung (for example `certified` →
   `supported`).
3. `retest_blocked` MUST cite a typed
   `disabled_reason_code` (`signature_review_required`,
   `policy_narrowed_fleet`, `network_unreachable`,
   `mirror_only_cached_subset`).

### 3.14 `removal_surface_kind`

Where a removal-or-disable affordance for this bundle
projects. The set is closed:

- `start_center_browse_bundles`
- `bundle_detail_panel`
- `project_doctor_recommended_action`
- `support_export_recommendation`
- `cli_or_headless_remove`
- `org_policy_recall`
- `not_applicable_local_draft`

Rules (frozen):

1. Every manifest names exactly one
   `removal_surface_kind`.
2. `not_applicable_local_draft` is the only conforming value
   for `local_draft_bundle`.

### 3.15 `rollback_surface_kind`

Where a rollback-to-previous-revision affordance projects.
The set is closed:

- `bundle_detail_panel`
- `project_doctor_recommended_action`
- `support_export_recommendation`
- `cli_or_headless_rollback`
- `org_policy_pinned_revision`
- `not_applicable_no_prior_revision`

Rules (frozen):

1. Every manifest names exactly one
   `rollback_surface_kind`.
2. `not_applicable_no_prior_revision` is the only conforming
   value when `previous_revision_ref` is null.

## 4. Component inventory

Every manifest emits exactly one
`bundle_component_inventory` block.

### 4.1 Required slots

The inventory is a record with one array per component kind
(§3.8). Each slot is required as a key; the value is allowed
to be the empty array, but the key MUST NOT be missing so
reviewers see the absence:

- `extension_sets[]`
- `profile_presets[]`
- `surface_presets[]`
- `task_recipes[]`
- `launch_recipes[]`
- `debug_recipes[]`
- `template_or_scaffold_refs[]`
- `docs_packs[]`
- `tour_packs[]`
- `glossary_packs[]`
- `migration_mappings[]`
- `certification_targets[]`
- `evidence_links[]`

### 4.2 Per-entry shape

Each entry inside any slot is a `bundle_component_record`
with:

- `component_id` (opaque, stable across revisions).
- `component_kind` (§3.8) — MUST match the parent slot.
- `component_revision` — opaque revision id; integer or
  semver string.
- `component_summary` — ≤ 256 graphemes,
  redaction-aware.
- `component_source_class` — `bundle_source_class` (§3.2);
  defaults to the manifest's source class but may narrow to
  a stricter class (a `certified` bundle MAY include a
  `community` glossary pack with the entry's source class
  set to `community`).
- `component_signer_source_class` (§3.5) when the entry
  ships its own signature.
- `component_evidence_link_refs[]` — required on
  `certification_target`, `migration_mapping`,
  `template_or_scaffold_ref`, and any entry whose
  `component_source_class` differs from the manifest's
  source class.
- `component_disclosure_required = true|false` — whether
  the entry MUST render in the bundle detail panel.
  Defaults to `true`; `false` is reserved for redaction-only
  entries (for example, glossary packs with redacted
  identifiers).
- `keyboard_reachable = true` — non-conforming if false.

### 4.3 Inventory rules (frozen)

1. The inventory MUST NOT contain entries whose
   `component_kind` does not match the parent slot.
2. An entry whose `component_source_class` is `community`
   or `imported` MUST carry a `signer_continuity_class`
   even when the manifest's overall source class is
   stricter.
3. `template_or_scaffold_ref` entries MUST resolve to a
   `template_registry_entry_record` id; free-form template
   strings are non-conforming.
4. `migration_mapping` entries MUST cite a typed
   `source_object_class` and a typed
   `target_component_kind` from §3.8.
5. `certification_target` entries MUST cite a
   `scoreboard_family_id` and a `public_proof_packet_shape`
   from
   [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml)
   and a `workflow_bundle_id` of the bundle itself.
6. The inventory's `extension_sets[]` MUST never silently
   imply install authority. Extension activation rides
   through the
   [environment-starter summary contract](../ux/template_and_prebuild_contract.md#6)
   and the trust-prompt contract; the inventory only
   enumerates what would be installed if the user commits.

## 5. Bundle identity

Every manifest emits exactly one identity block. Identity is
the **stable** part of the manifest: every other field on the
manifest is qualified by these ids.

### 5.1 Required identity fields

- `bundle_id` — opaque stable id, namespaced to the bundle's
  authoring channel (for example
  `launch_bundle:typescript_web_app.seed`,
  `org_bundle:acme.frontend_baseline`,
  `community_bundle:rust.embedded_pack`,
  `imported_bundle:user.vscode_default`,
  `local_draft_bundle:user.scratch.web`).
- `bundle_revision` — integer (≥ 1); append-only across
  manifest history.
- `bundle_revision_semver` — optional semver string for
  human reference; opaque to schema.
- `bundle_class` (§3.1).
- `bundle_source_class` (§3.2).
- `bundle_status_class` (§3.3).
- `bundle_signer_source_class` (§3.5).
- `signer_continuity_class` — re-exported from source-
  acquisition seed.
- `signature_class` — re-exported from template registry
  schema.
- `bundle_channel_relation_class` (§3.6).
- `previous_revision_ref` — opaque ref to the prior
  revision when the relation is anything other than
  `not_applicable_local_draft`. `null` when the manifest is
  the first accepted revision.
- `compatible_aureline_range` — a `runtime_compatibility_
  range` block (re-exported from template registry schema)
  covering the minimum and maximum-exclusive Aureline
  runtime versions and at least one
  `compatible_release_channel_ref`.
- `archetype_bindings[]` — at least one when
  `bundle_class = launch_bundle`. Each entry names an
  `archetype_row_id` and an `archetype_revision`.
- `support_class` — re-exported from template contract §3.3.
- `mirror_or_offline_packaging_posture` (§3.4).
- `mirror_freshness_ref` — required when packaging posture
  is `live_or_mirror`, `mirror_only`, or
  `signed_offline_bundle`.
- `dependency_markers[]` — typed
  `bundle_dependency_marker` entries (§3.7); empty array
  is allowed but the key MUST be present.
- `evidence_link_refs[]` — typed `evidence_link_kind`
  entries (§3.9); required by §3.8 rule 3 on
  certified / managed-approved / community-reviewed rows.
- `minted_at` — monotonic timestamp.

### 5.2 Identity rules (frozen)

1. `bundle_id` is **stable**: renaming a `bundle_id` is
   breaking and requires a decision row in
   [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
   Bumping `bundle_revision` is additive-minor.
2. A manifest whose `bundle_id` already appears in
   [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml)
   MUST cite the same archetype row(s) the register names;
   silent re-binding is non-conforming.
3. `compatible_aureline_range` is the **only** authoritative
   source for Aureline-version compatibility on this bundle.
   A surface that asserts narrower or wider compatibility
   without amending the manifest is non-conforming.
4. `archetype_bindings[]` are required for
   `launch_bundle` and recommended for
   `org_approved_bundle` and `design_partner_bundle`. An
   `imported_user_bundle` may be archetype-unbound until
   review.

## 6. Source / status truth

The source-class and status-class taxonomy is the load-bearing
trust contract of this manifest. The pairings allowed between
`bundle_class` (§3.1), `bundle_source_class` (§3.2), and
`bundle_status_class` (§3.3) are the closed table in §7.

Rules (frozen) that hold across every manifest:

1. **Bundle state never implies hidden trust.** Selecting,
   importing, mirroring, or pinning a bundle MUST NOT widen
   `trust_state` from `restricted` or
   `pending_evaluation` to `trusted`. Trust review remains a
   typed step routed through `review_trust_and_open` or
   `continue_in_restricted_mode` from the entry-restore
   object model.
2. **Bundle state never implies hidden sign-in.** A bundle
   that requires identity to render at all (managed-cloud
   workspace identity, connected-provider identity)
   MUST cite the sign-in step as a dependency marker; it
   MUST NOT enable the surface as a side effect of bundle
   selection.
3. **Bundle state never implies hidden install authority.**
   The inventory enumerates what WOULD install; commit
   rides through the environment-starter summary contract,
   not the manifest. A surface that installs the inventory
   on bundle selection without the summary is non-
   conforming.
4. **Source class is named verbatim.** Every projection
   (Start Center row, diagnostics row, claim row, removal
   row) renders the manifest's `bundle_source_class`
   verbatim. Aliases (`semi_certified`, `org_special`,
   `community_premium`) are non-conforming.
5. **Status class is named verbatim.** Every projection
   renders the manifest's `bundle_status_class` verbatim;
   `community_unreviewed` MUST NOT silently render as
   `community_reviewed` because of UI density.
6. **Mirror or offline posture is named verbatim.** The
   manifest's `mirror_or_offline_packaging_posture` is the
   only authoritative source for the mirror / air-gap
   freshness chip a Start Center row, diagnostics row, or
   claim row reads.
7. **Signer continuity is never elided.** Every manifest
   carries `signer_continuity_class`; a UI that hides the
   class to make a row "less noisy" is non-conforming.

## 7. Bundle class linkage

The closed pairings between `bundle_class`,
`bundle_source_class`, and `bundle_status_class` are:

| `bundle_class` | Allowed `bundle_source_class` | Allowed `bundle_status_class` set | Required identity / inventory extras |
|---|---|---|---|
| `launch_bundle` | `certified` | `certified_current`, `certified_retest_pending`, `retest_needed`, `deprecated_or_archived`, `status_unknown` | `archetype_bindings[] ≥ 1`; cite `workflow_bundle_id` from register; cite at least one `cutline_ref`; `evidence_link_refs[]` covers every required scoreboard family on the cutline. |
| `org_approved_bundle` | `managed_approved`, `certified` | `managed_approved_current`, `certified_current`, `certified_retest_pending`, `retest_needed`, `deprecated_or_archived`, `status_unknown` | `org_policy_signing_root` or `org_mirror_signing_root` signer; `compatible_aureline_range` covers the org's pinned channel. |
| `imported_user_bundle` | `imported` | `imported_pending_review`, `community_unreviewed`, `community_reviewed`, `retest_needed`, `deprecated_or_archived`, `status_unknown` | `imported_source_object_ref` cited; signer continuity carries `first_acquisition_no_prior_signer` or `signer_unknown_user_review_required`; ingress review hook required to enable mutating side effects. |
| `design_partner_bundle` | `managed_approved`, `community` | `managed_approved_current`, `community_reviewed`, `community_unreviewed`, `retest_needed`, `deprecated_or_archived`, `status_unknown` | `design_partner_pilot_scope` ref; `pilot_expiry_or_review_class` ∈ `{quarterly, semiannual, annual, on_named_event}`; promotion to `launch_bundle` requires a decision row. |
| `local_draft_bundle` | `local_draft` | `local_draft`, `deprecated_or_archived` | `local_user_trust_only`, `unsigned_user_review_required`, or `repo_local_workspace_trust` signer; `removal_surface_kind = not_applicable_local_draft`. |

Rules (frozen):

1. Any pairing not listed in this table is non-conforming.
2. Adding a column or value to this table is additive-minor
   and bumps the schema version.
3. The four reserved class linkage rows — `launch_bundle`,
   `imported_user_bundle`, `org_approved_bundle`,
   `design_partner_bundle` — are also the **stable
   projection ids** Start Center, diagnostics, and claim
   surfaces use. A surface that mints
   `featured_bundle_row`, `team_bundle_row`,
   `imported_environment_row`, or
   `pilot_bundle_row` as parallel ids is non-conforming.

## 8. Lifecycle truth

Every manifest emits exactly one `bundle_lifecycle` block.

### 8.1 Required fields

- `lifecycle_state_class` (§3.10).
- `evidence_age_class` (§3.12).
- `retest_needed_posture` (§3.13).
- `successor_recommendation` — a record with:
  - `successor_recommendation_class` (§3.11).
  - `successor_bundle_id` and
    `successor_bundle_revision` — required when the class
    is `successor_recommended_*`.
  - `successor_summary_ref` — opaque ref to a reviewable
    sentence.
  - `decision_row_ref` — required when the recommendation
    is `successor_recommended_first_party_native` or when
    the recommendation crosses bundle classes.
- `certification_sheet_refs[]` — opaque refs to the
  certification-sheet artifacts the bundle leans on.
  Required on `launch_bundle` rows whose status is
  `certified_current` or `certified_retest_pending`.
- `removal_surface_kind` (§3.14).
- `rollback_surface_kind` (§3.15).
- `disclosed_freshness_window_class` — opaque enum drawn
  from
  [`artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml).
- `keyboard_reachable = true`.

### 8.2 Lifecycle rules (frozen)

1. **Bundle truth survives install.** Every manifest
   continues to project these lifecycle fields onto the
   bundle detail panel, project-doctor row, support
   export, and removal / rollback surfaces after install.
   A surface that drops the lifecycle block once the
   bundle is installed is non-conforming.
2. **Successor recommendation never widens authority.** A
   `successor_recommended_first_party_native` block
   pointing at a `launch_bundle` does NOT promote the
   current bundle to `launch_bundle`; the user navigates
   to the successor explicitly.
3. **Removal / rollback affordances cite a typed surface.**
   `removal_surface_kind` and `rollback_surface_kind`
   resolve to the typed enums above; "remove" or
   "rollback" buttons without a surface kind are non-
   conforming.
4. **Retest pressure flows through `support_class`.** If
   `retest_needed_posture = retest_required` the manifest's
   projection on Start Center / diagnostics / claim rows
   narrows `support_class` by one rung (per §3.13 rule 2).

## 9. Surface invariants (cross-cutting)

1. **Reviewable diff.** A reviewer comparing two manifest
   revisions MUST be able to read source class, status
   class, support class, signer continuity, packaging
   posture, evidence-link refs, lifecycle state, successor
   recommendation, dependency markers, and component
   inventory deltas without external context. A diff that
   collapses any of these axes is non-conforming.
2. **One identity, many surfaces.** Start Center,
   workspace-switcher, browse-bundles, project-doctor,
   support-export, claim manifest, certified-archetype
   report, CLI / headless `aureline bundle list`, and any
   later API all read the same `bundle_id` plus
   `bundle_revision`. A surface that mints an alternate id
   is non-conforming.
3. **No silent class promotion.** A community bundle
   surfacing in an org-managed gallery is still
   `bundle_source_class = community` until promoted; a
   surface that silently relabels it `managed_approved` is
   non-conforming.
4. **No silent claim widening.** A
   `community_reviewed` bundle MUST NOT project
   `support_class = certified` claim language; the
   review only certifies the review, not the bundle.
5. **No silent install authority.** Adding a bundle to a
   manifest, mirroring it, or moving it across channels
   MUST NOT install or activate any of its inventory
   without an explicit
   `environment_starter_summary_record` rendered before
   commit.
6. **Lifecycle visibility post-install.** Bundle truth
   survives install: the lifecycle block (§8) projects
   onto the bundle detail panel, the project-doctor row,
   the support export, and the removal / rollback
   surfaces. Hiding it because the bundle is installed is
   non-conforming.
7. **Stable projections.** The four class linkage rows in
   §7 are the stable projection ids on every claim
   surface; later Start Center, diagnostics, and CLI /
   headless surfaces project the same taxonomy without
   minting parallel labels.

## 10. Worked examples

Each example has a companion fixture under
[`/fixtures/workflow/bundles/`](../../fixtures/workflow/bundles/).
Every fixture is YAML and validates against
[`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).

### 10.1 Certified launch bundle (TypeScript web app)

`bundle_class = launch_bundle`,
`bundle_source_class = certified`,
`bundle_status_class = certified_current`,
`bundle_signer_source_class = core_signing_root`,
`mirror_or_offline_packaging_posture = live_or_mirror`,
`bundle_channel_relation_class = same_channel_revision`,
`lifecycle_state_class = active_supported`,
`evidence_age_class = fresh_current`,
`retest_needed_posture = not_required`. Inventory carries
extension sets, profile / surface presets, task / launch /
debug recipes, a template scaffold ref, docs / tour /
glossary packs, certification targets bound to all seven
scoreboard families on
`cutline:replacement_grade.typescript_web_app_developer`,
and the matching evidence links. See
[`launch_bundle_typescript_web_app.yaml`](../../fixtures/workflow/bundles/launch_bundle_typescript_web_app.yaml).

### 10.2 Org-approved bundle (internal frontend baseline)

`bundle_class = org_approved_bundle`,
`bundle_source_class = managed_approved`,
`bundle_status_class = managed_approved_current`,
`bundle_signer_source_class = org_policy_signing_root`,
`mirror_or_offline_packaging_posture = mirror_only`,
`bundle_channel_relation_class = same_channel_revision`,
`lifecycle_state_class = active_supported`. Inventory cites
team-managed extension sets, an org template, an org docs
pack, and an org migration mapping. See
[`org_approved_bundle_internal_frontend_baseline.yaml`](../../fixtures/workflow/bundles/org_approved_bundle_internal_frontend_baseline.yaml).

### 10.3 Imported-user bundle pending review

`bundle_class = imported_user_bundle`,
`bundle_source_class = imported`,
`bundle_status_class = imported_pending_review`,
`bundle_signer_source_class = unsigned_user_review_required`,
`mirror_or_offline_packaging_posture = live_origin_only`,
`bundle_channel_relation_class = branch_or_fork_revision`,
`lifecycle_state_class = lifecycle_unknown`,
`retest_needed_posture = retest_required`. Inventory cites
the imported source object, a migration mapping, and an
empty `certification_targets` slot (key present, value
empty). See
[`imported_user_bundle_pending_review.yaml`](../../fixtures/workflow/bundles/imported_user_bundle_pending_review.yaml).

### 10.4 Design-partner pilot bundle

`bundle_class = design_partner_bundle`,
`bundle_source_class = managed_approved`,
`bundle_status_class = community_reviewed`,
`bundle_signer_source_class = org_policy_signing_root`,
`mirror_or_offline_packaging_posture = live_or_mirror`,
`bundle_channel_relation_class = branch_or_fork_revision`,
`lifecycle_state_class = active_supported` with
`pilot_expiry_or_review_class = quarterly`. Inventory ships
a tour pack, a glossary pack, and a typed pilot scope ref.
See
[`design_partner_bundle_field_pilot.yaml`](../../fixtures/workflow/bundles/design_partner_bundle_field_pilot.yaml).

### 10.5 Local-draft bundle

`bundle_class = local_draft_bundle`,
`bundle_source_class = local_draft`,
`bundle_status_class = local_draft`,
`bundle_signer_source_class = local_user_trust_only`,
`mirror_or_offline_packaging_posture = live_origin_only`,
`bundle_channel_relation_class = not_applicable_local_draft`,
`lifecycle_state_class = active_supported`,
`removal_surface_kind = not_applicable_local_draft`. See
[`local_draft_bundle_user_authored.yaml`](../../fixtures/workflow/bundles/local_draft_bundle_user_authored.yaml).

### 10.6 Mirror-only / signed-offline-bundle posture

`bundle_class = launch_bundle`,
`bundle_source_class = certified`,
`bundle_status_class = certified_retest_pending`,
`bundle_signer_source_class = signed_offline_bundle_root`,
`mirror_or_offline_packaging_posture = signed_offline_bundle`,
`bundle_channel_relation_class = same_channel_revision`,
`evidence_age_class = aging_within_window`,
`retest_needed_posture = retest_recommended`. Carries an
`offline_bundle_freshness_ref` and renders an in-place
mirror / air-gap policy notice. See
[`launch_bundle_signed_offline_air_gap.yaml`](../../fixtures/workflow/bundles/launch_bundle_signed_offline_air_gap.yaml).

### 10.7 Deprecated bundle with successor

`bundle_class = launch_bundle`,
`bundle_source_class = certified`,
`bundle_status_class = deprecated_or_archived`,
`bundle_signer_source_class = core_signing_root`,
`mirror_or_offline_packaging_posture = live_or_mirror`,
`bundle_channel_relation_class = cross_channel_demotion`,
`lifecycle_state_class = soft_deprecated_with_successor`,
`successor_recommendation_class =
successor_recommended_first_party_native`,
`removal_surface_kind = bundle_detail_panel`,
`rollback_surface_kind = bundle_detail_panel`. See
[`launch_bundle_deprecated_with_successor.yaml`](../../fixtures/workflow/bundles/launch_bundle_deprecated_with_successor.yaml).

### 10.8 Status-unknown bundle (offline registry)

`bundle_status_class = status_unknown`,
`mirror_or_offline_packaging_posture = packaging_posture_unknown`,
`evidence_age_class = age_unknown`,
`retest_needed_posture = retest_blocked` with
`disabled_reason_code = network_unreachable`. The fixture
demonstrates that absent status renders verbatim, never
silently becomes `certified_current`. See
[`launch_bundle_status_unknown_offline.yaml`](../../fixtures/workflow/bundles/launch_bundle_status_unknown_offline.yaml).

A `manifest.yaml` index lives alongside the fixtures and
maps every fixture file to its `bundle_class`, the closed
sets it exercises, and the rules it validates.

## 11. Acceptance mapping

- **Reviewers can diff bundle manifests and see exactly what
  classes of product state a bundle may change or
  recommend.** §4 (component inventory), §5 (identity), §6
  (source / status truth), §7 (class linkage), §8
  (lifecycle), and §9 (surface invariants) together define
  the closed axes a manifest carries. Fixtures §10.1, §10.4,
  and §10.7 exercise the diff posture across class, status,
  channel-relation, and lifecycle axes.
- **Bundle identity is consistent across Start Center, docs,
  diagnostics, exports, and later CLI / headless paths.**
  §5 (identity), §7 (class linkage), and §9.2 / §9.7
  (one-identity-many-surfaces) freeze the stable
  `bundle_id` + `bundle_revision` projection. The
  `workflow_bundle_id` register at
  [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml)
  is the upstream source of truth this contract re-exports
  by reference rather than shadowing.
- **Bundle state never implies hidden trust, sign-in, or
  install authority not declared in the manifest.** §6
  rules 1, 2, and 3, plus §9.5, plus the requirement that
  install rides through the environment-starter summary
  contract before commit. Fixtures §10.3, §10.5, and §10.8
  exercise the invariant on imported, local-draft, and
  status-unknown rows.
- **Bundle manifests can support later lifecycle visibility
  and certification surfaces without redefining bundle
  identity or class.** §8 (lifecycle truth), §9.6
  (lifecycle visibility post-install), and §3.10–§3.15
  (closed lifecycle vocabularies). Fixtures §10.6 and
  §10.7 exercise the post-install lifecycle projection on
  retest-pending and deprecated rows.

## 12. Changing this contract

- **Additive-minor** changes (new `bundle_class`, new
  `bundle_source_class`, new `bundle_status_class`, new
  `mirror_or_offline_packaging_posture`, new
  `bundle_signer_source_class`, new
  `bundle_channel_relation_class`, new
  `bundle_dependency_marker_kind`, new
  `bundle_component_kind`, new `evidence_link_kind`, new
  `lifecycle_state_class`, new
  `successor_recommendation_class`, new
  `evidence_age_class`, new `retest_needed_posture`, new
  `removal_surface_kind`, new `rollback_surface_kind`)
  land here, in
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json),
  and in at least one fixture under
  [`/fixtures/workflow/bundles/`](../../fixtures/workflow/bundles/)
  in the same change. Adding a value bumps
  `workflow_bundle_manifest_schema_version`. Each new value
  cites the motivating bundle class, status class, or
  fixture.
- **Repurposing** an existing vocabulary value or a class
  linkage pairing in §7 is breaking and requires a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (template-and-prebuild
  contract, source-acquisition seed, start-center contract,
  archetype-detection schema, workflow-bundle id register)
  happen at source and this contract re-exports by
  reference; it MUST NOT shadow the change.
- The PRD / TAD / TDD / UI-UX spec wins on any disagreement
  with the quotations in §13; this contract and its schema
  plus fixtures update in the same change.

## 13. Source anchors

- `.t2/docs/Aureline_PRD.md:254` — devcontainer
  compatibility, workspace templates, and optional
  prebuild snapshots are part of the remote story from
  day one.
- `.t2/docs/Aureline_PRD.md:1259` — remote workspaces
  should accept repo-defined devcontainer metadata and
  optional prebuild snapshots so environment setup is
  reproducible and accelerable.
- `.t2/docs/Aureline_PRD.md:2328` — intelligent project
  scaffolding and generation: starter templates and
  agentic setup for new services / apps / modules using
  team standards.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:802` — §6.9
  templates, starters, and prebuilds: source class,
  support class, runtime / toolchain, freshness, setup
  actions, always-available bypass path, side-effect
  envelope.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:6346` — §17.7
  scaffolding, generation, and template health: signal
  classes (`live`, `cached`, `policy_evaluated`,
  `not_checked`).
- `.t2/docs/Aureline_Milestones_Document.md:3787` —
  environment-capsule schema draft, workspace-template
  seed, and prebuild-metadata baseline.

## 14. Linked artifacts

- Template gallery / prebuild / resume-live disclosure
  contract (source of truth for `template_source_class`,
  `support_class`, `runtime_and_toolchain_scope`,
  `template_lifecycle_class`,
  `declared_freshness_class`, `bypass_path_id`,
  `policy_notice_class`, `generation_preflight_axis`,
  `post_create_handoff_axis`):
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- Template registry entry contract (source of truth for
  `template_registry_origin_class`,
  `template_trust_source_class`,
  `template_certification_class`, `signature_class`,
  `template_health_cadence_class`,
  `template_registry_health_state_class`,
  `runtime_compatibility_range`):
  [`/docs/templates/template_registry_and_scaffold_contract.md`](../templates/template_registry_and_scaffold_contract.md)
  and
  [`/schemas/templates/template_registry_entry.schema.json`](../../schemas/templates/template_registry_entry.schema.json).
- Workspace archetype detection / readiness preflight
  contract (source of truth for `support_claim_class`,
  `detection_outcome`, archetype binding semantics):
  [`/docs/ux/archetype_detection_contract.md`](../ux/archetype_detection_contract.md)
  and
  [`/schemas/workspace/archetype_detection.schema.json`](../../schemas/workspace/archetype_detection.schema.json).
- Workflow-bundle id register (machine-readable upstream
  for `workflow_bundle_id`, `archetype_row_id`,
  `archetype_revision`, `scoreboard_family_id`,
  `public_proof_packet_shape`, `cutline_ref`,
  `freshness_cadence`, `downgrade_trigger`):
  [`artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml).
- Launch-wedge contract (source of truth for
  P0 personas, replacement-grade cutlines, launch wedge
  bundle binding):
  [`/docs/product/launch_wedge_contract.md`](../product/launch_wedge_contract.md).
- Launch-language bundle rubric (source of truth for
  language-bundle rows referenced by `archetype_bindings[]`
  on launch bundles):
  [`/docs/product/launch_language_bundle_rubric.md`](../product/launch_language_bundle_rubric.md).
- Start Center, workspace-switcher, open-flow, restore-card,
  and recent-work disclosure contract (source of truth for
  Start Center projection of bundle rows):
  [`/docs/ux/start_center_contract.md`](../ux/start_center_contract.md).
- Bundle manifest schema (machine-readable companion to
  this contract):
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).
- Worked-example fixtures:
  [`/fixtures/workflow/bundles/`](../../fixtures/workflow/bundles/).
