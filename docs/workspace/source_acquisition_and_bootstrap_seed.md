# Source-locator, checkout-plan, trust-stage, and bootstrap-queue seed

This document freezes the cross-surface vocabulary every Aureline
open, clone, import, template, prebuild, snapshot, and live-resume
flow uses when it names **what kind of source is being acquired**,
**where the source lives**, **what the checkout plan is allowed to
do before trust admission**, **which execution paths are blocked
until trust is admitted**, **how an interrupted acquisition can be
resumed, discarded, or opened read-only**, **which mirror-freshness
and signer-continuity evidence rides with the plan**, **which
read-only partial roots are already usable**, **which seed-level
topology markers (sparse workset, partial clone / promisor,
submodule init, LFS hydrate) apply**, and **which typed bootstrap
items (submodule init, LFS hydrate, package restore, generator
install, index warm-up, docs import, and similar) the plan enqueues
with attributable evidence**.

Project acquisition and bootstrap are product truth surfaces, not a
late-UX retrofit. If Aureline cannot say which target type was
resolved, which trust stage the plan is on, which browse-safe
actions are available before commit, which execution paths are
blocked, which bootstrap item is pending vs. failed vs. blocked on
network, and whether a missing artifact is not-yet-fetched vs.
genuinely absent, then open / clone / import / resume surfaces
will collapse into one opaque "setting up workspace" spinner and
support will re-invent the vocabulary per incident.

The machine-readable schemas live at:

- [`/schemas/workspace/source_locator.schema.json`](../../schemas/workspace/source_locator.schema.json)
- [`/schemas/workspace/checkout_plan.schema.json`](../../schemas/workspace/checkout_plan.schema.json)
- [`/schemas/workspace/bootstrap_queue_item.schema.json`](../../schemas/workspace/bootstrap_queue_item.schema.json)

The companion fixtures live under:

- [`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/)

The entry, recent-work, and restore-prompt model that hands a
source off to an open flow lives at:

- [`/docs/workspace/entry_restore_object_model.md`](./entry_restore_object_model.md)

The install / fleet topology and state-root plan the policy-guided
deployment lane reads from lives at:

- [`/docs/release/install_topology_plan.md`](../release/install_topology_plan.md)

The target-discovery, host-boundary, managed-workspace-lifecycle,
and install-review taxonomy that downstream execution and install
surfaces reuse lives at:

- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md)

The target-and-host-boundary verification packet that managed-
workspace resume flows project against lives at:

- [`/docs/verification/target_and_host_boundary_packet.md`](../verification/target_and_host_boundary_packet.md)

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, or UI/UX spec sections cited in §8, those documents win and
this document plus the three schemas MUST be updated in the same
change. Where a later Start Center, trust-review sheet,
interrupted-acquisition banner, bootstrap packet, support export,
or CLI entry surface mints its own locator, trust-stage,
resume-state, topology-marker, bootstrap-item, absence-class, or
attributable-evidence vocabulary, this document wins and the
surface is non-conforming.

## Why freeze this now

Project acquisition is the single most common place an IDE turns a
solvable question ("where is the missing file?") into an unsolvable
one ("setting up workspace…"). The hazards stack:

- A Start Center clicks *Clone* on a mirror-served URL and the user
  has no way to tell, before commit, whether the mirror is live
  with upstream or three weeks lagged.
- A user drops a `.zip` snapshot on the window and the shell
  silently runs the same post-extract hooks it would run on a
  trusted repo, because there is no trust-stage vocabulary distinct
  from "admitted / not admitted."
- A network blip interrupts `git clone --filter=blob:none` and the
  partial clone sits on disk as a read-only partial root, but the
  shell offers only `Try again` vs. `Cancel`, with no way to
  inspect what is already there.
- An LFS-pointer-only tree renders `File not found` on every
  pointer; the user cannot distinguish "not yet hydrated" from
  "genuinely absent" from "user misconfigured."
- A template / prebuild lands on disk with its setup actions
  queued, but the bootstrap packet collapses toolchain detect,
  dependency install, devcontainer attach, and extension restore
  into one opaque progress bar.
- A policy-guided deployment lane injects a policy-narrowing that
  blocks `generator_run` in restricted mode, but the surface says
  only "setup failed" instead of naming the concrete execution
  path that was refused and the policy that refused it.

Freezing the locator, the plan, and the bootstrap-item record —
before clone, import, template, snapshot, and resume flows harden
— means:

- open / clone / import / resume flows render target type, trust
  stage, and bootstrap work without one opaque "setting up
  workspace" spinner;
- interrupted acquisition has explicit resume, discard, and
  open-read-only states instead of a binary retry / cancel;
- Start Center, policy-guided deployment, support, and CLI flows
  all reuse the same source-acquisition vocabulary;
- bootstrap packets distinguish not-yet-fetched or not-yet-hydrated
  content from genuine absence or user misconfiguration, so
  "missing" is no longer a single failure class.

## Scope

This seed freezes three record kinds across three schemas:

1. `source_locator_record` — names the kind of source being
   acquired, the transport and auth posture, the declared
   freshness, the signer-continuity posture, and the current
   acquisition posture. One locator per source; downstream
   surfaces read the locator by id.
2. `checkout_plan_record` — names the trust stage, the browse-safe
   actions available before commit, the blocked execution paths,
   the resumable-acquisition state, the mirror-freshness evidence,
   the signer-continuity evidence, the read-only partial roots,
   the seed-level topology markers, the policy narrowing, and the
   typed next-step decision hooks. One plan per acquisition.
3. `bootstrap_queue_item_record` — names one typed setup work
   item (submodule init, LFS hydrate, package restore, generator
   install, index warm-up, docs import, toolchain install,
   devcontainer attach, extension restore, secret handle request,
   cache warm, settings materialize, mirror refresh, and similar),
   its execution class, its state, its absence class, its
   attributable evidence, and its typed repair hooks. Zero or more
   items per plan.

Out of scope:

- Full clone / fetch implementation, package-manager
  implementation, submodule / LFS implementation, generator
  framework implementation, or devcontainer attach implementation.
  The vocabulary freeze lands here; the surfaces that execute the
  items are later milestones.
- Final copy / microcopy. This document freezes the closed sets
  the copy resolves against; the rendered strings live with the
  shell interaction-safety contract.
- Repository-edge identity layers (filesystem identity, object-id
  layers, commit identity). Those are ADR-0006 and the future
  repository-edge contract. The topology markers here are
  deliberately seed-level: they label sparse / partial / submodule
  / LFS state at acquisition time without redefining the deeper
  edge contract.

## 1. Source-locator record

Every surface that identifies a source before commit emits exactly
one `source_locator_record`. Start Center rows, command-palette
open / clone / import rows, OS file association handlers,
drag-and-drop handlers, deep-link resolvers, CLI / headless entry
points, template / prebuild cards, recent-work activation,
migration-center imports, and companion-handoff returns all
resolve through the same record.

### 1.1 Locator class

The set is closed:

- `local_folder`
- `local_file`
- `workspace_file_manifest`
- `workset_manifest`
- `repo_url`
- `mirror_or_proxy_repo`
- `snapshot_archive`
- `template`
- `prebuild_snapshot`
- `live_resume_target`
- `handoff_packet`
- `portable_state_package`
- `recovery_checkpoint`
- `review_or_work_item_deep_link`

Rules (frozen):

1. Every record names exactly one `locator_class`. A surface that
   observes an unresolved locator denies with
   `locator_class_unresolved` rather than silently classify.
2. `mirror_or_proxy_repo` is not a substitute for `repo_url`: a
   `repo_url` served by a mirror uses `transport_class = mirror`
   on its `host_endpoint`; `mirror_or_proxy_repo` is reserved for
   locators whose primary identity is the mirror itself (for
   example, a customer-managed mirror index).
3. `live_resume_target` never implies `authority_live`; the
   `attach_authority_class` on the `live_session_descriptor` is
   authoritative.
4. `recovery_checkpoint` names a locator that resolves to a
   previously minted checkpoint; the record carries
   `entry_verb_hint = restore` so downstream surfaces do not
   re-classify.

### 1.2 Transport class

The set is closed:

- `local_filesystem`
- `https`
- `ssh`
- `git_protocol`
- `sftp`
- `mirror`
- `proxy`
- `air_gapped_media`
- `aureline_remote`
- `managed_cloud`
- `devcontainer_runtime`
- `container_runtime`
- `file_upload`
- `deep_link_handoff`
- `other`

Rules (frozen):

1. Transport is separate from locator class so a `repo_url` served
   by a `mirror`, or a `snapshot_archive` delivered via
   `air_gapped_media`, is inspectable without minting a new
   locator class.
2. `air_gapped_media` carries no `approval_ticket_ref`; acquisition
   via air-gap pairs with a separately reviewed offline bundle
   ticket.

### 1.3 Acquisition posture

The set is closed:

- `already_on_disk`
- `not_yet_acquired`
- `partially_acquired`
- `filtered_or_sparse`
- `hydrated_stale`
- `live_session_attached`
- `unreachable`
- `policy_blocked`
- `unknown`

Rules (frozen):

1. `partially_acquired` and `filtered_or_sparse` are distinct
   from `unreachable`: the first two are deliberate partial states
   the checkout plan tracks and may already permit read-only
   inspection on; the third is a transient reachability failure.
2. `unknown` is allowed only on locators whose canonical owner
   has not been re-reached for verification; the chip reads
   `unknown` rather than collapse into `already_on_disk`.

### 1.4 Declared freshness class

The set is closed:

- `live_origin`
- `mirror_fresh`
- `mirror_lagged`
- `mirror_stale`
- `offline_snapshot`
- `signed_offline_bundle`
- `unknown_freshness`

Rules (frozen):

1. A surface that silently rewrites `mirror_lagged` or
   `mirror_stale` to `mirror_fresh` is non-conforming.
2. `offline_snapshot` and `signed_offline_bundle` are distinct:
   `signed_offline_bundle` carries a producer signature whose
   continuity rides into §1.5.

### 1.5 Signer continuity class

The set is closed:

- `continuous_with_previous_acquisition`
- `new_signer_first_seen`
- `signer_changed_trust_on_first_use`
- `signer_changed_review_required`
- `signer_rotation_preauthorized`
- `unsigned`
- `signature_missing`
- `signature_mismatch`
- `not_applicable`

Rules (frozen):

1. `not_applicable` is allowed only when the locator never rides
   through a signer (for example, a `local_folder` with no
   ADR-0010 ticket).
2. `signer_changed_review_required` MUST route to a typed review
   hook on the companion `checkout_plan_record`. A surface that
   admits this class without a review ticket is non-conforming.

## 2. Checkout-plan record

Every surface that acquires a source emits exactly one
`checkout_plan_record`. Start Center trust-review sheets, open /
clone / import / resume / template / prebuild entry flows,
first-run policy-guided deployment lanes, interrupted-acquisition
recovery surfaces, support-bundle exporters, and CLI / headless
entry points all resolve through the same record.

### 2.1 Trust stage

The set is closed:

- `not_evaluated`
- `pre_fetch_inspection`
- `post_fetch_content_review`
- `admitted_restricted`
- `admitted_trusted`
- `quarantined`
- `policy_blocked`
- `signer_review_required`
- `reauth_required`
- `reconnect_required`

Rules (frozen):

1. `not_evaluated` is the initial stage before any inspection;
   it admits no browse-safe actions beyond naming the locator.
2. `pre_fetch_inspection` permits browse-safe actions on manifest
   / metadata only; no bytes are fetched.
3. `post_fetch_content_review` permits browse-safe actions on
   acquired content with execution paths from §2.3 still blocked.
4. `admitted_restricted` and `admitted_trusted` reuse the ADR-0001
   identity-mode admission vocabulary; this document does not
   redefine trust-state itself.
5. A surface that advances `trust_stage` without a typed admission
   record is non-conforming.

### 2.2 Browse-safe actions

The set is closed:

- `inspect_manifest`
- `inspect_history`
- `inspect_contributors`
- `inspect_readme`
- `inspect_license`
- `inspect_workflows_ci_config`
- `inspect_dependency_manifest`
- `inspect_scripts_catalog`
- `inspect_entry_points`
- `diff_before_commit`
- `render_diff_readonly`
- `show_signer_identity`
- `show_mirror_metadata`
- `show_upstream_delta`
- `show_partial_or_sparse_scope`
- `export_evidence_only`

Rules (frozen):

1. Every plan MUST advertise at least one browse-safe action
   (even `inspect_manifest` / `inspect_readme` on the thinnest
   pre-fetch inspection).
2. Browse-safe actions are read-only and non-executing. A
   browse-safe action MUST NOT trigger a hook, generator, LFS
   smudge filter, post-checkout script, or other side-effect.

### 2.3 Blocked execution paths

The set is closed:

- `post_checkout_hook`
- `pre_commit_hook`
- `git_filter_driver`
- `lfs_smudge_filter`
- `submodule_recursive_init`
- `submodule_update_command`
- `generator_install`
- `generator_run`
- `package_manager_restore`
- `package_manager_postinstall_script`
- `toolchain_bootstrap`
- `devcontainer_bootstrap`
- `prebuild_attach_side_effects`
- `remote_attach_handshake`
- `index_warmup_with_workers`
- `docs_import_fetch`
- `ai_context_warmup`
- `extension_activation`
- `workspace_auto_task`
- `deep_link_side_effect`

Rules (frozen):

1. Every plan MUST enumerate at least one blocked execution path.
2. Admission upgrades narrow the blocked list; a surface that
   widens the list opaquely (for example, by collapsing
   `generator_run` and `package_manager_postinstall_script` into
   one "scripts") is non-conforming.
3. A surface that claims a path is blocked without naming one of
   these classes is non-conforming.

### 2.4 Resumable acquisition

Every plan carries a typed `resumable_acquisition` block whose
`resume_state` is drawn from the closed set:

- `never_started`
- `in_progress`
- `interrupted_resumable`
- `interrupted_discard_required`
- `interrupted_open_read_only_available`
- `completed`
- `aborted`

Paired with a `discard_posture` drawn from:

- `no_discard_required`
- `discard_staging_only`
- `discard_with_compensation`
- `discard_blocked_require_admin`
- `discard_unavailable_manual_only`

Rules (frozen):

1. `interrupted_open_read_only_available` is distinct from
   `interrupted_resumable`: the former names read-only partial
   roots the plan already made safe to inspect; the latter names
   a resumable acquisition whose partial bytes are not yet
   review-ready.
2. A surface that collapses any two of the resume states into a
   generic "failed setup" banner is non-conforming.
3. Every interrupted plan MUST expose at least one of the
   next-step hooks `resume_acquisition`, `discard_and_restart`, or
   `open_read_only_partial`.

### 2.5 Mirror freshness

Every plan whose locator rides through a mirror or proxy carries
a typed `mirror_freshness` block. The block names a closed
`freshness_class` (re-exported from §1.4) plus an `upstream_delta_class`:

- `no_delta`
- `delta_within_declared_skew`
- `delta_outside_declared_skew`
- `delta_unmeasured`
- `delta_unknown`

Rules (frozen):

1. A surface that renders a chip MUST render
   `delta_outside_declared_skew` distinctly so the user does not
   mistake a stale mirror for live origin.
2. `delta_unmeasured` and `delta_unknown` are distinct:
   `delta_unmeasured` means the delta was not computed for this
   acquisition; `delta_unknown` means the canonical upstream was
   unreachable when the delta check ran.

### 2.6 Signer continuity

Every plan carries a typed `signer_continuity` block that quotes
the closed class from §1.5 and names typed refs for the signer
identity, the previous signer identity, the rotation policy (when
the class is `signer_rotation_preauthorized`), and the review
ticket (when the class is `signer_changed_review_required`).

### 2.7 Read-only partial roots

Zero or more typed roots the plan has already made safe to
inspect. Classes:

- `sparse_checkout_root`
- `partial_clone_filter_root`
- `promisor_pack_root`
- `shallow_history_root`
- `lfs_pointer_only_root`
- `submodule_placeholder_root`
- `archive_extracted_read_only`
- `template_materialized_read_only`
- `prebuild_attached_read_only`
- `remote_virtual_root`
- `portable_bundle_extracted_read_only`

Rules (frozen):

1. A surface that silently treats any of these as `fully_present`
   is non-conforming. The chip reads `Read-only partial` or a
   class-specific label verbatim.

### 2.8 Seed-level topology markers

Every plan carries at least one typed topology marker. The set is
closed:

- `sparse_workset_present`
- `partial_clone_filter_present`
- `promisor_remote_required`
- `shallow_history_present`
- `submodule_init_pending`
- `submodule_init_partial`
- `submodule_init_complete`
- `submodule_init_failed`
- `lfs_pointer_only`
- `lfs_hydrate_pending`
- `lfs_hydrate_partial`
- `lfs_hydrate_complete`
- `lfs_hydrate_failed`
- `none`

Rules (frozen):

1. A plan with no applicable topology uses
   `[{ marker_class: 'none' }]` rather than omit the field.
2. Topology markers land in bootstrap fixtures even before
   deeper repository-edge contracts land, so acquisition cannot
   collapse all missing content into one generic setup failure.
3. A surface that claims "submodule / LFS state is not known" on
   a plan that already labelled one of the markers above is
   non-conforming.

### 2.9 Policy narrowing

Zero or more typed policy-narrowing refs explain why blocked
execution paths include entries beyond the plan's default
trust-stage posture. Classes:

- `workspace_trust_policy`
- `admin_policy`
- `fleet_policy`
- `connected_provider_policy`
- `extension_effective_permission`
- `signature_policy`
- `mirror_or_airgap_policy`

### 2.10 Next-step decision hooks

Every plan advertises at least one typed next-step decision hook.
The set re-exports the entry-restore hook set (M00-28) and adds
six acquisition-specific hooks:

- `resume_acquisition`
- `discard_and_restart`
- `open_read_only_partial`
- `review_signer_change`
- `refresh_mirror`
- `switch_to_live_origin`

Free-form action labels are non-conforming.

## 3. Bootstrap-queue item record

Zero or more typed `bootstrap_queue_item_record` entries hang off
a `checkout_plan_record`. Each item names the typed setup work
the plan enqueues; together they let the shell, CLI, and support
surfaces render a typed progress list instead of one opaque
spinner.

### 3.1 Item class

The set is closed (additive-minor):

- `submodule_init`
- `lfs_hydrate`
- `partial_clone_hydrate`
- `shallow_history_deepen`
- `package_restore`
- `package_audit`
- `generator_install`
- `generator_run`
- `toolchain_install`
- `toolchain_detect`
- `devcontainer_attach`
- `prebuild_attach`
- `extension_restore`
- `extension_activation`
- `index_warm_up`
- `docs_import`
- `ai_context_warm_up`
- `secret_handle_request`
- `cache_warm`
- `settings_materialize`
- `profile_materialize`
- `credential_provisioning`
- `mirror_refresh`
- `other`

### 3.2 Execution class

The set is closed:

- `browse_safe`
- `side_effect_declared`
- `network_required`
- `privileged`
- `blocked`
- `deferred_until_trust_admitted`
- `manual_user_action_required`

Rules (frozen):

1. `browse_safe` items may run before trust admission.
2. `side_effect_declared` items MUST carry a typed
   `side_effect_envelope` (re-exported from the entry-restore
   schema); a surface that declares side-effects without the
   envelope is non-conforming.
3. `privileged` items require elevation or admin acknowledgement
   and MUST advertise a `request_admin_help` repair hook.

### 3.3 Item state

The set is closed:

- `pending`
- `running`
- `succeeded`
- `failed_recoverable`
- `failed_blocking`
- `skipped`
- `cancelled`
- `awaiting_admission`
- `awaiting_network`
- `awaiting_user_action`
- `awaiting_policy_decision`

Rules (frozen):

1. The `awaiting_*` states are distinct from `pending` so the
   bootstrap packet can say "this item is blocked on network"
   rather than "setup failed."
2. `skipped` MUST pair with a typed `skip_reason` (see §3.5).
3. `failed_recoverable`, `failed_blocking`, and the `awaiting_*`
   states MUST pair with a typed `blocker` (see §3.5) and at
   least one typed `repair_hook` (see §3.7).

### 3.4 Absence class

The set is closed:

- `present`
- `not_yet_fetched`
- `not_yet_hydrated`
- `partially_hydrated`
- `genuinely_absent`
- `user_misconfigured`
- `policy_blocked`
- `trust_blocked`
- `mirror_unreachable`
- `authority_expired`
- `schema_version_unsupported`
- `unknown`

Rules (frozen):

1. Every item carries exactly one `absence_class`. A surface that
   collapses `not_yet_fetched` or `not_yet_hydrated` into
   `genuinely_absent` is non-conforming.
2. `present` applies when the item's content is already
   materialized (for example, a `lfs_hydrate` item whose content
   is already hydrated from a prior acquisition).
3. `unknown` is allowed only on items whose canonical owner has
   not been re-reached for verification.

### 3.5 Skip reasons and blockers

Skip-reason classes (required on `skipped`):

- `user_deselected`
- `policy_excludes`
- `trust_excludes`
- `unsupported_on_target`
- `redundant_with_existing`
- `source_missing`
- `offline_bundle_excludes`
- `lossy_step_refused`
- `other`

Blocker classes (required on failure / awaiting states):

- `network_interruption`
- `auth_expired`
- `authority_revoked`
- `signer_mismatch`
- `mirror_unreachable`
- `disk_pressure`
- `quota_exceeded`
- `policy_blocked`
- `trust_blocked`
- `schema_version_unsupported`
- `dependency_item_failed`
- `user_cancelled`
- `unknown`

### 3.6 Attributable evidence

Every item carries at least one typed attributable-evidence entry
drawn from the closed set:

- `source_producer_identity`
- `checkout_plan_reference`
- `source_locator_reference`
- `package_manifest_ref`
- `lockfile_digest_ref`
- `generator_manifest_ref`
- `toolchain_manifest_ref`
- `extension_manifest_ref`
- `docs_pack_manifest_ref`
- `policy_bundle_ref`
- `trust_review_ticket_ref`
- `signer_continuity_ref`
- `mirror_freshness_ref`
- `previous_bootstrap_run_ref`
- `recovery_checkpoint_ref`
- `user_action_ref`

Rules (frozen):

1. An item with no evidence denies with
   `attributable_evidence_missing`.
2. Evidence is typed; free-form evidence strings are
   non-conforming.

### 3.7 Repair hooks

Closed set (required on failure / awaiting states):

- `retry_with_backoff`
- `retry_after_network_restored`
- `retry_after_reauth`
- `retry_after_policy_refresh`
- `refresh_mirror_then_retry`
- `switch_to_live_origin_then_retry`
- `deepen_history_then_retry`
- `skip_and_continue`
- `skip_and_open_read_only`
- `discard_and_restart`
- `open_read_only_partial`
- `request_admin_help`
- `open_minimal`
- `set_up_later`

## 4. Worked examples

Each example references a companion fixture under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/).

### 4.1 Clone with submodules and LFS, pre-fetch inspection

The user clicks `Clone` on a remote `github.com/acme/payments`
row in Start Center. The `source_locator_record` names
`locator_class = repo_url`, `transport_class = ssh`,
`acquisition_posture = not_yet_acquired`,
`declared_freshness_class = live_origin`,
`signer_continuity_class = continuous_with_previous_acquisition`.
The `checkout_plan_record` sits at
`trust_stage = pre_fetch_inspection`, advertises browse-safe
actions `inspect_manifest`, `inspect_history`, `show_signer_identity`,
and `show_partial_or_sparse_scope`, blocks `post_checkout_hook`,
`lfs_smudge_filter`, `submodule_recursive_init`,
`generator_install`, `generator_run`,
`package_manager_postinstall_script`, and `extension_activation`,
and carries topology markers `submodule_init_pending` and
`lfs_pointer_only`. The companion bootstrap queue has three
items: `submodule_init` (state `pending`, absence_class
`not_yet_fetched`, execution `deferred_until_trust_admitted`);
`lfs_hydrate` (state `pending`, absence_class `not_yet_hydrated`,
execution `network_required`); `package_restore` (state
`pending`, absence_class `not_yet_fetched`, execution
`deferred_until_trust_admitted`).

See the `clone_remote_with_submodules_and_lfs__*` fixtures
under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/)
(locator, plan, and bootstrap items for `submodule_init`,
`lfs_hydrate`, and `package_restore`).

### 4.2 Interrupted mirror clone, resume available

A partial-clone acquisition from a customer-managed mirror hit a
network blip after pulling the promisor pack. The
`source_locator_record` carries
`locator_class = mirror_or_proxy_repo`,
`transport_class = mirror`,
`acquisition_posture = partially_acquired`,
`declared_freshness_class = mirror_lagged`. The plan sits at
`trust_stage = post_fetch_content_review`,
`resumable_acquisition.resume_state = interrupted_resumable`,
`discard_posture = discard_staging_only`,
`failure_reason_class = network_interruption`,
`open_read_only_available = true`. The `mirror_freshness` block
declares `upstream_delta_class = delta_outside_declared_skew` with
a summary label. `read_only_partial_roots[]` names one
`promisor_pack_root` the user may inspect right now. Next-step
hooks: `resume_acquisition`, `open_read_only_partial`,
`discard_and_restart`, `switch_to_live_origin`.

See the `resume_interrupted_mirror_clone__*` fixtures under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/)
(locator and plan).

### 4.3 Snapshot archive import with read-only extraction

The user drags a `.tar.gz` snapshot archive onto the window. The
locator names `locator_class = snapshot_archive`,
`transport_class = file_upload`,
`acquisition_posture = partially_acquired`,
`declared_freshness_class = offline_snapshot`,
`signer_continuity_class = unsigned`. The plan sits at
`trust_stage = post_fetch_content_review`, browse-safe actions
are `inspect_manifest`, `inspect_readme`, `inspect_license`,
`diff_before_commit`, and `export_evidence_only`; blocked paths
include `post_checkout_hook`, `generator_install`,
`generator_run`, and `workspace_auto_task`. One
`read_only_partial_root` of class `archive_extracted_read_only`
makes the extracted tree inspectable. Topology markers are
`none`. Bootstrap queue is empty. Next-step hooks include
`review_trust_and_open` and `open_read_only_partial`.

See the `snapshot_archive_import__*` fixtures under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/)
(locator and plan).

### 4.4 Template / prebuild with typed bootstrap queue

The user opens a TS web-app prebuild from Start Center. The
locator names `locator_class = prebuild_snapshot`,
`transport_class = managed_cloud`,
`acquisition_posture = partially_acquired`,
`declared_freshness_class = live_origin`,
`signer_continuity_class = signer_rotation_preauthorized`. The
plan advertises a typed prebuild-bootstrap queue of five items:
`toolchain_detect` (browse-safe, succeeded), `toolchain_install`
(side-effect-declared, running), `package_restore`
(network-required, pending, absence `not_yet_fetched`),
`devcontainer_attach` (side-effect-declared, pending, absence
`not_yet_hydrated`), and `extension_restore` (side-effect-declared,
pending, absence `not_yet_fetched`). Topology markers include
`partial_clone_filter_present` and `sparse_workset_present`.
Next-step hooks include `open_minimal`, `set_up_later`, and
`open_read_only_partial`.

See the `prebuild_with_bootstrap_queue__*` fixtures under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/)
(locator, plan, and bootstrap items for `toolchain_install`,
`package_restore`, and `devcontainer_attach`).

### 4.5 Live-resume managed workspace

The user reopens a managed cloud workspace. The locator names
`locator_class = live_resume_target`,
`transport_class = managed_cloud`,
`acquisition_posture = live_session_attached`,
`signer_continuity_class = continuous_with_previous_acquisition`.
The `live_session_descriptor` sits at
`attach_authority_class = authority_pending_reauth`. The plan's
trust_stage is `reauth_required`, browse-safe actions are
`inspect_manifest` and `show_signer_identity`, blocked paths
include `remote_attach_handshake` and `generator_run`. Topology
markers are `none`. Bootstrap queue has one item
`credential_provisioning` in state `awaiting_user_action` with
blocker `auth_expired` and repair hooks `retry_after_reauth` and
`set_up_later`. Next-step hooks: `reauth_required` and
`continue_in_restricted_mode`.

See the `live_resume_managed_workspace__*` fixtures under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/)
(locator, plan, and the `credential_provisioning` bootstrap
item).

### 4.6 LFS pointer-only read-only browse

A sparse / shallow-clone of a repo whose assets live behind LFS
has already landed with LFS pointers only. The user chooses
`Open read-only` instead of hydrating. The locator is
`repo_url`, `acquisition_posture = filtered_or_sparse`. The plan
sits at `trust_stage = post_fetch_content_review`; topology
markers name `shallow_history_present` and `lfs_pointer_only`.
One read-only partial root of class `lfs_pointer_only_root`
makes the tree browsable. One bootstrap item `lfs_hydrate` sits
in state `pending` with absence_class `not_yet_hydrated` and
execution_class `deferred_until_trust_admitted`. Next-step hooks
include `open_read_only_partial` and `set_up_later`.

See the `lfs_pointer_only_read_only__*` fixtures under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/)
(locator, plan, and the `lfs_hydrate` bootstrap item).

### 4.7 Policy-guided deployment, generators blocked

A fleet-managed workstation clones a repo under an admin policy
that narrows `generator_run`, `workspace_auto_task`, and
`ai_context_warmup` even in `admitted_trusted`. The plan carries
one `policy_narrowing_ref` of `policy_source_class = admin_policy`
citing a policy bundle ref and the narrowed execution paths.
Bootstrap queue emits a `generator_install` item in state
`skipped` with `skip_reason = policy_excludes` and
`absence_class = policy_blocked`, plus a `package_restore` item
in state `pending` with execution `network_required`. Next-step
hooks include `open_minimal` and `set_up_later`.

See the `policy_guided_deployment_generators_blocked__*`
fixtures under
[`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/)
(locator, plan, and bootstrap items for `generator_install` and
`package_restore`).

## 5. Surface rules

These rules apply to every surface that renders, logs, exports,
or reasons about the records defined in §1 – §3.

1. **One canonical record per source / plan / item.** No surface
   mints private locator, trust-stage, resume-state,
   topology-marker, bootstrap-item, absence-class, or
   attributable-evidence vocabularies. Downstream surfaces read
   the record by id.
2. **Target type, trust stage, and bootstrap work are
   disclosed.** Open, clone, import, and resume flows render
   target type, trust stage, and the bootstrap queue without
   one opaque "setting up workspace" spinner.
3. **Interrupted acquisition has explicit branches.** Resume,
   discard, and open-read-only are typed next-step hooks, not
   free-form retry / cancel strings. A surface that collapses
   them into a binary retry / cancel is non-conforming.
4. **Mirror freshness and signer continuity are visible.** A
   mirror-served acquisition renders `mirror_lagged`,
   `mirror_stale`, or `delta_outside_declared_skew` distinctly.
   A signer change routes to `review_signer_change` before
   admission.
5. **Read-only partial roots are first-class.** Sparse,
   partial-clone, promisor, shallow-history, LFS-pointer-only,
   submodule-placeholder, archive-extracted, template-materialized,
   prebuild-attached, remote-virtual, and portable-bundle-extracted
   roots render `Read-only partial` with the class-specific chip.
6. **Topology markers land at seed level.** Sparse workset,
   partial-clone / promisor state, submodule init state, and LFS
   hydrate state ride on every bootstrap fixture so acquisition
   cannot collapse all missing content into one generic setup
   failure before the deeper repository-edge contracts land.
7. **Absence is typed.** `not_yet_fetched` and `not_yet_hydrated`
   are distinct from `genuinely_absent` and `user_misconfigured`.
   A bootstrap surface that renders "missing" without one of the
   typed classes is non-conforming.
8. **Attributable evidence is required.** Every bootstrap item
   cites at least one typed evidence entry so support, CLI, and
   policy surfaces can explain who / what authorized the item.
9. **Reusable vocabulary.** Start Center, policy-guided
   deployment, support, and CLI flows all read the same locator /
   plan / item records; no surface reinvents the vocabulary.
10. **Support parity.** Every record exports through support
    bundles, CLI / headless diagnostics, and claim manifests with
    the same fields it renders in chrome. Redaction is the only
    way to hide a field.

## 6. Changing this vocabulary

- **Additive-minor** changes (new `locator_class`,
  `transport_class`, `acquisition_posture`,
  `declared_freshness_class`, `signer_continuity_class`,
  `trust_stage`, `browse_safe_action_class`,
  `blocked_execution_path_class`, `acquisition_resume_state`,
  `discard_posture`, `mirror_freshness_class`,
  `upstream_delta_class`, `read_only_partial_root_class`,
  `topology_marker_class`, `policy_source_class`,
  `bootstrap_item_class`, `bootstrap_execution_class`,
  `bootstrap_item_state`, `absence_class`, `skip_reason_class`,
  `blocker_class`, `attributable_evidence_class`,
  `repair_hook_class`, `next_step_decision_hook`) land here and
  in the companion schema in the same change. The change MUST
  cite the motivating fixture or packet.
- **Repurposing** an existing value is breaking. It opens a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The PRD / TAD / TDD / UI-UX spec wins on any disagreement with
  the quotations in §8; this document and the schemas are
  updated in the same change.

## 7. Acceptance

- Open, clone, import, and resume flows can disclose target type,
  trust stage, and bootstrap work without one opaque "setting up
  workspace" spinner.
- Interrupted acquisition has explicit resume, discard, and
  open-read-only states via `acquisition_resume_state` and the
  acquisition-specific next-step decision hooks.
- Start Center, policy-guided deployment, support, and CLI flows
  can reuse the same source-acquisition vocabulary via
  `source_locator_record`, `checkout_plan_record`, and
  `bootstrap_queue_item_record`.
- Bootstrap packets can already distinguish not-yet-fetched or
  not-yet-hydrated content from genuine absence or user
  misconfiguration via the typed `absence_class` on every
  bootstrap item.
- Seed-level topology markers (sparse workset, partial clone /
  promisor state, submodule init state, LFS hydrate state) land
  on every bootstrap fixture even before the deeper
  repository-edge contracts land.
- The three schemas at
  [`/schemas/workspace/source_locator.schema.json`](../../schemas/workspace/source_locator.schema.json),
  [`/schemas/workspace/checkout_plan.schema.json`](../../schemas/workspace/checkout_plan.schema.json),
  and
  [`/schemas/workspace/bootstrap_queue_item.schema.json`](../../schemas/workspace/bootstrap_queue_item.schema.json)
  validate the seven worked-example fixtures under
  [`/fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/).

## 8. Source anchors

- `.t2/docs/Aureline_PRD.md` — project-entry, first-run import,
  and workspace-trust sections (open / clone / import / restore
  verbs as distinct flows; trust review before execution).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  repository-edge, managed-workspace lifecycle, and prebuild
  attach architecture (target discovery, attach authority,
  bootstrap orchestration).
- `.t2/docs/Aureline_Technical_Design_Document.md` — checkout /
  bootstrap orchestration, LFS / submodule / sparse / partial
  clone topology, mirror freshness, policy-guided deployment.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — Start Center
  trust-review sheets, interrupted-acquisition banners,
  read-only-partial chips, setup-queue list.
- `.t2/docs/Aureline_Milestones_Document.md` — acquisition and
  bootstrap as product truth surfaces; Start Center verb
  distinctness; no-account local lane.

## 9. Linked artifacts

- ADR (identity modes, trust vocabulary):
  [`docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md).
- ADR (filesystem identity, save pipeline, cache identity):
  [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- ADR (connected-provider / browser-handoff tickets):
  [`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md).
- Entry, recent-work, and restore-prompt model:
  [`docs/workspace/entry_restore_object_model.md`](./entry_restore_object_model.md).
- Install topology and state-root plan:
  [`docs/release/install_topology_plan.md`](../release/install_topology_plan.md).
- Target-discovery, host-boundary, and install-review taxonomy:
  [`docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md).
- Target-and-host-boundary verification packet:
  [`docs/verification/target_and_host_boundary_packet.md`](../verification/target_and_host_boundary_packet.md).
- Source-locator schema:
  [`schemas/workspace/source_locator.schema.json`](../../schemas/workspace/source_locator.schema.json).
- Checkout-plan schema:
  [`schemas/workspace/checkout_plan.schema.json`](../../schemas/workspace/checkout_plan.schema.json).
- Bootstrap-queue-item schema:
  [`schemas/workspace/bootstrap_queue_item.schema.json`](../../schemas/workspace/bootstrap_queue_item.schema.json).
- Worked-example fixtures:
  [`fixtures/workspace/bootstrap_cases/`](../../fixtures/workspace/bootstrap_cases/).
