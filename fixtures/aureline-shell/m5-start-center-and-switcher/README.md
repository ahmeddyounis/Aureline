# M5 Start Center and workspace-switcher parity

These fixtures pin the parity packet produced by the shell projection in
[`crate::m5_start_center_and_switcher`](../../../crates/aureline-shell/src/m5_start_center_and_switcher/mod.rs).
The packet seeds one canonical recent-work entry per M5 entry-target class —
local folder, workspace file, multi-root workspace, SSH target,
container/dev container, managed workspace, import packet, and bundle-backed
entry — plus the failure exemplars the lane must disclose (missing root,
relocated workspace, stale target, remote host unreachable, and partial
restore). Each entry is projected through both the live Start Center
recent-work projection and the live in-workspace switcher projection, and the
packet records, per row, that the two surfaces agree on the canonical target
kind, trust state, restore class, and unavailable-target failure state.

No row collapses two distinct target kinds into a generic recent-project row,
no surface widens trust beyond the canonical entry, and every unavailable or
partially restorable row carries an export-safe diagnostic redacted to the
target-kind label (never a raw path, host, or provider body).

## Files

| File | Inspector subcommand | What it pins |
|---|---|---|
| `packet.json` | `packet` | Full parity packet record. |
| `rows.json` | `rows` | Per-entry parity rows. |
| `diagnostics.json` | `diagnostics` | Export-safe diagnostics for unavailable / partial rows. |
| `coverage.json` | `coverage` | Surface-class and diagnostic-class coverage summary. |
| `support_export.json` | `support-export` | Support-export wrapper with case ids. |
| `compact.txt` | `compact` | Headless compact summary lines. |

## Regenerating

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_start_center_and_switcher --"
$BIN packet         > fixtures/aureline-shell/m5-start-center-and-switcher/packet.json
$BIN rows           > fixtures/aureline-shell/m5-start-center-and-switcher/rows.json
$BIN diagnostics    > fixtures/aureline-shell/m5-start-center-and-switcher/diagnostics.json
$BIN coverage       > fixtures/aureline-shell/m5-start-center-and-switcher/coverage.json
$BIN support-export > fixtures/aureline-shell/m5-start-center-and-switcher/support_export.json
$BIN compact        > fixtures/aureline-shell/m5-start-center-and-switcher/compact.txt
$BIN markdown       > artifacts/ux/m5/start-center-and-switcher-audit.md
```

The replay test
`crates/aureline-shell/tests/m5_start_center_and_switcher_fixtures.rs` asserts the
checked-in JSON is a literal projection of the seeded packet.
