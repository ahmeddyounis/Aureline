# M1 internal-boundary manifest validation lane

Unattended proof lane that validates
[`artifacts/governance/m1_open_local_capability_matrix.yaml`](../../../artifacts/governance/m1_open_local_capability_matrix.yaml)
against
[`schemas/governance/boundary_manifest.schema.json`](../../../schemas/governance/boundary_manifest.schema.json)
and the canonical truth sources the matrix joins:

- [`artifacts/governance/deployment_profiles.yaml`](../../../artifacts/governance/deployment_profiles.yaml)
  — the frozen deployment-profile vocabulary;
- [`artifacts/governance/residual_dependencies.yaml`](../../../artifacts/governance/residual_dependencies.yaml)
  — the canonical residual-dependency ledger (dependency classes,
  posture classes, and the per-profile postures the matrix may not
  relax).

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the matrix the runner asserts:

- **`row_id` is namespaced** under `boundary_row:` and is unique.
- **Closed vocabularies are honored** — `surface_family`,
  `boundary_class`, `manifest_status`, every `carries_truth_fields`
  entry, and every `failure_drill.drill_id` resolve against the
  matrix's closed vocabularies (which are themselves the vocabularies
  frozen in the schema).
- **Deployment profiles match the register** — every entry in
  `deployment_profiles` resolves against
  `artifacts/governance/deployment_profiles.yaml#deployment_profile_vocabulary`,
  and the matrix's vocabulary matches the register one-for-one.
- **`local_only` rows have no residual surface** — rows with
  `boundary_class = local_only` declare `residual_dependencies: []`.
- **`local_core_continuity` is present** — every row declares a
  non-empty `local_core_continuity` so absence narrows a claim rather
  than silently failing.
- **Residual dependencies are well-formed** — every
  `residual_dependencies[].dependency_class` resolves against the
  ledger's `dependency_class_vocabulary`; every `per_profile_posture`
  covers every frozen deployment profile with values in the closed
  posture vocabulary; every entry declares a non-empty
  `absence_impact`.
- **The matrix does not relax the ledger** — when the residual-
  dependency ledger declares a posture of `forbidden` or
  `not_applicable_structural` for a (`dependency_class`, profile) pair,
  the matrix MUST honor it verbatim. The ledger is canonical.
- **Carries-truth fields are real** — every row's
  `carries_truth_fields` list is non-empty and every entry resolves
  against `truth_field_vocabulary`.
- **Named runtime consumer is real** — the
  `consumer_bindings.named_runtime_consumer.consumer_ref` exists on
  disk and declares a non-empty `consumed_fields` list whose entries
  resolve against `truth_field_vocabulary`.
- **Required surface coverage** — at least one row exists for each
  member of `required_surface_family_coverage`
  (`shell`, `editor`, `workspace`, `search`, `terminal`, `support`).
- **Failure drills are reproducible** — every row names one drill in
  `failure_drill_id_vocabulary` plus the precise `expected_check_id`
  the runner reproduces when the drill is forced with `--force-drill`.

## Run

```bash
python3 tests/governance/m1_boundary_manifest_lane/run_m1_boundary_manifest_lane.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/boundary_manifest_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/governance/m1_boundary_manifest_lane/run_m1_boundary_manifest_lane.py \
    --repo-root . \
    --force-drill <row_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input. Use
this to prove the lane fails loudly on real regressions.

| Row | Drill | Expected check id |
|---|---|---|
| `boundary_row:shell.frame_and_status`               | `shell_drill.local_only_relaxed_to_managed`     | `boundary_manifest.local_only_must_have_no_residual_dependencies` |
| `boundary_row:editor.buffer_and_save`               | `editor_drill.local_core_continuity_dropped`    | `boundary_manifest.local_core_continuity_must_be_present`         |
| `boundary_row:workspace.vfs_and_local_git`          | `workspace_drill.posture_relaxes_forbidden`     | `boundary_manifest.posture_relaxes_ledger_forbidden`              |
| `boundary_row:search.local_index_and_grep`          | `search_drill.dependency_class_unknown`         | `boundary_manifest.residual_dependency_class_unknown`             |
| `boundary_row:terminal.local_pty_and_auth_callback` | `terminal_drill.boundary_class_unknown`         | `boundary_manifest.boundary_class_unknown`                        |
| `boundary_row:support.bundle_export_and_about`      | `support_drill.truth_field_unknown`             | `boundary_manifest.truth_field_unknown`                           |

Optional flags:

- `--matrix <path>` — point at an alternate matrix file.
- `--schema <path>` — point at an alternate schema file.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is
  embedded in the capture (defaults to
  `artifacts/build/build_identity.json`).

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `docs/governance/m1_boundary_manifest.md` |
| Capability matrix | `artifacts/governance/m1_open_local_capability_matrix.yaml` |
| Boundary-manifest schema | `schemas/governance/boundary_manifest.schema.json` |
| Latest capture | `artifacts/milestones/m1/captures/boundary_manifest_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/boundary_manifest.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.boundary_manifest` so reviewers can find the latest
capture, owner, and validation-lane reference without searching ad hoc
folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `artifacts/governance/m1_open_local_capability_matrix.yaml`
- `schemas/governance/boundary_manifest.schema.json`
- `artifacts/governance/deployment_profiles.yaml`
- `artifacts/governance/residual_dependencies.yaml`
- `docs/governance/m1_boundary_manifest.md`
