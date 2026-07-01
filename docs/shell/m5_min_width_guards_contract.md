# M5 min-width-guard (editor minimum / compare fallback / no unusable narrow pane) contract

This lane is the **minimum-useful-size capstone** on top of the frozen
[M5 shell-zone, responsive-class, and multi-window continuity matrix](m5_shell_zone_matrix_contract.md).
Where the matrix *declares* the responsive classes each claimed M5 surface family must
survive and the occupant transitions (side-by-side, tabbed, sheeted, overflowed,
solo-docked) it may take as width narrows, this lane *certifies* that the live surface
**enforces a minimum useful editor width and height**, that when a second group,
compare, diff, or dense inspector would violate that minimum the surface **falls back to
a declared safe compare mode** instead of silently producing an unusable narrow split,
and that breadcrumbs, active object identity, and recovery-critical status **stay
visible** while the fallback is active.

The lane exists so that M5 can honestly claim desktop maturity: an editor group never
shrinks below a usable minimum, a compare or diff that no longer fits falls back to
tabbed compare, a staged peek, sequential disclosure, or an explicit user choice rather
than two unreadable slivers, and the active object's breadcrumbs and recovery-critical
status never disappear while the presentation mode shifts.

## Governed families

The guard proof covers exactly the ten families the matrix freezes, and refuses to ship
if any is missing:

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

## Minimum useful size

The lane reserves an **absolute floor** (`320×200` px) that no usable pane may drop
below and a **standard minimum** (`480×320` px) that any pane claiming full enforcement
must reserve. Each row carries the minimum useful width and height it enforces; a row
claiming `min_useful_size_enforced` must meet the standard minimum, and a row disclosing
a `disclosed_reduced_minimum` may narrow toward the absolute floor but never below it.

## Safe-fallback set and per-class compare plan

Each row carries a **declared safe-fallback set** derived from the matrix occupant
transitions — a widest-to-narrowest ladder of
`side_by_side_split → tabbed_compare → staged_peek → sequential_disclosure → explicit_user_choice`
that must be ordered by required width and terminate in a universally-available safe mode
(`sequential_disclosure` or `explicit_user_choice`), so a surface always has a safe
compare mode no matter how narrow it becomes. It also carries one **per-class compare
plan** for each declared responsive class naming the strategy the surface lands in at
that class, whether the plan meets the minimum useful size, and whether identity and
status are preserved. The plans must:

- cover exactly the declared responsive classes (compact, standard, expanded);
- land only in strategies the family's safe-fallback set declares;
- stay **monotonic** — a narrower class uses a strategy that needs at most as much width
  as a wider one, so the compare mode always degrades gracefully as width shrinks.

## Per-family posture axes

Each row is certified across three posture axes:

- **min-size enforcement** — `min_useful_size_enforced` (green), a disclosed
  `disclosed_reduced_minimum` that stays above the absolute floor (yellow), or
  `pane_forced_below_usable_minimum` (red: the pane can be forced below a usable size).
- **compare fallback** — `safe_fallback_before_unusable_split` (green),
  `disclosed_fallback_narrowing` (yellow: the fallback trims a secondary pane's optional
  detail in a disclosed way), or `silent_unusable_split` (red: a compare/diff produced a
  silent unusable narrow split with no fallback).
- **status continuity** — `identity_breadcrumbs_status_preserved` (green),
  `disclosed_status_relocation` (yellow: recovery-critical status relocates to a
  disclosed, waivered still-visible affordance), or
  `status_or_identity_lost_under_fallback` (red).

## Derived status and the guard lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when
its frozen qualification is below Stable, its editor discloses a reduced-but-usable
minimum, its compare area discloses a narrowed fallback, or its recovery-critical status
is relocated to a disclosed waivered affordance. It drops to `red` when the editor or
compare pane can be forced below a usable minimum, a compare/diff produces a silent
unusable split, breadcrumbs / identity / recovery-critical status are lost under the
fallback, the safe-fallback set has no universal terminal or is not ordered, the primary
fallback strategy is undeclared, the declared minimum drops below the absolute floor, or
a per-class plan lands outside the declared set or is non-monotonic. The strategy-set and
plan checks are the lint that prevents a later narrow-pane regression from shipping as
stable — the Rust validator in `crates/aureline-shell/src/m5_min_width_guards` is the
authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_status_relocation`
narrowing must additionally carry an active, matching, unexpired waiver.

## Records

- **Guard packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact guard causes, and the blocking
  findings the lane refuses to ship with.
- **Guard dashboard** — a light projection the shell / windowing / layout / release
  automation reads to auto-narrow a claimed surface when its minimum-size proof falls out
  of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix
  ref, build id, each family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, pixel floors, and
short labels — never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or
credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards`) is the only
mint-from-truth path for:

- `artifacts/release/m5-min-width-guards-proof/packet.json`
- `artifacts/release/m5-min-width-guards-proof/dashboard.json`
- `artifacts/release/m5-min-width-guards-proof/support_export.json`
- `artifacts/release/m5-min-width-guards-proof/matrix.csv`
- `artifacts/shell/m5-min-width-guards.md` (this report's rendered companion)
- `fixtures/ui/m5-min-width-guards/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-min-width-guards.schema.json`](../../schemas/shell/m5-min-width-guards.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- validate
cargo test -p aureline-shell --test m5_min_width_guards_fixtures
cargo test -p aureline-shell m5_min_width_guards
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards --"
$BIN packet         > artifacts/release/m5-min-width-guards-proof/packet.json
$BIN dashboard      > artifacts/release/m5-min-width-guards-proof/dashboard.json
$BIN support-export > artifacts/release/m5-min-width-guards-proof/support_export.json
$BIN csv            > artifacts/release/m5-min-width-guards-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-min-width-guards.md
$BIN packet         > fixtures/ui/m5-min-width-guards/packet.json
$BIN dashboard      > fixtures/ui/m5-min-width-guards/dashboard.json
$BIN support-export > fixtures/ui/m5-min-width-guards/support_export.json
$BIN compact        > fixtures/ui/m5-min-width-guards/compact.txt
```
