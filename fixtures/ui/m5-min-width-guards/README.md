# M5 min-width-guard fixtures

Protected, redaction-safe fixtures for the M5 min-width-guard (editor minimum / compare
fallback / no unusable narrow pane) capstone (see
[`docs/shell/m5_min_width_guards_contract.md`](../../../docs/shell/m5_min_width_guards_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the
seed by `crates/aureline-shell/tests/m5_min_width_guards_fixtures.rs`. Do not hand-edit;
regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards --"
$BIN packet         > fixtures/ui/m5-min-width-guards/packet.json
$BIN dashboard      > fixtures/ui/m5-min-width-guards/dashboard.json
$BIN support-export > fixtures/ui/m5-min-width-guards/support_export.json
$BIN compact        > fixtures/ui/m5-min-width-guards/compact.txt
```

- `packet.json` — canonical guard packet (6 green, 4 yellow, 0 red).
- `dashboard.json` — the light guard dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
