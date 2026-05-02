# Learning, presentation, and classroom retention / share / delete / export contract

This document freezes the **privacy, retention, share, delete, and
export contract** every Aureline **learning-progress row**,
**dismissal event**, **don't-show-again toggle**, **speaker note**,
**classroom artifact**, **cited docs / glossary / command-tip /
imported-teaching pack**, **replay datum**, **teaching-session
bundle**, and **learning-mode profile state** resolves through before
it is retained, shared, deleted, exported, imported, or projected
onto a portable profile package, an offline pack, or a support
bundle. The goal is to keep learning and presentation data
**privacy-bounded** and **exportable without hidden retention or
surprise sharing** — a reviewer, a user, or a facilitator can read
off any learning artifact's retention class, share state, delete
posture, import provenance, and export lane without inspecting
storage bytes or guessing at provider-owned defaults.

The machine-readable schema lives at:

- [`/schemas/ux/learning_progress_retention.schema.json`](../../schemas/ux/learning_progress_retention.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/learning_privacy_cases/`](../../fixtures/ux/learning_privacy_cases/)

This contract is normative for the **retention class**, **locality
lane (local / private, shared, imported, or policy-managed)**,
**share state**, **delete scope**, **import provenance**, and
**export lane** of every learning artifact a guided shell, a
presentation surface, or a learning-mode profile observes. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, or milestone document,
those sources win and this document plus its companion schema and
fixtures update in the same change. Where a downstream tour,
glossary, classroom, or learning-mode pack mints a parallel
retention or share-state vocabulary, this contract wins and the pack
is non-conforming.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference:

- [`/docs/ux/learning_and_presentation_contract.md`](./learning_and_presentation_contract.md),
  [`/schemas/ux/learning_progress.schema.json`](../../schemas/ux/learning_progress.schema.json),
  and
  [`/schemas/ux/presentation_mode_state.schema.json`](../../schemas/ux/presentation_mode_state.schema.json)
  — `learning_progress_record`, `learning_dismissal_record`,
  `learning_progress_reset_record`, `learning_progress_manifest_record`,
  `presentation_session_record`, the closed `dismissal_class`,
  `dont_show_again_scope_class`, `progress_export_class`,
  `share_state_class`, `replay_retention_class`,
  `speaker_note_locality_class`, `audience_visibility_class`,
  `presenter_bar_visibility_class`, and `evidence_hook_class`
  vocabularies. Every retention row resolves against these upstream
  records by id; this contract carries the retention / locality /
  delete / import-provenance shape on top.
- [`/docs/ux/learnability_contract.md`](./learnability_contract.md)
  and
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  — `guided_surface_state_record`, `learning_mode_profile_record`,
  the closed `dismissal_class`, `reset_class`, and
  `progress_export_class` enums. Learning-mode profile state, the
  per-surface progress row, and the dismissal state ride those
  vocabularies; this contract mints no parallel dismissal lane.
- [`/docs/ux/no_account_local_entry_contract.md`](./no_account_local_entry_contract.md)
  and
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)
  — `state_portability_class`, `reset_class`, `export_class`,
  `profile_scope_class`. A retention row whose `locality_class` is
  `local_only_private` MUST land on a portability state row whose
  `state_portability_class` is one of `machine_local_diagnostic`,
  `device_scoped`, `account_scoped`, `policy_scoped`, or
  `ephemeral_session`; a `shared_visible` row carries through the
  portable profile lane; a `policy_managed_pushed` row reads through
  `policy_scoped`. This contract names the privacy lanes and binds
  the upstream portability table.
- [`/docs/ux/persistence_inspector_contract.md`](./persistence_inspector_contract.md)
  and
  [`/schemas/state/portable_state_package.schema.json`](../../schemas/state/portable_state_package.schema.json)
  — the remembered-state inspector, portable-state export sheet, and
  restore-provenance card. Every retention row this contract emits
  is inspectable through the persistence inspector and exportable
  through the portable-state export sheet; the export-review record
  on this contract is the learning-side projection of the export
  sheet.
- [`/docs/learning/learning_presentation_evidence_packet.md`](../learning/learning_presentation_evidence_packet.md)
  and
  [`/schemas/learning/learning_presentation_packet.schema.json`](../../schemas/learning/learning_presentation_packet.schema.json)
  — `cached_pack_continuity_state_class`, `cached_pack_kind_class`,
  `classroom_artifact_retention_class`, `tour_citation_audit_entry`,
  `cached_pack_continuity_entry`. Cited-pack retention rows resolve
  through the cached-pack continuity vocabulary; classroom-artifact
  rows resolve through the classroom-artifact retention vocabulary;
  this contract mints no parallel pack-continuity or classroom-
  artifact vocabulary.
- [`/docs/ux/learning_artifact_object_model.md`](./learning_artifact_object_model.md)
  and the six artifact schemas (`glossary_pack_item`, `tour_step`,
  `exercise_step`, `learning_mode_profile`, `teaching_session`,
  `speaker_note`) — the authored content shape every retention row
  pins by id. Speaker-note ownership / share-state, teaching-session
  share-state and replay-retention, and learning-mode-profile
  portability all resolve through those artifacts; this contract
  pins them by reference.
- [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md),
  [`/schemas/collaboration/follow_and_presenter_state.schema.json`](../../schemas/collaboration/follow_and_presenter_state.schema.json),
  and
  [`/schemas/collaboration/control_grant.schema.json`](../../schemas/collaboration/control_grant.schema.json)
  — `follow_target_record`, `presenter_state_record`,
  `control_grant_record`. `shared_visible` retention rows that ride
  a presenter / follow relationship pin those collaboration rows;
  authority **does not confer mutation** at the retention layer.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary so air-gapped, managed-cloud,
  enterprise-online, self-hosted, and individual-local envelopes
  inherit admissibility mechanically.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — `redaction_class` (ADR-0011) and `policy_context`. Every
  retention row, export review, and delete review carries a typed
  policy context and redaction class.

## Who reads this contract

- **Learning-mode, glossary, tour, and classroom pack authors** —
  to declare retention, share state, delete posture, and export lane
  for every artifact a pack ships, against one closed locality and
  retention vocabulary rather than per-pack prose.
- **Presentation, walkthrough, and facilitator authors** — to
  declare classroom-artifact retention, replay-retention, and
  speaker-note share-state per session against the upstream
  presentation contract, and to guarantee that audience identity and
  speaker notes never cross the audience boundary.
- **Reviewers, support, and compliance** — to read off any
  learning artifact's locality lane, retention class, delete
  posture, and export lane without crawling rendered UI screenshots,
  storage bytes, or pack source.
- **Admin-envelope and policy authors** — to disable, narrow, or
  pin learning-data sharing under a typed `policy_managed_pushed`
  lane rather than via bespoke policy hooks per pack.
- **Persistence-inspector and portable-state-export authors** — to
  project the learning-data portion of the inspector / export sheet
  through one retention vocabulary so a single export sheet
  enumerates progress, dismissals, don't-show-again state, speaker
  notes, classroom artifacts, cited packs, replay data, teaching
  session bundles, and learning-mode profile state under a single
  honest envelope.

## Why this exists

Without this contract, learning and presentation data drifts the
fastest of any privacy-bearing surface:

- a per-surface progress row is silently uploaded into a portable
  profile package that the user did not opt in to, leaking which
  tutorials they ran on which device;
- a `don't show again` toggle persists in a teaching-only lane that
  the portable-state export sheet does not enumerate, so the user
  cannot tell the toggle is being carried across machines;
- a speaker note authored as `presenter_owned_local` ends up
  mirrored into a session replay artifact because the replay-
  retention class was never declared;
- a classroom-managed session retains audience-visible activity
  beyond the session window with no `replay_retention_class`
  declared, so a later reviewer cannot tell what was retained and
  on what authority;
- a cited docs pack is bundled into an offline export with no
  `cached_pack_continuity_state_class`, so a later import resolves
  the pack as live-authoritative when it was actually mirrored or
  stale;
- a support bundle imports learning-progress rows that overwrite
  locally-authored progress with no `import_provenance_class`
  declared, so the user cannot tell which rows were authored on
  this profile and which were imported for support diagnostics;
- a policy-managed share lock is silently bypassed when the user
  toggles a per-profile `don't show again`, because the upstream
  `dismissal_class` was `policy_locked` but no retention row
  enforced the alignment;
- a delete action on the learning-mode profile widens its scope to
  unrelated workspace caches, recent-work metadata, or credential-
  store entries because the delete review never declared its
  preserves-unrelated-workspace-content guarantee;
- a portable profile package is exported without any record of which
  learning artifacts were excluded and why, so the user cannot
  reconstruct what stayed local.

This contract closes those gaps by declaring **one retention row
per learning artifact**, **one closed locality vocabulary** that
distinguishes local-only / shared / imported / policy-managed
state, **one closed retention vocabulary**, **one closed delete
vocabulary**, **one closed import-provenance vocabulary**, and **one
closed denial-reason set** so the persistence inspector, the
portable-state export sheet, the restore-provenance card, the
learning evidence packet, and a release-evidence reviewer all read
the same lanes.

## 1. Scope

This contract freezes four record kinds on
[`/schemas/ux/learning_progress_retention.schema.json`](../../schemas/ux/learning_progress_retention.schema.json):

1. `learning_artifact_retention_row_record` — one row per learning
   artifact, naming its locality, retention, delete posture, import
   provenance, and export lane.
2. `learning_artifact_export_review_record` — the export-side
   preflight projection: which retention rows will cross the export
   boundary, which are excluded, and the typed reason for every
   exclusion.
3. `learning_artifact_delete_review_record` — the delete-side
   preflight projection: which retention rows are in scope, the
   delete origin, the delete scope, and the const-true preserves-
   unrelated-workspace-content guarantee.
4. `learning_privacy_export_manifest_record` — the top-level
   aggregator binding all retention rows, export reviews, and delete
   reviews for a learning-mode profile, with the twelve const-true
   invariants asserted.

Out of scope at this revision:

- LMS integrations or hosted classroom services, including roster
  ingestion, grade pushback, and SCORM-style packaging.
- Concrete storage bytes for the eventual persistence database; the
  state-map and persistence-inspector contracts own those.
- Final visuals for the export sheet and delete review; the
  design-system style guide and the persistence-inspector contract
  own those.
- Author tooling (pack editors, importers, signing pipelines); the
  contract names the retention shape, downstream tooling consumes
  it.

## 2. Closed vocabularies (frozen)

All vocabularies below are frozen as closed enums on the schema
file. Adding a new value is additive-minor and bumps
`learning_progress_retention_schema_version`; repurposing an
existing value is breaking and requires a new decision row.

### 2.1 Local vocabularies (this contract)

| Vocabulary                       | Values                                                                                                                                                                                                                                  |
|----------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `learning_artifact_class`        | `progress_row`, `dismissal_event`, `dont_show_again_state`, `speaker_note`, `classroom_artifact`, `cited_pack`, `replay_data`, `teaching_session_bundle`, `learning_mode_profile_state`                                                  |
| `locality_class`                 | `local_only_private`, `shared_visible`, `imported_external_authored`, `policy_managed_pushed`                                                                                                                                           |
| `retention_class`                | `ephemeral_session`, `until_user_clears`, `until_session_ends`, `until_pack_revision_change`, `classroom_artifact_privacy_bounded`, `replay_blocked_by_policy`, `no_retention`                                                          |
| `delete_class`                   | `deletable_per_profile`, `deletable_per_device`, `deletable_per_account`, `deletable_by_policy_reset`, `not_deletable_locally`                                                                                                          |
| `import_provenance_class`        | `locally_authored`, `imported_from_portable_profile`, `imported_from_offline_pack`, `imported_from_support_bundle`, `policy_pushed_central`                                                                                             |
| `delete_origin_class`            | `user_initiated_delete`, `policy_initiated_delete`, `imported_profile_package_carried_delete`, `device_clear_on_launch_delete`, `facilitator_session_end_delete`                                                                        |
| `export_review_exclusion_reason` | `excluded_local_only_machine_bound`, `excluded_policy_blocked`, `excluded_unavailable_in_envelope`, `excluded_redaction_required`, `excluded_replay_blocked_by_policy`, `excluded_classroom_artifact_privacy_bounded`, `excluded_imported_support_bundle_audit_only`, `excluded_speaker_note_audience_visible_denied`, `excluded_pack_offline_unverified` |

### 2.2 Re-exported vocabularies (do **not** re-mint)

These vocabularies are re-exported verbatim from upstream schemas
and never re-minted on this contract:

- `dismissal_class`, `progress_export_class` — from
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  via
  [`/schemas/ux/learning_progress.schema.json`](../../schemas/ux/learning_progress.schema.json).
- `dont_show_again_scope_class`, `evidence_hook_class` — from
  [`/schemas/ux/learning_progress.schema.json`](../../schemas/ux/learning_progress.schema.json).
- `share_state_class`, `replay_retention_class`,
  `speaker_note_locality_class` — from
  [`/schemas/ux/presentation_mode_state.schema.json`](../../schemas/ux/presentation_mode_state.schema.json).
- `cached_pack_continuity_state_class`, `cached_pack_kind_class`,
  `classroom_artifact_retention_class` — from
  [`/schemas/learning/learning_presentation_packet.schema.json`](../../schemas/learning/learning_presentation_packet.schema.json).
- `deployment_profile_id` — from
  [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml).
- `redaction_class`, `policy_context` — from
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  and
  [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json).

## 3. The four privacy lanes

`locality_class` is the single switch that distinguishes the four
privacy lanes. Every retention row resolves through exactly one
lane.

| Lane                        | Plain meaning                                                                                                                                       | Required posture                                                                                                                          |
|-----------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| `local_only_private`        | Stays on this device / profile. Never reaches a portable profile package, an audience surface, or a remote relay.                                   | `progress_export_class` ∈ {`not_exported_machine_local`, `in_support_bundle_redacted`}; `share_state_class` ∈ {`local_only_no_share`, `shared_unavailable_in_envelope`}. |
| `shared_visible`            | Admissible to a named audience, classroom-managed pool, or facilitator-only adjunct under a shared `share_state_class`.                              | `share_state_class` MUST be one of the `shared_*` values; an air-gapped envelope downgrades to `shared_unavailable_in_envelope`.          |
| `imported_external_authored`| Imported from a portable profile package, an offline pack, or a support bundle authored elsewhere; carries provenance and never overwrites locally-authored items silently. | `import_provenance_class` ∈ {`imported_from_portable_profile`, `imported_from_offline_pack`, `imported_from_support_bundle`}; the matching source ref is required. |
| `policy_managed_pushed`     | Owned by the active policy bundle. The user cannot delete it locally.                                                                               | `import_provenance_class = policy_pushed_central`; `delete_class` ∈ {`deletable_by_policy_reset`, `not_deletable_locally`}; `active_policy_bundle_ref` and `policy_context` required. |

## 4. Retention rules per artifact class

Rules (frozen):

1. **Progress rows** ride the upstream
   `learning_progress_record` lanes. Retention is `until_user_clears`
   on every progress row that the user can reset; an
   `ephemeral_session` row is admissible only when
   `dismissal_class = per_session_one_shot`. Hidden retention beyond
   what the upstream `dismissal_class` permits is non-conforming
   (`denial_reason = learning_retention_dont_show_again_scope_does_not_match_dismissal_class`).
2. **Dismissal events** retain only what the upstream
   `learning_dismissal_record` carries; raw user identifiers,
   account ids, and pack body never appear. The retention row pins
   the dismissal record by id and aligns `dismissal_class` and
   `dont_show_again_scope_class`.
3. **`don't show again` state** carries a closed
   `dont_show_again_scope_class` whose value MUST agree with the
   row's `dismissal_class`: `per_profile_dismissable` ⇒
   `dont_show_again_per_profile`; `per_device_dismissable` ⇒
   `dont_show_again_per_device`; `policy_locked` ⇒
   `dont_show_again_locked_by_policy`; etc. Misalignment is
   non-conforming.
4. **Speaker notes** carry the upstream
   `speaker_note_locality_class` and align `share_state_class`:
   `presenter_owned_local` ⇒ `local_only_no_share`;
   `co_presenter_visible` ⇒ a `shared_*` value; `blocked_by_policy`
   ⇒ `shared_blocked_by_policy` and a policy bundle ref. Audience-
   visible speaker notes under a `local_only_no_share` posture are
   non-conforming
   (`denial_reason = learning_retention_speaker_note_audience_visible_under_local_only_share`).
5. **Classroom artifacts** retain audience-visible activity only
   under `classroom_artifact_privacy_bounded` retention with the
   `privacy_bounded_classroom_artifact` evidence hook reserved and a
   declared `replay_retention_class`. A classroom-artifact row that
   omits `replay_retention_class` is non-conforming
   (`denial_reason = learning_retention_classroom_artifact_lacks_replay_retention_class`).
6. **Cited packs** (`cached_pack_kind_class` ∈ {`docs_pack`,
   `glossary_pack`, `command_tip_pack`, `imported_teaching_pack`})
   declare a `cached_pack_continuity_state_class` and, when
   `retention_class = until_pack_revision_change`, pin
   `pack_revision_ref`. Offline export of a pack that ships under
   `pack_offline_bundle_unverified_suppressed` is non-conforming
   (`denial_reason = learning_retention_cited_pack_offline_export_lacks_continuity_state`).
7. **Replay data** declares a `replay_retention_class` and, when the
   class is `replay_blocked_by_policy`, an `active_policy_bundle_ref`
   and `policy_context`.
8. **Teaching-session bundles** declare a `share_state_class`, a
   `replay_retention_class`, and the bound `teaching_session_ref`;
   share state under air-gapped envelopes downgrades to
   `shared_unavailable_in_envelope`.
9. **Learning-mode profile state** rides the upstream
   `state_portability_class` lanes from
   [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json);
   `local_only_private` learning-mode state lands on
   `machine_local_diagnostic`, `device_scoped`, `account_scoped`,
   `policy_scoped`, or `ephemeral_session` portability; portable
   learning-mode state lands on `portable_profile_state`.

## 5. Local vs shared vs imported vs policy-managed

Rules (frozen):

1. **`local_only_private` never exports via the portable lane.**
   `progress_export_class` MUST be `not_exported_machine_local` or
   `in_support_bundle_redacted`; `share_state_class` MUST be
   `local_only_no_share` or `shared_unavailable_in_envelope`. A
   `local_only_private` row with `in_portable_profile_package` is
   non-conforming
   (`denial_reason = learning_retention_local_only_private_exports_via_portable_lane`).
2. **`shared_visible` matches the deployment envelope.** A
   `shared_to_classroom_managed_pool` row is admissible only on
   `enterprise_online` or `managed_cloud`; an air-gapped envelope
   downgrades to `shared_unavailable_in_envelope`. A row that asserts
   shared visibility under air-gapped is non-conforming
   (`denial_reason = learning_retention_share_state_does_not_match_envelope`).
3. **`imported_external_authored` carries provenance.** Every
   imported row pins the source via `imported_profile_package_ref`,
   `imported_offline_pack_ref`, or `imported_support_bundle_ref`,
   matching the `import_provenance_class`. An imported row with no
   provenance ref is non-conforming
   (`denial_reason = learning_retention_imported_artifact_lacks_provenance`).
4. **`policy_managed_pushed` carries a policy context.** Every
   policy-managed row pins `active_policy_bundle_ref` and a typed
   `policy_context`; `delete_class` is `deletable_by_policy_reset`
   or `not_deletable_locally`. A policy-managed row with no policy
   context is non-conforming
   (`denial_reason = learning_retention_policy_managed_pushed_lacks_policy_context`).
5. **Imported support-bundle rows are read-only.** A row whose
   `import_provenance_class = imported_from_support_bundle` MUST
   carry `delete_class = not_deletable_locally`; the user observes
   the imported audit metadata but cannot delete or re-export it.

## 6. Export review

The `learning_artifact_export_review_record` is the export-side
preflight: it lists every retention row that **will** cross the
export boundary on this review (`selected_row_refs`) and every row
that **will not** with a typed `export_review_exclusion_reason`
(`excluded_entries`). Hidden omission is non-conforming
(`denial_reason = learning_retention_export_review_excluded_artifact_lacks_reason`).

Rules (frozen):

1. **Every excluded row carries a typed reason.** A row excluded
   because it is `local_only_private` carries
   `excluded_local_only_machine_bound`; a row excluded under policy
   carries `excluded_policy_blocked`; a row excluded because the
   envelope cannot honour it carries `excluded_unavailable_in_envelope`;
   a row excluded because it would require redaction outside the
   review's `redaction_class` carries `excluded_redaction_required`;
   a replay row blocked by policy carries
   `excluded_replay_blocked_by_policy`; a classroom artifact whose
   privacy bound forbids export carries
   `excluded_classroom_artifact_privacy_bounded`; an imported support
   bundle row carries `excluded_imported_support_bundle_audit_only`;
   an audience-visible speaker note that would leak under the export
   redaction carries `excluded_speaker_note_audience_visible_denied`;
   an offline pack that ships unverified carries
   `excluded_pack_offline_unverified`.
2. **Selected rows resolve through allowed lanes.**
   `allowed_progress_export_classes` enumerates the export lanes the
   review asserts as admissible; every selected retention row's
   `progress_export_class` MUST be in that list. Rows whose
   `progress_export_class` would not be in the list MUST be on
   `excluded_entries` instead.
3. **Air-gapped reviews do not export classroom-managed state.** A
   review whose `deployment_profile_refs` includes `air_gapped` MUST
   exclude every `shared_to_classroom_managed_pool` row with
   `excluded_unavailable_in_envelope`.
4. **Imported rows do not silently re-export.** A retention row
   whose `locality_class = imported_external_authored` MUST land
   on `excluded_imported_support_bundle_audit_only`,
   `excluded_redaction_required`, or `excluded_unavailable_in_envelope`
   unless the review explicitly carries the source profile package
   reference and a re-export posture (covered by upstream portable-
   state package contract).

## 7. Delete review

The `learning_artifact_delete_review_record` is the delete-side
preflight: it lists every retention row in scope (`row_refs`),
declares the `delete_class` and `delete_origin_class`, and asserts
the const-true `preserves_unrelated_workspace_content = true`
guarantee.

Rules (frozen):

1. **Delete is scoped to the named retention rows.** The review
   MUST NOT delete unrelated workspace content, source files,
   workspace manifests, caches, recent-work metadata, or credential-
   store entries. A delete review that widens scope is non-conforming
   (`denial_reason = learning_retention_delete_review_widens_to_unrelated_workspace_content`).
2. **Delete scope matches locality lane.** A
   `deletable_per_profile` review is admissible only on
   `local_only_private` or `shared_visible` rows; a
   `deletable_by_policy_reset` review is admissible only on
   `policy_managed_pushed` rows; a `not_deletable_locally` review
   denies. A misaligned review is non-conforming
   (`denial_reason = learning_retention_delete_scope_does_not_match_locality_lane`).
3. **Policy-initiated deletes pin the policy bundle.** A review
   whose `delete_origin_class = policy_initiated_delete` or whose
   `delete_class = deletable_by_policy_reset` MUST pin
   `active_policy_bundle_ref`.
4. **Imported-profile-carried deletes pin the source.** A review
   whose `delete_origin_class =
   imported_profile_package_carried_delete` MUST pin
   `imported_profile_package_ref`.
5. **Facilitator-session-end deletes drain the session window.** A
   review whose `delete_origin_class =
   facilitator_session_end_delete` is the operation that releases
   `until_session_ends` retention rows for a teaching session and
   MUST land on `delete_class = deletable_per_profile` or
   `deletable_per_device`.

## 8. Import / export across portable profiles, offline packs, and support packets

Rules (frozen):

1. **Portable profile import preserves provenance.** Every retention
   row imported from a portable profile package lands as
   `locality_class = imported_external_authored` with
   `import_provenance_class = imported_from_portable_profile` and
   `imported_profile_package_ref` pinned; the row never silently
   adopts `locally_authored` provenance.
2. **Offline pack import resolves through cached-pack continuity.**
   Every retention row imported from an offline pack lands as
   `locality_class = imported_external_authored` with
   `import_provenance_class = imported_from_offline_pack`,
   `imported_offline_pack_ref` pinned, and a declared
   `cached_pack_continuity_state_class`. An offline-bundle-unverified
   pack lands on `pack_offline_bundle_unverified_suppressed` and
   denies export.
3. **Support packet import is read-only.** Every retention row
   imported from a support bundle lands as
   `locality_class = imported_external_authored` with
   `import_provenance_class = imported_from_support_bundle`,
   `imported_support_bundle_ref` pinned, and
   `delete_class = not_deletable_locally`; the row is observable
   for diagnostic review but never re-exports through a portable-
   profile lane.
4. **Cited packs survive export only with continuity state.** A
   cited-pack retention row exported into a support bundle or a
   portable profile MUST carry a non-suppressed
   `cached_pack_continuity_state_class`; a suppressed continuity
   state denies export with
   `learning_retention_cited_pack_offline_export_lacks_continuity_state`.
5. **Speaker notes never cross to the audience export.** A speaker
   note retention row whose `share_state_class = local_only_no_share`
   or `shared_to_facilitator_only` MUST be excluded from any export
   review whose `redaction_class` is `metadata_safe_default`
   (default audience-facing).

## 9. Invariants on the manifest

Every `learning_privacy_export_manifest_record` declares twelve
const-true invariants:

1. `local_only_private_never_exports_via_portable_lane`
2. `policy_managed_pushed_carries_policy_context`
3. `imported_artifact_carries_provenance_class`
4. `classroom_artifact_carries_replay_retention_class`
5. `replay_blocked_by_policy_carries_policy_context`
6. `speaker_note_share_state_matches_ownership`
7. `cited_pack_offline_export_carries_continuity_state`
8. `dont_show_again_scope_matches_dismissal_class`
9. `delete_scope_matches_locality_lane`
10. `export_review_lists_excluded_artifacts_with_typed_reason`
11. `delete_review_preserves_unrelated_workspace_content`
12. `share_state_matches_deployment_envelope`

A row asserting any invariant as `false` is a schema violation.

## 10. Acceptance mapping

| Acceptance clause                                                                                                                            | Resolved by                                                                                                                                                                                                                                |
|----------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Users and reviewers can tell what learning data stays local, what may be shared, and what survives export or restore.                         | §3 four privacy lanes, §4 retention rules per artifact class, §6 export review, §9 manifest invariants 1, 6, 7, 10, 12.                                                                                                                    |
| Learning/privacy states do not depend on hidden defaults or provider-owned storage assumptions.                                              | §2 closed vocabularies (no provider-owned defaults), §3 explicit lanes, §5 lane rules, §6 export-review exclusion taxonomy.                                                                                                                |
| Fixtures cover at least: local-only progress, shared presentation notes, offline cited-pack export, and policy-disabled sharing.             | §11 worked examples and `/fixtures/ux/learning_privacy_cases/`.                                                                                                                                                                            |
| Retention/share/delete/export rules cover progress, dismissals, don't-show-again state, speaker notes, classroom artifacts, cited packs, and replay data. | §1 record kinds, §2.1 `learning_artifact_class` (nine values), §4 per-class retention rules.                                                                                                                                              |
| Import/export behavior for portable profiles, offline packs, and support packets that reference learning artifacts.                          | §8 import/export rules, §2.1 `import_provenance_class`, §6 export review's exclusion taxonomy.                                                                                                                                             |

## 11. Worked examples

Fixtures under
[`/fixtures/ux/learning_privacy_cases/`](../../fixtures/ux/learning_privacy_cases/)
cover:

1. **Local-only progress retention** —
   `progress_local_only_private.yaml`. A
   `learning_artifact_retention_row_record` whose
   `artifact_class = progress_row`, `locality_class = local_only_private`,
   `retention_class = until_user_clears`,
   `progress_export_class = not_exported_machine_local`,
   `share_state_class = local_only_no_share`. Air-gapped admissible.
2. **Shared presentation speaker note** —
   `speaker_note_shared_presentation.yaml`. A
   `learning_artifact_retention_row_record` whose
   `artifact_class = speaker_note`,
   `locality_class = shared_visible`,
   `share_state_class = shared_to_facilitator_only`,
   `speaker_note_locality_class = speaker_notes_facilitator_only_local`,
   `retention_class = until_session_ends`.
3. **Classroom artifact privacy-bounded** —
   `classroom_artifact_privacy_bounded.yaml`. A
   `learning_artifact_retention_row_record` whose
   `artifact_class = classroom_artifact`,
   `retention_class = classroom_artifact_privacy_bounded`,
   `replay_retention_class = classroom_artifact_privacy_bounded`,
   `classroom_artifact_retention_class =
   classroom_artifact_privacy_bounded_disclosed`, with
   `privacy_bounded_classroom_artifact` evidence hook reserved.
4. **Offline cited-pack export** —
   `cited_pack_offline_export.yaml`. A
   `learning_artifact_retention_row_record` whose
   `artifact_class = cited_pack`,
   `pack_kind = docs_pack`,
   `cached_pack_continuity_state_class = pack_offline_bundle_verified`,
   `retention_class = until_pack_revision_change`,
   admissible on `air_gapped` and `enterprise_online`.
5. **Policy-disabled sharing** —
   `policy_disabled_sharing.yaml`. A
   `learning_artifact_retention_row_record` whose
   `locality_class = policy_managed_pushed`,
   `share_state_class = shared_blocked_by_policy`,
   `delete_class = not_deletable_locally`,
   `import_provenance_class = policy_pushed_central`,
   pinning the active policy bundle.
6. **Imported support-bundle progress (read-only audit)** —
   `progress_imported_from_support_bundle.yaml`. A
   `learning_artifact_retention_row_record` whose
   `artifact_class = progress_row`,
   `locality_class = imported_external_authored`,
   `import_provenance_class = imported_from_support_bundle`,
   `delete_class = not_deletable_locally`.
7. **Don't-show-again per-profile** —
   `dont_show_again_per_profile.yaml`. A
   `learning_artifact_retention_row_record` whose
   `artifact_class = dont_show_again_state`,
   `dismissal_class = per_profile_dismissable`,
   `dont_show_again_scope_class = dont_show_again_per_profile`,
   `progress_export_class = in_portable_profile_package_redacted`.
8. **Replay blocked by policy** —
   `replay_blocked_by_policy.yaml`. A
   `learning_artifact_retention_row_record` whose
   `artifact_class = replay_data`,
   `retention_class = replay_blocked_by_policy`,
   `replay_retention_class = replay_blocked_by_policy`,
   `locality_class = policy_managed_pushed`.
9. **Export review covering local-only and policy-blocked rows** —
   `export_review_local_only_and_policy_blocked.yaml`. A
   `learning_artifact_export_review_record` whose
   `selected_row_refs` includes a portable progress row and
   `excluded_entries` declares the local-only progress row with
   `excluded_local_only_machine_bound` and the policy-blocked row
   with `excluded_policy_blocked`.
10. **Delete review preserving unrelated workspace content** —
    `delete_review_preserves_workspace.yaml`. A
    `learning_artifact_delete_review_record` whose
    `delete_class = deletable_per_profile`,
    `delete_origin_class = user_initiated_delete`, and
    `preserves_unrelated_workspace_content = true`.
11. **Manifest declaring twelve invariants** —
    `learning_privacy_export_manifest_default_individual_learner.yaml`.
    A `learning_privacy_export_manifest_record` for the default
    individual-learner profile binding the eight retention rows
    above plus the export and delete reviews; declares all twelve
    const-true invariants.

## 12. Adding a new vocabulary value

Adding a new `learning_artifact_class`, `locality_class`,
`retention_class`, `delete_class`, `import_provenance_class`,
`delete_origin_class`, `export_review_exclusion_reason`,
`invariant_id`, or `denial_reason` is **additive-minor** and bumps
`learning_progress_retention_schema_version`. Repurposing an
existing value is **breaking** and requires a new decision row on
the launch decision register. A consumer surface that resolves a
value it does not recognize MUST deny with
`learning_progress_retention_schema_version_lagging` rather than
silently fall back.

## 13. Out of scope at this revision

- LMS integrations or hosted classroom services (roster ingestion,
  grade pushback, SCORM-style packaging).
- Concrete storage bytes for the eventual persistence database;
  the state-map and persistence-inspector contracts own those.
- Final visuals for the export sheet and delete review; the
  design-system style guide and the persistence-inspector contract
  own those.
- Pack editor / signing pipeline tooling; this contract names the
  retention shape, downstream tooling consumes it.
- Concrete policy-bundle content. The identity / policy-bundle
  contracts own those; this contract names the typed policy lane.
