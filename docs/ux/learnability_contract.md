# Learnability, guided-surface, and citation-backed explainer contract

This document freezes the cross-surface contract every Aureline
**onboarding card**, **glossary card**, **guided-tour step**,
**architecture explainer**, **contextual tip**, **keymap-bridge hint**,
**exercise step**, **learning-mode profile**, and **speaker-note /
teaching-session adjunct** resolves through before the shell starts
rendering teaching content. The goal is a teaching surface that stays
truthful, dismissible, and bounded by the same command and evidence
model as the rest of the product — so a reviewer can map a guided tip,
a glossary card, and a presentation step back to the same command and
citation truth every other row reads, with no special-case exception.

The machine-readable schema lives at:

- [`/schemas/ux/guided_surface_state.schema.json`](../../schemas/ux/guided_surface_state.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/learnability_cases/`](../../fixtures/ux/learnability_cases/)

This contract is normative for the projection, citation, dismissal,
reset, export, freshness, and suppression posture of learnability
surfaces. Where it disagrees with the PRD, TAD, TDD, UI/UX spec, or
milestone document, those sources win and this document plus its
companion schema and fixtures update in the same change. Where a
downstream tour, glossary, explainer, tip, exercise, learning-mode,
or teaching-session surface mints a parallel vocabulary, this
contract wins and the surface is non-conforming.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference:

- [`/docs/ux/start_center_contract.md`](./start_center_contract.md)
  and
  [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json)
  — Start Center zones, `primary_action_id`, and first-launch row
  disclosure. Onboarding cards that appear on the startup wedge are
  wrapped rows on that contract; this contract binds their teaching
  payload rather than their startup-wedge placement.
- [`/docs/ux/no_account_local_entry_contract.md`](./no_account_local_entry_contract.md)
  and
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)
  — `entry_surface_family`, `account_prompt_class`,
  `boundary_crossing_class`, `state_portability_class`, `reset_class`,
  `export_class`, `profile_scope_class`, and the onboarding-portability
  manifest. Tour-progress, dismissal, and learning-mode profile state
  declared here ride that contract's portability table; no guided
  surface mints its own portability lane.
- [`/docs/product/onboarding_measurement_plan.md`](../product/onboarding_measurement_plan.md)
  — measurement-surface vocabulary, readiness buckets, and
  first-useful-work event names guided-tour steps and exercise steps
  cite without re-inventing a teaching-only event model.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md),
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json),
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  — canonical `command_id`, alias lifecycle, keybinding refs, and
  discoverability projection refs. Every guided surface that claims
  it explains an Aureline action resolves that claim through a stable
  command id on the registry.
- [`/docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md),
  [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json),
  and
  [`/schemas/docs/symbol_linked_reference.schema.json`](../../schemas/docs/symbol_linked_reference.schema.json)
  — `docs_citation_anchor_record` (including `glossary_term_anchor`,
  `onboarding_step_anchor`), `derivation`, `source_class`,
  `freshness_class`, `version_match_state`, `reuse_surface`, and
  `export_posture`. Every citation a guided surface projects resolves
  through that object model; derived explanations retain their upstream
  anchors rather than flattening to prose-only authority.
- [`/docs/ux/keybinding_resolver_contract.md`](./keybinding_resolver_contract.md)
  and
  [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
  — keymap-bridge and legacy-keymap import vocabulary. Keymap-bridge
  hints cite a resolver record rather than inventing a one-off
  mutation path that would widen the command surface.
- [`/docs/ux/transient_surface_contract.md`](./transient_surface_contract.md)
  and
  [`/schemas/ux/transient_surface.schema.json`](../../schemas/ux/transient_surface.schema.json)
  — transient preview primitives. A contextual tip that renders as a
  tooltip, hovercard, popover, or peek composes by reference with the
  transient-preview record for its pointer / keyboard / touch route;
  this contract binds only the teaching payload.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary guided surfaces resolve against
  so air-gapped, managed-cloud, and restricted envelopes inherit
  suppression posture mechanically.

## Who reads this contract

- **Onboarding, tour, glossary, explainer, and learning-mode
  authors** — to emit teaching rows against one record shape whose
  authority class, citation backing, dismissal model, reset scope,
  export posture, freshness, and suppression cause are declared.
- **Docs, help, and AI-explanation authors** — to reuse the same
  citation object model for glossary cards, onboarding steps,
  architecture explainers, and AI-assisted explainer overlays without
  flattening them into uncited prose.
- **Presentation / facilitator / teaching-session authors** —
  to render speaker-note and audience-visible cues without losing the
  command id or docs anchor that backs the action being taught.
- **Support, admin-envelope, and policy authors** — to suppress
  teaching surfaces that cannot be honoured under the current envelope
  (policy-disabled learning, locale unavailable, docs pack missing,
  imported teaching pack unverified) with a typed cause rather than
  silent omission.

## Why this exists

Without this contract, teaching surfaces drift the fastest:

- an onboarding card mints its own "start here" verb instead of
  referencing the canonical `command_id`, so the card keeps working
  after the underlying command is renamed, disabled, or split —
  teaching a user an action that no longer exists or landed on a
  different route;
- a glossary card copies a one-line definition with no pack revision,
  locale, or freshness state, so a reviewer cannot tell which revision
  of which pack the user saw;
- a guided-tour step narrates a multi-step workflow as prose and
  loses the evidence chain back to the canonical sources, violating
  the citation contract rule that derived explanations retain upstream
  anchors;
- an architecture explainer opens a one-off private mutation path
  (a hidden "fix up for me" button) that is not a governed command
  and has no disabled-reason, approval, or audit story;
- a contextual tip survives after its cited docs pack has been
  mirrored-only or withdrawn, rendering as if the content were live;
- a keymap-bridge hint lands on a legacy-keymap verb that was never
  promoted to a canonical alias on the registry, creating a parallel
  keybinding space;
- an exercise step mutates files before the exercise has acknowledged
  the workspace trust posture;
- a speaker-note adjunct in presentation mode strips the command id
  and docs anchor, so what's on stage no longer maps to what a
  reviewer can trace back to command and docs truth;
- a learning-mode profile declares its own "tour progress" and
  "dismissal" bookkeeping outside the onboarding-portability manifest,
  so tour state leaks across accounts or fails to follow a portable
  profile package;
- an imported teaching pack (classroom, conference handout, offline
  bundle) renders as authoritative with no verification or freshness
  posture.

This contract closes all of those gaps by declaring one record per
guided-surface instance, one rule row per surface kind, one
learning-mode profile shape, one closed denial-reason set the
publisher MUST emit when any of the above fails, and one explicit
binding to the command-registry and citation-anchor truth model so
teaching content survives label churn, pack revisions, keymap import,
locale fallback, policy narrowing, and presentation handoff without
drifting into its own authority story.

## 1. Record kinds

### 1.1 `guided_surface_state_record`

One structured record per **(surface_kind, surface_id,
presented_at)** tuple. Emitted by the surface rendering the guided
content. Every record carries:

- **identity** — `surface_id`, `surface_kind`
  (`onboarding_card`, `glossary_card`, `guided_tour_step`,
  `architecture_explainer`, `contextual_tip`, `keymap_bridge_hint`,
  `exercise_step`, `learning_mode_profile_entry`,
  `speaker_note_adjunct`, `teaching_session_adjunct`).
- **authority** — `authority_class` in the closed set
  `{command_id_anchored, docs_anchor_anchored,
  symbol_reference_anchored, keymap_bridge_anchored,
  derived_with_upstream_anchors, aggregate_learning_mode_entry}`.
  `unanchored` is not a value; a surface that cannot declare an
  authority MUST deny rather than render.
- **primary anchor** — exactly one typed primary anchor identifying
  the row the surface is teaching against:
  - `command_id_ref` — canonical id on the command registry
    (from `command_registry_entry.schema.json`);
  - `docs_citation_anchor_ref` — anchor id resolving against
    `docs_citation_anchor_record`;
  - `symbol_linked_reference_ref` — symbol-linked-reference id; or
  - `keymap_bridge_ref` — resolver id for a legacy-to-canonical
    keybinding hint.
- **upstream anchors** — `upstream_citation_anchor_refs` (array
  of anchor ids). Empty for non-derived surfaces; non-empty for
  `authority_class = derived_with_upstream_anchors`, mirroring the
  citation-anchor derivation rule.
- **docs-pack and locale** — `docs_pack_ref`,
  `docs_pack_revision_ref`, `locale` (BCP-47), and
  `locale_fallback_disclosed` (boolean).
- **freshness** — `freshness_class` in `{live_authoritative,
  cached_current, stale_past_ttl, cached_mirror_only,
  unverified_upstream}` at mint time, plus
  `version_match_state_at_mint` recorded against the running build.
- **dismissal / reset / export** — `dismissal_class`, `reset_class`,
  `progress_export_class` (re-projected from the onboarding-portability
  vocabulary so no surface invents parallel lanes).
- **suppression** — `active` (boolean) and, when `active = false`,
  exactly one `suppression_cause_class` from the closed set in §5.
- **teaching-session adjunct class** — non-null when
  `surface_kind ∈ {speaker_note_adjunct, teaching_session_adjunct}`;
  MUST preserve the primary command id or docs anchor across the
  presentation handoff.
- **exercise posture** — non-null when
  `surface_kind = exercise_step`; `exercise_mutation_posture` is one
  of `read_only_walkthrough`, `mutates_workspace_after_ack`, or
  `blocked_until_trust_granted`.
- **deployment / policy / redaction** — `deployment_profile_refs`
  (which envelopes this surface is admissible under),
  `policy_context` (policy epoch and trust state from the citation
  contract), and `redaction_class` (from ADR-0011 via the citation
  contract).
- **measurement linkage** — optional
  `measurement_surface_ref` pointing at the onboarding-measurement
  surface vocabulary so tour and exercise steps can be qualified by
  the same scoreboard rows as the rest of first-run.
- **timestamps** — `minted_at` (monotonic UTC).

### 1.2 `guided_surface_rule_record`

One row per `surface_kind`. Freezes the citation posture, admissible
authority classes, required primary anchor kinds, dismissal and reset
posture, maximum freshness, allowed teaching-adjunct classes, and the
command-identity-preservation rule that binds the surface. The shell
MUST resolve every `guided_surface_state_record` against exactly one
`guided_surface_rule_record` for its surface kind and reject any
surface that violates the rule's allowed sets.

### 1.3 `learning_mode_profile_record`

One record per learning-mode profile (default individual learner,
classroom managed pool, facilitator-led session, air-gapped offline
pack, privacy-reduced local-only, policy-disabled). Names the
`profile_class`, the set of `active_surface_kinds`, the
`disabled_surface_kinds` under this envelope, the allowed
`progress_export_class`es, the dismissal and reset scope, the
deployment profiles the profile is admissible under, and the
invariants (§7) declared const-true on the row.

## 2. Surface kinds

`guided_surface_kind` is closed at ten values:

| Kind                           | Intent                                                                                                                                        |
|--------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
| `onboarding_card`              | A first-run or re-entry card teaching a Start Center / switcher row; MUST bind to a `primary_action_id` via a registry `command_id`.          |
| `glossary_card`                | A single-term definition card; MUST cite a `glossary_term_anchor` on the docs citation model (`source_class` authoritative).                  |
| `guided_tour_step`             | One step on a multi-step tour; MUST cite an `onboarding_step_anchor` and, if the step describes an action, a `command_id_ref`.                |
| `architecture_explainer`       | A structural explainer about a subsystem; MUST cite a `symbol_linked_reference_ref` and SHOULD retain upstream anchors when AI-assisted.      |
| `contextual_tip`               | An anchored tip on a focused surface; composes with `transient_preview_record` for its render. MUST cite a docs anchor or a command id.       |
| `keymap_bridge_hint`           | A "you pressed X in your prior keymap — in Aureline this is Y" hint; MUST cite a `keymap_bridge_ref` resolving to the canonical command id.   |
| `exercise_step`                | A single step of a guided exercise; MUST declare `exercise_mutation_posture`; MUST NOT mutate the workspace before trust is acknowledged.    |
| `learning_mode_profile_entry`  | A row on a learning-mode profile landing; MUST reference a `learning_mode_profile_record` and aggregate other entries by their opaque ids.   |
| `speaker_note_adjunct`         | A facilitator-visible speaker note paired with a presentation step; MUST preserve the command id and docs anchor the audience-facing step teaches. |
| `teaching_session_adjunct`     | An audience-visible teaching cue (exercise-cue caption, handoff note) paired with a presentation step; same command-identity-preservation rule. |

## 3. Authority classes and primary anchors

`guidance_authority_class` is closed at six values:

- `command_id_anchored` — the primary anchor is a
  `command_id_ref` drawn from the command-registry seed. The
  surface is teaching a canonical action; the id survives label
  churn, keymap import, and CLI projection.
- `docs_anchor_anchored` — the primary anchor is a
  `docs_citation_anchor_ref`. The surface is teaching a concept
  defined in a docs pack, a glossary, a runbook, or a generated
  reference.
- `symbol_reference_anchored` — the primary anchor is a
  `symbol_linked_reference_ref`. The surface is teaching a
  structural concept bound to symbols or code spans.
- `keymap_bridge_anchored` — the primary anchor is a
  `keymap_bridge_ref` that resolves to a canonical command id on
  the registry. The bridge hint MUST NOT land on a non-canonical
  verb; if the canonical id is missing, the surface suppresses with
  `keymap_bridge_unresolved`.
- `derived_with_upstream_anchors` — the primary anchor is a
  derived citation anchor (`source_class = derived_explanation`)
  AND `upstream_citation_anchor_refs` names at least one anchor
  whose `source_class` is authoritative per the citation contract.
  AI-assisted explainers, paraphrased glossary cards, and
  synthesized architecture overviews MUST take this class; they
  MUST NOT present as primary authority.
- `aggregate_learning_mode_entry` — reserved for
  `learning_mode_profile_entry` rows that aggregate other entries by
  opaque id. The entry MUST reference its aggregated entries by id;
  it does not project its own narrative anchor.

Rules (frozen):

1. **Every guided surface cites at least one governed anchor.** A
   `guided_surface_state_record` whose typed primary anchor is null
   is non-conforming
   (`denial_reason = learnability_surface_without_command_or_docs_anchor`).
2. **Derived surfaces retain their upstream anchors.** A surface
   whose `authority_class = derived_with_upstream_anchors` and whose
   `upstream_citation_anchor_refs` is empty is non-conforming
   (`denial_reason = learnability_surface_derived_without_upstream`).
3. **Forbidden targets are forbidden.** A surface MUST NOT cite an
   anchor whose `reuse_surfaces` does not include one of
   `onboarding`, `glossary_card`, `ai_explanation_overlay`,
   `docs_pane`, or `hosted_review_evidence` — flattening a
   support-bundle-only or signing-evidence-only anchor into a
   teaching card is non-conforming
   (`denial_reason = learnability_surface_cites_forbidden_target`).
4. **No hidden mutation paths.** A surface MUST NOT declare or link
   to a one-off private action that is not a governed command on the
   registry. Primary-action buttons on onboarding cards, inline
   actions on explainers, and "apply suggestion" controls on exercise
   steps all resolve through a `command_id_ref`; anything else is
   non-conforming
   (`denial_reason = learnability_surface_opens_private_mutation_path`).

## 4. Dismissal, reset, export, and portability

`dismissal_class` is closed at five values:

- `per_session_one_shot` — the surface dismisses for the current
  session only; no persisted record.
- `per_profile_dismissable` — dismissal persists on the portable
  profile package; follows the user across devices.
- `per_device_dismissable` — dismissal persists on the device only;
  does not leak across devices (classroom-managed pools, sensitive
  environments).
- `policy_locked` — dismissal is governed by the active policy
  bundle; the user cannot dismiss locally.
- `not_dismissable` — the surface does not expose a dismiss
  affordance (typically `learning_mode_profile_entry`).

`reset_class` and `progress_export_class` re-project the
onboarding-portability vocabulary
(`resettable_per_profile`, `resettable_per_device`,
`resettable_per_account`, `resettable_by_policy`,
`not_resettable_locally`; `in_portable_profile_package`,
`in_portable_profile_package_redacted`,
`not_exported_machine_local`, `in_support_bundle_redacted`,
`blocked_by_policy`) — no parallel lanes.

Rules (frozen):

1. **Dismissal is explicit.** A `guided_surface_state_record` with
   a null `dismissal_class` is non-conforming
   (`denial_reason = learnability_surface_dismissed_state_collapsed`).
2. **Portable profile state never leaks account- or policy-scoped
   data.** A surface whose `dismissal_class = per_profile_dismissable`
   and whose `progress_export_class` is neither
   `in_portable_profile_package` nor
   `in_portable_profile_package_redacted` is non-conforming
   (`denial_reason = learnability_surface_progress_export_class_mismatch`).
3. **Policy-locked surfaces cite the bundle.** A surface whose
   `dismissal_class = policy_locked` MUST carry a non-null
   `policy_context.policy_epoch`; otherwise
   `learnability_surface_policy_disabled_still_rendered` applies.

## 5. Freshness, suppression, and absence

`freshness_class` is closed at five values:

- `live_authoritative` — the cited pack revision and command
  registry entry match the running build exactly; the surface renders
  as current.
- `cached_current` — the surface is projected from a cache that is
  still within TTL and the upstream pack is reachable.
- `stale_past_ttl` — TTL has expired; the surface MUST disclose a
  freshness hint or suppress.
- `cached_mirror_only` — the deployment profile is air-gapped or
  mirror-only; the surface is allowed to render against the mirror
  snapshot with the mirror posture disclosed.
- `unverified_upstream` — the upstream pack could not be verified
  (imported teaching pack, offline-bundle relay); the surface MUST
  suppress unless the imported pack has passed verification.

`suppression_cause_class` is closed at twelve values and MUST be
populated when `active = false`:

- `stale_citation_superseded` — the cited anchor has been superseded
  (pack revision bump, renamed-refactor); the surface MAY offer the
  migrate-to-replacement repair hook.
- `docs_pack_missing` — the docs pack is not installed on this
  device and the cited anchor cannot resolve locally.
- `locale_unavailable` — the requested locale is not covered by
  the pack revision the surface resolves against; a
  `locale_fallback_disclosed = true` fallback is required if
  the surface still renders.
- `policy_disabled_learning_surface` — the active policy bundle
  disables learning surfaces of this kind.
- `imported_teaching_pack_unavailable` — a classroom / conference /
  offline teaching pack referenced by the learning-mode profile is
  missing, unverified, or revoked.
- `incompatible_build_drift` — `version_match_state_at_mint` is
  `incompatible_drift_detected`.
- `exercise_step_blocked_by_workspace_trust` — the exercise step
  would mutate the workspace but workspace trust has not been
  acknowledged.
- `keymap_bridge_unresolved` — the keymap-bridge hint cannot resolve
  to a canonical command on the registry (alias removed, command
  retired without replacement).
- `command_id_missing` — the `command_id_ref` does not resolve on
  the current registry revision.
- `command_id_deprecated_without_replacement` — the command is on
  the registry but its lifecycle is retired and no replacement is
  published.
- `command_id_disabled_in_current_policy` — the command is disabled
  by the active policy bundle; the teaching surface MUST NOT render
  as if the action were available.
- `teaching_session_expired` — a facilitator-led session window has
  elapsed; speaker-note and teaching-session adjuncts suppress.

Rules (frozen):

1. **A surface MUST declare its freshness.** A
   `guided_surface_state_record` with a null `freshness_class` is
   non-conforming.
2. **Stale content is never presented as live.** A surface whose
   `freshness_class ∈ {stale_past_ttl, cached_mirror_only,
   unverified_upstream}` that renders without its freshness
   disclosure is non-conforming
   (`denial_reason = learnability_surface_citation_stale_presented_as_live`).
3. **Locale fallbacks are disclosed.** A surface that falls back
   to a non-requested locale MUST set
   `locale_fallback_disclosed = true`; otherwise
   `learnability_surface_locale_fallback_not_disclosed` applies.
4. **Missing docs packs suppress openly.** A surface that cannot
   resolve its cited pack locally MUST set `active = false` with
   `suppression_cause_class = docs_pack_missing`; rendering silently
   from a stale or partial local snapshot is non-conforming
   (`denial_reason = learnability_surface_docs_pack_missing_not_disclosed`).
5. **Imported teaching packs are verified.** An imported teaching
   pack rendered with `freshness_class = unverified_upstream` is
   non-conforming
   (`denial_reason = learnability_surface_imported_teaching_pack_unverified`).

## 6. Teaching sessions and presentation mode

Presentation and teaching surfaces MUST keep the same command and
citation truth as the rest of the product. The rule is uniform:

- A `speaker_note_adjunct` that names an action MUST carry the same
  `command_id_ref` as its audience-visible step, and MUST preserve
  the primary docs anchor (when present).
- A `teaching_session_adjunct` that narrates an exercise cue MUST
  cite the same docs anchor or command id the exercise step cites;
  a cue that paraphrases without citation is non-conforming
  (`denial_reason = learnability_surface_presentation_step_loses_command_link`).
- A teaching session whose envelope window has elapsed MUST suppress
  with `teaching_session_expired`; the session does not linger past
  its declared window.

Cross-reference with the first-run onboarding, guided-tour,
architecture-map, and presentation-mode contracts: those surfaces
may declare their own layout, timing, and staging rules, but the
teaching payload they project resolves through one
`guided_surface_state_record` per step, one
`guided_surface_rule_record` per kind, and one
`learning_mode_profile_record` per session so command and citation
truth survive staging and sharing.

## 7. Invariants

Every `learning_mode_profile_record` MUST declare the following
const-true invariants:

1. `every_guided_surface_cites_command_or_docs_anchor` — no record
   has a null typed primary anchor.
2. `derived_guided_surface_retains_upstream` — every derived surface
   has a non-empty `upstream_citation_anchor_refs`.
3. `no_private_mutation_path_outside_command_registry` — every
   inline action resolves to a registry `command_id`.
4. `stale_or_unverified_surface_suppresses_or_discloses` — no
   surface presents stale or unverified content as live.
5. `portable_dismissal_never_leaks_account_or_policy_state` — a
   `per_profile_dismissable` surface exports only through the
   portable-profile lane.
6. `policy_disabled_learning_surface_never_renders` — no surface
   whose envelope has `policy_disabled_learning_surface` set is
   rendered under that envelope.
7. `locale_fallback_is_disclosed` — locale fallbacks carry the
   disclosure flag.
8. `exercise_steps_respect_workspace_trust` — no
   `exercise_step` mutates the workspace until trust is acknowledged.
9. `imported_teaching_pack_is_verified_before_render` — no
   `unverified_upstream` teaching pack renders.
10. `presentation_adjunct_preserves_command_identity` — every
    speaker-note and teaching-session adjunct preserves the command
    id or docs anchor of its audience-visible step.
11. `learning_mode_profile_exports_match_portability_lanes` — profile
    exports resolve through the onboarding-portability export
    vocabulary, not a teaching-only lane.
12. `every_guided_surface_resolves_against_rule_record` — every
    emitted state record names a `guided_surface_rule_record` for
    its surface kind.

## 8. Denial reasons

The following denial reasons are reserved. A shell that would
otherwise render a non-conforming surface MUST deny with the matching
reason rather than silently fall back:

- `learnability_surface_without_command_or_docs_anchor`
- `learnability_surface_derived_without_upstream`
- `learnability_surface_cites_forbidden_target`
- `learnability_surface_opens_private_mutation_path`
- `learnability_surface_citation_stale_presented_as_live`
- `learnability_surface_locale_fallback_not_disclosed`
- `learnability_surface_docs_pack_missing_not_disclosed`
- `learnability_surface_dismissed_state_collapsed`
- `learnability_surface_progress_export_class_mismatch`
- `learnability_surface_policy_disabled_still_rendered`
- `learnability_surface_imported_teaching_pack_unverified`
- `learnability_surface_presentation_step_loses_command_link`
- `learnability_surface_schema_version_lagging`

## 9. Acceptance mapping

| Acceptance clause                                                                                                      | Resolved by                                                                                                                                                  |
|------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Learnability surfaces can be traced back to command ids, docs anchors, or source references instead of floating prose. | §3 authority classes, §1.1 typed primary anchor, invariant 1.                                                                                                |
| Dismissal and progress state behavior is explicit and export-safe.                                                     | §4 dismissal / reset / export classes, invariant 5, invariant 11.                                                                                            |
| Claimed learning or guided surfaces cannot create private privileged actions outside the governed command system.     | §3 rule 4 and invariant 3, denial reason `learnability_surface_opens_private_mutation_path`.                                                                 |
| Reviewer can map a guided tip, glossary card, and presentation step back to the same command and citation truth model.| §1.1 / §3 authority classes, §6 presentation rule, invariant 10, fixture set in `/fixtures/ux/learnability_cases/`.                                          |

## 10. Adding a new vocabulary value

Adding a new `guided_surface_kind`, `guidance_authority_class`,
`dismissal_class`, `reset_class`, `progress_export_class`,
`freshness_class`, `suppression_cause_class`, `citation_posture`,
`teaching_session_adjunct_class`, `learning_mode_profile_class`,
`exercise_mutation_posture`, or `denial_reason` is **additive-minor**
and bumps `guided_surface_schema_version`. Repurposing an existing
value is **breaking** and requires a new decision row on the launch
decision register. A consumer surface that resolves a value it does
not recognize MUST deny with
`learnability_surface_schema_version_lagging` rather than silently
map to a default.

## 11. Worked examples

Fixtures under [`/fixtures/ux/learnability_cases/`](../../fixtures/ux/learnability_cases/)
cover:

1. **Onboarding card anchored to a command id** —
   `onboarding_card_open_folder_command_anchored.json`. A
   `primary_action.open_folder` card bound to the canonical command
   id, `live_authoritative` freshness, `per_profile_dismissable`,
   exported on the portable profile package.
2. **Glossary card backed by a docs anchor** —
   `glossary_card_docs_anchor_backed.json`. A `glossary_term_anchor`
   citation projected into a glossary card, `docs_anchor_anchored`,
   `per_session_one_shot` dismissal.
3. **Guided-tour step citing command + docs anchor** —
   `guided_tour_step_command_and_docs_anchor.json`. An
   `onboarding_step_anchor` paired with a `command_id_ref`,
   `live_authoritative`, measurement-surface ref to
   `surface_first_useful_edit`.
4. **Architecture explainer backed by a symbol-linked reference** —
   `architecture_explainer_symbol_reference.json`. A
   `symbol_linked_reference_ref` plus a derived-explanation
   `upstream_citation_anchor_refs` list; `per_profile_dismissable`.
5. **Contextual tip bound to a keybinding bridge** —
   `contextual_tip_keymap_bridge.json`. A `keymap_bridge_ref` that
   resolves to a canonical command id, rendering as a transient
   hovercard; `per_device_dismissable`.
6. **Exercise step blocked until trust is acknowledged** —
   `exercise_step_blocked_by_trust.json`. `active = false` with
   `exercise_step_blocked_by_workspace_trust`, exercise mutation
   posture `blocked_until_trust_granted`.
7. **Policy-disabled learning surface suppressed** —
   `policy_disabled_learning_surface_suppressed.json`.
   `active = false` with `policy_disabled_learning_surface`, the
   surface is not rendered under the current policy epoch.
8. **Speaker-note adjunct preserves command identity** —
   `speaker_note_adjunct_preserves_command_identity.json`. A
   `teaching_session_adjunct_class = speaker_note_for_architecture_map`
   pairing that retains the same `command_id_ref` and docs anchor
   as its audience-visible step.
9. **Learning-mode profile manifest** —
   `learning_mode_profile_manifest.json`. A
   `learning_mode_profile_record` declaring every const-true
   invariant, the allowed surface kinds, and the export classes for
   the default individual learner profile.

## 12. Out of scope at this revision

- Shipping a complete learning mode in M0. This contract freezes
  the boundary; the actual tour content, glossary, architecture
  map, presentation mode, and learning-mode switching UI land in
  later milestones and ride this contract.
- Final visuals (card padding, tour-step illustration, presentation
  layout). The design-system style guide and the presentation-mode
  contract own those.
- Analytics and scoreboarding beyond the measurement-surface
  linkage. The onboarding-measurement plan owns the qualification
  event names; this contract only names the linkage field.
- Actual policy-bundle definitions that disable or narrow learning
  surfaces. The identity / policy-bundle contracts own those; this
  contract names the typed suppression cause.
