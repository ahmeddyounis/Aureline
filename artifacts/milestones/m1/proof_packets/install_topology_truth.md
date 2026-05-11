# Proof packet: M1 install-topology truth surfaces and silent-deployment baseline

Purpose: anchor proof captures for the unattended M1 lane that
validates the canonical install-topology truth seed and the silent-
deployment design packet. The lane proves the seed is consumable by
Help/About, diagnostics, support exports, release-evidence reviews,
CLI help, and the M1 design packet — without re-encoding the install-
mode, channel, updater-owner, side-by-side, association-ownership,
state-root, revert-path, or silent-deployment baseline vocabulary on
each surface.

Reviewer entry point:
[`/docs/install/m1_install_topology_truth.md`](../../../docs/install/m1_install_topology_truth.md).
Silent-deployment design packet:
[`/design/m1/silent_deployment_baseline.md`](../../../design/m1/silent_deployment_baseline.md).

## Canonical sources

- `artifacts/install/state_root_matrix.yaml` — seed rows the runner
  consumes. Carries:
  - the M1 envelope (`schema_version`, `matrix_id`, `owner_dri`,
    `overview_page`, `silent_deployment_design_packet_ref`,
    `install_profile_card_source`, `state_root_map_source`,
    `row_schema_ref`, `build_identity_ref`, `validation_lane_ref`),
  - closed envelope vocabularies for install-mode, channel, updater-
    owner, binary-root, side-by-side relation, file-association
    ownership, protocol-handler ownership, revert-path,
    silent-deployment baseline, M1 truth surface, and failure-drill
    id,
  - required coverage lists (`required_install_mode_coverage`,
    `required_m1_truth_surface_coverage`),
  - the named runtime consumers the seed asserts are live, and
  - one install-truth profile row per supported install profile with
    the uniform `(install_truth_profile_id, install_mode_class,
    channel_class, updater_owner_class, binary_root_class,
    side_by_side_relation_class, paired_channel_class,
    file_association_ownership_class,
    protocol_handler_ownership_class, revert_path_class,
    silent_deployment_baseline_class,
    durable_state_root_class_refs, m1_truth_surfaces, owner_dri,
    install_profile_card_ref, failure_drill)` shape.

- `schemas/install/install_topology_truth.schema.json` — envelope
  schema; freezes vocabularies, required coverage lists, named
  consumer shape, matrix identity, and pins the canonical landing
  page and silent-deployment design packet paths.

- `schemas/install/install_topology_truth_row.schema.json` — row
  schema; freezes the closed install-mode, channel, updater-owner,
  binary-root, side-by-side-relation, file-association,
  protocol-handler, revert-path, silent-deployment-baseline, and
  truth-surface vocabularies plus the conditional invariants
  (side_by_side_preview disclosure, managed_deployed revert path,
  portable revert / silent baseline pins).

- `tests/governance/m1_install_topology_truth_lane/run_m1_install_topology_truth_lane.py`
  — unattended runner that replays the seed and emits the durable
  JSON capture.

## Upstream sources the seed projects against

- `artifacts/release/install_topology_matrix.yaml` — install-profile
  cards (install_mode, channel, updater_owner, binary_root,
  side_by_side_relation, silent_install_support). The seed's
  `install_profile_card_ref` on each row resolves against this file.
- `artifacts/release/state_root_map.yaml` — durable state-root
  classes, file-association registration class, protocol-handler
  ownership class, and per-channel separation rules.
- `artifacts/release/silent_deployment_seed.yaml` — return-code family,
  failure-reason, and remediation pointer vocabularies that the
  silent baseline resolves into.

## Named runtime consumers

- `docs/install/m1_install_topology_truth.md` — reviewer-facing
  landing page that quotes the seeded rows verbatim so Help/About,
  diagnostics, support exports, release-evidence reviews, and CLI
  help read the same install-topology vocabulary as the seed.
- `design/m1/silent_deployment_baseline.md` — human-readable design
  packet that records the baseline silent / managed deployment
  behaviour without claiming enterprise depth the M1 product cannot
  execute.
- `tests/governance/m1_install_topology_truth_lane/run_m1_install_topology_truth_lane.py`
  — live CI/review consumer (this lane) that replays the seed, asserts
  closed-vocabulary agreement, structural invariants, required
  coverage, upstream-source resolution, named-consumer resolution, and
  reproduces every named failure drill loudly.

## Live runtime consumers (read-only)

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/install_topology_truth_validation_capture.json`

## Install-mode coverage

The seed asserts the following install modes are present:

| `install_truth_profile_id` | Install mode | Channel | Updater | Revert | Silent baseline |
| --- | --- | --- | --- | --- | --- |
| `per_user_installed.stable` | `per_user_installed` | `stable` | `user` | `in_app_revert` | `scriptable_baseline_with_exit_codes` |
| `per_machine_installed.stable` | `per_machine_installed` | `stable` | `admin` | `package_manager_revert` | `scriptable_baseline_with_exit_codes` |
| `portable.portable_stable` | `portable` | `portable_stable` | `none_portable` | `portable_swap` | `portable_swap_no_silent_required` |
| `managed_deployed.stable` | `managed_deployed` | `stable` | `managed_fleet` | `managed_pin_to_prior_build` | `managed_silent_partial` |
| `side_by_side_preview.preview` | `side_by_side_preview` | `preview` | `user` | `in_app_revert` | `scriptable_baseline_with_exit_codes` |
| `offline_bundle.stable` | `offline_bundle` | `stable` | `user` | `package_manager_revert` | `scriptable_baseline_with_exit_codes` |

## Failure-drill coverage

Six named drills, all reproducible under
`--force-drill <install_truth_profile_id>:<drill_id>`:

| Row | Drill | Expected check id |
| --- | --- | --- |
| `per_user_installed.stable` | `install_topology_truth_drill.per_user_installed_state_roots_dropped` | `install_topology_truth.durable_state_root_class_refs_required` |
| `per_machine_installed.stable` | `install_topology_truth_drill.per_machine_installed_updater_owner_dropped` | `install_topology_truth.updater_owner_class_required` |
| `portable.portable_stable` | `install_topology_truth_drill.portable_silent_baseline_widened_to_managed_silent_full` | `install_topology_truth.silent_deployment_baseline_managed_silent_full_blocked_in_baseline` |
| `managed_deployed.stable` | `install_topology_truth_drill.managed_deployed_revert_path_dropped_to_unsupported` | `install_topology_truth.revert_path_class_unsupported_blocked_for_managed_deployed` |
| `side_by_side_preview.preview` | `install_topology_truth_drill.side_by_side_preview_relation_dropped_to_none` | `install_topology_truth.side_by_side_preview_relation_must_disclose_paired_channel` |
| `offline_bundle.stable` | `install_topology_truth_drill.offline_bundle_help_about_surface_dropped` | `install_topology_truth.help_about_truth_surface_required` |

## Refresh

Re-run the validation lane after a change to:

- the seed YAML,
- either schema (envelope or row),
- the reviewer-facing landing page,
- the silent-deployment design packet,
- any upstream source the seed projects against, or
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for closed-vocabulary membership
(install_mode_class, channel_class, updater_owner_class,
binary_root_class, side_by_side_relation_class,
file_association_ownership_class, protocol_handler_ownership_class,
revert_path_class, silent_deployment_baseline_class,
m1_truth_surface_class), the conditional invariants
(side_by_side_preview disclosure, managed_deployed revert path,
portable revert / silent baseline pins), the help_about coverage rule,
the silent-deployment baseline rule (no row publishes
`managed_silent_full`), the required install-mode and truth-surface
coverage rules, named-runtime-consumer existence, and its six named
failure drills.
