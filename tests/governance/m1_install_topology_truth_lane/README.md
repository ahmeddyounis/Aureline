# M1 install-topology truth-surface seed validation lane

Unattended proof lane that validates the M1 install-topology truth seed
at
[`artifacts/install/state_root_matrix.yaml`](../../../artifacts/install/state_root_matrix.yaml)
against:

- [`schemas/install/install_topology_truth.schema.json`](../../../schemas/install/install_topology_truth.schema.json) — envelope schema (vocabularies, required coverage, named consumers);
- [`schemas/install/install_topology_truth_row.schema.json`](../../../schemas/install/install_topology_truth_row.schema.json) — row vocabulary; and
- the canonical landing page at
  [`docs/install/m1_install_topology_truth.md`](../../../docs/install/m1_install_topology_truth.md)
  plus the silent-deployment design packet at
  [`design/m1/silent_deployment_baseline.md`](../../../design/m1/silent_deployment_baseline.md)
  so the seed cannot quietly point at a missing reviewer entry.

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the seed the runner asserts:

- **Envelope is canonical** — `record_kind` is
  `install_topology_truth_row_record` and
  `install_topology_truth_row_schema_version` is `1`.
- **`install_truth_profile_id` is unique** and matches the row schema's
  pattern.
- **Closed vocabularies match the row schema verbatim** for
  `install_mode_class`, `channel_class`, `updater_owner_class`,
  `binary_root_class`, `side_by_side_relation_class`,
  `file_association_ownership_class`,
  `protocol_handler_ownership_class`, `revert_path_class`,
  `silent_deployment_baseline_class`, and the
  `m1_truth_surface_class` enum.
- **Owner / state-root / surface coverage is present** — `owner_dri` is
  a non-empty `@handle`, `durable_state_root_class_refs` is non-empty,
  and `m1_truth_surfaces` is non-empty and contains `help_about`.
- **Upstream install-profile card resolves** — the base path in
  `install_profile_card_ref`
  (`artifacts/release/install_topology_matrix.yaml`) exists on disk.
- **paired_channel_class agrees with side_by_side_relation_class** —
  null when relation is `none`, non-null and in the channel vocabulary
  otherwise.
- **side_by_side_preview rows disclose the paired channel** — relation
  is non-`none` and `paired_channel_class` is non-null.
- **managed_deployed rows don't claim unsupported revert** —
  `revert_path_class` is in
  `{managed_pin_to_prior_build, package_manager_revert}`.
- **portable rows publish the portable baseline** — `revert_path_class
  = 'portable_swap'` and `silent_deployment_baseline_class =
  'portable_swap_no_silent_required'`.
- **M1 silent-deployment baseline is honoured** — no row publishes
  `silent_deployment_baseline_class = 'managed_silent_full'` and the
  envelope vocabulary does not contain the forbidden token.
- **Failure drill is well-formed** — `drill_id` is in
  `failure_drill_id_vocabulary`, `forced_input` declares at least one
  drift, and `expected_check_id` and `actionable_next_action` are
  non-empty.

Envelope coverage:

- **Required install-mode coverage** — every member of
  `required_install_mode_coverage`
  (`per_user_installed, per_machine_installed, portable,
  managed_deployed, side_by_side_preview`) appears at least once
  across the entries.
- **Required M1 truth-surface coverage** — every member of
  `required_m1_truth_surface_coverage`
  (`help_about, diagnostics, support_export, release_evidence`) appears
  on at least one row.
- **Named runtime consumers exist** — every
  `named_runtime_consumers[].consumer_ref` resolves on disk and
  `consumed_fields` is non-empty.

## Run

```bash
python3 tests/governance/m1_install_topology_truth_lane/run_m1_install_topology_truth_lane.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/install_topology_truth_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/governance/m1_install_topology_truth_lane/run_m1_install_topology_truth_lane.py \
    --repo-root . \
    --force-drill <install_truth_profile_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input.

| Row (`install_truth_profile_id`) | Drill | Expected check id |
| --- | --- | --- |
| `per_user_installed.stable` | `install_topology_truth_drill.per_user_installed_state_roots_dropped` | `install_topology_truth.durable_state_root_class_refs_required` |
| `per_machine_installed.stable` | `install_topology_truth_drill.per_machine_installed_updater_owner_dropped` | `install_topology_truth.updater_owner_class_required` |
| `portable.portable_stable` | `install_topology_truth_drill.portable_silent_baseline_widened_to_managed_silent_full` | `install_topology_truth.silent_deployment_baseline_managed_silent_full_blocked_in_baseline` |
| `managed_deployed.stable` | `install_topology_truth_drill.managed_deployed_revert_path_dropped_to_unsupported` | `install_topology_truth.revert_path_class_unsupported_blocked_for_managed_deployed` |
| `side_by_side_preview.preview` | `install_topology_truth_drill.side_by_side_preview_relation_dropped_to_none` | `install_topology_truth.side_by_side_preview_relation_must_disclose_paired_channel` |
| `offline_bundle.stable` | `install_topology_truth_drill.offline_bundle_help_about_surface_dropped` | `install_topology_truth.help_about_truth_surface_required` |

Optional flags:

- `--matrix <path>` — point at an alternate seed file.
- `--envelope-schema <path>` — alternate envelope schema.
- `--row-schema <path>` — alternate row schema.
- `--build-identity <path>` — alternate build-identity record.
- `--report <path>` — change the capture output path.

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `docs/install/m1_install_topology_truth.md` |
| Silent-deployment design packet | `design/m1/silent_deployment_baseline.md` |
| Seed (canonical) | `artifacts/install/state_root_matrix.yaml` |
| Envelope schema | `schemas/install/install_topology_truth.schema.json` |
| Row schema | `schemas/install/install_topology_truth_row.schema.json` |
| Latest capture | `artifacts/milestones/m1/captures/install_topology_truth_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/install_topology_truth.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.install_topology_truth` so reviewers can find the
latest capture, owner, and validation-lane reference without searching
ad hoc folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `artifacts/install/state_root_matrix.yaml`
- `schemas/install/install_topology_truth.schema.json`
- `schemas/install/install_topology_truth_row.schema.json`
- `docs/install/m1_install_topology_truth.md`
- `design/m1/silent_deployment_baseline.md`
- any upstream source the seed projects against
- any named runtime consumer's path or read fields
