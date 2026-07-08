# Merge-Readiness Panels: Governance and Queue-Block Truth

This document is the contract for the M5 packet that implements the reusable
merge-readiness panel, merge-queue entry, and stack-dependency chip. It narrows the
`merge_readiness_panel`, `merge_queue_entry`, and `stack_dependency_chip` components
frozen in the
[review-request/checks/merge-queue component matrix](../../../schemas/ui/m5-review-request-check-queue-component-matrix.schema.json)
into an implemented panel contract. The packet is the canonical M5 control source
for this lane: the review workspace, review lists, companion queues, handoff
packets, CLI/headless output, diagnostics, Help/About, merge-queue drawers, and
support exports ingest the checked-in packet rather than cloning queue pills or a
single provider-specific readiness number.

The goal is to make queue and landing readiness explicit **before** Aureline offers
merge, enqueue, restack, or handoff actions.

- Record kind: `merge_readiness_panel_governance_and_queue_block_truth`
- Schema: [`schemas/ui/m5-merge-readiness-panel.schema.json`](../../../schemas/ui/m5-merge-readiness-panel.schema.json)
- Canonical support export: [`artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth/support_export.json`](../../../artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth/support_export.json)
- Summary artifact: [`artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth.md`](../../../artifacts/review/m5/ship_merge_readiness_panels_merge_queue_entries_and_stack_dependency_chips_with_governance_and_queue_block_truth.md)
- Fixtures: [`fixtures/ui/m5-merge-readiness-panels/`](../../../fixtures/ui/m5-merge-readiness-panels/)
- Producer: `aureline_review::current_merge_readiness_panel_export`

## The panel contract

Each `panels[]` entry is a merge-readiness panel for one review. It answers, from
the panel alone, who owns the queue, how ready the change is, and — when it is not
ready — exactly why:

| Field | Meaning |
| --- | --- |
| `review_id_label`, `queue_owner_label` | Which review the panel is about and who owns the queue (the queue owner is never omitted). |
| `governance` | Who governs the queue state (see below). |
| `provider_freshness` | Provider-freshness state reused verbatim from the frozen matrix (`M5ReviewComponentStaleProviderState`). |
| `readiness_state` | The readiness verdict: ready, queued-waiting, or one of five blocked states. |
| `claims_authoritative` | Whether the panel presents an authoritative queue result; must match the derived authority. |
| `blocked_reason_detail` | Why the change is blocked; required and non-empty whenever the state is blocked. |
| `stale_base_note` | The stale-base label; required when blocked on a stale base. |
| `approval_recomputation_note` | The recomputation label; required when blocked on approval recomputation. |
| `auto_merge_scope` | The declared scope of any auto-merge/queue action. |
| `queue_entries[]` | The merge-queue entries in this panel's queue. |
| `stack_chips[]` | The stack-dependency chips shown on the panel. |

## Governance — the never-masquerade axis

`governance` is the core honesty axis. The three kinds are never allowed to
masquerade as one another:

- `provider_managed` — the code host's merge queue owns the authoritative state.
- `repo_policy_managed` — a repository merge policy Aureline applies owns the
  authoritative state.
- `aureline_local_estimate` — Aureline computed a local estimate only; the result is
  **not** authoritative.

The packet must cover all three across its panels, or it fails validation with
`governance_coverage_missing`, so all three are distinguishable in the same lane.

## Queue-result authority — readable without raw provider pages

`resolve_merge_readiness_disclosure(governance, provider_freshness, readiness_state)`
derives the `QueueResultAuthority` a reader sees, so a user can tell **authoritative**,
**estimated**, **stale**, or **blocked** apart without opening raw provider pages:

- `blocked` when the readiness state is blocked (takes precedence).
- else `stale` when provider freshness is `provider_stale`, `provider_unreachable`,
  `provider_conflict`, or `local_only_continuation`.
- else `estimated` when governance is `aureline_local_estimate`.
- else `authoritative`.

`claims_authoritative` must equal `authority == authoritative`. A local estimate that
claims to be authoritative **and** an authoritative result understated as an estimate
both fail with `authority_misrepresented` — **this is the never-masquerade device**,
catching both directions.

The resolver also drives the fallback and boundary requirements:

- `needs_blocked_reason` holds for any blocked state; a missing `blocked_reason_detail`
  fails with `blocked_reason_missing`.
- `needs_local_continue_fallback` holds when provider freshness is degraded; a missing
  `local_continue_fallback` fails with `local_continue_fallback_missing`.
- `needs_browser_handoff_boundary` holds for any `provider_unreachable` panel; a
  missing `browser_handoff_boundary` fails with `browser_handoff_boundary_missing`.

A `blocked_on_stale_base` panel with an empty `stale_base_note` fails with
`stale_base_note_missing`; a `blocked_on_approval_recomputation` panel with an empty
`approval_recomputation_note` fails with `approval_recomputation_note_missing` — so a
stale base and an approval recomputation are labeled, never hidden behind a generic
warning pill.

## Merge-queue entries and stack-dependency chips

Each `queue_entries[]` row carries its own `entry_state`; a blocked entry with an
empty `blocked_reason_detail` fails with `queue_entry_blocked_reason_missing`, so
queue/ready/block states stay stable and explainable in the detail pane too.

Each `stack_chips[]` chip carries a `relation`. A `stack_parent_blocked` chip with an
empty `blocking_note` fails with `stack_blocking_note_missing`, so a blocked stack
parent is always explained rather than silently blocking the change.

## Track invariant

The `trust_review` block encodes the hard invariants — all must hold for the packet
to validate: `provider_local_estimate_distinct`, `queue_owner_always_explicit`,
`blocked_reason_never_generic_warning`, `stale_base_labeled_not_hidden`,
`approval_recomputation_explicit`, `stack_blocking_explicit`,
`auto_merge_scope_explicit`, `provider_outage_preserves_local_continuation`,
`stale_sync_never_collapses_review_lane`,
`no_forced_raw_provider_navigation_for_triage`, `downgrade_narrows_instead_of_hides`,
and `stale_or_underqualified_blocks_promotion`.

One guardrail is enforced structurally beyond the trust bits: a panel whose actions
expose only `open_provider_in_browser` for triage fails with
`forced_raw_provider_navigation`; ordinary triage keeps an in-product action such as
`merge_now`, `restack_onto_base`, `recompute_approvals`, or `continue_local_review`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when proof
goes stale `auto_narrow_on_stale` narrows the lane. The supported downgrade triggers
are `proof_stale`, `policy_blocked`, `provider_freshness_stale`, `stale_base_unlabeled`,
`approval_recompute_pending`, `stack_parent_blocked`, `queue_ownership_unresolved`,
`browser_handoff_unavailable`, `trust_narrowing`, `scope_expansion_unqualified`, and
`upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/ui/m5-merge-readiness-panels/) show a stale
provider-backed queue preserving local continuation and a stack-blocked,
approval-recomputing lane whose blocking reasons stay explicit; both remain valid
because narrowing is explicit, not hidden.

## Boundary

Raw provider queue responses, credentials, and live provider payloads never cross
this boundary. The packet carries only metadata, freshness states, governance
distinctions, readiness verdicts, blocking reasons, and contract references.
