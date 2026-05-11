# Proof packet: M1 notebook trust badges and representation-state cues seed

Purpose: anchor proof captures for the M1 bounded prototype that
exposes **notebook trust** and **representation state** explicitly on
one certified notebook-like preview row. The wedge keeps workspace,
notebook, kernel, output, and widget trust visibly distinct, and a row
that claims it will autoexecute active content on notebook open is
rejected with a typed invariant.

Reviewer landing page:
[`docs/ux/m1_notebook_trust_badges_seed.md`](../../../../docs/ux/m1_notebook_trust_badges_seed.md).

## Canonical sources

- Crate (wedge + projection): `crates/aureline-shell/`
  - `src/notebook_trust_badges/mod.rs` — `NotebookTrustBadgeWedge`,
    `NotebookTrustBadgeCardRecord`, `NotebookTrustBadgeRow`,
    `NotebookTrustBadgeRowBuilder`, the typed
    `WorkspaceTrustState` / `NotebookTrustRung` /
    `KernelAvailability` / `OutputTrustState` / `WidgetTrustState`
    axes, the `CellContentClass`, `RepresentationState`, and
    `EscapeHatch` vocabularies, the canonical
    `NotebookTrustBadgeClaimLimit` set, the typed
    `NotebookTrustBadgeInvariantViolation` rejection set, and the
    deterministic `render_plaintext()` block.
  - `src/notebook_trust_badges/tests.rs` — unit + fixture tests
    covering both protected walks, the named failure drill, and the
    adjacent drills on active-content-on-untrusted-rungs, missing
    escape hatches, widget rows without explicit widget trust, live
    outputs without a kernel, trust-axes collapse, deterministic
    plaintext rendering, and serde round-trip.
- Crate (shared degraded-state vocabulary): `crates/aureline-shell/`
  - `src/state_cards/degraded_state.rs` — `DegradedStateToken`
    (`Warming` / `Limited` / `PolicyBlocked` / ...). The wedge maps
    its per-row degraded chip through this vocabulary instead of
    forking a notebook-specific one.
- Crate (shared representation vocabulary): `crates/aureline-content-safety/`
  - `src/transfer.rs` — `RepresentationActionId`,
    `RepresentationClass`, `BodyPosture`. The wedge's
    [`RepresentationState`](../../../crates/aureline-shell/src/notebook_trust_badges/mod.rs)
    is a superset that adds `sandboxed_active` and
    `tombstone_static_fallback` while reusing the existing
    `raw` / `sanitized` / `escaped` / `blocked_metadata_only` tokens.
- Fixtures: `fixtures/notebooks/m1_trust_badge_cases/`
  - `protected_walk_fully_trusted_local.json` — fully trusted local
    notebook, all axes set independently, no honesty markers, no
    invariant violations, code cell still carries a `safe_preview`
    escape hatch.
  - `protected_walk_mixed_trust_untrusted.json` — imported,
    untrusted-tainted notebook with markdown / code / widget rows.
    Code cell renders escaped + `Limited`; widget renders
    tombstone-static-fallback + `PolicyBlocked`; axes remain
    visibly distinct.
  - `failure_drill_autoexecute_on_open.json` — named failure drill: a
    buggy caller flips `will_autoexecute_on_open = true`. The wedge
    surfaces the typed `autoexecute_on_open` invariant and lights
    `any_row_claims_autoexecute_on_open`.
- Reviewer doc: `docs/ux/m1_notebook_trust_badges_seed.md`

## Upstream contracts the wedge projects against (without forking)

- `docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md` —
  the frozen `notebook_trust_rung` ladder and the four-axis trust
  posture vocabulary the wedge mirrors verbatim.
- `docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`
  — canonical notebook document model, kernel transport, and trust
  axes.
- `crates/aureline-content-safety/src/transfer.rs` — the
  representation-class / action-id vocabulary the wedge speaks.
- `crates/aureline-shell/src/state_cards/degraded_state.rs` — the
  shared `DegradedStateToken` chrome chips the wedge maps per-row
  degraded badges through.

## Protected walks

1. **Fully trusted local notebook.** Open the wedge against a
   fully-trusted local notebook with a markdown cell, a code cell, and
   a rich output. The wedge MUST render with `workspace_trust_state =
   trusted_workspace`, `notebook_trust_rung = fully_trusted_user`,
   `kernel_availability = local_managed_available`,
   `output_trust_state = live_from_current_session`, and
   `widget_trust_state = not_applicable` — all five axes set
   independently and visibly distinct. The code cell carries a
   `safe_preview` escape hatch even at the highest trust rung. No row
   sets `will_autoexecute_on_open`; no invariant violations fire.
2. **Mixed-trust imported notebook.** Open the wedge against an
   imported, untrusted-tainted notebook with the same shape. The
   code cell renders `representation_state = escaped`, the widget
   output renders `representation_state = tombstone_static_fallback`,
   both rows carry honesty markers, and both rows offer the
   `safe_preview` escape hatch. The five axes remain visibly distinct;
   no row autoexecutes on open; no invariant violations fire.

Evidence:

- `crates/aureline-shell/src/notebook_trust_badges/tests.rs::protected_walk_fully_trusted_local_renders_clean_card`
- `crates/aureline-shell/src/notebook_trust_badges/tests.rs::protected_walk_mixed_trust_untrusted_notebook_keeps_axes_distinct`
- `crates/aureline-shell/src/notebook_trust_badges/tests.rs::fixture_protected_walk_fully_trusted_local_replays_into_the_wedge`
- `crates/aureline-shell/src/notebook_trust_badges/tests.rs::fixture_protected_walk_mixed_trust_untrusted_replays_into_the_wedge`
- Fixtures:
  `fixtures/notebooks/m1_trust_badge_cases/protected_walk_fully_trusted_local.json`,
  `fixtures/notebooks/m1_trust_badge_cases/protected_walk_mixed_trust_untrusted.json`

## Failure drill — notebook open MUST NOT autoexecute active content

A buggy caller flips `will_autoexecute_on_open = true` on a code cell.
The wedge MUST surface the typed
`NotebookTrustBadgeInvariantViolation::AutoexecuteOnOpen { row_id }`
against the offending row, light
`any_row_claims_autoexecute_on_open` on the card, and render a
summary line ending `INVARIANTS BLOCKED` so the chrome cannot let
notebook open silently run the cell.

Evidence:

- `crates/aureline-shell/src/notebook_trust_badges/tests.rs::failure_drill_autoexecute_on_open_is_rejected_with_typed_invariant`
- `crates/aureline-shell/src/notebook_trust_badges/tests.rs::fixture_failure_drill_autoexecute_on_open_surfaces_typed_invariant`
- Fixture:
  `fixtures/notebooks/m1_trust_badge_cases/failure_drill_autoexecute_on_open.json`

## Adjacent drills — axes stay distinct; representation stays honest

- `active_content_on_untrusted_rung_surfaces_typed_invariant` — a
  code cell that renders `sandboxed_active` under
  `untrusted_tainted` lands the typed
  `active_content_on_untrusted_rung` invariant rather than silently
  exposing live content.
- `missing_safe_preview_escape_hatch_on_active_content_is_rejected` —
  a code cell row that drops every escape hatch lands the
  `missing_safe_preview_escape_hatch` invariant.
- `widget_row_without_explicit_widget_trust_is_rejected` — a widget
  row that leaves `widget_trust_state = not_applicable` lands the
  `widget_trust_not_applicable_for_widget` invariant.
- `live_outputs_without_kernel_is_rejected` — outputs claim
  `live_from_current_session` while the kernel is unavailable. The
  wedge surfaces `live_outputs_without_kernel`.
- `trust_axes_collapse_when_code_cell_runs_with_not_applicable_kernel`
  — a code cell row exists but kernel availability is
  `not_applicable`. The wedge surfaces `trust_axes_collapsed` and
  names the collapsed axis verbatim.

## Validation command

```
cargo test -p aureline-shell --lib notebook_trust_badges
```

## Evidence storage

- Crate sources:
  `crates/aureline-shell/src/notebook_trust_badges/`,
  `crates/aureline-shell/src/state_cards/degraded_state.rs`,
  `crates/aureline-content-safety/src/transfer.rs`
- Reviewer doc: `docs/ux/m1_notebook_trust_badges_seed.md`
- Fixtures: `fixtures/notebooks/m1_trust_badge_cases/`
