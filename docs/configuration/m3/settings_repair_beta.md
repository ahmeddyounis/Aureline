# Settings repair, reset, import, and migration review (beta)

This page is the reviewer-facing landing for the beta-grade settings
repair surface. Implementation lives in
`crates/aureline-settings/src/repair_review/`. The cross-tool
boundary schema is
`schemas/config/settings_repair_plan.schema.json`, which delegates to
the canonical settings write-intent and change-preview packet at
`schemas/settings/write_intent.schema.json`.

## Goal

Finish the settings safety story: every reset, repair,
reapply-imported-fragment, or revert-migration-step action goes
through the same write-intent pipeline as ordinary UI/CLI/sync
writes. No surface invents a private "broad reset" or
convenience-write path that bypasses the resolver, the policy
ceiling, or the checkpoint rules. Hidden resets and wrong-artifact
writes are blocked rather than explained after the fact.

## What's in a repair plan

Every action produces one `SettingsRepairPlan` record:

- `action_class` — frozen vocabulary:
  `reset_current_value`, `reset_section`, `repair_drift`,
  `reapply_imported_profile_fragment`, `revert_migration_step`.
- `target_scope` — the resolver scope token the plan would touch
  (e.g. `user_global`, `workspace`).
- `target_scope_class` — repair-facing classification of the
  target: `user`, `profile`, `workspace`, `policy_owned`, or
  `machine_local`. Policy-owned artifacts always refuse.
- `target_artifact_ref` — opaque ref that names the artifact that
  would actually receive the write (settings file path, profile
  fragment ref, or migration step ref).
- `actor_class` / `reason_class` — actor/reason vocabulary shared
  with the write-intent packet.
- `preview_class` — strictest preview/checkpoint/approval posture
  across the affected rows. Multi-row actions
  (`reset_section`, `reapply_imported_profile_fragment`) and
  `revert_migration_step` always bump at least to
  `rollback_checkpoint_required`.
- `checkpoint_required` / `checkpoint_ref` / `rollback_action_ref` —
  the checkpoint a multi-row plan preserves *before* apply and the
  rollback handle the user can route to *after* apply.
- `write_intents` — per-row inspector
  `setting_write_preview_record` produced by the same
  `preview_write` flow the UI/CLI/sync/policy/support pipeline uses.
- `blocked_write_reasons` — typed reasons (see below). Surfaces MUST
  quote the typed token; "couldn't repair" without a reason is a bug.
- `locked_classes` — classes that fell outside the writable surface
  (`policy_owned_class`, `capability_locked`, `retired_setting`,
  `managed_mode_only`, `non_writable_scope`).
- `hidden_reset_guard` — explicit verdict naming whether the plan
  would have broadened scope or touched unselected adjacent settings,
  plus the user's selected and refused setting id lists.
- `verdict` — overall verdict:
  `ready_to_apply`, `awaiting_preview`, `awaiting_checkpoint`,
  `awaiting_approval`, `denied`.
- `user_decision` — `pending`, `accepted`, `declined`, `withdrawn`.
- `effective_before` — inspector records for every affected row
  captured before apply, so compare/revert UIs can render the diff
  without re-resolving.

## Review sheets

Each repair flow renders one of five review sheets above the diff
affordance. The sheets share the same `SettingsRepairReviewSheet`
projection — the body is the canonical plan, the head is the typed
copy:

- **Reset this value** — `reset_current_value`. Restores one value
  at the named scope back to the next inherited source.
- **Reset this section** — `reset_section`. Restores every value
  inside one section prefix at one scope; always preserves a
  checkpoint before apply.
- **Repair drifted value** — `repair_drift`. Restores the
  last-known intended value at one scope when the written value has
  drifted (common after a partial import or hand-edit).
- **Re-apply imported profile fragment** —
  `reapply_imported_profile_fragment`. Only the rows the user
  re-selects are touched; the underlying profile artifact stays
  untouched.
- **Revert migration step** — `revert_migration_step`. Uses the
  checkpoint captured before the migration applied; the row carries
  the transform class and lossy/rollback flags so the user knows
  what they are reverting.

## Hidden-reset prevention

Any plan that would silently broaden scope or touch unselected
adjacent settings is refused before apply:

- `selected_setting_ids` is frozen at request time; the projection
  never extends it.
- Adjacent rows that show up in `proposed_values` but not in
  `selected_setting_ids` produce
  `adjacent_setting_refused` and land in
  `refused_setting_ids`.
- Target scopes that map to `policy_owned` produce
  `policy_owned_class` and refuse the plan.
- Setting-level scope-broadening (e.g. a denied write that would
  have widened trust) surfaces through the underlying write
  preview's `scope_broadening_verdict`; the plan inherits the denial
  and surfaces it as `blocked_write_reasons`.

## Blocked-write vocabulary

Frozen tokens; never invent new ones:

| code | meaning |
| --- | --- |
| `policy_owned_class` | Setting is owned by admin policy. |
| `non_writable_scope` | Scope is not in the setting's `allowed_scopes`. |
| `retired_setting` | Setting is retired and refuses writes. |
| `capability_dependency_unmet` | A declared capability dependency is not satisfied. |
| `checkpoint_missing` | A rollback checkpoint must be recorded before apply. |
| `approval_missing` | An approval ticket must be granted before apply. |
| `managed_mode_only` | Setting can only be written by a managed authority. |
| `scope_broadening_refused` | Plan would have widened the user-selected scope. |
| `adjacent_setting_refused` | Plan would have touched an unselected adjacent row. |
| `unknown_setting` | Setting is not registered with the resolver. |

## Checkpoints and rollback

Multi-row plans (`reset_section`,
`reapply_imported_profile_fragment`) and `revert_migration_step`
always set `checkpoint_required=true`. The resolver refuses apply
until `checkpoint_ref` is populated.

After apply, the resolver populates `rollback_action_ref`. The
post-apply UI uses this ref to render compare/revert without
re-deriving the diff.

## Support export

`SettingsRepairSupportExport` carries every plan emitted in the
session (or the supplied bundle). The export keeps the per-plan
`user_decision`, so support tooling can quote whether the user
accepted, declined, or withdrew the plan — and which scope was
selected when they did.

## Fixtures

Canonical replay fixtures live under
`fixtures/config/m3/settings_repair_and_reset/`. See the README
there for the exact regeneration commands. Every fixture is a
literal projection of `aureline-settings` running against the seed
catalog; no hand-edits.

## Out of scope for this surface

- Generic settings-sync control plane.
- Full fleet-policy editor.
- Imported-profile authoring (the plan re-applies, it does not
  rewrite, the profile artifact).
