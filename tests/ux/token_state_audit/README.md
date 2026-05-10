# Token / state / reduced-motion audit

Unattended runner that walks the protected M1 shell, Start Center, search
palette, and trust surfaces and asserts they keep loading semantic tokens,
the shared component-state vocabulary, and the shared reduced-motion preset
contract — never ad hoc literals or per-surface timers.

## Run the protected walk

```bash
python3 tests/ux/token_state_audit/run_token_state_audit.py --repo-root .
```

The command emits a durable JSON capture at
`artifacts/milestones/m1/captures/token_motion_audit_validation_capture.json`
and exits non-zero on any regression.

## Run the failure drill

Each fixture row under `fixtures/ux/reduced_motion_cases/` declares a named
failure drill. To replay one and confirm the audit reports the exact drift
the case promises:

```bash
python3 tests/ux/token_state_audit/run_token_state_audit.py \
  --repo-root . \
  --force-drill shell_chrome_drop_focus_ring_token
```

The runner injects the forced input (e.g. drops a required token from the
"observed" set) and verifies the expected `check_id` is reported. The lane
exits 0 only when the drill reproduced exactly as declared.

## Wiring

| Input  | Path                                                           |
| ------ | -------------------------------------------------------------- |
| Cases  | `fixtures/ux/reduced_motion_cases/*.yaml`                      |
| Sink   | `artifacts/milestones/m1/captures/token_motion_audit_validation_capture.json` |
| Packet | `artifacts/milestones/m1/proof_packets/token_motion_audit.md`  |
| Index  | `artifacts/milestones/m1/artifact_index.yaml#token_motion_audit` |
