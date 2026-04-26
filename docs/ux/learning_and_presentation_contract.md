# Onboarding, learning-mode, guided-tour, architecture-map, and presentation-mode contract

This document freezes the cross-surface contract every Aureline
**first-run onboarding flow**, **learning-mode profile**, **guided
tour or architecture map**, **glossary card**, **exercise step**,
**command tip**, **walkthrough**, and **presentation-mode session**
resolves through before the shell starts rendering teaching or
presentation content. The goal is a discoverability and teaching
surface that stays a thin layer over canonical workspace surfaces —
so a reviewer can map a guided tip, a tour waypoint, a glossary card,
and a presentation step back to the same command and citation truth
every other row reads, with no special-case private execution path
and no separate privileged shell.

The machine-readable schemas live at:

- [`/schemas/ux/guided_tour.schema.json`](../../schemas/ux/guided_tour.schema.json)
- [`/schemas/ux/learning_progress.schema.json`](../../schemas/ux/learning_progress.schema.json)
- [`/schemas/ux/presentation_mode_state.schema.json`](../../schemas/ux/presentation_mode_state.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/learning_cases/`](../../fixtures/ux/learning_cases/)

This contract is normative for the audience-class admissibility,
indexing-readiness disclosure, waypoint citation, layout-anchor
locality, share-state envelope, follow-state binding, replay-retention
posture, breakaway recovery, and reduced-motion / accessibility
posture of onboarding, learning-mode, guided-tour, architecture-map,
and presentation-mode surfaces. Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, or milestone document, those sources win and this
document plus its companion schemas and fixtures update in the same
change. Where a downstream tour, walkthrough, classroom, or
presentation surface mints a parallel vocabulary, this contract wins
and the surface is non-conforming.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference:

- [`/docs/ux/learnability_contract.md`](./learnability_contract.md)
  and
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  — `guided_surface_state_record`, `guided_surface_rule_record`,
  `learning_mode_profile_record`, the closed `guided_surface_kind`
  enum, the `guidance_authority_class` and `suppression_cause_class`
  vocabularies, and the twelve const-true invariants on the
  learnability manifest. Tour waypoints, walkthrough steps, and
  presentation adjuncts emit one `guided_surface_state_record` per
  step and resolve against the kind's rule row; this contract binds
  the multi-step / sequenced / shared shape on top.
- [`/docs/ux/no_account_local_entry_contract.md`](./no_account_local_entry_contract.md)
  and
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)
  — `entry_surface_family`, `account_prompt_class`,
  `boundary_crossing_class`, `state_portability_class`, `reset_class`,
  `export_class`, `profile_scope_class`, and the onboarding-portability
  manifest. First-run no-account onboarding tours, learning-mode
  progress, dismissal, and "don't show again" state ride that
  contract's portability table; this contract mints no parallel lane.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md),
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json),
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  — canonical `command_id`, alias lifecycle, keybinding refs.
  Every primary-action button on an onboarding card, every inline
  action on a tour waypoint, every "apply suggestion" control on an
  exercise step, and every command-tip pack row resolves through a
  stable `command_id` on the registry. No private mutation path
  outside the registry is admissible.
- [`/docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md),
  [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json),
  and
  [`/schemas/docs/symbol_linked_reference.schema.json`](../../schemas/docs/symbol_linked_reference.schema.json)
  — `docs_citation_anchor_record` (including `glossary_term_anchor`,
  `onboarding_step_anchor`), `derivation`, `source_class`,
  `freshness_class`, `version_match_state`, `reuse_surface`, and
  `export_posture`. Every waypoint that paraphrases or summarizes a
  pack body retains its upstream anchor; an architecture-map node
  that claims to teach a structural concept resolves through a
  symbol-linked reference rather than a private graph node payload.
- [`/docs/ux/localization_and_locale_pack_contract.md`](./localization_and_locale_pack_contract.md)
  — locale-pack manifest, source-language fallback. Glossary cards,
  waypoints, and command tips that fall back from the requested
  locale MUST disclose the fallback; silent locale degradation is
  non-conforming.
- [`/docs/ux/keybinding_resolver_contract.md`](./keybinding_resolver_contract.md)
  and
  [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
  — keymap-bridge resolver. A tour waypoint that surfaces a keymap
  bridge tip composes with this resolver and lands on a canonical
  command id; a tip that lands on a non-canonical verb suppresses.
- [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md),
  [`/schemas/collaboration/follow_and_presenter_state.schema.json`](../../schemas/collaboration/follow_and_presenter_state.schema.json),
  and
  [`/schemas/collaboration/control_grant.schema.json`](../../schemas/collaboration/control_grant.schema.json)
  — `follow_target_record`, `presenter_state_record`,
  `presenter_handoff_record`, `control_grant_record`. Presentation-
  mode share state, follow grant, presenter handoff, and audience
  breakaway resolve through these collaboration records;
  presentation-mode authority MUST NOT confer mutation, and shared
  terminal / shared debug control is never inferable from a
  presenter or follow row.
- [`/docs/ux/transient_surface_contract.md`](./transient_surface_contract.md)
  and
  [`/schemas/ux/transient_surface.schema.json`](../../schemas/ux/transient_surface.schema.json)
  — transient preview primitives. A waypoint or command tip that
  renders as a tooltip, hovercard, popover, or peek panel composes
  by reference with a `transient_preview_record`; this contract
  binds only the multi-step / agenda / share-state shape on top.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary so air-gapped, managed-cloud,
  and restricted envelopes inherit suppression posture mechanically.

## Who reads this contract

- **Onboarding, tour, walkthrough, glossary, and command-tip
  authors** — to emit multi-step teaching content against one tour
  / waypoint / command-tip-pack object model whose audience class,
  indexing-readiness disclosure, locale fallback, evidence-hook
  reserve, and deployment-profile envelope are declared.
- **Architecture-map authors** — to project subsystem maps as
  symbol-anchored canvas nodes rather than private graph payloads,
  so a reviewer can trace a node back to a `symbol_linked_reference`.
- **Learning-mode and classroom authors** — to declare progress,
  dismissal, "don't show again", reset, and export posture against
  the onboarding-portability lanes; no teaching-only progress lane
  is admissible.
- **Presentation, walkthrough, and facilitator authors** — to render
  presenter bar, agenda or waypoint rail, spotlight frame, breakaway
  banner, and speaker-note locality on top of canonical workspace
  surfaces, with the share state, follow grant, replay/retention
  class, reduced-motion behavior, and accessibility/keyboard route
  all resolved against frozen vocabularies.
- **Support, admin-envelope, and policy authors** — to suppress
  onboarding, learning, or presentation surfaces that cannot be
  honoured under the current envelope (policy-disabled learning,
  share-state blocked, replay-retention blocked, locale unavailable,
  imported pack unverified) with a typed cause rather than silent
  omission.

## Why this exists

Without this contract, discoverability and teaching surfaces drift
the fastest:

- a first-run welcome flow opens its own private "set up your
  account" mutation path that bypasses the no-account local-entry
  rule, breaking the local-first promise;
- a multi-step guided tour runs over a partial index and presents
  itself as live, so a user follows a "click here" cue against a
  surface that is actually still indexing;
- an architecture map mints a private graph-node anchor with no
  symbol-linked reference back to the source it claims to teach,
  so reviewers cannot reconstruct what the diagram corresponds to
  in the codebase;
- a glossary card falls back to source language without disclosure,
  so the user reads English content under a localized shell as if
  it were translated;
- an exercise step says "I'll fix that for you" and mounts an
  inline mutation path that is not a registry command — leaving the
  workspace changed with no governed audit trail;
- a "don't show again" toggle persists in a teaching-only progress
  lane, so dismissal does not follow the user's portable profile
  package and does not disclose to portability export;
- a presentation-mode shell mints its own "presenter shell" whose
  layout anchors do not correspond to the canonical workspace
  surfaces, so audience-visible content drifts away from what the
  rest of the product teaches;
- a presentation surface leaks speaker notes to the audience or
  exposes the presenter bar to viewers, violating the local /
  shared boundary;
- a walkthrough infers shared-terminal control from presenter
  state, widening authority outside the governed control-grant lane;
- a classroom session retains audience-visible replay artifacts
  with no privacy-bounded class declared, so a later reviewer
  cannot tell what was retained and on what authority;
- a breakaway from the presenter view leaves the audience member
  in an unspecified state with no recovery class, so rejoin behavior
  is implicit rather than declared.

This contract closes all of those gaps by declaring four object
models — guided tour / waypoint / architecture map / command-tip
pack on one schema, per-surface progress / dismissal / reset /
manifest on a second schema, and presentation session / layout
anchor / breakaway event / invariant manifest on a third schema —
plus one closed denial-reason set per schema and one explicit
binding to the learnability, command-registry, citation-anchor,
locale-pack, follow-and-presenter, and onboarding-portability truth
models. Tours, walkthroughs, learning modes, and presentations
remain a thin layer over canonical workspace surfaces.

## 1. Record kinds

### 1.1 `guided_tour_record`, `tour_waypoint_record`, `architecture_map_record`, `command_tip_pack_record`

Frozen on
[`/schemas/ux/guided_tour.schema.json`](../../schemas/ux/guided_tour.schema.json).

- One **`guided_tour_record`** per multi-step tour or walkthrough.
  Names the `tour_audience_class`, the `indexing_readiness_class`,
  the sequenced `waypoint_refs`, the optional `command_tip_pack_ref`,
  the locale and `source_language_fallback_class`, the docs-pack
  refs, the deployment-profile envelope, the reserved evidence
  hooks, the policy context, and the redaction class.
- One **`tour_waypoint_record`** per step. Cites exactly one
  canonical anchor (`command_id_anchor`, `docs_citation_anchor`,
  `symbol_linked_reference_anchor`, or `graph_node_anchor` which
  resolves through a symbol-linked reference), names the
  `layout_target_class` (a canonical workspace surface), declares
  the `partial_indexing_disclosure_class`, the
  `source_language_fallback_class`, the freshness class, the locale
  and locale-fallback disclosure, and the active flag.
- One **`architecture_map_record`** per architecture map. Holds a
  set of `architecture_map_node_record`s each binding a
  `symbol_linked_reference_ref` and an optional
  `upstream_citation_anchor_refs` so derived narratives retain
  their upstream anchors.
- One **`command_tip_pack_record`** per bundle of command tips
  referenced by waypoints. Each `command_tip_record` cites a
  canonical `command_id_ref` plus optional `docs_citation_anchor_ref`
  and `keymap_bridge_ref`.

### 1.2 `learning_progress_record`, `learning_dismissal_record`, `learning_progress_reset_record`, `learning_progress_manifest_record`

Frozen on
[`/schemas/ux/learning_progress.schema.json`](../../schemas/ux/learning_progress.schema.json).

- One **`learning_progress_record`** per (`learning_profile_ref`,
  `guided_surface_ref`) tuple. Names the `completion_class`, the
  re-exported `dismissal_class` and `reset_class`, the
  `progress_export_class`, the `dont_show_again_scope_class`, the
  optional `tour_ref` / `waypoint_ref`, the deployment-profile
  envelope, and the policy context.
- One **`learning_dismissal_record`** per dismissal event. Names
  the `dismissal_origin_class` (user explicit, "don't show again"
  checked, session expired, policy narrowed, imported profile
  carried, facilitator marked complete) plus the dismissal class
  and "don't show again" scope.
- One **`learning_progress_reset_record`** per reset event. Names
  the `progress_reset_origin_class`.
- One **`learning_progress_manifest_record`** aggregator declaring
  the eight const-true invariants for a learning profile's
  progress and dismissal state.

### 1.3 `presentation_session_record`, `presentation_layout_anchor_record`, `breakaway_event_record`, `presentation_invariant_manifest_record`

Frozen on
[`/schemas/ux/presentation_mode_state.schema.json`](../../schemas/ux/presentation_mode_state.schema.json).

- One **`presentation_session_record`** per presentation or
  walkthrough session. Names the presenter actor and role, the
  bound `guided_tour_ref`, the audience-visibility class, the
  presenter-bar visibility class, the speaker-note locality class,
  the share-state class, the follow-mode class, the focus-broadcast
  posture, the replay-retention class, the reduced-motion posture,
  the accessibility/keyboard class, the local/remote/shared
  boundary class, the deployment-profile envelope, and the
  reserved evidence hooks. The follow relationship resolves through
  a `follow_target_ref` on the collaboration follow record;
  mutating control resolves only through a `control_grant_record`
  and never through this row.
- One **`presentation_layout_anchor_record`** per layout anchor on
  the session: `presenter_bar`, `agenda_or_waypoint_rail`,
  `spotlight_frame`, `breakaway_banner`, or `speaker_note_locality`.
  Each anchor names a canonical `layout_target_class` (Start
  Center, command palette, primary editor, docs pane, architecture
  map canvas, review pane, etc.) plus a
  `local_remote_shared_boundary_class` so a reviewer can audit
  whether the anchor is local-only, requires a relay, or is shared
  to a named or managed audience.
- One **`breakaway_event_record`** per audience-side breakaway
  with a `breakaway_recovery_class` (auto-rejoin, manual invite,
  opt-out until session end, presenter revoked no rejoin, envelope
  narrowed no rejoin).
- One **`presentation_invariant_manifest_record`** aggregator
  declaring the twelve const-true invariants for the session set.

## 2. Closed vocabularies (frozen)

All vocabularies below are frozen as closed enums on the schema
files. Adding a new value is additive-minor and bumps the
companion `*_schema_version`; repurposing a value is breaking
and requires a new decision row.

### 2.1 Tour and waypoint vocabularies (`guided_tour.schema.json`)

| Vocabulary                            | Values                                                                                                                                                  |
|---------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| `tour_audience_class`                 | `first_run_no_account`, `first_run_after_opt_in`, `learning_mode_individual`, `learning_mode_classroom_managed`, `facilitator_led_walkthrough`, `architecture_map_walkthrough`, `post_migration_review`, `policy_disabled` |
| `indexing_readiness_class`            | `live_authoritative_full_index`, `partial_index_disclosed`, `stale_index_disclosed`, `cached_mirror_only_index`, `index_unavailable_suppressed`         |
| `waypoint_anchor_kind`                | `command_id_anchor`, `docs_citation_anchor`, `symbol_linked_reference_anchor`, `graph_node_anchor`                                                      |
| `layout_target_class`                 | `start_center`, `command_palette`, `workspace_switcher`, `primary_editor`, `docs_pane`, `architecture_map_canvas`, `review_pane`, `activity_center`, `support_center`, `settings_pane`, `no_canvas_target_facilitator_only` |
| `partial_indexing_disclosure_class`   | `no_disclosure_required_full_index`, `partial_index_disclosed_inline`, `partial_index_disclosed_in_freshness_chip`, `partial_index_disclosed_in_freshness_chip_and_route`, `partial_index_suppressed_active_false` |
| `source_language_fallback_class`      | `no_fallback_locale_native`, `fallback_to_source_language_disclosed`, `fallback_blocked_locale_unavailable`, `fallback_blocked_pack_missing`            |

### 2.2 Progress / dismissal vocabularies (`learning_progress.schema.json`)

| Vocabulary                            | Values                                                                                                                                                  |
|---------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| `completion_class`                    | `not_started`, `in_progress`, `completed`, `skipped`, `dismissed`, `blocked_by_envelope`                                                                |
| `dismissal_origin_class`              | `user_dismissed_explicit`, `user_dont_show_again_checked`, `session_expired_no_user_action`, `envelope_narrowed_under_policy`, `imported_profile_package_carried_dismissal`, `facilitator_marked_complete` |
| `dont_show_again_scope_class`         | `no_dont_show_again`, `dont_show_again_per_session`, `dont_show_again_per_profile`, `dont_show_again_per_device`, `dont_show_again_locked_by_policy`    |
| `progress_reset_origin_class`         | `user_initiated_reset`, `policy_initiated_reset`, `imported_profile_package_carried_reset`, `device_clear_on_launch_reset`, `facilitator_session_end_reset` |

### 2.3 Presentation vocabularies (`presentation_mode_state.schema.json`)

| Vocabulary                            | Values                                                                                                                                                  |
|---------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|
| `share_state_class`                   | `local_only_no_share`, `shared_to_named_audience`, `shared_to_classroom_managed_pool`, `shared_to_facilitator_only`, `shared_blocked_by_policy`, `shared_unavailable_in_envelope` |
| `replay_retention_class`              | `no_replay_retained`, `metadata_only_replay_local_redacted`, `metadata_only_replay_in_support_bundle_redacted`, `replay_blocked_by_policy`, `classroom_artifact_privacy_bounded` |
| `reduced_motion_posture_class`        | `respects_user_reduced_motion_setting`, `forced_reduced_motion_classroom_default`, `forced_reduced_motion_policy_locked`, `motion_unavailable_local_view_only` |
| `accessibility_keyboard_class`        | `full_keyboard_route_required`, `screen_reader_announcements_required`, `high_contrast_focus_required`, `no_special_accessibility_route_admitted_facilitator_only` |
| `presenter_bar_visibility_class`      | `presenter_bar_visible_to_presenter_only`, `presenter_bar_visible_to_co_presenters`, `presenter_bar_hidden_audience_facing`, `presenter_bar_unavailable_degraded_projection` |
| `audience_visibility_class`           | `audience_sees_workspace_surfaces_only`, `audience_sees_workspace_plus_agenda_rail`, `audience_sees_workspace_plus_spotlight_frame`, `audience_sees_workspace_plus_breakaway_banner`, `audience_sees_nothing_local_view_only` |
| `speaker_note_locality_class`         | `speaker_notes_facilitator_only_local`, `speaker_notes_co_presenter_visible`, `speaker_notes_hidden_no_facilitator_route`, `speaker_notes_blocked_by_policy` |
| `local_remote_shared_boundary_class`  | `local_only_machine_bound`, `remote_relay_required`, `shared_audience_managed_pool`, `shared_audience_named_invitees`, `boundary_unavailable_in_envelope` |
| `breakaway_recovery_class`            | `auto_rejoin_on_relay_recovery`, `manual_rejoin_invite_required`, `opt_out_until_session_end`, `presenter_revoked_no_rejoin`, `envelope_narrowed_no_rejoin` |
| `layout_anchor_kind`                  | `presenter_bar`, `agenda_or_waypoint_rail`, `spotlight_frame`, `breakaway_banner`, `speaker_note_locality`                                             |

## 3. Tours, waypoints, architecture maps, and command tips

Rules (frozen):

1. **Every waypoint cites exactly one canonical anchor.** A
   `tour_waypoint_record` whose `anchor.anchor_kind` does not
   resolve to a `command_id`, a `docs_citation_anchor`, a
   `symbol_linked_reference`, or (for architecture-map waypoints)
   a `graph_node_anchor` that itself resolves to a
   `symbol_linked_reference` is non-conforming
   (`denial_reason = guided_tour_waypoint_without_canonical_anchor`).
2. **Architecture-map nodes are symbol-anchored.** An
   `architecture_map_node_record` MUST populate
   `symbol_linked_reference_ref`; minting a private graph-node
   payload as the primary anchor is non-conforming.
3. **No private mutation paths.** A waypoint primary action, an
   inline action on a tour step, and any "apply suggestion" control
   on a command tip resolve through a `command_id_ref`. Anything
   else is non-conforming
   (`denial_reason = guided_tour_action_opens_private_mutation_path`).
4. **Layout targets are canonical.** A waypoint MUST land on a
   `layout_target_class` from the closed set; a tour that mints a
   private "tour shell" target is non-conforming
   (`denial_reason = guided_tour_layout_target_not_canonical_surface`).
5. **Partial-index disclosure is explicit.** A waypoint over a
   partial or stale index MUST set
   `partial_indexing_disclosure_class` to a
   `partial_index_disclosed_*` value; rendering a partial-index tour
   without disclosure is non-conforming
   (`denial_reason = guided_tour_partial_index_not_disclosed`).
6. **Locale fallbacks are disclosed.** A waypoint or command tip
   that resolves through `fallback_to_source_language_disclosed`
   MUST set `locale_fallback_disclosed = true`; silent fallback is
   non-conforming
   (`denial_reason = guided_tour_locale_fallback_not_disclosed`).
7. **First-run no-account tours never require an account.** A tour
   whose `tour_audience_class = first_run_no_account` MUST be
   admissible on `individual_local` or `self_hosted` deployment
   profiles. Any waypoint on such a tour whose primary action's
   `boundary_crossing_class` widens trust or reaches a remote
   resource is non-conforming
   (`denial_reason = guided_tour_audience_envelope_mismatch`).
8. **Imported teaching packs are verified.** A tour whose docs-pack
   ref or command-tip pack ref resolves through an unverified
   imported pack is non-conforming
   (`denial_reason = guided_tour_imported_pack_unverified`).

## 4. Progress, dismissal, "don't show again", and portability

Rules (frozen):

1. **Every progress row resolves against a guided-surface record.**
   A `learning_progress_record` MUST name a `guided_surface_ref`;
   teaching-only progress with no underlying guided surface is
   non-conforming.
2. **"Don't show again" scope matches the dismissal class.** A
   `per_profile_dismissable` surface MUST NOT carry a
   `dont_show_again_per_device` scope, and so on
   (`denial_reason = learning_progress_dont_show_again_scope_mismatch`).
3. **Portable progress never leaks account- or policy-state.** A
   `per_profile_dismissable` progress row MUST export through the
   portable-profile lane only
   (`denial_reason = learning_progress_export_class_mismatch`).
4. **Policy-locked progress carries policy context.** A
   `policy_locked` progress row MUST populate
   `policy_context.policy_epoch`
   (`denial_reason = learning_progress_policy_locked_lacks_policy_context`).
5. **Imported dismissal carries an origin.** A
   `learning_dismissal_record` whose
   `dismissal_origin_class = imported_profile_package_carried_dismissal`
   MUST populate `imported_profile_package_ref`
   (`denial_reason = learning_progress_imported_dismissal_lacks_origin`).
6. **`blocked_by_envelope` never marks completed.** A row whose
   `completion_class = blocked_by_envelope` MUST NOT also carry a
   `dont_show_again` scope outside `no_dont_show_again` /
   `dont_show_again_locked_by_policy`
   (`denial_reason = learning_progress_blocked_by_envelope_carries_completion`).

Reset and export classes re-project the onboarding-portability
vocabulary (`resettable_per_profile`, `resettable_per_device`,
`resettable_per_account`, `resettable_by_policy`,
`not_resettable_locally`; `in_portable_profile_package`,
`in_portable_profile_package_redacted`, `not_exported_machine_local`,
`in_support_bundle_redacted`, `blocked_by_policy`) — no parallel
lanes, and a teaching surface that mints a private "teaching-only"
progress lane is non-conforming.

Progress, dismissal, and "don't show again" state are not stored in
the repo-owned source tree; they ride the portable profile package
or the per-device diagnostic lane per the onboarding-portability
contract.

## 5. Evidence-hook reserves

Five evidence-hook reserves are admitted on tour, waypoint,
architecture-map, command-tip pack, presentation-session, layout-
anchor, breakaway-event, and progress-manifest records so a later
audit can be qualified without redefining object shape:

- `tour_citation_audit` — reserves space for a packet that walks
  every waypoint citation back to its canonical anchor and reports
  any drift.
- `cached_pack_continuity` — reserves space for a packet that
  reports whether the docs/locale/teaching pack the tour resolves
  against has remained continuous (no missed revision, no mirror-
  only-without-disclosure window).
- `presentation_role_review` — reserves space for a packet that
  reports the presenter, co-presenter, observer, and audience
  bindings during a session for a later facilitator review.
- `layout_restore_proof` — reserves space for a packet that
  proves every layout anchor on a session landed on a canonical
  workspace surface (no private presentation shell minted).
- `privacy_bounded_classroom_artifact` — reserves space for a
  packet that reports what audience-visible artifacts a classroom
  or facilitator-led session retained, under what privacy-bounded
  retention class, with no audience identity leaked.

The reserve is additive-optional; a row that does not need a hook
omits it and does not bump the schema version.

## 6. Invariants

Every `learning_progress_manifest_record` MUST declare the
following const-true invariants:

1. `every_progress_row_resolves_against_a_guided_surface_record`
2. `portable_progress_never_leaks_account_or_policy_state`
3. `dont_show_again_scope_matches_underlying_dismissal_class`
4. `blocked_by_envelope_never_marks_completed`
5. `policy_locked_progress_carries_policy_context`
6. `imported_dismissal_carries_origin_class`
7. `reset_lane_matches_dismissal_lane`
8. `machine_local_progress_never_exports_via_portable_lane`

Every `presentation_invariant_manifest_record` MUST declare the
following const-true invariants:

1. `every_layout_anchor_lands_on_canonical_workspace_surface`
2. `speaker_notes_never_cross_to_audience`
3. `presenter_bar_never_renders_to_audience`
4. `presentation_authority_does_not_confer_mutation`
5. `follow_state_resolves_against_collaboration_follow_record`
6. `share_state_matches_deployment_envelope`
7. `replay_retention_matches_export_lane`
8. `reduced_motion_user_setting_respected_unless_disclosed`
9. `audience_visible_surfaces_are_only_canonical_workspace_surfaces`
10. `breakaway_recovery_is_explicit_and_envelope_bounded`
11. `speaker_note_adjunct_preserves_command_identity`
12. `evidence_hooks_reserved_for_presentation_role_review_and_classroom_artifact`

## 7. Presentation mode and walkthroughs

Presentation mode is a thin layer over canonical workspace surfaces.
The rules are uniform:

1. **Layout anchors land on canonical surfaces.** Every
   `presentation_layout_anchor_record` MUST name a
   `layout_target_class` from the same set tour waypoints use
   (Start Center, command palette, primary editor, docs pane,
   architecture-map canvas, review pane, etc.). Minting a private
   "presentation shell" target is non-conforming
   (`denial_reason = presentation_session_layout_anchor_not_canonical_surface`).
2. **Presenter bar never renders to the audience.** Audience-facing
   `audience_visibility_class` values forbid
   `presenter_bar_visibility_class = presenter_bar_unavailable_degraded_projection`
   from also being interpreted as audience-visible; the closed
   audience-visibility set excludes the presenter bar
   (`denial_reason = presentation_session_audience_sees_presenter_bar`).
3. **Speaker notes never cross to the audience.** A
   `speaker_note_locality_class` of
   `speaker_notes_co_presenter_visible` is admitted only when
   audience-visibility is not `audience_sees_nothing_local_view_only`;
   speaker notes are otherwise facilitator-only or hidden
   (`denial_reason = presentation_session_speaker_notes_visible_to_audience`).
4. **Share state matches the envelope.** A
   `share_state_class = shared_to_classroom_managed_pool` is
   admissible only on `enterprise_online` or `managed_cloud`
   deployment profiles; an air-gapped envelope MUST resolve
   `shared_unavailable_in_envelope` rather than render shared
   surfaces
   (`denial_reason = presentation_session_share_state_envelope_mismatch`).
5. **Follow state rides the collaboration follow record.** A
   shared session MUST populate `follow_target_ref` so the audience
   follow relationship reads through the
   `follow_target_record` lane on the collaboration schema. A
   focus-broadcast active posture without a presenter or
   co-presenter role denies with
   `presentation_session_focus_broadcast_without_presenter_role`.
6. **Authority does not confer mutation.** A presentation row that
   asserts mutating authority denies with
   `presentation_session_authority_inferred_for_mutation`. Mutating
   control rides a `control_grant_record` on the collaboration
   schema and is never inferable from a presenter or follow row.
7. **Replay retention is declared.** A
   `replay_blocked_by_policy` or
   `classroom_artifact_privacy_bounded` row MUST attach an
   evidence-hook reserve so the audit can be qualified later
   (`denial_reason = presentation_session_replay_retention_envelope_mismatch`).
8. **Reduced motion respects the user.** A
   `forced_reduced_motion_*` posture MUST disclose its override; a
   row that overrides the user's reduced-motion setting without
   disclosure is non-conforming
   (`denial_reason = presentation_session_reduced_motion_overridden_without_disclosure`).
9. **Breakaway is explicit and envelope-bounded.** Every
   `breakaway_event_record` carries a `breakaway_recovery_class`;
   `presenter_revoked_no_rejoin` and `envelope_narrowed_no_rejoin`
   forbid a `follow_target_ref_after_recovery`
   (`denial_reason = presentation_session_breakaway_recovery_inconsistent_with_envelope`).
10. **Speaker-note adjunct preserves command identity.** Speaker-
    note and teaching-session adjuncts inherit invariant 10 from
    the learnability contract (`presentation_adjunct_preserves_command_identity`):
    the adjunct retains the same `command_id_ref` (and primary
    docs anchor) as its audience-visible step.

## 8. Acceptance mapping

| Acceptance clause                                                                                                                | Resolved by                                                                                                                                                  |
|----------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reviewers can trace every guided action back to the same command and docs systems used elsewhere in the product.                | §3 rules 1–4, learnability contract §3 authority classes, §1.1 anchor / §1.3 follow target binding.                                                          |
| Progress, dismissal, and "don't show again" state are explicit, portable, resettable, and not stored in repo-owned source trees. | §2.2 vocabularies, §4 rules 1–6, learning_progress_record schema constraints, learning_progress_manifest_record invariants 1, 2, 3, 7, 8.                    |
| Fixtures cover at least: no-account first-run, cited tour over partial indexing, glossary card with source-language fallback, exercise step with reversible scope, presentation mode with shared follow disabled/enabled. | Fixture set in `/fixtures/ux/learning_cases/` (see §11).                                                                                                     |
| Presentation and guided-tour surfaces can be audited for source citations, cached-pack fallback, shared-role boundaries, and privacy/export posture with the same packet vocabulary later used in release evidence. | §5 evidence hooks (`tour_citation_audit`, `cached_pack_continuity`, `presentation_role_review`, `layout_restore_proof`, `privacy_bounded_classroom_artifact`). |
| Fixtures cover a guided session with follow on/off, presenter notes hidden from viewers, breakaway recovery, and cited waypoint navigation over the same underlying content surfaces. | Fixture set in `/fixtures/ux/learning_cases/` (see §11).                                                                                                     |

## 9. Denial reasons

The denial-reason sets are reserved on the three companion schemas.
A shell that would otherwise render a non-conforming row MUST deny
with the matching reason rather than silently fall back:

- `guided_tour.schema.json`:
  `guided_tour_waypoint_without_canonical_anchor`,
  `guided_tour_partial_index_not_disclosed`,
  `guided_tour_locale_fallback_not_disclosed`,
  `guided_tour_layout_target_not_canonical_surface`,
  `guided_tour_audience_envelope_mismatch`,
  `guided_tour_action_opens_private_mutation_path`,
  `guided_tour_imported_pack_unverified`,
  `guided_tour_schema_version_lagging`.
- `learning_progress.schema.json`:
  `learning_progress_dismissal_class_mismatch`,
  `learning_progress_export_class_mismatch`,
  `learning_progress_dont_show_again_scope_mismatch`,
  `learning_progress_blocked_by_envelope_carries_completion`,
  `learning_progress_imported_dismissal_lacks_origin`,
  `learning_progress_policy_locked_lacks_policy_context`,
  `learning_progress_schema_version_lagging`.
- `presentation_mode_state.schema.json`:
  `presentation_session_layout_anchor_not_canonical_surface`,
  `presentation_session_speaker_notes_visible_to_audience`,
  `presentation_session_share_state_envelope_mismatch`,
  `presentation_session_replay_retention_envelope_mismatch`,
  `presentation_session_follow_state_without_presenter_role`,
  `presentation_session_focus_broadcast_without_presenter_role`,
  `presentation_session_authority_inferred_for_mutation`,
  `presentation_session_audience_sees_presenter_bar`,
  `presentation_session_breakaway_recovery_inconsistent_with_envelope`,
  `presentation_session_reduced_motion_overridden_without_disclosure`,
  `presentation_session_schema_version_lagging`.

## 10. Adding a new vocabulary value

Adding a new `tour_audience_class`, `indexing_readiness_class`,
`waypoint_anchor_kind`, `layout_target_class`,
`partial_indexing_disclosure_class`, `source_language_fallback_class`,
`completion_class`, `dismissal_origin_class`,
`dont_show_again_scope_class`, `progress_reset_origin_class`,
`share_state_class`, `replay_retention_class`,
`reduced_motion_posture_class`, `accessibility_keyboard_class`,
`presenter_bar_visibility_class`, `audience_visibility_class`,
`speaker_note_locality_class`, `local_remote_shared_boundary_class`,
`breakaway_recovery_class`, `layout_anchor_kind`, `evidence_hook_class`,
or `denial_reason` is **additive-minor** and bumps the relevant
`*_schema_version`. Repurposing an existing value is **breaking**
and requires a new decision row on the launch decision register.
A consumer surface that resolves a value it does not recognize MUST
deny with the matching `*_schema_version_lagging` reason rather
than silently map to a default.

## 11. Worked examples

Fixtures under
[`/fixtures/ux/learning_cases/`](../../fixtures/ux/learning_cases/)
cover:

1. **No-account first-run path** —
   `first_run_no_account_tour.yaml`. A `guided_tour_record` and
   sequenced `tour_waypoint_record`s for a first-run tour that
   resolves on `individual_local` and `self_hosted` profiles with
   no account opt-in. Every waypoint cites a canonical
   `command_id_ref` from the registry; the tour never widens trust.
2. **Cited tour over partial indexing** —
   `tour_over_partial_indexing.yaml`. A `guided_tour_record` whose
   `indexing_readiness_class = partial_index_disclosed`, with
   waypoints that set
   `partial_indexing_disclosure_class = partial_index_disclosed_in_freshness_chip`
   so the user reads the partial-index state rather than mistaking
   the tour for live.
3. **Glossary card with source-language fallback** —
   `glossary_card_source_language_fallback.yaml`. A
   `tour_waypoint_record` and companion `guided_surface_state_record`
   for a glossary card whose locale falls back to the docs source
   language, with `locale_fallback_disclosed = true` and
   `source_language_fallback_class = fallback_to_source_language_disclosed`.
4. **Exercise step with reversible scope** —
   `exercise_step_reversible_scope.yaml`. A `tour_waypoint_record`
   bound to an exercise-step `guided_surface_state_record` with
   `exercise_mutation_posture = mutates_workspace_after_ack`, plus
   a companion `learning_progress_record` whose dismissal lane is
   `per_profile_dismissable` and progress export is portable.
5. **Presentation mode with shared follow disabled** —
   `presentation_session_shared_follow_disabled.yaml`. A
   `presentation_session_record` whose
   `share_state_class = local_only_no_share`,
   `follow_mode_class = follow_paused_local_view_only`, and
   `audience_visibility_class = audience_sees_nothing_local_view_only`.
   The session is admissible on the local-only envelope and runs
   without a follow target ref.
6. **Presentation mode with shared follow enabled** —
   `presentation_session_shared_follow_enabled.yaml`. A
   `presentation_session_record` with
   `share_state_class = shared_to_named_audience`,
   `follow_mode_class = presenter_broadcast_follow`,
   `focus_broadcast_posture_class = focus_broadcast_active`,
   `presenter_bar_visibility_class = presenter_bar_hidden_audience_facing`,
   `speaker_note_locality_class = speaker_notes_facilitator_only_local`,
   bound to a `follow_target_ref` on the collaboration follow
   record and a `presenter_state_ref` on the presenter state record.
   Layout anchors land on canonical workspace surfaces (primary
   editor + agenda rail + spotlight frame).
7. **Breakaway recovery and cited waypoint navigation** —
   `presentation_breakaway_and_waypoint_navigation.yaml`. A
   `breakaway_event_record` with
   `breakaway_recovery_class = manual_rejoin_invite_required` plus
   the `tour_waypoint_record` the audience navigates to via the
   cited waypoint route after rejoin — proving the audience and
   presenter share the same waypoint citation lane.
8. **Learning-progress manifest** —
   `learning_progress_manifest_individual_learner.yaml`. A
   `learning_progress_manifest_record` declaring the eight const-
   true invariants and binding portable progress, dismissal, and
   reset rows.
9. **Presentation invariant manifest** —
   `presentation_invariant_manifest.yaml`. A
   `presentation_invariant_manifest_record` declaring the twelve
   const-true invariants for the presentation-mode session set.

## 12. Out of scope at this revision

- Shipping a complete learning mode, classroom managed-pool
  workflow, or full presentation mode in M0. This contract freezes
  the boundary; the actual tour content, walkthrough library,
  classroom enrollment, presentation UI, and post-session review
  flows land in later milestones and ride this contract.
- Final visuals (presenter bar layout, spotlight frame styling,
  agenda rail density). The design-system style guide owns those.
- Analytics and scoreboarding beyond the existing measurement-
  surface linkage on the learnability contract; the onboarding-
  measurement plan owns the qualification event names.
- Actual policy-bundle definitions that disable share state,
  replay retention, or learning surfaces. The identity / policy-
  bundle contracts own those; this contract names the typed
  suppression cause.
