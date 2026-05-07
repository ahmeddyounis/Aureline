# Template gallery, prebuild / warm-start, resume-live, and open-without-starter disclosure contract

This document freezes the cross-surface disclosure contract every
**template gallery**, **prebuild / warm-start picker**, **resume-
live workspace card**, **environment-starter summary sheet**,
**post-create handoff summary**, **template-health row**, and
**starter policy notice** inherits before the scaffolding,
prebuild-service, warm-start, or managed-workspace launcher
surfaces are implemented. The goal is an honest, same-weight
choice at startup: templates and prebuilds stay accelerators, not
hidden infrastructure lock-in, and `Open folder / workspace
without the starter` remains a first-class path at every step of
the flow.

The companion machine-readable matrix lives at:

- [`/artifacts/ux/template_source_class_matrix.yaml`](../../artifacts/ux/template_source_class_matrix.yaml)

The companion fixtures live under:

- [`/fixtures/ux/template_and_prebuild_states/`](../../fixtures/ux/template_and_prebuild_states/)

The warm-start chooser freshness + revalidation contract (lane-level truth that
the resume-live card and prebuild picker MUST preserve) lives at:

- [`/artifacts/entry/warm_start_chooser_contract.md`](../../artifacts/entry/warm_start_chooser_contract.md)
- [`/schemas/entry/freshness_revalidation.schema.json`](../../schemas/entry/freshness_revalidation.schema.json)
- [`/fixtures/entry/warm_start_cases/`](../../fixtures/entry/warm_start_cases/)

This contract is normative for the disclosure posture. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, or milestone document
anchors quoted in §14, those sources win and this document plus
its companion matrix and fixtures update in the same change. Where
a downstream gallery, picker, summary sheet, health row, handoff
card, or policy notice mints a parallel vocabulary, this contract
wins and the surface is non-conforming.

This contract mints **no** new `entry_verb`, `target_kind`,
`resulting_mode`, `trust_state`, `admission_class`,
`restore_level`, `missing_target_state`, `recovery_class`,
`next_step_decision_hook`, `locator_class`, `transport_class`,
`acquisition_posture`, `trust_stage`, `browse_safe_action`,
`blocked_execution_path`, `resume_state`, `signer_continuity_class`,
`topology_marker`, `bootstrap_item_class`, `execution_class`,
`absence_class`, `blocker`, `skip_reason`, `repair_hook`, or
`attributable_evidence` values. Every row here re-exports — by
reference — the vocabulary frozen in
[`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
(§1–§4) and
[`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../workspace/source_acquisition_and_bootstrap_seed.md)
(§1–§3), and quotes the `startup_state` tokens from
[`/docs/ux/entry_restore_truth_audit.md`](./entry_restore_truth_audit.md)
(§6). The Start Center, workspace-switcher, open-flow, restore-
card, and recent-work disclosure contract in
[`/docs/ux/start_center_contract.md`](./start_center_contract.md)
owns the five first-launch primary actions, the seeded-state
matrix, the zone ordering, and the privacy-reduction posture that
gallery surfaces compose into. This contract owns the disclosure
posture **inside** the template, prebuild, starter-summary,
resume-live, post-create handoff, and template-health surfaces
that those verbs route into.

## Who reads this contract

- **Template-gallery, prebuild-picker, environment-starter, and
  post-create-handoff authors** wiring the surfaces the shell
  routes `primary_action.start_from_template_or_prebuild`,
  `primary_action.clone_repository` with a declared starter, and
  any `resume_live_session` card into. Every row, sheet, and card
  on those surfaces resolves through a record shape defined here.
- **Designers** sizing starter cards, preflight sheets, health
  rows, policy banners, and resume-live cards so source, support,
  runtime, freshness, trust, setup cost, and the bypass path land
  same-weight with the primary open path on the same surface.
- **Docs, support, compatibility, and measurement authors**
  attributing starter-open, prebuild-attach, resume-live, and
  failed-setup evidence to the same record kinds the shell
  renders.

## 1. Scope

- Freeze one `template_and_prebuild_gallery_record` that every
  first-launch gallery, in-session gallery, and workspace-
  switcher gallery entry reads.
- Freeze one `template_card_record` per template row and one
  `prebuild_card_record` per warm-start / prebuild row, each
  quoting source class, support class, runtime / toolchain scope,
  declared freshness, signer-continuity class, setup-cost class,
  policy narrowing, and bypass-path id verbatim.
- Freeze one `environment_starter_summary_record` that renders
  the side-effect envelope the user reviews before a template or
  prebuild is committed. The envelope covers the six axes UI/UX
  spec §6.9 freezes — identity / provenance, expected result,
  setup actions, time / connectivity expectation, cleanup or
  rollback path, bypass path — and the additional typed
  previews of extension install, task run, secret handle, trust
  grant, remote provisioning, and port binding.
- Freeze one `resume_live_workspace_card_record` for managed-
  cloud, remote, devcontainer, and container workspaces whose
  `entry_verb` is `resume` and `resulting_mode` is
  `resume_live_session`. The card separates **resume live**,
  **reopen from prebuild / snapshot**, **clone fresh**, and
  **open without starter**; it never collapses them into one
  generic `Open` button.
- Freeze one `post_create_handoff_summary_record` that reports
  what the starter actually did — what succeeded, what was
  skipped, what was partially applied, what cleanup ran — and
  what local-safe continuation path remains when setup fails.
- Freeze one `template_health_row_record` that labels each
  health check as `live`, `cached`, `policy_evaluated`,
  `not_checked`, or `stale_or_invalid` so `healthy` does not
  collide with "last known good."
- Freeze one `starter_policy_notice_record` that explains, in-
  place, why availability is narrowed when a fleet, admin,
  workspace-trust, connected-provider, signature, mirror, or
  air-gap policy excludes starter rows or setup actions. The
  notice never silently drops rows; it names the narrowing class
  and the resolution hook.
- Name the closed vocabularies (§3) that bound source class,
  support class, runtime / toolchain scope, lifecycle class,
  freshness (re-exported), setup-cost class, availability-
  narrowing class, bypass-path id, template-health signal and
  check class, generation-preflight axes, post-create handoff
  axes, and policy-notice class.

## 2. Out of scope

- The template registry, the prebuild service, and the warm-
  start backend. Protocols, storage, and invalidation rules live
  with the repository-edge and release-artifact contracts; this
  document only pins the closed sets the gallery / picker /
  summary surfaces resolve against.
- The environment-capsule, devcontainer, Nix, Compose, or
  toolchain-manifest parser implementations. The source-
  acquisition seed in
  [`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../workspace/source_acquisition_and_bootstrap_seed.md)
  reserves the bootstrap-item classes those parsers enqueue; this
  contract reserves the disclosure slots the gallery renders.
- Managed-workspace provisioning, suspend / resume / rebuild, and
  expiry lifecycle. UI/UX spec §16.11 owns the managed-workspace
  state taxonomy; this contract re-exports the relevant slots
  onto the resume-live card shape only.
- Final user-facing copy / microcopy. The shell interaction-
  safety contract and the UX style guide own the exact strings;
  this contract pins the closed sets the copy resolves against.
- Telemetry wire format. The onboarding / first-useful-work
  measurement plan reserves the event names; this contract only
  tags records with the `entry_route_id` the plan cites.

## 3. Frozen vocabulary (re-exported) and new closed sets

This contract mints no new entry-restore or source-acquisition
vocabulary. It re-exports by reference:

- `entry_verb`, `target_kind`, `resulting_mode`, `source_surface`,
  `admission_class`, `destination_disposition`, `collision_class`,
  `next_step_decision_hook`, `side_effect_envelope` — entry-
  restore object model §1.
- `recent_work_target_state`, `portability_class`,
  `restore_availability`, `safe_recovery_action` — §2.
- `restore_level`, `missing_target_state`,
  `session_execution_posture`, `checkpoint_linked_recovery_class`
  — §3–§4.
- `locator_class`, `transport_class`, `acquisition_posture`,
  `freshness_class` (declared), `signer_continuity_class`,
  `trust_stage`, `browse_safe_action`, `blocked_execution_path`,
  `resume_state`, `discard_posture`, `mirror_freshness`,
  `read_only_partial_root_class`, `topology_marker`,
  `policy_narrowing_class`, `acquisition_next_step_hook` —
  source-acquisition seed §1–§2.
- `bootstrap_item_class`, `bootstrap_execution_class`,
  `bootstrap_item_state`, `bootstrap_absence_class`,
  `bootstrap_skip_reason`, `bootstrap_blocker`,
  `bootstrap_attributable_evidence`, `bootstrap_repair_hook` —
  source-acquisition seed §3.
- `trust_state` — ADR-0001.
- `startup_state` tokens — entry-restore truth audit §6.
- `navigation_route_id`, `escalation_tier`, `disclosure_depth` —
  navigation and escalation contract §3.
- `start_center_zone`, `primary_action_id`, `disclosure_class`,
  `privacy_reduction_mode`, `freshness_class`/`absence_class`
  (row-level) — start-center contract §3.

This contract introduces thirteen small vocabularies that are
scoped to template / prebuild / resume-live / starter-summary /
post-create / health / policy surfaces and never substitute for
any frozen upstream vocabulary:

### 3.1 `template_and_prebuild_surface_family`

Which surface is being described. The set is closed:

- `template_gallery` — the browseable gallery of templates and
  scaffold packs (first-launch or in-session).
- `prebuild_picker` — the warm-start / prebuild picker that
  lists attachable prebuild snapshots for a given source.
- `environment_starter_summary` — the preflight summary sheet
  the user reviews before a template or prebuild is committed.
- `resume_live_card` — the card for a workspace whose runtime
  is still live (managed-cloud, SSH, devcontainer, container,
  remote tunnel) and `entry_verb = resume`.
- `post_create_handoff` — the handoff summary after a starter
  runs to completion, partial success, or failure.
- `template_health_row` — a single row in the template-health
  report surface (per-template and per-detail-page).
- `starter_policy_notice` — a disclosure notice that explains,
  in-place, why availability is narrowed by policy / mirror /
  air-gap / signature review.

Rules (frozen):

1. A surface that mints an eighth family (`quick_start_tile`,
   `project_wizard`, etc.) is non-conforming; the seven
   families above are the full set.
2. `template_gallery`, `prebuild_picker`, and `resume_live_card`
   MUST expose an equal-weight bypass path on the same surface
   (§3.9). A family that hides the bypass path behind a "more"
   menu is non-conforming.
3. `environment_starter_summary` MUST render before commit; a
   surface that runs setup actions with the summary only
   rendered after the click is non-conforming.

### 3.2 `template_source_class`

Who authored the starter and under what support and trust
posture. The set is closed (re-exports UI/UX spec §6.9 plus two
availability-specific classes):

- `first_party` — authored and signed by Aureline; appears on
  every build with the build-channel identity.
- `team_managed` — authored by the user's organisation, admin,
  or fleet policy bundle; signed by the team-managed signer.
- `community` — authored by an external contributor and
  distributed through a governed community channel.
- `local_only` — authored locally (user or project) and never
  promoted beyond the current device.
- `mirror_cached` — a first-party, team-managed, or community
  template or prebuild delivered through a mirror / proxy /
  offline bundle rather than live origin.
- `uncertified` — a source whose provenance has not been
  asserted (no signature, no support posture, no freshness
  evidence). Allowed in gallery with explicit notice only;
  forbidden above `uncertified_excluded` policy narrowing.

Rules (frozen):

1. Every template or prebuild row names exactly one
   `template_source_class`. A surface that emits a row without a
   source class is non-conforming.
2. A `mirror_cached` row MUST also name the underlying source
   class it mirrors (`mirror_cached_of: first_party|team_managed|community`)
   so the user can tell what is being mirrored from where.
3. A `community` or `uncertified` row MUST carry a
   `signer_continuity_class` from the source-acquisition seed
   §1.5. A row that elides the signer posture is non-
   conforming.
4. A `local_only` template MUST NOT be promoted into an
   authority-carrying starter (no silent trust widening, no
   implicit signer). Its rollout path rides through an
   explicit publish flow.

### 3.3 `support_class`

How the starter is supported over time. The set is closed:

- `officially_supported`
- `community_supported`
- `experimental`
- `legacy_deprecated`
- `unsupported`
- `support_unknown`

Rules (frozen):

1. Every gallery row names exactly one `support_class`;
   omitting the class is non-conforming.
2. `support_unknown` is allowed only on `community` or
   `uncertified` sources and MUST pair with a review hook
   drawn from the entry-restore §1.7 set
   (`review_trust_and_open` or `review_archetype_match`).
3. `legacy_deprecated` and `unsupported` MUST render a
   deprecation or unsupported notice inline on the card;
   silent placement of these rows without the notice is non-
   conforming.

### 3.4 `runtime_and_toolchain_scope`

Where the starter's runtime and toolchain actually live. The set
is closed:

- `local_only` — the resulting workspace runs entirely on the
  local device with no devcontainer, no remote image, and no
  managed-cloud dependency.
- `local_with_devcontainer` — local device, but setup attaches
  a devcontainer (`devcontainer_attach` bootstrap item).
- `local_with_container` — local device, but setup attaches a
  non-devcontainer container runtime (`container_runtime`
  transport class).
- `remote_image_required` — the resulting workspace runs on a
  remote image (SSH, container, or managed runtime) and cannot
  operate locally.
- `managed_cloud_required` — the resulting workspace requires
  a managed-cloud workspace (UI/UX §16.11); `resume_live` is
  the typical follow-up verb.
- `mixed_local_and_remote` — some work is local, some is
  remote; the card MUST split the scope into an explicit local
  part and an explicit remote part.
- `not_declared` — the starter does not declare a runtime
  scope. Allowed only on `community` or `local_only` sources
  and MUST carry a `toolchain_detect` bootstrap item.

Rules (frozen):

1. Every template or prebuild card names exactly one
   `runtime_and_toolchain_scope`. A row that emits a card
   without the scope is non-conforming.
2. `remote_image_required` and `managed_cloud_required` MUST
   pair with a `remote_target_descriptor_ref` on the upstream
   `recent_work_entry_record` / `project_entry_action_record`.
3. `mixed_local_and_remote` MUST render a local / remote split
   inline on the `environment_starter_summary` before commit.

### 3.5 `template_lifecycle_class`

The distinguishing lane a card routes into. The set is closed:

- `resume_live_workspace` — reattach to a live session; no
  materialisation, no setup re-run, no trust widening.
  `entry_verb = resume`; `resulting_mode = resume_live_session`.
- `start_from_snapshot` — materialise a prebuild / warm-start
  snapshot from a point-in-time artifact. `entry_verb =
  start_from_snapshot`; `resulting_mode =
  open_prebuild_with_setup_actions` or `open_prebuild_minimal`.
- `clone_fresh_repository` — clone a repository without any
  starter attached. `entry_verb = clone`; `resulting_mode =
  clone_then_review` or `clone_then_open`.
- `template_based_start` — materialise a template bundle (one
  or more files plus optional setup actions). `entry_verb =
  start_from_snapshot` (template is a snapshot class);
  `resulting_mode = open_prebuild_with_setup_actions` or
  `open_prebuild_minimal`.
- `prebuild_warm_start` — attach a prebuild to an already-
  cloned repository root so the initial bootstrap items
  short-circuit. `entry_verb = start_from_snapshot`;
  `resulting_mode = open_prebuild_minimal`.
- `open_without_starter` — the bypass lane. `entry_verb =
  open`, `clone`, or `import`; starter is explicitly not
  attached.

Rules (frozen):

1. Every gallery / picker / resume-live card names exactly one
   `template_lifecycle_class`. A card that collapses two
   lanes — for example, `resume_live_workspace` and
   `start_from_snapshot` — into a single `Open` button is
   non-conforming.
2. `resume_live_workspace` MUST NOT re-run setup actions,
   re-install extensions, re-fetch dependencies, or re-grant
   trust; its sole bootstrap-item class is
   `remote_attach_handshake`. A resume-live card that
   enqueues `package_restore`, `generator_run`, or
   `devcontainer_bootstrap` is non-conforming.
3. `open_without_starter` MUST route through the plain
   `primary_action.open_folder`, `primary_action.open_workspace`,
   or `primary_action.clone_repository` verbs exactly as the
   Start Center exposes them. It MUST NOT mint a separate
   `open_with_no_starter_button` verb.

### 3.6 `declared_freshness_class`

Re-exported from the source-acquisition seed §1.4 without
modification; listed here so downstream tools that read this
contract alone still discover the set. The set is closed:

- `live_origin`
- `mirror_fresh`
- `mirror_lagged`
- `mirror_stale`
- `offline_snapshot`
- `signed_offline_bundle`
- `unknown_freshness`

Rules (frozen):

1. Every template / prebuild card and every resume-live card
   renders the declared freshness chip. Hiding the chip is
   non-conforming; `mirror_lagged` and `mirror_stale` MUST
   render distinctly from `mirror_fresh`.
2. `unknown_freshness` is allowed only when the canonical
   owner has not been re-reached for verification. A surface
   that silently rewrites `unknown_freshness` to `live_origin`
   is non-conforming.

### 3.7 `starter_setup_cost_class`

Expected time-and-connectivity posture of the starter. The set
is closed:

- `no_setup` — the starter is effectively a copy; the
  resulting workspace opens without any bootstrap queue item
  beyond naming the locator. Pairs with `resulting_mode =
  open_prebuild_minimal`.
- `light_local_setup` — seconds; no network; local-only
  bootstrap items (`toolchain_detect`, `settings_materialize`,
  `profile_materialize`).
- `network_package_restore` — seconds to minutes; a registry
  or mirror is contacted (`package_restore`,
  `docs_import`, `index_warm_up`).
- `devcontainer_bootstrap` — minutes; a devcontainer is
  built or attached (`devcontainer_attach`, optional
  `toolchain_install`).
- `container_bootstrap` — minutes; a non-devcontainer
  container is attached (`container_runtime`).
- `managed_remote_provisioning` — minutes; a managed-cloud
  workspace is provisioned (`credential_provisioning`,
  `remote_attach_handshake`, managed-workspace lifecycle).
- `prebuild_attach_warm` — fast; a pre-warmed image / layer
  cache is attached (`prebuild_attach`) with no cold
  `package_restore`.
- `time_unknown_network_unknown` — cost could not be
  estimated. Allowed only when the checkout plan's
  `acquisition_posture` is `unknown` and the card names
  `starter_setup_cost_class = time_unknown_network_unknown`
  explicitly.

Rules (frozen):

1. Every gallery card and every preflight summary names
   exactly one `starter_setup_cost_class`. A card that
   renders "quick start" without a cost class is non-
   conforming.
2. `network_package_restore`, `devcontainer_bootstrap`,
   `container_bootstrap`, and `managed_remote_provisioning`
   MUST expose an explicit `bypass_path_id` from §3.9 on the
   same card.
3. `prebuild_attach_warm` MUST carry an attributable
   evidence ref — a `previous_bootstrap_run_ref` or a
   `mirror_freshness_ref` — so "warm" is demonstrable, not
   asserted.

### 3.8 `template_availability_narrowing_class`

Why a row or a set of rows is narrowed. The set is closed:

- `no_narrowing`
- `policy_narrowed_fleet`
- `policy_narrowed_admin`
- `policy_narrowed_workspace_trust`
- `connected_provider_policy`
- `mirror_only_cached_subset`
- `offline_no_bundle`
- `signature_review_required`
- `uncertified_excluded`
- `target_runtime_unavailable`

Rules (frozen):

1. A gallery that renders fewer rows than its canonical
   source advertises MUST render a `starter_policy_notice`
   (§7) naming the narrowing class. Silent narrowing is
   non-conforming.
2. `policy_narrowed_*` classes MUST cite a typed
   `policy_narrowing_class` ref from the source-acquisition
   seed §2.9; raw "policy" strings are non-conforming.
3. `mirror_only_cached_subset` MUST cite the mirror-freshness
   ref from the source-acquisition seed §2.5; a notice that
   says "some templates unavailable offline" without the
   freshness ref is non-conforming.
4. `signature_review_required` MUST cite the signer-continuity
   ref and the review ticket ref (source-acquisition seed
   §2.6).

### 3.9 `bypass_path_id`

Closed set of the always-available `open-without-starter`
routes the gallery / picker / summary / resume-live card MUST
advertise alongside the primary starter action. Each bypass
path resolves to an entry verb already frozen in entry-restore
object model §1.1 — this set never mints a new verb:

- `bypass.open_folder_without_starter` — entry_verb `open`,
  target_kind `local_folder` or `local_repo_root`.
- `bypass.open_workspace_without_starter` — entry_verb
  `open`, target_kind `workspace_manifest`.
- `bypass.clone_repository_without_starter` — entry_verb
  `clone`, target_kind `remote_repository`, resulting_mode
  `clone_then_review`.
- `bypass.create_empty_workspace` — entry_verb `open`,
  target_kind `workspace_manifest`, destination_disposition
  `write_to_user_destination`, `resulting_mode =
  workspace_candidate`.
- `bypass.open_prebuild_minimal` — entry_verb
  `start_from_snapshot`, resulting_mode
  `open_prebuild_minimal`; snapshot is attached but no
  setup actions run. Paired with `set_up_later`.
- `bypass.set_up_later` — entry_verb unchanged; setup
  actions on the envelope are deferred; the card routes to
  `next_step_decision_hook = set_up_later`.
- `bypass.continue_without_starter` — post-create or mid-
  setup bypass; the handoff summary (§6) offers this to
  close out a failed or partial setup without running
  further actions.

Rules (frozen):

1. Every gallery row, every picker row, every preflight
   summary sheet, and every resume-live card MUST advertise
   at least one `bypass_path_id`. A surface that hides all
   bypass paths is non-conforming.
2. Bypass-path affordances MUST render at the same weight as
   the primary starter action. A bypass path hidden behind a
   secondary `...` menu, or rendered at smaller type, or
   placed below the primary-action fold on the preflight
   summary, is non-conforming.
3. `bypass.create_empty_workspace` MUST resolve through the
   plain `primary_action.open_workspace` flow. It MUST NOT
   run a scaffold, a generator, a package restore, or an
   extension install.
4. `bypass.open_prebuild_minimal` and `bypass.set_up_later`
   MUST preserve all bootstrap items as `pending /
   awaiting_user_action` rather than `skipped`; the user may
   run them later from the post-create handoff or the
   command palette.

### 3.10 `template_health_signal_class`

Per-health-check provenance signal. The set is closed (UI/UX
§17.7):

- `live` — the check was just run against the canonical
  source.
- `cached` — the check is served from cache; the row MUST
  expose a last-validated timestamp class (`minutes_ago`,
  `hours_ago`, `days_ago`, `older`).
- `policy_evaluated` — the check was resolved against an
  admin / fleet policy rather than a runtime probe.
- `not_checked` — the check was not attempted on this
  surface.
- `stale_or_invalid` — the cache expired or the probe
  returned an invalid response; the check is flagged.

Rules (frozen):

1. Every `template_health_row_record` names exactly one
   `template_health_signal_class`. A row that shows "healthy"
   without a signal class is non-conforming.
2. `cached` MUST expose the timestamp class in the row
   subtitle or chip; hiding the cache age is non-conforming.
3. `stale_or_invalid` MUST NOT block the `Open without
   starter` bypass path. The row flags the issue; the bypass
   remains available at the same weight.

### 3.11 `template_health_check_class`

Closed set of health-check categories:

- `toolchain_compatibility`
- `os_runtime_compatibility`
- `template_freshness`
- `policy_restriction`
- `signature_state`
- `network_reachability`
- `dependency_manifest`
- `extension_availability`

Rules (frozen):

1. Every `template_health_row_record` names exactly one
   `template_health_check_class`. Free-form check strings are
   non-conforming.
2. `policy_restriction` MUST cite a typed
   `policy_narrowing_class` from the source-acquisition seed
   §2.9 so the user can see which policy blocked the check.

### 3.12 `generation_preflight_axis`

Closed set of axes the environment-starter summary discloses
before commit. The first six match UI/UX spec §6.9 verbatim;
the remaining six split out the typed previews the side-effect
envelope already carries:

- `identity_and_provenance`
- `expected_result`
- `setup_actions`
- `time_and_connectivity_expectation`
- `cleanup_or_rollback_path`
- `bypass_path`
- `extension_install_preview`
- `task_run_preview`
- `secret_handle_preview`
- `trust_grant_preview`
- `remote_provisioning_preview`
- `port_binding_preview`

Rules (frozen):

1. Every `environment_starter_summary_record` renders the
   first six axes. A summary that elides any of them is non-
   conforming.
2. The six typed previews are required only when the
   starter's `side_effect_envelope` declares them; a summary
   that names `extension_install_preview` without naming the
   specific extension ids is non-conforming.
3. `trust_grant_preview` MUST NOT imply the user has already
   granted trust; it previews what would happen if they do.
   A summary that pre-grants trust on render is non-
   conforming.

### 3.13 `post_create_handoff_axis`

Closed set of axes the post-create handoff summary renders:

- `created_workspace_identity`
- `pending_setup_actions_list`
- `trust_state_after_create`
- `run_now_actions`
- `run_later_actions`
- `review_files_actions`
- `rollback_or_delete_action`
- `local_only_continuation_path`
- `post_create_cleanup_if_failed`
- `setup_outcome_summary` — closed subset
  (`all_actions_succeeded`, `partial_success`, `failed`,
  `bypass_taken`, `deferred`).

Rules (frozen):

1. Every `post_create_handoff_summary_record` names exactly
   one `setup_outcome_summary` and renders every axis whose
   slot the starter's envelope reserved.
2. `partial_success` and `failed` MUST render
   `post_create_cleanup_if_failed` verbatim, naming what
   succeeded, what was skipped, what was partially applied,
   and what cleanup was performed. A handoff that shows
   "Setup failed" without the itemised breakdown is non-
   conforming.
3. `local_only_continuation_path` MUST be present on every
   handoff whose `runtime_and_toolchain_scope` is anything
   other than `managed_cloud_required`; users who just
   materialised files locally always retain a local-safe
   path forward.

### 3.14 `policy_notice_class`

Closed set of in-place notices explaining availability
narrowing:

- `fleet_policy_notice`
- `admin_policy_notice`
- `workspace_trust_policy_notice`
- `connected_provider_policy_notice`
- `signature_policy_notice`
- `mirror_or_airgap_policy_notice`
- `community_source_notice`
- `uncertified_source_notice`
- `target_runtime_unavailable_notice`
- `managed_workspace_policy_notice`

Rules (frozen):

1. Every `starter_policy_notice_record` names exactly one
   `policy_notice_class` and cites at least one resolution
   hook from the entry-restore §1.7 set or the source-
   acquisition seed §2.10 set.
2. A notice MUST render in-place on the affected surface
   (gallery, picker, summary, resume-live card), not only
   as a deferred toast; deferred toasts that arrive after
   the user already clicked a row are non-conforming.
3. The notice MUST disclose **why** availability is
   narrowed; strings like "Some templates are unavailable"
   without the narrowing class and the resolution hook are
   non-conforming.

## 4. Gallery / picker surface record

Every template gallery, prebuild picker, or resume-live card
collection emits exactly one
`template_and_prebuild_gallery_record`.

### 4.1 Required fields

- `record_kind = template_and_prebuild_gallery_record`.
- `surface_id` (opaque).
- `template_and_prebuild_surface_family` (§3.1).
- `startup_state_ref` — one `startup_state` token (entry-
  restore truth audit §6) the gallery is resolving against
  (typically `startup_state:first_run`,
  `startup_state:warming_startup`,
  `startup_state:offline_startup`, or
  `startup_state:unsupported_startup`).
- `privacy_reduction_mode` — re-export from the start-center
  contract §3.5.
- `template_card_refs[]` — ordered template rows the gallery
  renders.
- `prebuild_card_refs[]` — ordered prebuild rows when the
  family is `prebuild_picker`.
- `resume_live_card_refs[]` — ordered resume-live cards when
  the family is `resume_live_card`.
- `bypass_path_ids[]` — at least one
  `bypass_path_id` (§3.9); MUST include
  `bypass.open_folder_without_starter` on
  `template_gallery` and `prebuild_picker`.
- `policy_notice_refs[]` — every `starter_policy_notice_record`
  currently applied to this gallery.
- `health_row_refs[]` — when the family is
  `template_gallery` and the gallery renders an inline
  health strip, each strip cell points at a
  `template_health_row_record`.
- `equal_weight_bypass_invariant = true` — required; a
  gallery that emits `false` is non-conforming and denies
  with `bypass_path_narrowed_below_equal_weight`.
- `keyboard_reachability_posture` — `all_primary_focusable`
  is the only conforming value. Every card's primary action
  and its bypass action MUST be keyboard-reachable.
- `minted_at` — monotonic timestamp.

### 4.2 Layout invariants

1. **Bypass alongside primary.** The bypass-path affordance
   renders at the same zone, same weight, and same focus
   order as the primary starter action. A gallery whose
   bypass is one screen below the starter grid is non-
   conforming.
2. **Narrowing explained in-place.** Every narrowed row
   renders with a `policy_notice_class` chip or inline
   banner naming the class; see §3.14.
3. **Freshness chip mandatory.** Every card renders the
   `declared_freshness_class` chip (§3.6). Hiding the chip
   is non-conforming.
4. **Marketplace-first forbidden.** The gallery MUST NOT
   promote a marketplace "install extensions first" row
   above the starter rows; this composes with start-center
   contract §3.2 rule 3.

## 5. Template / prebuild card records

### 5.1 `template_card_record`

Required fields:

- `record_kind = template_card_record`.
- `template_card_id` (opaque).
- `template_source_class` (§3.2), plus
  `mirror_cached_of` when the source class is
  `mirror_cached`.
- `support_class` (§3.3).
- `runtime_and_toolchain_scope` (§3.4).
- `template_lifecycle_class` (§3.5) — MUST be
  `template_based_start` or `open_without_starter` on a
  template-gallery row.
- `declared_freshness_class` (§3.6), plus typed
  `last_validated_timestamp_class` when the freshness is
  `mirror_fresh`, `mirror_lagged`, `mirror_stale`, or
  `unknown_freshness`.
- `signer_continuity_class` — from source-acquisition seed
  §1.5.
- `starter_setup_cost_class` (§3.7).
- `availability_narrowing_class` (§3.8).
- `bypass_path_ids[]` (§3.9) — at least one.
- `side_effect_envelope_preview_ref` — refers to the
  `environment_starter_summary_record` this card would
  commit into. MUST be resolvable before commit.
- `policy_notice_ref` — optional ref when a
  `starter_policy_notice_record` applies to this row.
- `disabled_reason_code` — optional; drawn from the
  command-descriptor `disabled_reason_code` vocabulary
  when the row is visible-but-disabled
  (`policy_narrowed_fleet`, `mirror_only_cached_subset`,
  `signature_review_required`, `uncertified_excluded`,
  `target_runtime_unavailable`, `network_unreachable`).
- `keyboard_reachable = true`.

Rules (frozen):

1. A `template_card_record` whose `support_class` is
   `legacy_deprecated` or `unsupported` MUST render the
   deprecation / unsupported notice on the card, not only
   in the detail sheet.
2. A card whose `starter_setup_cost_class` is
   `network_package_restore`,
   `devcontainer_bootstrap`, `container_bootstrap`, or
   `managed_remote_provisioning` MUST expose at least one
   bypass path (§3.9 rule 2) and MUST NOT advertise
   "quick start" on its primary label.
3. A card that enables commit without a resolvable
   `side_effect_envelope_preview_ref` is non-conforming
   (setup without a preview is forbidden).

### 5.2 `prebuild_card_record`

Required fields (all of §5.1 plus):

- `template_lifecycle_class` — MUST be
  `prebuild_warm_start` or `start_from_snapshot` on a
  prebuild-picker row.
- `prebuild_artifact_descriptor_ref` — the
  `artifact_descriptor` (entry-restore object model §1.2)
  for the prebuild snapshot.
- `prebuild_warm_evidence_refs[]` — at least one of
  `previous_bootstrap_run_ref`, `mirror_freshness_ref`, or
  a signed-offline-bundle ref so the "warm" claim is
  attributable.

Rules (frozen):

1. A `prebuild_card_record` whose
   `declared_freshness_class` is `unknown_freshness` MUST
   NOT render `starter_setup_cost_class = prebuild_attach_warm`.
   "Warm" without evidence is non-conforming.
2. Attaching a prebuild MUST NOT silently upgrade
   `admission_class` from `trust_review_required` to
   `admitted`. Trust review remains a separate, explicit
   step.

## 6. Environment-starter summary record

Every preflight summary before a template or prebuild commits
emits exactly one `environment_starter_summary_record`.

### 6.1 Required fields

- `record_kind = environment_starter_summary_record`.
- `summary_id` (opaque).
- `template_card_ref` or `prebuild_card_ref` — the source
  card.
- `entry_verb_on_commit` — from entry-restore §1.1 (`open`,
  `clone`, `start_from_snapshot`, `resume`).
- `resulting_mode_on_commit` — from §1.3.
- `generation_preflight_axes_rendered[]` — every axis
  from §3.12 the summary renders.
- `setup_actions_list[]` — one `bootstrap_queue_item_record`
  ref per declared setup action. Free-form action strings
  are non-conforming.
- `extension_install_preview[]` — required when
  `generation_preflight_axes_rendered` includes
  `extension_install_preview`. Each entry names the
  extension id, publisher class, signature state, and
  permission class; no aggregate "will install extensions"
  string.
- `secret_handle_preview[]` — required when
  `generation_preflight_axes_rendered` includes
  `secret_handle_preview`. Each entry names the handle
  class (`brokered_handle`), the reason, and whether work
  continues without it.
- `trust_grant_preview` — required when
  `generation_preflight_axes_rendered` includes
  `trust_grant_preview`. Names the `trust_state_before` and
  the `trust_state_after_grant` plus the
  `admission_class_after_grant`.
- `port_binding_preview[]` — required when
  `generation_preflight_axes_rendered` includes
  `port_binding_preview`. Each entry names the port class,
  listener kind, and whether the binding is local-only or
  exposed.
- `remote_provisioning_preview` — required when
  `runtime_and_toolchain_scope` is `remote_image_required`
  or `managed_cloud_required`. Names the target class,
  region or tenant, persistence class, and expected cost /
  connectivity class.
- `time_and_connectivity_expectation` —
  `starter_setup_cost_class` (§3.7) plus an expected-time
  class (`seconds`, `seconds_to_minutes`, `minutes`,
  `minutes_to_tens_of_minutes`, `unknown`).
- `cleanup_or_rollback_path` — names the cleanup posture
  (`discard_staging_only`, `discard_with_compensation`,
  `discard_unavailable_manual_only`) and the recovery
  class from the entry-restore object model §4.
- `bypass_path_ids[]` (§3.9) — at least one; MUST be
  same-weight.
- `keyboard_reachable = true`.

### 6.2 Commit rules

1. The summary MUST render before the commit click. A
   surface that commits setup on the first click and
   renders the summary only afterwards is non-conforming.
2. Commit resolves to a `project_entry_action_record`
   (entry-restore §1). The summary MUST NOT mint its own
   entry record.
3. If the summary is navigated away from without commit,
   no `side_effect_envelope` item runs. The surface remains
   in the pre-commit `trust_stage` (`pre_fetch_inspection`
   or `post_fetch_content_review`).

## 7. Resume-live workspace card record

Every workspace-switcher entry, Start Center row, or recent-
work entry whose target resolves to a live session (managed-
cloud, SSH, devcontainer, container, remote tunnel) emits
exactly one `resume_live_workspace_card_record`.

### 7.1 Required fields

- `record_kind = resume_live_workspace_card_record`.
- `resume_live_card_id` (opaque).
- `target_kind` — from entry-restore §1.2 (one of
  `managed_cloud_workspace`, `ssh_workspace`,
  `container_workspace`, `devcontainer_workspace`,
  `remote_repository`).
- `entry_verb_on_commit = resume`.
- `resulting_mode_on_commit = resume_live_session`.
- `attach_authority_class` — from source-acquisition seed
  live-session descriptor (`authority_live`,
  `authority_expiring`, `authority_expired`,
  `authority_unknown`).
- `managed_workspace_state_ref` — when
  `target_kind = managed_cloud_workspace`, the current
  managed-workspace state (UI/UX §16.11:
  `requested`, `provisioning`, `ready`, `suspended`,
  `expiring`, `rebuild_required`, `deleted_unavailable`).
- `declared_freshness_class` (§3.6) — on the live-session
  metadata, not on the underlying source.
- `side_effect_envelope_on_resume` — MUST be empty of
  mutating actions. A resume-live card that carries
  `package_restore`, `generator_run`,
  `devcontainer_bootstrap`, or `extension_activation` is
  non-conforming.
- `alternative_lanes[]` — advertises the four distinct
  lanes from §3.5:
  - `resume_live_workspace` (the default on this card)
  - `start_from_snapshot` (reopen from the latest
    prebuild / snapshot instead)
  - `clone_fresh_repository` (clone the source fresh,
    no starter)
  - `open_without_starter` (bypass)
- `bypass_path_ids[]` — at least
  `bypass.open_folder_without_starter` or
  `bypass.clone_repository_without_starter`, and
  `bypass.continue_without_starter` when the live session
  is `rebuild_required`.
- `policy_notice_refs[]` — in-place notices when the
  live session is blocked by policy.
- `keyboard_reachable = true`.

### 7.2 Lane-separation rules

1. A resume-live card that merges `resume`, `reopen from
   snapshot`, `clone fresh`, and `open without starter`
   into a single `Open` button is non-conforming. The
   four `alternative_lanes` render as distinct
   affordances.
2. A resume-live card whose `attach_authority_class` is
   `authority_expired` MUST route through `reauth_required`
   (entry-restore §1.7) before committing; it MUST NOT
   silently refresh authority on click.
3. `managed_workspace_state = rebuild_required` MUST
   render a
   `recreate-from-template-path` affordance resolving to
   `regenerate_from_canonical_source` (entry-restore §4).
   `Resume` is disabled with `disabled_reason_code =
   rebuild_required`.
4. **Warm-start honesty is lane-backed.** When a surface renders
   the four `alternative_lanes`, it MUST also satisfy the lane-level
   freshness + revalidation contract in
   `artifacts/entry/warm_start_chooser_contract.md` and MUST be able
   to export a `warm_start_chooser_set_record` (and decision/outcome
   chain) per `schemas/entry/freshness_revalidation.schema.json` so
   liveness, age, pending updates, and revalidation requirements do
   not disappear after the click.

## 8. Post-create handoff summary record

Every completion of a starter-driven flow emits exactly one
`post_create_handoff_summary_record`.

### 8.1 Required fields

- `record_kind = post_create_handoff_summary_record`.
- `handoff_id` (opaque).
- `source_summary_ref` — the
  `environment_starter_summary_record` the user committed.
- `created_workspace_ref` — reference to the resulting
  `recent_work_entry_record`; `null` when the starter
  produced `setup_outcome_summary = bypass_taken` without
  materialising a workspace.
- `setup_outcome_summary` — from §3.13 closed subset.
- `post_create_handoff_axes_rendered[]` — every axis
  from §3.13 the handoff renders.
- `succeeded_actions[]`, `skipped_actions[]`,
  `partially_applied_actions[]`, `failed_actions[]` —
  each a list of `bootstrap_queue_item_record` refs.
  Aggregate counts are allowed but never substitute for
  the itemised lists.
- `cleanup_performed[]` — list of cleanup refs actually
  executed (required when `setup_outcome_summary` is
  `partial_success` or `failed`).
- `trust_state_after_create` — from ADR-0001.
- `run_now_action_refs[]`, `run_later_action_refs[]`,
  `review_files_action_refs[]`,
  `rollback_or_delete_action_ref`,
  `local_only_continuation_path_ref`.
- `keyboard_reachable = true`.

### 8.2 Rules

1. A `post_create_handoff_summary_record` whose
   `setup_outcome_summary` is `failed` MUST render the
   four outcome lists — `succeeded_actions`,
   `skipped_actions`, `partially_applied_actions`,
   `failed_actions` — and `cleanup_performed`.
   Collapsing any of these into "Setup did not finish" is
   non-conforming.
2. `rollback_or_delete_action_ref` MUST resolve to a
   recovery class from entry-restore §4. Free-form
   "delete everything" verbs are non-conforming.
3. `local_only_continuation_path_ref` is required unless
   the starter's `runtime_and_toolchain_scope` is
   `managed_cloud_required`.

## 9. Template-health row record

Each cell in a template-health report emits one
`template_health_row_record`.

### 9.1 Required fields

- `record_kind = template_health_row_record`.
- `template_health_row_id` (opaque).
- `template_card_ref` — the card the row reports on.
- `template_health_check_class` (§3.11).
- `template_health_signal_class` (§3.10).
- `last_validated_timestamp_class` — required when the
  signal is `cached`.
- `result` — closed subset (`pass`, `warn`, `fail`,
  `not_applicable`).
- `issue_summary` — ≤ 256 graphemes, redaction-aware; null
  on `pass`.
- `bypass_path_still_available = true` — required; a row
  that emits `false` is non-conforming (the bypass path
  stays available regardless of health).
- `keyboard_reachable = true`.

### 9.2 Rules

1. A row whose `result` is `fail` or `warn` MUST NOT
   collapse the underlying `template_health_check_class`
   into a generic "health issue" string; the specific
   check class renders verbatim.
2. A row whose `template_health_signal_class` is
   `stale_or_invalid` MUST pair with a typed repair hook
   (`refresh_mirror_then_retry`,
   `switch_to_live_origin_then_retry`, or
   `retry_after_policy_refresh`).

## 10. Starter policy notice record

Every in-place disclosure that narrows availability emits one
`starter_policy_notice_record`.

### 10.1 Required fields

- `record_kind = starter_policy_notice_record`.
- `notice_id` (opaque).
- `policy_notice_class` (§3.14).
- `availability_narrowing_class` (§3.8).
- `policy_narrowing_class_refs[]` — from source-acquisition
  seed §2.9 when a policy narrowed the set.
- `affected_template_card_refs[]`,
  `affected_prebuild_card_refs[]` — the rows the notice
  narrows. May be `["*"]` to mean the whole gallery.
- `resolution_hook_refs[]` — at least one, drawn from
  entry-restore §1.7 and source-acquisition seed §2.10.
- `disclosed_in_zone` — `disclosure_band` above
  `primary_work_resume` when narrowing is blocking;
  `inline_row_chrome` when narrowing is per-row; see
  start-center contract §3.2 for zone rules.
- `keyboard_reachable = true`.

### 10.2 Rules

1. A notice MUST render **before** the user clicks a
   narrowed row. A notice that appears only in a toast
   after the click is non-conforming.
2. A notice MUST name the narrowing class; strings like
   "Unavailable" without the class are non-conforming.

## 11. Surface rules (cross-cutting)

1. **Equal-weight bypass.** On every gallery, picker,
   preflight summary, and resume-live card, at least one
   `bypass_path_id` renders at the same zone, same weight,
   and same focus order as the primary starter action.
2. **No silent trust.** Selecting a template, prebuild, or
   resume-live card MUST NOT widen `trust_state` from
   `restricted` or `pending_evaluation` to `trusted`.
   Trust review remains an explicit, typed step routed
   through `review_trust_and_open` or
   `continue_in_restricted_mode`.
3. **No silent extension install.** A starter's
   `extension_install_preview` MUST render before commit.
   A surface that installs extensions on commit without
   the preview is non-conforming.
4. **No silent setup-task run.** A starter's
   `setup_actions_list` MUST render before commit as
   typed `bootstrap_queue_item_record` refs. Opaque
   "prepare environment" actions are non-conforming.
5. **Same-weight non-starter open.** Every surface keeps
   at least one non-template, non-prebuild entry path of
   equal weight: `bypass.open_folder_without_starter`,
   `bypass.open_workspace_without_starter`,
   `bypass.clone_repository_without_starter`, or
   `bypass.create_empty_workspace`.
6. **Freshness never hidden.** Every template, prebuild,
   and resume-live card renders
   `declared_freshness_class` verbatim;
   `mirror_lagged` / `mirror_stale` / `unknown_freshness`
   render distinctly.
7. **Source class never hidden.** Every row names its
   `template_source_class`; `community` and `uncertified`
   MUST carry a signer-continuity class.
8. **Narrowing never silent.** Availability narrowing
   always renders a typed `starter_policy_notice_record`
   in-place.
9. **Failed-setup truthful.** Every failed or partial
   starter renders the four outcome lists and the cleanup
   list; "Setup did not finish" is non-conforming.
10. **Four-lane separation.** `resume_live_workspace`,
    `start_from_snapshot`, `clone_fresh_repository`, and
    `open_without_starter` never flatten into one `Open`
    button; §3.5 rule 1.
11. **Health never blocks bypass.** Template-health rows,
    including `stale_or_invalid`, never block the
    `Open without starter` bypass path.
12. **No redefinition upstream.** This contract re-exports
    by reference; a surface that mints parallel entry-
    verb, target-kind, resulting-mode, locator-class,
    trust-stage, bootstrap-item, or recovery-class values
    is non-conforming.

## 12. Seeded-state matrix

How each `startup_state` token interacts with template /
prebuild / resume-live surfaces. Rows compose with the
start-center contract §11 matrix; they do not replace it.

| `startup_state` | Gallery rendered | Prebuild picker | Resume-live card | Required bypass paths | Required notices |
|---|---|---|---|---|---|
| `first_run` | Allowed as `secondary_entry` row only; primary open / clone / import / restore / add_root still render first. | Not rendered (no prior workspace). | Not rendered. | `bypass.open_folder_without_starter`, `bypass.create_empty_workspace`. | `community_source_notice` on community rows if any are rendered. |
| `warming_startup` | Allowed; may render `cached` health signals. | Allowed for recent-work rows with prebuild evidence. | Allowed for live managed / remote workspaces. | Same as `first_run` plus `bypass.set_up_later`. | Optional `mirror_only_cached_subset` notice if warming without network. |
| `offline_startup` | Rendered with `declared_freshness_class` ∈ {`mirror_stale`, `offline_snapshot`, `signed_offline_bundle`, `unknown_freshness`}. | Rendered only when a signed-offline-bundle or mirror-cached prebuild exists. | `resume_live` is disabled with `disabled_reason_code = network_unreachable`; `alternative_lanes` still render the other three. | All four required: `bypass.open_folder_without_starter`, `bypass.open_workspace_without_starter`, `bypass.clone_repository_without_starter`, `bypass.set_up_later`. | `mirror_or_airgap_policy_notice` or `mirror_only_cached_subset` notice required when gallery is narrowed. |
| `unsupported_startup` | May be hidden under `hide_all_except_open_and_clone` (start-center §3.5). | Hidden. | Hidden. | `bypass.open_folder_without_starter`, `bypass.clone_repository_without_starter` only. | `fleet_policy_notice` or `admin_policy_notice` required, blocking above `primary_work_resume`. |
| `reopen_with_pending_restore` | Available but subordinate to restore card. | Available. | Available if the pending session is resume-live. | Same as `first_run`. | None by default. |
| `open_without_restore` | Available. | Available. | Available. | Same as `first_run`. | None. |

## 13. Worked examples

Each example has a companion fixture under
[`/fixtures/ux/template_and_prebuild_states/`](../../fixtures/ux/template_and_prebuild_states/).

### 13.1 First-party template with local-only setup

Gallery shows a first-party template (`template_source_class
= first_party`, `support_class = officially_supported`,
`runtime_and_toolchain_scope = local_only`,
`declared_freshness_class = live_origin`,
`signer_continuity_class = continuous_with_previous_acquisition`,
`starter_setup_cost_class = light_local_setup`). Commit
runs two local bootstrap items (`toolchain_detect`,
`settings_materialize`). Bypass path
`bypass.open_folder_without_starter` renders at equal
weight. See
[`template_gallery_first_party_local_only.json`](../../fixtures/ux/template_and_prebuild_states/template_gallery_first_party_local_only.json).

### 13.2 First-party template with devcontainer setup

Gallery shows a first-party template with a devcontainer
requirement (`runtime_and_toolchain_scope =
local_with_devcontainer`, `starter_setup_cost_class =
devcontainer_bootstrap`). Preflight summary renders all six
§3.12 primary axes plus `extension_install_preview`,
`port_binding_preview`, and `trust_grant_preview`. Bypass
path `bypass.open_prebuild_minimal` and
`bypass.open_folder_without_starter` both render. See
[`environment_starter_summary_devcontainer.json`](../../fixtures/ux/template_and_prebuild_states/environment_starter_summary_devcontainer.json).

### 13.3 Community template with signature review required

Gallery shows a community template
(`template_source_class = community`,
`signer_continuity_class = signer_changed_review_required`,
`availability_narrowing_class = signature_review_required`,
`disabled_reason_code = signature_review_required`). An in-
place `starter_policy_notice` carries
`policy_notice_class = community_source_notice` and
resolution hook `review_signer_change`. Bypass path
`bypass.clone_repository_without_starter` remains at equal
weight. See
[`template_card_community_signature_review.json`](../../fixtures/ux/template_and_prebuild_states/template_card_community_signature_review.json).

### 13.4 Stale / issue-flagged prebuild with bypass intact

Prebuild picker shows a warm-start row whose declared
freshness is `mirror_stale` and whose template-health
report returns `result = warn` on
`template_freshness` with
`template_health_signal_class = stale_or_invalid`. The
picker flags the issue in-place (chip + inline banner) and
does not hide the row. `bypass.open_folder_without_starter`
and `bypass.set_up_later` both render at equal weight;
`bypass_path_still_available = true` is asserted on every
health row. See
[`prebuild_picker_stale_warmstart.json`](../../fixtures/ux/template_and_prebuild_states/prebuild_picker_stale_warmstart.json).

### 13.5 Fleet-policy-narrowed gallery

An admin-fleet machine whose fleet policy excludes
community sources and signature-review-required templates.
The gallery renders the surviving rows and a
`starter_policy_notice` with
`policy_notice_class = fleet_policy_notice`,
`availability_narrowing_class = policy_narrowed_fleet`,
`policy_narrowing_class_refs = [fleet_policy]`, and
resolution hook `continue_in_restricted_mode`. No rows are
hidden without the notice. See
[`template_gallery_fleet_policy_narrowed.json`](../../fixtures/ux/template_and_prebuild_states/template_gallery_fleet_policy_narrowed.json).

### 13.6 Mirror-only cached subset, offline startup

Offline startup where the local mirror carries a subset of
the canonical gallery. The gallery renders only the
mirror-cached rows
(`template_source_class = mirror_cached`,
`mirror_cached_of = first_party`,
`declared_freshness_class = mirror_stale`), with a
`starter_policy_notice` of class
`mirror_or_airgap_policy_notice` and
`availability_narrowing_class =
mirror_only_cached_subset`. All four bypass paths render:
`bypass.open_folder_without_starter`,
`bypass.open_workspace_without_starter`,
`bypass.clone_repository_without_starter`,
`bypass.set_up_later`. See
[`template_gallery_mirror_only_offline.json`](../../fixtures/ux/template_and_prebuild_states/template_gallery_mirror_only_offline.json).

### 13.7 Resume-live managed-cloud card with four lanes

Workspace-switcher card for a managed-cloud workspace whose
state is `ready` and whose `attach_authority_class` is
`authority_live`. The card renders all four
`alternative_lanes`: `resume_live_workspace` (default),
`start_from_snapshot` (reopen from latest prebuild),
`clone_fresh_repository` (clone the source fresh), and
`open_without_starter` (bypass). No mutating bootstrap
items are enqueued on the resume-live lane; only
`remote_attach_handshake`. See
[`resume_live_managed_cloud_card.json`](../../fixtures/ux/template_and_prebuild_states/resume_live_managed_cloud_card.json).

### 13.8 Post-create handoff with partial setup failure

After a starter that enqueued five bootstrap items,
`setup_outcome_summary = partial_success`:
`succeeded_actions` = [`toolchain_detect`,
`package_restore`], `skipped_actions` = [`ai_context_warm_up`
(skip_reason `offline_bundle_excludes`)],
`partially_applied_actions` = [`extension_restore`],
`failed_actions` = [`devcontainer_attach`]. The handoff
renders `cleanup_performed = [discard_staging_only on
devcontainer_attach layer]`, `run_later_action_refs`
listing the three unsuccessful items, and a
`local_only_continuation_path_ref`. See
[`post_create_handoff_partial_setup_failure.json`](../../fixtures/ux/template_and_prebuild_states/post_create_handoff_partial_setup_failure.json).

### 13.9 Admin-policy-narrowed gallery

An organization admin policy narrows the canonical gallery on this
device. The gallery renders the surviving rows plus a
`starter_policy_notice_record` with
`policy_notice_class = admin_policy_notice` and
`availability_narrowing_class = policy_narrowed_admin`. The notice cites
typed `policy_narrowing_class_refs = [admin_policy]` and offers the
resolution hook `continue_in_restricted_mode`. Bypass paths remain at
equal weight. See
[`template_gallery_admin_policy_narrowed.json`](../../fixtures/ux/template_and_prebuild_states/template_gallery_admin_policy_narrowed.json).

### 13.10 Offline startup with no bundle

Offline startup where no signed offline bundle and no usable mirror
cache is available for the canonical gallery. The gallery renders only a
local-safe row plus a `starter_policy_notice_record` with
`policy_notice_class = mirror_or_airgap_policy_notice` and
`availability_narrowing_class = offline_no_bundle`, offering resolution
hooks `refresh_mirror`, `switch_to_live_origin`, and `set_up_later`. All
four offline bypass paths render at equal weight. See
[`template_gallery_offline_no_bundle.json`](../../fixtures/ux/template_and_prebuild_states/template_gallery_offline_no_bundle.json).

### 13.11 Target-runtime-unavailable row

First-run gallery where one starter requires a managed runtime that is
unavailable. The row remains visible-but-disabled with
`disabled_reason_code = target_runtime_unavailable` and carries an
in-place `starter_policy_notice_record` with
`policy_notice_class = target_runtime_unavailable_notice`. Resolution
hooks `reconnect_required`, `continue_in_restricted_mode`, and
`set_up_later` keep the state actionable, while bypass paths remain at
equal weight. See
[`template_gallery_target_runtime_unavailable.json`](../../fixtures/ux/template_and_prebuild_states/template_gallery_target_runtime_unavailable.json).

### 13.12 Toolchain/OS compatibility health strip

Gallery renders an inline health strip for a starter whose prerequisites
are incompatible with this host. The strip includes
`os_runtime_compatibility` with `result = fail` (`template_health_signal_class = live`)
and `toolchain_compatibility` with `result = warn` (`template_health_signal_class = cached`,
`last_validated_timestamp_class = older`) so the surface cannot
over-claim compatibility. Bypass remains unblocked. See
[`template_gallery_toolchain_os_incompatible.json`](../../fixtures/ux/template_and_prebuild_states/template_gallery_toolchain_os_incompatible.json).

## 14. Acceptance mapping

- **Non-template, non-prebuild open path of equal weight.**
  Surface rules §11.1, §11.5, and §3.9 rule 2 require at
  least one bypass path rendered at same zone, same weight,
  and same focus order on every gallery, picker,
  preflight summary, and resume-live card. Fixtures §13.1,
  §13.2, §13.4, §13.5, §13.6, §13.7, §13.9, §13.10,
  §13.11, and §13.12 all advertise the bypass paths
  explicitly.
- **No silent trust / extension install / setup run.**
  Surface rules §11.2, §11.3, §11.4, plus §6.2 rule 1
  (summary before commit) and §3.5 rule 2 (resume-live
  runs no mutating bootstrap) prevent silent grants and
  silent side effects. Fixture §13.7 demonstrates the
  resume-live rule; fixture §13.2 shows the preflight
  previews.
- **Docs / support / compatibility claims later.** Every
  closed vocabulary (§3) and every record shape (§4–§10)
  is re-exported by id into the matrix at
  [`/artifacts/ux/template_source_class_matrix.yaml`](../../artifacts/ux/template_source_class_matrix.yaml),
  so docs authors, support exporters, and compatibility
  audits read the same ids the shell renders.
- **Stale / issue-flagged fixture with bypass intact.**
  Fixture §13.4 exercises
  `declared_freshness_class = mirror_stale` and
  `template_health_signal_class = stale_or_invalid` with
  the bypass paths still rendered at equal weight.
- **Policy-narrowed or mirror-only fixture explains
  narrowing without external context.** Fixtures §13.5
  (`fleet_policy_notice`), §13.6
  (`mirror_or_airgap_policy_notice`), §13.9
  (`admin_policy_notice`), and §13.10
  (`mirror_or_airgap_policy_notice`) all render the
  `starter_policy_notice_record` in-place with the
  narrowing class and resolution hooks named verbatim.

## 15. Changing this contract

- **Additive-minor** changes (new
  `template_and_prebuild_surface_family`, new
  `template_source_class`, new `support_class`, new
  `runtime_and_toolchain_scope`, new
  `template_lifecycle_class`, new
  `starter_setup_cost_class`, new
  `availability_narrowing_class`, new `bypass_path_id`,
  new `template_health_signal_class` or
  `template_health_check_class`, new
  `generation_preflight_axis`, new
  `post_create_handoff_axis`, new `policy_notice_class`)
  land here and in the companion matrix plus at least one
  fixture in the same change. Every new value MUST cite
  the motivating startup state, record kind, or fixture.
- **Repurposing** an existing vocabulary value is
  breaking and opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (entry-restore object
  model, source-acquisition seed, start-center contract)
  happen at source and this contract re-exports by
  reference; it MUST NOT shadow the change.
- The PRD / TAD / TDD / UI-UX spec wins on any
  disagreement with the quotations in §16; this contract
  and its matrix plus fixtures update in the same change.

## 16. Source anchors

- `.t2/docs/Aureline_PRD.md:254` — dev-container
  compatibility, workspace templates, and optional
  prebuild snapshots are part of the remote story from
  day one.
- `.t2/docs/Aureline_PRD.md:1259` — remote workspaces
  should accept repo-defined devcontainer metadata and
  optional prebuild snapshots so environment setup is
  reproducible and accelerable.
- `.t2/docs/Aureline_PRD.md:1468` — content-addressed
  caches are scoped separately for indexes, prebuilds,
  and AI artifacts; remote workspace images and
  prebuild artifacts should be content-addressed and
  mirror-friendly.
- `.t2/docs/Aureline_PRD.md:2328` — intelligent project
  scaffolding and generation: starter templates and
  agentic setup for new services / apps / modules using
  team standards.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:802` — §6.9
  templates, starters, and prebuilds: the six
  disclosure axes (source class, support class,
  runtime / toolchain, freshness, setup actions,
  always-available bypass path) plus the side-effect
  envelope rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:6346` — §17.7
  scaffolding, generation, and template health: the six
  scaffold surfaces and the health-signal classes
  (`live`, `cached`, `policy_evaluated`, `not_checked`).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:5878` —
  §16.11 managed workspace provisioning and
  suspend / resume; resume-live card composes the
  closed state set.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:10778` —
  §23.74 shell-zoning and starter-disclosure drills:
  `set-up-later` and `open-without-starter` paths must
  stay visible in corpora.
- `.t2/docs/Aureline_Milestones_Document.md:3787` —
  environment-capsule schema draft, workspace-template
  seed, and prebuild-metadata baseline.
- `.t2/docs/Aureline_Milestones_Document.md:3893` —
  environment-capsule resolver beta with devcontainer /
  Nix / Compose parsing; workspace-template bundle alpha
  and prebuild fingerprint / invalidation baseline.

## 17. Linked artifacts

- Entry / restore object model (source of truth for
  entry verbs, target kinds, resulting modes, restore
  levels, recovery classes, next-step decision hooks
  this contract re-exports):
  [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md).
- Source-locator, checkout-plan, trust-stage, and
  bootstrap-queue seed (source of truth for locator
  class, transport class, acquisition posture,
  declared freshness, signer continuity, trust stage,
  browse-safe actions, blocked execution paths,
  resumable-acquisition state, read-only partial
  roots, topology markers, bootstrap item / state /
  absence / blocker / repair-hook classes):
  [`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../workspace/source_acquisition_and_bootstrap_seed.md).
- Start Center, workspace-switcher, open-flow,
  restore-card, and recent-work disclosure contract
  (source of truth for `start_center_zone`,
  `primary_action_id`, `disclosure_class`,
  `privacy_reduction_mode`, and the seeded-state
  matrix this contract composes into):
  [`/docs/ux/start_center_contract.md`](./start_center_contract.md).
- Entry / restore placeholder truth audit (source of
  truth for `startup_state` tokens):
  [`/docs/ux/entry_restore_truth_audit.md`](./entry_restore_truth_audit.md).
- Navigation hierarchy and escalation contract (source
  of truth for `navigation_route_id` and zone rules):
  [`/docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md).
- Template-source-class matrix (machine-readable
  companion to this contract):
  [`/artifacts/ux/template_source_class_matrix.yaml`](../../artifacts/ux/template_source_class_matrix.yaml).
- Worked-example fixtures:
  [`/fixtures/ux/template_and_prebuild_states/`](../../fixtures/ux/template_and_prebuild_states/).
