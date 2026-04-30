# End-of-support notice, compatibility-window report, and migration-helper handoff contract

This document is the narrative companion to the end-of-support notice,
compatibility-window report, and migration-helper handoff packet
Aureline freezes for every surface that warns a user about an upcoming
or past support cutoff, an active deprecation, or a breaking
compatibility change ahead. It pins which notice classes exist, which
cutoff kinds are admissible, which affected scopes the report carries,
which migration-helper actions stay distinct (so they cannot collapse
into one opaque "upgrade" button), and where one-click backup or
checkpoint is required before durable user state or supported
compatibility mode is altered.

Companion artifacts:

- [`/schemas/release/end_of_support_notice.schema.json`](../../schemas/release/end_of_support_notice.schema.json)
  — boundary schema for one `end_of_support_notice_record` projecting
  notice class, cutoff target, affected scope, factual
  what-still-works projections, typed risk-after-cutoff projections,
  exactly one safest next step, and the required surface set.
- [`/schemas/release/compatibility_window_report.schema.json`](../../schemas/release/compatibility_window_report.schema.json)
  — boundary schema for one `compatibility_window_report_record`
  projecting the per-category compatibility breakdown, the
  migration-helper handoff action set, the backup-or-checkpoint
  requirement gate, and the export-path set.
- [`/fixtures/release/end_of_support_cases/`](../../fixtures/release/end_of_support_cases)
  — seed `end_of_support_notice_record` and
  `compatibility_window_report_record` fixtures for the four required
  acceptance cases (approaching end of support, past end of support,
  active deprecation, breaking compatibility ahead).

Cross-linked artifacts already in the repository:

- [`/docs/release/channel_support_window_contract.md`](./channel_support_window_contract.md)
  and
  [`/schemas/release/support_window_badge.schema.json`](../../schemas/release/support_window_badge.schema.json)
  — channel identity and support-window badge contract. The
  support-window badge resolves the channel identity and the support
  class on a steady-state build; this contract takes over when the
  build has crossed (or is approaching) a cutoff. Every notice's
  `affected_scope.support_window_badge_refs` resolves into a badge.
- [`/docs/release/channel_and_branch_contract.md`](./channel_and_branch_contract.md)
  and
  [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
  — channel-and-branch matrix. Every notice's
  `affected_scope.channel_row_refs` resolves here.
- [`/docs/release/compatibility_report_template.md`](./compatibility_report_template.md)
  and
  [`/schemas/release/compatibility_row.schema.json`](../../schemas/release/compatibility_row.schema.json)
  — release compatibility report. Every category row's
  `compatibility_row_refs` resolves into one or more rows of the
  release compatibility report.
- [`/docs/release/update_ready_review_contract.md`](./update_ready_review_contract.md),
  [`/schemas/release/update_ready_review.schema.json`](../../schemas/release/update_ready_review.schema.json),
  and
  [`/schemas/release/extension_impact_forecast.schema.json`](../../schemas/release/extension_impact_forecast.schema.json)
  — pre-apply update review. The update-center banner consumes one
  notice through its support-window-change disclosure, and every
  category row whose category is `extensions`, `automations_or_macros`,
  or `marketplace_listings` quotes the extension-impact forecast by
  ref.
- [`/docs/release/update_and_rollback_contract.md`](./update_and_rollback_contract.md),
  [`/schemas/release/update_manifest.schema.json`](../../schemas/release/update_manifest.schema.json),
  and
  [`/schemas/release/helper_version_negotiation.schema.json`](../../schemas/release/helper_version_negotiation.schema.json)
  — update manifest, rollback, and helper-negotiation packet. The
  migration-helper handoff `rollback_to_checkpoint` action ref points
  into a rollback panel in this family; the `helpers_or_sidecars` and
  `remote_agents` category rows resolve into helper-negotiation packets.
- [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](../migration/first_run_import_diff_and_rollback_contract.md),
  [`/schemas/migration/import_diff_preview.schema.json`](../../schemas/migration/import_diff_preview.schema.json),
  and
  [`/schemas/migration/import_rollback_checkpoint.schema.json`](../../schemas/migration/import_rollback_checkpoint.schema.json)
  — first-run import diff and rollback. The migration-helper handoff
  `preview_planned_changes_dry_run`, `backup_or_checkpoint_durable_state`,
  and `apply_planned_changes` actions resolve their preview and
  checkpoint refs into this family. This contract reuses the
  preview-then-checkpoint-then-apply discipline frozen there for the
  end-of-support migration path.
- [`/docs/state/migration_and_restore_playbook.md`](../state/migration_and_restore_playbook.md)
  — state migration and restore playbook. Every notice whose affected
  scope includes `profiles_and_settings` or
  `workspace_or_layout_schemas` resolves through the state-plane and
  fidelity-label vocabulary frozen there.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  and
  [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
  — qualification matrix and version-skew register. Every category
  row's `version_skew_register_refs` resolves here.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` §5.20 (release rhythm and channel
  discipline), §9.9 (mixed-version compatibility, negotiation, and
  upgrade posture), §9.12 (enterprise deployment hooks).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §25.9
  (install, portable-mode, and fleet-rollout architecture), §26.5
  (distributed compatibility and version-skew policy), §27.8–§27.9
  (release widening and stable-facing claim movement).
- `.t2/docs/Aureline_Milestones_Document.md` §6.18 (install and update
  behaviour as product truth), §8.12 (release widening and
  stable-facing claim movement), §12.1.6 (LTS / backport posture).

## 1. Why publish this now

The channel-and-support-window badge contract froze the steady-state
channel identity and support class. The release-candidate card froze
the release-status surface. The update-ready-review contract froze
pre-apply update trust. The compatibility-report template froze the
row-by-row compatibility format. The first-run import diff and rollback
contract froze the preview-then-checkpoint-then-apply discipline.

What was still implicit was the **end-of-support pivot** — the moment
when a build, channel, or category transitions from steady-state
"supported" into one of four distinct postures:

1. **Approaching** the end of its support window;
2. **Past** the end of its support window;
3. **Actively deprecated** with a named replacement; or
4. **Breaking compatibility ahead** before the platform change ships
   as stable or LTS truth.

Left implicit, every surface (update center, About panel, compatibility
report, migration / import workflow, release notes) would re-invent its
own end-of-support copy and silently collapse the four postures into
generic fear language ("Your version is out of date — upgrade now").
That copy obscures THREE specific facts a user MUST be able to inspect:

- **What still works.** The build is still installable; existing
  workflows keep running until the named cutoff.
- **What becomes higher risk.** Specific risk classes (security-only,
  no security updates, helper-negotiation failure, profile-state-loss
  risk, extension-publication blocked, etc.) are typed, not free-text.
- **What the safest next step is.** A single typed action class with a
  named route ref — not a generic "upgrade" button.

This contract freezes those three projections AND the migration-helper
handoff packet that performs the safest next step. The handoff packet
keeps **scan**, **preview**, **backup or checkpoint**, **apply**,
**rollback**, **dry-run**, and **export environment summary** as
DISTINCT actions rather than collapsing them into one opaque upgrade
button. One-click backup or checkpoint is required before any apply
that alters durable user state or supported compatibility mode.

This is a **pre-implementation plan**. No banner, About-panel
projection, update-center end-of-support disclosure, compatibility-
window report renderer, or migration helper is implemented at this
revision. Every fixture is tagged `seeded` / `proposed`; rows are not
deleted, they are superseded by an ADR / RFC recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## 2. Scope

Frozen at this revision:

- One closed `notice_class` vocabulary with four values (approaching,
  past, active deprecation, breaking compatibility ahead).
- One closed `notice_severity_class` vocabulary that resolves against
  factual postures (informational, plan-named-action, act-now,
  inspect-and-export) rather than generic fear severities ("warning",
  "danger", "critical").
- One closed `cutoff_kind_class` vocabulary with five values (exact
  calendar date, exact release version, calendar window, version
  window, not-yet-set pending council review).
- One closed `affected_scope_class` vocabulary with thirteen values
  spanning build, channel, extensions, automations, command and action
  schemas, profiles and settings, workspace and layout schemas, remote
  agents, helpers and sidecars, CLI and headless surface, SDK and API,
  marketplace listings, and support-export format.
- One closed `risk_after_cutoff_class` vocabulary with nine typed risk
  classes; generic risk copy is non-conforming.
- One closed `what_still_works_class` vocabulary with seven factual
  classes; the banner cannot collapse to fear copy.
- One closed `safest_next_step_class` vocabulary with ten typed
  classes; free-text "upgrade now" calls to action are non-conforming.
- One closed `required_surface_class` vocabulary with eight values, of
  which four (update center banner, About panel banner, compatibility
  report banner, migration / import workflow banner) form the required
  floor.
- One closed `affected_category_class` vocabulary on the
  compatibility-window report restricted to the categories that have
  row-by-row breakdowns.
- One closed `category_window_state_class` vocabulary with six values
  (inside window, approaching, past, breaking ahead, actively
  deprecated, not-applicable refusal state).
- One closed `migration_helper_action_class` vocabulary with seven
  values kept DISTINCT (scan, preview, backup or checkpoint, apply,
  rollback, dry-run, export environment summary).
- One closed `action_routing_kind_class` vocabulary with four values
  (interactive user-initiated, headless dry-run only, support-export
  only, blocked by policy).
- One closed `mutation_class` vocabulary with four values (no mutation
  inspect-only, alters durable user state, alters supported
  compatibility mode, alters both).
- One closed `backup_requirement_class` vocabulary with five values,
  of which three (one-click backup required, one-click checkpoint
  required, backup required for compat-mode change) demand a one-click
  control before apply.
- One closed `export_path_class` vocabulary with five values; every
  compatibility-window report MUST cite at least one export path so
  the report is downloadable BEFORE any platform change ships as
  stable or LTS truth.

Out of scope until a superseding decision row opens:

- Executing migrations, running the migration helper, or shipping a
  hosted update service. The task spec marks both out of scope.
- Enforcing support policy (which capabilities lose support, on what
  cadence, under which deployment profiles). Support-policy cadence
  is benchmark-council / release-council authority.
- Final marketing copy for banner labels. The contract freezes the
  machine vocabulary; the copy resolves against it.

## 3. Notice-class vocabulary

Closed set. Each class binds different cutoff, evidence, and recovery
rules in the schema's allOf block.

| Notice class | Cutoff kinds admitted | What still works floor | Risk floor | Safest next step floor |
|---|---|---|---|---|
| `approaching_end_of_support` | `exact_calendar_date`, `exact_release_version`, `range_calendar_window`, `range_version_window` | factual class required (not `not_applicable_breaking_change_only`) | typed class required | any of the ten typed classes |
| `past_end_of_support` | same as approaching | factual class required (not `not_applicable_breaking_change_only`) | typed class required, `not_applicable_pre_release_only` forbidden | any of the ten typed classes |
| `active_deprecation` | `exact_calendar_date`, `exact_release_version`, `range_calendar_window`, `range_version_window` | factual class required | typed class required | any of the ten typed classes |
| `breaking_compatibility_ahead` | any of the five (including `not_yet_set_pending_council_review`) | any factual class | typed class required | restricted to `export_environment_summary_then_review_compatibility_window_report`, `migrate_extensions_via_compatibility_window_report`, `run_migration_helper_handoff_with_required_backup`, or `pause_until_named_release_lands_on_stable` |

### 3.1 Why the four classes are kept distinct

Collapsing the four classes into one chip ("end of life") loses three
distinct decisions:

- **Approaching vs. past.** Approaching means the build is still
  inside its support window; past means it is outside. The safest
  next step differs (plan vs. act now). The risk profile differs
  (security-only ahead vs. no security updates remaining).
- **Active deprecation vs. end of support.** A deprecated capability
  is being replaced; the safest next step names the supported
  replacement. End of support is a window cutoff for the carrier
  build; the safest next step names the supported floor.
- **Breaking compatibility ahead vs. past end of support.** Breaking
  ahead is a future event the user can inspect and export against
  BEFORE it ships; past end of support is a current event. The
  schema gates breaking-ahead notices to the four
  inspect-export-or-pause safest-next-step classes so the banner
  cannot recommend "upgrade now" before there is anything to upgrade
  to.

## 4. Cutoff vocabulary

Closed five-value `cutoff_kind_class`.

| Cutoff kind | Required fields | Admissible on |
|---|---|---|
| `exact_calendar_date` | `cutoff_at` non-null | every notice class |
| `exact_release_version` | `cutoff_version_label` non-null | every notice class |
| `range_calendar_window` | both `cutoff_window_starts_at` and `cutoff_window_ends_at` non-null | every notice class |
| `range_version_window` | `cutoff_version_label` non-null | every notice class |
| `not_yet_set_pending_council_review` | none | only `breaking_compatibility_ahead` |

The schema enforces these pairings: an `exact_calendar_date` notice
without a `cutoff_at` timestamp is non-conforming; a `range_version_window`
notice without a `cutoff_version_label` is non-conforming;
`not_yet_set_pending_council_review` on an approaching, past, or active
deprecation notice is non-conforming because that cutoff is, by
definition, already known.

## 5. Affected scope

Every notice MUST bind at least one `affected_scope_class`. The closed
thirteen-value vocabulary is the union of:

- **Carrier-build scopes** (`desktop_product_build`, `release_channel`)
  that point at one or more exact-build identities or channel rows.
- **Category scopes** (`extensions`, `automations_or_macros`,
  `command_or_action_schemas`, `profiles_and_settings`,
  `workspace_or_layout_schemas`, `remote_agents`,
  `helpers_or_sidecars`, `cli_or_headless_surface`, `sdk_or_api`,
  `marketplace_listings`, `support_export_format`) that point at
  category rows in the companion compatibility-window report.

When the affected scope includes any of the seven category scopes
admitted by the compatibility-window report
(`extensions`, `automations_or_macros`, `command_or_action_schemas`,
`profiles_and_settings`, `workspace_or_layout_schemas`, `remote_agents`,
`helpers_or_sidecars`), the notice MUST cite a non-null
`compatibility_window_report_ref` so the consuming surface can pivot in
O(1) into the row-by-row breakdown rather than reconstructing it from
prose.

## 6. What still works, risk after cutoff, safest next step

These three projections are what stop end-of-support banners from
collapsing into generic fear language.

### 6.1 What still works

Closed seven-value vocabulary. The notice MUST bind at least one
factual what-still-works projection so the banner cannot read as
"everything is broken." Approaching and past notices forbid the
`not_applicable_breaking_change_only` class because the build is
still installable.

| Class | Meaning |
|---|---|
| `read_only_inspection_remains_safe` | Inspectors, About panels, compatibility reports, support exports continue to work without risk. |
| `existing_workflows_continue_until_cutoff` | Workflows the user has already saved continue running until the named cutoff. |
| `existing_extensions_continue_until_cutoff` | Already-installed extensions continue loading until the named cutoff. |
| `existing_remote_agent_attach_continues_until_helper_skew_window_ends` | Remote agent attach keeps working until the helper-negotiation skew window ends. |
| `existing_durable_state_remains_owned_by_user` | Profile, settings, layout, and workspace state remain user-owned and untouched. |
| `no_runtime_change_until_breaking_change_ships` | Reserved for `breaking_compatibility_ahead`: nothing changes at runtime until the change ships. |
| `not_applicable_breaking_change_only` | Reserved for `breaking_compatibility_ahead` only when the change is breaking and there is no current-state continuity to preserve. |

### 6.2 Risk after cutoff

Closed nine-value vocabulary. The notice MUST bind at least one typed
risk class. `past_end_of_support` notices forbid
`not_applicable_pre_release_only` because the build has reached
general-availability cutoff.

| Class | Meaning |
|---|---|
| `security_only_updates_remaining_no_feature_updates` | Security updates continue; feature updates have stopped. |
| `no_security_updates_remaining` | Past the security-only window; no further updates. |
| `compatibility_drift_against_supported_compatibility_rows` | Supported compatibility rows narrow; the build falls outside the supported envelope over time. |
| `new_signed_artifact_install_blocked` | The platform refuses to install new signed artifacts on the affected build. |
| `extension_or_marketplace_publication_blocked` | New extension or marketplace publication is blocked. |
| `helper_or_remote_agent_negotiation_will_fail` | Helper or remote-agent version negotiation will reject the build past cutoff. |
| `profile_or_workspace_state_loss_risk_without_migration` | Profile, settings, layout, or workspace state risks loss without a migration helper run. |
| `mirror_or_air_gapped_install_will_fall_behind` | Mirror or air-gapped installs cannot keep up because the upstream feed stops. |
| `not_applicable_pre_release_only` | Reserved for pre-release-only carrier builds (forbidden on `past_end_of_support`). |

### 6.3 Safest next step

Closed ten-value vocabulary. Every notice MUST bind exactly one
typed safest_next_step. Free-text "upgrade now" calls to action are
non-conforming.

| Class | Required refs | Admissible on |
|---|---|---|
| `update_to_named_supported_floor` | `next_step_route_ref` non-null | any notice |
| `switch_to_lts_train_or_named_floor` | `next_step_route_ref` non-null | any notice |
| `migrate_extensions_via_compatibility_window_report` | `compatibility_window_report_ref` non-null | any notice |
| `run_migration_helper_handoff_with_required_backup` | `migration_helper_handoff_ref` non-null AND `compatibility_window_report_ref` non-null | any notice |
| `export_environment_summary_then_review_compatibility_window_report` | `compatibility_window_report_ref` non-null | any notice |
| `request_managed_admin_action_or_unblock_policy` | `next_step_route_ref` non-null | any notice |
| `open_compatibility_report_and_route_to_supported_archetype` | `next_step_route_ref` non-null | any notice |
| `in_channel_rollback_to_previous_build` | `next_step_route_ref` non-null | any notice |
| `pause_until_named_release_lands_on_stable` | none | any notice |
| `no_action_required_continue_inspecting` | none | any notice (typically `breaking_compatibility_ahead` informational) |

`breaking_compatibility_ahead` notices restrict the safest_next_step
to `export_environment_summary_then_review_compatibility_window_report`,
`migrate_extensions_via_compatibility_window_report`,
`run_migration_helper_handoff_with_required_backup`, or
`pause_until_named_release_lands_on_stable`. The other six classes are
forbidden because the change has not shipped yet.

## 7. Required surfaces

Every `end_of_support_notice_record` MUST render onto at least these
four surfaces:

1. **Update center banner** (`update_center_banner`) — the channel
   chip, support-class chip, support-window state, and end-of-support
   risk render BEFORE the user clicks "update".
2. **About panel banner** (`about_panel_banner`) — same fields visible
   without leaving the product.
3. **Compatibility report banner** (`compatibility_report_banner`) —
   the row-by-row compatibility report renders the notice alongside the
   compatibility rows so refusal classes do not get rendered as a
   generic "unsupported" chip.
4. **Migration / import workflow banner** (`migration_or_import_workflow_banner`)
   — destination posture is never silently widened; the user importing
   from another install or another tool sees the destination's notice
   class, cutoff, and safest next step before completing the import.

Admissible secondary surfaces (not required but admitted by the
schema): `release_notes_card`, `issue_report_packet`,
`headless_dry_run_output`, `support_export_bundle`. Tooling MAY add
more required surfaces in a later additive-minor revision; it MUST NOT
drop one.

## 8. Compatibility-window report

The compatibility-window report is the row-by-row breakdown the
end-of-support notice points at. It carries:

- A scope envelope binding at least one `affected_category_class`.
- A `category_rows` array with one row per affected category. Each
  row carries a typed `category_window_state_class`, refs into
  `compatibility_row.schema.json` rows, refs into the version-skew
  register, refs into support-window badges, and refs into
  `extension_impact_forecast.schema.json` rows when the category is
  `extensions`, `automations_or_macros`, or `marketplace_listings`.
- A `migration_helper_handoff` block with the action set kept
  DISTINCT (scan, preview, backup, apply, rollback, dry-run, export)
  and a `backup_requirement` block.
- An `export_paths` array binding at least one export path so the
  report is downloadable BEFORE any platform change ships as stable
  or LTS truth.

### 8.1 Category-window state

Closed six-value vocabulary.

| Class | Meaning |
|---|---|
| `inside_supported_window_no_action` | Row is inside its supported skew / freshness window; no action required. |
| `approaching_end_of_window_action_within_named_window` | Row is approaching the end of its window; named action within the window. |
| `past_end_of_window_higher_risk_named` | Row is past its window; the risk class is named per row. |
| `breaking_change_ahead_inspect_and_export_before_ship` | A breaking change is upcoming and the row is inspectable / exportable BEFORE the change ships. |
| `actively_deprecated_with_replacement_named` | Row is actively deprecated with a named replacement; the row's `named_replacement_label` is non-null. |
| `not_applicable_refusal_state` | Reserved for category rows that have been registered but not yet seeded with evidence. |

### 8.2 Extension-impact-forecast linkage

Rows whose category is `extensions`, `automations_or_macros`, or
`marketplace_listings` AND whose window state is
`past_end_of_window_higher_risk_named`,
`actively_deprecated_with_replacement_named`, or
`breaking_change_ahead_inspect_and_export_before_ship` MUST cite at
least one `extension_impact_forecast_ref` so the row is backed by the
typed forecast vocabulary frozen on the update-ready review and not by
free-text caveats.

## 9. Migration-helper handoff

Closed seven-value `migration_helper_action_class`. The actions stay
DISTINCT rather than collapsed into one opaque "upgrade" button. The
schema enforces the discipline:

- Each action class is admitted at most once per handoff packet.
- An `apply_planned_changes` action MUST be paired with a
  `rollback_to_checkpoint` action on the same packet. The rollback
  target is named BEFORE the apply runs.
- An `apply_planned_changes` action whose `mutation_class` alters
  durable user state OR supported compatibility mode MUST be paired
  with a `backup_or_checkpoint_durable_state` action AND a
  `backup_requirement_class` of `one_click_backup_required_before_apply`,
  `one_click_checkpoint_required_before_apply`, or
  `backup_required_compat_mode_change`.

| Action class | Inspect-only? | Required refs |
|---|---|---|
| `scan_environment_read_only` | yes | none |
| `preview_planned_changes_dry_run` | yes | `preview_diff_ref` non-null |
| `backup_or_checkpoint_durable_state` | no (writes a checkpoint, not user state) | `checkpoint_ref` non-null |
| `apply_planned_changes` | no | `preview_diff_ref` non-null; `checkpoint_ref` non-null when `mutation_class` alters durable state or compat mode |
| `rollback_to_checkpoint` | no | `rollback_ref` non-null AND `checkpoint_ref` non-null |
| `dry_run_apply_no_durable_writes` | yes | none |
| `export_environment_summary_for_review` | yes | `export_artifact_ref` non-null |

### 9.1 Mutation-class vocabulary

Closed four-value `mutation_class` describing what an apply action
mutates. Generic "upgrade" mutations are non-conforming.

| Class | Meaning |
|---|---|
| `no_mutation_inspect_only` | Action does not mutate durable state. Required for scan, preview, dry-run, export. |
| `alters_durable_user_state` | Action writes profile, settings, layout, workspace, or other user-owned durable state. |
| `alters_supported_compatibility_mode` | Action changes the build's support class or compatibility mode (e.g. switches the compatibility envelope). |
| `alters_durable_user_state_and_compatibility_mode` | Action does both. |

### 9.2 One-click backup or checkpoint requirement

Closed five-value `backup_requirement_class`. Three of the five
demand a one-click control BEFORE apply:

| Class | Required refs |
|---|---|
| `one_click_backup_required_before_apply` | `checkpoint_ref` non-null AND `one_click_route_ref` non-null |
| `one_click_checkpoint_required_before_apply` | `checkpoint_ref` non-null AND `one_click_route_ref` non-null |
| `backup_required_compat_mode_change` | `checkpoint_ref` non-null AND `one_click_route_ref` non-null |
| `backup_not_required_no_durable_state_altered` | none (only admissible when mutation_class is `no_mutation_inspect_only`) |
| `backup_unsupported_inspect_only_report` | none (only admissible on a report whose handoff omits apply) |

The schema enforces that any handoff packet whose actions include an
`apply_planned_changes` action with `mutation_class` of
`alters_durable_user_state`, `alters_supported_compatibility_mode`, or
`alters_durable_user_state_and_compatibility_mode` MUST resolve to one
of the three required-before-apply backup-requirement classes.

## 10. Export paths

Closed five-value `export_path_class`. Every compatibility-window
report MUST cite at least one export path so consuming surfaces (the
release evidence packet, the support bundle, the headless dry-run
output, and the migration export) carry the report verbatim, not as
free-text caveats.

| Class | Format |
|---|---|
| `compatibility_window_report_yaml_export` | YAML export of the record. |
| `compatibility_window_report_json_export` | JSON export of the record. |
| `support_bundle_compatibility_packet` | Compatibility packet inside a support bundle. |
| `headless_dry_run_packet` | Packet emitted by the headless dry-run path. |
| `release_evidence_packet_link` | Link into a release-evidence packet. |

`breaking_compatibility_ahead` notices, by §3 and §6.3, MUST point at a
compatibility-window report whose export paths are non-empty so the
breaking change is inspectable and exportable BEFORE it ships as
stable or LTS truth.

## 11. Failure modes prevented

1. *Update center renders "End of support — upgrade now" with no
   factual what-still-works and no typed risk class.* — Refused by
   the schema's `what_still_works.minItems: 1` and
   `risk_after_cutoff.minItems: 1` requirements plus the closed
   class vocabularies.
2. *Generic "upgrade" button collapses scan, preview, backup, apply,
   rollback, dry-run, and export into one action.* — Refused by the
   migration-helper handoff's per-class uniqueness rules and the
   apply-requires-rollback-and-backup pairing rules.
3. *Apply runs without a one-click backup or checkpoint when durable
   user state is about to change.* — Refused by the backup-requirement
   pairing rule on `apply_planned_changes` actions whose
   `mutation_class` alters durable state or compatibility mode.
4. *Breaking compatibility ahead lands on stable as a surprise.* —
   Refused by the breaking-ahead safest-next-step gate (export /
   migrate / handoff / pause only) and the export-paths floor.
5. *Past end-of-support notice claims `not_applicable_pre_release_only`
   risk.* — Refused by the per-class allOf rule forbidding that
   risk class on past notices.
6. *Approaching notice quotes the not-yet-set cutoff kind.* — Refused
   by the per-class allOf rule restricting approaching, past, and
   active-deprecation notices to the four bounded cutoff kinds.
7. *Active deprecation row drops the named replacement.* — Refused by
   the category row's per-state allOf rule forcing
   `named_replacement_label` non-null on
   `actively_deprecated_with_replacement_named` rows.
8. *Migration / import workflow imports into a build that has crossed
   end of support without showing the destination notice.* — Refused
   by the `migration_or_import_workflow_banner` entry in the required
   surface set floor.
9. *Compatibility-window report cannot be exported before the platform
   change ships.* — Refused by the report's `export_paths.minItems: 1`
   requirement.
10. *Rollback action cites no rollback target.* — Refused by the
    per-action allOf rule forcing both `rollback_ref` and
    `checkpoint_ref` non-null on `rollback_to_checkpoint` actions.

## 12. Acceptance criteria mapped to evidence

The acceptance criteria from the spec map to the schema, doc, and
fixtures as follows:

- *End-of-support banners say what still works, what becomes higher
  risk, and what the safest next step is instead of using generic fear
  language.* — Pinned by §6 (What still works, risk after cutoff,
  safest next step), the schema's `what_still_works.minItems: 1`,
  `risk_after_cutoff.minItems: 1`, and exactly-one
  `safest_next_step` requirements, and the
  [`approaching_end_of_support_extension_compatibility_drift`](../../fixtures/release/end_of_support_cases/approaching_end_of_support_extension_compatibility_drift.yaml)
  and
  [`past_end_of_support_security_only_window_closed`](../../fixtures/release/end_of_support_cases/past_end_of_support_security_only_window_closed.yaml)
  fixtures.
- *Breaking compatibility ahead can be inspected and exported before
  the platform change ships as stable or LTS truth.* — Pinned by §3
  (Notice-class vocabulary), §6.3 (Safest next step), §10 (Export
  paths), the schema's `breaking_compatibility_ahead` allOf rule
  restricting safest_next_step to inspect / export / migrate / pause,
  the report's `export_paths.minItems: 1` requirement, and the
  [`breaking_compatibility_ahead_command_schema_v3`](../../fixtures/release/end_of_support_cases/breaking_compatibility_ahead_command_schema_v3.yaml)
  fixture.
- *Fixtures cover at least: approaching EOS, past EOS, deprecation
  active, and breaking compatibility ahead.* — Provided in
  [`/fixtures/release/end_of_support_cases/`](../../fixtures/release/end_of_support_cases).

## 13. Change control

- Adding a `notice_class`, `notice_severity_class`, `cutoff_kind_class`,
  `affected_scope_class`, `risk_after_cutoff_class`,
  `what_still_works_class`, `safest_next_step_class`,
  `required_surface_class`, `affected_category_class`,
  `category_window_state_class`, `migration_helper_action_class`,
  `action_routing_kind_class`, `mutation_class`,
  `backup_requirement_class`, or `export_path_class` value is
  additive-minor. Adding an entry requires bumping the schema version
  and updating this document in the same change.
- Repurposing an existing vocabulary value is breaking. Repurposing
  requires a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and the concurrence of the release council.
- Adding a new required surface to the floor (currently four) is
  breaking; consumers MAY add admissible secondary surfaces without a
  schema bump.
- Weakening any §11 invariant (the apply-requires-rollback-and-backup
  pairing, the breaking-ahead safest-next-step gate, the export-paths
  floor, the named-replacement-label rule, the per-cutoff-kind
  required-fields rule) is breaking.

## 14. Status

Contract is **seeded**. Every fixture is tagged `seeded` / `proposed`.
Rows are not deleted; they are superseded by a follow-on ADR / RFC
recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
