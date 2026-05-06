# Workspace-Admission Checkpoint, Readiness-Task Grouping, and Blocked-vs-Optional Setup Contract

This document freezes the **post-entry workspace-admission checkpoint**
every entry activation emits once Aureline has resolved a target,
the **readiness-task grouping rules** that partition setup work into
`blocking_now`, `recommended_soon`, and `optional_later`, and the
**admission cards and banners** the shell renders so a user can tell —
at a glance — what already works, what setup is recommended, what is
policy- or trust-blocked, and what was intentionally deferred.

The contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI / UX Spec, or design-system style guide, those sources win and this
document plus its schema and fixtures update in the same change.
Where Start Center, the workspace switcher, the project doctor /
attention inbox, the post-clone handoff, the post-import handoff, the
deep-link intent review, the CLI / headless front-end, or a managed
cloud surface mints a parallel admission-checkpoint object, parallel
readiness grouping, or parallel admission card / banner family, this
contract wins and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/workspace/admission_checkpoint.schema.json`](../../schemas/workspace/admission_checkpoint.schema.json)
  — boundary schema for three records: `workspace_admission_checkpoint_record`,
  `admission_card_record`, and `admission_banner_record`.
- [`/fixtures/workspace/admission_cases/`](../../fixtures/workspace/admission_cases/)
  — worked YAML cases that exercise the closed admission-class set,
  the four blocked-reason classes (trust, policy, missing prerequisite,
  deployment profile), the four optional-reason classes, the three
  readiness buckets, the seven setup-location classes, the nine
  archetype-recommendation-source classes, the admission-card and
  admission-banner families, and the `Continue without` fallback.

This contract composes with, and does not replace:

- [`/docs/ux/archetype_detection_contract.md`](./archetype_detection_contract.md)
  and [`/schemas/workspace/archetype_detection.schema.json`](../../schemas/workspace/archetype_detection.schema.json)
  for detection outcomes, source-labeled signals, recommendation /
  policy / user-choice separation, the readiness task and bucket
  vocabulary, the first-useful-work route matrix, and the
  Set-up-later / remembered-routing invariants. The
  `workspace_admission_checkpoint_record` defined here is the
  promotion of the embedded `admission_checkpoint` object on the
  `workspace_archetype_admission_record` into a first-class boundary
  record. It re-exports vocabulary unchanged and adds the
  `archetype_recommendation_source_class`, `setup_location_class`,
  `blocked_reason_class`, `optional_reason_class`,
  `continue_without_class`, and the admission card / banner families
  so the post-entry surface stays inspectable at the schema boundary.
- [`/docs/ux/project_entry_contract.md`](./project_entry_contract.md)
  and [`/schemas/ux/entry_chooser_row.schema.json`](../../schemas/ux/entry_chooser_row.schema.json)
  / [`/schemas/ux/open_flow_sheet.schema.json`](../../schemas/ux/open_flow_sheet.schema.json)
  for the entry-verb matrix, entry-chooser row, and open-flow sheet
  pre-commit invariants. The admission checkpoint is emitted **after**
  open-flow-sheet commit, and **before** any post-entry side effect
  (trust grant, dependency restore, extension recommendation, hook /
  task execution, runtime attach, profile retarget). The checkpoint
  never re-opens the verb matrix, never widens the trust posture, and
  never bypasses the open-flow sheet's §5.3 pre-commit rules.
- [`/docs/ux/clone_review_contract.md`](./clone_review_contract.md)
  and the post-clone trust-stage record, and
  [`/docs/ux/import_handoff_review_contract.md`](./import_handoff_review_contract.md)
  and the post-entry handoff card record. The clone and import
  contracts own the per-verb post-row review and the verb-bound
  post-entry handoff cards. The admission checkpoint and admission
  cards / banners defined here are the **route-agnostic** post-entry
  view: a single object the project-doctor, attention inbox, support
  export, and CLI / headless preview can read regardless of which
  verb produced the entry.
- [`/docs/ux/trust_prompt_contract.md`](./trust_prompt_contract.md)
  and [`/docs/adr/0001-identity-mode-trust.md`](../adr/0001-identity-mode-trust.md)
  / [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for the `trust_state` vocabulary and the trust-prompt surface.
  The admission checkpoint **previews** trust state and trust-review
  requirements; the trust transition is owned by the trust-prompt
  surface and never fires from the admission checkpoint itself.
- [`/docs/ux/start_center_contract.md`](./start_center_contract.md),
  [`/docs/ux/recent_work_and_restore_card_contract.md`](./recent_work_and_restore_card_contract.md),
  and [`/docs/ux/notification_contract.md`](./notification_contract.md)
  for the surface taxonomies the admission cards and banners render
  inside. The admission card record is the per-bucket projection of
  the readiness section onto an inspectable card; the admission
  banner record is the workspace-wide banner that summarises what
  works now and what remains gated.

## Who reads this contract

- **Shell, Start Center, project-doctor, attention-inbox, status-bar,
  notification, workspace-switcher, post-clone handoff, post-import
  handoff, deep-link intent review, CLI / headless front-end, support
  console, and managed-workspace authors** wiring post-entry truth.
  Every entry activation emits exactly one
  `workspace_admission_checkpoint_record`, plus zero or more
  `admission_card_record` and `admission_banner_record` rows derived
  from the same checkpoint id.
- **Designers** sizing post-entry copy so the user can tell in-place
  what already works, what setup is recommended, what is blocked,
  and which fallback is safest.
- **Docs, support, accessibility, and measurement authors**
  attributing post-entry behavior to the same record family the shell
  renders, so a Start Center activation, a clone post-handoff card, an
  import post-entry card, and a CLI `--explain` text block trace to
  the same `admission_checkpoint_id`.

## 1. Scope

This contract freezes:

- One **workspace-admission checkpoint object** (§3) every entry
  activation emits once the open-flow sheet has committed and Aureline
  has resolved a root identity. The checkpoint carries:
  the entry action ref and source surface, the resolved target kind
  and resulting mode, the `root_identity_class` and opaque
  `root_identity_ref`, the workspace-scope ref, the `trust_state` and
  `admission_class`, the `archetype_recommendation_source_class` for
  every recommendation surfaced, the `setup_location_class` for every
  setup task that may run, the readiness-bucket grouping with typed
  `blocked_reason_class` and `optional_reason_class`, the
  `continue_without_class` fallback, and the
  `plain_open_available` / `ordinary_editing_available` invariants.
- The **readiness-task grouping rules** (§4) partition setup work
  into exactly three buckets — `blocking_now`, `recommended_soon`,
  `optional_later` — and require every blocking item to declare one
  of four typed `blocked_reason_class` values
  (`blocked_by_trust`, `blocked_by_policy`,
  `blocked_by_missing_prerequisite`, `blocked_by_deployment_profile`).
  Optional items declare one of four typed `optional_reason_class`
  values so a "blocked" item is never confused with an "optional"
  item even when both bucket arrays are empty.
- The **admission cards and banners contract** (§5) requires the
  shell to render at least one `admission_card_record` per non-empty
  bucket and exactly one `admission_banner_record` per checkpoint.
  The card and banner sets are closed; a surface that mints a generic
  "complete setup" card or a generic "Setup failed" banner is
  non-conforming.
- The **cross-surface invariants** (§6) so Start Center,
  project doctor / attention inbox, status bar, notifications,
  post-clone handoff, post-import handoff, CLI / headless preview,
  and support exports stay semantically aligned on the same checkpoint
  id and the same admission card / banner records.

## 2. Out of Scope

- Detection signal collection, archetype recommendation, policy
  evaluation, trust-prompt rendering, dependency restore, extension
  install, runtime attach, profile retarget, or readiness-task
  execution. Those are owned by the archetype-detection contract,
  the trust-prompt contract, the bundle-review contract, the
  package-restore contract, the extension-recommendation contract,
  and the appropriate runtime / profile contracts.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and shell-interaction-safety contract own the strings.
- Telemetry wire format. The onboarding measurement plan reserves
  `er.*` route ids and the post-entry attention bucket; this
  contract only tags the checkpoint with the
  `entry_route_id_family` it belongs to.
- The crash-recovery and entry-restore object model itself. The
  admission checkpoint composes with `project_entry_action_record`,
  `restore_prompt_record`, and `migration_result_record` and never
  redefines them.

## 3. Workspace-Admission Checkpoint

Every entry activation that opens, clones, imports, adds a root,
restores, resumes, or starts from a snapshot emits one
`workspace_admission_checkpoint_record` once the upstream open-flow
sheet has committed and Aureline has resolved a `root_identity_ref`.
A pure no-op open (open-flow sheet class
`open_local_target_no_review_required` per the project-entry contract
§5.1, no trust delta, no setup tasks, no recommendations) MAY skip
emitting an admission checkpoint; every other activation MUST emit
one. Skipping the checkpoint when any readiness bucket is non-empty,
when policy applies, when trust is `pending_evaluation`, or when the
admission class is non-`admitted` is non-conforming.

### 3.1 Required fields

- `record_kind = workspace_admission_checkpoint_record`.
- `admission_checkpoint_schema_version = 2`.
- `admission_checkpoint_id` — opaque, stable, safe to log and export.
- `entry_action_ref` — re-export of
  `project_entry_action_record.action_id` from the entry-restore
  object model.
- `entry_source` — re-export from the entry-restore object model
  (`single_file_open`, `folder_or_repo_open`, `repository_clone`,
  `review_or_incident_deep_link`, `restore_last_session`,
  `imported_state_or_handoff_packet`).
- `target_kind` — re-export from the entry-restore object model.
- `resulting_mode` — re-export from the entry-restore object model.
- `root_identity_class` — closed set (§3.2).
- `root_identity_ref` — opaque ref into the entry-restore filesystem
  / remote / handoff identity. A checkpoint that lacks
  `root_identity_ref` MUST set `root_identity_class =
  `no_root_identity_yet` and `admission_class = `needs_repair`;
  emitting a checkpoint without a root identity in any other case is
  non-conforming.
- `workspace_scope_ref` — opaque ref into the workspace / workset /
  multi-root scope the checkpoint applies to.
- `trust_state` — re-export of ADR-0001 trust vocabulary.
- `trust_review_class` — closed set (§3.3).
- `admission_class` — re-export of the entry-restore admission class.
- `archetype_recommendation_source_classes[]` — non-empty when at
  least one recommendation is surfaced; closed set (§3.4).
- `setup_location_classes[]` — non-empty when readiness work exists;
  closed set (§3.5).
- `readiness_bucket_summary` — see §4.
- `continue_without_class` — closed set (§3.6).
- `plain_open_available` — boolean. Required `true` whenever
  ordinary editing remains available, otherwise the checkpoint MUST
  cite the typed reason via `admission_class` and at least one
  blocking item.
- `ordinary_editing_available` — boolean. Required `true` whenever
  the user can read, edit, and search files at the resolved root
  without first running setup. A checkpoint whose
  `ordinary_editing_available = false` MUST carry at least one
  `blocking_now` task whose `blocked_reason_class` explains the
  block.
- `detected_fact_refs[]`, `recommendation_refs[]`,
  `policy_block_refs[]`, `blocked_setup_refs[]`,
  `optional_guidance_refs[]` — opaque ref arrays into the upstream
  archetype-detection record.
- `admission_card_refs[]` — opaque ref array into emitted
  `admission_card_record` rows.
- `admission_banner_ref` — opaque ref into the emitted
  `admission_banner_record`.
- `summary` — redaction-aware text (≤ 1024 graphemes).

Optional fields: `archetype_ref`,
`compatible_bundle_refs[]`, `restore_prompt_ref`,
`migration_result_ref`, `post_clone_trust_stage_ref`,
`post_entry_handoff_card_ref`, `prior_user_choice_ref`.

### 3.2 `root_identity_class` (closed)

The class of identity Aureline anchored the checkpoint to. The
checkpoint MUST report exactly one class.

- `filesystem_path_identity` — local file or folder identity.
- `repo_root_identity` — repository root anchored by VCS metadata.
- `workspace_manifest_identity` — Aureline workspace manifest at the
  resolved root.
- `workset_manifest_identity` — workset (slice) manifest.
- `remote_target_identity` — remote repository, SSH, container,
  devcontainer, or managed-cloud target.
- `managed_cloud_identity` — managed-cloud workspace identity.
- `handoff_packet_identity` — portable / handoff / support / archive
  packet identity.
- `template_or_prebuild_identity` — template or prebuild snapshot
  identity.
- `no_root_identity_yet` — Aureline could not anchor a root identity
  (deep-link review awaiting clone / open, missing remote agent,
  unresolved competitor config). The checkpoint MUST cite the
  blocking item.

### 3.3 `trust_review_class` (closed)

How the checkpoint relates to the trust-prompt surface.

- `no_trust_review_required` — the resolved trust state is trusted
  and no widening is pending.
- `trust_review_pending` — the trust prompt has not yet been
  invoked; the checkpoint shows the resulting state preview.
- `trust_review_in_progress` — the trust prompt is open and the
  checkpoint is waiting on its outcome.
- `trust_review_blocked_by_policy` — the trust transition is
  policy-narrowed; the checkpoint MUST also carry a
  `policy_block_ref`.
- `trust_revalidation_required` — the prior trust grant expired
  (managed-cloud resume, deep-link replay, snapshot freshness) and a
  re-grant is required.
- `trust_review_not_applicable` — the entry verb does not change
  trust (e.g. inspect-only of a handoff packet).

### 3.4 `archetype_recommendation_source_class` (closed)

A recommendation reaches the user only after Aureline can name where
the recommendation came from. The checkpoint reports one or more
sources for the recommendations it surfaces; a recommendation that
cannot resolve a source class is non-conforming and MUST be denied
upstream.

- `detected_facts` — derived from manifest, lockfile, filesystem
  layout, runtime probe, or VCS metadata signals.
- `heuristic_inference` — derived from Aureline heuristics that are not
  strong enough to present as a fact or as bundle metadata. The surface
  MUST keep the inference label visible and MUST cite the basis signals
  or facts that made the heuristic relevant.
- `bundle_metadata` — derived from a workflow-bundle marker or
  certified-bundle metadata.
- `admin_policy` — narrowed or expanded by signed policy or fleet
  policy.
- `prior_user_choice` — derived from a remembered routing /
  setup-later / dismissed-recommendation record.
- `extension_contribution` — contributed by an installed extension's
  detector or workflow hint.
- `template_default` — declared by a template / prebuild snapshot the
  user opened from.
- `import_packet` — declared by a portable / handoff / support /
  archive packet's manifest.
- `mixed_recommendation_source` — the recommendation set is fed by
  more than one of the eight classes above. The checkpoint MUST also
  list the contributing classes individually.

### 3.5 `setup_location_class` (closed)

Where readiness work runs, or why it does not run. The checkpoint
reports one or more classes; a checkpoint with non-empty readiness
buckets and an empty `setup_location_classes` is non-conforming.

- `local_machine` — runs on the user's machine.
- `container` — runs in a project container.
- `devcontainer` — runs in a devcontainer.
- `remote_agent` — runs on a remote agent attached to the workspace.
- `managed_workspace` — runs inside a managed-cloud workspace.
- `browser_handoff` — runs via a browser-handoff (auth, approval).
- `no_execution` — purely declarative (e.g. extension recommendation
  list, docs link, dismissable hint).
- `mixed_setup_location` — readiness tasks span more than one
  location. The checkpoint MUST also list the per-task locations
  through the readiness section.

### 3.6 `continue_without_class` (closed)

The single safe fallback the user may take instead of running setup.
A checkpoint with non-empty `blocking_now` MUST resolve to one of
`continue_in_restricted_mode`, `inspect_only`, `compare_before_restore`,
or `no_continue_without_action_available`. A checkpoint with empty
`blocking_now` MUST resolve to `set_up_later`, `open_minimal`,
`open_plain_explorer`, or `dismiss_recommendation`.

- `set_up_later` — defer the offered setup; remember the choice as
  a narrowing hint per the archetype-detection set-up-later contract.
- `open_minimal` — open the workspace without optional setup.
- `open_plain_explorer` — open the explorer without route guidance.
- `continue_in_restricted_mode` — continue editing under restricted
  trust; setup that requires trust is gated.
- `inspect_only` — inspect the artifact (handoff / archive /
  competitor config) without writing into a durable destination.
- `compare_before_restore` — compare the artifact / packet against
  the active workspace before any restore.
- `dismiss_recommendation` — dismiss a non-blocking recommendation.
- `no_continue_without_action_available` — every safe fallback is
  itself blocked. The checkpoint MUST cite at least one
  `policy_block_ref`, `blocked_setup_ref`, or
  `restore_prompt_ref` and MUST surface the
  `no_safe_fallback_admission_banner` banner class.

## 4. Readiness-Task Grouping Rules

Readiness-task vocabulary (`readiness_task_class`,
`readiness_task_state`, `execution_boundary`, `side_effect_class`)
re-exports unchanged from the archetype-detection schema. This
contract pins the **grouping rules** the shell follows when those
tasks are surfaced through the admission checkpoint and the admission
cards.

### 4.1 Three buckets, never collapsed

The readiness section of the checkpoint groups tasks into exactly
three arrays: `blocking_now`, `recommended_soon`, `optional_later`.
A surface that flattens these into a single "Complete setup" or
"Onboarding" list is non-conforming. Empty buckets remain valid; the
admission card / banner family (§5) makes the empty state honest.

- `blocking_now` — tasks that prevent the requested first useful
  surface, a required safety review, or the resulting mode the entry
  promised. Examples: trust review for a mutating setup step, a
  policy block, a missing remote agent for a live deep link, an
  import compare required before restore, or a profile lock that
  prevents the activation under the current envelope.
- `recommended_soon` — tasks that improve fidelity but MUST NOT
  monopolise plain editing or safe inspection. Examples: dependency
  restore, extension recommendation, package-manager selection, index
  warmup, devcontainer build.
- `optional_later` — additive tasks. Examples: docs import, AI
  context warmup, nonessential extension install, optional test
  discovery, layout customisation.

### 4.2 Blocked-reason classes (closed)

Every task in the `blocking_now` bucket MUST declare exactly one
`blocked_reason_class`. Free-form "Not available" copy is
non-conforming.

- `blocked_by_trust` — the task requires a trust grant the user has
  not yet given. Pairs with `trust_review_class =
  trust_review_pending` or `trust_revalidation_required` and a
  trust-prompt route.
- `blocked_by_policy` — admin / fleet / managed policy narrows the
  action. The task MUST also carry one or more
  `policy_block_refs[]` so the rendered card cites the policy
  source.
- `blocked_by_missing_prerequisite` — a runtime, package manager,
  container, kernel, network, or remote agent the task depends on is
  unavailable. The task MUST cite the missing prerequisite via its
  `summary` and (where applicable) `execution_boundary`.
- `blocked_by_deployment_profile` — the active deployment profile,
  template profile lock, or managed-envelope profile constraint
  prevents the task. Required when the profile baseline is
  `locked_profile_required` per the project-entry contract or when
  a managed envelope pins a profile that the current activation
  cannot satisfy.

A `blocking_now` task that lacks `blocked_reason_class` is
non-conforming. A surface that renders the same task across two
different reason classes (e.g. styling a policy block as a missing
prerequisite) is non-conforming.

### 4.3 Optional-reason classes (closed)

Every task in `optional_later` MUST declare exactly one
`optional_reason_class`. Tasks in `recommended_soon` MAY declare
one; if absent the surface MUST treat the task as
`recommended_for_fidelity`.

- `optional_additive` — purely additive task that does not gate any
  promised resulting mode.
- `optional_recommended_only` — surfaced only because a
  recommendation cited it; the user can dismiss without consequence.
- `optional_user_dismissed` — already dismissed; surfaced for
  inspectability and re-enable.
- `optional_freshness_based` — surfaced because a freshness signal
  (mirror lag, snapshot stale) suggests a refresh; not required for
  the current activation.

### 4.4 Blocked-vs-optional separation invariants

1. **Distinct objects.** A blocked task and an optional task are
   different objects. A surface that renders a single ambiguous row
   that resolves into either bucket is non-conforming.
2. **No collapse to generic failure copy.** A blocked task MUST
   render with its `blocked_reason_class` cited. Replacing the
   typed reason with "Setup failed", "Couldn't complete setup", or
   "Try again later" is non-conforming.
3. **No optional-disguising of blocked.** A surface that pushes a
   blocked task into `optional_later` (or the dismissable hint
   surface) to make the empty `blocking_now` bucket appear cleaner
   is non-conforming.
4. **No blocked-promotion of optional.** A surface that lifts an
   optional task into `blocking_now` to coerce setup is non-
   conforming. Recommendations that escalate must produce an
   archetype-detection-level outcome (`policy_blocks_setup`,
   `policy_blocks_trust_widening`, `policy_requires_managed_path`)
   or a missing-prerequisite signal first.
5. **Plain open survives an empty-bucket workspace.** A checkpoint
   with empty buckets MUST still set `plain_open_available = true`
   and `ordinary_editing_available = true`, except when the
   admission class is `policy_blocked` or `needs_repair`. In those
   cases the checkpoint MUST cite at least one
   `blocking_now` task and one `policy_block_ref` /
   `restore_prompt_ref` so the empty appearance is not misread.
6. **Set-up-later is a narrowing hint, not a hide.** Choosing
   `Set up later` MAY remove a task from a card's foreground but
   MUST NOT remove it from the underlying readiness bucket. The
   checkpoint MUST keep the deferred task ref reachable via
   `optional_guidance_refs` or the project-doctor reminder surface
   per the archetype-detection set-up-later contract.

### 4.5 Readiness-bucket summary (required)

The checkpoint carries a `readiness_bucket_summary` with the per-
bucket counts of tasks and (per bucket) the per-reason-class counts.
A surface that renders only the totals without exposing the typed
reason counts is non-conforming.

- `blocking_now_total` and `blocking_now_by_reason` (one count per
  `blocked_reason_class`).
- `recommended_soon_total` and `recommended_soon_by_class` (count
  per `readiness_task_class`).
- `optional_later_total` and `optional_later_by_reason` (count per
  `optional_reason_class`).

## 5. Admission Cards and Banners

Every checkpoint emits one banner and zero or more cards. Together
they explain — at a glance — what works now, what setup is
recommended, what is blocked, and what was deferred. The card and
banner sets are closed; a surface that mints a generic card or
banner is non-conforming.

### 5.1 `admission_card_class` (closed)

The shell MUST emit one card per non-empty bucket plus a
`continue_without_card` whenever
`continue_without_class != dismiss_recommendation`. Cards MAY also
appear for empty buckets when the surface needs to narrate the
empty state ("No setup is required at this root").

- `what_works_now_card` — names what the user can do without running
  any setup. Required on every checkpoint whose
  `ordinary_editing_available = true`.
- `recommended_setup_card` — projects the `recommended_soon` bucket.
  Required when that bucket is non-empty.
- `blocked_setup_card` — projects the `blocking_now` bucket. Required
  when that bucket is non-empty. The card MUST list every
  `blocked_reason_class` represented.
- `deferred_setup_card` — projects deferred tasks (set-up-later,
  dismissed recommendations, optional later items grouped under
  `optional_user_dismissed`). Required when there is at least one
  deferred task; otherwise omitted.
- `policy_narrowing_card` — projects the `policy_block_refs[]` and
  the narrowed capability classes. Required when the policy section
  is non-empty.
- `prerequisite_repair_card` — projects missing-prerequisite tasks
  and names where the repair runs. Required when at least one
  `blocking_now` task carries `blocked_reason_class =
  blocked_by_missing_prerequisite`.
- `deployment_profile_card` — projects deployment-profile blocks and
  the override / unlock affordance. Required when at least one
  blocking task carries `blocked_reason_class =
  blocked_by_deployment_profile`.
- `continue_without_card` — projects the resolved
  `continue_without_class` and the safe fallback affordance.
  Required whenever the checkpoint has a non-empty
  `blocking_now` bucket OR the user landed via a verb whose
  trust posture is not `trust_pending_until_admission` or
  `no_trust_review_required`.
- `optional_recommendations_card` — projects `optional_later` tasks
  the user has not yet dismissed. Optional, but required when the
  `optional_later` bucket is non-empty and the
  `deferred_setup_card` is not surfacing those entries.

### 5.2 `admission_card_record` required fields

- `record_kind = admission_card_record`.
- `admission_card_schema_version = 2`.
- `admission_card_id` — opaque, stable.
- `admission_checkpoint_ref` — opaque ref to the checkpoint.
- `admission_card_class` — closed set (§5.1).
- `referenced_task_refs[]` — task refs from the underlying readiness
  bucket. Required for every card other than `what_works_now_card`,
  `policy_narrowing_card`, `deployment_profile_card`, and
  `continue_without_card` (those carry their own ref arrays — see
  next two fields).
- `referenced_policy_block_refs[]` — required on
  `policy_narrowing_card` and on `blocked_setup_card` rows whose
  represented blocked-reason set includes `blocked_by_policy`.
- `referenced_recommendation_refs[]` — optional on every card, used
  to anchor recommendation provenance.
- `bypass_action` — re-export of the archetype-detection bypass-
  action set. Required on `continue_without_card`,
  `deferred_setup_card`, `policy_narrowing_card`, and any card whose
  underlying tasks have a typed bypass.
- `presentation_label` — redaction-aware text (≤ 1024 graphemes).
- `summary` — redaction-aware text (≤ 1024 graphemes).

Optional fields: `card_severity_class`
(`informational`, `recommendation`, `attention_required`,
`blocked`), `disabled_reason_code`, `keyboard_reachable` (default
`true`).

### 5.3 `admission_banner_class` (closed)

Every checkpoint emits exactly one banner. The banner is the
single-string shell-wide summary the status bar, attention inbox,
and notifications resolve into. The set is closed.

- `ready_to_work_banner` — empty `blocking_now`, ordinary editing
  available, no policy narrowing.
- `recommended_setup_banner` — `recommended_soon` non-empty,
  `blocking_now` empty.
- `partial_setup_banner` — `blocking_now` non-empty but at least one
  `plain_open_available = true` fallback survives.
- `blocked_setup_banner` — `blocking_now` non-empty AND
  `ordinary_editing_available = false`.
- `policy_restricted_banner` — `admission_class = policy_blocked`.
- `prerequisite_missing_banner` — at least one
  `blocked_by_missing_prerequisite` task.
- `deployment_profile_locked_banner` — at least one
  `blocked_by_deployment_profile` task.
- `deferred_setup_banner` — `blocking_now` empty AND at least one
  deferred or dismissed task; the banner narrates the deferred set.
- `mixed_setup_banner` — readiness tasks span more than one
  `setup_location_class`; the banner cites the locations.
- `no_safe_fallback_banner` — `continue_without_class =
  no_continue_without_action_available`. Reserved for the rare case
  where every fallback is itself blocked.

### 5.4 `admission_banner_record` required fields

- `record_kind = admission_banner_record`.
- `admission_banner_schema_version = 1`.
- `admission_banner_id` — opaque, stable.
- `admission_checkpoint_ref`.
- `admission_banner_class` — closed set (§5.3).
- `presentation_label` — redaction-aware text (≤ 1024 graphemes).
- `summary` — redaction-aware text (≤ 1024 graphemes).
- `bypass_action` — the resolved
  `continue_without_class` from the checkpoint.
- `severity_class` — `informational`, `recommendation`,
  `attention_required`, `blocked`.
- `keyboard_reachable` — boolean; required `true`.

Optional fields: `attention_inbox_routing_ref`, `status_bar_chip_ref`,
`notification_event_ref`.

### 5.5 Admission cards and banners — pre-side-effect invariants

A surface that violates any of the following is non-conforming:

1. **No side effect from card / banner render.** Rendering an
   admission card or banner MUST NOT install packages, contact
   remotes the entry did not already require, attach runtimes,
   widen trust, or retarget the active profile. Cards / banners
   are inspectable previews, not commit affordances.
2. **No silent dismissal of blocked.** A surface that allows the
   user to dismiss a `blocked_setup_card` /
   `blocked_setup_banner` without showing the blocked-reason class
   and the underlying task / policy ref is non-conforming. A
   `Hide for now` action that removes a `blocked_setup_card` MUST
   re-surface the same card on the next entry until the underlying
   block is resolved.
3. **No generic failure copy.** Cards and banners MUST cite the
   typed reason class. Free-form copy ("Setup failed",
   "Couldn't get ready", "Try again") is non-conforming.
4. **Mirrors the checkpoint.** Each card's
   `referenced_task_refs[]` / `referenced_policy_block_refs[]` MUST
   resolve into the checkpoint's blocked-setup or optional-guidance
   ref arrays. Cards that introduce orphan refs are non-conforming.
5. **Banner reflects the worst surviving state.** When more than
   one banner class is plausible, the banner resolves to the most
   blocking one (the order in §5.3 is normative).
6. **Continue-without survives.** A card or banner that hides the
   `continue_without_class` affordance is non-conforming. The
   `continue_without_card` MUST be keyboard-reachable and
   announceable.

## 6. Cross-Surface Invariants

The checkpoint, cards, and banners are projected into every shell
surface that explains the post-entry state. The following invariants
keep that projection sound:

1. **One checkpoint per entry activation.** Every entry activation
   that mutates trust, schedules setup, narrows capability, or
   invokes a restore / migration emits exactly one
   `workspace_admission_checkpoint_record`. A surface that emits
   two checkpoints for the same `entry_action_ref` is non-
   conforming.
2. **One banner per checkpoint.** A second banner on the same
   checkpoint is non-conforming. Card families may grow (§5.1), but
   the banner is single-valued.
3. **Same record across surfaces.** Start Center, the workspace
   switcher, the project doctor / attention inbox, the status bar,
   notifications, the post-clone handoff card, the post-import
   handoff card, the deep-link intent review, the CLI / headless
   `--explain` text block, and support exports MUST resolve to the
   same `admission_checkpoint_id`. Surface chrome (icon, accelerator,
   accent) MAY differ; semantics MUST NOT.
4. **CLI / headless render the same axes.** A CLI `--explain`
   text block MUST render `archetype_recommendation_source_classes[]`,
   `setup_location_classes[]`, `readiness_bucket_summary`,
   `blocked_reason_class` per blocking task,
   `optional_reason_class` per optional task, and the resolved
   `continue_without_class`. Collapsing axes (e.g. omitting
   `setup_location_classes[]`) is non-conforming.
5. **Support export keeps reasons typed.** A support export of the
   admission checkpoint MUST preserve the typed reason classes. A
   support export that flattens the typed reasons into a free-form
   "Setup failed" body is non-conforming.
6. **Notifications quote the banner class.** A notification that
   surfaces post-entry state MUST quote the banner class and the
   `continue_without_class`; opaque "Setup needs attention" copy
   without the class chip is non-conforming.
7. **Set-up-later does not widen authority.** The set-up-later /
   remembered-routing invariants from the archetype-detection
   contract apply unchanged. A checkpoint that records a
   set-up-later choice is a narrowing hint; it never widens trust,
   suppresses required review, or installs packages.
8. **Continue-without is verb-bound.** Per the project-entry
   contract §5.2 `fallback_disclosure`, the `continue_without_class`
   resolved here MUST be drawn from the open-flow sheet's typed
   fallback set. A checkpoint that resolves to a fallback the
   open-flow sheet did not promise is non-conforming.
9. **Recommendation provenance is required.** Every recommendation
   surfaced via an admission card MUST trace to one or more
   `archetype_recommendation_source_classes[]` on the checkpoint.
   A recommendation without a source class is non-conforming and
   MUST be denied upstream.

## 7. Fixture corpus

The fixture corpus under `/fixtures/workspace/admission_cases/`
contains one worked YAML record per scenario; each fixture is a
`workspace_admission_checkpoint_record`, an `admission_card_record`,
or an `admission_banner_record`. Each YAML carries a `__fixture__`
prelude naming the scenario, the records exercised, and the contract
sections asserted.

Required scenarios:

- `certified_repo_admitted_with_recommendations.yaml` —
  `admitted` checkpoint, empty `blocking_now`, non-empty
  `recommended_soon`, `ready_to_work_banner` /
  `recommended_setup_banner` pair, archetype source =
  `detected_facts` + `bundle_metadata`.
- `restricted_clone_policy_blocked_setup.yaml` — clone landed but
  policy blocks dependency restore, blocked-reason
  `blocked_by_policy`, banner
  `policy_restricted_banner`, `continue_without_class =
  continue_in_restricted_mode`.
- `missing_remote_agent_blocked_prerequisite.yaml` — deep-link
  arrival without a remote agent, blocked-reason
  `blocked_by_missing_prerequisite`, banner
  `prerequisite_missing_banner`, `continue_without_class =
  inspect_only`, `root_identity_class =
  no_root_identity_yet`.
- `deployment_profile_locked_blocked.yaml` — managed envelope pins
  a required profile the user lacks; blocked-reason
  `blocked_by_deployment_profile`, banner
  `deployment_profile_locked_banner`, `continue_without_class =
  open_minimal`.
- `mixed_workspace_boundary_choice_pending.yaml` — mixed-or-
  ambiguous detection, `blocking_now` carries
  `user_boundary_choice` with reason `blocked_by_trust` (the user
  must scope the workspace before any setup commits), banner
  `partial_setup_banner`.
- `generic_workspace_optional_only.yaml` — unknown / generic
  workspace, empty `blocking_now`, only optional later items,
  banner `ready_to_work_banner`, `continue_without_class =
  set_up_later`.
- `imported_handoff_compare_admitted_partial.yaml` — import
  handoff packet, archetype source `import_packet` +
  `bundle_metadata`, `compare_before_restore` continue-without,
  banner `recommended_setup_banner`, post-entry handoff card
  ref preserved.
- `remembered_route_set_up_later_admission.yaml` — remembered
  route applies as a narrowing hint, banner
  `deferred_setup_banner`, `continue_without_class =
  set_up_later`, archetype source `prior_user_choice`.
- `no_safe_fallback_blocked.yaml` — every fallback is itself
  blocked, banner `no_safe_fallback_banner`,
  `continue_without_class =
  no_continue_without_action_available`.

## 8. Versioning and change control

The schema declares
`admission_checkpoint_schema_version = 2`,
`admission_card_schema_version = 2`, and
`admission_banner_schema_version = 1`. Adding a new
`root_identity_class`, `trust_review_class`,
`archetype_recommendation_source_class`, `setup_location_class`,
`continue_without_class`, `blocked_reason_class`,
`optional_reason_class`, `admission_card_class`, or
`admission_banner_class` is **additive-minor** and bumps the
corresponding `*_schema_version`. Repurposing an existing value,
weakening any §4.4 or §5.5 invariant, or relaxing the
"blocked vs optional are different objects" rule is **breaking** and
requires a new ADR row plus a coordinated update of the archetype-
detection contract, the project-entry contract, and the entry-restore
object model.
