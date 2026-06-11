# M5 Companion, Incident, Sync, Residency, and Offboarding Matrix Fixtures

These fixtures are generated deterministically from the first-consumer matrix
builder in `aureline-companion` and validate against
`schemas/companion/freeze-the-m5-companion-incident-sync-and-offboarding-matrix-with-staged-rollout-lanes.schema.json`.

## managed_sync_withheld_matrix.json

A matrix where the managed-sync lane's evidence packet failed validation, so the
lane is narrowed to `held` and its rollout `withheld` rather than shipped greener
than the proof. Every other lane remains at its canonical qualification, and the
security review and consumer projection stay fully satisfied. Demonstrates that
invalid evidence withholds a lane instead of hiding it.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_m5_companion_matrix -- sync_withheld
```

## residency_narrowed_matrix.json

A matrix where the residency lane's customer-managed-key / end-to-end-encryption
claim could not be verified, so a Beta lane narrows to Preview and its staged
rollout narrows from `staged_rollout` to `early_access`. Demonstrates that an
unprovable residency claim narrows the qualification and rollout stage.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_m5_companion_matrix -- residency_narrowed
```
