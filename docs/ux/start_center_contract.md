# Start Center, workspace-switcher, and open-flow disclosure contract

This document freezes the cross-surface disclosure contract every
**Start Center**, **workspace-switcher**, **open-flow sheet**,
**restore prompt**, and **recent-work list** inherits before the
startup wedge is implemented. The goal is an honest, low-friction
startup surface: `Open`, `Clone`, `Import`, `Restore`, and
`Recent work` remain distinct first-class actions; the shell never
routes first-run through a sign-in wall; restore prompts never
collapse exact, dirty-buffer, checkpoint-rollback, and evidence-only
recovery into one generic reopen CTA; account, service, and policy
state is disclosed in-place rather than inferred from absence.

The machine-readable schema lives at:

- [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/start_center_rows/`](../../fixtures/ux/start_center_rows/)

This contract is normative for the disclosure posture. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, or milestone document,
those sources win and this document plus its companion schema and
fixtures update in the same change. Where a downstream Start Center,
workspace-switcher, open-flow, or restore-card surface mints a
parallel vocabulary, this contract wins and the surface is
non-conforming.

This contract mints **no** new entry-verb, target-kind,
resulting-mode, trust-state, admission-class, restore-level,
missing-target-state, recovery-class, or next-step-decision-hook
values. Every row here re-exports — by reference — the vocabulary
frozen in
[`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
(§1–§4) and quotes the startup-state tokens from
[`/docs/ux/entry_restore_truth_audit.md`](./entry_restore_truth_audit.md)
(§6). Navigation routes resolve through
[`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md);
the Start Center is one route, not the only route, and every
primary action it offers remains reachable later from the command
palette, main menu, `Open Recent`, and `Switch Project`.

## Who reads this contract

- **Shell and startup-wedge authors** wiring the Start Center, the
  workspace-switcher palette / menu / dedicated view, the open-flow
  sheet, and the first-launch restore card. Every row on those
  surfaces resolves through a record shape defined here.
- **Designers** sizing first-launch copy, chips, and placements so
  account / service / policy disclosures land in-place and the
  primary work-resume actions are never below account nags,
  release-marketing cards, or marketplace-first content.
- **Docs, support, and measurement authors** attributing first-run,
  first-open, restore-success, and opt-in-boundary evidence to the
  same record kinds the shell renders.

## 1. Scope

- Freeze one `start_center_surface_record` that every first-launch
  landing, empty-profile landing, no-prior-session landing, and
  re-entered Start Center view reads.
- Freeze one `start_center_primary_action_record` per first-launch
  primary action (`Open folder`, `Open workspace`,
  `Clone repository`, `Restore last session`, `Import from…`) and
  per workspace-switcher primary action (`Add root`,
  `Start from template or prebuild`). The five first-launch
  actions are required on every non-takeover surface state.
- Freeze one `recent_work_row_disclosure_record` that wraps the
  `recent_work_entry_record` (from entry_restore_object_model §2)
  with typed freshness, absence, account-opt-in, and
  privacy-reduction posture. The wrapped row never contradicts the
  wrapped record; when they disagree, the record wins and the
  wrapping surface is non-conforming.
- Freeze one `restore_card_record` that summarises a pending
  `restore_prompt_record` with counts by restore level, recovery
  class, and per-pane session-execution posture, and reserves the
  three primary actions `Restore now`, `Skip once`, and
  `Open clean` — never one generic `Reopen`.
- Freeze one `disclosure_banner_record` for account, service,
  mirror, extension-host, offline, policy, privacy-reduction, and
  build / channel-identity disclosures that may ride on the Start
  Center or the workspace-switcher. Every banner cites its
  `disclosure_class`, resolution posture, and allowed placement
  zone.
- Freeze one `workspace_switcher_view_record` that names the
  switcher's host route (palette, main menu, dedicated view) and
  re-advertises each primary action so the same verbs remain
  reachable later in the session.
- Name the closed zone vocabulary (§3) that bounds **where**
  account nags, release-marketing, and marketplace-first content
  may NOT appear relative to primary work-resume actions.

## 2. Out of scope

- Final visuals (exact icons, rail widths, card padding). The
  style guide and shell-zone contract own those.
- The actual Start Center, workspace-switcher palette, or open-flow
  sheet implementation. Those are later milestones; the disclosure
  and record shapes freeze here first.
- Per-OS system-menu, dock, jump-list, or share-sheet routing for
  Start Center entry. The platform-adapter contract owns that.
- Final user-facing copy / microcopy. The shell-interaction-safety
  contract and the UX style guide own the exact strings; this
  contract pins the closed sets the copy resolves against.
- Telemetry wire format. The onboarding measurement plan reserves
  the event names; this contract only tags records with the
  entry-route id the plan cites.

## 3. Frozen vocabulary (re-exported)

This contract mints no new entry-restore vocabulary. It re-exports
by reference:

- `entry_verb`, `target_kind`, `resulting_mode`, `source_surface`,
  `admission_class`, `destination_disposition`, `collision_class`,
  `next_step_decision_hook`, `side_effect_envelope` — entry-restore
  object model §1.
- `recent_work_target_state`, `portability_class`,
  `restore_availability`, `safe_recovery_action`,
  `recent_work_entry_record` — §2.
- `restore_level`, `missing_target_state`,
  `session_execution_posture`, `checkpoint_linked_recovery_class`
  — §3–§4.
- `trust_state` — ADR-0001.
- `startup_state` tokens (e.g. `startup_state:first_run`,
  `startup_state:offline_startup`, `startup_state:unsupported_startup`,
  `startup_state:reopen_with_pending_restore`,
  `startup_state:restore_failed`, `startup_state:restore_skipped`,
  `startup_state:open_without_restore`,
  `startup_state:empty_state_or_placeholder_transition`) —
  entry-restore truth audit §6.
- `navigation_route_id`, `escalation_tier`, `disclosure_depth` —
  navigation and escalation contract §3.

This contract introduces five small vocabularies that are scoped
to startup surfaces and never substitute for any frozen upstream
vocabulary:

### 3.1 `start_center_surface_family`

Which startup-wedge surface is being described. The set is closed:

- `start_center` — the first-launch / no-prior-session landing.
- `workspace_switcher_palette` — the command-palette-hosted
  switcher route.
- `workspace_switcher_menu` — the main-menu-hosted
  `Switch Project` / `Open Recent` submenu.
- `workspace_switcher_dedicated_view` — a full-surface switcher
  view (e.g. after explicit `Close folder` or explicit switcher
  open).
- `open_flow_sheet` — the `Open / Open Workspace` file-picker
  sheet that hosts the open-flow disclosure band.
- `restore_card` — the standalone restore card a
  `restore_prompt_record` renders into.

Rules (frozen):

1. A surface that mints a sixth family (`welcome_dashboard`,
   `home_tab`, `projects_hub`, etc.) is non-conforming; the six
   families above are the full set.
2. `workspace_switcher_menu`, `workspace_switcher_palette`, and
   `workspace_switcher_dedicated_view` MUST re-advertise the same
   primary actions the `start_center` family advertises on the
   same profile — no surface may silently drop verbs the other
   offers.

### 3.2 `start_center_zone`

Closed placement zones on the Start Center and workspace-switcher
surfaces. Account nags, release-marketing, and marketplace-first
content are **forbidden** above the `primary_work_resume` zone on
seeded startup states.

- `primary_work_resume` — hosts the five first-launch primary
  actions and the restore card (when one is pending). Always
  rendered above every other zone on seeded startup states.
- `recent_work_list` — recent-work rows. Sits below
  `primary_work_resume`.
- `secondary_entry` — `Start from template or prebuild`, CLI-hint
  row, deep-link reentry row. Below `recent_work_list`.
- `profile_selection` — profile / account switcher. May sit
  alongside `primary_work_resume` on the right or below the
  secondary-entry zone; never above `primary_work_resume`.
- `disclosure_band` — account, service, policy, mirror, and
  privacy-reduction disclosures that are not a primary action.
  May sit above `primary_work_resume` **only** when the
  disclosure is blocking (`required` account or policy posture,
  mirror-only / air-gapped constraint, privacy-reduced mode
  active on a sensitive-environment launch). Non-blocking
  disclosures (optional opt-ins, release-marketing,
  marketplace-first) are forbidden in this zone.
- `help_and_build_identity_footer` — build / channel identity,
  "What's new" entry point, `Safe mode`, docs link. Always last.

Rules (frozen):

1. **No account nags above work resume.** A row whose
   `disclosure_class` is `account_state_disclosure` and whose
   `account_opt_in_posture` is `optional_local_path_available`
   MAY NOT render in a zone above `primary_work_resume`.
2. **No release-marketing above work resume.** Release-marketing
   cards, `What's new` hero cards, and tip-of-the-day rows MAY
   render only in `help_and_build_identity_footer`.
3. **No marketplace-first above work resume.** Marketplace
   browse, extension-recommendation carousel, and provider-
   linked promos MAY render only in `secondary_entry` or
   `help_and_build_identity_footer`, and never as the top row.
4. **Blocking disclosures remain in-place.** A blocking
   disclosure (`required_for_this_row`, `policy_blocked`,
   `unavailable_in_this_envelope`, `sensitive_environment_clear_on_launch`)
   renders in `disclosure_band` above `primary_work_resume` and
   quotes the remedy hook inline.

### 3.3 `account_opt_in_posture`

For every row that exposes account / service / sync / provider
state. The set is closed:

- `optional_local_path_available` — the row's primary action
  works without account sign-in; sign-in is a **side** affordance
  offered in-place. First-run and every local-only recent-work
  row MUST resolve here.
- `required_for_this_row` — the row cannot proceed without
  account or service opt-in (e.g. clone of a private repo that
  needs OAuth handle; resume of a managed-cloud workspace with
  expired authority). The row MUST name the required remedy hook
  (`reauth_required`, `reconnect_required`, `review_trust_and_open`)
  verbatim from the frozen `next_step_decision_hook` set.
- `deferred_review_pending` — opt-in is being reviewed (connected-
  provider approval ticket, admin policy review, post-import
  validation). The row MUST NOT claim the opt-in is complete.
- `unavailable_in_this_envelope` — the opt-in cannot be
  attempted in this envelope (air-gapped mirror, managed profile
  that forbids the service, restricted mode). The row MUST NOT
  present an opt-in affordance that will simply fail; it names
  the envelope class instead.

Rules (frozen):

1. **No-account-free-row drift.** A row tagged
   `optional_local_path_available` that routes through a sign-in
   wall for the primary action is non-conforming. Start Center
   and workspace-switcher surfaces MUST keep at least one
   primary action (`Open folder` minimum) usable with
   `optional_local_path_available` on every seeded surface state.
2. **No forced-first-run sign-in.** On `startup_state:first_run`,
   no primary-action row may render with
   `account_opt_in_posture = required_for_this_row` unless the
   managed envelope forces it (`unavailable_in_this_envelope` is
   then named explicitly and a local path is preserved).
3. **Remedy hooks are typed.** Every non-`optional_local_path_available`
   row names at least one `next_step_decision_hook` verbatim.

### 3.4 `freshness_class` and `absence_class`

Disclosure axes on recent-work rows, cached onboarding/help packs,
templates / prebuilds, and not-installed docs content. Both sets
are closed.

Freshness:

- `live` — the canonical owner answered within the freshness
  window and the row reflects the current state.
- `cached_with_timestamp` — the row is served from cache; the
  row MUST expose a last-validated timestamp class
  (`minutes_ago`, `hours_ago`, `days_ago`, `older`).
- `stale_offline` — the row was last validated while online but
  the current envelope is offline; the row MUST NOT claim
  `reachable` on a remote target.
- `unknown_since` — the canonical owner has not been re-reached
  since a disruption (device sleep, mirror refresh failure, auth
  lapse). Chip reads `unknown`, never `reachable`.
- `not_installed` — applies to cached docs / help packs, extension
  curated knowledge packs, and templates / prebuilds that are
  referenced but not materialised locally. The row MUST expose a
  `fetch_deferred` action rather than silently fail on click.
- `fetch_deferred` — user or policy deferred fetch. The row
  discloses the deferral.

Absence (only present when the row's target is not materialised):

- `never_acquired`
- `not_yet_materialised`
- `removed_by_user`
- `policy_blocked`
- `unreachable`
- `quarantined`
- `superseded_by_newer`

Rules (frozen):

1. A row that carries `freshness_class = cached_with_timestamp`
   MUST expose the timestamp class in the row subtitle or chip;
   hiding the cache age is non-conforming.
2. A row that carries an `absence_class` MUST render a typed
   safe-recovery action from the upstream
   `safe_recovery_action` set; generic `Try again` is
   non-conforming.
3. Cached onboarding / help packs, templates / prebuilds, and
   not-installed docs content MUST disclose `freshness_class`
   and — when absent — `absence_class` before the row is
   clickable. A row that promises content it cannot deliver is
   non-conforming.

### 3.5 `privacy_reduction_mode`

Closed set of privacy-reduction postures Start Center and the
workspace-switcher MAY enter, typically on sensitive-environment
launches or explicit user request:

- `default` — no reduction applied.
- `hide_paths` — `presentation_subtitle` redacted to class
  (`local_folder`, `remote_repository`, `managed_workspace`)
  without raw paths or URLs.
- `hide_recent_work` — the `recent_work_list` zone is hidden;
  `Open` and `Clone` remain reachable via primary actions.
- `hide_account_affordances` — account, sign-in, and provider-
  linked affordances are hidden; local-only work paths remain.
- `hide_all_except_open_and_clone` — Start Center renders only
  `Open folder` and `Clone repository`, plus the build /
  channel-identity footer and `Safe mode`.
- `sensitive_environment_clear_on_launch` — the surface
  announces that recent work, restore prompts, and cached hints
  were intentionally cleared at launch for this envelope.

Rules (frozen):

1. Privacy reduction is a **shell-visible** posture; it MUST
   render a `privacy_reduced_mode_notice` disclosure banner in
   `disclosure_band` above `primary_work_resume` when any
   non-`default` value is active.
2. A surface that silently hides rows without a visible notice
   is non-conforming (users cannot distinguish a privacy-hidden
   row from a missing row).
3. Every privacy-reduction mode MUST still expose at least
   `Open folder` and `Clone repository` as primary actions; the
   "clear on launch" posture MUST NOT strand the user without
   an entry verb.

### 3.6 Clear-controls and sensitive-environment posture

For sensitive environments (shared devices, kiosk, dogfood
review seats, public demos), Start Center surfaces MUST expose:

- A visible `Clear recent work` control resolving to the
  `remove_from_recents` safe-recovery action with a confirm
  step (preview class `listed_preview`); never an immediate
  wipe without confirmation.
- A visible `Clear cached onboarding / help` control resolving
  to cache purge of the cached onboarding / help packs (only);
  never purging a project-local pack.
- A visible `Exit privacy-reduced mode` control when a
  non-`default` privacy-reduction mode is active.

### 3.7 `primary_action_id`

Closed set of primary-action ids the Start Center and workspace-
switcher advertise. The five first-launch primary actions are
required on every non-takeover Start Center state.

- `primary_action.open_folder` — target_kind `local_folder` or
  `local_repo_root`; resulting_mode `folder` / `repo_root`.
- `primary_action.open_workspace` — target_kind
  `workspace_manifest`; resulting_mode `workspace_with_roots`
  or `workspace_candidate`.
- `primary_action.clone_repository` — target_kind
  `remote_repository`; resulting_mode `clone_then_review` /
  `clone_then_open` (never `clone_only` without an advertised
  follow-up).
- `primary_action.restore_last_session` — entry_verb `restore`;
  resulting_mode `restore_last_session` /
  `restore_from_checkpoint`. Always pairs with a
  `restore_card_record`.
- `primary_action.import_from` — entry_verb `import`;
  target_kind `portable_state_package` / `handoff_packet` /
  `competitor_config_root` / `template_or_prebuild_snapshot`.
- `primary_action.add_root` — workspace-switcher only; widens
  the active workspace with a new root.
- `primary_action.start_from_template_or_prebuild` — target_kind
  `template_or_prebuild_snapshot`; resulting_mode
  `open_prebuild_with_setup_actions` or `open_prebuild_minimal`.

Rules (frozen):

1. **Five distinct first-launch verbs.** `primary_action.open_folder`,
   `primary_action.open_workspace`, `primary_action.clone_repository`,
   `primary_action.restore_last_session`, and
   `primary_action.import_from` MUST render as distinct rows /
   buttons / commands on every Start Center state that is not a
   takeover (`unsupported_startup` may narrow to
   `open_folder` + `clone_repository` under
   `hide_all_except_open_and_clone`).
2. **No `Get started` collapse.** A surface that collapses two
   or more primary actions into a single `Get started` / `Start`
   / `Begin` button is non-conforming.
3. **Later reachability.** Every primary action MUST remain
   reachable later from the command palette
   (`route.command_palette`), the main menu (`route.global_menu`),
   `Open Recent` (on the `File` menu), and `Switch Project`
   (workspace-switcher entry). The `start_center_surface_record`
   reserves a `later_reach_routes[]` for these.

### 3.8 `restore_card_primary_action_id`

Closed set of the three distinct primary actions every restore
card exposes. `Undo` is forbidden on `compensating_rollback` and
`regenerate_from_canonical_source` recovery classes per entry-
restore object model §4.

- `restore_card.restore_now` — commit the advertised
  `restore_level` and `checkpoint_linked_recovery_class` from
  the referenced `restore_prompt_record`.
- `restore_card.skip_once` — dismiss the prompt for this launch;
  the `restore_prompt_record` is retained for evidence but not
  materialised. Routes to
  `startup_state:restore_skipped`.
- `restore_card.open_clean` — explicit
  `open_without_restore` decision. Routes to
  `startup_state:open_without_restore` and is recorded as an
  audit outcome, not inferred from absence.

Rules (frozen):

1. The three actions above are required on every non-takeover
   restore card. A card that renders a single generic `Reopen`
   or `Continue` CTA is non-conforming.
2. The restore card MUST name the `restore_level` verbatim
   (`Exact restore`, `Compatible restore`, `Layout only`,
   `Recovered drafts`, `Evidence only`) and the advertised
   `checkpoint_linked_recovery_class` verbatim. A card that
   shows a higher level than the underlying
   `restore_prompt_record` declares is non-conforming.
3. The card MUST separate **exact restore**,
   **dirty-buffer recovery**, **checkpoint rollback**, and
   **evidence-only recovery** in its summary counts. A card
   that flattens those four into a single "reopen N items"
   total is non-conforming.

### 3.9 `disclosure_class`

Closed set of disclosure-banner classes:

- `account_state_disclosure` — account present / absent,
  sign-in required, deferred review, unavailable in envelope.
- `service_state_disclosure` — sync, model gateway, marketplace,
  managed-cloud service state.
- `policy_state_disclosure` — admin policy envelope, workspace-
  trust envelope, fleet deployment bundle.
- `mirror_state_disclosure` — mirror / proxy / air-gapped
  posture.
- `offline_state_disclosure` — network reachability on routes
  that prefer remote resources.
- `extension_host_state_disclosure` — extension-host warming /
  quarantined / offline.
- `build_channel_identity` — build id, channel (stable / beta /
  nightly / preview / LTS), clean-room / signing provenance
  hint. Always rendered in `help_and_build_identity_footer`.
- `privacy_reduced_mode_notice` — required in `disclosure_band`
  whenever `privacy_reduction_mode` is non-`default`.

Rules (frozen):

1. Every non-`build_channel_identity` disclosure cites a
   `resolution_hook` from the upstream `next_step_decision_hook`
   set (e.g. `reauth_required`, `reconnect_required`,
   `continue_in_restricted_mode`, `set_up_later`).
2. `build_channel_identity` MUST be visible on every Start
   Center state (including takeovers) so the user always knows
   which build they are in.

## 4. Start-center surface record

Every first-launch, empty-profile, no-prior-session, and re-entered
Start Center view emits exactly one
`start_center_surface_record`. The shell renders zones, actions,
rows, banners, and the restore card by reading this record; no
zone mints its own state.

### 4.1 Required fields

- `record_kind = start_center_surface_record`.
- `surface_id` (opaque).
- `start_center_surface_family` (§3.1). `start_center` is the
  only family allowed to host the five first-launch primary
  actions as the initial landing; the three workspace-switcher
  families re-advertise them.
- `startup_state_ref` — one `startup_state` token from entry-
  restore truth audit §6. Names the truthful startup posture the
  surface is rendering against.
- `privacy_reduction_mode` (§3.5).
- `primary_actions[]` — ordered list of
  `start_center_primary_action_record`s. MUST include all five
  first-launch primary_action_ids on every `start_center` family
  record unless the state is `startup_state:unsupported_startup`
  with an explicit narrowing reason.
- `restore_card_ref` — optional reference to a
  `restore_card_record`. Required when `startup_state_ref` is
  `startup_state:reopen_with_pending_restore` or
  `startup_state:restore_failed`.
- `recent_work_row_refs[]` — ordered list of
  `recent_work_row_disclosure_record`s. May be empty on
  `startup_state:first_run` and on `privacy_reduction_mode =
  hide_recent_work` / `hide_all_except_open_and_clone` /
  `sensitive_environment_clear_on_launch`.
- `disclosure_banner_refs[]` — disclosure banners currently
  rendered, each tagged with `start_center_zone`.
- `later_reach_routes[]` — reserved route refs advertising how
  each primary action remains reachable later
  (`route.command_palette`, `route.global_menu`,
  `route.open_recent_submenu`, `route.switch_project`).
- `keyboard_reachability_posture` — `all_primary_focusable` is
  the only conforming value on non-takeover states; every
  primary action, every recent-work row's primary hook, and the
  three restore-card actions MUST be keyboard-reachable without
  a pointer. A surface that emits a different posture is
  non-conforming.
- `build_channel_identity_ref` — reference to a
  `disclosure_banner_record` whose `disclosure_class` is
  `build_channel_identity`. Always required.
- `minted_at` — monotonic timestamp.

### 4.2 Zone ordering invariants

1. `primary_work_resume` renders before any other zone except a
   blocking `disclosure_band` (§3.2 rule 4).
2. `recent_work_list` renders after `primary_work_resume` and
   before `secondary_entry`.
3. `secondary_entry` renders before
   `help_and_build_identity_footer`; marketplace-first and
   release-marketing content is allowed only in
   `help_and_build_identity_footer` (or outright forbidden
   depending on `disclosure_class`).
4. `help_and_build_identity_footer` renders last and is never
   suppressed (even in privacy-reduced modes).

## 5. Primary-action record

Every primary action — on Start Center and on workspace-switcher
surfaces — emits one `start_center_primary_action_record`.

### 5.1 Required fields

- `record_kind = start_center_primary_action_record`.
- `primary_action_id` (§3.7).
- `entry_verb`, `target_kind_candidate`,
  `resulting_mode_candidate` — re-exports from entry-restore
  object model §1. The action MUST resolve into exactly one of
  the candidate verbs when committed; a row that leaves all
  three null is non-conforming.
- `account_opt_in_posture` (§3.3).
- `next_step_decision_hooks[]` — at least one; drawn from the
  entry-restore §1.7 closed set.
- `side_effect_envelope` — required when
  `primary_action_id` is
  `primary_action.start_from_template_or_prebuild` or any
  `primary_action.clone_repository` that may attach a managed
  / container / devcontainer runtime on open.
- `disclosed_in_zone` — the `start_center_zone` the action
  renders in. MUST be `primary_work_resume` for the five
  first-launch actions.
- `keyboard_reachable = true` — required; a non-reachable
  primary action is non-conforming.
- `disabled_reason_code` — optional; drawn from the command-
  descriptor `disabled_reason_code` vocabulary when the action
  is visible-but-disabled (e.g. restore in envelopes without a
  pending prompt).

### 5.2 Rules

1. A primary-action record with `account_opt_in_posture =
   required_for_this_row` MUST name the remedy hook
   (`reauth_required`, `reconnect_required`, or
   `review_trust_and_open`) in `next_step_decision_hooks`.
2. A record whose `primary_action_id` is
   `primary_action.restore_last_session` MUST carry a
   `restore_card_ref`; a record without the companion card
   that promises `Restore` is non-conforming.
3. A record whose `primary_action_id` is
   `primary_action.import_from` MUST name the candidate target
   kinds explicitly (at least one of
   `portable_state_package`, `handoff_packet`,
   `competitor_config_root`, or
   `template_or_prebuild_snapshot`); a row that offers `Import`
   without naming the source class is non-conforming.

## 6. Recent-work row disclosure record

Every recent-work row on the Start Center, workspace-switcher,
and `Open Recent` submenu emits one
`recent_work_row_disclosure_record` that wraps the upstream
`recent_work_entry_record` (by `recent_work_id` ref) with typed
disclosure posture.

### 6.1 Required fields

- `record_kind = recent_work_row_disclosure_record`.
- `recent_work_entry_ref` — opaque ref to the upstream
  `recent_work_entry_record`.
- Re-exported fields rendered verbatim from the wrapped record
  (never contradicted): `presentation_label`,
  `presentation_subtitle`, `target_kind`, `target_state`,
  `portability_class`, `trust_state`, `restore_availability`,
  `pinned`, `last_opened_at`.
- `freshness_class` (§3.4) — every row discloses this.
- `absence_class` — required when `target_state` is any value
  other than `reachable` or `stale_metadata`.
- `account_opt_in_posture` (§3.3).
- `privacy_redaction_applied` — which fields the active
  `privacy_reduction_mode` redacted (closed subset:
  `subtitle_path`, `account_affordance`, `last_opened_at`,
  `none`).
- `row_actions[]` — typed `safe_recovery_action`s from the
  upstream §2.6 set. Every row MUST expose a subset covering at
  minimum `pin` / `unpin`, `remove_from_recents`, and one of
  `open` / `open_in_new_window` / `open_restricted` /
  `locate_missing_target` / `reconnect` / `reauth` /
  `compare_before_restore` / `open_without_restore` /
  `reveal_in_explorer` — consistent with the row's target
  state. `Open anyway` is non-conforming.
- `disclosed_in_zone = recent_work_list` — the only conforming
  zone for a recent-work row.

### 6.2 Disclosure rules

1. **No readiness overclaim.** A row whose `freshness_class` is
   not `live` MUST render the cache / offline / unknown chip;
   a row that renders as `reachable` when the upstream record
   is `stale_metadata`, `missing_target`, or
   `moved_target_detected` is non-conforming.
2. **No drifted `restore_availability`.** The wrapped record's
   `restore_availability` renders verbatim. A row that
   advertises `exact` when the record says `layout_only` is
   non-conforming (see §2.5 of the upstream record).
3. **Account opt-in disclosed in-place.** Provider-linked rows
   (`remote_repository`, `ssh_workspace`, managed-cloud) MUST
   disclose `account_opt_in_posture`; a row that hides
   `required_for_this_row` behind a generic `Open` click is
   non-conforming.
4. **Absence never silent.** A row with an `absence_class`
   MUST render at least one typed remedy action.
5. **Flat `Reopen` forbidden.** A recent-work row that promises
   `Reopen` without naming its underlying `restore_availability`
   or `target_state` is non-conforming.

## 7. Restore-card record

Every `restore_prompt_record` surfaced into the Start Center or a
standalone restore card emits exactly one
`restore_card_record`. The card is the shell's summarisation of
the upstream prompt; it quotes fields verbatim.

### 7.1 Required fields

- `record_kind = restore_card_record`.
- `restore_prompt_ref` — opaque ref to the upstream
  `restore_prompt_record`.
- `restore_level` — quoted verbatim from the prompt
  (`exact_restore`, `compatible_restore`, `layout_only`,
  `recovered_drafts`, `evidence_only`, `no_restore`).
- `recovery_class_refs[]` — the
  `checkpoint_linked_recovery_class` values from the prompt.
- `summary_counts` — the four separable counts that MUST NOT
  be flattened:
  - `exact_restore_item_count` — items the card will rehydrate
    at `exact_restore`.
  - `dirty_buffer_recovery_item_count` — items from the
    recovery journal (`restore_from_recovery_journal`).
  - `checkpoint_rollback_item_count` — items bound to a
    checkpoint rollback class
    (`exact_rollback_from_checkpoint`,
    `compensating_rollback`,
    `restore_from_session_checkpoint`,
    `restore_from_migration_checkpoint`,
    `restore_from_settings_backup`,
    `restore_from_local_history_snapshot`).
  - `evidence_only_item_count` — items retained as evidence
    only (`evidence_only` restore level or
    `manual_recovery_required` class).
- `session_execution_posture_by_pane[]` — each pane's
  `session_execution_posture` (one of
  `transcript_restored_not_rerun`, `session_ended`,
  `reconnect_available`, `rerun_required`, `context_unavailable`,
  `live_session_continued`). `live_session_continued` is
  allowed only when the runtime actually survived.
- `missing_target_states[]` — quoted verbatim from the prompt.
- `primary_actions[]` — the three typed restore-card primary
  actions from §3.8. MUST include all three on every non-
  takeover card.
- `disclosed_in_zone = primary_work_resume` — restore cards
  render with the primary-action row.
- `keyboard_reachable = true`.

### 7.2 Flattening rules

1. The four `summary_counts` fields are separable; a card that
   shows a single aggregate `N items to restore` without the
   per-class breakdown is non-conforming.
2. A card whose `restore_level` is `exact_restore` MUST NOT
   silently re-run mutating commands, notebook cells, debug
   attach, or remote mutations — the card reads the upstream
   prompt's rule (§3.1 of entry-restore object model) verbatim.
3. A card whose `summary_counts.evidence_only_item_count > 0`
   MUST render the evidence-only line separately; flattening
   evidence into the `Restore now` total is non-conforming.

## 8. Disclosure-banner record

Every account / service / policy / mirror / offline / extension-
host / privacy-reduction / build-channel banner emits one
`disclosure_banner_record`.

### 8.1 Required fields

- `record_kind = disclosure_banner_record`.
- `disclosure_class` (§3.9).
- `disclosed_in_zone` — bound by §3.2 zone rules.
- `account_opt_in_posture` — required for
  `account_state_disclosure`, `service_state_disclosure`,
  `policy_state_disclosure`.
- `freshness_class` — required for
  `service_state_disclosure`, `mirror_state_disclosure`,
  `offline_state_disclosure`.
- `resolution_hooks[]` — at least one typed
  `next_step_decision_hook` for every non-
  `build_channel_identity` banner.
- `build_channel_id` — required for
  `build_channel_identity` (opaque id; resolves to the running
  build identity in the release-artifact graph).

### 8.2 Placement rules

1. `account_state_disclosure` with
   `optional_local_path_available` — forbidden in
   `disclosure_band`; MAY render alongside a specific row or in
   `profile_selection`. Never above `primary_work_resume`.
2. `policy_state_disclosure` with `required_for_this_row` or
   `unavailable_in_this_envelope` — allowed above
   `primary_work_resume` as a blocking disclosure.
3. `privacy_reduced_mode_notice` — allowed (required) above
   `primary_work_resume` whenever a non-`default`
   `privacy_reduction_mode` is active.
4. `build_channel_identity` — always in
   `help_and_build_identity_footer`; never above
   `primary_work_resume`.

## 9. Workspace-switcher view record

Every workspace-switcher surface emits one
`workspace_switcher_view_record` so the record chain for "how to
switch projects later" is first-class, not assumed.

### 9.1 Required fields

- `record_kind = workspace_switcher_view_record`.
- `start_center_surface_family` — one of
  `workspace_switcher_palette`, `workspace_switcher_menu`,
  `workspace_switcher_dedicated_view`.
- `host_route_ref` — `navigation_route_id` the switcher renders
  on (`route.command_palette`, `route.global_menu`,
  `route.sidebar`, `route.dedicated_view`).
- `primary_actions[]` — MUST re-advertise the five first-launch
  primary actions (plus `primary_action.add_root` for in-
  workspace widening).
- `recent_work_row_refs[]` — re-reads the same
  `recent_work_row_disclosure_record`s the Start Center reads.
  The switcher MUST NOT render a diverged row shape.
- `later_reach_routes[]` — reserved so the switcher is
  traceable back from support exports.

### 9.2 Rules

1. **Parity with Start Center.** The switcher renders the same
   verbs, same row disclosure posture, and same zone ordering
   as the Start Center. A switcher that drops a verb the Start
   Center offers is non-conforming.
2. **Keyboard-only reachable.** Every action on the switcher
   MUST be reachable from `route.command_palette` and from
   `route.global_menu` without a pointer. Palette-only or
   mouse-only switchers are non-conforming.

## 10. Surface rules (cross-cutting)

1. **Primary work-resume first.** On every seeded startup
   state, the five first-launch primary actions (and the
   restore card, when present) render in `primary_work_resume`
   before account nags, release-marketing, or marketplace-first
   content appear anywhere on the surface.
2. **Account-free path preserved.** Every Start Center state
   that is not an `unavailable_in_this_envelope` takeover MUST
   expose at least one primary action with
   `account_opt_in_posture = optional_local_path_available`.
3. **Distinct verbs.** `Open`, `Clone`, `Import`, `Restore`,
   `Recent work`, and `Add root` render as separate rows /
   buttons / commands. A `Get started` collapse is non-
   conforming (see entry-restore object model §7.2).
4. **Restore-class separation.** Exact restore, dirty-buffer
   recovery, checkpoint rollback, and evidence-only recovery
   never flatten into one `Reopen` CTA. The restore card cites
   the four `summary_counts` separately and the three typed
   primary actions (`Restore now`, `Skip once`, `Open clean`).
5. **Freshness and absence disclosed.** Every recent-work row,
   cached onboarding / help pack, template / prebuild, and
   not-installed docs row discloses `freshness_class` and, when
   absent, `absence_class`. Hiding cache age is non-conforming.
6. **Later reachability.** Every primary action and every
   recent-work row remains reachable later via the command
   palette, main menu, `Open Recent`, and `Switch Project`.
7. **Build / channel identity visible.** Every Start Center
   state (including takeovers) renders the
   `build_channel_identity` disclosure banner in
   `help_and_build_identity_footer`.
8. **Keyboard reachability.** Every primary-action row, every
   recent-work row's primary hook, and the three restore-card
   actions are keyboard-reachable. A surface that requires a
   pointer for any primary affordance is non-conforming.
9. **Privacy reduction visible.** A non-`default`
   `privacy_reduction_mode` MUST render a
   `privacy_reduced_mode_notice` banner; silent hiding is non-
   conforming.
10. **No redefinition upstream.** Recent-work rows, primary
    actions, restore cards, and switcher views never mint
    entry-verb, target-kind, resulting-mode, restore-level,
    missing-target-state, recovery-class, or
    next-step-decision-hook values. They always resolve through
    the entry-restore object model.

## 11. Seeded-state matrix (how each startup state behaves)

The matrix below binds each `startup_state` token (truth-audit
§6) to the disclosure posture a conforming Start Center surface
renders.

| `startup_state` | Primary actions rendered | Recent-work zone | Restore card | Account posture default | Required blocking disclosures |
|---|---|---|---|---|---|
| `first_run` | All five first-launch verbs in `primary_work_resume`. | Empty; may render `start_from_template_or_prebuild` in `secondary_entry`. | None. | `optional_local_path_available` on Open / Clone / Import. | None. |
| `reopen_with_pending_restore` | All five; `restore_last_session` paired with restore card. | Populated with disclosure posture; `freshness_class` varies per row. | Required; three actions. | Varies per row. | None (restore is in `primary_work_resume`). |
| `restore_failed` | All five; `restore_last_session` disabled with `disabled_reason_code = restore_failed_previously`. | Populated. | Required; `summary_counts.evidence_only_item_count > 0`. | Varies. | `recovery_ladder_packet` link in `help_and_build_identity_footer`. |
| `restore_skipped` | All five; prior `Skip once` recorded in audit. | Populated. | None (prompt retained for evidence). | Varies. | None. |
| `open_without_restore` | All five. | Populated. | None. | Varies. | None; the choice is recorded in audit, not re-prompted. |
| `warming_startup` | All five; recent-work chips read `stale_metadata` where applicable. | Populated. | None unless a restore prompt is pending. | Varies. | Optional extension-host-state disclosure. |
| `partial_startup` | All five; impacted rows render typed remedy hooks. | Populated. | None unless pending. | Varies. | Extension-host-state disclosure; optional mirror-state disclosure. |
| `offline_startup` | All five; local-only rows reachable, remote rows render `stale_offline` / `unknown_since`. | Populated. | None unless pending. | `required_for_this_row` on remote rows; local rows remain `optional_local_path_available`. | Offline-state disclosure in `disclosure_band` above `primary_work_resume` (blocking on remote-preferred routes). |
| `unsupported_startup` | May narrow to `primary_action.open_folder` + `primary_action.clone_repository` under `hide_all_except_open_and_clone`; MUST keep at least these two. | May be hidden. | None. | `unavailable_in_this_envelope` on affected rows. | Policy-state disclosure blocking above `primary_work_resume`. |
| `empty_state_or_placeholder_transition` | Five; every row truthful about placeholder posture. | Empty or transitional. | None. | `optional_local_path_available` minimum. | None (matches truth-audit §6.10). |

## 12. Worked examples

Each example has a companion fixture under
[`/fixtures/ux/start_center_rows/`](../../fixtures/ux/start_center_rows/).

### 12.1 First-run, no-account, local-first

Fresh install on a previously-unused device. Start Center renders
all five first-launch primary actions in `primary_work_resume`;
every action carries
`account_opt_in_posture = optional_local_path_available`; no
account nag, no release-marketing card, no marketplace-first
content. The `build_channel_identity` banner renders in
`help_and_build_identity_footer`. See
[`start_center_first_run_no_account.json`](../../fixtures/ux/start_center_rows/start_center_first_run_no_account.json).

### 12.2 Offline managed-cloud reopen

The user reopens on a device whose network is down for the
managed-cloud workspace. Local-only recent-work rows remain
`reachable` with `freshness_class = live`; managed-cloud rows
render `stale_offline` and carry
`account_opt_in_posture = required_for_this_row` with remedy
hooks `reconnect_required` / `reauth_required`. A blocking
`offline_state_disclosure` renders in `disclosure_band` above
`primary_work_resume`. See
[`start_center_offline_managed.json`](../../fixtures/ux/start_center_rows/start_center_offline_managed.json).

### 12.3 Recent-work row, missing target

A recent-work row whose folder moved on disk. The row's
`freshness_class = unknown_since`,
`absence_class = unreachable`, typed row actions
`locate_missing_target`, `remove_from_recents`,
`reveal_in_explorer`. The row never claims `reachable`; the chip
reads `unknown`. See
[`start_center_recent_work_missing_target.json`](../../fixtures/ux/start_center_rows/start_center_recent_work_missing_target.json).

### 12.4 Privacy-reduced Start Center on a shared device

`privacy_reduction_mode = hide_paths`; every recent-work row's
`presentation_subtitle` is redacted to its target-kind class.
`hide_account_affordances` hides the sign-in affordances;
`Open folder` and `Clone repository` remain reachable. A
`privacy_reduced_mode_notice` banner renders above
`primary_work_resume`; `Clear recent work` and
`Exit privacy-reduced mode` controls render in
`help_and_build_identity_footer`. See
[`start_center_recent_work_privacy_reduced.json`](../../fixtures/ux/start_center_rows/start_center_recent_work_privacy_reduced.json).

### 12.5 Restore card, compatible restore

A standard reopen after a minor extension-version bump. The
restore card names
`restore_level = compatible_restore`,
`summary_counts.exact_restore_item_count = 3`,
`dirty_buffer_recovery_item_count = 2`,
`checkpoint_rollback_item_count = 0`,
`evidence_only_item_count = 0`; session-execution posture per
pane is `transcript_restored_not_rerun` for terminal panes and
`reconnect_available` for one debug pane. The three primary
actions `Restore now`, `Skip once`, `Open clean` render
distinctly. See
[`start_center_restore_card_compatible.json`](../../fixtures/ux/start_center_rows/start_center_restore_card_compatible.json).

### 12.6 Restore card, evidence-only recovery

A reopen after a corrupt-restorable-state failure. The card
names `restore_level = evidence_only`,
`summary_counts.evidence_only_item_count = 5`,
`exact_restore_item_count = 0`,
`dirty_buffer_recovery_item_count = 1` (journal available),
`checkpoint_rollback_item_count = 0`; missing-target states
include `corrupt_restorable_state`. The card does not flatten
the one recoverable dirty buffer into a single "reopen 6"
total; it renders the two counts separately. The primary
actions remain three distinct verbs. See
[`start_center_restore_card_evidence_only.json`](../../fixtures/ux/start_center_rows/start_center_restore_card_evidence_only.json).

### 12.7 Workspace-switcher palette view

Mid-session switcher opened via the command palette. The
switcher re-advertises all five first-launch primary actions
plus `primary_action.add_root`; recent-work rows render with
the same disclosure posture the Start Center used at launch.
The view names its `host_route_ref = route.command_palette`
and reserves `later_reach_routes[] =
[route.global_menu, route.open_recent_submenu,
route.switch_project]`. See
[`workspace_switcher_palette_view.json`](../../fixtures/ux/start_center_rows/workspace_switcher_palette_view.json).

### 12.8 Unsupported-envelope narrowing

A managed-fleet machine whose policy envelope forbids import,
clone, and restore paths. The Start Center enters
`privacy_reduction_mode = hide_all_except_open_and_clone`;
primary actions narrow to `primary_action.open_folder` and
`primary_action.clone_repository`; a blocking
`policy_state_disclosure` renders above `primary_work_resume`
with resolution hook `continue_in_restricted_mode`. See
[`start_center_unsupported_envelope.json`](../../fixtures/ux/start_center_rows/start_center_unsupported_envelope.json).

## 13. Acceptance mapping

- **Distinct semantics.** The five first-launch primary actions
  (`Open folder`, `Open workspace`, `Clone repository`,
  `Restore last session`, `Import from…`) render distinctly via
  the `primary_action_id` closed set (§3.7) and the
  `start_center_primary_action_record` shape (§5).
- **Account / service / policy posture disclosed per row.**
  Every row carries `account_opt_in_posture` (§3.3) from a
  closed set (`optional_local_path_available`,
  `required_for_this_row`, `deferred_review_pending`,
  `unavailable_in_this_envelope`).
- **Offline / managed / account-free / missing-target / privacy-
  reduced behaviour readable from the contract.** The seeded
  state matrix (§11) pairs each `startup_state` token with
  required primary actions, recent-work posture, restore-card
  presence, account-posture default, and blocking
  disclosures.
- **Restore classes never flattened.** `summary_counts` (§7.1)
  separates exact restore, dirty-buffer recovery, checkpoint
  rollback, and evidence-only recovery, and the three typed
  restore-card primary actions (§3.8) replace any generic
  `Reopen` CTA.
- **Later reachability is a record, not a promise.**
  `later_reach_routes[]` on the Start Center surface (§4.1)
  and the workspace-switcher view (§9.1) freezes the palette,
  main menu, `Open Recent`, and `Switch Project` as advertised
  routes, so the same actions remain reachable after the
  wedge closes.
- **Schema coverage.** The schema at
  [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json)
  validates the eight worked-example fixtures under
  [`/fixtures/ux/start_center_rows/`](../../fixtures/ux/start_center_rows/).

## 14. Changing this contract

- **Additive-minor** changes (new `start_center_surface_family`,
  new `start_center_zone`, new `account_opt_in_posture`, new
  `freshness_class` / `absence_class`, new
  `privacy_reduction_mode`, new `disclosure_class`, new
  `primary_action_id`) land here and in the companion schema
  plus at least one fixture in the same change. Every new value
  MUST cite the motivating startup state or fixture.
- **Repurposing** an existing zone, posture, or action id is
  breaking and opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (entry-verb, target-kind,
  resulting-mode, restore-level, missing-target-state,
  recovery-class, next-step-decision-hook) happen in the
  entry-restore object model; this contract re-exports by
  reference and MUST NOT shadow the change.
- The PRD / TAD / TDD / UI-UX spec wins on any disagreement
  with the quotations in §15; this contract and the schema
  update in the same change.

## 15. Source anchors

- `.t2/docs/Aureline_PRD.md:284` — first-run onboarding is a
  launch risk, not polish.
- `.t2/docs/Aureline_PRD.md:1300` — crash recovery MUST degrade
  gracefully from exact session restore to dirty-buffer
  recovery to open-clean with preserved evidence, never
  directly to silent loss.
- `.t2/docs/Aureline_PRD.md:1703` — recovery ladder: safe mode
  → extension quarantine → cache reset → restricted mode.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:762` — Start Center
  primary actions (`Open folder`, `Open workspace`,
  `Clone repository`, `Restore last session`, `Import from…`).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:784` — recent-work
  row fields (workspace / project name, path, kind icon,
  last-opened timestamp, trust state, restore availability,
  pin / remove / locate / reconnect actions).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:836` — restore-class
  taxonomy (Exact session restore, Context restore with
  placeholders, Dirty-buffer recovery, Checkpoint rollback,
  Evidence-only recovery).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:866` — restore-
  fidelity controlled terms.
- `.t2/docs/Aureline_Milestones_Document.md:1023` — Start
  Center keeps `Open`, `Clone`, `Import`, `Restore`, and
  `Recent work` distinct with a no-account local path.
- `.t2/docs/Aureline_Milestones_Document.md:1544` — entry
  verbs stay distinct across Start Center, command surfaces,
  OS file association, drag-and-drop, and CLI / headless
  entry.

## 16. Linked artifacts

- Entry / restore object model (source of truth for entry
  verbs, target kinds, restore levels, recovery classes, and
  next-step decision hooks this contract re-exports):
  [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md).
- Entry / restore placeholder truth audit (source of truth
  for `startup_state` tokens this contract's seeded-state
  matrix resolves against):
  [`/docs/ux/entry_restore_truth_audit.md`](./entry_restore_truth_audit.md).
- Navigation hierarchy and escalation contract (source of
  truth for `navigation_route_id` and `escalation_tier`):
  [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md).
- Embedded-surface boundary contract (source of truth for
  embedded docs / help / marketplace panes referenced from the
  footer and disclosure bands):
  [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md).
- Onboarding / first-useful-work / migration measurement plan
  (source of truth for `entry_route_id` every primary-action
  record tags):
  [`/docs/product/onboarding_measurement_plan.md`](../product/onboarding_measurement_plan.md).
- Recovery-ladder packet (source of truth for `safe_mode` and
  recovery-ladder rung ids that block-restore and
  unsupported-envelope disclosures cite):
  [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md).
- Start-center surface schema:
  [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json).
- Worked-example fixtures:
  [`/fixtures/ux/start_center_rows/`](../../fixtures/ux/start_center_rows/).
