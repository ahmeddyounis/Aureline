# Harden environment-capsule resolution — M4 reviewer artifact

This artifact summarizes the checked-in stable capsule-resolution truth
packet for release reviewers. The canonical packet is
[`harden_environment_capsule_resolution_truth_packet.json`](./harden_environment_capsule_resolution_truth_packet.json);
the reviewer-facing contract is at
[`docs/runtime/m4/harden-environment-capsule-resolution.md`](../../../docs/runtime/m4/harden-environment-capsule-resolution.md).

## What the packet promises

For each of the five capsule-resolution lanes (`devcontainer_lane`,
`nix_lane`, `compose_lane`, `shell_sdk_lane`,
`template_prebuild_lane`) the packet certifies:

- One `capsule_resolution_quality` row at `launch_stable` with
  `release_evidence_review` evidence and
  `auto_block_on_missing_evidence` automation.
- Nine `capsule_field_admission` rows covering every required typed
  capsule field: `host_or_base_image_identity`, `target_plan`,
  `resolved_toolchain_locks`, `projected_environment_variables`,
  `secret_references`, `writable_mount_model`,
  `service_startup_ordering`, `trust_network_posture`, `provenance`.
  Each row binds `auto_narrow_on_capsule_field_gap` automation
  against `conformance_suite_evidence`.
- Six `prebuild_fingerprint_admission` rows covering every prebuild
  fingerprint component: `commit_or_tree_identity`, `capsule_hash`,
  `platform_arch`, `policy_epoch`, `extension_lock_digest`,
  `critical_toolchain_digest`. Each row binds
  `auto_narrow_on_fingerprint_gap` automation against
  `conformance_suite_evidence` so a mismatched fingerprint always
  forces a visible cold or partially-warm path.
- Six `invalidation_reason_admission` rows covering every visible
  invalidation reason: `cold_path`, `partially_warm_path`,
  `fingerprint_mismatch`, `untrusted_template_metadata`,
  `blocked_hook`, `stale_prebuild`. Each row binds
  `auto_narrow_on_invalidation_reason_gap` automation against
  `failure_recovery_drill_evidence`.
- Five `project_doctor_finding_admission` rows covering every
  Project Doctor finding code: `wrong_interpreter`, `stale_prebuild`,
  `blocked_activator`, `drifted_toolchain`,
  `untrusted_template_metadata`. Each row binds
  `auto_narrow_on_project_doctor_finding_gap` automation against
  `failure_recovery_drill_evidence`.
- One `materialized_identity_admission` row binding
  `auto_narrow_on_materialized_identity_drift` against
  `automated_functional_evidence` and carrying non-empty
  `requested_artifact_identity_binding`,
  `materialized_runtime_identity_binding`, and
  `no_silent_prebuild_reuse: true` so the requested template /
  capsule / prebuild artifact identity is always distinguishable from
  the materialized runtime instance and silent prebuild reuse is
  refused.

Nine consumer projections (`editor_run_surface`, `terminal_pane`,
`task_panel`, `cli_headless`, `project_doctor`, `support_export`,
`release_proof_index`, `help_about`, `conformance_dashboard`)
preserve the packet id and every vocabulary verbatim.

## Promotion state

`stable` across all five lanes, with zero validation findings. The
support export bundles the packet without raw command lines, raw
process env bytes, raw capsule bodies, secrets, or ambient
credentials.

## Narrowed-below-stable drills

The fixture corpus at
[`fixtures/runtime/m4/harden_environment_capsule_resolution/`](../../../fixtures/runtime/m4/harden_environment_capsule_resolution/)
exercises six narrowed-below-stable postures:

1. `launch_stable_with_unbound_evidence_blocks_stable.json` — the
   devcontainer lane's quality row claims `launch_stable` while its
   evidence is `evidence_unbound`. The packet demotes to
   `blocks_stable` with `missing_evidence_class` +
   `launch_stable_with_unbound_binding` findings.
2. `missing_prebuild_fingerprint_component_blocks_stable.json` — the
   devcontainer lane drops its `critical_toolchain_digest`
   prebuild-fingerprint admission. `blocks_stable` with
   `missing_prebuild_fingerprint_coverage`.
3. `materialized_identity_admits_silent_prebuild_reuse_blocks_stable.json`
   — the devcontainer lane's materialized-identity row drops the
   `no_silent_prebuild_reuse` attestation. `blocks_stable` with
   `materialized_identity_admission_admits_silent_prebuild_reuse` and
   `missing_materialized_identity_admission`.
4. `narrowed_row_missing_disclosure_ref_blocks_stable.json` — the
   devcontainer lane's quality row narrows to `launch_stable_below`
   and drops its disclosure ref. `blocks_stable` with
   `narrowed_row_missing_disclosure_ref` and
   `downgrade_automation_missing_disclosure_ref`.
5. `projection_collapses_invalidation_reason_vocabulary_blocks_stable.json`
   — the `project_doctor` projection drops the invalidation-reason
   vocabulary. `blocks_stable` with
   `invalidation_reason_vocabulary_collapsed`,
   `missing_consumer_projection`, and `consumer_projection_drift`.
6. `raw_source_material_blocks_stable.json` — the devcontainer
   lane's quality row admits raw command lines / env bytes / capsule
   bodies past the boundary. `blocks_stable` with
   `raw_source_material_present`.

## Where the packet lands

- Editor run surface: per-pane "why this capsule?" chip reads the
  lane's capsule-field, prebuild-fingerprint, and invalidation-reason
  rows.
- Terminal pane / task panel: read the materialized-identity binding
  to show the requested vs. materialized identity on every run.
- CLI/headless inspector (`aureline env inspect`): projects the
  packet for `aureline env inspect --explain` and headless launch
  flows.
- Project Doctor: ingests the project-doctor-finding admissions and
  invalidation reasons so stale prebuilds, blocked hooks, drifted
  toolchains, and untrusted template metadata each carry a stable
  finding code.
- Support export: bundles the packet verbatim (raw private material
  excluded).
- Help/About proof card: reads the materialized-identity and
  fingerprint admissions.
- Release proof index, conformance dashboard: cite the packet id and
  the nine vocabularies.

## How to regenerate

```bash
python3 tools/regenerate_harden_environment_capsule_resolution_truth_packet.py
cargo test -p aureline-runtime --test harden_environment_capsule_resolution_truth_packet
```

The generator is the canonical seed for both the artifact and the
fixture corpus; the Rust contract validates either one and refuses to
publish unless every required row, binding, projection, and disclosure
ref is in place.
