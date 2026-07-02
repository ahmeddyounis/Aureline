# M5 keybinding resolver inspection fixtures

Protected fixtures for the M5 keybinding resolver inspection certification (task **M05-742**). These are
minted by the headless emitter `aureline_shell_m5_keybinding_resolver_inspectors` — the only
mint-from-truth path — and replayed bit-for-bit by
`crates/aureline-shell/tests/m5_keybinding_resolver_inspectors_fixtures.rs`.

- `packet.json` — the seeded inspection packet (10 rows: 6 green / 4 yellow / 0 red).
- `dashboard.json` — the light dashboard projection of the packet.
- `support_export.json` — the support-export wrapper quoting the packet, dashboard, and case ids.
- `compact.txt` — the compact headless review lines.

Regenerate after any seed change:

```sh
BIN=target/debug/aureline_shell_m5_keybinding_resolver_inspectors
cargo build -q -p aureline-shell --bin aureline_shell_m5_keybinding_resolver_inspectors
"$BIN" packet         > fixtures/commands/m5-keybinding-resolver-inspectors/packet.json
"$BIN" dashboard      > fixtures/commands/m5-keybinding-resolver-inspectors/dashboard.json
"$BIN" support-export > fixtures/commands/m5-keybinding-resolver-inspectors/support_export.json
"$BIN" compact        > fixtures/commands/m5-keybinding-resolver-inspectors/compact.txt
```

The same records are also published under
`artifacts/release/m5-keybinding-resolver-inspectors-proof/` (plus `matrix.csv`) and the markdown report
under `artifacts/commands/m5-keybinding-resolver-inspectors.md`.
