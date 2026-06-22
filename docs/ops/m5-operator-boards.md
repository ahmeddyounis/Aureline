# Operator overview-board contract

This document freezes the first real Aureline operator **overview boards**: the
incident-response, support-queue, admin-approvals, and release-readiness boards an
operator actually opens. An overview board is a trustworthy summary over many
operator objects — not a separate truth model. Each board binds a surface family
from the [operator-surface matrix](./m5-operator-surfaces.md) and renders tiles
that summarize the same canonical incident, support, admin, and release objects
the detail surfaces own.

The hard part is keeping the summary honest as it scales. This contract pins five
things every board must hold:

1. **Canonical object identity, never a dashboard-only id.** Every tile carries
   an `object_ref` — the same `aureline://` handle the detail surfaces use — and
   its open-detail route is that exact ref.
2. **No silent green.** A tile's `effective_state` is *computed* from its
   displayed state, freshness, and blocker/waiver state, so a stale or waived tile
   can never be reported `clear`.
3. **Owner and blocker/waiver state stay first-class.** Owner, decision right, and
   a visible blocker/waiver reason are required tile fields, never hover-only
   chrome.
4. **Shared filters and saved views.** One filter-facet vocabulary spans every
   board; a saved view names its filters and its order (with a stated reason).
5. **Export parity.** Each board freezes its default view as a machine-readable
   export that preserves the exact filters, order, scope, freshness, ownership,
   and blocker/waiver semantics outside the live UI.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/ops/m5-operator-boards.schema.json`](../../schemas/ops/m5-operator-boards.schema.json)
  — boundary schema for `m5_operator_board_set`.
- [`/fixtures/ops/m5-operator-boards/canonical_boards.json`](../../fixtures/ops/m5-operator-boards/canonical_boards.json)
  — the published canonical board set; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/ops/m5-operator-boards.md`](../../artifacts/ops/m5-operator-boards.md)
  — the human-readable companion (board, tile, view, and invariant tables).
- `crates/aureline-support/src/m5_operator_boards/` — the builder, the
  no-silent-green tile rule, saved-view application, export, validation, and the
  human-readable projection.
- `cargo run -p aureline-support --example dump_m5_operator_boards` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Boards

Each board binds a surface family from the operator-surface matrix by that
matrix's own surface id, so the board renders the shared surface contract rather
than a per-surface clone.

| Board token | Board | Bound matrix surface | Scope | Default redaction |
| --- | --- | --- | --- | --- |
| `incident_response` | Incident response | `operator_surface.triage_inbox` | shared_team | metadata_safe_default |
| `support_queue` | Support queue | `operator_surface.triage_inbox` | shared_team | internal_support_restricted |
| `admin_approvals` | Admin approvals | `operator_surface.triage_inbox` | managed_org | operator_only_restricted |
| `release_readiness` | Release readiness | `operator_surface.operational_overview_board` | shared_team | metadata_safe_default |

Each board carries: a stable `board_id` (`operator_board.<token>`), the consumers
that render it, a default saved view, its saved views, its actions, its tiles, and
a frozen export of the default view.

## Tiles

A tile summarizes one canonical object. Required fields: the canonical
`object_ref`, the `object_kind`, the owner and decision right, the
`displayed_state`, the evidence `freshness`, the `blocker_waiver` state and a
visible `blocker_reason`, the computed `effective_state`, the scope, an explicit
rank, the `open_detail_ref` (which equals `object_ref`), and whether the tile is
comparable.

`effective_state` is computed, not stored as opinion:

- an active blocker (or an expired waiver) forces `blocked`;
- a live waiver forces `attention` — a waived finding is acknowledged risk, never
  green;
- otherwise, a would-be-`clear` tile whose evidence is not `fresh` or `recent`
  downgrades to `unconfirmed`.

The canonical fixture proves each downgrade path: a stale incident card downgraded
to `unconfirmed`, a waived review item rendered `attention`, and blocked
incident/support/admin tiles rendered `blocked`.

## Shared filters and saved views

One filter-facet vocabulary spans every board:

| Facet | Closed vocabulary | Filters on |
| --- | --- | --- |
| `state` | yes | computed effective state |
| `freshness` | yes | evidence age (`fresh`, `recent`, `stale`, `very_stale`, `never`) |
| `owner` | no (open) | owner |
| `blocker_waiver` | yes | `none`, `blocked`, `waived`, `waiver_expired` |
| `scope` | yes | `local_private`, `shared_team`, `managed_org` |
| `object_kind` | yes | canonical object kind |

A saved view is a named, shareable filter-and-order. It carries its own scope and
owner, a list of filter clauses (AND across clauses, OR within a clause), and an
order with one of the order keys — `effective_state_severity`, `freshness`,
`explicit_rank`, or `owner` — plus a stated reason so the board never sorts by a
hidden rule. `apply_view` is deterministic: it filters by the clauses, sorts by
the order key, and tie-breaks by `tile_id`.

## Export parity

`export_board_view` resolves a saved view into a `board_export_view`: a frozen,
ordered, filtered snapshot that preserves the applied filters, order, scope, and
owner, labels itself `snapshot_only`, and carries one row per surviving tile with
its `effective_state`, `freshness`, ownership, `blocker_reason`, and
`open_detail_ref`. Each board freezes the export of its default view, and the
`operator_boards.export_parity` invariant asserts that frozen export equals
re-applying the default view — so the truth survives outside the live UI and a
lossy export fails CI.

## Invariants

The builder computes each invariant's `holds` flag from the built boards, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `operator_boards.canonical_object_identity` — every tile carries an `aureline://`
  object handle and routes open-detail to that exact handle.
- `operator_boards.surface_binding` — every board binds a matrix surface family by
  the matrix's own surface id.
- `operator_boards.no_silent_green` — every tile's effective state equals the
  computed no-silent-green state.
- `operator_boards.owner_blocker_visible` — every tile names an owner and decision
  right, and a blocked/waived tile carries a visible reason.
- `operator_boards.saved_views_present` — every board has a saved view, its default
  resolves, and every view names its order.
- `operator_boards.shared_filter_vocabulary` — every filter clause references a
  defined facet and uses valid values on closed facets.
- `operator_boards.open_detail_parity` — every board offers a canonical open-detail
  action and every tile's open-detail route is its object handle.
- `operator_boards.export_parity` — every board's frozen export equals re-applying
  its default view.
- `operator_boards.export_preserves_truth` — every export carries the view's scope,
  owner, filters, and order, is labeled a snapshot, and preserves each row's state,
  freshness, ownership, and blocker/waiver reason.
- `operator_boards.first_real_boards_present` — the incident, support, admin, and
  release boards are all present.
- `operator_boards.stable_ids_unique` — board ids, view ids, and tile ids are
  unique.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw payloads, or
absolute paths — only opaque `aureline://` object handles, repo-relative refs,
stable tokens, and short reviewable sentences. `is_support_export_safe()` enforces
that `raw_payload_excluded` is true and every ref is a repo-relative object ref or
`aureline://` handle, so the set is safe to embed in a support export verbatim.

## Composes with

This contract builds on (and does not replace) the
[operator-surface matrix](./m5-operator-surfaces.md), which freezes the surface
families, the shared state vocabulary, and the operator paths these boards render
on. The boards bind that matrix for object identity rather than restating it.
