# Onboarding, first-useful-work, and migration measurement plan

This document defines how Aureline measures **switching success** —
first-run, first open, first useful edit, migration review, restore
success, and opt-in-versus-continue-local behaviour — starting from
the pre-implementation foundations milestone rather than after
launch. The account-free local-first path is a first-class product
claim; this plan makes it measurable rather than narrative.

The plan is the **shared truth** that later telemetry lanes, the
benchmark council, compatibility scoreboards, UX evidence packets,
and support / crash-report pipelines resolve against. At this
milestone nothing is collected yet — the plan freezes event names,
metric shapes, entry-route IDs, failure categories, owners, and the
seed corpus / scoreboard the later implementations MUST emit against.

Companion artifacts:

- [`/artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml)
  — task-success corpus seed covering first-run, open, clone, import,
  restore, resume, and start-from-snapshot scenarios with entry-route
  id, archetype-detection outcome, readiness bucket, first-useful-work
  target surface, restore class, and setup posture per scenario.
- [`/artifacts/product/no_account_switching_scoreboard_seed.yaml`](../../artifacts/product/no_account_switching_scoreboard_seed.yaml)
  — scoreboard seed for account-free local work, service-opt-in
  boundary outcomes, and intentionally narrowed startup / import
  capability.

This document is normative for the measurement shape. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, or the milestone
document, those sources win and this document MUST be updated in
the same change. Where a later telemetry, benchmark, or evidence
lane mints a parallel event family, this plan wins and the lane is
non-conforming.

Milestone / planning slugs do not appear in the event names, metric
names, entry-route IDs, corpus IDs, or scoreboard row IDs defined
here: those strings will eventually reach product chrome, support
bundles, and public evidence exports.

## 1. Why freeze this measurement plan now

1. Switching quality is a launch-bearing system. The milestone doc
   treats first run, migration, and early learnability as part of
   the replacement-grade claim — not as polish. If we wait until
   after launch to decide what "first useful edit" means, every
   surface that touches the journey (Start Center, recent-work,
   restore prompt, migration center, doctor, support bundle) will
   ship its own private success notion and the product will tell
   the user five different stories about the same outcome.
2. The no-account local-first path is *narrative* until we specify
   how to qualify it. Treating account-free local use as product
   truth requires a scoreboard that separates it from opt-in
   service paths and records when startup or import intentionally
   narrowed capability. Without that, "no account required" is an
   assertion; with it, it is a claim.
3. Entry routes are not interchangeable. A `Start Center` click,
   a recent-work reopen, a restore prompt, a protocol-handler
   reentry, a clone / import, and a plain `File > Open` resolve
   through different vocabularies (per the entry-restore object
   model §1.6) and collapsing them into one "first-run success"
   metric hides the failure modes the milestone doc explicitly
   names as release-bearing.
4. Measurement has to be observable before full semantic warm-up.
   `first-useful-navigation` is not `first-useful-edit`: the plan
   records both, in order, so the benchmark council can tell
   whether we reached the editor and buffer before the index was
   complete.

## 2. Scope

### In scope

- Success criteria, failure categories, event / metric names, and
  owners for six measurement surfaces:
  - first-run (the no-prior-profile path),
  - first open (every subsequent project-entry),
  - first useful edit (durable, user-originated buffer mutation),
  - migration review (dry-run diff, apply, rollback),
  - restore success (session / crash / topology restore),
  - opt-in-versus-continue-local (boundary crossings out of the
    account-free path).
- A frozen **entry-route taxonomy** every measurement surface reads.
- A frozen **readiness-bucket** set (`blocking_now`,
  `recommended_soon`, `optional_later`) that every corpus scenario
  classifies each setup action against, so scoreboards can separate
  "user is blocked" from "user chose to defer".
- The seed **task-success corpus** (checked-in YAML with concrete
  fixture repo references) and the seed **no-account switching
  scoreboard** (checked-in YAML scoreboard family definitions with
  worked rows).
- Linkage rules to the benchmark council charter and UX evidence
  families so the plan is cited, not re-derived, by downstream
  evidence.

### Out of scope

- Shipping telemetry collection. No event emitter is implemented
  at this milestone; the plan only reserves the names and shapes.
- Choosing a wire format or storage for the event stream.
  Telemetry payloads will follow the ADR-0005 subscription
  envelope and the support-bundle packet family when those land.
- Benchmark hardware, fixture hardware reference profiles, or
  statistical confidence rules beyond naming the packet families
  the council will rule on. The benchmark-council charter owns
  those rules.
- Account-free vs account-required packaging, pricing, or
  deployment-profile claims. The boundary-manifest strawman owns
  that classification; the scoreboard seed links to it rather
  than restating it.
- Final user-facing copy for readiness chips, restore badges, or
  migration outcomes. The entry-restore object model and
  shell-interaction-safety contract own the copy.

## 3. Measurement surfaces

Each surface has a **success criterion**, a closed **failure-category**
set, a **per-entry-route event family** (emitters MUST tag every
record with an entry-route ID from §4), an **owner**, and a
**link-out** to the downstream evidence pack that consumes it.

Event names use `snake_case` without milestone slugs. All event
names are reserved here; emitters added later MUST use the exact
strings below.

### 3.1 First-run (`surface_first_run`)

The no-prior-profile / no-prior-session path. The user has never
opened Aureline on this device (or has explicitly reset first-run).

| Field | Value |
|---|---|
| Success criterion | User reaches an admitted entry action record (entry_verb ∈ {`open`, `clone`, `start_from_snapshot`, `import`}) whose resulting mode is non-`inspect_only`, without being forced through account creation, marketplace detour, or non-dismissible tour. |
| Failure categories (closed) | `forced_sign_in_before_useful_local_work`, `forced_marketplace_detour`, `forced_tour_blocking_useful_work`, `start_center_unreachable`, `entry_verb_collapsed_into_get_started`, `network_required_for_local_entry`, `aborted_before_admission`. |
| Primary events | `first_run_reached`, `first_run_entry_route_selected`, `first_run_admitted`, `first_run_abandoned`, `first_run_failure_classified`. |
| Derived metrics | `first_run_success_rate{entry_route}`, `time_to_first_admission{entry_route}` (monotonic-duration packet per the governance chronology model), `first_run_account_free_rate`. |
| Owner (DRI) | product-scope forum chair (`@ahmeddyounis`) with docs_public_truth + support_export attestation. |
| Consumed by | benchmark-council Bootstrap / entry-parity scoreboard; UX evidence packet (first-run audit); support-bundle first-run summary. |

### 3.2 First open (`surface_first_open`)

Every post-first-run project-entry: the user opens, clones, imports,
restores, resumes, or starts-from-snapshot. A session may contain
many first-open records (one per `project_entry_action_record`);
the surface is always per-entry, not per-session.

| Field | Value |
|---|---|
| Success criterion | `project_entry_action_record` commits with `admission_class = admitted` and `resulting_mode` matches the user's declared intent (no silent downgrade to `inspect_only` / `open_prebuild_minimal` without a user-acknowledged bypass). |
| Failure categories (closed) | `target_kind_unresolved`, `admission_denied_policy`, `admission_denied_trust`, `admission_denied_needs_repair`, `admission_denied_needs_reconnect`, `admission_denied_needs_reauth`, `resulting_mode_silently_downgraded`, `destination_disposition_mismatch`, `collision_unreviewed_before_commit`. |
| Primary events | `entry_verb_resolved`, `target_kind_classified`, `resulting_mode_committed`, `admission_decided`, `first_open_completed`, `first_open_denied`. |
| Derived metrics | `open_success_rate{entry_verb, entry_route, target_kind}`, `time_from_intent_to_admission{entry_route}`, `admission_denial_rate{denial_class}`. |
| Owner (DRI) | shell_command_system lane chair (`@ahmeddyounis`). |
| Consumed by | benchmark-council Bootstrap / entry-parity scoreboard; migration-center evidence when verb = `import`; crash-recovery evidence when verb = `restore`. |

### 3.3 First useful edit (`surface_first_useful_edit`)

The first durable, user-originated buffer mutation that survives
save (or a deliberate in-memory-only decision). This surface is
observable **before** full semantic warm-up: it measures that the
editor, buffer, save pipeline, and recovery journal are ready even
if the semantic graph and index are still warming.

| Field | Value |
|---|---|
| Success criterion | A user-originated, non-stub-scaffold edit reaches a durable state (saved to workspace VFS, or explicitly-held dirty buffer with a recovery-journal entry) within an envelope that does not require index completeness. |
| Failure categories (closed) | `editor_blocked_on_index_warmup`, `save_blocked_on_service`, `recovery_journal_unavailable`, `edit_lost_before_journal`, `trust_gate_blocks_edit_without_explanation`, `buffer_read_only_unexplained`, `first_edit_required_sign_in`. |
| Primary events | `first_useful_navigation_reached` (reached an editable or readable surface before warm-up), `first_useful_edit_started`, `first_useful_edit_durable`, `first_useful_edit_blocked`. |
| Derived metrics | `time_to_first_useful_navigation{entry_route}`, `time_to_first_useful_edit{entry_route, target_kind}`, `useful_edit_before_warm_up_rate` (edits that landed before `semantic_warmup_completed`). |
| Owner (DRI) | aureline-buffer + aureline-vfs lane chairs (`@ahmeddyounis`) with architecture-council co-review. |
| Consumed by | benchmark-council Bootstrap / entry-parity and certified-archetype scoreboards; support-bundle performance summary. |

### 3.4 Migration review (`surface_migration_review`)

Dry-run diff, per-item outcome attribution, and rollback of an
import (competitor config, portable-state package, handoff packet,
bundle adoption). Every record reads a `migration_result_record`
per the entry-restore object model §5.

| Field | Value |
|---|---|
| Success criterion | The user sees a per-item outcome (`exact` / `translated` / `approximated` / `skipped` / `blocked` / `needs_review` / `rollback_available`) **before commit**, is offered at least one of the four migration next-step hooks (`roll_back_import`, `keep_imported_state`, `adopt_recommended_bundle`, `review_unsupported_items`), and — when rollback is used — the workspace returns to the pre-apply checkpoint without side-effect leakage. |
| Failure categories (closed) | `dry_run_skipped`, `outcome_aggregated_not_per_item`, `unsupported_items_hidden`, `rollback_checkpoint_missing`, `rollback_failed`, `rollback_side_effect_leak`, `parity_score_aggregate_hid_weak_category`, `needs_review_not_flagged`, `blocked_reason_free_form`. |
| Primary events | `migration_dry_run_produced`, `migration_parity_scored`, `migration_applied`, `migration_rolled_back`, `migration_outcome_recorded` (per item), `migration_rollback_checkpoint_written`, `migration_rollback_checkpoint_restored`. |
| Derived metrics | `migration_rollback_cleanliness_rate`, `migration_per_item_coverage_rate` (outcomes with all required typed reasons / validators), `parity_score_category_coverage` (per-category, not aggregate). |
| Owner (DRI) | compatibility_ecosystem_review forum chair (`@ahmeddyounis`) with docs_public_truth for migration-guide language. |
| Consumed by | benchmark-council Keymap / settings / theme / profile migration scoreboard; compatibility report packet. |

### 3.5 Restore success (`surface_restore_success`)

Session restore, crash recovery, topology-adjust restore, and
continuity-card restore. Every record reads a
`restore_prompt_record` per the entry-restore object model §3.

| Field | Value |
|---|---|
| Success criterion | The prompt advertises a `restore_level` (`exact_restore` / `compatible_restore` / `layout_only` / `recovered_drafts` / `evidence_only` / `no_restore`), the materialised result does not exceed the advertised level, and every non-`fully_present` missing-target state pairs with a typed recovery class from the closed §4 set. No silent re-execution of mutating commands on `exact_restore` / `compatible_restore`. |
| Failure categories (closed) | `restore_level_promised_higher_than_delivered`, `missing_target_state_silently_dropped`, `corrupt_restorable_state_silently_discarded`, `silent_mutating_command_replay`, `dirty_buffer_lost_without_journal`, `recovery_class_free_form`, `live_session_inferred_from_absence`. |
| Primary events | `restore_prompt_presented`, `restore_level_advertised`, `restore_level_delivered`, `restore_missing_target_classified`, `restore_recovery_class_selected`, `restore_completed`, `restore_abandoned`. |
| Derived metrics | `restore_fidelity_match_rate` (delivered == advertised), `crash_to_recovered_drafts_rate`, `missing_target_recovery_action_coverage`, `exact_restore_no_replay_rate`. |
| Owner (DRI) | aureline-buffer + shell_command_system (`@ahmeddyounis`), with support_export attestation. |
| Consumed by | support-bundle crash-recovery summary; recovery-ladder scoreboard. |

### 3.6 Opt-in-versus-continue-local (`surface_opt_in_boundary`)

Every boundary crossing from the account-free / local-only lane to
a service-opt-in lane (identity, model gateway, telemetry,
collaboration relay, sync, hosted marketplace, workspace control
plane, extension registry). The surface is **not** about forbidding
opt-in; it is about measuring whether the local path kept working
when opt-in was declined.

| Field | Value |
|---|---|
| Success criterion | At every opt-in prompt, the user can continue local work without the service (the boundary-manifest row's `absence_narrows_to` clause is honored), the opt-in is truly optional (a `set_up_later` / `continue_in_restricted_mode` decision hook is offered same-weight), and declining does not retroactively degrade previously-useful local flows. |
| Failure categories (closed) | `opt_in_forced_to_reach_useful_work`, `decline_degraded_prior_local_flow`, `continue_local_hidden_or_subordinate`, `absence_narrowing_undeclared`, `service_error_collapsed_into_needs_account`, `retroactive_lockout_after_decline`. |
| Primary events | `opt_in_prompt_presented{capability_row}`, `opt_in_accepted`, `opt_in_declined`, `continue_local_selected`, `narrowed_capability_advertised`, `local_flow_continued_after_decline`, `local_flow_degraded_after_decline`. |
| Derived metrics | `no_account_local_continuity_rate` (sessions reaching first-useful-edit without any opt-in accept), `decline_without_degradation_rate`, `narrowing_advertised_before_commit_rate`. |
| Owner (DRI) | product-scope + security_trust_review forum chairs (`@ahmeddyounis`) with boundary-manifest author co-review. |
| Consumed by | no-account switching scoreboard (every row reads this surface); compatibility report; release-evidence claim manifests that cite "no account required for core local use". |

## 4. Entry-route taxonomy (frozen)

Every event under §3 carries exactly one `entry_route_id` from the
closed set below. Entry routes are read-only; surfaces do not mint
additional routes. The route is distinct from the
`project_entry_action_record.source_surface` vocabulary — it
aggregates source surfaces by measurement shape so the scoreboard
can separate warm-start from protocol-handler reentry without
losing the upstream surface id.

The closed set of `entry_route_id` values:

- `er.start_center` — Start Center first-run or explicit Start
  Center row click. Includes the first-run screen on installs
  with no recent work.
- `er.recent_work_reopen` — reopen from a `recent_work_entry_record`
  surface (Start Center, palette-recents, workspace switcher, OS
  jump list, CLI recents).
- `er.restore_prompt` — the session-restore / crash-recovery /
  topology-adjust prompt presented a `restore_prompt_record` and
  the user committed from it.
- `er.protocol_handler_reentry` — OS file association,
  drag-and-drop, deep-link resolver, companion-handoff return,
  browser-handoff approval-ticket redemption. A protocol handler
  can land on any target kind; the route tags the **entry** only.
- `er.clone_or_import` — palette / Start Center / command-invoked
  `clone` or `import` verb. Collapses clone and import because
  the milestone doc's Bootstrap / entry-parity scoreboard measures
  them together; downstream analysis splits by `entry_verb`.
- `er.plain_open` — palette `Open…`, CLI positional `aureline
  <path>`, `File > Open…`, quick-open chosen as an entry. The
  plain-open route never implies a prior session was present.
- `er.workspace_switch` — command-palette workspace switcher /
  window-level switch to a different workspace while Aureline is
  already running. Session continues; a new
  `project_entry_action_record` is still emitted.
- `er.warm_start` — Aureline reopened into an existing session
  without a restore prompt (the continuity path). Measured
  separately from `er.restore_prompt` because it carries no
  dehydration step.

Rules (frozen):

1. Every event in §3 carries exactly one `entry_route_id`. A
   record that cannot resolve one denies with
   `entry_route_unresolved` rather than default to a generic
   route; silent defaulting is non-conforming.
2. `er.protocol_handler_reentry` is a measurement aggregator, not
   a trust grant. Deep-link resolution follows the entry-restore
   object model §1.2 rules (deep links never elevate trust).
3. `er.warm_start` events MUST NOT be filed as `er.restore_prompt`.
   Collapsing warm-start into restore hides the "no dehydration
   occurred" case that the benchmark-council Bootstrap / entry-
   parity scoreboard reads.
4. `first_useful_navigation_reached` MUST fire before
   `semantic_warmup_completed` is awaited; this is the event that
   records "editor is interactive even while the index is still
   warming". Surfaces that block navigation on warm-up are
   non-conforming.
5. Adding an `entry_route_id` is additive-minor; renaming or
   repurposing an existing value is breaking and opens a new
   decision row in
   [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## 5. Readiness buckets (frozen)

The UI/UX spec §6 names three readiness buckets. This plan freezes
them as measurement dimensions so scoreboards can separate "user
is blocked" from "user chose to defer" from "we prompted
unnecessarily".

- `blocking_now` — the task blocks first-useful-work on this
  entry. A scenario that lists a `blocking_now` task MUST carry
  either a completion outcome (`completed`, `skipped_by_user`,
  `failed`) or a typed reason the surface was reached at all
  without blocking (`false_positive_blocker`).
- `recommended_soon` — the task is advisable but does not block
  first-useful-work. `Set up later` and `Dismiss recommendation`
  MUST remain same-weight actions (milestone doc §3.37 rule).
- `optional_later` — the task exists for completeness /
  discoverability. Declining an `optional_later` task MUST NOT
  produce a dark pattern (re-prompt on next launch without a
  change in state, hidden state flip, or retroactive degradation).

Every task-success corpus scenario in
[`/artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml)
classifies every setup action into exactly one bucket. A scenario
that does not classify a setup action is non-conforming.

## 6. Archetype-detection outcome (reused)

Every corpus scenario carries an
`archetype_detection_outcome` drawn from the closed set reserved
by the runtime target-discovery / install-review taxonomy and
the UI/UX spec §DS.1:

- `certified_archetype_match` — detected archetype matches a
  Certified row in the PRD §5.38 archetype program.
- `supported_archetype_match` — detected archetype matches a
  Supported row.
- `community_archetype_match` — detected archetype matches a
  Community row.
- `experimental_archetype_match` — detected archetype matches an
  Experimental row.
- `probable_archetype_partial` — archetype detection reached
  `Probable` confidence (UI/UX spec DS.1); the corpus records the
  evidence age and detection source.
- `unrecognised_archetype` — no archetype matched; scenario is
  measured against the base first-useful-work path only.
- `archetype_detection_unavailable` — detector could not run
  (policy, sandbox, missing pack); scenario is measured with
  this state declared.

The set is closed. Adding a value is additive-minor; renaming is
breaking.

## 7. Ownership map (who owns the resulting metrics)

Ownership resolves through
[`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
Every metric derived from §3 resolves to exactly one primary DRI
and a co-reviewing lane. At the pre-implementation foundations
milestone the sole maintainer holds every role under the
`single-maintainer-backup` waiver; the rows below name the
**lane** so the assignment does not need to be rewritten when a
named backup lands.

| Measurement surface | Primary DRI lane | Co-reviewing lane | Evidence consumer |
|---|---|---|---|
| `surface_first_run` | product_scope_review | docs_public_truth, support_export | benchmark_lab Bootstrap/entry-parity |
| `surface_first_open` | shell_command_system | aureline-vfs, product_scope_review | benchmark_lab Bootstrap/entry-parity |
| `surface_first_useful_edit` | aureline-buffer | aureline-vfs, architecture_council | benchmark_lab certified-archetype |
| `surface_migration_review` | compatibility_ecosystem_review | docs_public_truth, support_export | benchmark_lab migration scoreboard |
| `surface_restore_success` | aureline-buffer | shell_command_system, support_export | support bundle; recovery ladder |
| `surface_opt_in_boundary` | product_scope_review | security_trust_review, docs_public_truth | no-account switching scoreboard; release_evidence claim manifest |

Rules (frozen):

1. A metric without a primary DRI lane is non-conforming. Adding
   a surface requires adding a row here.
2. Evidence consumers read metrics by event name; they do not
   re-derive. Re-derivation is a governance violation.
3. Co-reviewing lanes sign off on the seed corpus at every per-
   milestone review; the benchmark-council charter §4 ratifies
   their sign-off.

## 8. Seed task-success corpus

See [`/artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml).

The corpus at this milestone covers the following scenarios; each
row carries `entry_route_id`, `archetype_detection_outcome`, a
per-setup-action `readiness_bucket`, a
`first_useful_work_target_surface`, a `restore_class` (from the
§3 restore-level set, or `not_applicable`), and the three setup
postures (`blocking_now`, `recommended_soon`, `optional_later`).
Every scenario references a **fixture repo** by id (the seed
corpus points at concrete paths under `/fixtures/workspace/…` and
reserves fixture IDs for archetype-specific repos that later
milestones will materialise under `/fixtures/archetypes/…`).

Seed scenarios (all nine resolve against the eight worked
entry-restore fixtures plus the TS-web-app and Python-data-app
archetype seeds in the corpus):

1. `tsc.first_run_start_center_local_folder` — first run on a
   device with no prior profile, user opens a local folder from
   Start Center. Archetype detection recognises a TS web app.
2. `tsc.plain_open_local_folder_unknown_archetype` — `File >
   Open…` on a folder with no archetype match; measures the
   `er.plain_open` / `unrecognised_archetype` success path.
3. `tsc.protocol_handler_reentry_single_file` — OS file
   association opens a single file from a deep link; measures
   `er.protocol_handler_reentry` resolving to `target_kind =
   local_file`, `resulting_mode = single_file`, no trust
   widening.
4. `tsc.clone_then_review_remote_repo` — palette `Clone`
   resolves to a remote repository with `resulting_mode =
   clone_then_review`; measures `er.clone_or_import` without
   opt-in.
5. `tsc.import_vs_code_settings_dry_run_then_apply` — import
   from a VS Code install; measures migration-review dry-run,
   per-item outcome coverage, and an optional rollback path.
6. `tsc.restore_prompt_after_crash_compatible` — reopen after a
   crash; measures `er.restore_prompt` delivering
   `compatible_restore` with `missing_target_states =
   [binary_or_extension_version_changed, missing_extension_host]`.
7. `tsc.resume_managed_cloud_workspace_reauth_required` —
   resume a managed cloud workspace; measures authority
   re-evaluation without a new dehydration step
   (`er.warm_start` is not applicable because a reauth prompt
   is required).
8. `tsc.start_from_prebuild_ts_web_app_bypass` — start-from-
   snapshot TS web-app template; measures
   `open_prebuild_minimal` bypass same-weight with the full
   setup path.
9. `tsc.recent_work_reopen_missing_target_locate` — reopen a
   recent-work row whose target moved on disk; measures
   `er.recent_work_reopen` + `target_state = missing_target`
   routing through `locate_missing_target`.

Every scenario lists:

- `fixture_repo_ref` — an id resolving to a concrete fixture
  path today or a reserved slot to be populated in a later
  milestone;
- `entry_route_id` from §4;
- `archetype_detection_outcome` from §6;
- `first_useful_work_target_surface` — the editor / tree /
  README / changed-files / restore-compare / migration-center
  surface the user should land on, drawn from UI/UX spec §6 and
  the Start Center entry table;
- `readiness_bucket_task_rows[]` — per-setup-action rows each
  carrying `bucket`, `execution_boundary`, `side_effect_class`,
  and `expected_outcome`;
- `restore_class` when applicable (for `er.restore_prompt` and
  `er.warm_start` scenarios);
- `setup_posture_summary` — one of `blocking_now_count`,
  `recommended_soon_count`, `optional_later_count` counts per
  bucket, reserved for scoreboarding;
- `expected_events[]` — the §3 event families the scenario MUST
  emit when run;
- `evidence_consumer[]` — which scoreboard rows or evidence
  packets read this scenario.

## 9. Seed no-account switching scoreboard

See [`/artifacts/product/no_account_switching_scoreboard_seed.yaml`](../../artifacts/product/no_account_switching_scoreboard_seed.yaml).

The scoreboard seeds three row families:

1. **No-account local work** — each row asserts a capability
   that Aureline claims is reachable with no account, cites the
   boundary-manifest row that underwrites it (local_core rows
   only), lists the §3 events that qualify the claim, and
   declares the measurement target (a closed-vocabulary
   threshold class — `must_succeed_on_every_attempt`,
   `must_not_require_opt_in_prompt`, `must_not_degrade_after_decline`
   — with a narrowing reference when partial narrowing is
   intentional).
2. **Service-opt-in boundary outcomes** — each row names a
   service-plane capability (`identity_policy_service`,
   `model_gateway`, `telemetry_support_pipeline`,
   `collaboration_relay`, `managed_sync_profile`,
   `workspace_control_plane`, `extension_registry_mirror`,
   `hosted_marketplace_ui`), the boundary crossing the opt-in
   implies, the `absence_narrows_to` claim from the boundary
   manifest, and the §3 surface_opt_in_boundary events that
   qualify "declined without retroactive degradation".
3. **Intentionally narrowed startup / import capability** —
   each row names a startup or import flow that intentionally
   narrows capability (air-gapped first run; import declining
   BYOK key storage; restore prompt advertising `layout_only`
   because the extension host is missing), the **claim the
   scoreboard makes** ("capability was narrowed explicitly and
   advertised before commit"), and the events the surface MUST
   emit to prove the narrowing was advertised. This row family
   exists so reviewers can tell the difference between "we lost
   capability" and "we chose to narrow and said so".

The scoreboard is a **seed**: the row set is a starting point,
the threshold values are placeholder (`threshold_state:
"to_be_set_by_benchmark_council"`) until the benchmark council
ratifies them. Reviewers can nonetheless read the row shape and
see *how* the account-free local claim will be qualified.

Rules (frozen):

1. Every row cites exactly one
   [`docs/product/boundary_manifest_strawman.md`](./boundary_manifest_strawman.md)
   row id and exactly one §3 measurement surface.
2. Claim widening on a scoreboard row requires the benchmark
   council charter's §4 promotion path.
3. A scoreboard row that cannot cite an `absence_narrows_to`
   clause is non-conforming; narrowing without saying so is the
   failure mode the milestone doc §3.37 explicitly forbids.

## 10. Change policy

- **Additive-minor** changes — new `entry_route_id`, new
  readiness-bucket task class, new failure category within an
  existing `failure_categories` set, new event name, new
  scenario, new scoreboard row — land in this document, the
  corpus YAML, or the scoreboard YAML in the same change. The
  change cites the motivating fixture or evidence packet.
- **Repurposing** an event name, an `entry_route_id`, a
  readiness bucket, a failure-category token, a scoreboard-row
  id, or a success criterion is breaking. It opens a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The PRD / TAD / TDD / UI-UX spec / milestone doc win on any
  disagreement with the quotations in §13; this document, the
  seed corpus, and the scoreboard seed update together.

## 11. What this plan is not

- It is **not** a telemetry specification. No emitter is built
  at this milestone. The plan reserves names and shapes; the
  telemetry lane (a later milestone) builds the emitters under
  the ADR-0005 subscription envelope.
- It is **not** the claim manifest. Release-evidence claims
  live under
  [`artifacts/governance/governance_packet_template.yaml`](../../artifacts/governance/governance_packet_template.yaml)
  and cite this plan.
- It is **not** a substitute for the benchmark-council charter.
  Benchmarks resolve statistical, fixture, and reproducibility
  rules under the charter; this plan supplies the **event
  family** the benchmark council reads.

## 12. Acceptance

- Six measurement surfaces each carry a success criterion, a
  closed failure-category set, primary event names, derived
  metrics, a primary DRI lane, and an evidence consumer (§3).
- The entry-route taxonomy distinguishes direct open, restore,
  switch, and warm-start, plus Start Center, recent-work
  reopen, restore prompt, protocol-handler reentry, and clone /
  import (§4 closed set `er.*`).
- Readiness buckets `blocking_now`, `recommended_soon`, and
  `optional_later` are frozen with rules for setup posture
  (§5).
- Archetype-detection outcome vocabulary is frozen as a closed
  set (§6).
- The ownership map names a primary DRI lane per surface (§7).
- The seed task-success corpus is version-controlled, each row
  carries entry-route ID, archetype-detection outcome,
  readiness bucket(s), first-useful-work target surface,
  restore class, setup posture, and references concrete fixture
  repos (§8 → YAML).
- The seed scoreboard shows how account-free local work,
  service-opt-in boundary outcomes, and intentionally narrowed
  capability will be qualified later (§9 → YAML).
- The plan is linked from the docs index, the benchmark-council
  charter's evidence-consumer list, and the entry-restore
  object model (§14), so benchmark and UX evidence materials
  can cite it by path.

## 13. Source anchors

- `.t2/docs/Aureline_Milestones_Document.md:1017` — Onboarding,
  migration-assistance, learnability, and first-run-truth
  governance.
- `.t2/docs/Aureline_Milestones_Document.md:1023` — Start Center
  and primary shell entry points MUST keep `Open`, `Clone`,
  `Import`, `Restore`, and `Recent work` distinct, with a
  no-account path to useful local work.
- `.t2/docs/Aureline_Milestones_Document.md:1669` — Migration,
  compatibility, benchmark-council, and public-proof
  scoreboarding, including Bootstrap / entry-parity measuring
  "clone/open/import/resume success, first-useful-work path,
  trust staging, rollback cleanliness, and destination-
  collision handling".
- `.t2/docs/Aureline_Milestones_Document.md:1980` — "No account
  required for core local use" is a claimed row; the scoreboard
  seed exists to qualify it.
- `.t2/docs/Aureline_Milestones_Document.md:2284` — Onboarding,
  migration-assistance, and first-run-truth contract.
- `.t2/docs/Aureline_PRD.md:284` — first-run onboarding is an
  identified launch risk, not a polish afterthought.
- `.t2/docs/Aureline_PRD.md:1573` — §5.38 Reference workspaces,
  archetype certification, and compatibility program (Certified
  / Supported / Community / Experimental archetype taxonomy).
- `.t2/docs/Aureline_PRD.md:3311` — "first-run onboarding
  should detect existing tools and offer a dry-run import with
  a diff preview and rollback option".
- `.t2/docs/Aureline_PRD.md:4670` — required migration source
  scope (VS Code settings, keybindings, snippets, tasks, launch
  configs, themes, selected extensions).
- `.t2/docs/Aureline_PRD.md:4691` — "Open repo and reach first
  useful edit" performance target; the first-useful-work
  surface here measures the path that performance refers to.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:3244` —
  "Preview and parity scoring — show a dry-run diff,
  unsupported items, behavior changes, and an estimated
  workflow-parity score before apply".
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:762` — Start Center
  primary actions.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:1120` — entry
  source → first useful surface routing table (used verbatim by
  the corpus scenarios for `first_useful_work_target_surface`).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:1132` — readiness
  work grouped into `Blocking now`, `Recommended soon`,
  `Optional later`.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:17165` — Appendix
  DS archetype-detection, readiness-preflight, and first-
  useful-work templates.

## 14. Linked artifacts

- Entry / restore / migration object model:
  [`docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)
  and its schema
  [`schemas/workspace/entry_and_restore_result.schema.json`](../../schemas/workspace/entry_and_restore_result.schema.json).
- Runtime target-discovery / install-review taxonomy (archetype
  detection confidence slots):
  [`docs/runtime/target_discovery_and_install_review_taxonomy.md`](../runtime/target_discovery_and_install_review_taxonomy.md).
- Product boundary manifest strawman (boundary rows the opt-in
  scoreboard cites):
  [`docs/product/boundary_manifest_strawman.md`](./boundary_manifest_strawman.md).
- Benchmark-council charter (the forum that reads §3 events):
  [`docs/governance/benchmark_council_charter.md`](../governance/benchmark_council_charter.md).
- Ownership matrix (DRI / backup / waiver resolution):
  [`artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
- Decision register (dependency / supersession path):
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- Control-artifact index (this plan's canonical home and
  cadence):
  [`artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml).
- Task-success corpus seed:
  [`artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml).
- No-account switching scoreboard seed:
  [`artifacts/product/no_account_switching_scoreboard_seed.yaml`](../../artifacts/product/no_account_switching_scoreboard_seed.yaml).
