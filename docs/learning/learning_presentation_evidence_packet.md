# Learning/presentation evidence packet, tour-citation audit, and cached-pack continuity contract

This contract freezes one shared vocabulary for the
**learning/presentation evidence packet**. It exists so milestone
close, release readiness, accessibility audit, docs-pack review,
support handoff, and public-proof review can state, in one
inspectable record, exactly which guided surfaces (guided tours,
learning-mode profiles, architecture maps, glossary packs,
command-tip packs, presentation sessions) the window audited, what
tour-citation drift was observed, what cached-pack continuity state
the docs/glossary/teaching packs are in, what presenter / co-presenter
/ observer / audience role bindings the session set declares, what
layout-restore proof every layout anchor lands on, what reduced-motion
and keyboard-reachability posture the surfaces hold, and what
classroom-artifact retention and export-redaction posture the export
pipelines guarantee — without scattering the same information across
screenshots, ad-hoc demos, support-bundle appendices, claim-manifest
narrowings, and release-notes paragraphs.

The learning/presentation evidence packet is a single object family.
Every packet projects one window (milestone close, release train,
weekly governance review, ad-hoc review, support handoff,
accessibility audit, docs-pack review) and renders eight typed
sections:

1. The **schema-version pin block** — pinned integer versions of
   `schemas/ux/guided_surface_state.schema.json`,
   `schemas/ux/guided_tour.schema.json`,
   `schemas/ux/learning_progress.schema.json`, and
   `schemas/ux/presentation_mode_state.schema.json` so a reviewer can
   detect schema drift between the packet and its sources.
2. The **surface-coverage matrix** — typed list of the guided-surface
   kinds the packet exercises (onboarding card, glossary card,
   guided-tour step, architecture explainer, architecture map,
   contextual tip, keymap-bridge hint, exercise step, command-tip
   pack, learning-mode profile, speaker-note adjunct,
   teaching-session adjunct, presentation session) plus the
   `freshness_class`, the
   `accessibility_keyboard_coverage_class`, and the worked-example
   fixture refs that back the claim.
3. The **tour-citation audit** — every waypoint citation the packet
   covered walked back to its canonical anchor (`command_id`,
   `docs_citation_anchor`, `symbol_linked_reference`, or
   `keymap_bridge`) plus the observed
   `tour_citation_drift_class` so a reviewer can see whether the
   tour or walkthrough still resolves against the running build's
   command registry, docs pack revision, and symbol-linked-reference
   set, or whether the citation has drifted (anchor remapped, command
   id remapped, docs pack revision drifted, symbol reference
   remapped, keymap bridge canonical id drifted, or anchor
   unresolvable).
4. The **cached-pack continuity** — typed list naming, per
   `docs_pack_ref` / `glossary_pack_ref` / `command_tip_pack_ref` /
   `imported_teaching_pack_ref`, the observed
   `cached_pack_continuity_state_class` (live authoritative full
   install, not installed, cached within TTL, mirror-only,
   stale past TTL, offline bundle verified, offline bundle
   unverified, revoked or withdrawn) plus the observed
   `freshness_class` and the cited
   `docs_pack_revision_ref` so the learning experience stays honest
   about source freshness and availability.
5. The **presentation-role review** — typed list naming, per session,
   the presenter / co-presenter / observer / audience bindings, the
   `share_state_class`, the `follow_mode_class`, the
   `speaker_note_locality_class`, the
   `presenter_bar_visibility_class`, the
   `audience_visibility_class`, the
   `local_remote_shared_boundary_class`, plus any
   `presenter_handoff_record` and `breakaway_event_record` refs and
   their `breakaway_recovery_class`.
6. The **layout-restore proof** — typed list naming, per layout
   anchor on the packet's covered sessions (presenter bar, agenda or
   waypoint rail, spotlight frame, breakaway banner, speaker-note
   locality), the `layout_target_class` it landed on (a canonical
   workspace surface) and the `local_remote_shared_boundary_class` it
   honoured, so a reviewer can prove no private "presentation shell"
   target was minted.
7. The **classroom-artifact retention aggregate** — typed list
   naming, per session, the
   `classroom_artifact_retention_class` (no artifact retained,
   metadata-only local redacted, metadata-only in support bundle
   redacted, classroom artifact privacy-bounded, replay blocked by
   policy) plus the
   `replay_retention_class` re-projected from the
   presentation-mode-state schema, so a reviewer can audit what
   audience-visible artifacts a classroom or facilitator-led session
   retained without leaking audience identity.
8. The **export-redaction posture** — typed list asserting
   `no_raw_speaker_notes_cross_boundary`,
   `audience_identity_never_leaked`,
   `imported_pack_verification_state_preserved`,
   `cached_pack_continuity_state_preserved`,
   `replay_retention_class_preserved`, and the per-export floor that
   presenter-bar audience-invisibility, speaker-note locality,
   breakaway recovery class, accessibility/keyboard state,
   reduced-motion state, and policy-locked dismissal posture travel
   through every export pipeline.

The packet also carries the typed window kind, the typed audience and
redaction profile (paired by schema gates), the typed
linked-artifact-families block (the learnability contract, the
learning-and-presentation contract, the guided-surface state schema,
the command descriptor contract, and the citation anchor object
model must always be cited), the typed change-significance summary
(informational, release-bearing, claim-narrowing,
claim-widening-blocked), the typed consuming-surface parity floor,
the typed `policy_context`, the typed `running_build_identity_ref`,
and the typed evaluated/minted/frozen/superseded/withdrawn
timestamps. Raw speaker-note bodies, raw audience chat or Q&A bodies,
raw audience identity, raw glossary pack bodies, raw URLs, raw
absolute paths, raw imported teaching pack payloads, and raw secrets
MUST NOT appear; the record carries opaque refs, typed vocabulary,
and bounded reviewable summaries only.

The contract is pre-implementation. It defines the reusable record
shape, the closed vocabularies, the projection rules, the export-
parity floor, the change-significance rules, and the fixture corpus.
It does not implement tours, walkthroughs, learning-mode flows,
classroom enrollment, presentation-mode UI, or post-session review.

## Companion artifacts

- [`/schemas/learning/learning_presentation_packet.schema.json`](../../schemas/learning/learning_presentation_packet.schema.json)
  — boundary schema for one
  `learning_presentation_evidence_packet_record` plus the
  audit-event family.
- [`/fixtures/learning/learning_presentation_cases/`](../../fixtures/learning/learning_presentation_cases/)
  — worked records covering an informational milestone-close packet,
  an accessibility-audit packet that asserts reduced-motion and
  keyboard-reachability coverage, a release-bearing packet that
  asserts parity across every consuming surface, a claim-narrowing
  packet that pulls a public claim back because the imported
  teaching pack is unverified under the current envelope, a
  claim-widening-blocked packet held by a docs-pack mirror-only
  window plus a presenter handoff with breakaway recovery, and a
  denial event for an export-redaction posture that misses the
  five-class floor.
- [`/docs/ux/learnability_contract.md`](../ux/learnability_contract.md)
  and
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  — `guided_surface_state_record`, `guided_surface_rule_record`,
  `learning_mode_profile_record`, `guided_surface_kind`,
  `guidance_authority_class`, `freshness_class`,
  `suppression_cause_class`, and the twelve const-true invariants
  on the learnability manifest. The packet's surface-coverage matrix
  re-exports the closed `guided_surface_kind` set without
  redeclaring it.
- [`/docs/ux/learning_and_presentation_contract.md`](../ux/learning_and_presentation_contract.md)
  and the companion schemas
  [`/schemas/ux/guided_tour.schema.json`](../../schemas/ux/guided_tour.schema.json),
  [`/schemas/ux/learning_progress.schema.json`](../../schemas/ux/learning_progress.schema.json),
  and
  [`/schemas/ux/presentation_mode_state.schema.json`](../../schemas/ux/presentation_mode_state.schema.json)
  — `guided_tour_record`, `tour_waypoint_record`,
  `architecture_map_record`, `command_tip_pack_record`,
  `learning_progress_record`, `presentation_session_record`,
  `presentation_layout_anchor_record`, `breakaway_event_record`,
  `presentation_invariant_manifest_record` plus their closed
  vocabularies (tour audience class, indexing readiness class,
  layout target class, partial indexing disclosure class, source
  language fallback class, share state class, replay retention
  class, reduced-motion posture class, accessibility keyboard
  class, presenter bar visibility class, audience visibility class,
  speaker note locality class, local-remote-shared boundary class,
  breakaway recovery class, layout anchor kind). The packet is the
  audit-time evidence family that qualifies the five evidence-hook
  reserves declared on those records (`tour_citation_audit`,
  `cached_pack_continuity`, `presentation_role_review`,
  `layout_restore_proof`, `privacy_bounded_classroom_artifact`).
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  — canonical `command_id` registry. Tour-citation audit entries
  with `anchor_kind = command_id_anchor` resolve through this
  registry; a citation that no longer resolves on the running
  build's registry MUST emit
  `command_id_remapped_disclosed` or
  `citation_anchor_unresolvable`.
- [`/docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md)
  and
  [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  — `docs_citation_anchor_record`, `docs_pack_ref`,
  `docs_pack_revision_ref`, `freshness_class`, and the docs-pack
  freshness state machine. Cached-pack continuity entries cite
  citation anchors and pack revisions through this object model
  rather than minting a parallel pack-status vocabulary.
- [`/schemas/docs/symbol_linked_reference.schema.json`](../../schemas/docs/symbol_linked_reference.schema.json)
  — symbol-linked-reference object model. Architecture-map and
  symbol-anchored waypoint citations resolve here.
- [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md),
  [`/schemas/collaboration/follow_and_presenter_state.schema.json`](../../schemas/collaboration/follow_and_presenter_state.schema.json),
  and
  [`/schemas/collaboration/control_grant.schema.json`](../../schemas/collaboration/control_grant.schema.json)
  — `follow_target_record`, `presenter_state_record`,
  `presenter_handoff_record`, `control_grant_record`. Presentation-
  role review entries cite presenter, co-presenter, follow target,
  and breakaway records through the collaboration schema; mutating
  control is never inferable from a presenter or follow row and is
  only readable through a `control_grant_record`.
- [`/schemas/ux/transient_surface.schema.json`](../../schemas/ux/transient_surface.schema.json)
  — transient preview primitives composed by contextual tips,
  command tips, and waypoint hovercards. Surface-coverage entries
  whose `guided_surface_kind` renders through a transient preview
  cite the transient-preview record by ref.
- [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
  — keymap-bridge resolver. Tour-citation audit entries whose
  `anchor_kind = keymap_bridge_anchor` resolve through this
  resolver and emit
  `keymap_bridge_canonical_id_drifted_disclosed` when the bridge no
  longer lands on the canonical command id.
- [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)
  — onboarding-portability manifest. Learning-progress and
  dismissal evidence on the packet rides this contract's portability
  table; the packet does not mint a teaching-only progress lane.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary. The packet declares which
  envelopes the audited surfaces are admissible under and resolves
  share-state / replay-retention envelope mismatches through this
  vocabulary.

This contract **composes with and does not replace** vocabularies
already frozen in:

- the learnability contract (`guided_surface_kind`,
  `guidance_authority_class`, `dismissal_class`,
  `freshness_class`, `suppression_cause_class`,
  `learning_mode_profile_class`).
- the learning-and-presentation contract (tour audience class,
  indexing readiness class, layout target class, share state class,
  replay retention class, reduced-motion posture class,
  accessibility keyboard class, presenter bar visibility class,
  audience visibility class, speaker note locality class,
  local-remote-shared boundary class, breakaway recovery class,
  layout anchor kind).
- ADR 0011 (`freshness_class`, `redaction_class`, `client_scope`).
- ADR 0007 (broker-owned redaction pass).
- ADR 0001 / ADR 0018 (workspace-trust state).

If this document disagrees with those sources, those sources win and
this document plus the schema are updated in the same change.

The eventual learning / presentation crate's Rust types are the
schema of record. The boundary schema at
`schemas/learning/learning_presentation_packet.schema.json` is the
cross-tool boundary every non-owning surface reads.

## Why freeze this now

Without one frozen packet, every release / milestone / accessibility
review / docs-pack review / support handoff is free to invent its own
way of summarising learning and presentation truth. Each divergence
widens a different axis silently:

1. *A release-readiness reviewer reads "the tour works" from a
   screenshot.* The reviewer cannot tell which tour audience it
   exercised, what indexing readiness state it ran under, what tour
   waypoints had drift, or whether the docs pack the waypoints cite
   is still installed and within TTL.
2. *An accessibility auditor asks "did the tour respect reduced
   motion?".* Without one packet that pins the reduced-motion
   posture and keyboard-reachability state per surface kind, the
   auditor has to reconstruct it from screen-reader logs and video
   demos that may already be redacted.
3. *A claim-manifest reviewer reads "presentation mode is shippable"
   without knowing whether speaker-note locality and presenter-bar
   visibility were honoured under the audited share-state and
   audience-visibility envelope.* The claim is unverifiable.
4. *A support engineer asks "what classroom artifacts were
   retained?".* Without one packet that pins the
   classroom-artifact retention class plus the replay-retention
   class per session, the engineer has to reconstruct it from the
   support bundle that may not even include the retained artifact
   metadata.
5. *A public-proof packet exports a speaker-note body because the
   redaction pipeline did not preserve the
   `speaker_notes_facilitator_only_local` posture.* Speaker-note
   content leaks.
6. *A docs-pack review packet reports "glossary cards cite the
   pack" without disclosing whether any of the audited cards
   resolved against a stale-past-TTL or mirror-only pack snapshot.*
   The audit lost continuity because no record paired the
   cached-pack continuity state to the freshness class and the
   docs-pack revision ref.
7. *A breakaway from the presenter view leaves the audience member
   in an unspecified state in the audit.* Without one packet that
   pins the breakaway recovery class per event, the audit cannot
   tell whether rejoin was auto, manual, or denied under the
   current envelope.

The freeze matters now, ahead of the live tour, walkthrough,
learning-mode, classroom, and presentation-mode pipelines, so every
later surface can read **the same** packet shape, **the same**
tour-citation / cached-pack-continuity / presentation-role /
layout-restore / classroom-artifact / export-redaction vocabulary,
and **the same** audience-redaction pairing rules instead of
inventing per-window equivalents.

## Frozen vocabulary

This contract introduces the following frozen vocabularies. Each is
owned by
`schemas/learning/learning_presentation_packet.schema.json`;
downstream surfaces re-export by reference and never mint a parallel
value.

### Window kinds (frozen, seven values)

`window_kind_class`: `milestone_close_window`,
`release_train_window`, `weekly_governance_review_window`,
`ad_hoc_review_window`, `support_handoff_window`,
`accessibility_audit_window`, `docs_pack_review_window`. A packet
whose window cannot be typed denies with
`window_kind_class_unresolved` rather than collapsing to
`ad_hoc_review_window`.

### Audience and redaction profile (frozen, paired)

`audience_class` (six values): `engineering_internal`,
`support_handoff`, `enterprise_audit`, `release_readiness`,
`accessibility_audit`, `public_proof_safe`.

`redaction_profile_class` (six values):
`engineering_internal_only`, `support_handoff_redacted`,
`enterprise_audit_redacted`, `release_readiness_summary`,
`accessibility_audit_redacted`, `public_proof_safe_zero_payload`.

The schema gates pair every audience to exactly one redaction
profile. A mismatched pairing denies with
`redaction_profile_audience_pairing_mismatch`. The pairing rule:

| `audience_class`        | `redaction_profile_class`            |
|-------------------------|--------------------------------------|
| `engineering_internal`  | `engineering_internal_only`          |
| `support_handoff`       | `support_handoff_redacted`           |
| `enterprise_audit`      | `enterprise_audit_redacted`          |
| `release_readiness`     | `release_readiness_summary`          |
| `accessibility_audit`   | `accessibility_audit_redacted`       |
| `public_proof_safe`     | `public_proof_safe_zero_payload`     |

### Surface coverage (frozen, thirteen values)

`surface_coverage_kind_class`: `covers_onboarding_card_surface`,
`covers_glossary_card_surface`, `covers_guided_tour_step_surface`,
`covers_architecture_explainer_surface`,
`covers_architecture_map_surface`, `covers_contextual_tip_surface`,
`covers_keymap_bridge_hint_surface`, `covers_exercise_step_surface`,
`covers_command_tip_pack_surface`,
`covers_learning_mode_profile_surface`,
`covers_speaker_note_adjunct_surface`,
`covers_teaching_session_adjunct_surface`,
`covers_presentation_session_surface`. A packet that covers no
surface denies with `surface_coverage_empty`.

### Tour-citation drift (frozen, seven values)

`tour_citation_drift_class`: `citation_resolved_exact`,
`citation_anchor_remapped_disclosed`,
`command_id_remapped_disclosed`,
`docs_pack_revision_drifted_disclosed`,
`symbol_linked_reference_remapped_disclosed`,
`keymap_bridge_canonical_id_drifted_disclosed`,
`citation_anchor_unresolvable`. Each entry MUST cite a
`waypoint_id_ref`, the `anchor_kind` (mirroring
`waypoint_anchor_kind` from
`schemas/ux/guided_tour.schema.json`), and the resolved canonical
ref the audit walked back to (or null if the anchor is
unresolvable).

### Cached-pack continuity (frozen, eight values)

`cached_pack_continuity_state_class`:
`pack_live_authoritative_full_install`,
`pack_not_installed_suppressed`, `pack_cached_within_ttl`,
`pack_mirrored_disclosed`, `pack_stale_past_ttl_disclosed`,
`pack_offline_bundle_verified`,
`pack_offline_bundle_unverified_suppressed`,
`pack_revoked_or_withdrawn_suppressed`. Each entry MUST cite a
`pack_kind` (`docs_pack`, `glossary_pack`, `command_tip_pack`, or
`imported_teaching_pack`), the `pack_ref`, the
`pack_revision_ref`, and the observed `freshness_class` from the
citation-anchor object model.

### Presentation-role review (frozen, ten values)

`presentation_role_review_state_class`:
`presenter_role_bound_to_actor`,
`co_presenter_role_bound_to_actor`,
`observer_role_bound_to_actor`, `audience_role_bound_to_pool`,
`presenter_handoff_recorded`,
`breakaway_event_recorded_with_recovery`,
`follow_target_resolved_against_collaboration_record`,
`share_state_matches_envelope`,
`presenter_bar_audience_invisible_confirmed`,
`speaker_note_locality_audience_invisible_confirmed`. Each entry
MAY cite a `presentation_session_id_ref`,
`presenter_state_id_ref`, `follow_target_id_ref`,
`presenter_handoff_id_ref`, `breakaway_event_id_ref`, and
`control_grant_id_ref` so the role binding lineage is reviewable.

### Layout-restore proof (frozen, eight values)

`layout_restore_proof_class`:
`every_layout_anchor_lands_on_canonical_workspace_surface`,
`presenter_bar_anchor_canonical`,
`agenda_or_waypoint_rail_anchor_canonical`,
`spotlight_frame_anchor_canonical`,
`breakaway_banner_anchor_canonical`,
`speaker_note_locality_anchor_canonical`,
`no_private_presentation_shell_target_minted`,
`layout_target_class_in_canonical_set`. Each entry MAY cite a
`presentation_layout_anchor_id_ref` plus the observed
`layout_target_class` and
`local_remote_shared_boundary_class` so the audit can be traced
back to the underlying anchor row.

### Accessibility / keyboard coverage (frozen, six values)

`accessibility_keyboard_coverage_class`:
`respects_user_reduced_motion_setting_confirmed`,
`forced_reduced_motion_classroom_default_disclosed`,
`forced_reduced_motion_policy_locked_disclosed`,
`full_keyboard_route_required_confirmed`,
`screen_reader_announcements_required_confirmed`,
`high_contrast_focus_required_confirmed`. Each entry MAY cite the
`presentation_session_id_ref` or
`guided_surface_state_id_ref` it was confirmed against.

### Classroom-artifact retention (frozen, five values)

`classroom_artifact_retention_class`:
`no_classroom_artifact_retained`,
`metadata_only_local_redacted`,
`metadata_only_in_support_bundle_redacted`,
`classroom_artifact_privacy_bounded_disclosed`,
`replay_blocked_by_policy_disclosed`. Each entry MAY cite the
`presentation_session_id_ref` and the
`replay_retention_class` re-projected from
`schemas/ux/presentation_mode_state.schema.json` so the audit
preserves the upstream retention class without redeclaring it.

### Export-redaction posture (frozen, fourteen values)

`export_redaction_posture_class`:
`no_raw_speaker_notes_cross_boundary`,
`no_raw_audience_chat_or_qna_in_packet`,
`audience_identity_never_leaked`,
`imported_pack_verification_state_preserved`,
`cached_pack_continuity_state_preserved`,
`presenter_bar_audience_invisible_preserved`,
`speaker_note_locality_preserved`,
`replay_retention_class_preserved`,
`breakaway_recovery_class_preserved`,
`accessibility_keyboard_state_preserved`,
`reduced_motion_state_preserved`,
`policy_locked_dismissal_carries_policy_epoch`,
`support_export_redacts_raw_pack_paths`,
`public_proof_export_carries_zero_raw_payload`.

The schema enforces a five-class export-redaction floor: every
packet MUST carry at minimum
`no_raw_speaker_notes_cross_boundary`,
`audience_identity_never_leaked`,
`imported_pack_verification_state_preserved`,
`cached_pack_continuity_state_preserved`, and
`replay_retention_class_preserved`. Missing any of those denies
with `export_redaction_posture_minimum_floor_unmet`.

### Linked artifact families (frozen, fourteen values)

`linked_artifact_family_class`: `learnability_contract`,
`learning_and_presentation_contract`,
`guided_surface_state_schema`, `guided_tour_schema`,
`learning_progress_schema`, `presentation_mode_state_schema`,
`command_descriptor_contract`,
`command_registry_entry_schema`, `citation_anchor_object_model`,
`symbol_linked_reference_schema`,
`follow_and_presenter_state_schema`, `control_grant_schema`,
`transient_surface_schema`, `keybinding_resolver_schema`.

The schema enforces a five-class linked-artifact floor: every
packet MUST cite at minimum `learnability_contract`,
`learning_and_presentation_contract`,
`guided_surface_state_schema`,
`command_descriptor_contract`, and
`citation_anchor_object_model`. Missing any denies with
`linked_artifact_family_minimum_floor_unmet`.

### Change significance (frozen, four values)

`change_significance_class`: `informational`,
`release_bearing`, `claim_narrowing`, `claim_widening_blocked`.

`claim_narrowing` packets MUST cite
`claim_manifest_row_id_refs`. `claim_widening_blocked` packets
MUST cite at least one `blocking_caveat_classes` entry drawn from
either `tour_citation_drift_class`,
`cached_pack_continuity_state_class`, or
`classroom_artifact_retention_class`.

### Consuming-surface parity floor (frozen, nine values)

`consuming_surface_parity_floor_class`:
`guided_tour_parity_required`, `glossary_card_parity_required`,
`architecture_map_parity_required`,
`command_tip_pack_parity_required`,
`learning_mode_parity_required`,
`presentation_mode_parity_required`,
`release_evidence_parity_required`,
`support_handoff_parity_required`,
`accessibility_audit_parity_required`. A surface that diverges
from the floor is non-conforming and denies with
`consuming_surface_parity_floor_unmet`.

### Denial reasons (frozen)

`learning_presentation_packet_unknown`,
`audience_class_unresolved`,
`redaction_profile_class_unresolved`,
`redaction_profile_audience_pairing_mismatch`,
`window_kind_class_unresolved`,
`surface_coverage_empty`,
`surface_coverage_kind_unresolved`,
`tour_citation_audit_empty`,
`tour_citation_drift_class_unresolved`,
`cached_pack_continuity_empty`,
`cached_pack_continuity_state_class_unresolved`,
`presentation_role_review_empty`,
`presentation_role_review_state_class_unresolved`,
`layout_restore_proof_empty`,
`layout_restore_proof_class_unresolved`,
`accessibility_keyboard_coverage_empty`,
`accessibility_keyboard_coverage_class_unresolved`,
`classroom_artifact_retention_class_unresolved`,
`export_redaction_posture_minimum_floor_unmet`,
`linked_artifact_family_minimum_floor_unmet`,
`change_significance_class_unresolved`,
`claim_narrowing_requires_claim_manifest_row_refs`,
`claim_widening_blocked_requires_blocking_caveat`,
`consuming_surface_parity_floor_unmet`,
`guided_tour_schema_version_pin_missing`,
`learning_progress_schema_version_pin_missing`,
`presentation_mode_state_schema_version_pin_missing`,
`guided_surface_state_schema_version_pin_missing`,
`raw_body_forbidden_on_boundary`,
`learning_presentation_packet_schema_version_lagging`,
`policy_epoch_expired`, `policy_blocked`.

### Audit event ids (frozen, seven values)

`learning_presentation_packet_audit_event_id`:
`learning_presentation_packet_minted`,
`learning_presentation_packet_updated`,
`learning_presentation_packet_frozen_for_handoff`,
`learning_presentation_packet_superseded`,
`learning_presentation_packet_withdrawn`,
`learning_presentation_packet_audit_denial_emitted`,
`learning_presentation_packet_schema_version_bumped`.

## Truthfulness posture (normative)

Every rule below is normative. A packet that violates any of them is
non-conforming regardless of how the violation is painted.

1. **One packet, one window, one audience-redaction pairing.** A
   packet pins exactly one `window_kind_class`, one
   `audience_class`, and exactly one paired
   `redaction_profile_class`. A mismatched pairing denies with
   `redaction_profile_audience_pairing_mismatch`.
2. **Schema-version pins are mandatory.** Every packet carries the
   four schema-version pins above. A packet missing any pin denies
   with the matching `*_schema_version_pin_missing` reason.
3. **Surface coverage cannot be empty.** Every packet pins at least
   one `surface_coverage_kind_class` and at least one fixture ref.
   An empty surface-coverage matrix denies with
   `surface_coverage_empty`.
4. **Tour-citation audit is mandatory.** Every packet that covers
   `covers_guided_tour_step_surface`,
   `covers_architecture_map_surface`,
   `covers_command_tip_pack_surface`, or
   `covers_glossary_card_surface` MUST emit a non-empty
   tour-citation audit. Missing audit denies with
   `tour_citation_audit_empty`.
5. **Cached-pack continuity is mandatory.** Every packet declares
   the cached-pack continuity state for every audited
   `docs_pack_ref`, `glossary_pack_ref`, `command_tip_pack_ref`,
   and `imported_teaching_pack_ref`. Empty denies with
   `cached_pack_continuity_empty`.
6. **Presentation-role review is mandatory whenever a session is
   covered.** Every packet that covers
   `covers_presentation_session_surface` MUST emit a non-empty
   presentation-role review. Empty denies with
   `presentation_role_review_empty`.
7. **Layout-restore proof is mandatory whenever a session is
   covered.** Every packet that covers
   `covers_presentation_session_surface` MUST emit a non-empty
   layout-restore proof. Empty denies with
   `layout_restore_proof_empty`.
8. **Accessibility/keyboard coverage is mandatory.** Every packet
   declares at least one
   `accessibility_keyboard_coverage_class`. Empty denies with
   `accessibility_keyboard_coverage_empty`. Accessibility-audit
   windows MUST cite at least one
   `respects_user_reduced_motion_setting_confirmed` (or its
   forced-with-disclosure variants) plus
   `full_keyboard_route_required_confirmed`.
9. **Export redaction posture floor is enforced.** Every packet
   carries at minimum the five export-redaction floor classes.
   Missing any denies with
   `export_redaction_posture_minimum_floor_unmet`.
10. **Linked artifact floor is enforced.** Every packet cites the
    five linked-artifact-family floor classes. Missing any denies
    with `linked_artifact_family_minimum_floor_unmet`.
11. **Change significance pairs to its evidence.** A
    `claim_narrowing` packet cites at least one
    `claim_manifest_row_id_refs` entry. A
    `claim_widening_blocked` packet cites at least one
    `blocking_caveat_classes` entry.
12. **Frozen / superseded / withdrawn packets MUST set
    `frozen_at`.** A packet that asserts a `superseded_at` or
    `withdrawn_at` timestamp without `frozen_at` denies via the
    schema gates.
13. **No raw payloads cross the boundary.** Raw speaker-note
    bodies, raw audience chat or Q&A bodies, raw audience identity,
    raw glossary or docs pack bodies, raw URLs, raw absolute
    paths, raw imported teaching pack payloads, and raw secrets
    MUST NOT appear. A row that carries a raw payload denies with
    `raw_body_forbidden_on_boundary`.
14. **Captured tours and sessions are replay material, not current
    truth.** Tour-citation audits, cached-pack continuity entries,
    presentation-role review entries, and layout-restore proof
    entries record the state at the audit window; they do not
    assert that the underlying surface is still live or that the
    cited pack revision is still installed on every device.
    Reviewers reading a frozen packet read the captured truth, not
    the current truth.

## Audience-redaction pairing (normative)

The audience-redaction pairing table above is the only admitted
pairing. A reviewer reading a packet whose audience is
`accessibility_audit` MUST see `redaction_profile_class =
accessibility_audit_redacted` and MUST NOT see
`engineering_internal_only`. The pairing is enforced by the schema
gates; an exporter MAY NOT bypass the pairing by minting an
out-of-band field.

A packet narrowed for an audience whose pairing the source packet
does not admit MUST be re-minted under a new packet id (the
`supersedes_packet_id_ref` field carries the chain). The original
packet is not edited in-place to change audience.

## Tour-citation audit rules (normative)

Each `tour_citation_audit_entry` ties one drift class to the cited
waypoint, the anchor kind, and the canonical ref the audit walked
back to.

- `citation_resolved_exact` cites the
  `waypoint_id_ref`, the `anchor_kind`, and the
  `resolved_canonical_ref` (a `command_id`, a
  `docs_citation_anchor_id`, a `symbol_linked_reference_id`, or a
  `keybinding_resolver_id`). The waypoint citation walks back to
  exactly the canonical anchor it was minted against.
- `citation_anchor_remapped_disclosed` records that the docs
  citation anchor was remapped by the citation contract's
  derivation rule and the upstream anchors still resolve.
- `command_id_remapped_disclosed` records that the canonical
  `command_id` on the registry was renamed-via-alias-with-lifecycle;
  the audit cites the new id and the alias lifecycle.
- `docs_pack_revision_drifted_disclosed` records that the docs
  pack revision the waypoint cites is no longer the latest
  revision; the audit cites the captured revision and the latest
  revision.
- `symbol_linked_reference_remapped_disclosed` records that the
  symbol-linked reference id was remapped under a renamed-refactor
  with a typed remap chain.
- `keymap_bridge_canonical_id_drifted_disclosed` records that the
  keymap bridge no longer resolves to the canonical command id and
  the bridge MUST suppress with `keymap_bridge_unresolved` until
  the canonical id is restored.
- `citation_anchor_unresolvable` records that no canonical anchor
  resolves; the waypoint MUST suppress (`active = false`) with
  `command_id_missing` or
  `command_id_deprecated_without_replacement` per the learnability
  contract.

The packet does NOT redeclare the citation derivation rule, the
command-registry alias lifecycle, or the symbol-linked-reference
remap state machine. It re-exports the closed enum surface where a
typed pivot is needed and cites the canonical owners by id.

## Cached-pack continuity rules (normative)

Each `cached_pack_continuity_entry` ties one continuity state to a
cited pack ref and a captured pack revision.

- `pack_live_authoritative_full_install` is the default state for
  a docs / glossary / command-tip / imported-teaching pack that is
  installed locally and matches the running build's pinned revision.
- `pack_not_installed_suppressed` records that the pack is not
  installed; every guided surface that cites it MUST suppress with
  `docs_pack_missing` per the learnability contract. The audit
  counts the suppressed surfaces and does not silently render any
  of them as live.
- `pack_cached_within_ttl` records a cache that is still inside the
  TTL window for the pack family.
- `pack_mirrored_disclosed` records an air-gapped or managed-cloud
  envelope in which the pack is mirrored from the upstream rather
  than fetched live; the surface MAY render with the mirror posture
  disclosed.
- `pack_stale_past_ttl_disclosed` records a pack that has aged past
  its TTL; the audit names the captured TTL and the elapsed
  duration class so a reviewer can audit the staleness without the
  packet leaking the raw timestamp.
- `pack_offline_bundle_verified` records an imported offline bundle
  whose verification (signature, manifest, checksum) succeeded.
- `pack_offline_bundle_unverified_suppressed` records an imported
  offline bundle whose verification failed; every guided surface
  that cites it MUST suppress with
  `imported_teaching_pack_unavailable` per the learnability
  contract and `guided_tour_imported_pack_unverified` per the
  learning-and-presentation contract.
- `pack_revoked_or_withdrawn_suppressed` records a pack that has
  been revoked by the owning authority or withdrawn for cause; the
  audit captures the revocation reason class without leaking the
  revocation message body.

## Presentation-role review rules (normative)

Each `presentation_role_review_entry` ties one role-review state
class to the cited session, role binding, follow target, presenter
handoff, breakaway event, or control grant row.

- `presenter_role_bound_to_actor`,
  `co_presenter_role_bound_to_actor`,
  `observer_role_bound_to_actor`, and
  `audience_role_bound_to_pool` cite the
  `presentation_session_id_ref` and the
  `presenter_state_id_ref` (and, for audience pools, the bound
  pool id from the share-state envelope) so a later facilitator
  review can audit who held what role under what envelope.
- `presenter_handoff_recorded` cites the
  `presenter_handoff_id_ref` from
  `schemas/collaboration/follow_and_presenter_state.schema.json`.
  The packet does NOT mint a parallel handoff record.
- `breakaway_event_recorded_with_recovery` cites the
  `breakaway_event_id_ref` and the observed
  `breakaway_recovery_class` re-projected from
  `schemas/ux/presentation_mode_state.schema.json`.
- `follow_target_resolved_against_collaboration_record` cites the
  `follow_target_id_ref` so the audit confirms the follow
  relationship reads through the
  `follow_target_record` lane on the collaboration schema.
- `share_state_matches_envelope` confirms the audited
  `share_state_class` is admissible under the captured deployment
  profile and policy bundle (e.g.
  `shared_to_classroom_managed_pool` is admissible only on
  `enterprise_online` or `managed_cloud` profiles per the
  learning-and-presentation contract).
- `presenter_bar_audience_invisible_confirmed` and
  `speaker_note_locality_audience_invisible_confirmed` confirm the
  presenter bar and speaker notes were never rendered to the
  audience in the audited session, mirroring invariants 2 and 3
  on the presentation invariant manifest.

Mutating control rides a `control_grant_record` and is never
inferable from a presenter or follow row; the role review entry
MAY cite a `control_grant_id_ref` when a session paired a presenter
role with a separate control grant for screen-share / co-edit.

## Layout-restore proof rules (normative)

Each `layout_restore_proof_entry` ties one layout-restore proof
class to the cited layout anchor row, the observed
`layout_target_class`, and the observed
`local_remote_shared_boundary_class`.

- `every_layout_anchor_lands_on_canonical_workspace_surface` is the
  aggregate proof; the entry cites every audited
  `presentation_layout_anchor_id_ref`.
- `presenter_bar_anchor_canonical`,
  `agenda_or_waypoint_rail_anchor_canonical`,
  `spotlight_frame_anchor_canonical`,
  `breakaway_banner_anchor_canonical`, and
  `speaker_note_locality_anchor_canonical` are the per-anchor
  proofs.
- `no_private_presentation_shell_target_minted` confirms no anchor
  resolved against a private "presentation shell" target. A packet
  that carries a layout anchor whose target is outside the closed
  `layout_target_class` set denies via the schema gates with
  `presentation_session_layout_anchor_not_canonical_surface`.
- `layout_target_class_in_canonical_set` is the simplest proof: the
  observed target MUST be drawn from the closed
  `layout_target_class` set on
  `schemas/ux/guided_tour.schema.json` (Start Center, command
  palette, workspace switcher, primary editor, docs pane,
  architecture map canvas, review pane, activity center, support
  center, settings pane, no-canvas-target facilitator-only).

## Accessibility / keyboard coverage rules (normative)

Each `accessibility_keyboard_coverage_entry` ties one coverage
class to the cited surface or session row.

- `respects_user_reduced_motion_setting_confirmed` confirms the
  surface or session honours the user's reduced-motion setting per
  the design-system style guide.
- `forced_reduced_motion_classroom_default_disclosed` and
  `forced_reduced_motion_policy_locked_disclosed` record forced
  overrides; the override MUST be disclosed per
  `presentation_session_reduced_motion_overridden_without_disclosure`
  on the learning-and-presentation contract.
- `full_keyboard_route_required_confirmed` confirms a complete
  keyboard route to and through every surface kind the packet
  exercised.
- `screen_reader_announcements_required_confirmed` and
  `high_contrast_focus_required_confirmed` confirm screen-reader
  and focus posture per the accessibility/keyboard class on
  `schemas/ux/presentation_mode_state.schema.json`.

Accessibility-audit windows MUST cite at least one reduced-motion
class plus `full_keyboard_route_required_confirmed`. Other window
kinds MAY cite a smaller subset but MUST NOT be empty.

## Classroom-artifact retention rules (normative)

Each `classroom_artifact_retention_entry` ties one retention class
to the cited session row and the upstream
`replay_retention_class`.

- `no_classroom_artifact_retained` is the local-only-no-share or
  classroom-disabled posture.
- `metadata_only_local_redacted` and
  `metadata_only_in_support_bundle_redacted` mirror the
  metadata-only replay classes on
  `schemas/ux/presentation_mode_state.schema.json` (local-only or
  in-support-bundle, redacted in either case).
- `classroom_artifact_privacy_bounded_disclosed` is the explicit
  privacy-bounded class; the entry MUST cite the privacy-bound
  policy and MUST NOT leak audience identity.
- `replay_blocked_by_policy_disclosed` records the policy block;
  the entry cites the `policy_epoch` from the packet's
  `policy_context`.

The classroom-artifact retention aggregate is the export-safe view
of what audience-visible artifacts the audited sessions retained;
the entry counts retained artifact classes without quoting any
artifact body.

## Export redaction posture rules (normative)

The export redaction posture is the packet's commitment to the
learning / presentation export pipelines. It composes with — does
not replace — the learnability and learning-and-presentation
redaction posture:

- `no_raw_speaker_notes_cross_boundary` mirrors the
  `speaker_note_locality_class` discipline at
  `schemas/ux/presentation_mode_state.schema.json`.
- `no_raw_audience_chat_or_qna_in_packet` confirms that audience
  chat / Q&A bodies (when implemented in later milestones) are not
  carried on this packet; the packet stores opaque ids only.
- `audience_identity_never_leaked` mirrors the
  classroom-artifact privacy-bounded rule: audience identity is
  recorded as an opaque pool id or named-invitee id, never as a
  raw display name or email.
- `imported_pack_verification_state_preserved` mirrors the
  `pack_offline_bundle_verified` /
  `pack_offline_bundle_unverified_suppressed` rule: every export
  preserves the verification state.
- `cached_pack_continuity_state_preserved` mirrors the cached-pack
  continuity rule: every export preserves the per-pack
  continuity state.
- `presenter_bar_audience_invisible_preserved`,
  `speaker_note_locality_preserved`,
  `replay_retention_class_preserved`, and
  `breakaway_recovery_class_preserved` mirror the corresponding
  invariants on the presentation invariant manifest.
- `accessibility_keyboard_state_preserved` and
  `reduced_motion_state_preserved` mirror the accessibility and
  reduced-motion posture: every export preserves the audited
  state class.
- `policy_locked_dismissal_carries_policy_epoch` mirrors the
  learning-progress policy-locked rule: every export preserves the
  `policy_epoch` for policy-locked progress rows.
- `support_export_redacts_raw_pack_paths` and
  `public_proof_export_carries_zero_raw_payload` are the
  audience-specific guarantees applied on top of the floor.

## Linked artifact families (normative)

The five-class linked-artifact floor (`learnability_contract`,
`learning_and_presentation_contract`,
`guided_surface_state_schema`, `command_descriptor_contract`,
`citation_anchor_object_model`) is the minimum every packet cites.
A packet MAY cite any of the additional nine families but MUST NOT
omit any of the five-floor families. The floor is enforced via
contains-floor `allOf` gates on the schema; an out-of-band field
MAY NOT be used to bypass the floor.

## Change significance (normative)

The change-significance class describes what the packet asserts
about the wider claim surface:

- `informational` packets establish a baseline for later windows to
  compare against; they MUST NOT cite a claim manifest row.
- `release_bearing` packets back release-notes claims about the
  audited surfaces; they cite at least one claim manifest row by id.
- `claim_narrowing` packets pull a public claim back; they MUST
  cite at least one `claim_manifest_row_id_refs` entry.
- `claim_widening_blocked` packets are held by an open caveat; they
  MUST cite at least one `blocking_caveat_classes` entry drawn
  from `tour_citation_drift_class`,
  `cached_pack_continuity_state_class`, or
  `classroom_artifact_retention_class`.

## Consuming-surface parity floor (normative)

Every packet declares at least one parity-floor class. A packet
that asserts release-bearing significance MUST cite at minimum
`release_evidence_parity_required`; a packet under an
`accessibility_audit_window` MUST cite at minimum
`accessibility_audit_parity_required`. A surface that diverges
from the cited parity floor is non-conforming and the audit emits
`consuming_surface_parity_floor_unmet`.

## How later release evidence summarises this packet

Later release-evidence packets, accessibility-audit reports,
docs-pack review reports, support-handoff bundles, and public-proof
indexes do **not** mint a second packet family for tour-citation
audits, cached-pack continuity, presentation-role review, layout-
restore proof, or classroom-artifact retention. They link the
matching `learning_presentation_evidence_packet_record` by
`learning_presentation_packet_id` and re-export the typed
section(s) they need. The export-redaction posture and audience-
redaction pairing rules guarantee the linked packet survives
audience narrowing without leaking speaker notes, audience identity,
imported pack bodies, or raw pack paths.

## Adding a new vocabulary value

Adding a new `window_kind_class`, `audience_class`,
`redaction_profile_class`, `surface_coverage_kind_class`,
`tour_citation_drift_class`,
`cached_pack_continuity_state_class`,
`presentation_role_review_state_class`,
`layout_restore_proof_class`,
`accessibility_keyboard_coverage_class`,
`classroom_artifact_retention_class`,
`export_redaction_posture_class`,
`linked_artifact_family_class`, `change_significance_class`,
`consuming_surface_parity_floor_class`,
`learning_presentation_packet_audit_event_id`, or `denial_reason`
is **additive-minor** and bumps
`learning_presentation_packet_schema_version`. Repurposing an
existing value is **breaking** and requires a new decision row on
the launch decision register. A consumer surface that resolves a
value it does not recognize MUST deny with
`learning_presentation_packet_schema_version_lagging` rather than
silently map to a default.

## Acceptance mapping

| Acceptance clause                                                                                                                         | Resolved by                                                                                                                                                                                |
|-------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Guided surfaces can prove where their facts and actions come from rather than relying on hand-authored copy alone.                        | §"Tour-citation audit", §"Surface coverage", §"Linked artifact families" floor (learnability + learning-and-presentation + command-descriptor + citation-anchor).                          |
| Presentation-mode claims can show restore continuity, role boundaries, and cached/offline behavior with explicit evidence.                | §"Presentation-role review", §"Layout-restore proof", §"Cached-pack continuity", §"Classroom-artifact retention".                                                                          |
| The packet is reusable in release, docs, accessibility, and support reviews without changing terminology.                                 | §"Window kinds" (seven values), §"Audience and redaction profile" (six paired classes including accessibility_audit), §"Consuming-surface parity floor", §"How later release evidence summarises this packet". |
| Tutorial and presentation packets can explain source class, citation set, and mirror/offline behavior without separate bespoke vocabulary.| §"Tour-citation audit" re-uses citation-anchor + command-registry vocabularies; §"Cached-pack continuity" re-uses freshness_class + docs_pack_revision_ref; no parallel pack-status lane.  |
| Guided surfaces, glossary packs, architecture maps, and tour steps must use the shared knowledge-pack and learning-tour matrix.           | §"Surface coverage" (re-exports `guided_surface_kind`); §"Cached-pack continuity" (re-exports `freshness_class`); §"Tour-citation audit" (re-exports `waypoint_anchor_kind`).                |

## Worked examples

Fixtures under
[`/fixtures/learning/learning_presentation_cases/`](../../fixtures/learning/learning_presentation_cases/)
cover:

1. **Milestone-close informational baseline** —
   `milestone_close_informational.yaml`. A
   `learning_presentation_evidence_packet_record` whose window is
   `milestone_close_window`, audience `engineering_internal`,
   audits onboarding cards, glossary cards, guided-tour steps, and
   the individual learner learning-mode profile under a
   `live_authoritative` baseline; informational significance.
2. **Accessibility-audit reduced-motion + keyboard reachability** —
   `accessibility_audit_reduced_motion.yaml`. A packet under
   `accessibility_audit_window` /
   `accessibility_audit_redacted` audience, asserting
   `respects_user_reduced_motion_setting_confirmed` and
   `full_keyboard_route_required_confirmed` across guided-tour
   steps, presentation sessions (with shared follow disabled), and
   architecture map.
3. **Release-train parity assertion** —
   `release_train_parity_assertion.yaml`. A
   `release_train_window` / `release_readiness` packet asserting
   parity across every consuming surface (guided-tour, glossary
   card, architecture map, command-tip pack, learning mode,
   presentation mode, release evidence, support handoff,
   accessibility audit) with one `presenter_handoff_recorded` plus
   one `breakaway_event_recorded_with_recovery` entry.
4. **Claim-narrowing imported teaching pack unverified** —
   `claim_narrowing_imported_pack_unverified.yaml`. A packet that
   pulls a public claim back because the imported teaching pack
   used by the classroom-managed-pool tour has failed verification;
   `change_significance_class = claim_narrowing` with
   `pack_offline_bundle_unverified_suppressed`.
5. **Claim-widening-blocked docs-pack mirror-only + breakaway** —
   `claim_widening_blocked_pack_mirrored.yaml`. A packet held by
   `pack_mirrored_disclosed` plus a
   `breakaway_event_recorded_with_recovery` whose recovery class is
   `manual_rejoin_invite_required`;
   `change_significance_class = claim_widening_blocked` with
   blocking caveats cited.
6. **Audit denial export-redaction floor unmet** —
   `audit_denial_export_redaction_floor_unmet.yaml`. A draft
   packet whose export-redaction posture missed two of the five
   floor classes; the audit emits
   `learning_presentation_packet_audit_denial_emitted` with
   `export_redaction_posture_minimum_floor_unmet`.

## Out of scope at this revision

- Shipping a complete learning mode, classroom managed-pool
  workflow, presentation mode, or accessibility-audit tooling. This
  contract freezes the audit-evidence boundary; the actual tour
  content, walkthrough library, classroom enrollment, presentation
  UI, post-session review, and accessibility audit harness land in
  later milestones and ride this contract.
- Final visuals (presenter bar layout, agenda rail density, audit
  report styling). The design-system style guide owns those.
- Analytics and scoreboarding beyond the existing measurement-
  surface linkage on the learnability contract; the onboarding-
  measurement plan owns the qualification event names.
- Actual policy-bundle definitions that disable share state,
  replay retention, learning surfaces, or accessibility overrides.
  The identity / policy-bundle contracts own those; this packet
  names the typed suppression cause and preserves the
  `policy_epoch` on every export.
