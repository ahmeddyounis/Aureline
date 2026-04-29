# Update-ready review, impact forecast, and rollback-before-restart contract

This document freezes the pre-apply update review object family Aureline
uses before desktop update flows, channel switching, side-by-side
installs, and emergency rollback paths mutate local or managed state. It
is a contract layer, not an updater implementation or final UI design.

The goal is that a user, admin, release operator, support export, or
headless dry run can answer the same questions before apply or restart:

- what signed update was verified, blocked, or only partially checked;
- which target version and channel are being selected;
- which artifact families, install profiles, channels, state roots, and
  package or extension rows may change;
- which migration tasks are generated before restart;
- which restart class is required and why;
- which rollback or last-known-good path will exist after apply; and
- which claims come from current evidence rather than hopeful release
  prose.

Companion artifacts:

- [`/schemas/release/update_ready_review.schema.json`](../../schemas/release/update_ready_review.schema.json)
  - boundary schema for `update_ready_review_record`.
- [`/schemas/release/extension_impact_forecast.schema.json`](../../schemas/release/extension_impact_forecast.schema.json)
  - boundary schema for extension, package, workflow-bundle, SDK, and
    marketplace forecast rows consumed by update review.
- [`/fixtures/release/update_ready_cases/`](../../fixtures/release/update_ready_cases/)
  - worked cases for a normal signed update, side-by-side channel
    change, blocked update, migration-required update, and rollback-ready
    emergency path.
- [`/docs/release/update_and_rollback_contract.md`](./update_and_rollback_contract.md)
  and
  [`/schemas/release/update_manifest.schema.json`](../../schemas/release/update_manifest.schema.json)
  - update-manifest, rollback, downgrade, helper-negotiation, and
    publication fields this review projects.
- [`/docs/release/release_status_surface_contract.md`](./release_status_surface_contract.md)
  - release-candidate card, version-bump, evidence freshness,
    support-window, compatibility, and rollback/revocation vocabulary.
- [`/docs/release/channel_and_branch_contract.md`](./channel_and_branch_contract.md)
  and
  [`/docs/release/install_profile_card_contract.md`](./install_profile_card_contract.md)
  - channel identity, side-by-side admission, install-profile, rollout,
    durable-state, handler, and last-known-good vocabulary.
- [`/docs/compat/compatibility_row_seed.md`](../compat/compatibility_row_seed.md),
  [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](../migration/first_run_import_diff_and_rollback_contract.md),
  and
  [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  - compatibility-row, migration-task, backup, and rollback-checkpoint
    semantics used when an update narrows support or requires local
    state migration.
- [`/docs/extensions/sdk_publication_contract.md`](../extensions/sdk_publication_contract.md),
  [`/docs/extensions/registry_and_offline_bundle_seed.md`](../extensions/registry_and_offline_bundle_seed.md),
  and
  [`/docs/ecosystem/extension_lockfile_and_recommendation_contract.md`](../ecosystem/extension_lockfile_and_recommendation_contract.md)
  - extension SDK, marketplace, mirror/offline, and package-impact
    sources consumed by the forecast rows.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` sections on release rhythm, signed update
  feeds, side-by-side installs, deprecation, compatibility, and release
  evidence.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections on
  install topology, release artifacts, signed update metadata,
  side-by-side channel rules, support windows, and mixed-version
  compatibility.
- `.t2/docs/Aureline_Technical_Design_Document.md` sections on release
  center, support windows, signed metadata, rollback/revocation records,
  extension compatibility, and break-glass publication.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix BK plus adjacent
  release, install-profile, support-window, migration, and extension
  lifecycle templates.

If this document disagrees with those sources, those sources win and
this document plus its companion schemas and fixtures update in the same
change.

## Scope

Frozen here:

- one `update_ready_review_record` shape for pre-apply and
  before-restart review;
- one `extension_impact_forecast_record` shape for extension, package,
  SDK, workflow-bundle, and marketplace impact rows;
- forecast classes for no known extension impact, compatible with
  warning, uncertified row, migration required, blocked by policy, and
  rollback-ready path available;
- field groups that disclose signed update verification state, target
  version/channel, affected artifact families, impact forecasts,
  migration tasks, restart-required class, support-window or
  compatibility-window changes, and rollback visibility before apply or
  restart;
- parity rules that keep update review, release notes, what-is-new,
  compatibility, migration, support, and headless output on one
  evidence-backed language; and
- install-profile, side-by-side, last-known-good, and emergency
  path notes so channel switching never hides state or restart effects.

Out of scope:

- updater service implementation;
- installer scripts, download scheduling, binary delta formats, fleet
  controller behavior, or final product UI polish;
- raw binaries, raw signatures, raw extension manifests, raw lockfiles,
  raw package-manager output, raw user state, raw support bundles, raw
  policy bundles, and raw credentials.

## Invariants

1. **Review precedes mutation.** An update that can change binaries,
   helpers, state roots, channels, support windows, extension runtime,
   package lockfiles, or migration tasks produces an
   `update_ready_review_record` before apply or restart.
2. **Trust is layered.** Signed update verification states separate
   digest, signature, platform trust, provenance, freshness, revocation,
   mirror lineage, and policy decisions. A verified signature does not
   imply compatibility or rollback readiness.
3. **Impact is forecast, not certainty.** Extension and package impact
   rows are forecasts based on current evidence. Stale or missing
   evidence downgrades language instead of claiming a future runtime
   outcome.
4. **Restart cost is explicit.** The review names the restart-required
   class and the reason before restart. Unknown restart impact is a
   blocking review state until evidence or policy narrows it.
5. **Rollback is visible before restart.** If rollback, repin,
   last-known-good repair, checkpoint restore, or emergency channel
   switch will be available after apply, the review names the path and
   its limits before restart. If rollback is blocked or manual-only, the
   review says so before apply.
6. **Support-window changes travel with the update.** Any target that
   changes channel, narrows a support window, approaches end of support,
   passes end of support, or changes compatibility-window posture
   carries that disclosure in the review.
7. **Side-by-side is an install-profile decision.** A staged
   side-by-side update cites install-profile and channel-pair refs. It
   does not imply shared state, handler ownership, or automatic import.
8. **Emergency paths are structured.** Break-glass channel switch,
   emergency rollback, mirror-only emergency, and last-known-good repair
   use the same review object with explicit approval, reason,
   reconciliation, support, and local-state safety fields.
9. **Cross-surface language is shared.** Release notes, what-is-new,
   compatibility, migration, support exports, and headless update output
   may summarize the review, but they do not re-key or reinterpret the
   underlying refs.

## Object Model

| Object | Required identity | Required content | Reconstruction rule |
|---|---|---|---|
| Update-ready review | `review_id` plus `update_manifest_ref` | source/target versions and channels, verification state, affected artifact families, impact forecast rollup, migration tasks, restart class, rollback visibility, support-window and compatibility-window disclosure, parity links, install-profile notes, emergency notes, final gate decision | Consumed by update center, release notes, what-is-new, migration center, support exports, and headless dry runs. |
| Impact forecast row | `forecast_id` plus `review_ref` | subject identity, forecast class, evidence basis, compatibility/certification refs, policy decision, migration task refs, rollback-ready path, local-state safety note | One row per extension, package, bridge, SDK, workflow bundle, or marketplace subject whose impact is known, warning-worthy, uncertified, migration-required, policy-blocked, or rollback-ready. |
| Migration task | `task_id` | task class, scope, trigger, required-before state, automation availability, rollback/undo note, evidence refs | Generated before restart when state, extension bridge, package, cache, profile, policy, or support-window changes require action. |
| Rollback path | `rollback_path_ref` | path class, last-known-good target, checkpoint or manifest refs, retention window, unsupported limits, support/export refs | Rendered before apply or restart and preserved for support reconstruction. |

## Update-ready Review Fields

Every review carries these field groups.

| Field group | Required truth |
|---|---|
| `source_refs` | Update manifest, release-center, candidate card, release-note, compatibility, certification, migration, support, policy, and install-profile refs that generated the review. |
| `current_state` | Current version, channel, support window, exact-build refs, install-profile refs, and local-state posture before update. |
| `target_state` | Target version, channel, support window, exact-build refs, release stage, rollout ring, channel transition class, and target install-profile posture. |
| `verification` | Signed update verification state, digest/signature/platform/provenance proof states, revocation and freshness refs, mirror/import receipt state, and policy gate state. |
| `affected_artifact_families` | Artifact families and artifact refs affected by apply, restart, rollback, docs/schema/support updates, extension/package rows, or marketplace metadata changes. |
| `impact_forecast_rollup` | Worst forecast class, counts per forecast class, forecast refs, evidence freshness, and summary wording for update center, release notes, and migration surfaces. |
| `migration_tasks` | Generated tasks, required-before state, automation availability, backup/checkpoint refs, undo/rollback notes, and owner or policy refs. |
| `restart` | Restart-required class, restart blockers, user-visible effect, state preserved note, and evidence refs. |
| `rollback_visibility` | Rollback visibility class, rollback path class, last-known-good or repin target, checkpoint/manifest refs, retention window, unsupported limits, and support/export refs. |
| `support_and_compatibility` | Channel/support-window change, end-of-support risk, compatibility-window risk, certified-archetype deltas, retained support, and migration guidance refs. |
| `side_by_side_and_profile` | Install-profile transition class, source/target install-profile refs, side-by-side pair ref, state-root isolation refs, import decision refs, handler ownership note, and portable/managed restrictions. |
| `emergency_path` | Whether emergency action is active, action class, approval/reason refs, break-glass event refs, reconciliation target, follow-up refs, and local-state safety statement. |
| `parity` | Release-note refs, what-is-new refs, version-bump refs, archetype/certification delta refs, evidence freshness, allowed claim language class, local-state safety language class, and no-overclaim note. |
| `final_gate` | Whether apply/restart is allowed, warning-only, migration-required, policy-blocked, rollback-ready emergency, side-by-side staged, or review-only. |

## Forecast Classes

Impact forecast rows use a closed class set. The class describes current
evidence and review behavior, not a promise about what runtime code will
do after restart.

| Forecast class | Meaning | Required review behavior |
|---|---|---|
| `no_known_extension_impact` | Current evidence found no extension, package, bridge, SDK, workflow-bundle, or marketplace row that needs action. | Cite evidence freshness and avoid claiming no future impact. |
| `compatible_with_warning` | The subject appears compatible, but a version, bridge, permission, runtime, package-manager, or certification caveat should be visible before restart. | Show warning text, evidence refs, and migration or docs links when available. |
| `uncertified_row` | The subject has not been certified for the target scope or the evidence is missing/stale. | Downgrade claim language and route to compatibility or certification review. |
| `migration_required` | The subject requires a migration, repair, package refresh, bridge update, backup, or policy acknowledgement before or after restart. | Generate a migration task and name whether restart is blocked until task completion. |
| `blocked_by_policy` | Admin policy, trust policy, marketplace policy, mirror policy, or compatibility policy blocks the subject or the whole update. | Block apply/restart and cite policy decision refs. |
| `rollback_ready_path_available` | The subject can proceed because a rollback, repin, reinstall, quarantine, disable, or checkpoint path is already reviewed and retained. | Show rollback path and retention limits beside the update action. |

## Restart And Rollback Classes

`restart_required_class` values describe user-visible interruption:

- `no_restart_required`
- `window_reload_required`
- `desktop_restart_required`
- `helper_restart_required`
- `remote_agent_restart_required`
- `os_restart_required`
- `restart_blocked_until_migration`
- `restart_blocked_by_policy`
- `unknown_restart_impact`

`rollback_visibility_class` values describe what the user or admin will
be able to do after apply:

- `rollback_ready_before_apply`
- `rollback_ready_after_apply_until_cleanup`
- `rollback_ready_emergency_path`
- `rollback_requires_manual_review`
- `rollback_blocked`
- `rollback_not_applicable`
- `rollback_unknown_blocking`

`install_profile_transition_class` values describe how the target is
installed or selected:

- `upgrade_in_place`
- `staged_side_by_side`
- `side_by_side_import_required`
- `portable_extract_replace`
- `managed_fleet_rollout`
- `revert_to_last_known_good`
- `break_glass_channel_switch`
- `review_only_no_apply`

The review may render these as plain language, but the underlying class
stays machine-readable.

## Parity Rules

The update-ready review is the pre-apply source for release notes,
what-is-new, compatibility reports, migration center rows, support
exports, update-center cards, and headless dry-run output.

1. Release notes and what-is-new entries that mention the update must
   cite the same version-bump, affected artifact, support-window,
   compatibility, migration, and rollback refs as the review.
2. Certified-archetype deltas and compatibility rows must stay attached
   to the forecast and parity blocks. A release note may summarize a
   certification delta, but it cannot replace the row or broaden its
   scope.
3. Evidence freshness controls wording. Current evidence can use current
   claim language. Stale, waived, missing, or not-applicable evidence
   must render scoped, downgraded, or blocked wording.
4. Local-state safety language must be explicit. The review must say
   whether local profiles, settings, caches, workspaces, extensions,
   package lockfiles, and rollback checkpoints are preserved, migrated,
   rebuilt, blocked, or outside the update scope.
5. A support-window or compatibility-window risk must be visible before
   restart when the target narrows support, approaches end of support,
   is already past end of support, changes channel class, or requires
   migration work.
6. Emergency and break-glass paths use the same fields as ordinary
   updates plus the emergency refs. They do not use separate copy that
   omits rollback limits or reconciliation state.

## Side-by-side, Profile, And Emergency Notes

The review distinguishes these paths:

| Path | Required disclosure |
|---|---|
| Upgrade in place | Current and target build refs, affected artifact families, migration tasks, restart class, rollback path, and support-window changes. |
| Staged side-by-side | Source and target channels, side-by-side pair ref, install-profile refs, state-root isolation refs, handler ownership note, optional import decision, and rollback or keep-separate path. |
| Revert to last-known-good | Last-known-good exact-build refs, rollback manifest, affected artifact families, retained state roots, migration journal or backup refs, and manual-review blockers. |
| Break-glass channel switch | Emergency action, approval and reason refs, target channel, support-window narrowing, reconciliation target, local-state safety note, and rollback or repin path. |

An update review that omits these distinctions is non-conforming even if
the underlying update manifest verifies successfully.

## Fixture Matrix

The worked fixture set covers:

| Fixture | Required truth |
|---|---|
| `normal_signed_update.yaml` | Verified stable update, current evidence, no known extension impact, restart required, and rollback available before apply. |
| `side_by_side_channel_change.yaml` | Stable-to-preview side-by-side staging, separate state roots, uncertified or warning forecast rows, and import decision retained. |
| `blocked_policy_update.yaml` | Verified artifacts but policy blocks apply/restart and the forecast cites the blocked policy row. |
| `migration_required_update.yaml` | Signed update with migration-required forecast rows, generated migration tasks, backup/checkpoint refs, and restart blocked until migration review. |
| `rollback_ready_emergency.yaml` | Emergency path with break-glass refs, last-known-good rollback target, support-window disclosure, and rollback-ready forecast rows. |

## Change Control

Adding a vocabulary value to either schema is additive-minor only when
this document and at least one fixture update in the same change.
Repurposing an existing value is breaking and requires a decision record.
Product code, comments, fixture ids, and public copy must use
purpose-based names rather than planning ids.
