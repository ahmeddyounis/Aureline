# M5 shell-primitive release-proof contract

This lane is the **release-proof publication capstone** on top of the frozen
[M5 status-bar, transient-inspect, pane-control, and durable-progress-component
matrix](m5_shell_primitives_contract.md). Where the matrix *freezes* the ten governed shell
primitives — the status-bar item and overflow menu, the tooltip/hovercard/peek/pinned-preview
transient-inspect surfaces, the splitter handle and pane-resize preset, and the ambient progress
indicator and durable job row, with their status-item classes, overflow behaviors, representation
classes, promotion states, pane-resize states, progress states, source/provider/freshness labels,
non-visual accessibility routes, and mandatory labels — this lane *certifies and publishes* one
release-evidence proof per claimed shell primitive so that every claimed M5 shell-facing surface
has a current proof for its status/peek/splitter/progress truth or is automatically narrowed, ties
each row into the release evidence index, and lets a shell-primitives regression be detected
mechanically before a beta/stable claim widens.

The lane exists so that M5 can honestly claim mature shell quality: users never have to guess what a
status item means, never lose progress after looking away, never resize panes through brittle
pointer-only hit targets, and never mistake a hover-only reveal or a stale/cached preview for a
reachable, live truth — and reviewers can reopen the proof from the release evidence index rather
than restate it by hand.

## Governed primitives and truth pillars

The certification proof covers exactly the ten governed shell primitives, and refuses to ship if
any is missing. Each primitive is grouped under one of four truth pillars, and the rows must cover
all four:

- `ambient_instrumentation` — `status_bar_item`, `status_overflow_menu`
- `transient_inspect` — `tooltip`, `hovercard`, `peek_panel`, `pinned_preview_promotion`
- `pane_control` — `splitter_handle`, `pane_resize_preset`
- `durable_progress` — `progress_indicator`, `durable_job_row`

## Rendering profiles

Every primitive is certified across seven claimed rendering profiles, and refuses to ship if any is
missing. The compact, high-zoom, high-contrast, and reduced-motion profiles are exactly the
profile-coverage cases the implementation requirements name:

- `standard`, `compact`, `expanded`, `multi_window`, `high_zoom`, `high_contrast`, `reduced_motion`

## Per-primitive certification row

Each row certifies one frozen shell primitive and — pulled straight from the matrix's seeded
primitive row for that family — the status-item classes, overflow behaviors, representation classes,
promotion states, pane-resize states, progress states, source/provider/freshness labels,
accessibility routes, required labels, shell zone, responsive/window/surface classes, consumer
surfaces, and downgrade triggers. It is certified across four release-truth axes:

- **primitive truth** — `primitive_truth_certified_and_current` (green),
  `disclosed_reduced_truth_scope` (yellow: a low-priority slice is presented at a coarser scope
  while the primary state stays current and named), or `primitive_truth_collapsed_or_lost` (red: the
  typed state truth collapses into a generic spinner or anonymous chrome, or was lost when the
  surface compacted).
- **representation / freshness** — `source_freshness_representation_preserved` (green),
  `disclosed_partial_representation` (yellow: a low-priority representation detail is trimmed while
  the source, freshness, and representation truth stay preserved), or
  `source_or_freshness_hidden_or_stale` (red: the source/provider/freshness truth is hidden after
  compact/pin/promote, or a stale/cached preview reads as live canonical content).
- **interaction reach** — `keyboard_touch_reach_and_resize_certified` (green),
  `disclosed_reduced_reach_or_resize` (yellow: a coarser touch target or reduced keyboard resize
  step while a keyboard/touch path stays present and precise — backed by a waiver), or
  `pointer_or_hover_only_or_brittle_resize` (red: a critical truth or resize affordance is reachable
  only by pointer or hover, or a resize is brittle / not serializable).
- **exported proof parity** — `exported_surfaces_reflect_current_proof` (green),
  `disclosed_partial_export_refresh` (yellow: the export reflects the current proof while some
  low-priority detail is trimmed), or `exported_proof_stale_or_divergent` (red: the exported proof
  is stale or divergent from the current primitive state).

Each row also carries the hard invariant `never_hover_spinner_or_pointer_only`; `false` is a blocker
(a primitive keeps a critical truth or progress visible only through a hover reveal, a transient
spinner, or a pointer-only affordance, with no keyboard/focus or touch alternative).

## Derived status and the structural lints

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when it
discloses a reduced truth scope, a partial representation, a reduced interaction reach (backed by a
waiver), or a partial export refresh. It drops to `red` when any axis reaches its blocked state, a
primitive keeps critical truth hover-/spinner-/pointer-only, or its accessibility routes / required
labels / profile coverage are incomplete. Those structural lints — `accessibility_routes_complete`,
`required_labels_complete`, and `profiles_complete` — are what prevent a later primitive from
shipping without keyboard-focusable, screen-reader-announced, non-hover-reachable, pointer-optional,
high-contrast-safe, and support-exportable routes, without its identity/state/keyboard-route labels,
or without compact/high-zoom/high-contrast/reduced-motion (and standard/expanded/multi-window)
profile coverage. The rows must also cover all four truth pillars. The Rust validator in
`crates/aureline-shell/src/m5_shell_primitive_release_proof` is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_reduced_reach_or_resize` narrowing
must additionally carry an active, matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact certification causes, and the blocking findings
  the lane refuses to ship with.
- **Certification dashboard** — a light projection the shell / release automation / evidence index
  reads to auto-narrow a claimed primitive when its release proof falls out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix ref, build
  id, each family, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels — never raw
URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitive_release_proof`)
is the only mint-from-truth path for:

- `artifacts/release/m5-shell-primitive-release-proof-proof/packet.json`
- `artifacts/release/m5-shell-primitive-release-proof-proof/dashboard.json`
- `artifacts/release/m5-shell-primitive-release-proof-proof/support_export.json`
- `artifacts/release/m5-shell-primitive-release-proof-proof/matrix.csv`
- `artifacts/shell/m5-shell-primitive-release-proof.md` (this report's rendered companion)
- `fixtures/ui/m5-shell-primitive-release-proof/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-shell-primitive-release-proof.schema.json`](../../schemas/shell/m5-shell-primitive-release-proof.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitive_release_proof -- validate
cargo test -p aureline-shell --test m5_shell_primitive_release_proof_fixtures
cargo test -p aureline-shell m5_shell_primitive_release_proof
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitive_release_proof --"
$BIN packet         > artifacts/release/m5-shell-primitive-release-proof-proof/packet.json
$BIN dashboard      > artifacts/release/m5-shell-primitive-release-proof-proof/dashboard.json
$BIN support-export > artifacts/release/m5-shell-primitive-release-proof-proof/support_export.json
$BIN csv            > artifacts/release/m5-shell-primitive-release-proof-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-shell-primitive-release-proof.md
$BIN packet         > fixtures/ui/m5-shell-primitive-release-proof/packet.json
$BIN dashboard      > fixtures/ui/m5-shell-primitive-release-proof/dashboard.json
$BIN support-export > fixtures/ui/m5-shell-primitive-release-proof/support_export.json
$BIN compact        > fixtures/ui/m5-shell-primitive-release-proof/compact.txt
```
