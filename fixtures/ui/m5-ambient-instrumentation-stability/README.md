# M5 ambient-instrumentation stability fixtures

Protected, redaction-safe fixtures for the M5 ambient shell-instrumentation stability
certification capstone (see
[`docs/shell/m5_ambient_instrumentation_stability_contract.md`](../../../docs/shell/m5_ambient_instrumentation_stability_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the seed
by `crates/aureline-shell/tests/m5_ambient_instrumentation_stability_fixtures.rs`. Do not
hand-edit; regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_ambient_instrumentation_stability --"
$BIN packet         > fixtures/ui/m5-ambient-instrumentation-stability/packet.json
$BIN dashboard      > fixtures/ui/m5-ambient-instrumentation-stability/dashboard.json
$BIN support-export > fixtures/ui/m5-ambient-instrumentation-stability/support_export.json
$BIN compact        > fixtures/ui/m5-ambient-instrumentation-stability/compact.txt
```

- `packet.json` — canonical certification packet (4 green, 4 yellow, 0 red).
- `dashboard.json` — the light certification dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
