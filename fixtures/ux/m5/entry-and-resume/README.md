# M5 first-useful-work entry routes

These fixtures pin the entry-routes packet produced by the shell projection in
[`crate::m5_entry_routes`](../../../../crates/aureline-shell/src/m5_entry_routes/mod.rs).
The packet carries the safe first-run and setup-later posture forward into every
major M5 depth lane — notebook, request and database workspaces, profiler/trace
captures, framework packs, docs/local browser, preview routes, companion
handoff, managed sync, and offboarding.

Each lane has an entry route with an explicit local-core fallback, a
`set_up_later` action, a reviewable list of what Aureline has *not* yet done
(no kernel started, no request sent, no preview route exposed, no sync joined,
no offboarding action committed), and a first-useful-work measurement that is
reachable before any optional managed or provider-backed setup. No route
declares a hidden prerequisite (browser auth, provider attachment, kernel
execution, or managed sync) and none captures raw sensitive user content.

## Files

| File | Inspector subcommand | What it pins |
|---|---|---|
| `packet.json` | `packet` | Full entry-routes packet record. |
| `routes.json` | `routes` | Per-lane entry routes. |
| `coverage.json` | `coverage` | Lane coverage summary. |
| `support_export.json` | `support-export` | Support-export wrapper with case ids. |
| `compact.txt` | `compact` | Headless compact summary lines. |

## Regenerating

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_entry_routes --"
$BIN packet         > fixtures/ux/m5/entry-and-resume/packet.json
$BIN routes         > fixtures/ux/m5/entry-and-resume/routes.json
$BIN coverage       > fixtures/ux/m5/entry-and-resume/coverage.json
$BIN support-export > fixtures/ux/m5/entry-and-resume/support_export.json
$BIN compact        > fixtures/ux/m5/entry-and-resume/compact.txt
```

The replay test `crates/aureline-shell/tests/m5_entry_routes_fixtures.rs`
asserts the checked-in JSON is a literal projection of the seeded packet.
