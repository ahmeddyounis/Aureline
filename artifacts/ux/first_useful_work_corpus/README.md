# First-useful-work qualification corpus

Reviewer-readable companion to:

- [`/docs/product/onboarding_measurement_plan.md`](../../../docs/product/onboarding_measurement_plan.md)
- [`/docs/ux/entry_restore_truth_audit.md`](../../../docs/ux/entry_restore_truth_audit.md)
- [`/docs/ux/no_account_local_entry_contract.md`](../../../docs/ux/no_account_local_entry_contract.md)
- [`/artifacts/ux/no_account_switching_scoreboard.yaml`](../no_account_switching_scoreboard.yaml)
- [`/artifacts/ux/startup_route_rehearsal_packet.md`](../startup_route_rehearsal_packet.md)
- [`/fixtures/ux/first_useful_work_cases/`](../../../fixtures/ux/first_useful_work_cases/)

This directory turns the first-useful-work / first-run / first-open
journey from design intent into release-bearing proof. The
existing task-success corpus seed under
[`/artifacts/product/task_success_corpus_seed.yaml`](../../../artifacts/product/task_success_corpus_seed.yaml)
seeds nine **scenarios**; this corpus seeds the **qualification**
shape — one row per protected startup route × case-category × seed
case — that the rehearsal packet scores against and the scoreboard
aggregates by entry route and deployment profile.

A row in this corpus is the unit of:

1. **Rehearsal qualification.** Every row is scorable as
   `exact` / `compatible` / `partial` / `failed` using exactly
   one packet format
   ([`/artifacts/ux/startup_route_rehearsal_packet.md`](../startup_route_rehearsal_packet.md)),
   so every protected startup route resolves through one shape
   rather than a route-specific report.
2. **Failure-cause attribution.** Every row names exactly one
   `expected_blocker_class` from the closed set in §1.1 below, so
   reviewers can distinguish missing target / blocked
   prerequisite / policy gate / hidden setup / restore drift /
   no-blocker rows mechanically.
3. **Local-first claim qualification.** Every row declares a
   `local_first_claim_class` and a `decline_path_class` so the
   rehearsal packet can record whether Aureline preserved a
   credible local-first path when sign-in or managed services
   were absent or declined.

Rows resolve every axis to vocabulary already frozen upstream:

- `entry_verb`, `target_kind`, `resulting_mode`,
  `admission_class`, `next_step_decision_hook`, `restore_level`,
  `safe_recovery_action` — entry-restore object model.
- `entry_route_id` — onboarding measurement plan §4.
- `archetype_detection_outcome` — onboarding measurement plan §6.
- `readiness_bucket` — onboarding measurement plan §5.
- `first_useful_work_target_surface` — same set as the task-success
  corpus seed.
- `restore_class` — onboarding measurement plan §3.5 restore-level
  set, plus `not_applicable`.
- `deployment_profile_id` — deployment-profile register.
- `boundary_manifest_row_id` — boundary-manifest strawman.
- `startup_state_token` — entry-restore truth audit §6.
- `support_packet_family_ref` — startup-state copy-review.
- `recovery_ladder_rung_ref` — recovery-ladder packet.

This corpus mints **two** new closed vocabularies — every other
field re-exports an upstream one. The two new vocabularies are
the qualification class (§1.2) and the case-category id (§1.3),
plus the small attribution closures listed in §1.1 (blockers),
§1.4 (local-first claim), and §1.5 (decline path). Adding a value
to any of them is additive-minor and lands here, in every row
file, and in
[`/artifacts/ux/no_account_switching_scoreboard.yaml`](../no_account_switching_scoreboard.yaml)
and
[`/artifacts/ux/startup_route_rehearsal_packet.md`](../startup_route_rehearsal_packet.md)
in the same change. Repurposing a value is breaking and opens a
new decision row under
[`/artifacts/governance/decision_index.yaml`](../../../artifacts/governance/decision_index.yaml).

## 1. Frozen vocabularies (re-exported by every row file)

Row files in this directory MUST quote exactly one value (or the
listed subset) from each closed vocabulary. Free-form values are
non-conforming.

### 1.1 `expected_blocker_class`

The user-visible failure cause the row is gated on. One value
per row.

- `missing_target` — entry resolved, target was not where the
  recent-work / OS / deep-link record claimed it would be.
- `blocked_prerequisite` — admission denied because a typed
  prerequisite (extension host, language server, runtime,
  toolchain, sign-in for an explicitly account-bound surface)
  was not ready.
- `policy_gate` — admission denied or narrowed by an active
  policy bundle (managed fleet, kiosk, restricted mode).
- `hidden_setup` — admission committed but the user reached a
  resulting state that depends on undisclosed setup work
  (undisclosed package install, silent extension activation,
  silent sign-in, silent toolchain mutation).
- `restore_drift` — restore advertised one level and delivered
  another, or missing-target states were silently dropped.
- `no_blocker` — the row qualifies the no-blocker happy path on
  this protected route.

### 1.2 `rehearsal_qualification_class`

How the rehearsal packet scores the row when the route is
exercised end-to-end. One value per row.

- `exact` — first-useful-work reached on the declared
  `first_useful_work_target_surface` with no narrowing of
  `restore_class`, `resulting_mode`, or local-first claim, and
  no observed blocker outside the row's
  `expected_blocker_class`.
- `compatible` — first-useful-work reached and the route's
  declared narrowing was advertised before commit (e.g. the
  restore prompt advertised `compatible_restore`, the migration
  center marked items `needs_review`); the user-facing claim
  matches what the row predicted.
- `partial` — first-useful-work reached only after the user
  invoked an extra `safe_exit_action` (Continue local, Open
  without restore, Continue in restricted mode, Locate missing
  target, Roll back import). The row's claim is preserved but
  one of the readiness-bucket tasks the row labelled as
  `recommended_soon` or `optional_later` had to be acknowledged
  to reach the surface.
- `failed` — first-useful-work was not reachable on this route
  in this rehearsal, or the row's local-first / decline-path
  claim was violated (forced sign-in, retroactive degradation
  after decline, restore overclaim, hidden setup discovered
  after commit).

### 1.3 `case_category`

Which case category a row belongs to. The directory ships one
file per category; a row's `case_category` MUST match the file
it lives in.

- `local_open` — local folder / file / repo open, including
  `er.start_center` and `er.plain_open` first-useful-work paths.
  File: `local_open.yaml`.
- `clone` — palette / Start Center clone of a remote repo, with
  no automatic trust, install, or hook execution. File:
  `clone.yaml`.
- `import` — competitor-config / portable-state / handoff-packet
  import with dry-run, per-item attribution, and rollback
  checkpoint. File: `import.yaml`.
- `restore` — session / crash / topology restore through the
  restore prompt. File: `restore.yaml`.
- `missing_target_reopen` — recent-work reopen against a target
  that is no longer reachable on disk or remote. File:
  `missing_target_reopen.yaml`.
- `offline_or_mirror_open` — startup on a device offline or on
  an air-gapped / mirror-only profile. File:
  `offline_or_mirror_open.yaml`.
- `managed_sign_in_available_but_skipped` — managed sign-in is
  offered (managed cloud / enterprise online), the user
  declines, the local path stays useful. File:
  `managed_sign_in_available_but_skipped.yaml`.
- `service_opt_in_declined` — a non-identity service opt-in
  (telemetry, model gateway, marketplace, sync, collaboration
  relay) is offered, the user declines, the local flow stays
  useful. File: `service_opt_in_declined.yaml`.

### 1.4 `local_first_claim_class`

How the row asserts the local-first promise on this route. One
value per row.

- `local_first_floor` — the row qualifies that no account, no
  managed service, and no network are required to reach
  first-useful-work on this route.
- `local_first_with_optional_decline` — the row qualifies that
  declining one or more service opt-ins still reaches
  first-useful-work; the declines are listed in the row's
  `declined_opt_ins`.
- `local_first_narrowed_advertised` — the row qualifies that
  capability narrowing was advertised before commit (e.g.
  `restore_level = compatible_restore`, `narrowed_capability_advertised`)
  and the local-first floor still holds for the narrowed scope.
- `account_required_route` — the route is genuinely
  account-bound (e.g. resume of a managed cloud workspace
  whose authority cannot be re-evaluated locally); the row
  qualifies that declining the account prompt did not
  retroactively degrade prior local work.
- `local_first_claim_violated` — only used as the
  `failed`-class outcome shape on the row's negative test;
  rows whose default outcome is `local_first_claim_violated`
  are non-conforming.

### 1.5 `decline_path_class`

The shape of the safe exit / rollback / decline path the row
guarantees. One value per row.

- `no_decline_path_required` — the row's primary path does not
  encounter an opt-in, restore, or migration commit gate; no
  decline path is owed.
- `continue_local_same_weight` — when an opt-in is presented,
  Continue local / Set up later is offered same-weight with
  the opt-in card.
- `open_without_restore_same_weight` — when restore is offered,
  Open without restore / Safe mode is offered same-weight with
  Restore.
- `roll_back_import_available` — when migration is committed,
  Roll back is reachable with the workspace returning to the
  pre-apply checkpoint.
- `locate_or_remove_recents_same_weight` — when a recent-work
  target is missing, Locate / Remove from recents is offered
  same-weight with Open anyway.
- `continue_in_restricted_mode_same_weight` — when policy /
  managed gates apply, Continue in restricted mode is offered
  same-weight with the gated action.

### 1.6 Re-exported vocabularies (sources)

| Vocabulary | Source |
|---|---|
| `entry_verb`, `target_kind`, `resulting_mode`, `admission_class`, `next_step_decision_hook`, `restore_level`, `safe_recovery_action`, `portability_class` | `schemas/workspace/entry_and_restore_result.schema.json` (re-exported through `docs/workspace/entry_restore_object_model.md`) |
| `entry_route_id` (`er.*`) | `docs/product/onboarding_measurement_plan.md` §4 |
| `readiness_bucket` (`blocking_now` / `recommended_soon` / `optional_later`) | `docs/product/onboarding_measurement_plan.md` §5 |
| `archetype_detection_outcome` | `docs/product/onboarding_measurement_plan.md` §6 |
| `restore_class` | `docs/product/onboarding_measurement_plan.md` §3.5 |
| `first_useful_work_target_surface` | `artifacts/product/task_success_corpus_seed.yaml` `first_useful_work_target_surface_vocabulary` |
| `deployment_profile_id` | `artifacts/governance/deployment_profiles.yaml` |
| `boundary_manifest_row_id` | `docs/product/boundary_manifest_strawman.md` |
| `startup_state_token` | `artifacts/ux/startup_state_copy_review.yaml` `startup_state_token_vocabulary` |
| `support_packet_family_ref` | `artifacts/ux/startup_state_copy_review.yaml` `support_packet_family_ref_vocabulary` |
| `recovery_ladder_rung_ref` | `artifacts/ux/startup_state_copy_review.yaml` `recovery_ladder_rung_ref_vocabulary` |
| `measurement_surface` | `docs/product/onboarding_measurement_plan.md` §3 |
| `protected_metric_ref` | `artifacts/bench/protected_metrics.yaml` |
| `journey_class` | `schemas/traces/journey_trace.schema.json` |

## 2. Row shape (frozen)

Every row in every file in this directory MUST carry the
following fields. Free-form fields outside the closed
vocabularies are non-conforming.

| Field | Required | Notes |
|---|---|---|
| `row_id` | yes | `fuw_row:<case_category>.<slug>`. Slug uses snake_case, no milestone slugs. |
| `case_category` | yes | One value from §1.3. MUST match the file the row lives in. |
| `entry_verb` | yes | Closed vocab. |
| `target_kind` | yes | Closed vocab. |
| `resulting_mode` | yes | Closed vocab. |
| `entry_route_id` | yes | Closed vocab from §4 of the measurement plan. |
| `archetype_detection_outcome` | yes | Closed vocab. |
| `readiness_bucket_summary` | yes | `{blocking_now_count, recommended_soon_count, optional_later_count}` integers. |
| `readiness_bucket_task_rows` | yes | Per-task readiness bucket rows; same shape as the task-success corpus seed. |
| `first_useful_work_target_surface` | yes | Closed vocab. |
| `restore_class` | yes | Closed vocab including `not_applicable`. |
| `expected_blocker_class` | yes | One value from §1.1. |
| `rehearsal_qualification_class` | yes | One value from §1.2. The class the row claims when the rehearsal packet exercises it. |
| `safe_exit_actions` | yes | Non-empty list of `next_step_decision_hook` values. |
| `decline_path_class` | yes | One value from §1.5. |
| `local_first_claim_class` | yes | One value from §1.4. Never `local_first_claim_violated`. |
| `declined_opt_ins` | when `local_first_with_optional_decline` | List of `boundary_manifest_row_id` values that were declined on this route. |
| `deployment_profile_id` | yes | Closed vocab. |
| `startup_state_token` | yes | Closed vocab. |
| `boundary_manifest_row_id` | yes | The boundary row whose claim this route exercises. |
| `support_packet_family_refs` | yes | Non-empty list. |
| `recovery_ladder_rung_refs` | yes | Non-empty list, including `rung.none_required`. |
| `measurement_surface` | yes | Primary measurement surface. |
| `protected_metric_refs` | yes | Non-empty list. |
| `journey_classes` | yes | Non-empty list. |
| `supporting_corpus_scenario_refs` | yes | List of `tsc.*` ids from `artifacts/product/task_success_corpus_seed.yaml`; `[]` when the row reserves a route the seed corpus has not exercised yet. |
| `supporting_fixture_refs` | yes | List of fixture paths. At least one MUST live under `fixtures/ux/first_useful_work_cases/`. |
| `scoreboard_row_refs` | yes | Non-empty list of scoreboard row ids the row qualifies. |
| `evidence_consumer` | yes | Non-empty list of evidence-packet families that consume this row. |
| `notes` | optional | Free-text rationale for axes that the closed vocab cannot fully express; never used to mint state names. |

## 3. Rules (frozen)

1. **One row, one route, one case category, one packet shape.**
   Every row resolves to exactly one `entry_route_id`, exactly
   one `case_category`, and exactly one `rehearsal_qualification_class`.
   Rows that try to score multiple qualification classes on one
   route are split.
2. **Closed vocabulary only.** Every axis value MUST quote a
   value from a closed vocabulary in §1. Free-form text is
   reserved for `notes`.
3. **Failure cause is mechanical.** Every row carries exactly
   one `expected_blocker_class`. A row whose blocker is the
   `no_blocker` happy path MUST also carry
   `rehearsal_qualification_class = exact`. A row whose blocker
   is anything else MUST NOT score `exact`.
4. **Local-first claim is mechanical.** A row whose
   `local_first_claim_class = local_first_floor` MUST NOT list
   any `declined_opt_ins`; the floor case is "no opt-in
   reached, no decline required". A row whose
   `local_first_claim_class = local_first_with_optional_decline`
   MUST list at least one declined opt-in.
5. **Decline-path coverage.** A row whose `case_category` is
   `service_opt_in_declined` or
   `managed_sign_in_available_but_skipped` MUST set
   `decline_path_class` to one of
   `continue_local_same_weight`, `continue_in_restricted_mode_same_weight`,
   or `locate_or_remove_recents_same_weight` and MUST NOT use
   `no_decline_path_required`.
6. **Restore-drift attribution.** A row with
   `expected_blocker_class = restore_drift` MUST set
   `case_category` to `restore` and
   `rehearsal_qualification_class` to `failed` (drift is
   non-conforming by definition; the row exists so the packet
   can record what `failed` looks like for this route).
7. **Scoreboard linkage.** Every row cites at least one row id
   in
   [`/artifacts/ux/no_account_switching_scoreboard.yaml`](../no_account_switching_scoreboard.yaml).
   Rows that no scoreboard row reads are non-conforming.
8. **Fixture linkage.** Every row cites at least one fixture
   under
   [`/fixtures/ux/first_useful_work_cases/`](../../../fixtures/ux/first_useful_work_cases/).
   Reserving a fixture id is additive-minor; reusing a
   released fixture id for a different fixture is breaking.
9. **No milestone slugs.** Row ids, fixture ids, slugs, and
   notes MUST NOT contain milestone or task identifiers
   (`M00`, `M00-526`, `WP-…` and similar). These strings
   reach release evidence and support exports; planning
   metadata is removed before publication.
10. **Audit-row linkage.** Every row cites exactly one
    `startup_state_token` so the rehearsal packet can resolve
    the audited copy-review row from
    [`/artifacts/ux/startup_state_copy_review.yaml`](../startup_state_copy_review.yaml).

## 4. Files in this directory

| File | Case category | Notes |
|---|---|---|
| [`local_open.yaml`](./local_open.yaml) | `local_open` | Start Center / plain open of local folder, file, or repo. |
| [`clone.yaml`](./clone.yaml) | `clone` | Clone of remote repository with post-clone trust separation. |
| [`import.yaml`](./import.yaml) | `import` | Import of competitor-config / portable-state / handoff packet. |
| [`restore.yaml`](./restore.yaml) | `restore` | Session / crash / topology restore through the restore prompt. |
| [`missing_target_reopen.yaml`](./missing_target_reopen.yaml) | `missing_target_reopen` | Recent-work reopen against an unreachable target. |
| [`offline_or_mirror_open.yaml`](./offline_or_mirror_open.yaml) | `offline_or_mirror_open` | Startup offline or on an air-gapped / mirror-only profile. |
| [`managed_sign_in_available_but_skipped.yaml`](./managed_sign_in_available_but_skipped.yaml) | `managed_sign_in_available_but_skipped` | Managed sign-in offered, declined; local path stays useful. |
| [`service_opt_in_declined.yaml`](./service_opt_in_declined.yaml) | `service_opt_in_declined` | Non-identity service opt-in declined; local flow stays useful. |

## 5. How this corpus is exercised

Reviewers exercise the corpus by:

1. Picking the row(s) the route or case category requires.
2. Loading the
   [rehearsal packet template](../startup_route_rehearsal_packet.md)
   and filling one **route entry** per row.
3. Recording the observed `rehearsal_qualification_class`
   (`exact` / `compatible` / `partial` / `failed`), observed
   blocker, hidden-setup failures, startup-route drift, and
   whether the local-first claim held.
4. Aggregating the route entries into the per-row-family
   summary in
   [`/artifacts/ux/no_account_switching_scoreboard.yaml`](../no_account_switching_scoreboard.yaml).

This corpus is a **seed**. Adding rows is additive-minor.
Promoting threshold values away from
`to_be_set_by_benchmark_council` is reserved to the
benchmark-council charter §4 promotion path.
