# M5 owning-window routing fixtures

Protected, redaction-safe fixtures for the M5 owning-window routing capstone (see
[`docs/shell/m5_owning_window_routing_contract.md`](../../../docs/shell/m5_owning_window_routing_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the seed
by `crates/aureline-shell/tests/m5_owning_window_routing_fixtures.rs`. Do not hand-edit;
regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_owning_window_routing --"
$BIN packet         > fixtures/ui/m5-owning-window-routing/packet.json
$BIN dashboard      > fixtures/ui/m5-owning-window-routing/dashboard.json
$BIN support-export > fixtures/ui/m5-owning-window-routing/support_export.json
$BIN compact        > fixtures/ui/m5-owning-window-routing/compact.txt
```

- `packet.json` — canonical routing packet (6 green, 4 yellow, 0 red).
- `dashboard.json` — the light routing dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
