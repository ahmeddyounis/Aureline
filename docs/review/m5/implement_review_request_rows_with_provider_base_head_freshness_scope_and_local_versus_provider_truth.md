# Review-Request Rows: Provider/Base-Head/Branch Freshness, Scope, and Local-versus-Provider Truth

This document is the contract for the M5 packet that implements the reusable
review-request row. It narrows the `review_request_row` component frozen in the
[review-request/checks/merge-queue component matrix](../../../schemas/ui/m5-review-request-check-queue-component-matrix.schema.json)
into an implemented row contract. The packet is the canonical M5 control source
for this lane: review lists, inboxes, switchers, companion queues, handoff
packets, CLI/headless output, diagnostics, Help/About, and support exports ingest
the checked-in packet rather than cloning row text or provider-specific badges.

- Record kind: `review_request_row_local_versus_provider_truth`
- Schema: [`schemas/ui/m5-review-request-row.schema.json`](../../../schemas/ui/m5-review-request-row.schema.json)
- Canonical support export: [`artifacts/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth/support_export.json`](../../../artifacts/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth/support_export.json)
- Summary artifact: [`artifacts/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth.md`](../../../artifacts/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth.md)
- Fixtures: [`fixtures/ui/m5-review-request-rows/`](../../../fixtures/ui/m5-review-request-rows/)
- Producer: `aureline_review::current_review_request_row_export`

## The row contract

Each `rows[]` entry answers, from the row alone, what the object is, which
provider or local object owns it, what scope it covers, and how fresh that truth
is:

| Field | Meaning |
| --- | --- |
| `backing_kind` | The local-versus-provider distinction (see below). |
| `provider_identity_label`, `object_id_label` | Who owns the object and its id (PR/MR number or local bundle id). |
| `base_ref_label`, `head_ref_label`, `base_head_freshness` | Base/head or branch refs and their freshness (`current`, `stale_base`, `outdated_head`, `diverged`, `unknown`). |
| `stack_relation` | Stack position, including `stack_member_parent_blocked` when a parent blocks this row. |
| `scope` | What the row covers (`full_request`, `stack_segment`, `single_commit`, `partial_selection`). |
| `provider_freshness` | Provider-freshness state reused verbatim from the frozen matrix (`M5ReviewComponentStaleProviderState`). |
| `actions` | Direct open/export actions. |

The row carries source-contract references to the
[review-workspace](../../../schemas/review/review_workspace.schema.json),
[merge-queue entry](../../../schemas/review/merge_queue_entry.schema.json), and
[change-lineage](../../../schemas/review/change_lineage.schema.json) contracts by
id rather than embedding their content.

## Backing kind — local versus provider

`backing_kind` is the honesty axis. A reader can tell the four kinds apart from
the row alone:

- `local_review_estimate` — a local-only bundle; no hosted request exists yet.
- `provider_backed_request` — a real hosted pull/merge request.
- `offline_exported_packet` — cached or exported context, not live hosted truth.
- `browser_handoff_placeholder` — a placeholder that must hand off to the browser
  rather than claim hosted status.

The disclosures a row must carry are **derived**, never asserted directly, by
`resolve_review_request_row_disclosure(backing_kind, provider_freshness)`:

- `asserts_hosted_status` follows the backing kind alone — only a
  `provider_backed_request` may set `claims_provider_backed: true`. Any other kind
  claiming hosted status, or a provider-backed row dropping it, fails validation
  with `hosted_status_misrepresented`.
- `needs_local_continue_fallback` holds for local estimates, offline packets, and
  any row whose provider freshness is `provider_stale`, `provider_unreachable`,
  `provider_conflict`, or `local_only_continuation`. A missing
  `local_continue_fallback` fails with `local_continue_fallback_missing`.
- `needs_browser_handoff_boundary` holds for placeholders and any
  `provider_unreachable` row. A missing `browser_handoff_boundary` fails with
  `browser_handoff_boundary_missing`.

A degraded provider is therefore never flattened into a local estimate: a stale or
unreachable provider-backed row keeps `claims_provider_backed: true` while adding
the local-continue path.

## Track invariant

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: `provider_local_estimate_distinct`,
`local_estimate_never_claims_hosted`, `offline_exported_packet_distinct`,
`provider_freshness_explicit`, `base_head_relation_explicit`,
`stack_relation_explicit`, `browser_handoff_explicit`,
`local_continue_preserved_on_degraded_freshness`,
`no_forced_raw_provider_navigation_for_triage`,
`one_row_contract_no_hidden_provider_meaning`,
`downgrade_narrows_instead_of_hides`, and
`stale_or_underqualified_blocks_promotion`.

Two guardrails are enforced structurally beyond the trust bits:

- **No forced raw-provider navigation.** A `provider_backed_request` row whose
  `actions` contain only `open_provider_in_browser` fails with
  `forced_raw_provider_navigation`; ordinary triage keeps an in-product action.
- **Backing-kind coverage.** A packet must include at least one
  `local_review_estimate`, one `provider_backed_request`, and one
  `offline_exported_packet` row so all three are distinguishable in the same list;
  otherwise it fails with `backing_kind_coverage_missing`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the lane. The supported downgrade
triggers are `proof_stale`, `policy_blocked`, `provider_freshness_stale`,
`approval_invalidated`, `stack_parent_blocked`, `browser_handoff_unavailable`,
`trust_narrowing`, `scope_expansion_unqualified`, and
`upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/ui/m5-review-request-rows/) show a stale provider-backed
row preserving local continuation and a browser-handoff placeholder that never
claims hosted status; both remain valid because narrowing is explicit, not hidden.

## Boundary

Raw diff bodies, raw check logs, raw provider payloads, credentials, and live
provider responses never cross this boundary. The packet carries only metadata,
freshness states, backing-kind distinctions, and contract references.
