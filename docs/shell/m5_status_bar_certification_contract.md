# M5 status-bar item priority, placement, overflow & inspector back-link contract

This lane is the **status-bar certification capstone** on top of the frozen
[M5 status-bar, transient-inspect, pane-control, and durable-progress-component
matrix](m5_shell_primitives_contract.md). Where the matrix *freezes* the ambient
status primitives — the status-bar item and the status overflow menu, with their
status-item classes, overflow / severe-state displacement behaviors,
source/provider/freshness labels, accessibility routes, and mandatory labels — this
lane *certifies* that, in every claimed M5 status context, the status bar keeps
recovery-critical, execution/context, ongoing-work, and ambient-metadata items in
stable priority slots that never jitter or reflow around a spinner or vanity item,
that everything visible or overflowed stays reachable through keyboard search, a
status menu, or a palette route using the same label, that every item links back to
the narrowest useful inspector or command rather than a generic settings detour, and
that a support/export packet can reconstruct the visible and overflowed items, their
owning subsystems, and any critical-state displacement without a screenshot.

The lane exists so that M5 can honestly claim mature shell quality: users never have
to guess what a status item means, lose ambient state after looking away, or chase a
critical indicator through a hover-only reveal.

## Governed status contexts

The certification proof covers exactly eight claimed M5 status contexts, and refuses
to ship if any is missing:

- `notebook_lane` — Notebook lane status bar
- `data_api_lane` — Data / API lane status bar
- `remote_lane` — Remote lane status bar
- `preview_lane` — Preview lane status bar
- `review_lane` — Review lane status bar
- `profiler_lane` — Profiler lane status bar
- `incident_lane` — Incident lane status bar
- `desktop_base_lane` — Desktop base status bar

## Priority classes (stable placement order)

Every context keeps its items in the canonical, recovery-critical-first order — the
placement contract that preserves muscle memory and critical-state visibility:

1. `recovery_critical` (rank 0) — never displaced by ambient noise.
2. `execution_context` (rank 1) — the active target, mode, or deployment profile.
3. `ongoing_work` (rank 2) — background jobs, sync, progress attribution.
4. `ambient_metadata` (rank 3) — the first to compact into overflow.

## Per-context certification row

Each row names the ambient status primitives it drives (`status_bar_item` and
`status_overflow_menu`), the priority classes it keeps in stable slots, the reach
routes every item and overflow entry resolves through, and — pulled straight from the
frozen matrix's status-bar-item row — the status-item classes, overflow behaviors,
freshness labels, accessibility routes, required labels, consumer surfaces, and
downgrade triggers. It is certified across four posture axes:

- **placement stability** — `stable_priority_slots_no_jitter` (green),
  `disclosed_compact_priority_compaction` (yellow: a waivered compaction that drops
  only ambient-metadata items while recovery-critical / execution-context items stay
  pinned), or `unstable_slots_or_vanity_reflow` (red: the bar jitters or reflows
  around a spinner / vanity item, or a severe state displaces a truth-bearing peer).
- **overflow discoverability** — `keyboard_menu_palette_reachable` (green),
  `disclosed_reduced_overflow_route` (yellow: one route reduced, at least one
  non-hover route remains, disclosed), or `overflow_hover_or_pointer_only` (red).
- **inspector back-link** — `every_item_backlinks_to_narrowest_inspector` (green),
  `disclosed_grouped_backlink` (yellow), or `backlink_missing_or_generic_detour`
  (red: a missing back-link or a generic settings detour).
- **support-export parity** — `visible_and_overflowed_items_reconstructable` (green),
  `disclosed_partial_capture` (yellow), or `critical_displacement_absent_from_capture`
  (red).

Each row also carries the hard invariant `keyboard_reachable_without_hover`; `false`
is a blocker (a status item kept reachable only through pointer hover).

## Derived status and the structural lints

The green/yellow/red status is **derived, never asserted**. A row drops to `yellow`
when it discloses a compact priority compaction (backed by a waiver), a reduced
overflow route, a grouped inspector back-link, or a partial support-export capture. It
drops to `red` when any axis reaches its blocked state, a status item is hover-only,
its certified priority classes are not the canonical recovery-critical-first order,
its reach routes are incomplete, or it does not certify every frozen status-item
class. Those structural lints — `priority_order_well_formed`, `reach_routes_complete`,
`status_item_classes_complete` — are what prevent a later status bar from shipping a
jittering slot order, a hover-only overflow, or a dropped ambient class as stable. The
Rust validator in `crates/aureline-shell/src/m5_status_bar_certification` is the
authoritative gate.

A narrowed (non-green) row must disclose a reason; a
`disclosed_compact_priority_compaction` narrowing must additionally carry an active,
matching, unexpired waiver.

## Records

- **Certification packet** — the full set of rows with derived per-row status,
  aggregate green/yellow/red counts, active waivers, the exact certification causes,
  and the blocking findings the lane refuses to ship with.
- **Certification dashboard** — a light projection the status bar / attention router /
  release automation reads to auto-narrow a claimed status context when its
  certification proof falls out of policy.
- **Support export** — the packet plus dashboard plus stable case ids (packet id,
  matrix ref, build id, each context, each waiver id).

The records carry only stable ids, closed vocabulary, counts, refs, and short
labels — never raw URLs, raw local paths, raw usernames, raw hostnames, tokens, or
credentials.

## Artifacts

The headless emitter
(`cargo run -q -p aureline-shell --bin aureline_shell_m5_status_bar_certification`) is
the only mint-from-truth path for:

- `artifacts/release/m5-status-bar-certification-proof/packet.json`
- `artifacts/release/m5-status-bar-certification-proof/dashboard.json`
- `artifacts/release/m5-status-bar-certification-proof/support_export.json`
- `artifacts/release/m5-status-bar-certification-proof/matrix.csv`
- `artifacts/shell/m5-status-bar-certification.md` (this report's rendered companion)
- `fixtures/ui/m5-status-bar-certification/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The boundary schema is
[`schemas/shell/m5-status-bar-certification.schema.json`](../../schemas/shell/m5-status-bar-certification.schema.json).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_status_bar_certification -- validate
cargo test -p aureline-shell --test m5_status_bar_certification_fixtures
cargo test -p aureline-shell m5_status_bar_certification
```

Regenerate the artifacts after any change to the seed:

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_status_bar_certification --"
$BIN packet         > artifacts/release/m5-status-bar-certification-proof/packet.json
$BIN dashboard      > artifacts/release/m5-status-bar-certification-proof/dashboard.json
$BIN support-export > artifacts/release/m5-status-bar-certification-proof/support_export.json
$BIN csv            > artifacts/release/m5-status-bar-certification-proof/matrix.csv
$BIN markdown       > artifacts/shell/m5-status-bar-certification.md
$BIN packet         > fixtures/ui/m5-status-bar-certification/packet.json
$BIN dashboard      > fixtures/ui/m5-status-bar-certification/dashboard.json
$BIN support-export > fixtures/ui/m5-status-bar-certification/support_export.json
$BIN compact        > fixtures/ui/m5-status-bar-certification/compact.txt
```
