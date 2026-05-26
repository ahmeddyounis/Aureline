# Quick-open, file, symbol, and command-palette latency truth — stable contract

Status: Stable lane proof for certified workspace archetypes.

This document is the reviewer-facing contract for the stable quick-open,
file-search, symbol-search, and command-palette latency packet. The packet
is the single source of truth that the search shell, docs/help, CLI/headless
inspector, support export, and release proof index all read; surfaces MUST
NOT mint local copies or paraphrase status text.

## What the packet asserts

For each governed *certified archetype × surface* row, the packet asserts:

1. A **published p50/p95 budget** in milliseconds, with an optional waiver
   ref when the budget is intentionally narrowed or tightened.
2. An **observed latency capture** with a benchmark-lab capture ref and
   sample size.
3. The **partial-index truth class** for the row (`fully_indexed`,
   `hot_set_only`, `partial_index`, `stale_shard`, `index_unavailable`),
   plus a disclosure ref whenever the class is not `fully_indexed`.
4. The **session readiness states** the row keeps visible (`ready`,
   `hot_set_ready`, `warming`, `partial`, `blocked`, `withheld`,
   `provider_limited`, `policy_limited`). Stable rows MUST NOT collapse
   non-ready states into a generic spinner or success badge.
5. The **readiness transitions** that fired during the captured session,
   ordered by elapsed milliseconds, and whether each transition emitted the
   first useful row to the user.
6. The **durable query-session id** that identifies this session across row
   virtualization, preview-pane open/close, and ranking-refinement passes —
   so support export, AI context packets, CLI output, and replay fixtures
   all reference the same search event.
7. The **scope/workset ref** and **planner version** active when the
   session opened.

## Closed vocabulary

**Certified archetypes** — `typescript_javascript_web`,
`python_service_or_data_app`, `rust_workspace`,
`go_service_or_monorepo_slice`, `java_or_kotlin_service`,
`c_or_cpp_native_project`.

**Latency surfaces** — `quick_open`, `file_search`, `symbol_search`,
`command_palette`.

**Required consumer projections** — `search_shell`, `docs_help`,
`cli_headless`, `support_export`, `release_proof_index`. Each projection
MUST preserve the same packet id, query-session ids, readiness states, and
partial-index labels; MUST support JSON export; and MUST exclude raw
private material and ambient authority.

## Promotion states

A materialized packet is one of:

- `stable` — every row hits its budget, declares a disclosure when
  required, keeps readiness states visible, and every required projection
  preserves the packet verbatim.
- `narrowed_below_stable` — a warning-class finding is present (for
  example, a budget waiver is exercised and the row is intentionally
  narrowed below stable until the waiver clears).
- `blocks_stable` — a blocker finding is present (for example, an observed
  budget breach without a waiver, an unlabeled partial-index downgrade, a
  dropped consumer projection, or raw boundary material in the packet).

## Why this matters

The track invariant for this lane is *keep search, graph, and docs surfaces
useful before fully warm and explicit about scope, freshness, provenance,
and downgrade state at all times*. The packet's validation rules implement
that invariant directly: a stable row cannot ship with unlabeled
partial-index state, with a collapsed readiness vocabulary on any consumer
surface, with a query-session id that drifts across projections, or with
raw private material on the boundary. When delivery proves a narrower
stable claim, the packet narrows below stable rather than papering over the
gap.

## Where the packet lives

- Schema: `schemas/search/quick_open_latency_truth.schema.json`
- Reviewer artifact: `artifacts/search/m4/finalize-quick-open-file-symbol-command-search-latency.md`
- Stable packet artifact: `artifacts/search/m4/quick_open_latency_truth_packet.json`
- Fixture corpus: `fixtures/search/m4/quick_open_latency_truth/`
- Rust module: `crates/aureline-search/src/quick_open_latency_truth/mod.rs`

### Anchors

#### hot-set

The `hot_set_only` partial-index class disclosure: cold paths are still
warming while the hot-set lane answers; the row keeps `hot_set_ready`
visible until the cold lane promotes to `ready`.

#### partial-index

The `partial_index` and `stale_shard` partial-index class disclosure:
declared-scope coverage is incomplete or stale; the row keeps `partial`
visible and narrows blast-radius-large actions until the gap closes.
