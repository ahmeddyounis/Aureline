# M5 responsive-collapse fixtures

Protected, redaction-safe fixtures for the M5 responsive-collapse
(compact/standard/expanded) capstone (see
[`docs/shell/m5_responsive_collapse_contract.md`](../../../docs/shell/m5_responsive_collapse_contract.md)).

These files are minted by the headless emitter and asserted bit-for-bit equal to the
seed by `crates/aureline-shell/tests/m5_responsive_collapse_fixtures.rs`. Do not
hand-edit; regenerate with:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse --"
$BIN packet         > fixtures/ui/m5-responsive-collapse/packet.json
$BIN dashboard      > fixtures/ui/m5-responsive-collapse/dashboard.json
$BIN support-export > fixtures/ui/m5-responsive-collapse/support_export.json
$BIN compact        > fixtures/ui/m5-responsive-collapse/compact.txt
```

- `packet.json` — canonical collapse packet (6 green, 4 yellow, 0 red).
- `dashboard.json` — the light collapse dashboard projection.
- `support_export.json` — packet + dashboard + stable case ids.
- `compact.txt` — headless compact-line rendering.
