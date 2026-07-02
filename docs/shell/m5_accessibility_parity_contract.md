# M5 shell-primitive accessibility parity contract

This lane is the **accessibility parity certification capstone** on top of the frozen
[M5 status-bar, transient-inspect, pane-control, and durable-progress-component
matrix](m5_shell_primitives_contract.md). Where the matrix *freezes* the ten governed shell
primitives — the status-bar item and overflow menu, the tooltip/hovercard/peek/pinned-preview
transient-inspect surfaces, the splitter handle and pane-resize preset, and the ambient
progress indicator and durable job row, with their status-item classes, overflow behaviors,
representation classes, promotion states, pane-resize states, progress states,
source/provider/freshness labels, non-visual accessibility routes, and mandatory labels —
this lane *certifies* that, in every claimed non-default accessibility condition, the status,
hover/peek, splitter, and progress primitives stay keyboard- and screen-reader-reachable with
focus that returns after dismiss; stay legible and stable under high-zoom and high-contrast;
keep durable text alternatives when motion is reduced and touch / context-action alternatives
where a pointer affordance would otherwise be required; and reconstruct their accessibility
posture and primitive state from reusable fixtures and a support export rather than ad hoc
visual or screenshot checks.

The lane exists so that M5 can honestly claim mature shell quality: users never have to guess
what a status item means, never lose progress after looking away, never resize panes through
brittle pointer-only hit targets, and never mistake a hover-only reveal for a reachable truth
— under keyboard, screen reader, high-zoom, high-contrast, reduced-motion, or touch.

## Governed accessibility conditions

The certification proof covers exactly seven claimed non-default accessibility conditions, and
refuses to ship if any is missing. These are exactly the fixture-coverage cases the acceptance
criteria require: keyboard reach, focus return, narration, touch / context-action, high-zoom,
high-contrast, and reduced-motion:

- `keyboard_reach` — Keyboard-only reach
- `focus_return` — Focus return after dismiss
- `screen_reader_narration` — Screen-reader narration
- `touch_context_action` — Touch / context-action alternatives
- `high_zoom` — High-zoom / large-text rendering
- `high_contrast` — High-contrast rendering
- `reduced_motion` — Reduced-motion rendering

## Per-condition certification row

Each row certifies all ten frozen shell primitives together and — pulled straight from the
union across the matrix's ten primitive rows — the status-item classes, overflow behaviors,
representation classes, promotion states, pane-resize states, progress states,
source/provider/freshness labels, accessibility routes, required labels, shell zones, consumer
surfaces, and downgrade triggers. The union covers the full six accessibility routes
(`keyboard_focusable`, `screen_reader_announced`, `non_hover_reachable`, `pointer_optional`,
`high_contrast_safe`, `support_exportable`) and the full six required labels (`identity`,
`state`, `keyboard_route`, `source_provider`, `freshness`, `reopen_path`). It is certified
across four posture axes:

- **non-visual reach** — `keyboard_focus_and_narration_reachable` (green),
  `disclosed_reduced_reach_detail` (yellow: a long narration abbreviates or a focus-return
  lands one level up while every primitive stays keyboard- and screen-reader-reachable), or
  `truth_reachable_by_pointer_or_hover_only` (red: a primitive's truth is reachable only by
  pointer or hover, or focus does not return after dismiss).
- **zoom / contrast stability** — `legible_stable_under_zoom_and_contrast` (green),
  `disclosed_reduced_zoom_contrast_detail` (yellow: a label wraps to a shorter form or a
  decorative accent drops while every primitive stays legible), or
  `truncated_or_unreadable_under_zoom_or_contrast` (red: a truth-bearing item is truncated,
  clipped, or unreadable under high-zoom or high-contrast).
- **motion / touch alternative** — `durable_text_and_touch_alternatives_present` (green),
  `disclosed_reduced_alternative_detail` (yellow: a coarser touch target or a summarized text
  alternative while a durable text and touch path stays present — backed by a waiver), or
  `motion_only_or_pointer_only_affordance` (red: critical state or progress is conveyed by
  motion only or a pointer affordance only, with no durable text or touch alternative).
- **accessibility export** — `accessibility_posture_and_state_reconstructable` (green),
  `disclosed_partial_capture` (yellow: the export reconstructs the accessibility posture while
  some low-priority primitive detail is trimmed), or
  `accessibility_state_absent_from_capture` (red: the accessibility state is absent from the
  support-export capture).

Each row also carries the hard invariant `never_pointer_or_hover_only`; `false` is a blocker
(a primitive keeps a critical truth or progress visible only through a pointer hover or a
pointer-only affordance, with no keyboard/focus or touch alternative).

## Derived status and the structural lints

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when it
discloses a reduced non-visual reach detail, a reduced zoom/contrast detail, a reduced
motion/touch alternative (backed by a waiver), or a partial support-export capture. It drops
to `red` when any axis reaches its blocked state, a primitive keeps critical truth pointer- or
hover-only, or its accessibility routes / required labels are incomplete. Those structural
lints — `accessibility_routes_complete` and `required_labels_complete` — are what prevent a
later condition from shipping without keyboard-focusable, screen-reader-announced,
non-hover-reachable, pointer-optional, high-contrast-safe, and support-exportable routes, or
without its identity/state/keyboard-route/source-provider/freshness/reopen-path labels, on the
certified primitives. The Rust validator in `crates/aureline-shell/src/m5_accessibility_parity`
is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_reduced_alternative_detail`
narrowing must additionally carry an active, matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact certification causes, and the blocking
  findings the lane refuses to ship with.
- **Certification dashboard** — a light projection the shell / accessibility bridge / release
  automation reads to auto-narrow a claimed condition when its accessibility parity proof falls
  out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix ref,
  build id, each condition, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels — never
raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_accessibility_parity`)
is the only mint-from-truth path for:

- `artifacts/release/m5-accessibility-parity-proof/packet.json`
- `artifacts/release/m5-accessibility-parity-proof/dashboard.json`
- `artifacts/release/m5-accessibility-parity-proof/support_export.json`
- `artifacts/release/m5-accessibility-parity-proof/matrix.csv`
- `artifacts/shell/m5-accessibility-parity.md` (this report's rendered companion)
- `fixtures/ui/m5-accessibility-parity/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-accessibility-parity.schema.json`](../../schemas/shell/m5-accessibility-parity.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_accessibility_parity -- validate
cargo test -p aureline-shell --test m5_accessibility_parity_fixtures
cargo test -p aureline-shell m5_accessibility_parity
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_accessibility_parity --"
$BIN packet         > artifacts/release/m5-accessibility-parity-proof/packet.json
$BIN dashboard      > artifacts/release/m5-accessibility-parity-proof/dashboard.json
$BIN support-export > artifacts/release/m5-accessibility-parity-proof/support_export.json
$BIN csv            > artifacts/release/m5-accessibility-parity-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-accessibility-parity.md
$BIN packet         > fixtures/ui/m5-accessibility-parity/packet.json
$BIN dashboard      > fixtures/ui/m5-accessibility-parity/dashboard.json
$BIN support-export > fixtures/ui/m5-accessibility-parity/support_export.json
$BIN compact        > fixtures/ui/m5-accessibility-parity/compact.txt
```
