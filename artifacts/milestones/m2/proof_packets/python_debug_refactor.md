# External alpha Python debug, refactor, notebook handoff, and Git proof packet

```yaml
packet_id: review_packet:alpha.python_debug_refactor.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.python_debug_refactor
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T08:23:58Z
stale_after: P14D
source_revision: git:bc8cd2e87e960b5b84773c0dedfc47b638af5361
trigger_revision: python_debug_refactor_contract_set@2026-05-15
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
debug, refactor basics, notebook handoff, and local Git floor. It closes the
blocked packet state for the Python debug/refactor scoreboard row without
widening the launch-bundle claim.

Debug evidence is intentionally a seed surface. It proves debug-readiness
inspection, execution-context provenance, prerequisite disclosure, terminal
handoff, and support-export projection. It does not claim attach, launch,
breakpoint, watch, or debug-profile orchestration.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.python_debug_refactor`
- Acceptance matrix: `artifacts/compat/reference_workspace_rows.yaml`
- Reference workspace seed: `fixtures/workspaces/reference/python_data_app_archetype_seed.json`
- Launch bundle: `artifacts/bundles/python_launch_bundle_alpha.yaml`
- Language pack: `artifacts/language_packs/python_service_alpha.yaml`
- Review packet template: `docs/review/m2_review_packet_template.md`
- Known-limits packet: `artifacts/milestones/m2/known_limits_alpha.yaml`
- Latest capture: `artifacts/milestones/m2/captures/python_debug_refactor_validation_capture.json`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_narrowed`

Evidence is fresh and scoped to the individual-local preview channel. The row
is green because the Python wedge can prove debug readiness inspection,
preview-only rename/refactor basics, notebook handoff disclosure, local Git
change review, and support-export projection against the protected Python
reference workspace row.

The row remains deliberately narrowed. It does not claim full notebook parity,
notebook kernel execution, notebook round-trip certification, hosted notebook
parity, full debugger parity, broad refactor depth, risky Git history mutation,
hosted review-provider parity, or managed-cloud daily-driver parity.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/python_debug_refactor.md` |
| Latest capture | `artifacts/milestones/m2/captures/python_debug_refactor_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | `docs_claim_truth_proof`, `stale_after: P14D` |

## Protected Proof Path

Run the substrate checks cited by the capture:

```sh
cargo test -p aureline-shell debug_seed
cargo test -p aureline-language --test python_nav_alpha
cargo test -p aureline-language --test code_action_alpha
cargo test -p aureline-language --test python_service_pack_alpha
cargo test -p aureline-shell notebook_alpha
cargo test -p aureline-shell notebook_trust_badges
cargo test -p aureline-shell --test git_change_list_alpha
cargo test -p aureline-shell --test destructive_core_preview
python3 ci/check_alpha_scope.py --repo-root .
```

## Coverage

The capture cites the Python acceptance row in
`artifacts/compat/reference_workspace_rows.yaml` and the workflow rows this
packet owns:

- `archetype_row:python_service_or_data_app`
- `core_workflow:python_service_or_data_app.debug`
- `core_workflow:python_service_or_data_app.refactor_basics`
- `core_workflow:python_service_or_data_app.notebook_handoff`
- `core_workflow:python_service_or_data_app.git_review`
- `reference_workspace:refws.python_data_app_archetype_seed`

The environment and pytest workflow slots remain owned by
`proof_packet:alpha.python_environment_tests` and are cited only as upstream
preconditions:

- `core_workflow:python_service_or_data_app.interpreter_select`
- `core_workflow:python_service_or_data_app.run_tests`

Notebook scope stays at
`known_limit:external_alpha.notebook_handoff_only`. Full notebook kernel
parity, notebook round-trip certification, hosted notebook parity, and broad
notebook authoring are outside this packet.

## Substrate Consumed

- `crates/aureline-shell/src/debug_seed/` projects selected execution-context
  provenance, target/runtime badges, debug prerequisites, and support-export
  rows while reserving attach, launch, breakpoint, watch, and profile actions.
- `crates/aureline-language/src/python/` emits interpreter-bound hover,
  definition, reference, and rename-preview records with explicit completeness,
  generated/protected counts, rollback posture, and degraded-scope disclosure.
- `crates/aureline-language/src/code_actions/` keeps Python refactor basics on
  the shared code-action admission model: side-effect disclosure, preview
  requirement, named undo groups, and refusal of broad silent apply.
- `crates/aureline-language/src/packs/python_service.rs` binds the Python
  service pack to bounded alpha gaps, local Git review surfaces, launch-bundle
  reporting, safe-preview posture, and no broad refactor or debugger parity
  claim.
- `crates/aureline-shell/src/notebook_alpha/` and
  `crates/aureline-shell/src/notebook_trust_badges/` preserve notebook trust,
  diff, repair, and export labels while keeping the Python launch wedge at
  handoff disclosure only.
- `crates/aureline-shell/src/git_changes/` projects local Git status into the
  shell change list, editor chips, and review entry points from the same local
  Git authority.
- `crates/aureline-shell/src/review_preview/` preserves preview/apply/revert
  lineage, basis-drift blocking, checkpoint identity, validation, and recovery
  posture for destructive review paths that Python refactor and Git flows may
  cite.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture added.
- Scoreboard row moved to `green` and `conditional_go`.
- Scoreboard row now cites the acceptance matrix, reference workspace seed,
  launch bundle, language pack, review packet template, known-limits packet,
  owning packet, and latest capture.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
- `artifacts/milestones/m2/known_limits_alpha.yaml` is unchanged; notebook
  posture remains `known_limit:external_alpha.notebook_handoff_only`.
