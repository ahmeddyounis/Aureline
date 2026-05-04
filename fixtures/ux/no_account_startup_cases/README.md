# No-account local-first startup rehearsal fixtures

Seed fixtures for the no-account / local-first rehearsal packet
at
[`/artifacts/ux/no_account_startup_rehearsal_packet.md`](../../../artifacts/ux/no_account_startup_rehearsal_packet.md)
and the per-surface posture matrix at
[`/artifacts/ux/service_opt_in_boundary_rows.yaml`](../../../artifacts/ux/service_opt_in_boundary_rows.yaml).
Each fixture is keyed by an entry-verb class from the rehearsal
packet §1 closed set:

- `fresh_install_first_run`
- `open_local_folder`
- `open_local_workspace`
- `restore_previous_local_session`
- `import_local_pack`
- `decline_sign_in_or_service_opt_in`

The set also covers the three edge cases the rehearsal packet
must hold under: **offline**, **missing-target**, and
**restore-where-the-user-still-chooses-to-continue-locally**.

## What a fixture seeds

A fixture is a **seed**: it pins the resolved axes (entry verb,
target kind, archetype outcome, readiness-bucket counts, first-
useful-work target surface, restore class, expected blocker,
qualification class, account-prompt class, boundary-crossing
class, decline-path class, local-first claim class, safe-exit
actions, declined opt-ins, surface posture) and the rehearsal-
packet expected result for one row. It carries no raw absolute
paths, raw URLs, raw credentials, raw prompt text, or raw logs.
Every identity is an opaque ref; every timestamp is a monotonic
placeholder.

Each fixture:

- Names exactly one `entry_verb_class` from the rehearsal
  packet §1 closed set.
- Cites at least one
  [`fuw_row:*`](../../../artifacts/ux/first_useful_work_corpus/)
  corpus row by ref. Fixtures that exist only here (no upstream
  corpus row) are reserved for cases the corpus has not seeded
  yet and are marked `corpus_row_ref: not_attempted`.
- Cites at least one
  [`service_opt_in_boundary_row:*`](../../../artifacts/ux/service_opt_in_boundary_rows.yaml)
  boundary row by ref so reviewers can resolve which surface
  posture the row exercises.
- Asserts `local_first_claim_class != local_first_claim_violated`
  (the violation class is reserved for negative-test failure
  outcomes; the rehearsal packet's qualification class is what
  the packet would predict, never what the row asserts as its
  own steady state).
- Cites every closed-vocabulary axis verbatim from the
  rehearsal packet, no-account local-entry contract, and
  service-opt-in boundary rows file. Free-form fields outside
  the closed vocabularies are non-conforming.
- Reuses the entry / restore vocabulary re-exported from
  [`schemas/workspace/entry_and_restore_result.schema.json`](../../../schemas/workspace/entry_and_restore_result.schema.json).

## Cases

| Fixture | Entry verb | Network posture | Qualification class |
|---|---|---|---|
| `fresh_install_first_run_no_account.yaml` | `fresh_install_first_run` | `online` | `exact` |
| `open_local_folder_no_account.yaml` | `open_local_folder` | `online` | `exact` |
| `open_local_workspace_no_account.yaml` | `open_local_workspace` | `online` | `exact` |
| `restore_previous_local_session_no_account.yaml` | `restore_previous_local_session` | `online` | `compatible` |
| `import_local_pack_no_account.yaml` | `import_local_pack` | `offline` | `exact` |
| `decline_sign_in_at_first_run.yaml` | `decline_sign_in_or_service_opt_in` | `online` | `compatible` |
| `decline_service_opt_in_telemetry.yaml` | `decline_sign_in_or_service_opt_in` | `online` | `compatible` |
| `offline_first_run_no_account.yaml` | `fresh_install_first_run` | `offline` | `exact` |
| `missing_target_continue_local_no_account.yaml` | `open_local_folder` | `online` | `partial` |
| `restore_drift_continue_local_no_account.yaml` | `restore_previous_local_session` | `online` | `partial` |

## Rules

1. **No planning slugs.** Fixture filenames, ids, slugs, and
   notes MUST NOT contain milestone or task identifiers from
   the planning lane. These strings reach release evidence and
   support exports; planning metadata is removed before
   publication.
2. **One fixture, one entry verb, one rehearsal-packet shape.**
   A fixture pins exactly one `entry_verb_class` and exactly one
   `rehearsal_qualification_class`. Rows that try to exercise
   multiple verbs are split.
3. **Closed vocabulary only.** Every axis value MUST quote a
   value from a closed vocabulary upstream. Free-form text is
   reserved for the optional `notes` block.
4. **Boundary-row linkage.** Every fixture cites at least one
   `service_opt_in_boundary_row:*` id so the rehearsal packet's
   §5 surface-posture roll-up can resolve the declared posture
   the fixture exercises.
5. **Decline-path coverage.** A fixture whose
   `entry_verb_class` is `decline_sign_in_or_service_opt_in`
   MUST set `decline_path_class` to one of
   `continue_local_same_weight`,
   `continue_in_restricted_mode_same_weight`, or
   `locate_or_remove_recents_same_weight`, and MUST list at
   least one declined opt-in.
6. **Floor cases name no declined opt-ins.** A fixture whose
   `local_first_claim_class = local_first_floor` MUST NOT list
   any `declined_opt_ins`; the floor case is "no opt-in
   reached, no decline required".

## How fixtures are exercised

Reviewers exercise the fixtures by:

1. Picking the verb(s) the rehearsal lane requires.
2. Loading the
   [no-account local-first startup rehearsal packet](../../../artifacts/ux/no_account_startup_rehearsal_packet.md)
   and filling one §3 row per fixture exercised.
3. Recording the observed `rehearsal_qualification_class`
   (`exact` / `compatible` / `partial` / `failed`), observed
   blocker, hidden-setup failures, surface-posture drift, and
   whether the local-first claim held.
4. Aggregating the §3 rows into the §5 surface-posture roll-up,
   then the §7 account-free proof block.

This directory is a **seed**. Adding fixtures is additive-minor.
Promoting threshold values away from the seed shape lives in
the benchmark-council charter §4 promotion path.
