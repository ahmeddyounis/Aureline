# Trust-policy and install-topology smoke

Unattended proof lane that replays the smoke rows in
[`fixtures/install/m1_topology_rows.yaml`](../../../fixtures/install/m1_topology_rows.yaml)
against the canonical trust-state matrix, install-topology matrix,
state-root map, credential-store capability matrix, claimed-desktop
profile roster, native-trust integration matrix, local-baseline proof,
and the dogfood matrix.

The lane is deliberately runnable on CI/nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the matrix the runner asserts:

- **Install topology is inspectable** — the row's `install_truth_card_id`
  resolves to an `install_profile_card` whose `install_mode_class`,
  `channel_class`, `side_by_side_relation_class`, and
  `durable_state_root_refs` match the row's expectations.
- **State-root ownership is honest** — every
  `expected_durable_state_root_ref` resolves to a row in the state-root
  map whose `owning_channels` admits the smoke row's channel, or whose
  `shared_across_channels` flag is explicitly set.
- **Trust state is in vocabulary** — the row's `trust_state` is a value
  the trust-state matrix declares.
- **Managed shells stay honest** — a `managed_signed_in` row that admits
  `admin_policy_read` MUST carry a degraded/restricted `trust_state`.
- **Credential store posture is in vocabulary** — `unlock_state` is one
  of `locked`, `unlocking`, `unlocked`, `step_up_required`,
  `degraded_session_only`, or `unavailable`.
- **Local-baseline floor is admitted** — every row admits
  `file_open_and_edit`, `file_save_to_local_disk`, `local_search`,
  `local_git_read`, and `support_bundle_export_local_only` regardless
  of trust or credential posture.
- **Denials are typed** — every entry in `expected_blocked_surfaces`
  carries a typed `denial_reason` and a non-empty
  `actionable_explanation`.
- **Portable rows never mutate the host** — portable rows reference
  only `portable_colocated_root` state-root rows.
- **Side-by-side channels never collide on state** — Preview rows
  reference Preview-owned state roots; cross-channel state sharing is a
  named failure (`side_by_side.state_root_owning_channel_collision`).
- **Every required smoke dimension is exercised** — local-first,
  managed shell, locked / unavailable credential store, policy-blocked
  action, side-by-side channels, portable no-host-mutation, and
  safe-mode / restricted preview must each have at least one row.
- **Failure drills are reproducible** — every row names one drill in
  `failure_drill_id_vocabulary` plus the precise `expected_check_id`
  the runner reproduces when the drill is forced with `--force-drill`.

## Run

```bash
python3 tests/smoke/trust_policy_install_topology/run_trust_policy_install_topology_smoke.py \
    --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/install_topology_smoke_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/smoke/trust_policy_install_topology/run_trust_policy_install_topology_smoke.py \
    --repo-root . \
    --force-drill <smoke_row_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input. Use
this to prove the lane fails loudly on real regressions.

Optional flags:

- `--matrix <path>` — point at an alternate matrix file.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is
  embedded in the capture (defaults to
  `artifacts/build/build_identity.json`).

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `artifacts/ops/m1_install_topology_smoke_report.md` |
| Smoke matrix | `fixtures/install/m1_topology_rows.yaml` |
| Latest capture | `artifacts/milestones/m1/captures/install_topology_smoke_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/install_topology_smoke.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.install_topology_smoke` so reviewers can find the
latest capture, owner, and validation-lane reference without searching
ad hoc folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `artifacts/security/trust_state_matrix.yaml`
- `artifacts/release/install_topology_matrix.yaml`
- `artifacts/release/state_root_map.yaml`
- `fixtures/auth/credential_state_cases/store_capability_matrix.yaml`
- `artifacts/platform/native_trust_integration_matrix.yaml`
- `artifacts/managed/local_baseline_proof.yaml`
- `artifacts/milestones/m1/dogfood_matrix.yaml`

Stale captures are surfaced by the artifact-index validator at
`ci/check_m1_artifact_index.py`.
