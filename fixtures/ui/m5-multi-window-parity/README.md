# M5 multi-window truth-parity fixtures

Protected, redaction-safe fixtures for the M5 multi-window truth-parity capstone (see
[`docs/shell/m5_multi_window_parity_contract.md`](../../../docs/shell/m5_multi_window_parity_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the seed
by `crates/aureline-shell/tests/m5_multi_window_parity_fixtures.rs`. Do not hand-edit;
regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_multi_window_parity --"
$BIN packet         > fixtures/ui/m5-multi-window-parity/packet.json
$BIN dashboard      > fixtures/ui/m5-multi-window-parity/dashboard.json
$BIN support-export > fixtures/ui/m5-multi-window-parity/support_export.json
$BIN compact        > fixtures/ui/m5-multi-window-parity/compact.txt
```

- `packet.json` — canonical parity packet (6 green, 4 yellow, 0 red).
- `dashboard.json` — the light parity dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
