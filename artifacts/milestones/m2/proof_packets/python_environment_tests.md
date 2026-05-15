# External alpha Python interpreter, environment, and pytest proof packet

```yaml
packet_id: review_packet:alpha.python_environment_tests.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.python_environment_tests
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T08:15:33Z
stale_after: P14D
source_revision: git:2b819cecf18ba1033e33c42f32fbc3719aab0187
trigger_revision: python_environment_tests_contract_set@2026-05-15
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

This packet registers the current proof root for the external alpha Python
interpreter, environment, and pytest floor. It closes the blocked packet state
for Python environment detection, selected-interpreter provenance, pytest test
discovery, direct pytest dispatch contracts, rerun lineage, and task/support
export projection without widening the Python launch-bundle claim.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.python_environment_tests`
- Acceptance matrix: `artifacts/compat/reference_workspace_rows.yaml`
- Reference workspace seed: `fixtures/workspaces/reference/python_data_app_archetype_seed.json`
- Launch bundle: `artifacts/bundles/python_launch_bundle_alpha.yaml`
- Language pack: `artifacts/language_packs/python_service_alpha.yaml`
- Latest capture: `artifacts/milestones/m2/captures/python_environment_tests_validation_capture.json`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_current`

Evidence is fresh and scoped to the individual-local preview channel. The row
is green because the interpreter and pytest floor is inspectable through the
shared execution-context model, but the alpha claim remains intentionally
narrow. This packet does not claim debugger parity, refactor depth, Git/review
parity, managed-cloud daily-driver parity, or full notebook/kernel parity.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/python_environment_tests.md` |
| Latest capture | `artifacts/milestones/m2/captures/python_environment_tests_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | `benchmark_publication_proof`, `stale_after: P14D` |

## Protected Proof Path

Run the substrate checks cited by the capture:

```sh
cargo test -p aureline-runtime detectors::python
cargo test -p aureline-runtime pytest
cargo test -p aureline-language --test python_service_pack_alpha
cargo test -p aureline-language --test python_nav_alpha
cargo test -p aureline-shell run_context
cargo test -p aureline-shell tasks_seed
cargo test -p aureline-shell terminal_pane
python3 ci/check_alpha_scope.py --repo-root .
```

## Coverage

The capture cites the Python acceptance row in
`artifacts/compat/reference_workspace_rows.yaml` and each workflow claim it
contains. Environment/test-owned coverage is current for:

- `archetype_row:python_service_or_data_app`
- `core_workflow:python_service_or_data_app.interpreter_select`
- `core_workflow:python_service_or_data_app.run_tests`
- `reference_workspace:refws.python_data_app_archetype_seed`

The remaining Python row workflow slots are cited as wedge-boundary rows and
are not promoted by this packet:

- `core_workflow:python_service_or_data_app.debug`
- `core_workflow:python_service_or_data_app.refactor_basics`
- `core_workflow:python_service_or_data_app.notebook_handoff`
- `core_workflow:python_service_or_data_app.git_review`

Notebook scope stays at
`known_limit:external_alpha.notebook_handoff_only`. Full notebook kernel
parity, notebook round-trip certification, and hosted notebook parity remain
outside this packet.

## Substrate Consumed

- `crates/aureline-runtime/src/detectors/python/` resolves Python
  interpreter and environment-manager provenance for `uv`, `venv`, and Poetry,
  while surfacing missing, ambiguous, fallback, and unsupported states.
- `crates/aureline-runtime/src/discovery/pytest/` statically discovers pytest
  files and node ids, emits all-tests and per-item run contracts, stores direct
  runner argv, and projects queued/started or queued/blocked events through
  the canonical task-event stream.
- `crates/aureline-language/src/packs/python_service.rs` binds the Python
  service launch pack to interpreter-sensitive provider routes, pytest hooks,
  known gaps, docs, reporting refs, and safe export posture.
- `crates/aureline-language/src/python/` preserves selected-interpreter
  context in hover, definition, reference, and rename-preview records so
  language assistance cannot silently answer for the wrong environment.
- `crates/aureline-shell/src/run_context/`,
  `crates/aureline-shell/src/tasks_seed/`, and
  `crates/aureline-shell/src/terminal_pane/` project the same execution
  context into run-capable shell surfaces and support-export rows.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture added.
- Scoreboard row moved to `green` and `conditional_go`.
- Scoreboard row now cites the acceptance matrix, reference workspace seed,
  launch bundle, owning packet, and latest capture.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
- `artifacts/milestones/m2/known_limits_alpha.yaml` is unchanged.
