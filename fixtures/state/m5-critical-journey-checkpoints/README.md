# M5 critical-journey checkpoint fixtures

Protected, redaction-safe fixtures for the M5 critical-journey checkpoint capstone (see
[`docs/lifecycle/m5_critical_journey_checkpoints_contract.md`](../../../docs/lifecycle/m5_critical_journey_checkpoints_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the seed by
`crates/aureline-shell/tests/m5_critical_journey_checkpoints_fixtures.rs`. Do not hand-edit;
regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints --"
$BIN packet         > fixtures/state/m5-critical-journey-checkpoints/packet.json
$BIN dashboard      > fixtures/state/m5-critical-journey-checkpoints/dashboard.json
$BIN support-export > fixtures/state/m5-critical-journey-checkpoints/support_export.json
$BIN compact        > fixtures/state/m5-critical-journey-checkpoints/compact.txt
```

- `packet.json` — canonical certification packet (2 green, 3 yellow, 0 red).
- `dashboard.json` — the light certification dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
