# External alpha TS/JS run, test, terminal, and debug proof packet

```yaml
packet_id: review_packet:alpha.ts_js_run_test_debug.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.ts_js_run_test_debug
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T07:58:23Z
stale_after: P14D
source_revision: git:6ef4f052e1eabfa2962331901b375f19ca2304aa
trigger_revision: ts_js_task_debug_contract_set@2026-05-15
exact_build_identity_ref: artifacts/build/build_identity.json
channel_context: preview
deployment_context:
  - individual_local
claim_change_state: no_claim_widening
same_change_truth_refs:
  docs_ref: docs/milestones/m2_alpha_scope.md
  migration_ref: docs/migration/source_ecosystem_coverage_matrix.md
  help_truth_ref: docs/docs/help_about_service_health_routes.md
  known_limits_ref: artifacts/feedback/external_alpha_known_limits.md
  support_export_ref: docs/support/support_bundle_contract.md
```

This packet registers the current proof root for the external alpha TS/JS
task, test, terminal, and debug loop floor. It closes the blocked packet state
for package-script test discovery, direct package-manager dispatch, rerun
lineage, terminal/task output linkage, and debug-prep visibility without
widening the alpha-limited TS/JS launch-bundle claim.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.ts_js_run_test_debug`
- Acceptance matrix: `artifacts/compat/ts_js_acceptance_rows.yaml`
- Fitness-function catalog: `artifacts/bench/fitness_function_catalog.yaml`
- Reference workspace seed: `fixtures/workspaces/reference/ts_web_app_archetype_seed.json`
- Launch bundle: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`
- Latest capture: `artifacts/milestones/m2/captures/ts_js_run_test_debug_validation_capture.json`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_narrowed`

Evidence is fresh and scoped to the individual-local preview channel. The row
is green because the task/test/terminal/debug floor is inspectable and routed
through one execution-context model, but the debug portion remains a seed
surface. This packet does not claim DAP-host parity, live attach/launch
debugging, persistent breakpoint orchestration, or source-map stability beyond
the debug-prep disclosure path.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/ts_js_run_test_debug.md` |
| Latest capture | `artifacts/milestones/m2/captures/ts_js_run_test_debug_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | `benchmark_publication_proof`, `stale_after: P14D` |

## Protected Proof Path

Run the substrate checks cited by the capture:

```sh
cargo test -p aureline-runtime package_scripts
cargo test -p aureline-runtime execution_context_alpha
cargo test -p aureline-runtime fixtures_replay_with_one_event_grammar_across_wedges
cargo test -p aureline-shell tasks_seed
cargo test -p aureline-shell debug_seed
cargo test -p aureline-shell run_context
cargo test -p aureline-shell --test terminal_pane_session_cases
cargo test -p aureline-terminal
python3 ci/check_alpha_scope.py --repo-root .
```

## Coverage

The capture cites every row in `artifacts/compat/ts_js_acceptance_rows.yaml`.
Run/test/debug-owned coverage is current for:

- `ts_js_acceptance_row:test_discover_run_rerun_debug`

The Node debug/source-map row is accepted only as a disclosed seed path:

- `ts_js_acceptance_row:node_debug_sourcemaps_stability`

The debug-prep surface is intentionally limited to execution-context
inspection, support-export copy, and terminal hand-off on the same context.
Attach, launch, breakpoint-map, debug-profile, stack/variable, and source-map
stability claims remain outside this packet.

The remaining TS/JS acceptance rows are cited as wedge-boundary rows and are
not promoted by this packet. They remain owned by navigation, Git/review,
deployment, migration, supportability, package, preview, or AI evidence lanes
named by the alpha matrix and known-limits packet.

## Fitness-Function Catalog

The capture resolves `artifacts/bench/fitness_function_catalog.yaml` at
catalog revision `1` and cites the catalog rather than minting a local metric
name. The row does not add a new fitness-function id. Task/run/test/debug
parity stays bound to the `task_run_test_debug_parity_scoreboard` family and
to the catalog's existing benchmark-lab and command-graph governance rows.

## Substrate Consumed

- `crates/aureline-runtime/src/discovery/package_scripts/` discovers bounded
  TS/JS package scripts, emits direct package-manager dispatch contracts, and
  preserves rerun lineage with exact-vs-current context drift.
- `crates/aureline-runtime/src/tasks/` normalizes task, test, debug, terminal,
  package, notebook, and AI-tool lanes into one typed event stream with raw
  envelope retention and support-export projection.
- `crates/aureline-runtime/src/execution_context/`,
  `crates/aureline-runtime/src/targets/`, and
  `crates/aureline-runtime/src/provenance/` keep launch-capable surfaces on
  one target, toolchain, trust, policy, and provenance model.
- `crates/aureline-runtime/src/detectors/node/` records Node/package-manager
  resolution, unsupported package managers, and missing runtime states before
  dispatch.
- `crates/aureline-shell/src/run_context/`,
  `crates/aureline-shell/src/tasks_seed/`,
  `crates/aureline-shell/src/debug_seed/`,
  `crates/aureline-shell/src/terminal_pane/`, and
  `crates/aureline-shell/src/host_boundary_cues/` project the same context
  into shell-visible task, debug-prep, terminal, and boundary rows.
- `crates/aureline-terminal/` owns PTY session truth, terminal headers,
  scrollback, protocol corpus, and restore-no-rerun semantics.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture added.
- Scoreboard row moved to `green` and `conditional_go`.
- Scoreboard row now cites the acceptance matrix, fitness-function catalog,
  reference workspace seed, launch bundle, owning packet, and latest capture.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
- `artifacts/milestones/m2/known_limits_alpha.yaml` is unchanged.
