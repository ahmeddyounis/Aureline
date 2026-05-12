# Integrated Flow Smoke

Unattended binary smoke for the protected small-project dogfood fixtures in
[`artifacts/milestones/m1/dogfood_matrix.yaml`](../../../artifacts/milestones/m1/dogfood_matrix.yaml).

The runner launches `aureline_shell` with `--open`, `--exit-after-first-frame`,
and `--emit-startup-trace` for the supported action rows. It asserts the trace
contains `shell.first_frame_submit` and protected journey budget fields. For
`edit_save`, it also uses the binary's private headless edit/save hook against
temporary fixture copies and verifies the staged bytes round-trip on disk.

## Run

```bash
python3 tests/desktop/m1_integrated_flow_smoke/run_integrated_flow_smoke.py --repo-root .
```

The runner writes:

```text
artifacts/milestones/m1/captures/integrated_flow_smoke_validation_capture.json
```

Useful options:

- `--binary <path>`: use an already-built `aureline_shell`.
- `--no-build`: fail if the default binary is missing instead of building it.
- `--force-drill <row_id>`: intentionally targets a missing workspace path and
  exits non-zero with a typed force-drill check id.

The currently supported action kinds are `open`, `quick_open`, and
`edit_save`. `terminal`, `restore_session`, and `missing_target_recovery` are
reported as `pending_upstream` until their binary-level contracts are
claim-bearing.
