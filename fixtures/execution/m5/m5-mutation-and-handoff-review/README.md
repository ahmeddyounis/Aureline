# Fixtures: M5 mutation-and-handoff review matrix

This directory contains fixture metadata for the `m5_mutation_and_handoff_review_matrix`
packet.

The canonical full corpus is checked in at:

`artifacts/execution/m5/m5-mutation-and-handoff-review.json`

## Coverage

- `request_workspace_mutation`, `browser_runtime_action`, `preview_route_action`,
  `live_resource_operation`, `remote_mutation`, `companion_handoff`,
  `tunnel_route_action`, and `provider_console_handoff` are the only claimed mutation
  paths, and each carries exactly one row — no path inherits a reviewed apply from an
  adjacent one.
- Each path carries its own actor, target-context, review-sheet, mutation-receipt,
  execution, and support-export ref; every granted or pending approval also carries an
  approval-ticket ref, every compensable or provider-managed rollback carries a
  rollback-plan ref, and every time-bound route carries a route-expiry ref, so a reviewed
  mutation never drops its approval, rollback, or route-expiry semantics.
- Actor authority covers `verified`, `delegated`, `inherited`, and `unestablished`;
  approval covers `approved`, `not_required`, `required`, and `bypassed`; rollback covers
  `reversible`, `compensable`, `provider_managed`, and `unknown`; route effect covers
  `none`, `time_bound`, `persistent`, and `unbounded`; handoff continuity covers
  `not_handoff`, `preserved`, `renegotiated`, and `severed`. Duration covers `instant`,
  `short`, `extended`, and `open_ended`, and the fallback path covers `reviewed_retry`,
  `open_in_provider`, `vendor_console`, and `no_fallback`.
- Published readiness covers `reviewed_apply`, `approval_required`, `preview_only`, and
  `blocked`, and the review decision covers `apply`, `require_approval`, `preview_only`,
  `flag_for_review`, and `withhold`.
- The six canonical narrowing reasons — `unverified_actor`, `inherited_authority`,
  `approval_bypassed`, `rollback_unknown`, `unbounded_route`, and `handoff_severed` — are
  each exercised by at least one path.
- The commit gate is exercised in every direction: the clean local
  `request_workspace_mutation` and the authority-preserving `browser_runtime_action`
  publish reviewed applies; the `live_resource_operation` narrows to require approval; the
  `remote_mutation` and `preview_route_action` narrow to preview-only; the
  `companion_handoff` flags its renegotiated handoff for review; and the
  `tunnel_route_action` and `provider_console_handoff` are withheld entirely. The
  `remote_mutation` row is the inherited-authority guardrail case — a path from a trusted
  shell whose authority is only inherited is capped at preview-only and flagged — while the
  `browser_runtime_action` row proves a handoff is not a blanket block: a handoff that
  preserves authority and rollback semantics may commit a reviewed apply. The
  `provider_console_handoff` row proves a severed handoff is withheld with a vendor-console
  fallback rather than masquerading as a committed mutation. Each row's
  `published_readiness`, `review_decision`, and `narrowing_reasons` equal the recomputed
  gate decision, so release and desktop/CLI tooling can prove underqualified paths narrow
  before they commit.
