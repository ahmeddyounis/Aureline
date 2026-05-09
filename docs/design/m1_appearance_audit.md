# M1 appearance audit (goldens + token adoption)

This document is the reviewer-facing entrypoint for the **appearance** harness
seed used to keep the protected M1 shell surfaces visually consistent across
theme/contrast and density/reduced-motion postures.

The harness is intentionally narrow:

- screenshot goldens for the protected shell chrome surface seed
- token-adoption baselines that ensure protected surfaces keep loading styling
  primitives from the shared token registries (instead of surface-local
  literals)

## What is checked in CI

- Token-adoption baseline: `tools/ci/check_token_adoption.py`
  - Baseline: `tests/golden/appearance/shell_chrome/token_adoption_baseline.json`
  - Surfaces: shell chrome + shared component-state registry used by the shell
  - CI entrypoint: `ci/contract_validation.sh`

## What is captured as appearance goldens

- Shell chrome screenshot baselines (single-frame captures):
  - Directory: `tests/golden/appearance/shell_chrome/baselines`
  - Cases: dark/light/high-contrast (plus a density variant)

## Developer workflows

Capture baselines (requires a working desktop environment):

```bash
python3 tools/ci/capture_appearance_goldens.py --out-dir tests/golden/appearance/shell_chrome/baselines
```

Compare against the checked-in baselines:

```bash
python3 tools/ci/compare_appearance_goldens.py --baseline-dir tests/golden/appearance/shell_chrome/baselines
```

Update the token-adoption baseline (when the token surface requirements change
intentionally):

```bash
python3 tools/ci/check_token_adoption.py --update --baseline tests/golden/appearance/shell_chrome/token_adoption_baseline.json
```

## Evidence index

The M1 proof index includes an entry for this harness so reviewers can find the
latest capture and the canonical docs entrypoint without hunting.

- Proof lane: `artifacts/milestones/m1/artifact_index.yaml#appearance_harness`
- Latest capture: `artifacts/milestones/m1/captures/appearance_harness_validation_capture.json`
