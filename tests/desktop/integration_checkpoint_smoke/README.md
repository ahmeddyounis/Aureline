# Integration-checkpoint smoke

Unattended cross-subsystem proof lane that replays the frozen corpus in
[`corpus/integration_checkpoint_corpus.yaml`](corpus/integration_checkpoint_corpus.yaml)
against six subsystem inputs and proves the **integration** requirement that
every isolated Beta gate cannot: that six subsystems degrade *honestly
together*.

Every Beta subsystem is already gated on its own. A narrower cross-subsystem
lane (`tests/smoke/trust_policy_install_topology/`) covers three of these six.
This lane composes all six:

- **Extension install / disable / update** — `crates/aureline-extensions/src/{runtime,manifest_baseline,revocation}`
- **Trust / restricted mode** — `crates/aureline-shell/tests/workspace_trust_beta_fixtures.rs` + `aureline-auth` trust modules
- **Packaging / update / rollback** — `crates/aureline-install/src/{rollback,topology,repair_verify}` + `crates/aureline-release/src/release_center_model`
- **Enterprise proxy / policy path** — `crates/aureline-auth/src/policy_packs` + `crates/aureline-policy/src/{authority,simulation}`
- **Support bundle** — `crates/aureline-support/src/bundle`
- **Remote attach** — `crates/aureline-runtime/src/{remote_helper_skew_beta,capability_negotiation}`

The lane is deliberately runnable on CI without a graphical display: it only
consumes the pure data joined out of the corpus. There is no running app.

## What the lane proves

The corpus carries one all-green integrated case plus one case per subsystem
in which exactly that subsystem degrades. For every case the runner asserts:

- **The degraded subsystem reports it** — exactly the declared
  `degraded_subsystem` reports `degraded`, with a typed `degradation_reason`
  drawn from its own vocabulary and a non-empty explanation. A degraded
  subsystem that reports `healthy`, or contributes a clean `go`, is silent
  success and fails (`integration_checkpoint.degraded.*`).
- **The other five surface it (no silent success)** — every healthy peer must
  observe and acknowledge the degradation (`peer_degradation_observed` with a
  non-empty acknowledgement). A healthy peer that stays silent fails
  (`integration_checkpoint.peer.silent_success`).
- **The joint verdict is honest** — the verdict derived from the
  per-subsystem contributions matches the case's declared
  `expected_joint_verdict`, and a degraded case never resolves to `go`
  (`integration_checkpoint.joint.*`). The verdict vocabulary reuses the
  existing go / no-go projection vocabulary: `go`, `conditional_go`, `no_go`.
- **The joint verdict is surfaced consistently** — every consuming projection
  (`release_center_readiness`, `shell_readiness_chip`,
  `support_bundle_summary`) surfaces the same joint verdict; divergence fails
  (`integration_checkpoint.projection.verdict_divergence`).
- **All six subsystems are integrated** — the corpus must declare exactly the
  six subsystems and one all-green case, and every subsystem must be exercised
  as the degraded subsystem in some case. Deleting a subsystem `input_ref`, or
  stubbing a case posture inside it, fails the lane
  (`integration_checkpoint.subsystem.*`, `...coverage.*`).

## Run

```bash
python3 tests/desktop/integration_checkpoint_smoke/run_integration_checkpoint_smoke.py \
    --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/integration/integration_checkpoint_smoke_validation_capture.json`
and exits non-zero on any check failure. The capture is byte-identical for a
given corpus (no wall-clock fields), so re-running on an unchanged corpus
reproduces the same bytes.

### Prove the lane fails closed

The checkpoint only integrates all six subsystems if removing any one fails.
Demonstrate it without mutating files:

```bash
python3 tests/desktop/integration_checkpoint_smoke/run_integration_checkpoint_smoke.py \
    --repo-root . \
    --omit-subsystem remote_attach
```

`--omit-subsystem` may be repeated. Deleting any `corpus/subsystems/*.yaml`
file produces the same non-zero exit.

Optional flags:

- `--corpus <path>` — point at an alternate corpus file.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is referenced
  in the capture (defaults to `artifacts/build/build_identity.json`).

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Corpus (cases) | `tests/desktop/integration_checkpoint_smoke/corpus/integration_checkpoint_corpus.yaml` |
| Subsystem inputs | `tests/desktop/integration_checkpoint_smoke/corpus/subsystems/*.yaml` |
| Latest capture | `artifacts/integration/integration_checkpoint_smoke_validation_capture.json` |
| CI lane | `.github/workflows/integration_checkpoint_smoke.yml` |

## Refresh policy

Re-run the lane (and refresh the capture) when any subsystem input under
`corpus/subsystems/` or the case matrix in
`corpus/integration_checkpoint_corpus.yaml` changes.
