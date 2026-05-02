# Learning artifact object model

This document freezes the **content / authoring object model** every
Aureline **glossary pack item**, **tour step**, **exercise step**,
**learning-mode profile**, **teaching session**, and **speaker note**
resolves through before a guided shell renders it. The goal is to keep
learnability content in typed, portable, inspectable, independently-
versioned objects rather than scattered prose, ad-hoc UI state, or
parallel teaching-only schemas.

The machine-readable schemas live at:

- [`/schemas/ux/glossary_pack_item.schema.json`](../../schemas/ux/glossary_pack_item.schema.json)
- [`/schemas/ux/tour_step.schema.json`](../../schemas/ux/tour_step.schema.json)
- [`/schemas/ux/exercise_step.schema.json`](../../schemas/ux/exercise_step.schema.json)
- [`/schemas/ux/learning_mode_profile.schema.json`](../../schemas/ux/learning_mode_profile.schema.json)
- [`/schemas/ux/teaching_session.schema.json`](../../schemas/ux/teaching_session.schema.json)
- [`/schemas/ux/speaker_note.schema.json`](../../schemas/ux/speaker_note.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/learning_artifacts/`](../../fixtures/ux/learning_artifacts/)

This contract is normative for the **content shape, target-ref
locality, citation backing, completion-detection, allowed-action set,
sandbox/reversibility posture, tip intensity, jargon level, follow
state, replay/retention class, privacy scope, and note ownership /
share state** of every learning artifact a pack author publishes.
Where it disagrees with the PRD, TAD, TDD, UI/UX spec, or milestone
document, those sources win and this document plus its companion
schemas and fixtures update in the same change. Where a downstream
glossary, tour, exercise, learning-mode, teaching-session, or
speaker-note artifact mints a parallel vocabulary, this contract wins
and the artifact is non-conforming.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference:

- [`/docs/ux/learnability_contract.md`](./learnability_contract.md)
  and
  [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)
  — `guided_surface_state_record`, `guided_surface_rule_record`,
  `learning_mode_profile_record`, the closed `guided_surface_kind`,
  `guidance_authority_class`, `dismissal_class`, `reset_class`,
  `progress_export_class`, `freshness_class`, `suppression_cause_class`,
  `teaching_session_adjunct_class`, `learning_mode_profile_class`,
  and `exercise_mutation_posture` enums. The artifacts here are the
  **authored content** that a `guided_surface_state_record` resolves
  against at render time; this contract binds the artifact shape and
  carries no shell render state.
- [`/docs/ux/learning_and_presentation_contract.md`](./learning_and_presentation_contract.md)
  and
  [`/schemas/ux/guided_tour.schema.json`](../../schemas/ux/guided_tour.schema.json),
  [`/schemas/ux/learning_progress.schema.json`](../../schemas/ux/learning_progress.schema.json),
  [`/schemas/ux/presentation_mode_state.schema.json`](../../schemas/ux/presentation_mode_state.schema.json)
  — `guided_tour_record`, `tour_waypoint_record`,
  `command_tip_pack_record`, `learning_progress_record`,
  `presentation_session_record`, `presentation_layout_anchor_record`,
  the closed `tour_audience_class`, `indexing_readiness_class`,
  `layout_target_class`, `partial_indexing_disclosure_class`,
  `source_language_fallback_class`, `share_state_class`,
  `replay_retention_class`, `audience_visibility_class`,
  `speaker_note_locality_class`, `local_remote_shared_boundary_class`,
  `breakaway_recovery_class`, and `evidence_hook_class` enums. Tour
  step, teaching-session, and speaker-note artifacts on this contract
  resolve through these multi-step, share-state, follow-state, and
  replay/retention lanes; no parallel sequencing or share-state
  vocabulary is admissible.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md),
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json),
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  — canonical `command_id`, alias lifecycle, keybinding refs, and
  capability binding. Every artifact field that names a target
  action, a primary action, an allowed action, an apply-suggestion
  control, or a fix-up shortcut resolves through a stable
  `command_id` on the registry. Hidden write paths outside the
  command, trust, and approval systems are forbidden.
- [`/docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md),
  [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json),
  and
  [`/schemas/docs/symbol_linked_reference.schema.json`](../../schemas/docs/symbol_linked_reference.schema.json)
  — `docs_citation_anchor_record` (including `glossary_term_anchor`,
  `onboarding_step_anchor`), `derivation`, `source_class`,
  `freshness_class`, `version_match_state`, `reuse_surface`, and
  `export_posture`. Every glossary item and every tour-step body
  whose summary paraphrases or quotes a docs source carries the
  upstream anchor it derives from; an artifact that promises a
  citation and ships none denies as non-conforming.
- [`/docs/ux/contextual_teaching_contract.md`](./contextual_teaching_contract.md)
  and
  [`/schemas/ux/teaching_surface.schema.json`](../../schemas/ux/teaching_surface.schema.json)
  — contextual tip, migration bridge, why-unavailable, and
  source-language fallback payloads. Tour-step and exercise-step
  artifacts compose with these payloads when projected as a
  contextual tip; this contract carries the multi-step / sequenced /
  authored-pack shape on top.
- [`/docs/collaboration/shared_control_contract.md`](../collaboration/shared_control_contract.md),
  [`/schemas/collaboration/follow_and_presenter_state.schema.json`](../../schemas/collaboration/follow_and_presenter_state.schema.json),
  and
  [`/schemas/collaboration/control_grant.schema.json`](../../schemas/collaboration/control_grant.schema.json)
  — `follow_target_record`, `presenter_state_record`,
  `presenter_handoff_record`, `control_grant_record`. Teaching-session
  artifacts that declare a follow posture or a shared audience read
  through these collaboration records; the artifact never mints a
  parallel follow-target vocabulary, and authority **does not confer
  mutation** at the artifact layer.
- [`/docs/ux/no_account_local_entry_contract.md`](./no_account_local_entry_contract.md)
  and
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)
  — `state_portability_class`, `reset_class`, `export_class`,
  `profile_scope_class`. Learning-mode profile artifacts ride that
  contract's portability lanes; no teaching-only export lane is
  admissible.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary so air-gapped, managed-cloud,
  and restricted envelopes inherit admissibility mechanically.

## Who reads this contract

- **Glossary, tour, exercise, learning-mode, presentation, and
  classroom pack authors** — to author teaching content against one
  artifact record shape per kind whose fields are typed, citations
  resolved, target refs canonical, completion criteria explicit,
  allowed actions registry-bound, sandbox / reversibility posture
  declared, jargon level / tip intensity stated, and privacy scope
  enumerated.
- **Bundle / packaging / signing authors** — to bundle, version,
  sign, and ship learning artifacts as portable packs that survive
  pack revision, locale fallback, deployment envelope changes, and
  shell relayout without dragging UI state.
- **Reviewers and support** — to inspect a learning artifact and
  read off its target refs, citations, completion detectors, allowed
  actions, sandbox / reversibility class, privacy scope, and share
  state without crawling rendered UI screenshots.
- **Docs, help, and AI-explanation authors** — to reuse the same
  citation and target-ref object model glossary cards, onboarding
  steps, architecture explainers, and AI-assisted explainer overlays
  consume, so a glossary item authored here can light up in docs,
  help, AI-explanation overlays, and onboarding without parallel
  schemas.
- **Admin-envelope and policy authors** — to disable, narrow, or
  redact learning artifacts under a typed envelope rather than via
  bespoke policy hooks per pack.

## Why this exists

Without this contract, learning content drifts the fastest:

- a glossary card carries a hand-typed verb in its definition body
  instead of a `command_id_ref`, so the card keeps reading "Click
  Open folder" after the underlying command is renamed, replaced, or
  split — teaching a phantom action;
- a tour step pins on raw screen coordinates ("the third button on
  the second toolbar") instead of a stable target ref, so a single
  density change, splitter resize, or theme swap silently breaks the
  step;
- an exercise step "applies a fix" via an inline private mutation
  path that is not a registry command, has no governed audit trail,
  no rollback handle, and no trust prompt;
- a tour step's completion detector is implicit ("just click around
  for a bit"), so the user, the shell, and the analytics layer
  disagree about whether the step actually completed;
- a learning-mode profile mints a per-pack "tip intensity" ranking
  with no shared vocabulary, so a classroom pool and a beginner
  individual learner cannot share a profile bundle;
- a teaching session retains audience-visible artifacts with no
  declared replay-retention class, so a later reviewer cannot tell
  what was retained, on what authority, with what privacy scope;
- a speaker note ships with no ownership / share-state declaration,
  so it leaks to the audience the moment a session goes shared;
- a glossary card promises a citation in its body but ships with no
  anchor pin, so a reviewer cannot reconstruct which docs revision
  the entry was authored against;
- a teaching session declares a follow posture inline rather than
  through a `follow_target_record`, widening shared-control authority
  outside the governed collaboration lane.

This contract closes those gaps by declaring **one artifact record
per kind**, **one set of typed target refs per kind** (registry
`command_id`, docs anchor, symbol-linked reference, keymap-bridge
resolver, layout target class, file scope ref), **one closed allowed-
actions vocabulary that bottoms out on registry command ids**, **one
closed completion-detector vocabulary**, **one closed
sandbox / reversibility / tip-intensity / jargon-level / privacy-scope
vocabulary**, and **one explicit binding to the learnability,
guided-tour, learning-progress, presentation-mode, citation-anchor,
collaboration-follow, and onboarding-portability truth models** so
artifacts survive label churn, pack revisions, locale fallback,
deployment narrowing, share-state changes, and presentation handoff
without inventing parallel authority.

## 1. Artifact kinds

Each artifact carries one record kind plus an integer
`*_artifact_schema_version` field. Adding a new field is
additive-optional and does **not** bump the schema version; adding a
new closed enum value is additive-minor and bumps the version;
repurposing an existing value is breaking and requires a new
decision row.

### 1.1 `glossary_pack_item_artifact`

One record per glossary entry in a glossary pack. Fields:

- **identity** — `item_id` (opaque), `pack_ref` (opaque pin to the
  glossary pack manifest), `pack_revision_ref`, `default_locale`
  (BCP-47).
- **term + definition** — `term_short_label`, `definition_summary`
  (privacy-safe label text, no raw URLs, no raw paths, no raw
  prompt / completion text), `jargon_level_class`,
  `synonym_short_labels` (array of short labels).
- **citation** — `primary_docs_citation_anchor_ref` (required),
  `upstream_citation_anchor_refs` (required non-empty when the
  definition paraphrases more than the primary anchor),
  `derivation_class` (`primary`, `derived`,
  `derived_with_upstream_anchors`).
- **stable target refs** — `related_command_id_refs`,
  `related_symbol_linked_reference_refs`, `related_layout_target_class`
  pointing at canonical workspace surfaces; raw coordinates are
  forbidden.
- **deployment / privacy** — `deployment_profile_refs`,
  `privacy_scope_class`, `redaction_class`.

### 1.2 `tour_step_artifact`

One record per authored tour step. Fields:

- **identity** — `step_id` (opaque), `tour_pack_ref`,
  `pack_revision_ref`, `position_index`, `default_locale`.
- **content** — `title_short_label`, `body_summary`,
  `jargon_level_class`, `tip_intensity_class`.
- **target ref** — `target_ref` discriminated union over
  `command_id_target`, `docs_citation_anchor_target`,
  `symbol_linked_reference_target`, `keymap_bridge_target`,
  `layout_target` (canonical `layout_target_class`); raw coordinates
  are forbidden.
- **completion detector** — `completion_detector` discriminated
  union over `command_invocation_detector` (cite the canonical
  `command_id_ref` whose successful invocation completes the step),
  `docs_anchor_seen_detector`, `layout_focus_detector`
  (focus reaches a canonical `layout_target_class`),
  `time_window_detector` (max-duration capped, no exit hatch on
  trust-bearing steps), `manual_acknowledge_detector`.
- **allowed actions** — `allowed_action_refs` (array of opaque
  `command_id_ref`s); inline actions on the step MUST resolve to one
  of these and only these.
- **citation** — `primary_docs_citation_anchor_ref` (optional for
  layout-only steps, required when the body paraphrases),
  `upstream_citation_anchor_refs`,
  `source_language_fallback_class`.
- **deployment / privacy** — `deployment_profile_refs`,
  `privacy_scope_class`, `redaction_class`.

### 1.3 `exercise_step_artifact`

One record per authored exercise step. Fields:

- **identity** — `step_id`, `tour_pack_ref` (optional; exercises may
  ship outside a tour bundle), `pack_revision_ref`, `position_index`,
  `default_locale`.
- **content** — `title_short_label`, `body_summary`,
  `jargon_level_class`.
- **target ref** — same target-ref discriminator as §1.2; an
  exercise step that mutates the workspace MUST pin a stable
  `command_id_target` rather than a layout target.
- **completion detector** — same vocabulary as §1.2.
- **allowed actions** — `allowed_action_refs` (registry
  `command_id_ref`s only).
- **sandbox / reversibility** —
  `sandbox_class` (`read_only_walkthrough`,
  `mutates_workspace_after_ack`, `mutates_in_isolated_sandbox`,
  `blocked_until_trust_granted`),
  `reversibility_class` (`reversible_via_command`,
  `reversible_via_undo_history`, `irreversible_requires_explicit_ack`),
  `mutation_posture` (re-export of the upstream
  `exercise_mutation_posture` from
  `guided_surface_state.schema.json`).
- **citation** — same as §1.2.
- **deployment / privacy** — `deployment_profile_refs`,
  `privacy_scope_class`, `redaction_class`.

### 1.4 `learning_mode_profile_artifact`

One record per portable learning-mode profile bundle. Fields:

- **identity** — `profile_artifact_id`, `profile_class` (re-export
  of `learning_mode_profile_class` from
  `guided_surface_state.schema.json`),
  `profile_artifact_version` (semver-style short label).
- **defaults** — `default_jargon_level_class`,
  `default_tip_intensity_class`, `default_locale`,
  `default_dismissal_class`, `default_reset_class`,
  `default_progress_export_class`.
- **bound packs** — `glossary_pack_refs`, `tour_pack_refs`,
  `command_tip_pack_refs`, `docs_pack_refs`.
- **share / follow defaults** —
  `default_share_state_class`,
  `default_follow_mode_class` (opaque tag re-exported from
  `presentation_mode_state.schema.json`),
  `default_replay_retention_class`.
- **envelope** — `deployment_profile_refs`, `policy_context`,
  `redaction_class`, `state_portability_class` (re-export from
  `onboarding_portability_state.schema.json`).
- **invariants** — every `learning_mode_profile_artifact` declares
  a const-true block of the same twelve learnability invariants the
  `learning_mode_profile_record` declares so the artifact and the
  runtime state row read the same lanes.

### 1.5 `teaching_session_artifact`

One record per authored shared teaching or presentation session
bundle. Fields:

- **identity** — `session_artifact_id`, `session_class`
  (`facilitator_led_walkthrough`, `classroom_managed_session`,
  `live_pair_review`, `architecture_map_walkthrough`,
  `release_or_demo_walkthrough`, `local_only_dry_run`),
  `default_locale`.
- **bound content** — `tour_pack_ref`, `glossary_pack_refs`,
  `command_tip_pack_refs`, `speaker_note_pack_ref`.
- **share / follow / replay** —
  `share_state_class`, `audience_visibility_class`,
  `speaker_note_locality_class`,
  `presenter_bar_visibility_class`,
  `local_remote_shared_boundary_class`,
  `follow_state_class` (re-export of the collaboration follow
  posture; carries no mutation authority),
  `replay_retention_class`,
  `reduced_motion_posture_class`,
  `accessibility_keyboard_class`,
  `breakaway_recovery_default_class`.
- **envelope** — `deployment_profile_refs`, `policy_context`,
  `redaction_class`, `evidence_hooks_reserved`.

### 1.6 `speaker_note_artifact`

One record per authored speaker note. Fields:

- **identity** — `note_id`, `pack_ref`, `paired_step_ref`
  (opaque pin to a `tour_step_artifact` or `exercise_step_artifact`).
- **content** — `body_summary` (privacy-safe label text),
  `jargon_level_class`.
- **ownership / share state** —
  `ownership_class` (`presenter_owned_local`,
  `co_presenter_visible`, `facilitator_only_visible`,
  `facilitator_audience_visible`, `audience_visible`),
  `share_state_class` (re-export from
  `presentation_mode_state.schema.json`),
  `speaker_note_locality_class`.
- **command identity preservation** —
  `preserved_command_id_ref` MUST equal the paired step's primary
  `command_id_ref` whenever the paired step has one;
  `preserved_docs_citation_anchor_ref` MUST equal the paired
  step's primary docs anchor whenever present. A note that names
  an action without preserving its `command_id_ref` is
  non-conforming.
- **envelope** — `deployment_profile_refs`, `policy_context`,
  `redaction_class`.

## 2. Closed vocabularies (frozen)

All vocabularies below are frozen as closed enums on the schema
files. Adding a new value is additive-minor and bumps the relevant
`*_artifact_schema_version`; repurposing a value is breaking and
requires a new decision row.

### 2.1 Authoring-only vocabularies (this contract)

| Vocabulary                       | Values                                                                                                                                                       |
|----------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `jargon_level_class`             | `beginner`, `intermediate`, `advanced`, `expert`                                                                                                             |
| `tip_intensity_class`            | `silent_inline_only`, `gentle_hint`, `prompted_acknowledge`, `mandatory_step_required`                                                                       |
| `completion_detector_kind`       | `command_invocation_detector`, `docs_anchor_seen_detector`, `layout_focus_detector`, `time_window_detector`, `manual_acknowledge_detector`                   |
| `target_ref_kind`                | `command_id_target`, `docs_citation_anchor_target`, `symbol_linked_reference_target`, `keymap_bridge_target`, `layout_target`                                |
| `sandbox_class`                  | `read_only_walkthrough`, `mutates_workspace_after_ack`, `mutates_in_isolated_sandbox`, `blocked_until_trust_granted`                                         |
| `reversibility_class`            | `reversible_via_command`, `reversible_via_undo_history`, `irreversible_requires_explicit_ack`                                                                |
| `privacy_scope_class`            | `local_only_machine_bound`, `profile_portable`, `classroom_managed_pool`, `facilitator_only`, `audience_visible`                                             |
| `ownership_class` (speaker note) | `presenter_owned_local`, `co_presenter_visible`, `facilitator_only_visible`, `facilitator_audience_visible`, `audience_visible`                              |
| `derivation_class`               | `primary`, `derived`, `derived_with_upstream_anchors` (re-export of `derivation` from `schemas/docs/citation_anchor.schema.json`)                            |
| `session_class`                  | `facilitator_led_walkthrough`, `classroom_managed_session`, `live_pair_review`, `architecture_map_walkthrough`, `release_or_demo_walkthrough`, `local_only_dry_run` |
| `follow_state_class`             | `no_follow_local_only`, `presenter_broadcast_follow`, `optional_follow_audience_choice`, `audience_pinned_to_presenter`, `follow_blocked_by_envelope`        |

### 2.2 Re-exported vocabularies (do **not** re-mint)

These vocabularies are re-exported verbatim from upstream schemas
and never re-minted on this contract:

- `guided_surface_kind`, `guidance_authority_class`,
  `dismissal_class`, `reset_class`, `progress_export_class`,
  `freshness_class`, `suppression_cause_class`,
  `teaching_session_adjunct_class`, `learning_mode_profile_class`,
  `exercise_mutation_posture` — from
  `schemas/ux/guided_surface_state.schema.json`.
- `tour_audience_class`, `indexing_readiness_class`,
  `layout_target_class`, `partial_indexing_disclosure_class`,
  `source_language_fallback_class`, `evidence_hook_class` — from
  `schemas/ux/guided_tour.schema.json`.
- `share_state_class`, `replay_retention_class`,
  `audience_visibility_class`, `speaker_note_locality_class`,
  `presenter_bar_visibility_class`,
  `local_remote_shared_boundary_class`, `breakaway_recovery_class`,
  `reduced_motion_posture_class`, `accessibility_keyboard_class` —
  from `schemas/ux/presentation_mode_state.schema.json`.
- `state_portability_class` — from
  `schemas/ux/onboarding_portability_state.schema.json`.
- `deployment_profile_id` — from
  `artifacts/governance/deployment_profiles.yaml`.
- `redaction_class` — from
  `schemas/governance/capability_lifecycle.schema.json` (ADR-0011).
- `policy_context` — from
  `schemas/docs/citation_anchor.schema.json`.

## 3. Stable target refs over raw coordinates

Rules (frozen):

1. **Every authored artifact pins a stable target ref.** A
   `tour_step_artifact` or `exercise_step_artifact` whose `target_ref`
   resolves through none of `command_id_target`,
   `docs_citation_anchor_target`,
   `symbol_linked_reference_target`, `keymap_bridge_target`, or
   `layout_target` is non-conforming
   (`denial_reason = learning_artifact_target_ref_not_canonical`).
2. **Raw screen coordinates are forbidden.** A target ref MUST NOT
   carry pixel coordinates, rect bounds, or window-local positions;
   raw coordinates drift on density / zoom / theme / splitter
   resize. Layout targets resolve through the closed
   `layout_target_class` set on the upstream guided-tour contract
   (`start_center`, `command_palette`, `primary_editor`, etc.).
3. **Architecture-map targets ride the symbol-linked-reference lane.**
   A target ref into the architecture map canvas MUST populate
   `symbol_linked_reference_target.symbol_linked_reference_ref`; a
   private graph-node payload as primary target is non-conforming.

## 4. Citations where promised

Rules (frozen):

1. **Glossary items always carry a primary citation.** A
   `glossary_pack_item_artifact` whose
   `primary_docs_citation_anchor_ref` is null is non-conforming
   (`denial_reason = learning_artifact_glossary_item_uncited`).
2. **Paraphrased bodies retain upstream anchors.** A `tour_step_artifact`
   or `exercise_step_artifact` whose `body_summary` paraphrases or
   summarizes a docs source MUST populate
   `primary_docs_citation_anchor_ref` and, when more than one source
   contributed, `upstream_citation_anchor_refs`. A surface that
   promises a citation chain and ships none is non-conforming
   (`denial_reason = learning_artifact_promised_citation_missing`).
3. **Forbidden citation targets are forbidden.** An artifact MUST NOT
   cite an anchor whose `reuse_surfaces` does not include one of
   `onboarding`, `glossary_card`, `ai_explanation_overlay`,
   `docs_pane`, or `hosted_review_evidence` — the same rule the
   learnability contract applies. Flattening a support-bundle-only or
   signing-evidence-only anchor into a teaching artifact is
   non-conforming
   (`denial_reason = learning_artifact_cites_forbidden_target`).

## 5. Allowed actions, sandbox, reversibility

Rules (frozen):

1. **No hidden write paths.** Every value in
   `tour_step_artifact.allowed_action_refs` and
   `exercise_step_artifact.allowed_action_refs` MUST be a registry
   `command_id_ref`. A pack-private "fix this for me" verb that is
   not a governed command on the registry is non-conforming
   (`denial_reason = learning_artifact_hidden_write_path`).
2. **Actions cross trust and approval like everything else.** An
   exercise that names a registry command requiring a workspace-trust
   prompt or an admin step-up MUST honor that prompt at render time;
   the artifact does not mint its own bypass. The artifact's
   `sandbox_class` declares whether mutation is gated on trust
   acknowledgement (`blocked_until_trust_granted` or
   `mutates_workspace_after_ack`).
3. **Mutating exercises declare reversibility.** An
   `exercise_step_artifact` whose `sandbox_class` is one of
   `mutates_workspace_after_ack` or `mutates_in_isolated_sandbox`
   MUST populate `reversibility_class`. A mutating exercise with no
   reversibility declaration is non-conforming
   (`denial_reason = learning_artifact_mutating_exercise_without_reversibility`).
4. **`mutation_posture` agrees with `sandbox_class`.** A
   `mutation_posture = read_only_walkthrough` requires
   `sandbox_class = read_only_walkthrough`; a
   `mutation_posture = blocked_until_trust_granted` requires
   `sandbox_class = blocked_until_trust_granted`; otherwise the
   row is non-conforming
   (`denial_reason = learning_artifact_mutation_posture_mismatch`).

## 6. Completion detectors

Rules (frozen):

1. **Every step declares a completion detector.** A
   `tour_step_artifact` or `exercise_step_artifact` whose
   `completion_detector` is null is non-conforming
   (`denial_reason = learning_artifact_completion_detector_missing`).
2. **Mutating exercise steps complete via command invocation.** An
   `exercise_step_artifact` whose `sandbox_class` is one of
   `mutates_workspace_after_ack` or `mutates_in_isolated_sandbox`
   MUST set `completion_detector.detector_kind =
   command_invocation_detector` and pin the same `command_id_ref`
   the action was authored against, so the analytics layer, the
   shell, and the user agree on completion
   (`denial_reason = learning_artifact_mutating_exercise_completion_detector_mismatch`).
3. **Time-window detectors carry a max-duration.** A
   `time_window_detector` MUST populate
   `max_duration_seconds` (>0); a window with no cap is
   non-conforming.

## 7. Tip intensity, jargon level, and privacy scope

Rules (frozen):

1. **Tip intensity narrows under classroom-managed and policy
   envelopes.** A `tour_step_artifact` whose
   `tip_intensity_class = mandatory_step_required` is admissible
   only on the `learning_mode_classroom_managed_pool` or
   `facilitator_led_walkthrough` profiles; an artifact that pins
   mandatory intensity outside those profiles is non-conforming.
2. **Jargon level is explicit per artifact.** Every artifact whose
   shape carries `jargon_level_class` MUST populate it; rendering
   surfaces filter / re-order content by level rather than guessing.
3. **`privacy_scope_class` resolves through portable lanes only.**
   A `glossary_pack_item_artifact` or `tour_step_artifact` whose
   `privacy_scope_class = profile_portable` MUST be admissible on
   `individual_local`, `self_hosted`, or `enterprise_online`
   deployment profiles; pinning `profile_portable` against an
   air-gapped-only envelope is non-conforming.

## 8. Speaker-note ownership and share state

Rules (frozen):

1. **Notes always declare ownership.** A `speaker_note_artifact`
   with a null `ownership_class` is non-conforming.
2. **Audience-visible notes resolve through the audience-visibility
   lane.** A note whose `ownership_class = audience_visible` MUST
   set `share_state_class` to one of
   `shared_to_named_audience`, `shared_to_classroom_managed_pool`,
   or `shared_to_facilitator_only`; a note authored as
   audience-visible against `local_only_no_share` is
   non-conforming
   (`denial_reason = learning_artifact_speaker_note_share_state_mismatch`).
3. **Speaker notes preserve command identity on handoff.** A
   `speaker_note_artifact` whose paired step pins a
   `command_id_ref` MUST set `preserved_command_id_ref` to the same
   id; otherwise the note can drift away from the registry verb the
   audience-facing step teaches
   (`denial_reason = learning_artifact_speaker_note_command_identity_lost`).

## 9. Teaching-session follow / replay

Rules (frozen):

1. **Follow state never confers mutation.** A
   `teaching_session_artifact` carries a `follow_state_class` that
   reads as a posture, not as authority. Mutating control rides a
   `control_grant_record` on the collaboration schema and never the
   teaching-session artifact.
2. **Share state matches the envelope.** A
   `share_state_class = shared_to_classroom_managed_pool` is
   admissible only on `enterprise_online` or `managed_cloud`
   deployment profiles; an air-gapped envelope MUST resolve
   `shared_unavailable_in_envelope`.
3. **Replay retention is declared.** A
   `replay_retention_class` of `replay_blocked_by_policy` or
   `classroom_artifact_privacy_bounded` MUST attach an evidence-hook
   reserve so a later audit can be qualified.

## 10. Portability and versioning

Rules (frozen):

1. **Artifacts are versioned independently of UI shells.** Each
   schema carries an integer `*_artifact_schema_version`; bumping
   the shell's render version does not bump the artifact version.
2. **Artifact bytes are inspectable.** Every required field is a
   typed scalar, opaque id, short label, or closed-enum value;
   raw URLs, raw paths, raw prompt / completion text, and raw
   credential material never cross this boundary.
3. **Artifacts ship with declared provenance.** A
   `pack_ref` and `pack_revision_ref` pin every artifact to the
   pack revision it was authored in so a reviewer can reconstruct
   the as-shipped state.
4. **Privacy scope drives export.** `privacy_scope_class =
   local_only_machine_bound` artifacts MUST NOT be packaged into a
   portable profile package; `profile_portable` artifacts MUST
   project through the `state_portability_class` lane.

## 11. Acceptance mapping

| Acceptance clause                                                                                                              | Resolved by                                                                                                                                                                          |
|--------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Learning artifacts are portable, inspectable, and versioned independently of one-off UI implementations.                      | §1 record kinds (each with `*_artifact_schema_version`), §10 portability rules, fixture set in `/fixtures/ux/learning_artifacts/`.                                                   |
| Reviewers can tell which learning objects are local/private, shareable, exportable, or policy-managed.                        | §2.1 `privacy_scope_class`, §8 speaker-note ownership / share state, §9 teaching-session share / replay rules, §10 portability.                                                      |
| The object model is aligned enough with docs/help and presentation evidence to avoid parallel schemas later.                  | §2.2 re-exported vocabularies (no re-minting), §1.1 / §1.2 docs citation anchor refs, §1.5 / §1.6 follow / replay / speaker-note locality refs from upstream presentation contract.  |
| Object refs are preferred over raw coordinates, citations are required where promised, hidden write paths are forbidden.      | §3 stable target refs, §4 citations where promised, §5 allowed actions / sandbox / reversibility.                                                                                    |

## 12. Adding a new vocabulary value

Adding a new `jargon_level_class`, `tip_intensity_class`,
`completion_detector_kind`, `target_ref_kind`, `sandbox_class`,
`reversibility_class`, `privacy_scope_class`, `ownership_class`,
`session_class`, `follow_state_class`, or `denial_reason` is
**additive-minor** and bumps the relevant
`*_artifact_schema_version`. Repurposing an existing value is
**breaking** and requires a new decision row on the launch decision
register. A consumer surface that resolves a value it does not
recognize MUST deny with the matching
`*_artifact_schema_version_lagging` reason rather than silently
fall back.

## 13. Worked examples

Fixtures under
[`/fixtures/ux/learning_artifacts/`](../../fixtures/ux/learning_artifacts/)
cover:

1. **Glossary pack item with primary citation** —
   `glossary_pack_item_workspace_trust.yaml`. A
   `glossary_pack_item_artifact` for the workspace-trust term, citing
   a `glossary_term_anchor` on the docs citation model, jargon level
   `intermediate`, `profile_portable` privacy scope.
2. **Tour step landing on a canonical layout target** —
   `tour_step_open_folder.yaml`. A `tour_step_artifact` whose
   `target_ref` is a `layout_target` of `start_center`, completion
   detector `command_invocation_detector` pinning
   `cmd:core:open_folder`, allowed action set `[cmd:core:open_folder]`,
   tip intensity `gentle_hint`, jargon level `beginner`.
3. **Tour step body paraphrased with citation chain** —
   `tour_step_indexing_paraphrased.yaml`. A `tour_step_artifact`
   whose body summarizes a docs page; `derivation_class` is
   `derived_with_upstream_anchors`, `upstream_citation_anchor_refs`
   names two authoritative anchors.
4. **Exercise step with reversible mutation** —
   `exercise_step_format_document_reversible.yaml`. An
   `exercise_step_artifact` whose `sandbox_class` is
   `mutates_workspace_after_ack`, `reversibility_class` is
   `reversible_via_undo_history`, `mutation_posture` is
   `mutates_workspace_after_ack`, completion detector pins the same
   `command_id_ref` as the allowed action.
5. **Exercise step blocked until trust granted** —
   `exercise_step_blocked_until_trust.yaml`. An
   `exercise_step_artifact` whose `sandbox_class` is
   `blocked_until_trust_granted`, `mutation_posture` is
   `blocked_until_trust_granted`, with no allowed actions until
   trust is acknowledged.
6. **Speaker note preserving command identity** —
   `speaker_note_open_folder_preserves_identity.yaml`. A
   `speaker_note_artifact` paired with the open-folder tour step
   whose `preserved_command_id_ref` equals the paired step's command
   id, ownership `facilitator_only_visible`, share state
   `shared_to_named_audience`.
7. **Speaker note hidden from the audience under local-only share** —
   `speaker_note_facilitator_only_local.yaml`. A
   `speaker_note_artifact` whose `ownership_class` is
   `presenter_owned_local`, `share_state_class` is
   `local_only_no_share`, paired with a dry-run rehearsal step.
8. **Shared teaching session bundle** —
   `teaching_session_shared_classroom.yaml`. A
   `teaching_session_artifact` for a classroom managed session;
   `share_state_class` is `shared_to_classroom_managed_pool`,
   `audience_visibility_class` is
   `audience_sees_workspace_plus_agenda_rail`,
   `replay_retention_class` is `classroom_artifact_privacy_bounded`,
   `follow_state_class` is `presenter_broadcast_follow`.
9. **Local-only dry-run teaching session** —
   `teaching_session_local_only_dry_run.yaml`. A
   `teaching_session_artifact` whose `session_class` is
   `local_only_dry_run`, `share_state_class` is
   `local_only_no_share`, `follow_state_class` is
   `no_follow_local_only`, `replay_retention_class` is
   `no_replay_retained`.
10. **Portable learning-mode profile artifact** —
    `learning_mode_profile_default_individual_learner.yaml`. A
    `learning_mode_profile_artifact` for the default individual
    learner; declares the twelve learnability invariants, default
    jargon level `intermediate`, default tip intensity `gentle_hint`,
    bound glossary / tour / command-tip / docs packs, and the
    portable progress-export lane.
11. **Air-gapped offline learning-mode profile** —
    `learning_mode_profile_air_gapped_offline.yaml`. A
    `learning_mode_profile_artifact` whose `profile_class` is
    `air_gapped_offline_pack`; `state_portability_class` forbids
    portable export, `default_share_state_class` is
    `shared_unavailable_in_envelope`.

## 14. Out of scope at this revision

- Shipping the actual content packs (glossary entries, tour bodies,
  exercise narratives, presentation scripts). This contract freezes
  the artifact shape; pack content lands in later product work and
  rides this contract.
- Final visuals (card padding, tour-step illustration, presentation
  layout). The design-system style guide and the presentation-mode
  contract own those.
- Author tooling (pack editors, importers, signing pipelines). The
  contract names the artifact shape; downstream tooling consumes it.
- Actual policy-bundle definitions that disable or narrow learning
  artifacts. The identity / policy-bundle contracts own those; this
  contract names the typed envelope and privacy scope.
