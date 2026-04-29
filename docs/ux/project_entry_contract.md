# Project-Entry Verb Matrix, Entry-Chooser Row, and Open-Flow Sheet Contract

This document freezes the **product-verb matrix** for entering a
project, the **entry-chooser row** shape every chooser surface
renders, and the **open-flow sheet** that previews target type,
policy restrictions, and resulting mode before any durable write or
trust change.

The contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI / UX Spec, or design-system style guide, those sources win and
this document plus its schemas and fixtures update in the same
change. Where a Start Center, command palette, drag-drop preview,
system-open handler, CLI / headless front-end, or workspace switcher
mints a parallel verb, row shape, or open-flow surface, this contract
wins and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ux/entry_chooser_row.schema.json`](../../schemas/ux/entry_chooser_row.schema.json)
  — boundary schema for one `entry_chooser_row_record`.
- [`/schemas/ux/open_flow_sheet.schema.json`](../../schemas/ux/open_flow_sheet.schema.json)
  — boundary schema for one `open_flow_sheet_record`.
- [`/fixtures/ux/project_entry_cases/`](../../fixtures/ux/project_entry_cases/)
  — worked cases for the file, folder, workspace, remote, template,
  archive / handoff, and recent / restore entry rows plus their
  paired open-flow sheets.

This contract composes with, and does not replace:

- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
  for `entry_verb`, `target_kind`, `resulting_mode`,
  `admission_class`, restore levels, missing-target states, and
  next-step decision hooks. M00-445 mints **no** new entry-verb,
  target-kind, resulting-mode, trust-state, admission-class,
  restore-level, missing-target-state, recovery-class, or
  next-step-decision-hook value.
- [`/docs/ux/start_center_contract.md`](./start_center_contract.md)
  and [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json)
  for the Start Center / workspace-switcher disclosure record. The
  `start_center_primary_action_record` resolves into one
  `entry_chooser_row_record` on activation.
- [`/docs/ux/workspace_entry_route_matrix.md`](./workspace_entry_route_matrix.md)
  and [`/schemas/workspace/entry_route.schema.json`](../../schemas/workspace/entry_route.schema.json)
  for the route-level `workspace_entry_route_id` and per-route
  preview / commit / failure-recovery rules. The open-flow sheet
  here is the per-row, pre-commit projection of those route rules
  onto a single user activation.
- [`/docs/ux/desktop_affordance_contract.md`](./desktop_affordance_contract.md)
  and [`/schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
  for OS-facing entry paths (file association, drag-drop, system
  open, deep link, CLI / headless). Those surfaces hand off to the
  entry chooser and the open-flow sheet defined here; they do not
  invent a private opener.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md)
  for the underlying surface-class taxonomy
  (`window_attached_sheet`, `full_sheet`, dedicated review
  surface). The open-flow sheet always renders inside one of those
  surface classes.
- [`/docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  and [`/docs/adr/0001-identity-mode-trust.md`](../adr/0001-identity-mode-trust.md)
  for trust-state and authority-delta vocabulary. Trust changes
  remain owned by the trust-prompt surface; the open-flow sheet
  only **previews** the delta before commit.

## Who reads this contract

- **Shell, Start Center, palette, drag-drop, system-open, CLI, and
  workspace-switcher authors** wiring entry rows. Every row resolves
  through one `entry_chooser_row_record` and (when the user
  activates it) one paired `open_flow_sheet_record`.
- **Designers** sizing first-launch and in-session entry copy so
  Open / Clone / Import / Add root / Restore / Resume /
  StartFromSnapshot remain distinct verbs across surfaces.
- **Docs, support, accessibility, and measurement authors**
  attributing entry behavior to the same row and sheet records the
  shell renders, so a CLI invocation, a system open, and a Start
  Center click trace to the same project_entry_action_record.

## 1. Scope

This contract freezes:

- One **project-entry verb matrix** (§3) covering the seven entry
  verbs (`open`, `clone`, `import`, `add_root`, `restore`, `resume`,
  `start_from_snapshot`) with their stable target-kind set,
  resulting-mode set, trust posture, profile default, command-id
  family, and entry-route-id family. A surface that collapses any
  two verbs into a generic "get started" / "begin" / "start" row is
  non-conforming.
- One `entry_chooser_row_record` (§4) every chooser surface emits.
  A row carries the verb, a closed entry-chooser row kind (file,
  folder, workspace, remote, template, archive / handoff, recent,
  restore), source surface, candidate target kinds, candidate
  resulting modes, trust posture, profile default, command-id
  linkage, account-opt-in posture, and the typed disclosures the
  surface must show before activation.
- One `open_flow_sheet_record` (§5) every entry activation emits
  before durable write or trust change. The sheet projects the
  route preview onto a single chooser activation and freezes the
  required disclosure axes (§5.2): target type, policy
  restrictions, result mode, destination disposition, side
  effects, profile default, trust posture, restore class, and
  fallback / cancellation actions.
- The cross-surface invariants (§6) so Start Center, palette,
  drag-drop, system open, CLI / headless, and workspace switcher
  stay semantically aligned with the same row and sheet records.

## 2. Out of Scope

- File-association registration, palette command registration, OS
  protocol-handler wiring, and per-OS open-with semantics. The
  desktop-affordance contract owns those.
- The actual Start Center, palette, drag-drop preview, or open-flow
  sheet implementation. M00-445 freezes the row and sheet records
  the implementation reads.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and shell-interaction-safety contract own the strings.
- Telemetry wire format. The onboarding measurement plan reserves
  `er.*` route ids; this contract only tags rows and sheets with
  the `entry_route_id_family` they belong to.
- Workspace-trust prompt visuals, policy-review takeover, and
  rollback-checkpoint inspection. Those are owned by the
  trust-prompt, policy-review, and rollback-checkpoint contracts;
  the open-flow sheet only **previews** their inputs.

## 3. The project-entry verb matrix

This matrix is the source of truth for which verbs Aureline exposes
as project entry. Every chooser row, open-flow sheet, command-id
family, and route-id family resolves through exactly one row of this
matrix. Collapsing rows is non-conforming.

| `entry_verb` | Allowed `target_kind` set | Allowed `resulting_mode` set | Trust posture (preview) | Profile default | Command-id family | Route-id family |
|---|---|---|---|---|---|---|
| `open` | `local_file`, `local_folder`, `local_repo_root`, `workspace_manifest`, `workset_manifest` | `single_file`, `folder`, `repo_root`, `workspace_with_roots`, `workspace_candidate`, `workset_slice` | `trust_pending_until_admission` (open never grants trust) | `sticky_for_target_or_default` | `command.workspace.open_*` | `workspace_entry.open_folder`, `workspace_entry.open_workspace` |
| `clone` | `remote_repository` | `clone_then_review`, `clone_then_open`, `clone_then_add`, `clone_only` | `trust_never_implied_by_clone` (clone is always a write to staging or a destination, never a trust grant) | `default_profile` (a clone never inherits the source repo's profile) | `command.workspace.clone_*` | `workspace_entry.clone_repository` |
| `import` | `portable_state_package`, `handoff_packet`, `competitor_config_root`, `template_or_prebuild_snapshot`, `support_bundle_replay` | `extract_then_review`, `compare_before_restore`, `apply_to_active_workspace` | `trust_unchanged_until_admit` (apply is gated until the migration result admits) | `depends_on_artifact_class` (portable-profile imports may set the active profile; handoff / support imports do not) | `command.workspace.import_*` | `workspace_entry.import` |
| `add_root` | `local_folder`, `local_repo_root`, `remote_repository`, `ssh_workspace`, `container_workspace`, `devcontainer_workspace`, `managed_cloud_workspace` | `workspace_with_roots`, `workset_slice` | `trust_per_root_admission` (each new root re-evaluates trust independently of the active workspace) | `unchanged` (add-root never silently changes the active profile) | `command.workspace.add_root_*` | `workspace_entry.open_workspace` (the multi-root variant) |
| `restore` | `recovery_checkpoint`, `local_folder`, `local_repo_root`, `workspace_manifest`, `workset_manifest`, `managed_cloud_workspace` (the prior workspace target) | `restore_last_session`, `restore_from_checkpoint` | `trust_inherited_from_target` (restore never widens trust beyond the prior target's admitted state) | `sticky_for_target` | `command.workspace.restore_*` | `workspace_entry.restore_last_session` |
| `resume` | `managed_cloud_workspace`, `ssh_workspace`, `container_workspace`, `devcontainer_workspace`, `template_or_prebuild_snapshot` | `resume_live_session`, `open_prebuild_minimal` | `trust_revalidated_at_resume` (authority is re-evaluated against the live session) | `sticky_for_target` | `command.workspace.resume_*` | `workspace_entry.resume_snapshot` |
| `start_from_snapshot` | `template_or_prebuild_snapshot` | `open_prebuild_with_setup_actions`, `open_prebuild_minimal` | `trust_pending_until_admission` (the snapshot does not pre-trust) | `default_or_template_default` (a template may declare a default profile; the user may override before commit) | `command.workspace.start_from_snapshot_*` | `workspace_entry.resume_snapshot` (the snapshot variant) |

### 3.1 Stable resulting-mode vocabulary per verb

The matrix in §3 is closed. The `entry_chooser_row.schema.json`
schema enforces it via `allOf` conditional clauses: each
`entry_chooser_row_record` whose `entry_verb_candidate` is `clone`
MUST carry a `target_kind_candidates[]` drawn only from
`{remote_repository}` and a `resulting_mode_candidates[]` drawn only
from `{clone_then_review, clone_then_open, clone_then_add,
clone_only}`. A row that mints a remote-clone result mode like
`open_then_review` is non-conforming. The same rule applies to every
other verb.

### 3.2 Verb-distinctness rules

1. **No `Get started` collapse.** A surface that renders a single
   row that resolves into more than one of `{open, clone, import,
   add_root, restore, resume, start_from_snapshot}` is
   non-conforming. The Start Center five-action rule (§3.7 of the
   Start Center contract) is the seeded projection of this rule.
2. **No silent verb promotion.** A row whose
   `entry_verb_candidate` is `open` MUST NOT silently promote to
   `clone` or `import` because the chosen target happens to be a
   remote URL or a portable package. The chooser instead routes
   the user to the right verb's row, with a typed handoff hook
   (`reroute_to_clone`, `reroute_to_import`, see §4.5).
3. **No verb-by-side-effect inference.** A row that names
   `entry_verb_candidate = open` but whose preview shows a
   side-effect class of `dependency_install` /
   `containerized_bootstrap` / `devcontainer_bootstrap` /
   `remote_attach` / `browser_auth_handshake` /
   `secret_handle_request` MUST cite its bypass path
   (`open_prebuild_minimal`, `open_plain_folder`, `set_up_later`)
   and a sticky disabled-reason if the prerequisite is missing;
   side-effects do not silently rebrand the verb.
4. **Trust posture is verb-bound.** The trust posture in §3 is
   per verb; surfaces MUST NOT widen it (e.g. a clone row that
   advertises `trust_inherited_from_target` is non-conforming).
5. **Profile default is verb-bound.** The profile-default values
   in §3 are the only allowed values per verb. A row that promises
   "carry the source repo's profile" on a `clone` row is
   non-conforming.
6. **Command-id family is verb-bound.** A row's
   `command_id_ref` MUST resolve into the family for its verb.
   Cross-family routing (e.g. resolving
   `command.workspace.clone_*` from an `open` row) is
   non-conforming.

## 4. Entry-chooser row record

Every chooser surface that lets the user pick a project entry —
Start Center, command palette, `Open Recent` submenu,
workspace-switcher, drag-drop preview, system-open handoff sheet,
CLI / headless preview — emits one `entry_chooser_row_record` per
row.

### 4.1 Required fields

- `record_kind = entry_chooser_row_record`.
- `row_id` (opaque, stable for the chooser instance).
- `entry_chooser_row_kind` — closed set (§4.2).
- `entry_chooser_surface_class` — closed set (§4.3).
- `source_surface` — re-export from entry-restore object model
  §1.6 (the value that will land on the
  `project_entry_action_record` if the row is committed).
- `entry_verb_candidate` — re-export from entry-restore object
  model §1.1.
- `target_kind_candidates[]` — non-empty subset of the verb's
  allowed target-kind set (§3).
- `resulting_mode_candidates[]` — non-empty subset of the verb's
  allowed resulting-mode set (§3).
- `trust_posture_class` — closed set (§4.4).
- `profile_default_class` — closed set (§4.6).
- `command_id_ref` — opaque ref to the command-descriptor row
  the chooser will invoke (`command.workspace.<verb>_*`). A row
  that lacks `command_id_ref` is non-conforming; entry must
  always trace to one canonical command id.
- `entry_route_id_family` — closed set drawn from the route
  matrix (`workspace_entry.open_folder`,
  `workspace_entry.open_workspace`,
  `workspace_entry.clone_repository`, `workspace_entry.import`,
  `workspace_entry.resume_snapshot`,
  `workspace_entry.restore_last_session`,
  `workspace_entry.deep_link`,
  `workspace_entry.open_in_safe_mode`,
  `workspace_entry.continue_in_restricted_mode`).
- `account_opt_in_posture` — re-export from Start Center §3.3
  (`optional_local_path_available`, `required_for_this_row`,
  `deferred_review_pending`, `unavailable_in_this_envelope`).
- `next_step_decision_hooks[]` — at least one; drawn from the
  entry-restore §1.7 closed set.
- `disclosed_in_zone` — `start_center_zone` (when the chooser is
  Start Center / workspace-switcher) or the surface-local zone id
  when the chooser is palette / drag-drop / system-open / CLI.
- `keyboard_reachable` — boolean; required `true` on every row
  on a non-takeover surface.
- `presentation_label` — redaction-aware (≤ 1024 graphemes).

Optional fields: `presentation_subtitle`, `disabled_reason_code`,
`recent_work_entry_ref` (when `entry_chooser_row_kind` is
`recent`), `restore_prompt_ref` (when `entry_chooser_row_kind` is
`restore`), `artifact_descriptor_ref` (when row kind is
`archive_or_handoff` or `template`), `remote_target_descriptor_ref`
(when row kind is `remote`), `deep_link_descriptor_ref` (when
`source_surface` is `deep_link`), `reroute_to_verb` (§4.5).

### 4.2 `entry_chooser_row_kind` (closed)

The user-visible row class. Bound to a verb only via §4.7.

- `file` — single local file. Verb is always `open`.
- `folder` — local folder or local repo root. Verb is always
  `open` (or `add_root` when the chooser surface is the
  workspace-switcher in an active workspace; see §4.7 row 2).
- `workspace` — workspace or workset manifest. Verb is always
  `open`.
- `remote` — remote repository, ssh / container / devcontainer /
  managed-cloud target. Verb is `clone` for non-attach remotes,
  `add_root` when a remote is being widened into the active
  workspace, `resume` for managed-cloud / SSH /
  (dev)container live-attach, or `open` when the remote target
  resolves to an already-cloned local repo root.
- `template` — template or prebuild snapshot. Verb is
  `start_from_snapshot` for new entries, `resume` for
  re-attaching a live prebuild instance.
- `archive_or_handoff` — portable state package, handoff
  packet, competitor config root, support bundle replay. Verb is
  always `import`.
- `recent` — recent-work row. Verb is the verb the recent
  resolves into (typically `open` or `resume`); see §4.7 row 5.
- `restore` — restore-prompt row. Verb is always `restore`.

A surface that mints a ninth row kind (`projects_hub`,
`favorites`, `cloud_dashboard`, etc.) is non-conforming.

### 4.3 `entry_chooser_surface_class` (closed)

Which chooser surface the row is rendered in. The set is closed:

- `start_center` — first-launch / no-prior-session landing.
- `workspace_switcher_palette`,
  `workspace_switcher_menu`,
  `workspace_switcher_dedicated_view` — the three switcher
  families from Start Center §3.1.
- `command_palette` — command-palette hosted entry rows
  (`Open…`, `Clone…`, `Import…`, etc.).
- `open_recent_submenu` — main-menu `Open Recent` submenu and
  jump-list / dock recents.
- `drag_drop_preview` — preview surface shown when a target is
  dragged onto an Aureline window before drop is accepted.
- `system_open_handoff_sheet` — sheet rendered when the OS
  hands off a target via file association, open-with, or
  reveal-from-shell.
- `cli_headless_preview` — text / JSON preview rendered by the
  CLI or headless front-end before the entry commits (e.g.
  `aureline open --preview`, `aureline import --dry-run`).
- `deep_link_intent_review` — surface rendered when a deep link
  resolves to an entry candidate awaiting review.

A surface that introduces a tenth class is non-conforming.

### 4.4 `trust_posture_class` (closed)

Closed set of pre-commit trust postures the row advertises. Each
verb's allowed value is fixed by §3:

- `trust_pending_until_admission` — the entry will leave trust
  in `pending_evaluation` until the trust prompt admits it.
  Used by `open` and `start_from_snapshot`.
- `trust_never_implied_by_clone` — the clone produces bytes in
  staging or a destination but never grants trust; the resulting
  workspace re-enters trust review on first open.
- `trust_unchanged_until_admit` — import never widens trust
  until the migration-result review admits items.
- `trust_per_root_admission` — each new root re-evaluates trust
  independently from the active workspace.
- `trust_inherited_from_target` — restore reads the prior
  target's admitted trust state; it never widens it.
- `trust_revalidated_at_resume` — resume re-evaluates authority
  against the live session; expired authority blocks the resume.

### 4.5 `reroute_to_verb` (optional)

Some chooser activations resolve to a verb different from the row
the user clicked (e.g. dragging a `.tar.zst` archive onto a folder
row reroutes from `open` to `import`). When the chooser detects a
reroute it MUST emit a row with `reroute_to_verb` set and a
typed `next_step_decision_hook`:

- `reroute_to_verb` is one of the seven verbs in §3.
- `next_step_decision_hooks[]` MUST include exactly one of
  `review_trust_and_open`, `review_migration_report`,
  `compare_before_restore`, `review_archetype_match`, or
  `review_unsupported_items` to name what the rerouted activation
  will review.

### 4.6 `profile_default_class` (closed)

Closed set of profile-default postures a row advertises. Each
verb's allowed value is fixed by §3:

- `default_profile` — the profile baseline is the deployment
  default. Used by `clone` (default).
- `last_active_profile` — the profile baseline is the last
  active profile in this build / device.
- `sticky_for_target` — the profile baseline is whichever
  profile was last sticky for this exact target. Used by
  `restore` and `resume`.
- `sticky_for_target_or_default` — sticky if available, else
  the deployment default. Used by `open`.
- `unchanged` — the active profile is preserved untouched. Used
  by `add_root`.
- `depends_on_artifact_class` — profile baseline is computed
  from the import artifact's `artifact_class`. Used by `import`.
  A portable-profile import may set the active profile; a
  handoff / competitor / support-bundle import never silently
  retargets the active profile.
- `default_or_template_default` — the deployment default unless
  the template / prebuild snapshot declares one. Used by
  `start_from_snapshot`. A template-declared profile MUST be
  user-overridable in the open-flow sheet.
- `locked_profile_required` — managed envelopes may pin a
  required profile; the row MUST disclose this and route to
  `unavailable_in_this_envelope` if the user lacks the profile.

### 4.7 Row-kind ↔ verb binding rules

1. `file`, `workspace` rows MUST carry `entry_verb_candidate =
   open`.
2. `folder` rows carry `open` when the surface is Start Center /
   palette / system-open / drag-drop on first open; they carry
   `add_root` only when the surface is `workspace_switcher_*`
   inside an active workspace.
3. `remote` rows carry `clone` by default; `add_root` when the
   remote is being widened into the active workspace; `resume`
   for managed-cloud / SSH / (dev)container live-attach; `open`
   only when the remote target resolves to an already-cloned
   local repo root.
4. `template` rows carry `start_from_snapshot` for new entries,
   `resume` for re-attaching a live prebuild instance.
5. `recent` rows carry the verb the recent resolves into (which
   the upstream `recent_work_entry_record` declares via its
   `target_kind` and `restore_availability`); the chooser MUST
   NOT collapse a recent into a generic `Reopen` row.
6. `restore` rows carry `restore`. A row that promises `Restore`
   without a `restore_prompt_ref` is non-conforming.
7. `archive_or_handoff` rows carry `import`.
8. A row that violates 1–7 is non-conforming.

### 4.8 Disclosure rules

1. **Account opt-in disclosed in-place.** A row whose
   `account_opt_in_posture` is not
   `optional_local_path_available` MUST name the remedy hook
   verbatim from `next_step_decision_hooks[]`.
2. **Trust posture disclosed in-place.** A row's
   `trust_posture_class` is rendered as a chip / subtitle on the
   row; a row that hides the trust delta until after click is
   non-conforming.
3. **Profile default disclosed in-place when non-default.** A
   row whose `profile_default_class` is `unchanged`,
   `sticky_for_target`, `depends_on_artifact_class`,
   `default_or_template_default`, or `locked_profile_required`
   MUST disclose the class on the row when it differs from the
   deployment default.
4. **Disabled rows cite typed reason.** A row rendered visible-
   but-disabled MUST carry a `disabled_reason_code`. Free-form
   "Not available" copy is non-conforming.
5. **Same row across surfaces.** When the same logical entry is
   reachable from two chooser surfaces, both surfaces MUST emit
   row records with the same `entry_verb_candidate`,
   `target_kind_candidates[]`, `resulting_mode_candidates[]`,
   `trust_posture_class`, `profile_default_class`,
   `command_id_ref`, and `entry_route_id_family`. Surface-local
   chrome (icon, accelerator, accent) MAY differ.

## 5. Open-flow sheet record

Every chooser-row activation that may write bytes, change trust,
attach a runtime, retarget a profile, or rehydrate a session emits
one `open_flow_sheet_record` BEFORE the durable change. The sheet
is the per-row, pre-commit projection of the upstream route preview
contract (workspace-entry route matrix §Preview Contract) onto the
exact target the user picked.

Activations that are pure no-write, no-trust-change, no-runtime-
attach, no-profile-retarget operations (e.g. `Open file` on a local
text file in an already-trusted workspace) MAY skip the sheet only
when their `open_flow_sheet_class` resolves to
`open_local_target_no_review_required` AND every required disclosure
axis (§5.2) resolves to a no-op class. Skipping the sheet for any
other class is non-conforming.

### 5.1 `open_flow_sheet_class` (closed)

- `open_local_target` — local file / folder / repo root /
  workspace / workset open. Sheet shows target identity, trust
  posture, restore availability when present.
- `open_local_target_no_review_required` — pure local open with
  no trust delta, no restore prompt, no destination write, no
  runtime attach, no profile retarget. Sheet may be elided per
  §5 rule 2.
- `clone_remote_target` — remote clone. Sheet shows host, auth
  posture, destination disposition, collision class, network /
  proxy / mirror posture, submodule / LFS / bootstrap side
  effects, and the post-clone resulting mode.
- `import_artifact` — portable / handoff / competitor / support
  / template import. Sheet shows artifact class, producer /
  signature posture, affected scope, blocked items, comparison
  or dry-run result, rollback class, and side effects.
- `add_root_to_active_workspace` — sheet shows the new root's
  identity, trust posture, policy delta, and whether adding the
  root changes the workspace boundary.
- `restore_or_resume` — sheet shows restore level, missing-
  target states, dirty-buffer summary, per-pane execution
  posture, recovery class, and `open_without_restore` /
  `compare_before_restore` bypass paths.
- `start_from_template_or_prebuild` — sheet shows template /
  prebuild identity, set-up actions, bypass path, profile
  default (and override affordance), and resulting mode.
- `deep_link_intent_review` — sheet shows origin, handler
  ownership, route class, target identity, replay posture, trust
  / policy / tenant scope, authority delta, and the typed degraded
  fallback. The sheet always renders for deep links resolving
  to a non-`authority_delta = none` route.

### 5.2 Required disclosure axes

Every `open_flow_sheet_record` carries the following axes. A
sheet that cannot populate an axis MUST deny with a typed
`review_requirement` instead of defaulting to a generic
"open anyway" path.

- `target_type_disclosure` — `target_kind` and (where
  available) `object_identity_ref` /
  `filesystem_identity_ref` / `remote_target_descriptor` /
  `artifact_descriptor`. Required on every sheet.
- `policy_restrictions_disclosure` — current `trust_state`,
  resulting `trust_state` if known, `authority_delta` (one of
  `none`, `narrows`, `widens`, `revalidates`, `re-binds`),
  `policy_epoch_ref`, and any blocking
  `policy_state_disclosure`. Required on every sheet.
- `result_mode_disclosure` — the single
  `resulting_mode` the activation will materialize on commit.
  When the row advertised more than one
  `resulting_mode_candidate` (e.g. `clone_then_review` vs
  `clone_then_open`) the sheet MUST resolve to one before
  commit; an unresolved candidate set on commit is non-
  conforming.
- `destination_disposition_disclosure` —
  `destination_disposition` and `collision_class`. Required on
  every sheet whose `open_flow_sheet_class` may write bytes
  (`clone_remote_target`, `import_artifact`,
  `start_from_template_or_prebuild`, and any
  `restore_or_resume` whose recovery class writes durable bytes).
- `side_effect_disclosure` — `side_effect_envelope` from the
  upstream entry-restore object model
  (`setup_actions_class`, `time_class`,
  `connectivity_class`, `cleanup_class`, `bypass_path`).
  Required when the row's verb may produce setup actions.
- `profile_default_disclosure` — the resolved
  `profile_default_class` (§4.6) and, when applicable, an
  `override_affordance` (boolean) the user may toggle before
  commit. Required on every sheet whose verb is `import`,
  `start_from_snapshot`, or `clone` (when a deployment binds a
  default profile to clones).
- `trust_posture_disclosure` — the row's
  `trust_posture_class` (§4.4) and, on commit, whether the
  trust prompt will be invoked next. Required on every sheet.
- `restore_class_disclosure` — `restore_level`,
  `missing_target_states[]`, per-pane
  `session_execution_posture`, and
  `checkpoint_linked_recovery_class`. Required on every
  `restore_or_resume` sheet.
- `fallback_disclosure` — at least one typed cancellation,
  rollback, restricted-continuation, safe-open, inspect-only,
  or previous-workspace action drawn from the upstream
  fallback set. Required on every sheet.

### 5.3 Pre-commit invariants

A surface that violates any of the following is non-conforming:

1. **No durable write before sheet commit.** No durable byte
   may land on disk before the user commits the sheet. Labelled
   staging is allowed (`destination_disposition =
   write_to_labelled_staging`) and MUST be disclosed.
2. **No trust change before sheet commit.** `trust_state` MUST
   NOT widen before the user commits the sheet; the sheet
   previews the resulting trust state, but the trust transition
   is owned by the trust-prompt surface.
3. **No runtime attach before sheet commit.** Devcontainer /
   container / SSH / managed-cloud attach, browser auth
   handshakes, and secret-handle requests MUST NOT execute
   before sheet commit. The sheet discloses the side effect; the
   commit triggers it.
4. **No profile retarget before sheet commit.** The active
   profile MUST NOT change before the user commits a sheet whose
   `profile_default_class` is non-`unchanged`. A sheet whose
   `profile_default_disclosure.override_affordance = true` MUST
   surface the override in-place.
5. **No verb mutation after row activation.** The row's
   `entry_verb_candidate` is the verb the sheet commits. A
   sheet that promotes `open` to `clone` after activation is
   non-conforming; reroutes are emitted as a new row record per
   §4.5.
6. **Fallback always reachable.** The sheet exposes at least
   one typed fallback action drawn from §5.2's
   `fallback_disclosure`. A sheet whose only commit affordance
   is the primary action (no cancel / no previous-workspace /
   no inspect-only) is non-conforming.
7. **Keyboard reachable.** Every commit, fallback, and override
   affordance MUST be keyboard reachable.

### 5.4 Sheet surface class

The `open_flow_sheet_record` resolves into one of the surface
classes from the dialog / sheet contract:

- `window_attached_sheet` — default for
  `open_local_target`, `add_root_to_active_workspace`,
  `restore_or_resume` (small), and short clones / imports.
- `full_sheet` — required when the disclosure axes don't fit
  with the primary action visible (large clone, large import,
  template / prebuild with multiple set-up actions).
- `dedicated_review_surface` — required when the activation
  needs side-by-side comparison (import compare, restore vs
  open clean, deep-link intent review with replay-deny
  evidence).
- `cli_headless_text_block` — the CLI / headless rendering of
  the same record. The text block reads the same fields; it
  MUST NOT collapse axes that the GUI sheet keeps separate.

A surface that nests a second product-owned dialog or sheet on
top of the open-flow sheet is non-conforming. Platform auth and
platform file-picker overlays remain the only allowed nested
overlays per the dialog / sheet contract.

## 6. Cross-surface invariants

The verb matrix, row record, and sheet record are projected into
every chooser surface. The following invariants keep the projection
sound:

1. **One row record per surface row.** Every visible chooser row
   on every surface emits one `entry_chooser_row_record`. A
   surface that renders an entry verb without a backing record is
   non-conforming.
2. **One sheet per activation.** Every committing activation
   that may write, change trust, attach a runtime, retarget a
   profile, or rehydrate a session emits one
   `open_flow_sheet_record` BEFORE the durable change. Pure
   no-op opens (§5 rule 2) are the only exception.
3. **Same record across surfaces.** When the same logical entry
   is reachable from Start Center, palette, drag-drop preview,
   system-open handoff, and CLI / headless preview, the row
   records emitted on each surface MUST agree on the seven
   verb-bound fields named in §4.8 rule 5. Surface-local chrome
   may differ; semantics may not.
4. **Deep links route through the chooser.** A deep-link entry
   MUST emit a `deep_link_intent_review` row before any commit;
   it MUST NOT call a private opener. The deep-link descriptor
   reference rides on the row; the open-flow sheet renders the
   intent review.
5. **CLI / headless render the same axes.** A CLI / headless
   preview MUST render the same `open_flow_sheet_record`
   disclosure axes as the GUI sheet. Collapsing axes (e.g.
   omitting `policy_restrictions_disclosure`) is non-conforming.
6. **Drag-drop is a chooser surface, not a shortcut.** A drop
   MUST resolve into one row record and (when committing) one
   sheet record. A drop that silently writes bytes, changes
   trust, attaches a runtime, or retargets a profile bypasses
   §5.3 and is non-conforming.
7. **Recent and Restore stay distinct.** A `recent` row MUST
   NOT collapse into a `restore` row even when the upstream
   recent has a pending restore prompt. The chooser renders both
   rows, and activating the recent emits a row record whose
   verb matches the recent's `target_kind` (typically `open` or
   `resume`); the restore prompt is a separate row.
8. **Verb command-id linkage stable.** Each verb's command-id
   family is fixed by §3 and the command registry. A row that
   resolves into a different family than its verb declares is
   non-conforming.
9. **Entry-route id family stable.** Each verb maps to one or
   more route ids in the workspace-entry route matrix (§3, last
   column). A row whose `entry_route_id_family` does not match
   its verb is non-conforming.

## 7. Fixture corpus

The fixture corpus under `/fixtures/ux/project_entry_cases/`
contains one row record (and, when applicable, one paired sheet
record) per required entry chooser row kind:

- `entry_row_open_local_file_start_center.json` — `file` row +
  `open_local_target` sheet.
- `entry_row_open_local_folder_drag_drop.json` — `folder` row +
  `open_local_target` sheet (drag-drop preview).
- `entry_row_open_workspace_palette.json` — `workspace` row +
  `open_local_target` sheet (command palette).
- `entry_row_clone_remote_repository_start_center.json` —
  `remote` row + `clone_remote_target` sheet.
- `entry_row_start_from_template_snapshot.json` — `template`
  row + `start_from_template_or_prebuild` sheet.
- `entry_row_import_handoff_archive_compare.json` —
  `archive_or_handoff` row + `import_artifact` sheet.
- `entry_row_recent_work_remote_resume.json` — `recent` row +
  `restore_or_resume` sheet (managed-cloud resume).
- `entry_row_restore_last_session.json` — `restore` row +
  `restore_or_resume` sheet (restore-card pairing).
- `entry_row_add_root_workspace_switcher.json` — `folder` /
  `remote` row + `add_root_to_active_workspace` sheet.
- `entry_row_deep_link_intent_review.json` —
  `deep_link_intent_review` row + sheet.
- `entry_row_cli_headless_open_preview.json` — CLI / headless
  text-block render of an open-folder row + sheet.

Each fixture is a JSON document validated by the relevant boundary
schema and includes a `__fixture__` prelude naming the scenario,
the row kind exercised, the disclosure axes covered, and the
contract sections asserted.

## 8. Versioning and change control

The schemas declare `entry_chooser_row_schema_version = 1` and
`open_flow_sheet_schema_version = 1`. Adding a new
`entry_chooser_row_kind`, `entry_chooser_surface_class`,
`trust_posture_class`, `profile_default_class`,
`open_flow_sheet_class`, or disclosure axis is **additive-minor**
and bumps the schema version. Repurposing an existing value, or
weakening a §5.3 invariant, is **breaking** and requires a new
ADR row plus a coordinated update of the entry-restore object
model and the workspace-entry route matrix.
