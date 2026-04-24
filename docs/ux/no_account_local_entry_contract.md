# First-run no-account local-work path, service-opt-in boundary, and onboarding portability contract

This document freezes the cross-surface contract that Aureline's
**first-run no-account local-work path**, **service-opt-in
boundary**, and **onboarding portability state** resolve through
before the Start Center, workspace-switcher, first-run tour, and
migration-center surfaces are implemented.

The local-first promise is a launch-bearing product claim: useful
work on a previously-unused device MUST be reachable without
account creation, and Start Center, recent-work, import, restore,
and switcher surfaces MUST NOT re-route first-run through a sign-in
wall or a blocking account-nag card. This contract makes that
promise mechanical rather than narrative by binding every entry-
surface row, account-prompt decision, and onboarding state item to
one record carrying one closed account-prompt class, one boundary-
crossing class, one portability class, one reset class, and one
export class so hidden service dependencies at first run become
schema or policy violations rather than drift that shows up after
launch.

The machine-readable schema lives at:

- [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/entry_surface_rows/`](../../fixtures/ux/entry_surface_rows/)

This contract is normative for the disclosure posture of entry-
surface rows and the portability shape of onboarding state. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, or milestone document,
those sources win and this document plus its companion schema and
fixtures update in the same change. Where a downstream Start Center,
switcher, tour, or onboarding surface mints a parallel vocabulary,
this contract wins and the surface is non-conforming.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference:

- [`/docs/ux/start_center_contract.md`](./start_center_contract.md)
  and
  [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json)
  — Start Center / workspace-switcher surface families, zone
  vocabulary, primary-action ids, recent-work row disclosure,
  restore-card shape, disclosure-banner classes, `account_opt_in_posture`,
  `privacy_reduction_mode`, and `freshness_class` / `absence_class`.
- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
  — `entry_verb`, `target_kind`, `resulting_mode`,
  `admission_class`, `next_step_decision_hook`,
  `safe_recovery_action`, `portability_class`,
  `restore_availability`, `trust_state`, `side_effect_envelope`.
- [`/docs/ux/entry_restore_truth_audit.md`](./entry_restore_truth_audit.md)
  — `startup_state` token set.
- [`/docs/product/onboarding_measurement_plan.md`](../product/onboarding_measurement_plan.md)
  — entry-route ids, readiness buckets, measurement-surface
  vocabulary, and the qualification-event names scoreboard rows
  cite.
- [`/artifacts/product/no_account_switching_scoreboard_seed.yaml`](../../artifacts/product/no_account_switching_scoreboard_seed.yaml)
  — scoreboard row ids, threshold states, and boundary-manifest
  row citations every entry-surface row maps back to.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary
  (`individual_local`, `self_hosted`, `enterprise_online`,
  `air_gapped`, `managed_cloud`, `air_gapped_mirror_only`) every
  row resolves against.
- [`/schemas/identity/policy_bundle.schema.json`](../../schemas/identity/policy_bundle.schema.json)
  and
  [`/schemas/identity/entitlement_snapshot.schema.json`](../../schemas/identity/entitlement_snapshot.schema.json)
  — `active_policy_bundle_ref` and entitlement / grace state
  vocabulary policy-forced rows cite.

## Who reads this contract

- **First-run / Start Center / switcher / open-flow / import /
  restore / tour authors** wiring any surface that may render a
  sign-in card, consent request, tour step, or dismissal. Every
  decision to show, defer, or forbid an account prompt resolves
  through a record shape defined here.
- **Onboarding-state owners** choosing where tour progress, one-
  shot dismissals, imported-profile history, consent acknowledgement
  logs, recent-work metadata, and restore-prompt skip-registries
  live — portable profile state (syncs with the user), machine-local
  diagnostics (stays on this device), or ephemeral per-session state.
- **Policy and admin-envelope authors** naming which prompts are
  forced, deferrable, or unavailable for an envelope (managed-fleet,
  air-gapped, kiosk).
- **Docs, support, and measurement authors** linking every
  first-run, first-open, opt-in-boundary, and migration-review row
  to the same record kinds the shell renders and the scoreboard
  qualifies.

## 1. Scope

- Freeze one `entry_surface_row_record` that every Start Center,
  workspace-switcher, open-flow-sheet, recent-work-list, import-
  picker, restore-card, first-run-tour step, welcome banner,
  onboarding prompt, and account-opt-in card emits. Every row
  names exactly one `entry_surface_family`, one `primary_action_id`
  or `ancillary_action_id`, one `account_prompt_class`, one
  `account_prompt_timing_class`, one `boundary_crossing_class`,
  and one `scoreboard_row_ref` back to the no-account-switching
  scoreboard.
- Freeze one `account_prompt_record` describing when an account or
  service opt-in is **required**, **optional**, **deferrable**,
  **policy-forced**, or **unavailable** — with offline, managed,
  and sensitive-environment behaviour closed under one
  `account_prompt_timing_class`.
- Freeze one `onboarding_portability_state_record` that binds every
  named onboarding state item (tour progress, dismissals, imported-
  profile history, onboarding posture, recent-work metadata,
  restore-skipped registry, first-run-seen flags, consent
  acknowledgement log, import-outcome ledger, machine-local
  diagnostics ring) to exactly one `state_portability_class`, one
  `reset_class`, and one `export_class`, with a `portability_reason`
  free-text anchor a reviewer can read.
- Freeze one `onboarding_portability_manifest_record` aggregating
  the state items and declaring the invariants that make hidden
  service dependencies at first run, privacy-reduction bypasses of
  the Open / Clone floor, or silent portable-state leakage across
  accounts mechanical failures rather than UX drift.
- Name the closed vocabulary the contract introduces and rules the
  shell MUST obey when deciding which rows render, which prompts
  fire, and which state portability path a new onboarding item
  joins.

## 2. Out of scope

- Final visuals (exact icons, chip widths, card padding, tour
  animation). The style guide and shell-zone contract own those.
- The actual Start Center, switcher palette, open-flow sheet,
  first-run tour, or migration-center implementation. Those are
  later milestones; the disclosure, portability, and record shapes
  freeze here first.
- Per-OS system-menu, dock, jump-list, or share-sheet routing
  for entry-surface rows. The platform-adapter contract owns that.
- Final user-facing copy / microcopy. The shell-interaction-safety
  contract and the UX style guide own the exact strings; this
  contract pins the closed sets the copy resolves against.
- Wire format or transport for portable-profile synchronization.
  The state-and-recovery taxonomy and portable-profile-package
  contract own that; this contract only names which state items
  belong in that lane versus the machine-local lane.
- Implementation of every onboarding flow. The launch milestone
  names the surfaces in scope for M0; this contract freezes the
  boundary so later surfaces do not have to reconcile competing
  field names across entry flows, sign-in walls, tour progress,
  and privacy-reduction posture.

## 3. Frozen vocabulary

This contract re-exports — by reference — the vocabularies listed
in §companion-contracts and mints nothing new in those sets.

It introduces six small closed vocabularies that are scoped to
entry surfaces, account-prompt decisions, and onboarding state.

### 3.1 `entry_surface_family`

Which entry surface is being described. The set is closed:

- `start_center` — first-launch / no-prior-session Start Center.
- `workspace_switcher_palette` — command-palette-hosted switcher.
- `workspace_switcher_menu` — `Switch Project` / `Open Recent`
  submenu.
- `workspace_switcher_dedicated_view` — full-surface switcher
  view.
- `open_flow_sheet` — `Open` / `Open Workspace` file-picker sheet.
- `recent_work_list` — any surface rendering recent-work rows
  (Start Center, switcher, palette recents, CLI recents,
  `Open Recent` submenu).
- `restore_card` — standalone restore card a `restore_prompt_record`
  renders into.
- `import_picker` — picker that selects a `portable_state_package`,
  `handoff_packet`, `competitor_config_root`, or
  `template_or_prebuild_snapshot` for import / review.
- `first_run_tour_step` — one step of the first-run tour.
- `welcome_banner` — welcome / greeting banner on first-launch /
  reopen surfaces that is not a blocking disclosure.
- `account_opt_in_card` — a card that asks the user to sign in,
  connect a provider, enable sync, enable telemetry, enable
  managed-cloud, enable browser-assisted handoff, or approve a
  connected-provider request.
- `onboarding_prompt` — a prompt inviting the user to set up a
  workflow bundle, adopt a recommended archetype, enable a tour,
  or complete a deferred review.

Rules (frozen):

1. A surface that mints a thirteenth family is non-conforming;
   the twelve families above are the full set.
2. Every `account_opt_in_card` MUST emit one `entry_surface_row_record`
   carrying an `account_prompt_record` reference; a card that
   renders without a prompt record is non-conforming.
3. `welcome_banner` and `first_run_tour_step` MUST NOT replace or
   hide any of the five first-launch primary actions (see Start
   Center contract §3.7); a banner or tour step that occludes
   Open / Open Workspace / Clone / Restore / Import is non-
   conforming.

### 3.2 `account_prompt_class`

For every row or card that may ask the user to create an account,
sign in, connect a provider, approve a connected provider, enable
sync, enable telemetry, attach a managed-cloud workspace, or
approve a browser-assisted handoff. The set is closed:

- `no_prompt` — the row is reachable without any account, service,
  provider, sync, telemetry, or managed-cloud opt-in. The row's
  primary action commits local work. First-run and every purely
  local recent-work row MUST resolve here.
- `optional_prompt` — an opt-in is offered as a **side**
  affordance; declining it leaves the row's primary action
  working. Telemetry, BYOK-AI, sync, and marketplace-browse rows
  on `individual_local` typically resolve here.
- `deferrable_prompt` — the user may defer the prompt (set up
  later, remind me, review on next open). Deferral MUST NOT widen
  admission silently; the next render reads the same row state.
- `required_prompt` — the row's primary action cannot proceed
  without the opt-in (clone of a private repo needing OAuth
  handle; resume of a managed-cloud workspace with expired
  authority). MUST name at least one remedy hook
  (`reauth_required`, `reconnect_required`,
  `review_trust_and_open`) verbatim.
- `policy_forced_prompt` — the active policy bundle forces the
  opt-in for this envelope (managed-fleet that requires SSO for
  all work, compliance envelope that requires telemetry
  acknowledgement before any workspace opens). The prompt cannot
  be declined in-envelope; the row MUST cite the active
  `active_policy_bundle_ref` and name the
  `continue_in_restricted_mode` or
  `set_up_later` hook only when the policy allows it.
- `unavailable_prompt` — the opt-in cannot be attempted in this
  envelope (air-gapped, policy-forbidden service). The row MUST
  NOT present an opt-in affordance that will silently fail; it
  names the envelope class and renders the absence-narrows-to
  consequence.
- `not_applicable` — the row does not expose an account or
  service boundary at all (e.g. `Clear recent work` control,
  `build_channel_identity` banner). Used only when the row's
  class genuinely has no account-plane interaction.

Rules (frozen):

1. **No-account floor.** Every seeded `entry_surface_family`
   state that is not an `unavailable_prompt` takeover MUST expose
   at least one primary action whose `account_prompt_class` is
   `no_prompt`. The floor is `primary_action.open_folder`
   minimum; a surface that loses that floor is non-conforming.
2. **No forced-first-run sign-in.** On `startup_state:first_run`,
   no `primary_action_id` row renders with
   `account_prompt_class = required_prompt` unless the active
   envelope is a managed or policy-forced takeover; the row then
   carries `unavailable_prompt` or `policy_forced_prompt`
   **explicitly** and a local path (Open folder) remains
   reachable.
3. **Optional and deferrable prompts ride side affordances.**
   `optional_prompt` and `deferrable_prompt` cards MAY render in
   `profile_selection`, `secondary_entry`, or as row-local
   affordances, but MUST NOT render above `primary_work_resume`
   (per Start Center contract §3.2).
4. **Policy-forced prompts cite the bundle.** Every
   `policy_forced_prompt` row MUST cite `active_policy_bundle_ref`
   and `policy_forced_reason_code` drawn from the closed set
   `managed_fleet_sso`, `compliance_telemetry_required`,
   `managed_cloud_authority_required`,
   `admin_approval_required_for_extension_host`,
   `consent_acknowledgement_required`.
5. **`unavailable_prompt` carries absence narrowing.** The row
   MUST cite the boundary-manifest row's `absence_narrows_to`
   language so the user sees the consequence inline.
6. **Hidden dependency becomes a violation.** A surface that
   resolves an action labelled `no_prompt` or `optional_prompt`
   but actually triggers an account, service, provider-linked,
   managed-cloud, telemetry, or sync side effect is non-
   conforming. The invariant is enforceable from the
   `boundary_crossing_class` (§3.4) declaration.

### 3.3 `account_prompt_timing_class`

When the prompt is rendered relative to the user's entry. Closed
set:

- `never_shown` — the row does not render a prompt in any state.
- `shown_at_first_run_declinable` — fires once on
  `startup_state:first_run`, is declinable, and is not re-asked
  on subsequent launches unless the user resets onboarding
  state.
- `shown_at_entry_declinable` — fires at the row's entry point
  (clicking the affected action) and is declinable each time.
- `shown_only_on_boundary_crossing` — fires only when the user
  crosses from the account-free lane into a service-plane
  capability (first clone of a private repo, first managed-cloud
  workspace, first browser-assisted handoff).
- `shown_deferred_for_review` — fires after a deferred-review
  flow completes (connected-provider approval, post-import
  validation, admin policy review).
- `forced_by_policy` — fires because the active policy bundle
  forces it; cannot be declined in-envelope.
- `unavailable_in_envelope` — the prompt cannot fire; the row
  renders the envelope-class consequence instead.

Rules (frozen):

1. A row whose `account_prompt_class` is `no_prompt` MUST carry
   `account_prompt_timing_class = never_shown`.
2. A row whose `account_prompt_class` is `optional_prompt` MUST
   carry one of `shown_at_first_run_declinable`,
   `shown_at_entry_declinable`, or `never_shown`.
3. A row whose `account_prompt_class` is `required_prompt` MUST
   carry one of `shown_at_entry_declinable`,
   `shown_only_on_boundary_crossing`, or
   `shown_deferred_for_review`.
4. A row whose `account_prompt_class` is `policy_forced_prompt`
   MUST carry `forced_by_policy`.
5. A row whose `account_prompt_class` is `unavailable_prompt`
   MUST carry `unavailable_in_envelope`.

### 3.4 `boundary_crossing_class`

What the row's primary action actually touches. Closed set; this
is the ground truth the `account_prompt_class` label is checked
against:

- `no_boundary_crossed` — the primary action stays inside the
  account-free local-work lane: reads / writes only local files,
  local profile, local caches; no service-plane, no remote, no
  credential store, no managed-cloud authority, no telemetry
  emit, no provider-linked state.
- `reads_provider_state` — the action reads provider-linked
  state (clone visibility check, connected-provider metadata)
  without writing outside the user's local scope.
- `widens_trust` — the action changes the workspace or profile
  trust state (promote to trusted, demote to restricted).
- `touches_credential_store` — the action reads / writes the
  OS credential store or secret-broker-backed credential handle.
- `reaches_remote_resource` — the action reaches a remote
  repository, managed workspace, or mirror.
- `attaches_external_runtime` — the action attaches a container,
  devcontainer, remote SSH, or managed-cloud runtime.
- `emits_managed_telemetry` — the action emits a managed-
  telemetry event (not just local debug logs).
- `opens_browser_handoff` — the action opens a browser for OAuth
  or connected-provider handoff.
- `enters_service_plane` — catch-all for mixed service-plane
  entry that is not exactly one of the above; the row MUST still
  name the specific subclasses on `boundary_crossings_detail[]`.

Rules (frozen):

1. A row whose `account_prompt_class` is `no_prompt` MUST carry
   `boundary_crossing_class = no_boundary_crossed`. A row that
   claims `no_prompt` but declares any other boundary-crossing
   class is a schema violation.
2. A row whose `boundary_crossing_class` is not
   `no_boundary_crossed` MUST carry an `account_prompt_class` in
   `{optional_prompt, deferrable_prompt, required_prompt,
   policy_forced_prompt, unavailable_prompt}`. A
   `reads_provider_state` or `reaches_remote_resource` row
   labelled `no_prompt` is non-conforming.
3. The row MUST cite exactly one `scoreboard_row_ref` into the
   no-account-switching scoreboard. A row without a scoreboard
   citation cannot claim `no_prompt`, `optional_prompt`, or
   `policy_forced_prompt`; it is non-conforming.

### 3.5 `state_portability_class`

Where a named onboarding state item lives and how it moves with
the user. Closed set:

- `portable_profile_state` — the item belongs in the portable
  profile package. It moves with the user across devices and
  across clean installs when the user carries the profile.
  Example: tour progress, first-run-seen flags, dismissals the
  user intended as "I've acknowledged this".
- `machine_local_diagnostic` — the item stays on the device and
  is never exported as portable profile state. Machine-local
  logs, decode-recovery journal entries, and per-device
  diagnostic rings live here.
- `device_scoped` — the item is local to the device but part of
  the normal user state (not diagnostic). Recent-work metadata
  and the restore-prompt skipped registry live here.
- `account_scoped` — the item is scoped to a signed-in account
  (connected-provider approval tickets, managed-cloud
  entitlements). Never rides the portable profile package for a
  different account.
- `policy_scoped` — the item is owned by the active policy
  bundle (consent acknowledgement log, managed-fleet overrides).
  Never rides the portable profile package; the policy bundle
  carries it.
- `ephemeral_session` — the item lives only for the current
  session (in-memory restore-card dismissal, current tour step
  pointer). Never persisted.

Rules (frozen):

1. **One item, one class.** An onboarding state item MUST declare
   exactly one `state_portability_class`. Dual-homing (e.g. "rides
   both the profile package and the machine ring") is non-
   conforming; it masks where the item really moves.
2. **Account-scoped never leaks across accounts.** An
   `account_scoped` item MUST NOT ride the portable profile
   package when the user carries the profile to a different
   account.
3. **Portable does not mean unauditable.** Every
   `portable_profile_state` item MUST declare an `export_class`
   that the portable profile package contract renders against.
4. **No silent hidden portable state.** Every onboarding state
   item defined by a downstream feature MUST land one
   `onboarding_portability_state_record` in
   [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)-backed
   storage; an item that is not declared here cannot ride the
   portable profile package. The manifest invariant (§6) enforces
   this.

### 3.6 `reset_class` and `export_class`

Orthogonal axes every onboarding state item MUST declare.

`reset_class` (closed):

- `resettable_per_profile` — the user can reset this per-profile
  via `Reset onboarding state` in Settings.
- `resettable_per_device` — the user can reset this per-device
  (wipe machine-local tour state, decode-recovery ring).
- `resettable_per_account` — the user can reset this per-account
  (revoke connected-provider tickets).
- `resettable_by_policy` — only the active policy bundle can
  reset it (consent acknowledgement log).
- `not_resettable_locally` — the user cannot reset the item from
  the shell (audit-only ledger).

`export_class` (closed):

- `in_portable_profile_package` — included in the portable
  profile export as-is.
- `in_portable_profile_package_redacted` — included but with
  path / identifier redaction.
- `in_support_bundle_redacted` — not in the profile package,
  included (redacted) in support bundles.
- `in_handoff_packet` — included in explicit handoff packets
  only.
- `not_exported_machine_local` — never exported; machine-local
  only.
- `blocked_by_policy` — export blocked by the active policy
  bundle.

Rules (frozen):

1. A `portable_profile_state` item MUST have
   `export_class in {in_portable_profile_package,
   in_portable_profile_package_redacted}`.
2. A `machine_local_diagnostic` item MUST have
   `export_class in {not_exported_machine_local,
   in_support_bundle_redacted}`.
3. A `policy_scoped` item MUST have
   `export_class in {blocked_by_policy, in_support_bundle_redacted}`.
4. A `device_scoped` item that renders in recent-work lists MUST
   have `export_class = not_exported_machine_local` — recent-work
   metadata does not ride the portable profile package (see §5
   rule 1).

## 4. Entry-surface row record

Every entry-surface render emits exactly one
`entry_surface_row_record` per primary-action row, ancillary-action
row, or card. The record wraps — by reference — any upstream
record the row re-advertises (Start Center primary-action record,
recent-work row disclosure record, restore-card record, disclosure-
banner record, first-run tour step) and adds the account-prompt,
boundary-crossing, scoreboard, and deployment-profile axes that
make the no-account floor mechanical.

### 4.1 Required fields

- `record_kind = entry_surface_row_record`.
- `row_id` (opaque).
- `entry_surface_family` (§3.1).
- `deployment_profile_ref` — one `deployment_profile_id` from the
  deployment-profiles register.
- `startup_state_ref` — one `startup_state` token.
- `privacy_reduction_mode` — value from the Start Center contract
  (`default`, `hide_paths`, `hide_recent_work`,
  `hide_account_affordances`, `hide_all_except_open_and_clone`,
  `sensitive_environment_clear_on_launch`).
- `primary_action_id_ref` — required for rows that resolve to a
  Start Center primary action.
- `ancillary_action_id` — optional; for tour steps, banners,
  cards, and controls that are not a primary action. Drawn from
  `{tour_step, welcome_message, account_opt_in_card,
  onboarding_invite, dismissal_control, reset_onboarding_control,
  clear_recent_work_control, export_profile_control,
  exit_privacy_reduced_control}`.
- `wrapped_record_ref` — optional opaque ref to the upstream
  record (start_center_primary_action_record,
  recent_work_row_disclosure_record, restore_card_record,
  disclosure_banner_record). Recent-work and primary-action rows
  MUST carry this; first-run tour steps and account-opt-in cards
  that do not wrap an upstream record MAY omit it.
- `account_prompt_ref` — opaque ref to a companion
  `account_prompt_record` (§5). Every row whose
  `account_prompt_class` is not `no_prompt` or `not_applicable`
  MUST carry this.
- `account_prompt_class` (§3.2).
- `account_prompt_timing_class` (§3.3).
- `boundary_crossing_class` (§3.4).
- `boundary_crossings_detail[]` — optional list of additional
  subclass values from §3.4 when the row crosses more than one
  boundary.
- `scoreboard_row_ref` — one
  `scoreboard_row:<family>.<slug>` id from
  [`no_account_switching_scoreboard_seed.yaml`](../../artifacts/product/no_account_switching_scoreboard_seed.yaml).
  Required when `account_prompt_class` is `no_prompt`,
  `optional_prompt`, or `policy_forced_prompt`; optional otherwise.
- `measurement_surface_ref` — one value from the measurement plan
  surface vocabulary (`surface_first_run`, `surface_first_open`,
  `surface_first_useful_edit`, `surface_migration_review`,
  `surface_restore_success`, `surface_opt_in_boundary`).
- `entry_route_id` — `er.*` id from the measurement plan.
- `equal_weight_with_account_rows = true` — required when the row
  is a `no_prompt` primary action. The shell MUST NOT down-weight
  the row (smaller, below, or behind an account-opt-in card) just
  because the sibling row offers an opt-in.
- `disclosed_in_zone` — Start Center zone the row renders in.
- `keyboard_reachable = true` — required on every primary action
  and every `account_opt_in_card`; a card that requires a pointer
  to dismiss or accept is non-conforming.
- `minted_at` — monotonic timestamp.

### 4.2 Rules

1. A row whose `account_prompt_class` is `no_prompt` MUST render
   `equal_weight_with_account_rows = true` and MUST NOT be
   placed in a zone that sits below an `account_opt_in_card`
   on the same surface.
2. A row whose `primary_action_id_ref` is
   `primary_action.open_folder` MUST carry
   `account_prompt_class = no_prompt` on every non-unsupported
   startup state. Hiding `Open folder` behind a sign-in wall is
   non-conforming.
3. A row whose `privacy_reduction_mode` is
   `hide_recent_work` or `hide_paths` MUST still expose at least
   `primary_action.open_folder` and
   `primary_action.clone_repository` as `no_prompt` primary
   actions. The privacy-reduction mode controls which rows are
   visible, not whether the no-account floor still holds.
4. A row whose `privacy_reduction_mode` is
   `sensitive_environment_clear_on_launch` MUST still render the
   `Open folder` and `Clone repository` primary actions; a
   surface that wipes recent work AND removes the fastest path
   to open a local folder is non-conforming.
5. A row whose `entry_surface_family` is `first_run_tour_step`
   MUST carry `account_prompt_class` in `{no_prompt,
   optional_prompt}` and MUST NOT gate tour completion behind a
   `required_prompt` or `policy_forced_prompt`. The tour may
   invite an opt-in; it may not force one.
6. A row whose `entry_surface_family` is `account_opt_in_card`
   MUST carry an `account_prompt_class` in `{optional_prompt,
   deferrable_prompt, required_prompt, policy_forced_prompt,
   unavailable_prompt}` (never `no_prompt`, never
   `not_applicable`).

## 5. Account-prompt record

Every row that may render a prompt (anything not labelled
`no_prompt` or `not_applicable`) emits exactly one
`account_prompt_record` describing when and how the prompt fires.

### 5.1 Required fields

- `record_kind = account_prompt_record`.
- `prompt_id` (opaque).
- `entry_surface_family_ref` (§3.1).
- `account_prompt_class` (§3.2).
- `account_prompt_timing_class` (§3.3).
- `boundary_crossing_class` (§3.4).
- `boundary_crossings_detail[]` — optional subclass list.
- `declinable = true | false`. A
  `required_prompt` or `policy_forced_prompt` record MAY declare
  `declinable = false` to indicate the flow halts on decline;
  `optional_prompt` and `deferrable_prompt` MUST declare
  `declinable = true`.
- `decline_consequence_class` — drawn from the closed set
  `stays_in_local_only_lane`, `stays_in_restricted_mode`,
  `aborts_this_row_only`, `aborts_and_offers_local_alternative`,
  `unavailable_in_envelope_narrowing_advertised`.
  `decline_consequence_class` is required on every declinable
  prompt; the value MUST NOT be `degrades_local_capability` (no
  such value exists — declining a prompt MUST NOT silently degrade
  prior local capability).
- `active_policy_bundle_ref` — required when
  `account_prompt_class = policy_forced_prompt`.
- `policy_forced_reason_code` — required when
  `account_prompt_class = policy_forced_prompt`. Closed set:
  `managed_fleet_sso`, `compliance_telemetry_required`,
  `managed_cloud_authority_required`,
  `admin_approval_required_for_extension_host`,
  `consent_acknowledgement_required`.
- `unavailable_envelope_class` — required when
  `account_prompt_class = unavailable_prompt`. Closed set:
  `air_gapped`, `air_gapped_mirror_only`, `managed_fleet_blocked`,
  `restricted_mode`, `offline_indefinite`,
  `sensitive_environment_clear_on_launch`.
- `resolution_hooks[]` — typed `next_step_decision_hook` values;
  at least one required on every non-`no_prompt` record.
- `scoreboard_row_ref` — required when the prompt record sits on
  a boundary row the scoreboard qualifies.

### 5.2 Rules

1. **Decline never silently degrades local capability.** A
   prompt record whose decline consequence is "local capability
   becomes slower / narrower / unavailable even though the user
   had it before" is non-conforming. The closed
   `decline_consequence_class` set forbids that case by
   construction.
2. **Optional and deferrable prompts MUST NOT gate useful work.**
   An `optional_prompt` or `deferrable_prompt` that blocks
   `first_useful_edit_durable` or `first_useful_navigation_reached`
   is non-conforming (cross-check against scoreboard row
   `scoreboard_row:no_account_local_work.first_useful_edit_without_opt_in`).
3. **Policy-forced prompts name the bundle.** Every
   `policy_forced_prompt` record MUST carry a non-empty
   `active_policy_bundle_ref` and a `policy_forced_reason_code`
   from the closed set.
4. **Unavailable prompts name the envelope.** Every
   `unavailable_prompt` record MUST name the
   `unavailable_envelope_class` and render the absence-narrowing
   consequence from the underlying boundary-manifest row.

## 6. Onboarding portability state record

Every named onboarding state item emits exactly one
`onboarding_portability_state_record`. The manifest (§7) is the
top-level aggregator that declares the invariants.

### 6.1 Required fields

- `record_kind = onboarding_portability_state_record`.
- `state_item_id` — opaque id, drawn from a stable namespace
  (`state_item.tour_progress`,
  `state_item.dismissal.start_center_welcome_banner`,
  `state_item.imported_profile_history`,
  `state_item.onboarding_posture`,
  `state_item.recent_work_metadata`,
  `state_item.restore_prompt_skipped_registry`,
  `state_item.first_run_seen_flag`,
  `state_item.consent_acknowledgement_log`,
  `state_item.connected_provider_approval_ticket`,
  `state_item.decode_recovery_ring`,
  `state_item.session_execution_posture_cache`).
- `state_portability_class` (§3.5).
- `reset_class` (§3.6).
- `export_class` (§3.6).
- `portability_reason` — 1–1024-grapheme free text a reviewer can
  read; names why this item lives in that class. Required on
  every record.
- `profile_scope_class` — closed set:
  `per_profile`, `per_device`, `per_account`, `per_policy_bundle`,
  `per_session`. MUST be consistent with
  `state_portability_class` (see rule 1).
- `recovery_binding_class` — optional; closed set:
  `no_recovery_binding`, `recovery_ladder_participant`,
  `restore_prompt_input`, `migration_rollback_evidence`,
  `privacy_reduction_governed`. Indicates whether the item shows
  up in the recovery ladder packet, restore prompt, migration
  rollback evidence, or privacy-reduction clear-on-launch flow.
- `downstream_surface_refs[]` — optional refs to the surfaces
  that read the item (Start Center, switcher, tour, restore
  prompt, privacy-reduction notice, import rollback screen).

### 6.2 Portability-class / profile-scope consistency

1. `portable_profile_state` ↔ `per_profile`.
2. `machine_local_diagnostic` ↔ `per_device`.
3. `device_scoped` ↔ `per_device`.
4. `account_scoped` ↔ `per_account`.
5. `policy_scoped` ↔ `per_policy_bundle`.
6. `ephemeral_session` ↔ `per_session`.

A record that declares a `state_portability_class` /
`profile_scope_class` pair outside the six above is a schema
violation.

### 6.3 Canonical state-item table

This contract freezes the portability class for the following
named onboarding state items. Downstream features that introduce
new items seat a new row here in the same change.

| `state_item_id` | `state_portability_class` | `reset_class` | `export_class` | Why (portability_reason) |
|---|---|---|---|---|
| `state_item.tour_progress` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | Tour progress moves with the user across devices; resetting onboarding is expected. |
| `state_item.dismissal.start_center_welcome_banner` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | "I've acknowledged this" dismissals are profile-level intent. |
| `state_item.first_run_seen_flag` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | Whether first-run already ran is a profile fact, not a device fact. |
| `state_item.imported_profile_history` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package_redacted` | History of competitor-config imports and migration outcomes rides with the profile, redacting paths. |
| `state_item.onboarding_posture` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | Overall onboarding posture (novice / standard / advanced) is a profile preference. |
| `state_item.recent_work_metadata` | `device_scoped` | `resettable_per_device` | `not_exported_machine_local` | Recent-work list is device-local; the portable profile package does not leak workspace identifiers across devices. |
| `state_item.restore_prompt_skipped_registry` | `device_scoped` | `resettable_per_device` | `not_exported_machine_local` | Skipped restore prompts are device-local evidence; they do not travel with the profile. |
| `state_item.consent_acknowledgement_log` | `policy_scoped` | `resettable_by_policy` | `in_support_bundle_redacted` | Consent acknowledgements belong to the active policy bundle. |
| `state_item.connected_provider_approval_ticket` | `account_scoped` | `resettable_per_account` | `not_exported_machine_local` | Approval tickets are account-bound and do not leak across accounts. |
| `state_item.decode_recovery_ring` | `machine_local_diagnostic` | `resettable_per_device` | `in_support_bundle_redacted` | Diagnostic ring for decode-recovery cases; stays on the device, rides only the redacted support bundle. |
| `state_item.session_execution_posture_cache` | `ephemeral_session` | `resettable_per_device` | `not_exported_machine_local` | Per-session pane execution posture; lives for the session and is rebuilt on open. |

The full machine-readable table lives in the manifest record (§7)
and its fixture.

## 7. Onboarding portability manifest record

Exactly one `onboarding_portability_manifest_record` aggregates
every `onboarding_portability_state_record` in the repository and
declares the cross-cutting invariants. The manifest is the record
that governance tooling and the privacy-reduction review surface
read.

### 7.1 Required fields

- `record_kind = onboarding_portability_manifest_record`.
- `manifest_id` (opaque).
- `state_items[]` — list of
  `onboarding_portability_state_record` refs.
- `entry_surface_row_refs[]` — list of
  `entry_surface_row_record` refs the manifest is jointly
  validating (first-run, Start Center, switcher, tour,
  restore-card, welcome banner, account-opt-in card, onboarding-
  prompt rows).
- `invariants[]` — const-true invariant strings (see §7.2).
- `minted_at` — monotonic timestamp.

### 7.2 Required invariants (const true)

Every manifest record MUST declare every invariant below as const
`true`:

1. `no_account_floor_preserved_on_every_seeded_startup_state` —
   on every seeded startup state that is not an
   `unavailable_prompt` takeover, at least one primary action
   renders with `account_prompt_class = no_prompt` and the floor
   includes `primary_action.open_folder`.
2. `no_forced_first_run_sign_in` — on
   `startup_state:first_run`, no `primary_action_id` row renders
   with `account_prompt_class = required_prompt` unless the
   active envelope is `policy_forced_prompt` or
   `unavailable_prompt`.
3. `privacy_reduction_preserves_open_and_clone` — every non-
   `default` `privacy_reduction_mode` (including
   `hide_all_except_open_and_clone` and
   `sensitive_environment_clear_on_launch`) still renders
   `primary_action.open_folder` and
   `primary_action.clone_repository` as `no_prompt` rows.
4. `clear_recent_work_does_not_remove_open_path` — clearing
   recent-work metadata (via `clear_recent_work_control` or
   `sensitive_environment_clear_on_launch`) MUST NOT hide the
   fastest path to open a local folder or workspace.
5. `no_account_rows_equal_weight_with_opt_in_rows` — on every
   surface, `no_prompt` primary-action rows render
   `equal_weight_with_account_rows = true`.
6. `decline_never_silently_degrades_local_capability` — no
   prompt record declares a `decline_consequence_class` outside
   the closed set, and no row whose prompt was declined reports
   a prior-local capability as newly unavailable without an
   advertised narrowing citation.
7. `one_state_item_one_portability_class` — every named
   onboarding state item declares exactly one
   `state_portability_class`; dual-homing is non-conforming.
8. `portable_profile_state_never_leaks_account_scoped_items` —
   every `account_scoped` item has `export_class` in
   `{not_exported_machine_local, in_support_bundle_redacted}`;
   none export through `in_portable_profile_package`.
9. `recent_work_metadata_is_device_scoped` — recent-work
   metadata, restore-prompt skip registry, and machine-local
   diagnostic rings declare `state_portability_class` in
   `{device_scoped, machine_local_diagnostic}` and
   `export_class = not_exported_machine_local` (diagnostics MAY
   additionally ride the redacted support bundle).
10. `boundary_crossing_truth_matches_prompt_class` — every
    `no_prompt` row declares
    `boundary_crossing_class = no_boundary_crossed`, and every
    row that declares a non-`no_boundary_crossed` crossing
    carries a non-`no_prompt` / non-`not_applicable` prompt
    class.
11. `every_onboarding_state_item_is_declared` — every onboarding
    state item referenced by any
    `entry_surface_row_record` or
    `account_prompt_record` is declared in this manifest;
    silent hidden portable state is non-conforming.
12. `every_policy_forced_prompt_cites_a_bundle` — every row or
    prompt record with
    `account_prompt_class = policy_forced_prompt` cites a non-
    empty `active_policy_bundle_ref` and a closed
    `policy_forced_reason_code`.

A manifest that declares any invariant as `false` is non-
conforming by construction.

## 8. Cross-cutting rules

1. **First-run no-account local-work path is mechanical.** On
   `startup_state:first_run`, the Start Center renders all five
   first-launch primary actions as `no_prompt` rows under the
   `individual_local` profile. On `self_hosted`, the Start
   Center MAY narrow telemetry and sync to `optional_prompt`
   rows; it MUST NOT narrow the `no_prompt` floor.
2. **Service-opt-in boundary is explicit.** Every crossing out
   of the account-free lane renders an
   `account_opt_in_card` with `account_prompt_class` in
   `{optional_prompt, deferrable_prompt, required_prompt,
   policy_forced_prompt, unavailable_prompt}` and cites the
   boundary-manifest row the crossing leaves.
3. **Decline is a first-class outcome.** Every declinable
   prompt declares `decline_consequence_class` from the closed
   set; the closed set forbids silent degradation by
   construction.
4. **Offline and managed behaviour.** On
   `startup_state:offline_startup`, remote-preferred rows render
   `required_prompt` with timing `shown_only_on_boundary_crossing`
   and remedy hooks (`reconnect_required`, `reauth_required`);
   local-only rows stay `no_prompt`. On `managed_cloud` and
   `enterprise_online`, managed-cloud-dependent rows MAY declare
   `policy_forced_prompt` if the active policy bundle demands it;
   the local-only floor still holds.
5. **Portable profile state is declared once.** Every onboarding
   state item has exactly one row in the canonical state-item
   table (§6.3) or its machine-readable companion; downstream
   features do not mint hidden portable state.
6. **Privacy-reduction does not remove the no-account floor.**
   `hide_recent_work`, `hide_all_except_open_and_clone`, and
   `sensitive_environment_clear_on_launch` hide rows; they do
   not drop `Open folder` or `Clone repository`. Every
   privacy-reduction mode renders a
   `privacy_reduced_mode_notice` banner (per Start Center
   contract §3.5) so the user can distinguish a hidden row from
   a missing row.
7. **Equal-weight rule.** A `no_prompt` primary action MUST NOT
   render smaller, dimmer, below, or behind an
   `account_opt_in_card`. Zone placement follows the Start
   Center zone contract.
8. **Scoreboard linkage.** Every `no_prompt`,
   `optional_prompt`, and `policy_forced_prompt` row cites a
   scoreboard row so every first-run-without-account claim is
   qualified by a measurement surface, not promised by copy.
9. **Every onboarding state item has a reset path.** Even
   `not_resettable_locally` items must be listed, so support
   and policy reviewers can see what cannot be cleared without
   the policy bundle.
10. **No hidden service dependencies at first run.** If a
    primary-action row resolves `no_prompt` but the action
    actually triggers a service-plane boundary crossing, both
    the schema (rule on `boundary_crossing_class` ↔
    `account_prompt_class`) and the policy-review lane flag it
    as non-conforming. The invariant closes the drift path.

## 9. Deployment-profile matrix

How each deployment profile shapes the contract. The table below
summarises; individual rows live in the scoreboard and manifest
fixtures.

| `deployment_profile_id` | No-account floor | Typical `account_prompt_class` default | Notes |
|---|---|---|---|
| `individual_local` | Five first-launch primary actions rendered as `no_prompt`. | `no_prompt` for Open / Clone (public) / Import / Restore / first useful edit; `optional_prompt` for telemetry, sync, BYOK-AI, marketplace-browse. | The canonical local-first case. Offline is supported natively. |
| `self_hosted` | Five first-launch primary actions rendered as `no_prompt` for the operator identity; tenant users may see narrowed bundles. | `optional_prompt` for telemetry; `required_prompt` only on explicit clones of private remotes. | The self-hosted control plane governs sync / authority; local editing remains `no_prompt`. |
| `enterprise_online` | Five first-launch primary actions rendered as `no_prompt` or `policy_forced_prompt` depending on the active policy bundle. | `required_prompt` or `policy_forced_prompt` for managed-cloud actions; `no_prompt` for Open folder / Clone public. | Policy bundle MUST be cited on every `policy_forced_prompt`. |
| `air_gapped` | Five first-launch primary actions rendered; marketplace-browse, browser-handoff, and managed-cloud rows render `unavailable_prompt`. | `no_prompt` on Open / local Clone / Restore / Import from local package; `unavailable_prompt` on any network-touching opt-in. | Absence-narrows-to is advertised inline. |
| `managed_cloud` | `Open folder` (local) remains `no_prompt`; managed-cloud workspaces render `required_prompt` or `policy_forced_prompt`. | `required_prompt` on resume of managed-cloud workspaces with expired authority. | Local floor still holds; the managed-cloud rows add on top. |
| `air_gapped_mirror_only` | Same as `air_gapped`; the mirror route is the only remote-side surface. | `unavailable_prompt` on every non-mirror remote. | `freshness_class` and `absence_class` render on every cached row. |

## 10. Seeded-state matrix

Anchors each `startup_state` token to the account-prompt posture
an entry surface renders.

| `startup_state` | First-launch primary actions | Recent-work rows | Restore card | Default `account_prompt_class` | Account-opt-in cards allowed where |
|---|---|---|---|---|---|
| `first_run` | Five; all `no_prompt`. | Empty. | None. | `no_prompt`. | `profile_selection` or `secondary_entry`, never above `primary_work_resume`. |
| `reopen_with_pending_restore` | Five; primary `no_prompt` where possible, `required_prompt` on remote rows. | Populated per disclosure posture. | Required; three typed actions. | Varies. | Same as first_run. |
| `restore_failed` | Five; restore disabled. | Populated. | Required; evidence-only. | Varies. | Same as first_run. |
| `restore_skipped` | Five. | Populated. | None. | Varies. | Same as first_run. |
| `open_without_restore` | Five. | Populated. | None. | Varies. | Same as first_run. |
| `warming_startup` | Five; stale-metadata rows chip-marked. | Populated. | None. | Varies. | Same as first_run. |
| `partial_startup` | Five; impacted rows render remedy hooks. | Populated. | None. | Varies. | Same as first_run. |
| `offline_startup` | Five; remote rows `required_prompt` with offline remedy; local rows `no_prompt`. | Populated; remote rows `stale_offline` / `unknown_since`. | None unless pending. | `required_prompt` on remote; `no_prompt` on local. | `profile_selection` only. |
| `unsupported_startup` | May narrow to Open + Clone (see Start Center §3.7 rule 1). | May be hidden. | None. | `unavailable_prompt` on affected rows. | Policy banner blocks above `primary_work_resume`. |
| `empty_state_or_placeholder_transition` | Five; truthful about placeholder posture. | Empty / transitional. | None. | `no_prompt` minimum. | Same as first_run. |

## 11. Worked examples

Each example has a companion fixture under
[`/fixtures/ux/entry_surface_rows/`](../../fixtures/ux/entry_surface_rows/).

### 11.1 First-run no-account local folder row

Fresh install on a previously-unused device on the
`individual_local` profile. The `primary_action.open_folder` row
renders `account_prompt_class = no_prompt`,
`boundary_crossing_class = no_boundary_crossed`,
`scoreboard_row_ref = scoreboard_row:no_account_local_work.first_useful_edit_without_opt_in`,
`equal_weight_with_account_rows = true`. See
[`first_run_no_account_open_folder.json`](../../fixtures/ux/entry_surface_rows/first_run_no_account_open_folder.json).

### 11.2 Optional telemetry opt-in

First-run side affordance asking whether to enable telemetry.
`account_prompt_class = optional_prompt`,
`account_prompt_timing_class = shown_at_first_run_declinable`,
`boundary_crossing_class = emits_managed_telemetry`,
`decline_consequence_class = stays_in_local_only_lane`,
`scoreboard_row_ref = scoreboard_row:service_opt_in_boundary.telemetry_optional_at_first_run`.
See
[`service_opt_in_telemetry_optional.json`](../../fixtures/ux/entry_surface_rows/service_opt_in_telemetry_optional.json).

### 11.3 Managed-cloud resume requires reauth

Mid-session resume of a managed-cloud workspace after authority
lapse. `account_prompt_class = required_prompt`,
`account_prompt_timing_class = shown_at_entry_declinable`,
`boundary_crossing_class = reaches_remote_resource`,
`boundary_crossings_detail = [touches_credential_store,
opens_browser_handoff]`,
`decline_consequence_class = aborts_and_offers_local_alternative`,
`scoreboard_row_ref = scoreboard_row:service_opt_in_boundary.resume_managed_reauth_required`.
See
[`managed_cloud_resume_reauth_required.json`](../../fixtures/ux/entry_surface_rows/managed_cloud_resume_reauth_required.json).

### 11.4 Policy-forced enterprise SSO

Managed-fleet machine whose active policy bundle requires SSO
before any workspace opens. `account_prompt_class =
policy_forced_prompt`, `account_prompt_timing_class =
forced_by_policy`, `policy_forced_reason_code =
managed_fleet_sso`, `active_policy_bundle_ref = policy:fleet-sso-v7`.
`primary_action.open_folder` still renders `no_prompt` for
local-only folders if the bundle permits; otherwise it declares
`policy_forced_prompt` with the `continue_in_restricted_mode`
resolution hook. See
[`policy_forced_managed_fleet_sso.json`](../../fixtures/ux/entry_surface_rows/policy_forced_managed_fleet_sso.json).

### 11.5 Air-gapped marketplace unavailable

Air-gapped-mirror-only envelope. The marketplace-browse
`account_opt_in_card` renders
`account_prompt_class = unavailable_prompt`,
`account_prompt_timing_class = unavailable_in_envelope`,
`unavailable_envelope_class = air_gapped_mirror_only`, and
quotes the boundary-manifest row's absence-narrows-to
("browse UI is unavailable; CLI and mirror installs remain
available"). See
[`air_gapped_marketplace_unavailable.json`](../../fixtures/ux/entry_surface_rows/air_gapped_marketplace_unavailable.json).

### 11.6 Privacy-reduced sensitive environment

Shared-device launch with
`privacy_reduction_mode = hide_all_except_open_and_clone`.
Recent-work rows hidden; `primary_action.open_folder` and
`primary_action.clone_repository` still render as `no_prompt`.
A `privacy_reduced_mode_notice` banner renders above
`primary_work_resume`. `Clear recent work` and
`Exit privacy-reduced mode` controls render in the footer. See
[`privacy_reduced_open_and_clone_preserved.json`](../../fixtures/ux/entry_surface_rows/privacy_reduced_open_and_clone_preserved.json).

### 11.7 Portable tour progress

`state_item.tour_progress` lives in
`portable_profile_state`, `resettable_per_profile`,
`in_portable_profile_package`, `per_profile`. See
[`onboarding_state_tour_progress_portable.json`](../../fixtures/ux/entry_surface_rows/onboarding_state_tour_progress_portable.json).

### 11.8 Device-scoped recent-work metadata

`state_item.recent_work_metadata` lives in
`device_scoped`, `resettable_per_device`,
`not_exported_machine_local`, `per_device`. Intentionally not
in the portable profile package so workspace identifiers do not
leak across devices. See
[`onboarding_state_recent_work_metadata_device_scoped.json`](../../fixtures/ux/entry_surface_rows/onboarding_state_recent_work_metadata_device_scoped.json).

### 11.9 Imported-profile history rides the portable package

`state_item.imported_profile_history` lives in
`portable_profile_state`, `resettable_per_profile`,
`in_portable_profile_package_redacted`, `per_profile`. Moves
with the user so migration outcomes are reviewable after a
device change. See
[`onboarding_state_imported_profile_history_portable.json`](../../fixtures/ux/entry_surface_rows/onboarding_state_imported_profile_history_portable.json).

### 11.10 Onboarding portability manifest

Top-level
`onboarding_portability_manifest_record` binding the state-item
rows, the entry-surface-row rows, and the twelve const-true
invariants. Reviewers read this record to verify the no-account
floor, the boundary-crossing truth rule, and the single-class
portability rule hold across the repo at one revision. See
[`onboarding_portability_manifest.json`](../../fixtures/ux/entry_surface_rows/onboarding_portability_manifest.json).

## 12. Acceptance mapping

- **Local-first or account-free rows have an explicit no-account
  path rather than an implied one.** The
  `account_prompt_class = no_prompt` value and the
  `boundary_crossing_class = no_boundary_crossed` value,
  together with invariant §7.2.1 and §7.2.10 and the
  `equal_weight_with_account_rows` flag, make the no-account
  path a declared fact on every row, not an inference from
  absence.
- **Reviewers can tell which onboarding state is portable,
  resettable, exportable, or machine-local with documented
  reasons.** The
  `onboarding_portability_state_record` shape (§6) carries
  `state_portability_class`, `reset_class`, `export_class`,
  `profile_scope_class`, and a mandatory `portability_reason`
  free-text anchor. The canonical state-item table (§6.3) seats
  the initial rows; the manifest (§7) aggregates them.
- **Hidden service dependencies at first run become schema or
  policy violations rather than UX drift.** The
  `boundary_crossing_class` / `account_prompt_class` consistency
  rule (§3.4 rule 1), the `no_account_floor_preserved_on_every_seeded_startup_state`
  invariant (§7.2.1), and the `boundary_crossing_truth_matches_prompt_class`
  invariant (§7.2.10) together make a hidden crossing a
  structural non-conformance, not a reviewer nag.
- **Privacy-reduction never removes the Open / Clone floor.**
  The `privacy_reduction_preserves_open_and_clone` invariant
  (§7.2.3) and the `clear_recent_work_does_not_remove_open_path`
  invariant (§7.2.4) seat the rule.
- **No-account rows equal-weight with service-opt-in rows.** The
  `equal_weight_with_account_rows = true` requirement on every
  `no_prompt` primary action (§4.1) and the
  `no_account_rows_equal_weight_with_opt_in_rows` invariant
  (§7.2.5) seat the rule.

## 13. Changing this contract

- **Additive-minor** changes (new `entry_surface_family`, new
  `account_prompt_class`, new `account_prompt_timing_class`, new
  `boundary_crossing_class`, new `state_portability_class`, new
  `reset_class`, new `export_class`, new `profile_scope_class`,
  new `recovery_binding_class`, new `policy_forced_reason_code`,
  new `unavailable_envelope_class`, new canonical state item)
  land here and in the companion schema plus at least one
  fixture in the same change. Every new value MUST cite the
  motivating envelope or state item.
- **Repurposing** an existing class is breaking and opens a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (entry-verb, target-kind,
  resulting-mode, restore-level, missing-target-state, recovery-
  class, next-step-decision-hook, `account_opt_in_posture`,
  `privacy_reduction_mode`, primary-action-id) happen in the
  upstream contracts; this contract re-exports by reference and
  MUST NOT shadow the change.
- The PRD / TAD / TDD / UI-UX spec wins on any disagreement
  with the quotations below.

## 14. Source anchors

- `.t2/docs/Aureline_PRD.md:284` — first-run onboarding is a
  launch risk, not polish.
- `.t2/docs/Aureline_PRD.md:1300` — crash recovery MUST degrade
  gracefully from exact session restore to dirty-buffer recovery
  to open-clean with preserved evidence.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:762` — Start Center
  primary actions.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:784` — recent-work
  row fields.
- `.t2/docs/Aureline_Milestones_Document.md:1023` — Start
  Center keeps Open / Clone / Import / Restore / Recent work
  distinct with a no-account local path.
- `.t2/docs/Aureline_Milestones_Document.md:1544` — entry verbs
  stay distinct across Start Center, command surfaces, OS file
  association, drag-and-drop, and CLI / headless entry.

## 15. Linked artifacts

- Start Center / workspace-switcher / open-flow contract:
  [`/docs/ux/start_center_contract.md`](./start_center_contract.md).
- Entry / restore object model:
  [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md).
- Entry / restore truth audit:
  [`/docs/ux/entry_restore_truth_audit.md`](./entry_restore_truth_audit.md).
- Onboarding / first-useful-work / migration measurement plan:
  [`/docs/product/onboarding_measurement_plan.md`](../product/onboarding_measurement_plan.md).
- No-account switching scoreboard seed:
  [`/artifacts/product/no_account_switching_scoreboard_seed.yaml`](../../artifacts/product/no_account_switching_scoreboard_seed.yaml).
- Task-success corpus seed:
  [`/artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml).
- Deployment-profile register:
  [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml).
- Start Center surface schema:
  [`/schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json).
- Onboarding portability state schema:
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json).
- Worked-example fixtures:
  [`/fixtures/ux/entry_surface_rows/`](../../fixtures/ux/entry_surface_rows/).
