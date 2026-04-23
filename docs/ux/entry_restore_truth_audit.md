# Entry / restore placeholder truth audit

This audit is the reviewer-facing companion to
[`/artifacts/ux/startup_state_copy_review.yaml`](../../artifacts/ux/startup_state_copy_review.yaml)
and the seed corpus under
[`/fixtures/ux/entry_restore_states/`](../../fixtures/ux/entry_restore_states/).
It names every first-run, reopen, and restore-adjacent shell state
that the dogfood shell is permitted to render before the full
Start Center, recovery ladder, and migration-center surfaces ship,
and freezes — per state — the **truthful cause**, the
**user impact**, the **next-safe action**, the **blocked
capability**, and the **support / recovery / measurement
linkage** each state owes its user.

No audited state is allowed to imply that the workspace, restore,
or index is fully ready when the underlying state is only
warming, partial, degraded, or skipped. The audit is normative
at the copy-and-state level; where it disagrees with the PRD,
TAD, TDD, UI/UX spec, or milestone document, those sources win
and this document plus its companion artifacts MUST update in
the same change.

## Who reads this audit

- **Designers** sizing first-run, reopen, and restore copy — so
  chips, banners, and placeholders are written against the same
  truthful-state vocabulary across Start Center, recent-work
  cards, restore sheets, and empty editors.
- **Docs writers** attributing help topics to the same state
  names the shell renders, so "what does *Compatible restore* /
  *warming* / *open without restore* mean?" resolves to one
  answer.
- **Implementation owners** wiring the shell spike, entry /
  restore records, and later telemetry — so an engineer does
  not mint private state names that later diverge from support-
  bundle and journey-trace taxonomy.

## 1. Scope

- Freeze one audit row per placeholder startup state the
  dogfood shell may surface: **first run**, **reopen with
  pending restore**, **restore failed**, **restore skipped**,
  **open without restore**, **warming startup**, **partial
  startup**, **offline startup**, **unsupported startup**, and
  **empty-state or placeholder transitions**.
- Every row names exactly one truthful cause, one
  user-impact label class, at least one next-safe action hook
  (from the `next_step_decision_hook` closed set in
  [entry_restore_object_model §1.7](../workspace/entry_restore_object_model.md)),
  one blocked-capability token class, at least one link to a
  recovery-ladder rung or support-packet family, at least one
  measurement hook (journey-trace or protected-metric id), and
  the fixture file under
  [`/fixtures/ux/entry_restore_states/`](../../fixtures/ux/entry_restore_states/)
  that exercises it.
- Surfaces quote the row by id
  (`startup_state:<state_token>`). Copy-only shadow registries
  or free-form placeholder labels are non-conforming.

## 2. Out of scope

- Final user-facing microcopy. The rows pin the axes each
  placeholder must name; the shell-interaction-safety contract
  and the UX style guide own the exact words.
- Dogfood telemetry rollout. This audit reserves measurement
  hooks for later journey traces and protected-metric rows; no
  wire format is chosen here.
- New shell-level states. Every row resolves to vocabulary
  already frozen in
  [`entry_restore_object_model.md`](../workspace/entry_restore_object_model.md),
  [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md),
  [`recovery_ladder_packet.md`](../support/recovery_ladder_packet.md),
  and the onboarding measurement plan. A row that needs a
  token outside those vocabularies opens a new decision row
  rather than landing here.

## 3. Frozen vocabulary (re-exported)

The audit names no states of its own. Every row resolves to
values from:

- `entry_verb`, `target_kind`, `resulting_mode`,
  `admission_class`, `next_step_decision_hook` —
  [`entry_restore_object_model.md` §1](../workspace/entry_restore_object_model.md).
- `target_state`, `restore_availability`, `safe_recovery_action`
  — `§2`.
- `restore_level`, `missing_target_state`,
  `session_execution_posture`,
  `checkpoint_linked_recovery_class` — `§3`–`§4`.
- `entry_route_id` (`er.*`), `readiness_bucket`
  (`blocking_now` / `recommended_soon` / `optional_later`),
  measurement-surface failure categories — the
  [onboarding measurement plan §3 and §4](../product/onboarding_measurement_plan.md).
- `journey_class`, `checkpoint_class`, `degraded_posture_class`,
  `fallback_posture_class` — the
  [`journey_trace.schema.json`](../../schemas/traces/journey_trace.schema.json)
  boundary.
- Recovery-ladder rung ids (`rung.safe_mode`,
  `rung.extension_quarantine`, `rung.open_without_restore`,
  `rung.cache_index_repair`, `rung.restricted_mode_fallback`)
  — the
  [recovery-ladder packet](../support/recovery_ladder_packet.md).

## 4. Truthfulness posture (normative)

Every audited row obeys the following posture. A surface whose
rendered placeholder violates any of these rules is non-
conforming:

1. **No "Ready" overclaim.** A placeholder state that is not
   `fully_present` / `reachable` / `exact_restore` MUST NOT
   carry a label, chip, or banner that reads as ready. Labels
   like "Workspace ready", "All extensions loaded", "Index
   complete", or "Session restored" are forbidden when the
   underlying state is `warming`, `partial`, `degraded`,
   `skipped`, `restore_failed`, or an equivalent non-`reachable`
   token.
2. **Cause named, not implied.** Every non-ready state names
   its truthful cause token (e.g., `missing_extension_host`,
   `corrupt_restorable_state`, `authority_expired_for_remote`,
   `topology_changed`, `policy_blocked_restore`) before the
   user commits to a next action. Silent classification into
   a generic "Something went wrong" is non-conforming.
3. **User impact is labelled.** Every row names what the user
   loses or must defer right now — not just what Aureline is
   doing internally. Labels are drawn from the closed impact
   class set in §5.
4. **Next-safe action is typed.** Every row cites at least one
   `next_step_decision_hook`; free-form action text
   (`Try again`, `Continue`) is non-conforming. The hook is
   the safest path the user can take without further
   classification work.
5. **Blocked capability is named.** Every non-ready row names
   the capability class the placeholder blocks
   (`no_durable_edits_yet`, `terminal_transcripts_not_rerun`,
   `debug_session_ended`, `extension_host_offline`,
   `index_not_authoritative`, `remote_target_unreachable`,
   `policy_restricted_mode`, `migration_rollback_pending`,
   `start_from_snapshot_setup_pending`,
   `no_blocking_capability`). A label that forecloses
   capability the underlying state still allows is over-
   blocking; a label that promises capability the state does
   not support is over-claiming. Both are non-conforming.
6. **Recovery and support links are durable.** Every row cites
   at least one recovery-ladder rung id and at least one
   support-packet family. The shell renders the rung-linked
   verb verbatim per the recovery-ladder contract; `Undo` is
   forbidden on `compensating_rollback` /
   `regenerate_from_canonical_source`.
7. **Measurement hooks are reserved.** Every row reserves at
   least one journey-trace `journey_class` / `segment_class`
   and at least one protected-metric row (`ff.*`) that a
   later dogfood telemetry lane can fire against without
   inventing names.

## 5. User-impact label classes (closed)

Every audited row names exactly one `user_impact_label_class`.
The set is closed; additive-minor changes land in this audit
and in the companion YAML copy-review in the same change.

- `no_durable_work_yet` — user has not yet reached a surface
  that would persist work. Appropriate for first-run and
  empty-state transitions.
- `pending_user_review` — the shell is waiting on the user to
  triage restore / migration / recovery before any durable
  work runs.
- `work_continues_with_narrowed_capability` — the user can
  keep working locally, but a named capability is narrowed
  (extension host offline, remote unreachable, restricted
  mode, `evidence_only` restore).
- `prior_work_preserved_but_not_rerun` — dirty buffers,
  terminal transcripts, or notebooks are preserved but
  mutating commands did not rerun.
- `prior_work_not_recoverable_by_automation` — a state where
  the user must choose between `open_without_restore`,
  `safe_mode`, or `locate_missing_target` because automated
  recovery is exhausted.
- `no_recovery_on_irreversible_path` — the irreversible high-
  blast path cited a `no_recovery_available` revert class;
  reserved for audit parity with the shell-interaction-safety
  contract (no startup state lands here today).

## 6. Audit rows

Each row follows the schema:
**State token** · truthful cause · user impact · next-safe
action · blocked capability · recovery-ladder rung · support-
packet family · measurement hook · fixture.

### 6.1 `startup_state:first_run`

The no-prior-profile / no-prior-session path on a device that
has never opened Aureline (or has explicitly reset first-run).

- **Truthful cause tokens.** `entry_verb = open` (or the user
  has not yet committed one), `target_kind` not yet resolved
  (candidate `target_kind_unresolved` → denies into Start
  Center), `admission_class = admitted`,
  `authority_reevaluation_required = false`.
- **User impact.** `no_durable_work_yet`.
- **Next-safe actions.** `review_archetype_match`,
  `set_up_later`, `open_minimal`. The Start Center MUST keep
  `Open`, `Clone`, `Import`, `Restore`, and `Recent work`
  verbs distinct — a `Get started` collapse is non-conforming
  per the entry-restore object model §7.
- **Blocked capability.** `no_durable_edits_yet`. No banner
  claims an index is ready; the editor-with-root-discovery-cues
  surface is the first-useful-work target.
- **Recovery / support.** `rung.restricted_mode_fallback`
  (if `continue_in_restricted_mode` is chosen);
  support-packet family `first_run_evidence` (reserved).
- **Measurement.** `journey_class = startup_to_first_useful_chrome`
  and `startup_to_first_paint`; protected metrics
  `ff.warm_start_to_first_paint` and `ff.first_paint`; entry
  route `er.start_center`; onboarding measurement plan
  surface `surface_first_run` with failure category set
  (`forced_sign_in_before_useful_local_work`,
  `entry_verb_collapsed_into_get_started`,
  `start_center_unreachable`).
- **Fixture.**
  [`first_run.yaml`](../../fixtures/ux/entry_restore_states/first_run.yaml).

### 6.2 `startup_state:reopen_with_pending_restore`

The shell restarted and a `restore_prompt_record` is pending
user decision. The old session is not yet materialised; the
shell MUST NOT imply it is already restored.

- **Truthful cause tokens.** `entry_verb = restore`
  (candidate) or `entry_verb = open` with
  `restore_prompt_record` emitted; `restore_level` advertised
  (one of `exact_restore` / `compatible_restore` / `layout_only`
  / `recovered_drafts` / `evidence_only`) but not yet
  committed; `missing_target_states` enumerated.
- **User impact.** `pending_user_review`.
- **Next-safe actions.** `compare_before_restore`,
  `open_without_restore`, `safe_mode`. Copy MUST name the
  advertised restore level verbatim; promising a higher level
  than the prompt will deliver is non-conforming
  (entry-restore model §7.4).
- **Blocked capability.** `index_not_authoritative` (until
  restore commits); `terminal_transcripts_not_rerun` and
  `debug_session_ended` for any pane whose
  `session_execution_posture` is not
  `live_session_continued`.
- **Recovery / support.** `rung.open_without_restore` and
  `rung.safe_mode`; support-packet families
  `restore_prompt_evidence`, `recovery_ladder_packet`.
- **Measurement.** `journey_class = restore_adjacent`; entry
  route `er.restore_prompt`; measurement surface
  `surface_restore_success` with failure categories
  `restore_level_promised_higher_than_delivered`,
  `live_session_inferred_from_absence`.
- **Fixture.**
  [`reopen_with_pending_restore.yaml`](../../fixtures/ux/entry_restore_states/reopen_with_pending_restore.yaml).

### 6.3 `startup_state:restore_failed`

The shell attempted a restore and the restore did not deliver
the advertised level (e.g., corrupt layout, failed checkpoint
read, schema-migrated content refused to rehydrate).

- **Truthful cause tokens.** `restore_level = no_restore` or
  the advertised level degraded to a lower level at
  materialisation; `missing_target_states` includes
  `corrupt_restorable_state` or `schema_migrated`;
  `checkpoint_linked_recovery_class = manual_recovery_required`
  or `restore_from_recovery_journal` for dirty buffers.
- **User impact.** `prior_work_not_recoverable_by_automation`.
- **Next-safe actions.** `open_without_restore`, `safe_mode`,
  `locate_missing_target`. `compare_before_restore` is
  retained when a compare / export path exists.
- **Blocked capability.** `index_not_authoritative`,
  `prior_work_requires_manual_recovery`.
- **Recovery / support.** `rung.open_without_restore`,
  `rung.cache_index_repair`, `rung.safe_mode`; support-packet
  families `crash_recovery_evidence`, `recovery_ladder_packet`,
  `object_issue_handoff`.
- **Measurement.** `journey_class = restore_adjacent`;
  `degraded_posture_class = missing_target_recovered_to_layout_only`
  or `missing_target_recovered_to_compatible`; measurement
  surface `surface_restore_success` with failure categories
  `restore_level_promised_higher_than_delivered`,
  `corrupt_restorable_state_silently_discarded`,
  `dirty_buffer_lost_without_journal`.
- **Fixture.**
  [`restore_failed.yaml`](../../fixtures/ux/entry_restore_states/restore_failed.yaml).

### 6.4 `startup_state:restore_skipped`

The user declined the restore prompt, or a policy / admin
decision skipped the restore. The shell opens clean (or in a
narrowed mode) and MUST NOT imply that restore ran silently.

- **Truthful cause tokens.** `entry_verb = open` with the
  preceding `restore_prompt_record.next_step_decision_hooks`
  resolved to `open_without_restore`, or
  `missing_target_states` includes `policy_blocked_restore`.
- **User impact.** `work_continues_with_narrowed_capability`.
- **Next-safe actions.** `open_without_restore`,
  `review_trust_and_open`, `set_up_later`.
- **Blocked capability.** `prior_work_preserved_as_evidence_only`;
  `terminal_transcripts_not_rerun`,
  `debug_session_ended` for any prior session; recovery
  journal remains available.
- **Recovery / support.** `rung.open_without_restore`,
  `rung.restricted_mode_fallback` (when policy-blocked);
  support-packet families `restore_prompt_evidence`,
  `policy_audit_evidence`.
- **Measurement.** `journey_class = restore_adjacent`; entry
  route `er.restore_prompt` (when user-declined) or
  `er.plain_open` (when the user bypassed the prompt);
  measurement surface `surface_restore_success` with failure
  category `live_session_inferred_from_absence` specifically
  forbidden.
- **Fixture.**
  [`restore_skipped.yaml`](../../fixtures/ux/entry_restore_states/restore_skipped.yaml).

### 6.5 `startup_state:open_without_restore`

The explicit `open_without_restore` escape-hatch on a restore-
adjacent path. Distinct from `restore_skipped` because the
user made an *active* decision to discard / defer restore in
favour of reaching useful work now; surfaces quote this choice
on support exports so "I opened without restoring" is an
audit-record outcome, not a guess.

- **Truthful cause tokens.** `entry_verb = open`;
  `next_step_decision_hook = open_without_restore`; prior
  `restore_prompt_record` retained for evidence, not
  materialised.
- **User impact.** `work_continues_with_narrowed_capability`.
- **Next-safe actions.** `open_without_restore`,
  `compare_before_restore` (retained as a later escape hatch).
- **Blocked capability.** `prior_work_preserved_as_evidence_only`;
  `index_not_authoritative` until warming completes.
- **Recovery / support.** `rung.open_without_restore`,
  `rung.cache_index_repair`; support-packet families
  `restore_prompt_evidence`, `recovery_ladder_packet`.
- **Measurement.** `journey_class = restore_adjacent` +
  `startup_to_first_useful_chrome`; entry route
  `er.restore_prompt`; measurement surface
  `surface_restore_success` with failure category
  `live_session_inferred_from_absence` forbidden. Support
  export names the user's decision verbatim.
- **Fixture.**
  [`open_without_restore.yaml`](../../fixtures/ux/entry_restore_states/open_without_restore.yaml).

### 6.6 `startup_state:warming_startup`

The shell is interactive (editor reachable, buffer editable)
while semantic warm-up, index build, language-server attach,
or extension host activation is still running. The shell MUST
render the warming posture explicitly; a label implying a
fully-warm state is non-conforming.

- **Truthful cause tokens.** `first_useful_navigation_reached`
  has fired but `semantic_warmup_completed` has not;
  `target_state = stale_metadata` candidate on recent-work
  rows until warm-up resolves.
- **User impact.** `work_continues_with_narrowed_capability`.
- **Next-safe actions.** `open_minimal`, `set_up_later`,
  `continue_in_restricted_mode` (when the warming path
  intersects a trust / policy gate).
- **Blocked capability.** `index_not_authoritative`,
  `semantic_lookups_pending`,
  `extensions_not_yet_activated` (where applicable). The
  editor, buffer, save pipeline, and recovery journal remain
  durable per onboarding plan §3.3.
- **Recovery / support.** `rung.cache_index_repair` (on
  repeated warm-up failure); support-packet family
  `performance_evidence_packet`.
- **Measurement.** `journey_class = startup_to_first_useful_chrome`
  and `open_edit_save`; segment classes `first_useful_chrome`,
  `placeholder_open`, `placeholder_edit`, `placeholder_save`;
  protected metrics `ff.warm_start_to_first_paint` and
  `ff.first_paint`; measurement surface
  `surface_first_useful_edit` with failure category
  `editor_blocked_on_index_warmup` specifically monitored.
- **Fixture.**
  [`warming_startup.yaml`](../../fixtures/ux/entry_restore_states/warming_startup.yaml).

### 6.7 `startup_state:partial_startup`

Part of the shell came up; part did not. An extension host
crashed during activation, a language server failed to attach,
a remote file provider is unavailable, or the managed
workspace lifecycle is
`warming` / `recovering` / `quarantined` rather than `ready`.

- **Truthful cause tokens.** `missing_target_states` includes
  `missing_extension_host` / `quarantined_extension` /
  `missing_toolchain` / `missing_managed_workspace` /
  `missing_devcontainer`; `target_state` on affected
  recent-work rows flips to `stale_metadata` or
  `mode_downgraded`.
- **User impact.** `work_continues_with_narrowed_capability`.
- **Next-safe actions.** `safe_mode`,
  `continue_in_restricted_mode`, `locate_missing_target`,
  `open_minimal`.
- **Blocked capability.** `extension_host_offline`,
  `language_server_unattached`,
  `managed_workspace_not_ready`; durable edits remain
  available per §6.6.
- **Recovery / support.** `rung.extension_quarantine`,
  `rung.cache_index_repair`, `rung.restricted_mode_fallback`;
  support-packet families `recovery_ladder_packet`,
  `object_issue_handoff`, `managed_workspace_evidence`.
- **Measurement.** `journey_class = shell_open` +
  `startup_to_first_useful_chrome`;
  `degraded_posture_class = reduced_chrome_only` or
  `responsive_fallback_active`; `fallback_posture_class`
  candidates include `software_renderer_active`,
  `recovery_journal_replay_active`; measurement surface
  `surface_first_open` with failure categories
  `admission_denied_needs_repair`,
  `resulting_mode_silently_downgraded` forbidden.
- **Fixture.**
  [`partial_startup.yaml`](../../fixtures/ux/entry_restore_states/partial_startup.yaml).

### 6.8 `startup_state:offline_startup`

The shell started without a reachable network for a route
that prefers or requires remote resources (remote repository
reopen, managed-workspace resume, provider-linked recent-work
row). Aureline MUST keep local-only work paths available.

- **Truthful cause tokens.** `target_state = remote_unreachable`
  or `authority_expired` on affected recent-work rows;
  `missing_target_states` includes `missing_remote_target` /
  `authority_expired_for_remote` on restore prompts; entry
  actions against `remote_repository` / `ssh_workspace` /
  `container_workspace` / `devcontainer_workspace` /
  `managed_cloud_workspace` target kinds return
  `admission_class = needs_reconnect` or `needs_reauth`.
- **User impact.** `work_continues_with_narrowed_capability`.
- **Next-safe actions.** `reconnect_required`,
  `reauth_required`, `continue_in_restricted_mode`,
  `set_up_later`.
- **Blocked capability.** `remote_target_unreachable`,
  `managed_workspace_not_ready`, `provider_paths_offline`.
  Local-only rows remain `reachable`; `portability_class =
  local_only` rows keep full capability.
- **Recovery / support.** `rung.restricted_mode_fallback`,
  `rung.open_without_restore`; support-packet families
  `auth_evidence_packet`, `recovery_ladder_packet`.
- **Measurement.** `journey_class = shell_open` +
  `startup_to_first_useful_chrome`; entry routes include
  `er.warm_start` and `er.recent_work_reopen`; measurement
  surface `surface_opt_in_boundary` and
  `surface_first_open` with failure category
  `network_required_for_local_entry` specifically forbidden
  (first-run) and `admission_denied_needs_reconnect` /
  `admission_denied_needs_reauth` rendered as truthful
  denial, not as a generic error.
- **Fixture.**
  [`offline_startup.yaml`](../../fixtures/ux/entry_restore_states/offline_startup.yaml).

### 6.9 `startup_state:unsupported_startup`

The device, platform, policy envelope, or prior version
combination does not support a full startup — the shell may
still open in a restricted mode, but MUST NOT imply that the
unsupported path is fine.

- **Truthful cause tokens.** `admission_class = policy_blocked`
  or `missing_target_states` includes `policy_blocked_restore`
  / `binary_or_extension_version_changed` /
  `schema_migrated`; `target_state = policy_blocked` /
  `mode_downgraded` on affected recent-work rows;
  archetype-detection outcome
  `unrecognised_archetype` or
  `archetype_detection_unavailable` per the task-success
  corpus vocabulary.
- **User impact.** `work_continues_with_narrowed_capability`
  or `prior_work_not_recoverable_by_automation` depending on
  whether a narrowed path remains.
- **Next-safe actions.** `continue_in_restricted_mode`,
  `review_trust_and_open`, `set_up_later`, `safe_mode`.
- **Blocked capability.** `policy_restricted_mode`,
  `unsupported_platform_feature`,
  `schema_migration_required`. Durable local-only edits
  remain available when the restricted mode admits them.
- **Recovery / support.** `rung.restricted_mode_fallback`,
  `rung.safe_mode`; support-packet families
  `policy_audit_evidence`, `recovery_ladder_packet`.
- **Measurement.** `journey_class = shell_open`;
  `degraded_posture_class = reduced_chrome_only`;
  measurement surface `surface_first_run` /
  `surface_first_open` with failure categories
  `forced_sign_in_before_useful_local_work`,
  `admission_denied_policy`, `admission_denied_trust`
  rendered as truthful denial.
- **Fixture.**
  [`unsupported_startup.yaml`](../../fixtures/ux/entry_restore_states/unsupported_startup.yaml).

### 6.10 `startup_state:empty_state_or_placeholder_transition`

Placeholder or transitional surfaces — empty Start Center on
a fresh profile, the zone-slot placeholder a
`missing_extension_placeholder` leaves behind, a returned-
focus placeholder announced after a surface disappears, a
protocol-handler-reentry card that has not yet resolved its
target, or the transition between a restore prompt and the
materialised workspace. These surfaces MUST NOT imply any
readiness the underlying state does not hold; they are the
most common source of ready-claim drift.

- **Truthful cause tokens.** Responsive-fallback mode
  `missing_extension_placeholder` or focus-return state
  `returned_placeholder_announced` per the shell-interaction-
  safety contract; `target_kind` not yet resolved on a
  protocol-handler-reentry; `resulting_mode` not yet
  committed; Start Center has zero recent-work rows and the
  user has not yet picked a verb.
- **User impact.** `no_durable_work_yet` (empty state) or
  `pending_user_review` (transitional placeholder awaiting
  commit).
- **Next-safe actions.** `locate_missing_target` (when a
  target was expected), `review_archetype_match`,
  `open_minimal`, `set_up_later`,
  `remove_from_recents` (stale recent-work).
- **Blocked capability.** `no_durable_edits_yet`,
  `placeholder_surface_no_capability`,
  `extension_host_offline` (when the placeholder represents
  a missing-extension zone).
- **Recovery / support.** `rung.extension_quarantine` (when
  the placeholder represents a quarantined extension);
  support-packet families `empty_state_audit_evidence`,
  `recovery_ladder_packet`.
- **Measurement.** `journey_class = startup_to_first_useful_chrome`;
  `checkpoint_class = provisional_segment_boundary` is
  permitted for transitional spans that a later taxonomy
  lane may rename;
  `segment_class = provisional_segment` is allowed when no
  protected-journey value fits yet.
- **Fixture.**
  [`empty_state_or_placeholder_transition.yaml`](../../fixtures/ux/entry_restore_states/empty_state_or_placeholder_transition.yaml).

## 7. Acceptance mapping

- **No overclaim.** Every row in §6 asserts
  `overclaims_readiness = false` and pins the blocked-
  capability axis that disproves overclaim. The companion
  copy-review YAML encodes this as a per-row invariant.
- **Machine-readable to support / recovery / journey-trace.**
  Every row names recovery-ladder rung ids (or
  `rung.none_required`), support-packet family refs,
  fixture path, `journey_class` candidates, and protected-
  metric row ids. The startup_state token is the stable key
  the three downstream families resolve against.
- **Shared names across designers / docs / implementation.**
  The startup_state token set is named in this document,
  re-exported in the copy-review YAML, quoted by fixture
  file name, and wired through the measurement-plan
  surfaces. No surface mints a private state token.

## 8. Source anchors

- `.t2/docs/Aureline_PRD.md:284` — first-run onboarding is a
  launch risk, not polish.
- `.t2/docs/Aureline_PRD.md:1293` — recovery-journal /
  autosave forward-readability.
- `.t2/docs/Aureline_PRD.md:1300` — exact session restore
  MUST degrade gracefully to recovered drafts to clean-open
  with evidence, never directly to silent loss.
- `.t2/docs/Aureline_PRD.md:1703` — recovery ladder: safe
  mode → extension quarantine → cache reset → restricted
  mode.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:762` — Start
  Center primary actions (`Open folder`, `Open workspace`,
  `Clone repository`, `Restore last session`, `Import
  from…`).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:836` — restore-
  class taxonomy.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:866` — restore-
  fidelity rules (`Exact restore` / `Compatible restore` /
  `Layout only` / `Recovered drafts` / `Evidence only`).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:1068` — project-
  entry, clone / import / open, and workspace-admission
  flows.
- `.t2/docs/Aureline_Milestones_Document.md:1023` — Start
  Center keeps `Open`, `Clone`, `Import`, `Restore`, and
  `Recent work` distinct with a no-account local path.

## 9. Linked artifacts

- Copy-review YAML:
  [`/artifacts/ux/startup_state_copy_review.yaml`](../../artifacts/ux/startup_state_copy_review.yaml).
- Fixture corpus:
  [`/fixtures/ux/entry_restore_states/`](../../fixtures/ux/entry_restore_states/).
- Entry / restore object model (source of truth for most
  tokens referenced here):
  [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md).
- Onboarding / first-useful-work measurement plan:
  [`/docs/product/onboarding_measurement_plan.md`](../product/onboarding_measurement_plan.md).
- Attention / activity taxonomy (placeholder, focus-return,
  missing-extension vocabularies):
  [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md).
- Shell-level interaction-safety contract (placeholder /
  focus-return / responsive-fallback posture across all
  protected surfaces):
  [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md).
- Recovery-ladder packet (safe mode, extension quarantine,
  open-without-restore, cache / index repair, restricted-
  mode fallback):
  [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md).
- Journey-trace schema (measurement hook target):
  [`/schemas/traces/journey_trace.schema.json`](../../schemas/traces/journey_trace.schema.json).
- Protected-metric registry:
  [`/artifacts/bench/protected_metrics.yaml`](../../artifacts/bench/protected_metrics.yaml).

## 10. Changing this audit

- Adding a new startup-state row is additive-minor and lands
  here and in the companion copy-review YAML plus a matching
  fixture in the same change. The new row MUST resolve every
  axis to vocabulary already frozen upstream — no new tokens.
- Repurposing an existing startup-state token is breaking and
  opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- Copy and microcopy updates that do not change the axis
  bindings live in this document's row bodies and the
  companion YAML; the shell-interaction-safety contract and
  the UX style guide remain the sources of truth for
  rendering.
