# Procurement / verification packet contract

This document covers the *rendered* procurement / verification packets, renewal /
trial / seat-change summary cards, and admin-handoff packets: the concrete, typed
instances of the procurement surface that Aureline shows on its claimed
managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles.

Where the [admin-plane matrix](./m5-admin-plane.md) *names and freezes the
contract* — including the `procurement_verification_packet` surface family, the
states it admits, the controlled vocabularies it binds, and the proof packet that
keeps it current — this lane *renders that surface*. It turns procurement,
verification, renewal, and admin handoff into first-class local product surfaces: an
evaluator, auditor, renewer, or support engineer can, on the machine in front of
them, read the deployment mode, supported export paths, billing/owner scope,
validity-window and signature posture, evidence refs, residual dependencies, and
support/renewal handoff data that prove current posture; see each renewal, trial, or
seat-change event with its effective date, impacted managed features, as-of date,
local-only path, and the export/support next step; and export an admin-handoff
packet with build/channel, install mode, workspace archetype, bundle ids, and
affected features — all without a separate vendor console and without a still-active
paid seat to recover user-owned data.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update in
the same change.

## Companion artifacts

- [`/schemas/admin/m5-procurement.schema.json`](../../schemas/admin/m5-procurement.schema.json)
  — boundary schema for `m5_procurement_bundle`.
- [`/fixtures/admin/m5-procurement/canonical_procurement.json`](../../fixtures/admin/m5-procurement/canonical_procurement.json)
  — the published canonical procurement bundle; the freeze gate asserts the in-code
  builder equals it byte-for-byte.
- [`/artifacts/admin/m5-procurement.md`](../../artifacts/admin/m5-procurement.md)
  — the human-readable companion (per-profile tables).
- `crates/aureline-policy/src/m5_procurement/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-policy --example dump_m5_procurement` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Binds back to the matrix

The render layer is not free-form. Each profile packet binds back to the frozen
[admin-plane matrix](./m5-admin-plane.md):

- **Every state it shows is one the matrix admits.** The verification packet's
  `machine_state`, every event card's `machine_state`, the admin-handoff
  `machine_state`, and the `coverage.coverage_state` must each be in the matrix's
  `applicable_states` for the procurement surface — `active_enforced`,
  `signature_unverified`, `unconfirmed_stale`, `imported_snapshot_no_live`, or
  `unknown_requires_review` (`procurement.surface_states_within_matrix`).
- **Every token it uses is a matrix term.** The `owner_escalation`,
  `verification_signature_posture`, and freshness tokens are exactly the terms the
  matrix's shared vocabulary defines, and the completeness and export-form tokens
  are the ones the sibling render layers freeze.
- **One typed packet, every declared consumer.** Each profile is one packet
  consumed identically by the consumers the matrix declares for the procurement
  surface — commercial/procurement, Help/About, support export, release evidence,
  and managed service (`procurement.consumer_parity`).

So an edit that shows a state the matrix does not admit, or a token the matrix does
not define, flips an invariant and fails the freeze gate.

## The three rendered objects

Each profile renders three objects under the one procurement surface.

### 1. The procurement / verification packet

The metadata-safe posture proof a buyer or auditor needs. It names:

- the **deployment mode** and a one-line **summary**;
- the **verification / signature posture** and **machine state**;
- the **validity window** (`opens`, `closes`, `within_window`, and a window label)
  — a packet past validity or not currently verified is labeled, never shown
  verified (`procurement.validity_labeled`);
- the **billing / owner scope** and the **packet owner**, plus an **as-of date**
  (`procurement.owner_scope_and_asof`);
- the **supported export paths**, at least one of which works offline and none of
  which needs a paid seat (`procurement.export_paths_present`,
  `procurement.no_paid_seat_for_recovery`);
- the **evidence refs** that back current posture, each an export-safe repo ref
  (`procurement.evidence_refs_present`);
- the **residual dependencies**, each with a local-safe fallback
  (`procurement.residual_dependencies_honest`);
- the **canonical sources** it reuses by ref instead of restating
  (`procurement.reuses_canonical_objects`); and
- the **support / renewal handoff** owner and next step.

### 2. The renewal / trial / seat-change summary card

One card per commercial event. It discloses the **event type**, **effective date**,
**impacted managed features**, **as-of date**, **impacted billing scope**,
**local-only path**, and the **export/support next step**, and every event class
appears across the bundle (`procurement.events_disclose_impact`).

Crucially, in an entitlement-loss context a card **never outranks** the export,
delete, support, or local-continuation actions. Each card carries an ordered
`next_actions` list where every recovery action precedes any commercial
call-to-action, is flagged `outranks_recovery_actions = false`, and never requires a
paid seat to recover user-owned data (`procurement.events_never_outrank_recovery`).

### 3. The admin-handoff packet

Auto-derived from current managed state without manual curation. It carries the
**build ref** and **release channel**, **install mode**, **workspace archetype**,
**bundle ids**, **affected features**, and an **export-safe summary**
(`procurement.handoff_complete`).

## Honesty rules (computed invariants)

The builder computes each invariant's `holds` flag from the rendered data, so an
inconsistent edit flips an invariant and fails CI rather than passing silently:

- `procurement.verification_no_silent_green` — a packet, card, or handoff whose
  backing evidence is stale is never shown under a confirmed `active_enforced`
  state, and a packet that is not currently verified is never shown active.
- `procurement.reuses_canonical_objects` — every packet, card, and handoff reuses at
  least one canonical managed-state object (effective policy, entitlement/seat,
  retention/deletion, offboarding/continuity, endpoint posture, decision history) by
  an export-safe schema ref, and every family appears across the bundle.
- `procurement.locally_inspectable_offline` — every profile, including
  self-hosted, sovereign/air-gapped, and mirrored/offline, keeps a locally
  inspectable surface that needs no vendor console and stays exportable without a
  paid seat.
- `procurement.coverage_labeled` — an offline or past-validity coverage view is
  labeled with a non-complete completeness class and a non-active coverage state.
- `procurement.export_parity` — every object carries both an export-safe
  machine-readable summary and a plain-language handoff sentence, and every profile
  offers both export forms.
- `procurement.export_safe` — every stable id and schema ref is an opaque token or
  repo-relative ref, so the bundle is safe to embed in a support, procurement, or
  renewal export verbatim.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw provider payloads,
raw record bodies, or absolute paths — only opaque object refs, stable tokens,
rendered metadata-safe summaries, and short reviewable sentences. The freeze gate
re-proves support-export safety on the checked-in fixture.
