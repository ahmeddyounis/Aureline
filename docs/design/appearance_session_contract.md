# Appearance-session, token-overlay, and live follow-system state contract

This document freezes the **versioned state** every settings UI, theme
switcher, support exporter, QA capture, docs/help cross-link, and
migration flow uses to describe the live appearance session, the
in-effect token overlay, and the per-axis live follow-system posture.

The contract exists so live appearance changes — theme switching,
OS-follow behavior, density toggles, text-scale changes, reduced-motion
escalation, and token overrides — are **explicit, attributable state**
rather than hidden widget-local memory. A reviewer, support engineer,
or migration tool MUST be able to read a record under this contract and
explain (a) what is in effect, (b) where each effective token came
from, and (c) which OS signal a surface is allowed to apply live versus
which one must hold for an explicit confirm or review checkpoint.

The machine-readable schemas live at:

- [`/schemas/design/appearance_session.schema.json`](../../schemas/design/appearance_session.schema.json)
- [`/schemas/design/token_overlay.schema.json`](../../schemas/design/token_overlay.schema.json)

Worked fixtures live under:

- [`/fixtures/design/appearance_session_cases/`](../../fixtures/design/appearance_session_cases/)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, design-system style guide, or the upstream design-token
vocabulary, those sources win and this contract plus the schemas and
fixtures MUST be updated in the same change. Where a downstream surface
mints a parallel session shape, a parallel per-token overlay state
vocabulary, or a parallel "live vs confirm" rule, this contract wins and
the surface is non-conforming.

## Companion contracts

This contract reuses and composes with existing vocabulary by reference
instead of re-minting it:

- [`/docs/design/design_token_component_state_vocabulary.md`](./design_token_component_state_vocabulary.md)
  and
  [`/schemas/design/token_export_manifest.schema.json`](../../schemas/design/token_export_manifest.schema.json)
  own token families, theme classes, density classes, accessibility
  postures, semantic status, and trust-visual-state vocabulary.
- [`/docs/ux/appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md),
  [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json),
  and
  [`/schemas/ux/theme_import_report.schema.json`](../../schemas/ux/theme_import_report.schema.json)
  own the user-facing import-review, checkpoint-mint, and parity-claim
  workflow. This contract is the **state contract** the workflow
  produces; checkpoint and rollback refs cross this boundary by id only.
- [`/docs/ux/theme_and_visual_asset_contract.md`](../ux/theme_and_visual_asset_contract.md)
  and
  [`/schemas/ux/theme_package_manifest.schema.json`](../../schemas/ux/theme_package_manifest.schema.json)
  own theme-package shape; this contract references theme-package and
  theme-revision refs as opaque handles only.
- [`/docs/design/os_appearance_change_matrix.md`](./os_appearance_change_matrix.md)
  and
  [`/artifacts/design/appearance_live_change_matrix.yaml`](../../artifacts/design/appearance_live_change_matrix.yaml)
  own the per-profile OS appearance live-change matrix and the restart/reload
  disclosure posture for surfaces that cannot apply OS-triggered changes fully
  live.

## Why freeze this now

Live appearance changes drift the moment each surface invents its own
in-memory state for "follow OS" or "user picked dark":

- a settings UI remembers a preview dark theme but a webview keeps the
  prior light theme, leaving a half-themed shell with no checkpoint to
  revert against;
- an OS reduced-motion signal flips the shell live but a guided overlay
  keeps motion at standard because the surface relies on its own widget
  memory;
- a workspace overlay supplies a token but a support export cannot tell
  whether the value is `inherited` from the theme, `overridden` by the
  workspace, `deprecated` and waiting on a replacement, or `unmapped`
  with no target token;
- an OS contrast change applies live in the shell but the trust prompt
  needed an explicit confirm step that nobody recorded;
- a fallback chain that resolved the token drifts between releases
  because nobody recorded which scope it actually walked.

The contract forecloses these patterns by treating the appearance
session, the token overlay, and the live follow-system policy as three
distinct records that share one schema version and one cross-record
referencing convention. Once the boundary is frozen, every surface
reads the same in-effect state instead of negotiating field names.

## Scope

- Freeze one `appearance_session_record` shape carrying the active
  theme package and revision, mode (theme class), contrast state,
  accent source, density class, text scale, reduced-motion posture and
  source, follow-system posture, preview state, and checkpoint/rollback
  refs.
- Freeze one `token_overlay_record` shape carrying overlay scope,
  validation result, fallback chain, and the closed per-token
  value-state vocabulary `inherited`, `overridden`, `deprecated`, or
  `unmapped`.
- Freeze one `live_follow_system_policy_record` shape with the closed
  per-axis live-update vocabulary `live_apply_no_review`,
  `live_apply_with_revertable_checkpoint`, `confirm_review_required`,
  and `policy_blocked`. The record names which OS signal each axis
  reacts to.
- Freeze one `appearance_session_revision_event_record` shape so a
  versioned audit trail of live changes — preview start, OS-signal
  change, confirm, revert, rollback — is inspectable instead of being
  inferred from logs.

## Out of scope

- Theme engine, settings UI, signal-watcher, or live-apply pipeline
  implementation. Those compose over this state contract.
- Final product copy. Display copy may render the labels "Inherited",
  "Overridden", "Deprecated", "Unmapped", and the four
  live-follow-system labels; the closed machine values are fixed.
- The full token export manifest body. Token families and theme classes
  cross this boundary by ref through the design-token vocabulary
  schema.
- The full theme-import or token-overlay-import report body. Those
  remain owned by the M00-380 import contract and cross this boundary
  by id only when the active session cites an import report.

## 1. Record boundary

Every record under this contract MUST resolve every field to exactly
one of the four boundaries below. Flattening them into one payload is
non-conforming.

| Boundary | What it carries | Where it lives |
|---|---|---|
| **Appearance session state** | active theme package and revision, theme class, contrast, accent source, density, text scale, reduced motion, follow-system posture, preview state, checkpoint and rollback refs | `appearance_session_record` |
| **Live follow-system policy** | per-axis live-update class, OS signal source, confirm requirement, surface scope, policy reason | `live_follow_system_policy_record` |
| **Token overlay state** | scope, validation result, fallback chain, per-token value-state classification | `token_overlay_record` |
| **Session revision events** | per-event session-state diff with cause class, OS signal ref, checkpoint ref, and rollback ref | `appearance_session_revision_event_record` |

Rules (frozen):

1. A single appearance session emits exactly one
   `appearance_session_record` per session revision. The
   `session_revision` field is monotonically increasing.
2. The session record cites at most one
   `live_follow_system_policy_ref` and at most one
   `token_overlay_ref`. A surface that resolves a per-axis live policy
   without citing the policy record is non-conforming.
3. The token-overlay record is the only place per-token value state is
   recorded. Free-form prose in `notes` is non-conforming as a
   substitute for the four-class value vocabulary.
4. Every revision-event record cites the prior and resulting
   `session_revision` integers and names exactly one cause class.

## 2. Appearance session fields

The session record is the **shared inspectable body** every appearance
reader consumes. Required fields (frozen):

- `appearance_session_id` — opaque stable id for the session.
- `session_revision` — integer, monotonically increasing per session.
- `active_theme_package_ref` — opaque id of the active theme package.
- `active_theme_revision_ref` — opaque id of the active revision of
  that package.
- `mode_theme_class` — exactly one value from the closed theme class
  set (`dark_reference`, `light_parity`, `high_contrast_dark`,
  `high_contrast_light`).
- `contrast_mode` — `contrast_standard`, `contrast_high`, or
  `contrast_forced_colors`.
- `accent_source` — `system_accent`, `theme_package_accent`,
  `user_selected_accent`, `policy_locked_accent`, or
  `not_applicable`.
- `density_class` — `compact`, `standard`, or `comfortable`.
- `text_scale` — `scale_percent` (75–200) and `source` (`system`,
  `user`, `profile`, `workspace`, `policy`).
- `reduced_motion_posture` — closed accessibility-posture class
  (`motion_standard`, `motion_reduced`, `motion_low_motion`,
  `motion_power_saver`, `motion_critical_hot_path`).
- `reduced_motion_source` — `os_signal`, `user_setting`, `policy_cap`,
  `power_saver_signal`, `critical_hot_path`, or `not_applicable`.
- `follow_system_posture` — `follow_system`, `manual_override`,
  `managed_policy_override`, or `unavailable_platform_signal`.
- `preview_state` — `not_previewing`, `preview_pending_validation`,
  `preview_live`, `preview_failed_reverted`, `preview_committed`, or
  `rollback_applied`.
- `current_checkpoint_ref` — opaque id of the current preview/apply
  checkpoint, or `null` when no checkpoint is in effect.
- `rollback_ref` — opaque id of the rollback handle for this session
  revision, or `null` when no rollback is in effect.
- `live_follow_system_policy_ref` — opaque id of the policy record
  governing this session's per-axis live-update behavior.
- `token_overlay_ref` — opaque id of the in-effect token overlay
  record, or `null` when no overlay is active.
- `policy_context` — managed-policy epoch and trust-state stamp.
- `redaction_class` — closed redaction-class enum reused from the
  shared portability vocabulary.
- `revision_minted_at` — producer-local monotonic timestamp for this
  revision.

Rules (frozen):

1. Active theme package and revision refs are mandatory. A session
   without a resolved theme package is non-conforming; the producer
   MUST mint a fallback session record citing the design-system
   default theme rather than emit a session with `null` theme refs.
2. `mode_theme_class`, `contrast_mode`, and `accent_source` MUST be
   resolved values, not the user's preference name. A surface that
   stores `auto` or `system` instead of resolving to one of the closed
   values is non-conforming.
3. `preview_state ∈ {preview_pending_validation, preview_live,
   preview_committed}` requires a non-null `current_checkpoint_ref`.
4. `preview_state = rollback_applied` requires a non-null
   `rollback_ref`.
5. `follow_system_posture = follow_system` requires that the cited
   `live_follow_system_policy_ref` resolves at least one axis to
   `live_apply_no_review` or `live_apply_with_revertable_checkpoint`.
   A `follow_system` posture whose every axis is
   `confirm_review_required` is non-conforming; the surface MUST emit
   `manual_override` instead.
6. `reduced_motion_source = critical_hot_path` MUST be paired with
   `reduced_motion_posture = motion_critical_hot_path`. This pairing
   never downgrades through a follow-system event; only an explicit
   user setting may relax it.
7. `text_scale.source = policy` MUST be paired with
   `live_follow_system_policy` axis `text_scale = policy_blocked`.
8. The session record MUST NOT carry raw token values, raw paths, raw
   URLs, raw screenshots, or raw user content. Cross-record references
   travel by opaque id only.

## 3. Live follow-system policy

The live follow-system policy record names, per appearance axis,
whether an OS signal change applies live, applies live behind a
revertable checkpoint, requires an explicit confirm or review
checkpoint, or is policy-blocked.

The closed per-axis vocabulary is fixed.

| Display label | Machine enum | When it applies |
|---|---|---|
| `Live` | `live_apply_no_review` | The axis applies the OS signal live with no checkpoint and no confirm step. Used for axes whose change is reversible by the same OS signal (theme class on supported platforms, accent source). |
| `Live (revertable)` | `live_apply_with_revertable_checkpoint` | The axis applies live but mints a single revertable checkpoint so the user can roll back. Used for axes that change durable state or visible chrome (contrast mode escalation, density class). |
| `Confirm required` | `confirm_review_required` | The axis holds the OS signal pending an explicit user confirm, review sheet, or apply action. Used for axes whose live change would surprise users or invalidate workspace authority (text scale, full theme-package switch, manual reload-required toggles). |
| `Policy blocked` | `policy_blocked` | Managed policy denies live application of this axis regardless of the OS signal. The surface MUST surface the lock and the source. |

Required fields per axis row:

- `axis` — closed appearance-axis enum (`mode_theme_class`,
  `contrast_mode`, `accent_source`, `density_class`, `text_scale`,
  `reduced_motion_posture`, `follow_system_posture`).
- `live_update_class` — exactly one value from the closed vocabulary
  above.
- `os_signal_class` — closed OS-signal class (`os_theme_signal`,
  `os_contrast_signal`, `os_accent_signal`, `os_density_signal`,
  `os_text_scale_signal`, `os_reduced_motion_signal`,
  `os_forced_colors_signal`, `none`).
- `requires_checkpoint` — boolean. MUST be `true` when
  `live_update_class = live_apply_with_revertable_checkpoint` or
  `confirm_review_required`.
- `requires_user_confirm` — boolean. MUST be `true` when
  `live_update_class = confirm_review_required`.
- `policy_lock_reason_class` — `not_locked`, `managed_policy_cap`,
  `restricted_workspace_cap`, `critical_hot_path_cap`, or
  `platform_unavailable`.
- `surface_scope` — `global_appearance`,
  `profile_appearance`, `workspace_appearance`, or
  `extension_surface_appearance`. Names which surface scope the axis
  applies to.
- `notes` — short reviewer note (optional).

Rules (frozen):

1. The policy record MUST cover every appearance axis at least once.
   A policy with a missing axis is non-conforming.
2. `live_apply_no_review` is forbidden for `text_scale`,
   `mode_theme_class`, and `follow_system_posture`. Switching theme
   package live without a revertable checkpoint or confirm is
   non-conforming. (`mode_theme_class` MAY be `live_apply_no_review`
   only when paired with `os_signal_class = os_theme_signal` and a
   default-theme-package follow-system posture; explicit user theme
   selection MUST mint a revertable checkpoint.)
3. `policy_blocked` MUST cite a non-`not_locked`
   `policy_lock_reason_class`. A `policy_blocked` row whose lock
   reason is `not_locked` is non-conforming.
4. `requires_user_confirm = true` requires a non-null
   `confirm_action_ref` on the resolving session record. The session
   surface MUST present the confirm action; a "silently apply on next
   focus" pattern is non-conforming.
5. New axes are additive only and require a schema bump and a new
   policy row. Repurposing an existing axis is breaking.

## 4. Token-overlay state contract

The token-overlay record describes which tokens are in effect for the
session and where each value came from. The closed per-token value
state vocabulary is the four-class set fixed below.

| Display label | Machine enum | Meaning |
|---|---|---|
| `Inherited` | `inherited` | The effective value is the active theme package's value at the requested theme class and contrast mode. No higher-priority overlay has overridden it. |
| `Overridden` | `overridden` | A higher-priority overlay scope (user, profile, workspace, policy, extension, imported theme) has supplied a different value than the theme package would have produced. |
| `Deprecated` | `deprecated` | The overlay points at a deprecated token. The row MUST cite `deprecated_replacement_ref` and the value remains available only until the deprecation window closes. |
| `Unmapped` | `unmapped` | The overlay references a token slot that has no current target token (the slot was retired, never resolved, or the import dropped it). The row MUST be visible as inert; silently dropping is non-conforming. |

Required record fields:

- `token_overlay_id` — opaque stable id for the overlay record.
- `appearance_session_ref` — opaque id of the session this overlay
  resolves under.
- `overlay_scope` — `theme_package_default`, `user_global`, `profile`,
  `workspace`, `policy_managed`, `extension_contributed`, or
  `imported_theme`.
- `validation_state` — `valid`, `valid_with_warnings`,
  `inert_unresolved`, `blocked_policy`, or `rolled_back`.
- `entries[]` — at least one token-overlay entry row.
- `summary_counts` — counts of `inherited_count`, `overridden_count`,
  `deprecated_count`, and `unmapped_count` matching the entry rows.
- `fallback_chain[]` — ordered list of `fallback_chain_step` rows
  describing the resolution chain the overlay walks for any value that
  cannot resolve at the requested theme class and contrast mode.
- `source_import_report_ref` — opaque id of the M00-380 import or
  overlay report this overlay was minted from, or `null`.
- `rollback_ref` — opaque id for the rollback handle that restores
  the prior overlay, or `null` when no rollback is in effect.
- `policy_context` — managed-policy epoch and trust-state stamp.
- `redaction_class` — closed redaction-class enum.
- `minted_at` — producer-local monotonic timestamp.

Required entry fields:

- `entry_id` — opaque stable id for the entry row.
- `token_ref` — opaque target token id (the token the overlay
  resolves).
- `token_family_class` — closed token-family enum re-exported from
  the design-token vocabulary.
- `value_state_class` — exactly one of `inherited`, `overridden`,
  `deprecated`, or `unmapped`.
- `effective_scope` — the overlay scope that supplied the effective
  value (matches the overlay's `overlay_scope` for non-`inherited`
  rows; `inherited` rows resolve to `theme_package_default`).
- `fallback_chain_ref` — opaque id of the fallback-chain step that
  resolved this entry, or `null` when the value resolved without
  walking the fallback chain.
- `deprecated_replacement_ref` — opaque id of the replacement token
  for `deprecated` entries, or `null` for non-deprecated entries.
- `unmapped_source_slot_ref` — opaque id of the source slot that is
  unmapped, or `null` for non-unmapped entries.
- `validation_state` — per-entry validation result (`valid`,
  `valid_with_warnings`, `inert_unresolved`, or `blocked_policy`).
- `notes` — short reviewer note (optional).

Required fallback-chain step fields:

- `step_id` — opaque stable id for the step.
- `step_index` — integer ≥ 0; the chain is ordered low-to-high.
- `step_scope` — overlay scope being consulted at this step.
- `step_kind` — `theme_package_default`, `scope_override`,
  `deprecated_alias`, or `inert_placeholder`.
- `target_token_ref` — opaque token id consulted at this step, or
  `null` when the step is `inert_placeholder`.
- `applied` — boolean. Exactly one chain step per resolved entry
  carries `applied = true`; `inert_unresolved` and `unmapped` entries
  may carry zero applied steps and MUST cite the chain end as
  `inert_placeholder`.

Rules (frozen):

1. The four-class value vocabulary is closed. A surface that emits
   `partial`, `best_effort`, `dropped`, or another parallel label is
   non-conforming.
2. `summary_counts` MUST equal the count of corresponding
   `value_state_class` entries.
3. A `deprecated` entry MUST cite `deprecated_replacement_ref`. A
   replacement that points at the same token id is non-conforming.
4. An `unmapped` entry MUST cite `unmapped_source_slot_ref` and MUST
   resolve to an `inert_placeholder` chain end. Dropping the row is
   non-conforming.
5. `validation_state = rolled_back` MUST be paired with a non-null
   `rollback_ref`.
6. `validation_state = blocked_policy` MUST be paired with a
   `policy_managed` overlay scope or with a `policy_blocked` axis on
   the cited live-follow-system policy. A `blocked_policy` overlay
   without a matching policy lock is non-conforming.
7. The fallback chain is reviewer-visible. Compressing it to a single
   "best effort" line in `notes` is non-conforming.
8. New overlay scopes, fallback-step kinds, validation states, and
   value-state classes are additive only and require a schema bump.
   Repurposing an existing value is breaking.

## 5. Versioning and revision events

Live appearance changes are versioned through the
`session_revision` integer on the session record and a corresponding
`appearance_session_revision_event_record` for each transition.

Required event fields:

- `event_id` — opaque stable id for the event.
- `appearance_session_ref` — opaque id of the session.
- `prior_session_revision` — integer; the revision before this event.
- `resulting_session_revision` — integer; the revision after this
  event. MUST equal `prior_session_revision + 1`.
- `cause_class` — exactly one closed cause class (`os_signal_change`,
  `user_explicit_setting`, `policy_change`, `import_apply`,
  `import_revert`, `overlay_apply`, `overlay_revert`,
  `preview_start`, `preview_commit`, `preview_revert`,
  `checkpoint_rollback`, `power_saver_engaged`,
  `critical_hot_path_engaged`).
- `os_signal_ref` — opaque id of the upstream OS signal (when
  `cause_class = os_signal_change` or `power_saver_engaged`), or
  `null`.
- `confirm_action_ref` — opaque id of the user-visible confirm action
  (when the resolving live policy required one), or `null`.
- `checkpoint_ref` — opaque id of the checkpoint minted or consulted
  for this event, or `null`.
- `rollback_ref` — opaque id of the rollback handle (when the event
  produced or consumed one), or `null`.
- `changed_axes[]` — at least one appearance-axis enum. The set MUST
  match the diff between the prior and resulting session record.
- `applied_under_live_class` — closed live-update class
  (`live_apply_no_review`, `live_apply_with_revertable_checkpoint`,
  `confirm_review_required`, or `policy_blocked`) recording how the
  cited live policy resolved this event.
- `policy_context` — managed-policy epoch and trust-state stamp.
- `redaction_class` — closed redaction-class enum.
- `recorded_at` — producer-local monotonic timestamp.

Rules (frozen):

1. `applied_under_live_class = confirm_review_required` MUST cite a
   non-null `confirm_action_ref`. A confirm event without a recorded
   confirm action is non-conforming.
2. `applied_under_live_class = live_apply_with_revertable_checkpoint`
   MUST cite a non-null `checkpoint_ref`.
3. `cause_class = checkpoint_rollback` MUST cite a non-null
   `rollback_ref`.
4. `cause_class = critical_hot_path_engaged` MUST set
   `applied_under_live_class = live_apply_no_review` and MUST list
   `reduced_motion_posture` in `changed_axes`. The surface MUST NOT
   demand a confirm step for the engine-internal hot path.
5. `cause_class = policy_change` paired with
   `applied_under_live_class = policy_blocked` records the moment the
   policy lock engaged. The session record at the resulting revision
   MUST carry the matching `policy_blocked` axis on the live policy.
6. The event log MAY be retained per the redaction class. Replaying
   the event log against the prior session record MUST reproduce the
   resulting session record without further inputs.

## 6. Composition with the M00-380 import contract

The M00-380 import-and-checkpoint contract owns the **workflow** of
importing a theme, minting an appearance checkpoint, and producing a
parity claim. This contract owns the **state** the workflow produces.

Composition rules:

1. The session record's `current_checkpoint_ref` and `rollback_ref`
   are opaque ids minted by the M00-380 checkpoint surface. This
   contract treats them as opaque.
2. The session record's `active_import_report_refs` (if any) carry
   into the M00-380 `appearance_session_record`'s
   `active_import_report_refs[]` field by id. This contract does not
   re-render the import report body.
3. The token-overlay record's `source_import_report_ref` cites the
   M00-380 `token_overlay_report_record` when the overlay was minted
   by import. The four-class value vocabulary in this contract maps to
   the M00-380 import `mapping_state` enum as follows:

   | This contract | M00-380 mapping_state |
   |---|---|
   | `inherited` | `translated` (when no override applied) |
   | `overridden` | `translated` (when an override applied) or `substituted_fallback` |
   | `deprecated` | `deprecated_replacement` |
   | `unmapped` | `unresolved` |

   The mappings are reviewer-visible; a surface that conflates the
   import-side `blocked_honesty` posture with `unmapped` is
   non-conforming.
4. The live follow-system policy record is independent of the M00-380
   workflow. Importing a theme MUST NOT silently relax a
   `policy_blocked` or `confirm_review_required` axis.

## 7. Fixture coverage

The fixture corpus under
[`/fixtures/design/appearance_session_cases/`](../../fixtures/design/appearance_session_cases/)
covers at least:

- Steady-state session with `follow_system` posture and a live
  OS-theme signal applied under `live_apply_no_review`.
- Live preview with a single revertable checkpoint and a token overlay
  whose entries are mostly inherited.
- OS contrast change applied under
  `live_apply_with_revertable_checkpoint` with a revision event
  recording the checkpoint mint.
- User-driven theme-package switch held under
  `confirm_review_required`, with a confirm-action event applying it.
- Workspace token overlay with one `overridden`, one `deprecated`, and
  one `unmapped` entry plus a fallback chain that ends in an
  `inert_placeholder`.
- Policy-blocked text-scale axis with a paired `blocked_policy`
  overlay entry and a `policy_change` revision event.

Each fixture cites the schema directly (`yaml-language-server`
header) and resolves every required field.

## 8. Out of scope

- The theme engine, settings UI, signal-watcher implementation, or
  live-apply pipeline.
- Per-token raw values, raw screenshots, or raw OS-signal payloads.
- Marketplace, license, or distribution behavior for theme packages.
- A runtime conformance runner. The schemas are precise enough for a
  future runner to validate sessions, overlays, policies, and
  revision events against these records.
