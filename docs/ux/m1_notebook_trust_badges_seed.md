# M1 notebook trust badges and representation-state cues seed

This page is the reviewer-facing landing page for the bounded prototype
that exposes **notebook trust** and **representation state** explicitly
on one certified notebook-like preview row. The wedge lives at
[`crates/aureline-shell/src/notebook_trust_badges/`](../../crates/aureline-shell/src/notebook_trust_badges/mod.rs)
and is exercised by the unit and fixture tests in
[`crates/aureline-shell/src/notebook_trust_badges/tests.rs`](../../crates/aureline-shell/src/notebook_trust_badges/tests.rs).

The wedge is bounded:

- It covers one notebook-like preview row. It does not stand up a
  notebook editor, kernel transport, diff/repair engine, widget
  admission pipeline, or rich-output sandbox.
- It does not invent new trust vocabulary. Every closed enum mirrors
  the upstream contracts verbatim:
  - [`docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`](../notebooks/notebook_trust_and_roundtrip_preview_contract.md)
    for the `notebook_trust_rung` ladder and the four-axis posture;
  - [`docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md)
    for the canonical document / kernel / output / widget trust axes;
  - [`crates/aureline-content-safety/src/transfer.rs`](../../crates/aureline-content-safety/src/transfer.rs)
    for the representation-class vocabulary the wedge speaks (`raw`,
    `sanitized`, `escaped`, with the notebook-specific
    `sandboxed_active` and `tombstone_static_fallback` additions);
  - [`crates/aureline-shell/src/state_cards/degraded_state.rs`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
    for the shared `DegradedStateToken` chrome chips
    (Warming / Limited / PolicyBlocked / Offline / ...).
- It does not silently autoexecute active content. Every
  `NotebookTrustBadgeRowBuilder` sets `will_autoexecute_on_open = false`
  by default, and a row that flips the bit is rejected through the
  typed `NotebookTrustBadgeInvariantViolation::AutoexecuteOnOpen`.

## What the wedge owns

- One canonical
  [`NotebookTrustBadgeCardRecord`](../../crates/aureline-shell/src/notebook_trust_badges/mod.rs)
  data shape carrying:
  - `record_kind`, `schema_version`, `prototype_label_token` (always
    `m1_prototype_notebook_trust_badges_and_representation_state`);
  - the **five distinct trust axes** as separate tokens:
    `workspace_trust_state`, `notebook_trust_rung`,
    `kernel_availability`, `output_trust_state`, `widget_trust_state`;
  - a list of per-row [`NotebookTrustBadgeRow`](../../crates/aureline-shell/src/notebook_trust_badges/mod.rs)
    entries, each carrying the cell- or output-level content class
    (markdown / code / rich-output / widget-output), the currently
    visible `representation_state`
    (raw / sanitized / escaped / sandboxed_active /
    tombstone_static_fallback / blocked_metadata_only), the optional
    per-row degraded chip, a closed list of escape hatches
    (`safe_preview`, `open_in_browser`, `open_in_desktop`,
    `export_raw_source`, `keep_static_fallback`), and the
    non-optional `will_autoexecute_on_open` chip;
  - a canonical claim-limit set
    (`single_bounded_wedge_only`, `no_autoexecute_on_open`,
    `trust_axes_remain_distinct`,
    `no_kernel_or_transport_orchestration`,
    `no_widget_admission_pipeline`) the chrome quotes verbatim
    under every card;
  - a typed [`NotebookTrustBadgeInvariantViolation`](../../crates/aureline-shell/src/notebook_trust_badges/mod.rs)
    list surfaced on the card so a buggy caller cannot smuggle a
    flattened axis, an autoexecute claim, or a missing escape hatch
    past the chrome.
- A deterministic
  [`NotebookTrustBadgeCardRecord::render_plaintext()`](../../crates/aureline-shell/src/notebook_trust_badges/mod.rs)
  block downstream support exports and proof captures quote verbatim.

## Trust axes the wedge keeps visibly distinct

| Axis | Vocabulary source | Why it stays separate |
|---|---|---|
| Workspace trust | shell-wide workspace-trust state | The user/admin may trust the workspace without trusting an imported notebook. |
| Notebook trust rung | `notebook_trust_rung` (untrusted_tainted, untrusted_quarantined_for_review, structural_only_trusted, selective_cell_trust, fully_trusted_user, fully_trusted_workspace_policy, trust_revoked_pending_review) | Document trust is independent of kernel transport, output liveness, and widget binding. |
| Kernel availability | local_managed_available, local_managed_unavailable, remote_managed_available, remote_managed_unavailable, kernel_denied_by_policy, not_applicable | A trusted notebook may still have no kernel; an untrusted notebook may still have a kernel installed. |
| Output trust | live_from_current_session, captured_from_prior_session, replayed_from_snapshot, orphaned_output, widget_gated, not_applicable | Outputs may be evidence from a prior session even when the kernel cannot rerun them now. |
| Widget trust | widget_denied_by_default, widget_admitted_after_preview, widget_suppressed_by_policy, widget_content_class_blocked, widget_runtime_unavailable, not_applicable | Widget live binding is a separate admission from running code in a cell. |

The `trust_axes_collapsed` invariant fires when the wedge would
default any of these axes onto the notebook rung (e.g. a code cell
exists but `kernel_availability = not_applicable`, or the notebook is
fully trusted but workspace trust reads `unknown_workspace`).

## Representation state vocabulary

The wedge speaks the safe-preview representation vocabulary
[verbatim](../../crates/aureline-content-safety/src/transfer.rs)
(raw / sanitized / escaped / blocked_metadata_only) and adds two
notebook-specific cases:

- `sandboxed_active` — active content rendered inside an explicit
  sandbox boundary. This is the **only** representation that lets a
  user view live code or widget binding; the wedge refuses to combine
  it with an untrusted notebook rung.
- `tombstone_static_fallback` — widget / live output downgraded to a
  static fallback chip. The chrome quotes the fallback verbatim and
  offers the `keep_static_fallback` escape hatch alongside
  `safe_preview` so the user can keep the rendering without elevating
  trust.

## Protected walks

The fixtures in
[`fixtures/notebooks/m1_trust_badge_cases/`](../../fixtures/notebooks/m1_trust_badge_cases/)
drive the protected walks through the tests in
[`crates/aureline-shell/src/notebook_trust_badges/tests.rs`](../../crates/aureline-shell/src/notebook_trust_badges/tests.rs):

1. **Fully trusted local notebook** —
   [`protected_walk_fully_trusted_local.json`](../../fixtures/notebooks/m1_trust_badge_cases/protected_walk_fully_trusted_local.json).
   A fully trusted local notebook with one markdown cell, one
   sanitized code cell, and one rich output. Every axis is set
   independently
   (workspace=trusted, notebook=fully_trusted_user, kernel=local_managed_available,
   outputs=live_from_current_session, widget=not_applicable). The
   card renders with no honesty markers and no invariant violations.
   The code cell row still carries a `safe_preview` escape hatch even
   at the highest trust rung.
   Exercised by
   [`fixture_protected_walk_fully_trusted_local_replays_into_the_wedge`](../../crates/aureline-shell/src/notebook_trust_badges/tests.rs).
2. **Mixed-trust imported notebook** —
   [`protected_walk_mixed_trust_untrusted.json`](../../fixtures/notebooks/m1_trust_badge_cases/protected_walk_mixed_trust_untrusted.json).
   An imported, untrusted-tainted notebook with the same markdown +
   code + widget shape. The code cell renders `escaped` with an
   honesty marker and a `Limited` degraded chip; the widget output
   renders `tombstone_static_fallback` with `PolicyBlocked` and the
   `keep_static_fallback` escape hatch. Notebook, workspace, kernel,
   output, and widget axes remain visibly distinct: notebook
   untrusted, workspace restricted, kernel unavailable, outputs
   captured from a prior session, widgets denied. No row autoexecutes
   on open; no invariant violations fire.
   Exercised by
   [`fixture_protected_walk_mixed_trust_untrusted_replays_into_the_wedge`](../../crates/aureline-shell/src/notebook_trust_badges/tests.rs).

## Failure drill — `autoexecute_on_open` is refused

[`failure_drill_autoexecute_on_open.json`](../../fixtures/notebooks/m1_trust_badge_cases/failure_drill_autoexecute_on_open.json)
exercises the named failure drill ("notebook open must not auto-run
active content"). A buggy caller flips
`will_autoexecute_on_open = true` on a code cell. The drill confirms:

- the typed
  `NotebookTrustBadgeInvariantViolation::AutoexecuteOnOpen { row_id }`
  fires against the offending row;
- the card's `has_invariant_violations` and
  `any_row_claims_autoexecute_on_open` chips both light;
- the summary line reads `INVARIANTS BLOCKED` so the chrome must
  surface the failure before any active content can run.

Exercised by
[`failure_drill_autoexecute_on_open_is_rejected_with_typed_invariant`](../../crates/aureline-shell/src/notebook_trust_badges/tests.rs)
and
[`fixture_failure_drill_autoexecute_on_open_surfaces_typed_invariant`](../../crates/aureline-shell/src/notebook_trust_badges/tests.rs).

## Adjacent drills

- `active_content_on_untrusted_rung_surfaces_typed_invariant` — a code
  cell rendered `sandboxed_active` under `untrusted_tainted` lands the
  typed `active_content_on_untrusted_rung` invariant.
- `missing_safe_preview_escape_hatch_on_active_content_is_rejected` —
  a code-cell row with no escape hatches lands the typed
  `missing_safe_preview_escape_hatch` invariant.
- `widget_row_without_explicit_widget_trust_is_rejected` — a widget
  output row that leaves widget trust as `not_applicable` lands the
  typed `widget_trust_not_applicable_for_widget` invariant.
- `live_outputs_without_kernel_is_rejected` — outputs claim to be live
  from the current session, but kernel availability is unavailable.
  The wedge surfaces `live_outputs_without_kernel`.
- `trust_axes_collapse_when_code_cell_runs_with_not_applicable_kernel`
  — a code cell exists but kernel availability is `not_applicable`.
  The wedge surfaces `trust_axes_collapsed` and names the collapsed
  axis verbatim.

## Shared contracts the wedge projects against

The seed reuses these existing truth sources without forking:

- [`docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`](../notebooks/notebook_trust_and_roundtrip_preview_contract.md)
  — the frozen `notebook_trust_rung` ladder and the four-axis trust
  posture vocabulary.
- [`docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md)
  — canonical notebook document model, kernel transport, and trust
  axes.
- [`crates/aureline-content-safety/src/transfer.rs`](../../crates/aureline-content-safety/src/transfer.rs)
  and the adjacent safe-preview wedge
  [`docs/ux/m1_safe_preview_and_copy_export.md`](m1_safe_preview_and_copy_export.md)
  — the representation-class / action-id / scope / transform /
  omission vocabulary the wedge speaks.
- [`crates/aureline-shell/src/state_cards/degraded_state.rs`](../../crates/aureline-shell/src/state_cards/degraded_state.rs)
  — the shared `DegradedStateToken` chrome chips.

## Out of scope (deliberately)

- Notebook editor / runtime / kernel transport depth. The wedge does
  not own how cells execute, how kernels attach, or how outputs are
  produced.
- Diff / repair engines or rich-output sandbox productization. The
  wedge records the trust posture and representation state the chrome
  quotes; it does not own the rendering pipeline.
- Widget admission pipeline. The wedge surfaces the
  `widget_admitted_after_preview` token verbatim but does not own the
  admission flow.
- Broad market-facing notebook claims. The wedge is one bounded
  prototype row; the chrome quotes the prototype label chip verbatim
  on every card so downstream review cannot infer hidden depth.

## Validation command

```
cargo test -p aureline-shell --lib notebook_trust_badges
```
