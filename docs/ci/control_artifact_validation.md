# Contract-artifact validation lane

This lane turns the repository's contract-bearing governance artifacts into a
repeatable local and CI gate. It validates package inventory coverage and
forbidden edges, control-artifact index drift, stable-surface ownership and
traceability, decision/source-anchor integrity, compatibility and
deployment-profile joins, boundary-row links, claim-manifest refs, and
command-parity tool health.

Companion artifacts:

- [`/tools/ci/validate_contract_artifacts.py`](../../tools/ci/validate_contract_artifacts.py)
  — shared validator entry point.
- [`/ci/contract_validation.sh`](../../ci/contract_validation.sh)
  — local and CI wrapper that writes a JSON report and captures the
  human-readable summary.
- [`/fixtures/ci/contract_validation/missing_deployment_profile.json`](../../fixtures/ci/contract_validation/missing_deployment_profile.json)
  — checked-in failing scenario used to prove the deployment-profile gate
  still trips.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — canonical ownership row for this lane.

## Prerequisites

- `python3`
- `ruby` (the validator parses YAML through Ruby's built-in `Psych`)

## Run locally

Use the shared wrapper:

```bash
./ci/contract_validation.sh --out-dir target/contract-validation
```

Artifacts land under `target/contract-validation/`:

- `contract_validation_summary.txt`
- `contract_validation_report.json`

For direct invocation or custom report paths:

```bash
python3 tools/ci/validate_contract_artifacts.py \
  --repo-root . \
  --report target/contract-validation/contract_validation_report.json
```

## Failing example

This checked-in scenario mutates one compatibility row to use a deployment
profile id that does not exist in the shared vocabulary. The validator must
exit non-zero and report `compatibility_matrix.deployment_profiles_resolve`.

```bash
python3 tools/ci/validate_contract_artifacts.py \
  --repo-root . \
  --scenario fixtures/ci/contract_validation/missing_deployment_profile.json \
  --report target/contract-validation/missing_deployment_profile_report.json
```

## CI

GitHub Actions runs the same wrapper through
[`/.github/workflows/contract_validation.yml`](../../.github/workflows/contract_validation.yml)
and uploads `target/contract-validation/` as a workflow artifact.
