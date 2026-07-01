# M5 shell-zone occupancy & declared-slot routing contract

This lane is the **occupancy capstone** on top of the frozen
[M5 shell-zone, responsive-class, and multi-window continuity matrix](m5_shell_zone_matrix_contract.md).
Where the matrix *declares* which shell slot each claimed M5 surface family binds
to, this lane *certifies* that the live surface **actually occupies a declared
shell slot**, that its command, keyboard, docs, and onboarding routes all resolve
to the same slot and occupant, and that a dependency-missing or policy-blocked
occupant degrades into an explicit in-slot placeholder card that preserves spatial
continuity instead of collapsing the surrounding layout or inventing a private
chrome island.

The lane exists so that M5 can honestly claim desktop maturity: every new notebook,
data, review, preview, docs, operator, and incident surface routes through the one
declared shell-slot registry rather than inventing its own slot, collapse, or
multi-window behavior.

## Governed families

The occupancy proof covers exactly the ten families the matrix freezes, and refuses
to ship if any is missing:

- `notebook` — Notebook editor / cell surface
- `data_grid` — Tabular data grid surface
- `profiler` — Profiler / performance surface
- `pipeline` — Pipeline / workflow graph surface
- `docs` — Documentation reader surface
- `preview` — Preview surface (render, diff, media)
- `review` — Review / change-request surface
- `incident` — Incident / operations-response surface
- `companion` — Companion assistant surface
- `operator` — Operator / control-plane surface

## Per-family occupancy row

Each row names the family's declared canonical/fallback slot (pulled from the
matrix), the **registered slot set** it may attach to, the slot it currently
occupies, and three posture axes:

- **slot attachment** — `attached_to_declared_slot` (green) or
  `undeclared_slot_attachment` (red: a private chrome island).
- **occupant availability** — `occupant_available` (green), a disclosed
  `dependency_missing_placeholder` / `policy_blocked_placeholder` (yellow: the slot
  stays occupied by an explicit placeholder card that preserves spatial
  continuity), or `placeholder_collapsed_layout` (red: the placeholder collapsed the
  surrounding layout or lost the surface identity / reopen path).
- **route resolution** — `all_routes_resolve_to_slot_occupant` (green),
  `disclosed_route_fallback` (yellow: a route resolves to a disclosed, waivered
  alternative), or `conflicting_route_resolution` (red: a route resolves to a
  different slot or occupant than the declared owner).

## Derived status and the registered-slot lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow`
when its frozen qualification is below Stable, it shows a disclosed placeholder, or a
route falls back to a disclosed, waivered alternative. It drops to `red` when it
attaches outside any declared slot, its **occupied slot is not in the family's
registered slot set**, its canonical slot is not registered, its placeholder
collapses layout, or a route conflicts. The registered-slot check is the lint that
prevents a later unregistered sidebar, inspector, bottom-panel, or overlay
attachment from shipping as stable — the Rust validator in
`crates/aureline-shell/src/m5_shell_zone_occupancy` is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_route_fallback`
narrowing must additionally carry an active, matching, unexpired waiver.

## Records

- **Occupancy packet** — the full set of rows with derived per-row status,
  aggregate green/yellow/red counts, active waivers, the exact occupancy causes, and
  the blocking findings the lane refuses to ship with.
- **Occupancy dashboard** — a light projection the shell / windowing / layout /
  release automation reads to auto-narrow a claimed surface when its occupancy proof
  falls out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id,
  matrix ref, build id, each family, each occupied slot, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short
labels — never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or
credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy`) is
the only mint-from-truth path for:

- `artifacts/release/m5-shell-occupancy-proof/packet.json`
- `artifacts/release/m5-shell-occupancy-proof/dashboard.json`
- `artifacts/release/m5-shell-occupancy-proof/support_export.json`
- `artifacts/release/m5-shell-occupancy-proof/matrix.csv`
- `artifacts/shell/m5-shell-zone-occupancy.md` (this report's rendered companion)
- `fixtures/ui/m5-shell-occupancy/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-shell-zone-occupancy.schema.json`](../../schemas/shell/m5-shell-zone-occupancy.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy -- validate
cargo test -p aureline-shell --test m5_shell_zone_occupancy_fixtures
cargo test -p aureline-shell m5_shell_zone_occupancy
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_zone_occupancy --"
$BIN packet         > artifacts/release/m5-shell-occupancy-proof/packet.json
$BIN dashboard      > artifacts/release/m5-shell-occupancy-proof/dashboard.json
$BIN support-export > artifacts/release/m5-shell-occupancy-proof/support_export.json
$BIN csv            > artifacts/release/m5-shell-occupancy-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-shell-zone-occupancy.md
$BIN packet         > fixtures/ui/m5-shell-occupancy/packet.json
$BIN dashboard      > fixtures/ui/m5-shell-occupancy/dashboard.json
$BIN support-export > fixtures/ui/m5-shell-occupancy/support_export.json
$BIN compact        > fixtures/ui/m5-shell-occupancy/compact.txt
```
