# Sync, device registry, conflict review, and support parity — contract

This is the reviewer-facing companion for the stable lane that ships
**sync / device-registry truth, field-aware conflict review, device
participation state, profile portability, and support-export parity** as a
governed launch property: one governed record per sync posture that binds
**device participation truth**, **field-aware conflict review**,
**snapshot-class provenance**, **local-authoritative fallback**, the **secret
boundary**, **REL-SYNC-009 merge precedence**, **profile-roaming / offboarding
truth**, and **cross-surface parity** across the desktop UI, CLI inspect,
Help/About, support export, and admin device-registry view — all to a public
claim ceiling and an automatic narrow-below-Stable verdict.

This lane is a settings-side certification that *projects* the live settings
runtime. It does not invent a private "sync" read; it proves the
effective-settings object model resolves conflicts and stays attributable to one
source of truth across surfaces. It builds on the schema registry
(`aureline_settings::schema::SchemaRegistry`), the precedence engine and
locked-write flow (`aureline_settings::resolver`), the shared conflict
projection (`aureline_settings::inspector::conflict`), and the beta sync review
projection (`aureline_settings::sync`).

Do not clone status text from this doc — ingest the canonical machine sources:

- Records / fixtures:
  [`/fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support/`](../../../fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support/)
- Schema:
  [`/schemas/ux/ship-sync-device-registry-conflict-review-and-support.schema.json`](../../../schemas/ux/ship-sync-device-registry-conflict-review-and-support.schema.json)
- Release-evidence packet:
  [`/artifacts/ux/m4/ship-sync-device-registry-conflict-review-and-support.md`](../../../artifacts/ux/m4/ship-sync-device-registry-conflict-review-and-support.md)
- Typed source: `aureline_settings::sync_device_registry_stable` (`model`, `corpus`)
- Headless emitter: `aureline_settings_sync_device_registry_stable`
- Replay + invariant gate:
  `crates/aureline-settings/tests/sync_device_registry_stable_fixtures.rs`

## Why one governed certification record

Profile portability is read and acted on from many surfaces — the desktop
device-and-sync surface, a CLI / headless inspect, Help/About, a support export,
and the admin device-registry view. If each surface re-derives "which devices
participate, what synced, what is stale, what would be overwritten, and whether
the local profile is still authoritative" from its own private read, the surfaces
drift: the UI implies healthy roaming while a support export shows a stale
manifest, a paused device silently keeps emitting, or a synced bundle overwrites
a local scope with no checkpoint and no diff. The result is a green "profile
portability is replacement-grade" claim that is really an average over surfaces
that each describe sync a little differently — masking offline, stale,
policy-blocked, or secret-leaking sync as healthy continuity.

A `sync_device_registry_certification_record` closes that gap. For one sync
posture it binds:

- **Device participation truth.** Every `device_participation[]` row exposes a
  stable `device_id`, `participation_state`, `profile_durability`,
  `last_successful_sync`, `selected_scope_set`, `conflict_class`,
  `rollback_checkpoint_ref`, and `local_authoritative_fallback` — all
  `inspectable_without_mutation`, so a user can see participation without opting
  into a mutating sync action.
- **Field-aware conflict review.** Every `conflict_review[]` row classifies the
  `outcome_class` as `exact_match`, `translated`, `partial`, `stale_remote`,
  `policy_locked`, or `local_authoritative`, names the REL-SYNC-009 `merge_class`,
  and — when `overwrites_local` is true — carries a `change_preview_ref` and a
  `rollback_checkpoint_ref` before apply. The record's `outcome_coverage[]`
  rollup proves the review distinguishes all six outcomes rather than a generic
  "merge?" prompt.
- **Snapshot-class provenance.** Every `snapshots[]` row (local rollback
  checkpoint, portable profile export, managed sync snapshot, support recovery
  manifest) carries its `included_state_classes[]`, `excluded_state_classes[]`,
  `producer_aureline_version`, `producer_schema_version`, `integrity_hash`,
  `source_provenance`, and `local_authoritative_fallback`.
- **The secret boundary.** `secret_boundary[]` proves `dirty_buffer_journals`
  and `secret_material` are excluded from both the `sync` and `export` lanes;
  only reference-only metadata is allowed across.
- **Profile-roaming / offboarding truth.** `profile_roaming` carries the
  `latest_successful_sync_ref`, the `extension_inventory_ref`, the
  `remaining_retention_days`, and proves `local_launch_edit_authority_retained`
  stays true even when `managed_sync_available` is false, with
  `temporary_profiles_excluded` by default.
- **Cross-surface parity.** One `surface_parity[]` row per desktop UI, CLI
  inspect, Help/About, support export, and admin device-registry view, each
  proving it `consumes_shared_record` and does not `clones_prose`.

## REL-SYNC-009 local precedence

Local explicit edits win over stale remote copies until a user or
policy-approved merge says otherwise. The `merge_class` per `setting_category`
is checked directly:

| Category | Required merge class |
| --- | --- |
| `scalar` | `fieldwise_merge` |
| `additive_asset` | `additive_merge` |
| `structured_definition` (keybindings / tasks / launch / workset) | `explicit_conflict_review` |

A `stale_remote` outcome may use `local_precedence` instead, because an older
bundle never overwrites the local lineage. The
`pillars.merge_rules_enforced` verdict is derived from every row's
`merge_rule_satisfied`.

## The public claim ceiling and automatic narrowing

`pillars` are *derived* from the rows, never asserted. `claim_ceiling` records
what the posture is allowed to claim; the builder refuses to mint a record whose
ceiling exceeds its proof. `stable_qualification` then narrows the posture below
Stable with a named `narrowing_reasons[]` entry whenever a pillar fails or the
lowest surface marker is below Stable — so a posture never inherits Stable by
adjacency.

| Narrowing reason | Meaning |
| --- | --- |
| `device_participation_incomplete` | A device does not expose the required participation truth. |
| `conflict_review_not_field_aware` | Conflict review does not distinguish the required outcome classes. |
| `snapshot_provenance_missing` | A snapshot class is missing provenance. |
| `local_fallback_unproven` | Local-authoritative fallback is unproven, or an overwrite is unprotected. |
| `secret_boundary_unproven` | The secret boundary admits a forbidden state class. |
| `merge_rule_unenforced` | A merge class does not match the setting category. |
| `profile_roaming_incomplete` | Profile-roaming truth is incomplete or loses local authority. |
| `surface_clones_prose` | A surface clones prose instead of consuming the shared record. |
| `surface_not_yet_stable` | The lowest surface marker is below Stable. |

## Reachability, accessibility, and availability

The same record is reachable from the device registry, the command palette, the
status bar, and a menu command (`routes[]`), keyboard-first, and the recovery
routes (`recovery_routes[]`) hold across normal, high-contrast, and zoomed
layouts (`accessibility.layout_modes[]`). Every record stays available without an
account or managed services.

## Regenerating the records

```sh
cargo run -q -p aureline-settings \
  --bin aureline_settings_sync_device_registry_stable -- emit-fixtures \
  fixtures/ux/m4/ship-sync-device-registry-conflict-review-and-support
```

The replay gate
`crates/aureline-settings/tests/sync_device_registry_stable_fixtures.rs` fails if
the on-disk JSON drifts from the in-code projection.
