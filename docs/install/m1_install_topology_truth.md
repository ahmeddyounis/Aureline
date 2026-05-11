# Install-topology truth surfaces

This page is the reviewer-facing entry point for the install-topology
truth seed. It quotes the seeded install-profile rows verbatim so
Help/About, diagnostics, support exports, release-evidence reviews, and
CLI help all read the same install-mode, channel, updater-owner,
binary-root, side-by-side, file-association, protocol-handler, durable
state-root, revert-path, and silent-deployment-baseline vocabulary as
the seed artifact.

A row is the truth source for the surface; copy that disagrees with the
row is wrong. Adding a supported install profile is additive-minor;
removing one is breaking and lands by narrowing fields, not by deleting
the row.

Reviewers should read this page top-to-bottom and then open the seed at
[`/artifacts/install/state_root_matrix.yaml`](../../artifacts/install/state_root_matrix.yaml)
for the full row vocabulary.

## Canonical sources

- [`/artifacts/install/state_root_matrix.yaml`](../../artifacts/install/state_root_matrix.yaml)
  — canonical install-topology truth rows (entries + envelope).
- [`/schemas/install/install_topology_truth.schema.json`](../../schemas/install/install_topology_truth.schema.json)
  — envelope schema (vocabularies, required coverage, named consumers).
- [`/schemas/install/install_topology_truth_row.schema.json`](../../schemas/install/install_topology_truth_row.schema.json)
  — row schema (install mode, channel, updater owner, binary root,
  side-by-side relation, file/protocol association ownership, revert
  path, silent-deployment baseline, durable state-root refs, truth
  surfaces, named failure drill).
- [`/design/m1/silent_deployment_baseline.md`](../../design/m1/silent_deployment_baseline.md)
  — silent-deployment design packet that records the baseline behaviour
  and the limits the M1 product publishes honestly.

Upstream sources the seed projects against (it does not fork their
vocabularies):

- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  — install-profile cards (install mode, channel, updater owner,
  binary root, side-by-side relation, silent install support).
- [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
  — durable state-root classes, file-association registration class,
  protocol-handler ownership class, and per-channel separation rules.
- [`/artifacts/release/silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml)
  — return-code family, failure-reason, and remediation pointer
  vocabularies that the silent baseline resolves into.

## Closed vocabularies

The seed pins the following closed vocabularies; the validation lane
asserts agreement with the row schema `$defs`.

| Axis | Values |
| --- | --- |
| `install_mode_class` | `per_user_installed`, `per_machine_installed`, `portable`, `offline_bundle`, `managed_deployed`, `side_by_side_preview` |
| `channel_class` | `stable`, `preview`, `beta`, `lts`, `portable_stable`, `portable_preview` |
| `updater_owner_class` | `user`, `admin`, `managed_fleet`, `external_package_manager`, `none_portable` |
| `binary_root_class` | `per_user_profile_program_area`, `per_machine_program_area`, `portable_colocated`, `managed_image_root`, `offline_bundle_root` |
| `side_by_side_relation_class` | `none`, `stable_and_preview`, `preview_paired_with_stable`, `portable_isolated`, `managed_image_isolated` |
| `file_association_ownership_class` | `per_channel_namespaced`, `not_registered`, `managed_policy_owned` |
| `protocol_handler_ownership_class` | `per_channel_namespaced`, `not_registered`, `managed_policy_owned` |
| `revert_path_class` | `in_app_revert`, `package_manager_revert`, `portable_swap`, `managed_pin_to_prior_build`, `unsupported` |
| `silent_deployment_baseline_class` | `not_supported`, `scriptable_baseline_with_exit_codes`, `managed_silent_partial`, `portable_swap_no_silent_required` |
| `m1_truth_surface_class` | `help_about`, `diagnostics`, `support_export`, `release_evidence`, `cli_help` |

`managed_silent_full` is **intentionally absent** from the silent-
deployment vocabulary. The M1 product does not yet execute full silent
managed deployment with fleet-wide rollback orchestration, and any row
that claims that depth misleads operators. The validation lane asserts
the token stays out of every row and out of the closed vocabulary.

## Required coverage

The seed asserts the following coverage rules:

- `required_install_mode_coverage` — every member of
  `{per_user_installed, per_machine_installed, portable,
  managed_deployed, side_by_side_preview}` MUST appear at least once
  across the rows.
- `required_m1_truth_surface_coverage` — every member of `{help_about,
  diagnostics, support_export, release_evidence}` MUST appear at least
  once across the rows.
- Every row MUST list `help_about` so users can inspect install
  topology from the About pane.
- `managed_deployed` rows MUST NOT publish `revert_path_class =
  'unsupported'` (they publish `managed_pin_to_prior_build` or
  `package_manager_revert`).
- `portable` rows MUST publish `revert_path_class = 'portable_swap'` and
  `silent_deployment_baseline_class = 'portable_swap_no_silent_required'`.
- `side_by_side_preview` rows MUST publish a non-`none`
  `side_by_side_relation_class` and a non-null `paired_channel_class`.

## Seeded install-truth profile rows

| `install_truth_profile_id` | Mode | Channel | Updater | Side-by-side | Revert path | Silent baseline | Drill |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `per_user_installed.stable` | `per_user_installed` | `stable` | `user` | `stable_and_preview` (with `preview`) | `in_app_revert` | `scriptable_baseline_with_exit_codes` | `per_user_installed_state_roots_dropped` |
| `per_machine_installed.stable` | `per_machine_installed` | `stable` | `admin` | `stable_and_preview` (with `preview`) | `package_manager_revert` | `scriptable_baseline_with_exit_codes` | `per_machine_installed_updater_owner_dropped` |
| `portable.portable_stable` | `portable` | `portable_stable` | `none_portable` | `portable_isolated` | `portable_swap` | `portable_swap_no_silent_required` | `portable_silent_baseline_widened_to_managed_silent_full` |
| `managed_deployed.stable` | `managed_deployed` | `stable` | `managed_fleet` | `managed_image_isolated` (no pair) | `managed_pin_to_prior_build` | `managed_silent_partial` | `managed_deployed_revert_path_dropped_to_unsupported` |
| `side_by_side_preview.preview` | `side_by_side_preview` | `preview` | `user` | `preview_paired_with_stable` (with `stable`) | `in_app_revert` | `scriptable_baseline_with_exit_codes` | `side_by_side_preview_relation_dropped_to_none` |
| `offline_bundle.stable` | `offline_bundle` | `stable` | `user` | `none` | `package_manager_revert` | `scriptable_baseline_with_exit_codes` | `offline_bundle_help_about_surface_dropped` |

## Durable state-root coverage

Each row publishes the durable state-root class refs Help/About,
diagnostics, and support exports MUST surface. The refs resolve into
[`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml);
this seed does not fork the state-root vocabulary.

| Row | Durable state-root class refs |
| --- | --- |
| `per_user_installed.stable` | `per_user_configuration_root`, `per_user_recovery_root`, `per_user_derived_cache_root`, `per_user_keychain_or_secret_store` |
| `per_machine_installed.stable` | `per_machine_admin_policy_root`, `per_machine_shared_data_root`, `per_user_configuration_root`, `per_user_keychain_or_secret_store` |
| `portable.portable_stable` | `portable_colocated_root` |
| `managed_deployed.stable` | `per_machine_admin_policy_root`, `per_machine_shared_data_root`, `per_user_configuration_root` |
| `side_by_side_preview.preview` | `per_user_configuration_root`, `per_user_recovery_root`, `per_user_derived_cache_root` |
| `offline_bundle.stable` | `offline_bundle_mirror_metadata_root`, `per_user_configuration_root`, `per_user_recovery_root` |

## How changes are reviewed and propagated

1. **Propose the change in the seed row**, not in copy. Any change to a
   row's install-mode class, channel, updater owner, binary root,
   side-by-side relation, association ownership, revert path, silent
   baseline, durable state-root refs, or M1 truth surfaces lands in
   `artifacts/install/state_root_matrix.yaml` first.
2. **Re-run the validation lane.** The lane fails closed on missing
   vocabulary, missing state roots, dropped `help_about` surface,
   `managed_deployed` rows widened to `unsupported` revert, portable
   rows widened to `managed_silent_full`, and side-by-side rows that
   drop their paired-channel disclosure.
3. **Propagate to copy after the lane passes.** Help/About, diagnostics,
   support exports, release-evidence reviews, and CLI help consume the
   row's classes verbatim. Surfaces that disagree with the row are
   non-conforming.
4. **Retire by editing, not deleting.** Narrow the row's
   `m1_truth_surfaces` and adjust the upstream install-profile card
   referenced in `install_profile_card_ref` first.

## Validation lane

- Runner: [`tests/governance/m1_install_topology_truth_lane/run_m1_install_topology_truth_lane.py`](../../tests/governance/m1_install_topology_truth_lane/run_m1_install_topology_truth_lane.py)
- Latest capture:
  [`artifacts/milestones/m1/captures/install_topology_truth_validation_capture.json`](../../artifacts/milestones/m1/captures/install_topology_truth_validation_capture.json)
- Owning packet:
  [`artifacts/milestones/m1/proof_packets/install_topology_truth.md`](../../artifacts/milestones/m1/proof_packets/install_topology_truth.md)

Run the lane:

```bash
python3 tests/governance/m1_install_topology_truth_lane/run_m1_install_topology_truth_lane.py --repo-root .
```

Force a named failure drill:

```bash
python3 tests/governance/m1_install_topology_truth_lane/run_m1_install_topology_truth_lane.py \
    --repo-root . \
    --force-drill <install_truth_profile_id>:<drill_id>
```

Under `--force-drill` the runner exits 0 only when the row's declared
`expected_check_id` is reproduced from the forced input. Drift on the
unforced rows still fails the lane.
