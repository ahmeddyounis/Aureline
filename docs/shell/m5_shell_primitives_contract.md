# M5 status-bar, transient-inspect, pane-control, and durable-progress-component matrix (contract)

Task M05-748 — Workstream batch B87 (status-bar, tooltip/hovercard/peek, splitter,
and durable-progress-component truth across claimed M5 surfaces).

This lane freezes the canonical object model and controlled vocabulary for the
high-frequency shell primitives M5 claims but had left too implicit: status-bar
items, status overflow menus, tooltips, hovercards, peek panels, pinned-preview
promotions, splitter handles, pane-resize presets, progress indicators, and
durable job-row components. Every primitive is named once, bound to a canonical
shell zone, the responsive classes it must survive, the window classes it keeps
continuity across, and the claimed M5 surface families that render it — then
constrained by the same freshness, accessibility, and serialization rules
regardless of the surface that reaches it. Later shell-primitives rows consume
this matrix rather than inventing surface-local terms.

## Source of truth

- **Rust module (authoritative validator + seed):**
  `crates/aureline-shell/src/freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix/`
- **Headless emitter:** `cargo run -p aureline-shell --bin aureline_shell_m5_shell_primitives -- <subcommand>`
- **Boundary schema:** `schemas/shell/m5-shell-primitives.schema.json`
- **Support export (checked in, `include_str!` canonical):**
  `artifacts/release/m5-shell-primitives-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-shell-primitives-proof/matrix.csv`
- **Markdown report:** `artifacts/shell/m5-shell-primitives.md`
- **Narrowed fixtures:** `fixtures/ui/m5-shell-primitives/`

The Rust `validate()` gate is authoritative; the schema documents the shape. The
seed builder is the single producer of the checked-in support export and the
narrowed fixtures, so the in-code matrix, the artifact, and the fixtures never
drift (enforced by `checked_support_export_matches_seed` and
`checked_narrowed_fixtures_validate_and_match_seed_builders`).

## Reused vocabulary (no parallel naming)

The shell topology is **not** re-minted here. The eight canonical shell zones,
the compact/standard/expanded responsive classes, the four window classes, the
ten claimed M5 surface families, and the shell consumer surfaces are re-exported
verbatim from
`freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix`.
A shell primitive can never invent its own slot, layout class, window class, or
surface family.

## Governed primitive families (10)

`status_bar_item`, `status_overflow_menu`, `tooltip`, `hovercard`, `peek_panel`,
`pinned_preview_promotion`, `splitter_handle`, `pane_resize_preset`,
`progress_indicator`, `durable_job_row`.

Family predicates drive the family-specific lints:

- **Ambient** (`status_bar_item`, `status_overflow_menu`) — must declare
  status-item classes and overflow behaviors.
- **Transient inspect** (`tooltip`, `hovercard`, `peek_panel`,
  `pinned_preview_promotion`) — must declare representation classes.
- **Promoting** (`peek_panel`, `pinned_preview_promotion`) — must declare
  promotion states.
- **Pane control** (`splitter_handle`, `pane_resize_preset`) — must declare
  pane-resize states.
- **Progress** (`progress_indicator`, `durable_job_row`) — must declare progress
  states.
- **Freshness-carrying** (ambient ∪ transient-inspect ∪ progress) — must declare
  source/provider/freshness labels; pure layout controls carry none.

## Controlled vocabularies (frozen, self-describing)

- **status_item_class** (8): background_work, connection_target,
  deployment_profile, sync_freshness, problem_count, mode_indicator,
  notification_summary, capacity_meter.
- **overflow_behavior** (6): priority_pinned, collapse_to_overflow_menu,
  group_into_summary, drop_vanity_item, promote_severe_state,
  keyboard_reachable_overflow.
- **source_freshness_label** (6): live_canonical, cached_snapshot,
  stale_invalidated, provider_attributed, sampled_approximate, refresh_in_flight.
- **representation_class** (6): plain_tooltip, rich_hovercard, structured_peek,
  pinned_peek, provenance_strip, truncated_with_reopen.
- **promotion_state** (6): transient, pinned, promoted_to_panel,
  detached_to_window, demoted_to_transient, dismissed_preserved.
- **pane_resize_state** (7): idle, dragging, keyboard_step, snapped_to_preset,
  reset_to_default, clamped_to_min_width, collapsed_to_rail.
- **progress_state** (8): queued, running, grouped_batch, paused, succeeded,
  failed, canceled_by_user, reopenable_history.
- **accessibility_route** (6): keyboard_focusable, screen_reader_announced,
  non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable.
- **required_label** (6): identity, state, keyboard_route, source_provider,
  freshness, reopen_path. Mandatory on every primitive: identity, state,
  keyboard_route.

The `vocabulary_set` block is rebuilt from the typed `ALL` arrays and must match
canonically (`vocabulary_set_drift` otherwise).

## Hard invariants (per row — all MUST be false)

- `reflows_around_vanity_items` — status bars never reflow around spinners or
  vanity items.
- `hides_source_or_freshness` — hovercards/peek panels never hide
  source/provider/freshness truth, including after pinning.
- `keeps_critical_truth_hover_only` — no critical state or progress is visible
  only through hover or a transient spinner.
- `resizable_by_pointer_only` — panes are always keyboard-addressable, never
  pointer-only.

Any true value trips `primitive_invariant_violated`.

## Downgrade triggers (12)

`vanity_item_reflow`, `spinner_only_state`, `hover_only_critical_truth`,
`source_freshness_hidden`, `stale_preview_mistaken_for_live`,
`promotion_dropped_truth`, `pointer_only_resize`, `resize_state_not_serializable`,
`progress_lost_on_look_away`, `grouped_progress_unattributed`,
`severe_state_displaced_truth`, `proof_stale`. Stale proof auto-narrows the
primitive (`auto_narrow_on_stale`).

## Packet-level review blocks

- **governance_review** — 11 flags (overflow-safe ambient instrumentation, no
  vanity reflow, transient inspect preserves source/freshness, pinned preview
  keeps representation truth, keyboard-addressable & serializable pane resize,
  durable/reopenable progress, no hover/spinner-only truth, severe state
  displaces vanity not truth, every primitive bound to a zone, every primitive
  declares an accessibility route, later rows cannot invent parallel vocabulary).
- **consumer_projection** — 6 flags binding status bar, hovercard/peek, splitter,
  activity/progress center, support/export, and the accessibility bridge to this
  single source.
- **proof_freshness** — SLO hours, last refresh, auto-narrow-on-stale.
- **release_posture** — release packet ref, shell-primitives audit ref,
  support-export parity required, accessibility parity required.

## Narrowed fixtures

Both keep all ten primitives present (so `validate()` passes) while demonstrating
per-family narrowing:

- `pane_resize_preset_beta_narrowed.json` — `pane_resize_preset` → Beta (a slice
  of presets do not yet round-trip across multi-window restore).
- `pinned_preview_promotion_preview_narrowed.json` — `pinned_preview_promotion` →
  Preview (pending provenance-retention proof across all promotion transitions).

## Regenerating the artifacts

```sh
BIN=target/debug/aureline_shell_m5_shell_primitives
cargo build -p aureline-shell --bin aureline_shell_m5_shell_primitives
$BIN support-export > artifacts/release/m5-shell-primitives-proof/support_export.json
$BIN csv           > artifacts/release/m5-shell-primitives-proof/matrix.csv
$BIN report        > artifacts/shell/m5-shell-primitives.md
$BIN fixture-pane-resize-preset-beta-narrowed > fixtures/ui/m5-shell-primitives/pane_resize_preset_beta_narrowed.json
$BIN fixture-pinned-preview-promotion-preview-narrowed > fixtures/ui/m5-shell-primitives/pinned_preview_promotion_preview_narrowed.json
$BIN validate
```
