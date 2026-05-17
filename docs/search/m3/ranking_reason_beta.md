# Ranking Reason And Operator Truth Beta

This contract defines the search-owned packet that beta graph, AI, review,
CLI/headless, and support surfaces consume when they need to explain a ranked
row.

Implementation:
[`crates/aureline-search/src/ranking_reason/`](../../../crates/aureline-search/src/ranking_reason/).
Boundary schema:
[`schemas/search/search_operator_truth_packet.schema.json`](../../../schemas/search/search_operator_truth_packet.schema.json).
Protected fixture:
[`fixtures/search/m3/operator_truth_packets/`](../../../fixtures/search/m3/operator_truth_packets/).
Checked-in packet:
[`artifacts/search/m3/operator_truth_packets/search_operator_truth_beta_packet.json`](../../../artifacts/search/m3/operator_truth_packets/search_operator_truth_beta_packet.json).

## Product Contract

Every claimed search or graph-backed row that feeds AI context or review must
preserve one `search_operator_truth_beta_packet`. The packet carries:

- ordered ranking-reason tokens used by `Why this result?`;
- visible readiness and freshness labels;
- source packet refs for retrieval, graph cues, and drill evidence;
- partial-index drill rows and downgrade states;
- blocked actions for partial, stale, hidden, warming, or failing rows; and
- consumer projections proving search, graph overlay, AI context, review
  workspace, CLI/headless, and support export read the same packet.

The packet is metadata-only. It does not contain raw queries, raw source bodies,
raw provider payloads, credentials, ambient authority, or private rank weights.

## Partial-Index Drill

`partial_index_drill_beta_packet` is reusable evidence. Each drill row names the
affected row, observed state, reason tokens, downgrade state, and blocked
actions. Non-current drill states must downgrade affected rows:

| State | Required result |
| --- | --- |
| `partial_index` | row carries `yellow_partial_index` and blocks broad rename / cross-root apply |
| `stale_shard` | row carries `yellow_stale_shard` and blocks broad rename / cross-root apply |
| `hidden_scope` | row carries `yellow_hidden_scope` and preserves hidden-scope truth |
| `failing` | row carries `red_blocks_beta_promotion` |
| `warming` | row carries a partial downgrade until the lane is ready |

A drill that is not reusable, includes raw private material, or leaves a
degraded row without a downgrade blocks the packet.

## Consumer Parity

Search owns the packet. AI and review consumers wrap the exact search packet and
validate that the relevant projection is present. They must not reconstruct
ranking reasons or readiness labels from local UI rows.

Required preserved projections:

- `search_results`
- `graph_overlay`
- `ai_context`
- `review_workspace`
- `support_export`

`cli_headless` is included in the beta fixture so support and operator audits
can inspect the packet without desktop-only debug tooling.

## Promotion Behavior

The validator derives `promotion_state` from findings and row downgrades:

- `promotable` means no blockers or warning downgrades exist.
- `needs_review` means the packet is valid but at least one row is narrowed by
  current operator truth.
- `blocked` means the packet has a blocker finding or a row declares a red
  promotion block.

The checked-in packet intentionally includes a partial graph row and a stale
graph shard row. Both rows are downgraded and export-safe, so the packet
validates with no findings while remaining `needs_review`.
