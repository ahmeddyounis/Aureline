# M5 window lifecycle safety contract

This lane is the **window-lifecycle-safety capstone** on top of the frozen
[M5 shell-zone, responsive-class, and multi-window continuity matrix](m5_shell_zone_matrix_contract.md).
Where the matrix *declares* the window classes each claimed M5 surface family may live in
(primary workspace, secondary detached, floating utility, companion overlay), this lane
*certifies* that the live shell keeps three lifecycle promises for every governed family:

- a **cross-window drag/drop advertises the resulting verb** — `Move tab`, `Copy editor`,
  `Open compare here`, `Create window` — **before the drop completes**, and keeps every verb
  keyboard-reachable through a command equivalent;
- **closing a secondary window can never silently strand** a dirty buffer, a live approval,
  shared collaboration control, or a long-running evidence review;
- a **specialized window reopened after crash or restore falls back to the safest equivalent
  shell arrangement** when an extension, remote target, or feature pack is unavailable,
  rather than orphaning the object or landing on the wrong surface.

The lane exists so that M5 can honestly claim desktop maturity: a user always sees what a
cross-window drop will do before releasing it; closing a detached window never quietly loses
unsaved work, an in-flight approval, host/driver control, or an evidence review in progress;
and a reopened notebook, data, review, preview, docs, operator, or incident window degrades
to the safest layout it can instead of dropping the object.

## Governed families

The lifecycle proof covers exactly the ten families the matrix freezes, and refuses to ship
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

## Cross-window drag verbs

Every row declares a per-verb **cross-window drag plan** for each canonical drag verb —
`move_tab`, `copy_editor`, `open_compare_here`, and `create_window` — naming whether the verb
is advertised before the drop completes and whether it keeps a keyboard command equivalent.
The plans must:

- cover exactly the canonical drag verbs — no verb the drop affordance must advertise is left
  uncertified and none is invented;
- **advertise** the resulting verb before the cross-window drop completes;
- keep the verb **keyboard-reachable** through a command equivalent.

## Protected close resources

Every row must declare all four protected close resources — `dirty_buffer`, `live_approval`,
`collaboration_control`, and `evidence_review`. A row that fails to declare the full set
blocks: closing a secondary window could otherwise silently orphan one of them.

## Per-family posture axes

Each row is certified across three posture axes:

- **drag-verb disclosure** — `verb_disclosed_with_keyboard_parity` (green), a disclosed
  `disclosed_verb_reach_narrowing` where a drag verb is still advertised before the drop but
  reachable only through a disclosed command-palette equivalent rather than an inline
  pre-drop hint (yellow), or `verb_hidden_or_keyboard_lost` (red: a drop completed without
  advertising the verb, or a verb lost keyboard parity).
- **close-orphan guard** — `close_guarded_no_orphan` (green), a disclosed
  `disclosed_deferred_guard_relocation` where closing a secondary window defers a protected
  resource to a disclosed, waivered relocation into the primary window with a still-visible
  prompt (yellow), or `silent_orphan_on_close` (red: a close silently stranded a dirty
  buffer, approval, collaboration control, or evidence review).
- **safe-reopen fallback** — `reopens_safest_equivalent_layout` (green), a disclosed
  `disclosed_reduced_equivalent_fallback` where a specialized window reopens onto a disclosed
  reduced but still-safe equivalent layout when an exact dependency is missing while
  preserving identity and the reopen path (yellow), or `reopen_orphaned_or_wrong_surface`
  (red: a reopen orphaned the object or landed on the wrong surface).

## Derived status and the lifecycle lint

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when its
frozen qualification is below Stable, a drag verb is reachable only through a disclosed
command-palette equivalent, a secondary-window close defers a protected resource to a
disclosed waivered relocation, or a specialized-window reopen lands on a disclosed
reduced-but-safe equivalent. It drops to `red` when a cross-window drop hides the resulting
verb or loses keyboard parity, a close silently orphans a protected resource, a reopen
orphans the object or lands on the wrong surface, a protected resource is undeclared, or a
per-verb drag plan drops pre-drop disclosure or its keyboard command equivalent. The
protected-resource and drag-plan completeness checks are the lint that prevents a later
lifecycle regression from shipping as stable — the Rust validator in
`crates/aureline-shell/src/m5_window_lifecycle_safety` is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_deferred_guard_relocation`
narrowing must additionally carry an active, matching, unexpired waiver.

## Records

- **Lifecycle packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact lifecycle causes, and the blocking
  findings the lane refuses to ship with.
- **Lifecycle dashboard** — a light projection the shell / windowing / layout / status
  automation reads to auto-narrow a claimed surface when its lifecycle proof falls out of
  policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix ref,
  build id, each family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels — never
raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety`) is the only
mint-from-truth path for:

- `artifacts/release/m5-window-lifecycle-safety-proof/packet.json`
- `artifacts/release/m5-window-lifecycle-safety-proof/dashboard.json`
- `artifacts/release/m5-window-lifecycle-safety-proof/support_export.json`
- `artifacts/release/m5-window-lifecycle-safety-proof/matrix.csv`
- `artifacts/shell/m5-window-lifecycle-safety.md` (this report's rendered companion)
- `fixtures/ui/m5-window-lifecycle-safety/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-window-lifecycle-safety.schema.json`](../../schemas/shell/m5-window-lifecycle-safety.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety -- validate
cargo test -p aureline-shell --test m5_window_lifecycle_safety_fixtures
cargo test -p aureline-shell m5_window_lifecycle_safety
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_window_lifecycle_safety --"
$BIN packet         > artifacts/release/m5-window-lifecycle-safety-proof/packet.json
$BIN dashboard      > artifacts/release/m5-window-lifecycle-safety-proof/dashboard.json
$BIN support-export > artifacts/release/m5-window-lifecycle-safety-proof/support_export.json
$BIN csv            > artifacts/release/m5-window-lifecycle-safety-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-window-lifecycle-safety.md
$BIN packet         > fixtures/ui/m5-window-lifecycle-safety/packet.json
$BIN dashboard      > fixtures/ui/m5-window-lifecycle-safety/dashboard.json
$BIN support-export > fixtures/ui/m5-window-lifecycle-safety/support_export.json
$BIN compact        > fixtures/ui/m5-window-lifecycle-safety/compact.txt
```
