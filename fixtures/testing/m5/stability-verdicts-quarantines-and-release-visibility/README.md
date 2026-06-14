# Stability-verdict / quarantine fixtures

Proof fixtures for the stability-verdict / quarantine packet
(`test_stability_verdict_quarantine_packet`). Each fixture is an export-safe
packet that `StabilityVerdictQuarantinePacket::validate` accepts and that
exercises a specific truth the contract guarantees.

- `expired_quarantine_reopens_and_fails_readiness.json` — a packet whose verdicts
  span a locally verified `stable` row, a `confirmed_flaky` parameterized template
  kept distinct from concrete invocations, a `quarantined` row, and an
  `imported_only_unverified` row held read-only. Its single quarantine is the
  result of evaluating an active record past its `expires_at`
  (`QuarantineRecord::evaluated_at`): it has flipped to `expired_reopened`, carries
  a `reopened_attempt_ref`, `release_blocking` visibility, and `fails_readiness`
  impact — proving an expired quarantine reopens its scope and fails readiness
  instead of silently persisting as local UI state.

The boundary schema is
`schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json`;
the contract doc is
`docs/testing/m5/stability-verdicts-quarantines-and-release-visibility.md`.
Regenerate the canonical export and this fixture with:

```bash
cargo run -p aureline-runtime --example dump_stability_verdict_quarantine
cargo run -p aureline-runtime --example dump_stability_verdict_quarantine fixture
```
