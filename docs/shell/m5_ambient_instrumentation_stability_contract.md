# M5 ambient shell-instrumentation stability contract

This lane is the **ambient-instrumentation stability certification capstone** on top of the
frozen [M5 status-bar, transient-inspect, pane-control, and durable-progress-component
matrix](m5_shell_primitives_contract.md). Where the matrix *freezes* the ambient-instrumentation
primitives — the status-bar item, the status overflow menu, and the ambient progress
indicator, with their status-item classes, overflow behaviors, source/provider/freshness
labels, accessibility routes, and mandatory labels — this lane *certifies* that, in every
claimed M5 rendering profile, counters, spinners, and multi-job summaries stay legible and
compact without reflowing or flickering the status/header strip; that overflowed ambient
items stay searchable from the command palette or status menus with the same labels and
explanations used in the visible instrumentation; that multiple active jobs, repeated
updates, and quick state changes group into one meaningful summary rather than many
flickering primitives; and that this ambient-stability behavior is reconstructable from
reusable fixtures and a support export rather than ad hoc visual checks alone.

The lane exists so that M5 can honestly claim mature shell quality: users never have to
guess what a status item means, never lose an overflowed item to a hover-only reveal, and
never watch the status strip jitter as more live work lands.

## Governed rendering profiles

The certification proof covers exactly eight claimed M5 rendering profiles, and refuses to
ship if any is missing. The compact, high-zoom, reduced-motion, degraded-network, and
degraded-power profiles are exactly the fixture-coverage cases the acceptance criteria
require:

- `standard` — Standard desktop density
- `compact` — Compact / reduced-width density
- `expanded` — Expanded / wide density
- `multi_window` — Multi-window / detached shell
- `high_zoom` — High-zoom / large-text rendering
- `reduced_motion` — Reduced-motion rendering
- `degraded_network` — Degraded-network conditions
- `degraded_power` — Degraded-power / low-power conditions

## Per-profile certification row

Each row names the ambient primitives it drives (`status_bar_item`, `status_overflow_menu`,
and `progress_indicator`) and — pulled straight from the union of the frozen matrix's three
ambient rows — the status-item classes, overflow behaviors, source/provider/freshness
labels, required labels, accessibility routes, consumer surfaces, and downgrade triggers.
The union covers all eight status-item classes, all six overflow behaviors, and the full
six required labels — `identity`, `state`, `keyboard_route`, `source_provider`, `freshness`,
and `reopen_path`. It is certified across four posture axes:

- **counter stability** — `counter_spinner_summary_stable_no_reflow` (green),
  `disclosed_reduced_counter_detail` (yellow: a wide count abbreviates to a magnitude or a
  spinner label shortens while the item keeps its stable placement and meaning), or
  `status_reflows_or_flickers_on_update` (red: the status/header strip reflows or flickers
  when counters or spinners update).
- **overflow searchability** — `overflow_items_palette_searchable_same_labels` (green),
  `disclosed_reduced_overflow_search_detail` (yellow: the overflow search shows a shorter
  explanation or a grouped result while every item stays discoverable and keeps its label),
  or `overflow_item_undiscoverable_or_relabeled` (red: a displaced item drops out of the
  palette/status search or is relabeled, so it is reachable only by pointer hover).
- **grouped summary** — `multi_job_grouped_into_one_summary` (green),
  `disclosed_coarse_grouping` (yellow: distinct job classes fold into one chip sooner than
  the standard threshold while the summary stays meaningful and each job stays reachable —
  backed by a waiver), or `many_flickering_primitives_instead_of_summary` (red: many
  flickering primitives are shown instead of one grouped summary).
- **stability export** — `stability_fixtures_and_export_reconstructable` (green),
  `disclosed_partial_capture` (yellow: the export reconstructs the ambient instrumentation
  while some low-priority overflow entries are trimmed), or
  `stability_state_absent_from_capture` (red: the ambient-instrumentation state is absent
  from the support-export capture).

Each row also carries the hard invariant `never_reflows_around_vanity_items`; `false` is a
blocker (the status bar reflows around a vanity item, displacing a truth-bearing peer).

## Derived status and the structural lints

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow` when it
discloses a reduced counter detail, a reduced overflow-search detail, a coarse grouping
(backed by a waiver), or a partial support-export capture. It drops to `red` when any axis
reaches its blocked state, the status bar reflows around a vanity item, or its status-item
classes / overflow behaviors / required labels are incomplete. Those structural lints —
`status_item_classes_complete`, `overflow_behaviors_complete`, and `required_labels_complete`
— are what prevent a later ambient surface from shipping without every ambient truth class
staying legible, without a keyboard-reachable / summary-grouping / priority-pin overflow
path, or without its identity/state/keyboard-route/source-provider/freshness/reopen-path
labels. The Rust validator in
`crates/aureline-shell/src/m5_ambient_instrumentation_stability` is the authoritative gate.

A narrowed (non-green) row must disclose a reason; a `disclosed_coarse_grouping` narrowing
must additionally carry an active, matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status, aggregate
  green/yellow/red counts, active waivers, the exact certification causes, and the blocking
  findings the lane refuses to ship with.
- **Certification dashboard** — a light projection the shell / status bar / release
  automation reads to auto-narrow a claimed profile when its ambient-stability proof falls
  out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id, matrix
  ref, build id, each profile, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short labels —
never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_ambient_instrumentation_stability`)
is the only mint-from-truth path for:

- `artifacts/release/m5-ambient-instrumentation-stability-proof/packet.json`
- `artifacts/release/m5-ambient-instrumentation-stability-proof/dashboard.json`
- `artifacts/release/m5-ambient-instrumentation-stability-proof/support_export.json`
- `artifacts/release/m5-ambient-instrumentation-stability-proof/matrix.csv`
- `artifacts/shell/m5-ambient-instrumentation-stability.md` (this report's rendered companion)
- `fixtures/ui/m5-ambient-instrumentation-stability/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-ambient-instrumentation-stability.schema.json`](../../schemas/shell/m5-ambient-instrumentation-stability.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_ambient_instrumentation_stability -- validate
cargo test -p aureline-shell --test m5_ambient_instrumentation_stability_fixtures
cargo test -p aureline-shell m5_ambient_instrumentation_stability
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_ambient_instrumentation_stability --"
$BIN packet         > artifacts/release/m5-ambient-instrumentation-stability-proof/packet.json
$BIN dashboard      > artifacts/release/m5-ambient-instrumentation-stability-proof/dashboard.json
$BIN support-export > artifacts/release/m5-ambient-instrumentation-stability-proof/support_export.json
$BIN csv            > artifacts/release/m5-ambient-instrumentation-stability-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-ambient-instrumentation-stability.md
$BIN packet         > fixtures/ui/m5-ambient-instrumentation-stability/packet.json
$BIN dashboard      > fixtures/ui/m5-ambient-instrumentation-stability/dashboard.json
$BIN support-export > fixtures/ui/m5-ambient-instrumentation-stability/support_export.json
$BIN compact        > fixtures/ui/m5-ambient-instrumentation-stability/compact.txt
```
