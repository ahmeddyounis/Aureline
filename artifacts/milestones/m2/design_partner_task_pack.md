# External Alpha Design Partner Task Pack

This task pack names the daily-loop scenarios external alpha partners should
run, the required reference fixtures, and the pass/fail rubric for each claimed
alpha wedge. It consumes the alpha scope matrix and does not add new wedges.

## Canonical Inputs

- Alpha scope matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Alpha go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- Intake packet: `artifacts/milestones/m2/design_partner_intake_packet.md`
- Partner guide: `docs/alpha/design_partner_guide.md`
- Feedback taxonomy: `artifacts/feedback/design_partner_feedback_taxonomy.yaml`
- Known limits: `artifacts/feedback/external_alpha_known_limits.md`
- Upstream intake checklist: `artifacts/program/design_partner_intake_checklist.yaml`
- TypeScript / JavaScript fixture: `fixtures/workspaces/reference/ts_web_app_archetype_seed.json`
- Python fixture: `fixtures/workspaces/reference/python_data_app_archetype_seed.json`
- Shared benchmark manifest: `fixtures/benchmarks/corpus_manifest.yaml`

## Shared Completion Evidence

Every task result records:

- `task_script_id`;
- `wedge_ref`;
- `workflow_id`;
- `scoreboard_row_ref`;
- reference workspace or privacy-cleared partner workspace ref;
- operating system and deployment posture;
- pass/fail state;
- `blocker_severity`;
- redaction-safe evidence refs; and
- `known_limit_refs` when a failure is caused by unsupported scope.

## TypeScript / JavaScript Tasks

| Task script | Workflow id | Scoreboard row | Required fixture | Pass criteria | Fail criteria |
|---|---|---|---|---|---|
| `task.alpha.ts_js.open_to_first_useful_result` | `workflow.alpha.ts_js.open_to_first_useful_result` | `scoreboard_row:alpha_scope.ts_js_navigation` | `fixtures/workspaces/reference/ts_web_app_archetype_seed.json` | Partner opens the workspace and reaches a useful file, symbol, route, or search result before full indexing is required. Partial-index truth is visible. | No useful result, hidden full-index dependency, stale result without warning, or unsupported managed/cloud requirement. |
| `task.alpha.ts_js.rename_preview` | `workflow.alpha.ts_js.rename_preview` | `scoreboard_row:alpha_scope.ts_js_navigation` | `fixtures/workspaces/reference/ts_web_app_archetype_seed.json` | Partner can preview a project-reference rename, see partial-index or generated/read-only limits, and cancel without changing files. | Mutation applies without preview, scope silently widens, undo/revert posture is unclear, or generated/read-only targets are mislabeled. |
| `task.alpha.ts_js.test_debug_loop` | `workflow.alpha.ts_js.test_debug_loop` | `scoreboard_row:alpha_scope.ts_js_run_test_debug` | `fixtures/workspaces/reference/ts_web_app_archetype_seed.json` | Partner can discover, run, rerun, and debug a targeted test with visible execution-context and output evidence. | Test cannot be discovered, rerun target is wrong, debug route is opaque, or terminal/task output cannot be linked to the task. |
| `task.alpha.ts_js.git_review_basics` | `workflow.alpha.ts_js.git_review_basics` | `scoreboard_row:alpha_scope.ts_js_git_review` | `fixtures/workspaces/reference/ts_web_app_archetype_seed.json` | Partner can clone or open, create a branch, inspect a diff, commit, and prepare review evidence without hosted-provider parity assumptions. | Git state is stale or wrong, local-only review state is hidden, or provider-specific behavior is implied without evidence. |

## Python Tasks

| Task script | Workflow id | Scoreboard row | Required fixture | Pass criteria | Fail criteria |
|---|---|---|---|---|---|
| `task.alpha.python.interpreter_and_tests` | `workflow.alpha.python.interpreter_and_tests` | `scoreboard_row:alpha_scope.python_environment_tests` | `fixtures/workspaces/reference/python_data_app_archetype_seed.json` | Partner selects an interpreter or environment, discovers pytest tests, and runs a targeted test with visible environment truth. | Interpreter state is ambiguous, pytest discovery is unavailable without a known limit, or rerun uses the wrong environment. |
| `task.alpha.python.debug_and_refactor` | `workflow.alpha.python.debug_and_refactor` | `scoreboard_row:alpha_scope.python_debug_refactor` | `fixtures/workspaces/reference/python_data_app_archetype_seed.json` | Partner launches debug or inspects debug readiness, previews a refactor, and sees notebook adjacency as handoff scope only. | Full notebook parity is implied, debug mapping is opaque, refactor applies without preview, or partial language evidence is unlabeled. |
| `task.alpha.python.git_review_basics` | `workflow.alpha.python.git_review_basics` | `scoreboard_row:alpha_scope.python_debug_refactor` | `fixtures/workspaces/reference/python_data_app_archetype_seed.json` | Partner can clone or open, create a branch, inspect a diff, commit, and prepare review evidence on the Python fixture or cleared workspace. | Git state is stale or wrong, support export cannot reconstruct the task, or provider parity is implied without evidence. |

## Pass / Fail Rubric

| Result | Meaning | Required routing |
|---|---|---|
| `pass` | All required pass criteria are met, evidence is redaction-safe, and no new known limit is needed. | Route as `task_completion`. |
| `pass_with_scoped_workaround` | The task completes with a documented workaround that does not widen scope. | Route as `task_completion` with `scoped_workaround` severity. |
| `blocked_pending_packet` | The task may be supportable, but proof evidence, privacy review, or support export is not current. | Route as `alpha_task_blocker` or `privacy_redaction`. |
| `known_limit` | The task asks for unsupported scope or reveals a published caveat. | Route as `scope_or_known_limit` with `known_limit_refs`. |
| `fail` | A claimed alpha task cannot complete safely on the required fixture or cleared workspace. | Route as `alpha_task_blocker`. |

## Redaction-Safe Evidence

Task reports should prefer metadata and derived evidence:

- stable task, workflow, wedge, scoreboard, fixture, and build refs;
- redacted screenshots with partner, path, host, token, account, and customer
  text removed;
- support-export ids after redaction review;
- logs clipped to command, exit state, and typed diagnostic ids;
- no raw repository archives, raw support bundles, raw terminal transcripts, or
  unredacted paths unless `privacy_review_state: redaction_cleared`.
