# Contract-artifact validation lane

This lane turns the repository's contract-bearing governance artifacts into a
repeatable local and CI gate. It validates package inventory coverage and
forbidden edges, control-artifact index drift, stable-surface ownership and
traceability, decision/source-anchor integrity, compatibility and
deployment-profile joins, boundary-row links, claim-manifest refs,
frozen-surface manifest completeness and same-train obligations, and
command-parity tool health.

Companion artifacts:

- [`/tools/ci/validate_contract_artifacts.py`](../../tools/ci/validate_contract_artifacts.py)
  — shared validator entry point.
- [`/ci/contract_validation.sh`](../../ci/contract_validation.sh)
  — local and CI wrapper that writes a JSON report and captures the
  human-readable summary.
- [`/tools/check_frozen_surfaces.py`](../../tools/check_frozen_surfaces.py)
  — direct validator entry point for the frozen-surface manifest and
  changed-surface obligation checks.
- [`/fixtures/ci/contract_validation/missing_deployment_profile.json`](../../fixtures/ci/contract_validation/missing_deployment_profile.json)
  — checked-in failing scenario used to prove the deployment-profile gate
  still trips.
- [`/fixtures/ci/contract_validation/missing_frozen_surface_metadata.json`](../../fixtures/ci/contract_validation/missing_frozen_surface_metadata.json)
  — checked-in failing scenario used to prove a changed frozen surface
  still fails without diff metadata or companion updates.
- [`/artifacts/contracts/frozen_surface_manifest.yaml`](../../artifacts/contracts/frozen_surface_manifest.yaml)
  and [`/docs/governance/frozen_surface_ci_policy.md`](../governance/frozen_surface_ci_policy.md)
  — frozen-surface manifest and CI policy the lane now enforces.
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

To run only the frozen-surface gate directly:

```bash
python3 tools/check_frozen_surfaces.py \
  --repo-root . \
  --report target/contract-validation/frozen_surface_report.json
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

This checked-in scenario treats the command descriptor schema as changed
without touching the frozen-surface manifest, a diff artifact, or a
same-train companion artifact. The validator must exit non-zero and
report `frozen_surface_manifest.diff_metadata_required`.

```bash
python3 tools/check_frozen_surfaces.py \
  --repo-root . \
  --scenario fixtures/ci/contract_validation/missing_frozen_surface_metadata.json \
  --report target/contract-validation/missing_frozen_surface_metadata_report.json
```

## CI

GitHub Actions runs the same wrapper through
[`/.github/workflows/contract_validation.yml`](../../.github/workflows/contract_validation.yml)
and uploads `target/contract-validation/` as a workflow artifact.
