# Sync / device-registry certification — release evidence

Reviewer-facing evidence packet for the lane that ships **sync / device-registry
truth, field-aware conflict review, device participation state, profile
portability, and support-export parity** on claimed-stable desktop surfaces: one
canonical record per sync posture that binds device participation truth, a
field-aware conflict review covering exact-match / translated / partial /
stale-remote / policy-locked / local-authoritative outcomes, snapshot-class
provenance for the local rollback checkpoint / portable profile export / managed
sync snapshot / support recovery manifest, a secret boundary excluding
dirty-buffer journals and secret material from the sync and export lanes,
REL-SYNC-009 merge precedence, profile-roaming / offboarding truth that keeps
local launch and edit authority even when managed sync is unavailable,
cross-surface parity across the desktop UI / CLI inspect / Help/About / support
export / admin device-registry view, a public claim ceiling, an automatic
narrow-below-Stable verdict, recovery and route parity across the device registry
/ command palette / status bar / menus, accessibility across normal /
high-contrast / zoomed layouts, and rows that stay available without an account
or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support/`](../../../fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support/)
- Schema: [`/schemas/ux/ship-sync-device-registry-conflict-review-and-support.schema.json`](../../../schemas/ux/ship-sync-device-registry-conflict-review-and-support.schema.json)
- Companion doc: [`/docs/ux/m4/ship-sync-device-registry-conflict-review-and-support.md`](../../../docs/ux/m4/ship-sync-device-registry-conflict-review-and-support.md)
- Typed source: `aureline_settings::sync_device_registry_stable` (`model`, `corpus`)
- Headless emitter: `aureline_settings_sync_device_registry_stable`
- Replay + invariant gate: `crates/aureline-settings/tests/sync_device_registry_stable_fixtures.rs`

## The claimed-stable matrix

| Record | Posture | Claim | Surface marker | Narrowing reason |
| --- | --- | --- | --- | --- |
| `nominal.json` | nominal sync and device registry | **stable** | stable | — |
| `stale_remote_local_authoritative.json` | stale remote device, local authoritative | **stable** | stable | — |
| `managed_sync_unavailable.json` | managed sync unavailable, local authority retained | **stable** | stable | — |
| `secret_boundary_drill.json` | managed snapshot leaks a dirty-buffer journal | beta (narrowed) | stable | `secret_boundary_unproven` |
| `unprotected_overwrite_drill.json` | overwrite without a checkpoint or preview | beta (narrowed) | stable | `local_fallback_unproven` |
| `device_registry_view_in_preview.json` | admin device-registry view still in Preview | preview (narrowed) | preview | `surface_not_yet_stable` |

Coverage verdict: **3 Stable, 3 narrowed**. Each narrowed row names a reason and
drops below the launch cutline rather than inheriting an adjacent green row.

## Acceptance criteria → evidence

- **Profile export/import, optional sync, and conflict-review flows show what is
  portable, what stays machine-local, and what will be dropped or redacted before
  any overwrite occurs.** Each `snapshots[]` row lists `included_state_classes[]`
  and `excluded_state_classes[]`; the portable profile export and managed sync
  snapshot exclude `machine_local_topology`, `dirty_buffer_journals`, and
  `secret_material`. Every overwriting `conflict_review[]` row carries a
  `change_preview_ref` and a `rollback_checkpoint_ref` before apply.
- **Each participating device record exposes stable device identity,
  participation state, last successful sync, selected scope set, conflict class,
  rollback checkpoint, and local-authoritative fallback.** `device_participation[]`
  carries all of these and `inspectable_without_mutation = true`, so device and
  conflict-class state can be inspected without opting into a mutating sync action.
- **Conflict review is field-aware.** `outcome_coverage[]` rolls up the
  `outcome_class` of every `conflict_review[]` row; the Stable postures distinguish
  `exact_match`, `translated`, `partial`, `stale_remote`, `policy_locked`, and
  `local_authoritative`. The policy-locked `security.ai.egress_policy` row keeps
  its real resolver `lock_state` and never overwrites the local scope.
- **REL-SYNC-009 local precedence is enforced in the product.**
  `pillars.merge_rules_enforced` is derived from each row's `merge_rule_satisfied`:
  scalars use `fieldwise_merge`, the `security.trusted_folders` additive asset uses
  `additive_merge`, the `tasks.runner_profile` structured definition uses
  `explicit_conflict_review`, and the stale `editor.tab_size` row uses
  `local_precedence`.
- **Snapshot-class truth is explicit.** All four classes
  (`local_rollback_checkpoint`, `portable_profile_export`, `managed_sync_snapshot`,
  `support_recovery_manifest`) carry `producer_aureline_version`,
  `producer_schema_version`, `integrity_hash`, `source_provenance`, and
  `local_authoritative_fallback`.
- **The secret boundary is held.** `secret_boundary[]` excludes
  `dirty_buffer_journals` and `secret_material` on both the `sync` and `export`
  lanes; only reference-only metadata is allowed across.
- **Profile-roaming and offboarding truth carry into Help/About, support export,
  and the admin device-registry view.** `profile_roaming` carries the
  `latest_successful_sync_ref`, the `extension_inventory_ref`, the
  `remaining_retention_days`, and proves
  `local_launch_edit_authority_retained = true` even in
  `managed_sync_unavailable.json` where `managed_sync_available = false`.
  `temporary_profiles_excluded = true` keeps session-only profiles off the sync
  lane by default.
- **Any surface still lacking stable qualification is automatically narrowed.**
  `secret_boundary_drill.json` narrows to `beta` via `secret_boundary_unproven`;
  `unprotected_overwrite_drill.json` narrows to `beta` via `local_fallback_unproven`;
  `device_registry_view_in_preview.json` narrows to `preview` via
  `surface_not_yet_stable`. Each carries `honesty_marker_present = true` and a
  bounded `waiver_ref` on the offending row, snapshot, or surface.

## Guardrails honored

- No sync or restore flow overwrites a local scope without a structured change
  preview plus a rollback checkpoint: the unprotected-overwrite drill surfaces
  `local_fallback_unproven` and forces the claim below Stable.
- Dirty-buffer journals and secret material never cross the sync or export lane:
  the secret-boundary drill surfaces `secret_boundary_unproven` and narrows the
  claim rather than shipping the leak.
- Sync pauses, stale remotes, and managed-sync narrowing never obscure the local
  source of truth or overstate continuity: `stale_remote_local_authoritative.json`
  and `managed_sync_unavailable.json` keep `local_authoritative_fallback` and
  `local_launch_edit_authority_retained` true and disclose the narrowing on every
  surface.
- No conflict row that conforms widens trust, egress, permissions, or managed
  authority (`widens_authority = false`).
- The record stays available without an account or managed services
  (`available_without_account`, `available_without_managed_services`).

## Reproduce

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_sync_device_registry_stable -- index

cargo run -q -p aureline-settings \
  --bin aureline_settings_sync_device_registry_stable -- emit-fixtures \
  fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support

cargo test -p aureline-settings --test sync_device_registry_stable_fixtures
```
