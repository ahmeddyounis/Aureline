# M5 responsive-collapse (compact / standard / expanded) contract

This lane is the **responsive-collapse capstone** on top of the frozen
[M5 shell-zone, responsive-class, and multi-window continuity matrix](m5_shell_zone_matrix_contract.md).
Where the matrix *declares* the responsive classes each claimed M5 surface family
must survive and the ordered collapse ladder it falls through as width narrows, this
lane *certifies* that the live surface stays **identity-stable** across
compact / standard / expanded, that the docked-to-sheet transition preserves the same
object identity and task state, that no essential action becomes hover-only or
route-breaking as width narrows, and that 400% zoom and high-contrast layouts keep the
same route semantics and task state.

The lane exists so that M5 can honestly claim desktop maturity: optional detail moves
from the right inspector to a sheet or inline disclosure first, secondary bottom-panel
tabs collapse before the main editor is starved, low-frequency tools move to overflow
or drawers before primary navigation disappears, and a docked-to-sheet transition
never changes what task object the surface represents.

## Governed families

The collapse proof covers exactly the ten families the matrix freezes, and refuses to
ship if any is missing:

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

## Collapse ladder and per-class presentation

Each row carries the ordered **collapse ladder** pulled from the matrix — a subset of
`docked → sheet → overflow → placeholder` that must be ordered most-docked to
most-collapsed and terminate in an identity-preserving `placeholder`, so a surface can
never dead-end when it can no longer dock. It also carries one
**per-class presentation** for each declared responsive class naming the placement the
surface lands in at that class, whether identity is preserved there, and whether every
essential action stays reachable. The presentations must:

- cover exactly the declared responsive classes (compact, standard, expanded);
- land only in placements the family's collapse ladder declares;
- stay **monotonic** — compact lands at or below (more collapsed than) standard, which
  lands at or below expanded, so optional detail always sheds before the primary
  surface is starved.

## Per-family posture axes

Each row is certified across four posture axes:

- **collapse ladder** — `identity_stable_ladder` (green), a disclosed
  `disclosed_ladder_narrowing` that trims optional detail early (yellow), or
  `ladder_changes_identity` (red: a collapse step reframed the task).
- **identity continuity** — `identity_and_state_preserved` (green),
  `disclosed_state_rehydration` (yellow: the docked-to-sheet transition rehydrates task
  state through a disclosed, waivered path while preserving the object identity), or
  `identity_or_state_lost_on_transition` (red).
- **critical / action reach** — `all_critical_and_actions_reachable` (green),
  `disclosed_overflow_reach` (yellow: a low-frequency action moved to a disclosed
  keyboard-reachable overflow or drawer), `critical_state_hidden` (red), or
  `essential_action_hover_only_or_route_broken` (red).
- **zoom / contrast parity** — `routes_stable_at_zoom_and_contrast` (green),
  `disclosed_zoom_narrowing` (yellow: the high-zoom layout discloses a narrowed
  presentation while exposing the same routes), or `route_semantics_diverge_at_zoom`
  (red).

## Derived status and the collapse lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow`
when its frozen qualification is below Stable, its ladder takes a disclosed narrowing
step, its transition rehydrates state through a disclosed waivered path, an action
moves to a disclosed overflow, or its zoom/contrast layout is disclosed as narrowed. It
drops to `red` when responsive collapse changes the task identity, the ladder loses its
placeholder terminal or is not ordered, the transition loses identity or task state,
critical state is hidden, an essential action becomes hover-only or route-broken,
zoom/contrast diverges the route semantics, or a per-class presentation lands outside
the declared ladder or is non-monotonic. The ladder and presentation checks are the
lint that prevents a later collapse regression from shipping as stable — the Rust
validator in `crates/aureline-shell/src/m5_responsive_collapse` is the authoritative
gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_state_rehydration`
narrowing must additionally carry an active, matching, unexpired waiver.

## Records

- **Collapse packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact collapse causes, and the blocking
  findings the lane refuses to ship with.
- **Collapse dashboard** — a light projection the shell / windowing / layout / release
  automation reads to auto-narrow a claimed surface when its responsive proof falls out
  of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id,
  matrix ref, build id, each family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels —
never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse`) is the
only mint-from-truth path for:

- `artifacts/release/m5-responsive-collapse-proof/packet.json`
- `artifacts/release/m5-responsive-collapse-proof/dashboard.json`
- `artifacts/release/m5-responsive-collapse-proof/support_export.json`
- `artifacts/release/m5-responsive-collapse-proof/matrix.csv`
- `artifacts/shell/m5-responsive-collapse.md` (this report's rendered companion)
- `fixtures/ui/m5-responsive-collapse/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-responsive-collapse.schema.json`](../../schemas/shell/m5-responsive-collapse.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- validate
cargo test -p aureline-shell --test m5_responsive_collapse_fixtures
cargo test -p aureline-shell m5_responsive_collapse
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse --"
$BIN packet         > artifacts/release/m5-responsive-collapse-proof/packet.json
$BIN dashboard      > artifacts/release/m5-responsive-collapse-proof/dashboard.json
$BIN support-export > artifacts/release/m5-responsive-collapse-proof/support_export.json
$BIN csv            > artifacts/release/m5-responsive-collapse-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-responsive-collapse.md
$BIN packet         > fixtures/ui/m5-responsive-collapse/packet.json
$BIN dashboard      > fixtures/ui/m5-responsive-collapse/dashboard.json
$BIN support-export > fixtures/ui/m5-responsive-collapse/support_export.json
$BIN compact        > fixtures/ui/m5-responsive-collapse/compact.txt
```
