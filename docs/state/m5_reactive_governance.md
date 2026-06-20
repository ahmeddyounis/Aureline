# M5 reactive-state, subscription, and materialized-view governance matrix

The canonical M5 reactive-governance matrix is implemented in
[`crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs`](../../crates/aureline-reactive-state/src/m5_reactive_governance/mod.rs)
and serialized to
[`artifacts/state/m5_reactive_governance.json`](../../artifacts/state/m5_reactive_governance.json).

It is the single checked-in truth source for how every reactive M5
surface subscribes to authoritative state, which materialized-view
class it is, and how a presented claim is narrowed when the surface
outruns its authoritative epoch, scope, or invalidation guarantee.
Later product, help, support, and release surfaces ingest this matrix
instead of inventing surface-local stale-state wording.

The matrix is grounded in Appendix DB of the technical architecture
document (`DB.1` authority classes, `DB.2` subscription envelope,
`DB.3` materialized-view and invalidation rules) and mirrors the
subscription-envelope vocabulary frozen in
[`crates/aureline-reactive-state/src/envelope.rs`](../../crates/aureline-reactive-state/src/envelope.rs)
token for token.

## What the matrix freezes

- **Authority classes** — `workspace_vfs`, `buffer_editor`,
  `derived_knowledge`, `execution`, `policy_entitlement`,
  `provider_overlay` (Appendix DB.1). Every surface names the
  authority that owns its canonical truth.
- **Subscription envelope** — every surface carries a `query_family`,
  a `scope_class`, a snapshot epoch + delta sequence, freshness and
  completeness metadata, and a backpressure mode (Appendix DB.2,
  modeled by [`envelope::SubscriptionEnvelope`]).
- **Freshness / completeness / backpressure vocabulary** —
  `authoritative / warming / cached / stale / replayed / imported`,
  `full / partial / unloaded / unavailable`, and
  `realtime / coalesced / snapshot_required`.
- **Invalidation reasons** — the twelve frozen stale reasons
  (`producer_restart` … `causality_lost`).
- **Materialized-view classes** — `ephemeral_projection`,
  `durable_local_materialization`, `exportable_snapshot`,
  `managed_replicated_view`; each declares persistence, read
  authority, and delete semantics (Appendix DB.3).
- **Truth claims and automatic narrowing** — the eleven-value
  `truth_claim` vocabulary plus the canonical narrowing engine that
  downgrades a claim from any observed subscription state.

## Truth-claim narrowing

A reactive M5 surface is a **derived projection** of an authority. The
strongest claim a derived surface may present is `consistent_snapshot`
— never `exact_current_truth`, which is reserved for the owning
authority. From that ceiling, every observed degradation contributes a
candidate claim, and the **narrowest (lowest-confidence) candidate
wins**:

| Strength | Truth claim | Triggered by |
| --- | --- | --- |
| 0 | `provider_unavailable` | completeness `unavailable`, any terminal frame |
| 1 | `policy_limited_projection` | policy / entitlement limit |
| 2 | `imported_snapshot` | freshness `imported` |
| 3 | `replayed_snapshot` | freshness `replayed` |
| 4 | `stale_snapshot` | freshness `stale` |
| 5 | `cached_projection` | freshness `cached` |
| 6 | `warming_no_truth_yet` | freshness `warming`, completeness `unloaded` |
| 7 | `partial_projection` | completeness `partial` |
| 8 | `coalesced_stream` | backpressure `coalesced` / `snapshot_required` |
| 9 | `consistent_snapshot` | healthy derived ceiling |
| 10 | `exact_current_truth` | authority only — never a view |

`narrow_truth_claim(derivation, observed)` is the one function the
matrix rows, the real shell explainer, the support export, and
release tooling share. Each surface row's `claim_narrowing_rules` are
computed from this engine over the row's supported state sets, so the
declared rules can never drift from the engine.

## Surface matrix

| Surface | Authority | View class | Channels | Healthy claim |
| --- | --- | --- | --- | --- |
| `shell_workspace_tree` | `workspace_vfs` | `durable_local_materialization` | ui, cli_headless | `consistent_snapshot` |
| `shell_activity_center` | `execution` | `durable_local_materialization` | ui, cli_headless | `consistent_snapshot` |
| `editor_buffer_outline` | `buffer_editor` | `ephemeral_projection` | ui | `consistent_snapshot` |
| `search_results` | `derived_knowledge` | `durable_local_materialization` | ui, cli_headless | `consistent_snapshot` |
| `graph_neighborhood` | `derived_knowledge` | `ephemeral_projection` | ui | `consistent_snapshot` |
| `docs_browser` | `derived_knowledge` | `durable_local_materialization` | ui, cli_headless | `consistent_snapshot` |
| `ai_context_panel` | `derived_knowledge` | `ephemeral_projection` | ui | `consistent_snapshot` |
| `review_workspace` | `provider_overlay` | `managed_replicated_view` | ui, export | `consistent_snapshot` |
| `preview_output` | `execution` | `exportable_snapshot` | ui, export | `consistent_snapshot` |
| `companion_panel` | `provider_overlay` | `managed_replicated_view` | ui | `consistent_snapshot` |
| `policy_trust_banner` | `policy_entitlement` | `ephemeral_projection` | ui, cli_headless | `consistent_snapshot` |
| `headless_workspace_mirror` | `workspace_vfs` | `ephemeral_projection` | cli_headless, release | `consistent_snapshot` |
| `support_export_view` | `derived_knowledge` | `exportable_snapshot` | export, release | `consistent_snapshot` |

The matrix covers all six authority classes, all four view classes,
and all four presentation channels (UI, CLI/headless, export, and
release), so the same stale-state grammar reaches every channel.

## Cross-surface epoch parity

Surfaces sharing an authority class form one epoch-parity group and
read the same authoritative snapshot epoch. A member that lags its
authority epoch narrows its claim rather than presenting a parallel
epoch as current truth.

- `workspace_vfs` — `shell_workspace_tree`, `headless_workspace_mirror`
- `buffer_editor` — `editor_buffer_outline`
- `derived_knowledge` — `search_results`, `graph_neighborhood`, `docs_browser`, `ai_context_panel`, `support_export_view`
- `execution` — `shell_activity_center`, `preview_output`
- `policy_entitlement` — `policy_trust_banner`
- `provider_overlay` — `review_workspace`, `companion_panel`

## Materialized-view lifecycle

Persistence and delete semantics are computed from the view class per
Appendix DB.3, so a declaration cannot blur its lifecycle:

| View class | Persistence | Delete semantics | Rebuildable from authority |
| --- | --- | --- | --- |
| `ephemeral_projection` | `memory_only` | `evict_on_scope_change` | yes |
| `durable_local_materialization` | `local_cache_or_db` | `clear_or_rebuild` | yes |
| `exportable_snapshot` | `saved_artifact` | `replaced_by_new_snapshot` | no (never updated in place) |
| `managed_replicated_view` | `service_or_local_mirror` | `reconcile_on_reconnect` | yes |

## Fixtures

The fixture corpus under
[`fixtures/state/m5_reactive_governance/`](../../fixtures/state/m5_reactive_governance/)
pins one observed subscription state per interesting narrowing so
release and support tooling can prove the downgrade behavior:

| Fixture | Surface | Observed | Narrowed claim |
| --- | --- | --- | --- |
| `shell_warming` | `shell_workspace_tree` | warming, unloaded | `warming_no_truth_yet` |
| `shell_healthy` | `shell_workspace_tree` | healthy | `consistent_snapshot` |
| `search_stale` | `search_results` | stale | `stale_snapshot` |
| `docs_cached` | `docs_browser` | cached | `cached_projection` |
| `graph_partial` | `graph_neighborhood` | partial | `partial_projection` |
| `ai_policy_limited` | `ai_context_panel` | policy-limited | `policy_limited_projection` |
| `review_coalesced` | `review_workspace` | coalesced | `coalesced_stream` |
| `companion_unavailable` | `companion_panel` | unavailable + terminal | `provider_unavailable` |
| `preview_replayed` | `preview_output` | replayed, partial | `replayed_snapshot` |
| `support_imported` | `support_export_view` | imported, partial | `imported_snapshot` |

## Consumers

The matrix is consumed by real surfaces rather than re-described:

- shell state explainability —
  [`crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs`](../../crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs)
  renders, per surface, where truth came from, how fresh it is,
  whether scope is partial, and what invalidation can change it;
- metadata-safe support export —
  [`crates/aureline-support/src/m5_reactive_governance/mod.rs`](../../crates/aureline-support/src/m5_reactive_governance/mod.rs)
  folds the matrix into a support-export envelope that quotes the same
  narrowing for release and procurement readers;
- fixture replay —
  [`crates/aureline-reactive-state/tests/m5_reactive_governance.rs`](../../crates/aureline-reactive-state/tests/m5_reactive_governance.rs).

## Guardrails

- No derived M5 surface may present `exact_current_truth`; the
  validator rejects any row whose healthy claim is exact truth.
- Surfaces must list only degraded states in their supported sets;
  `authoritative` / `full` / `realtime` are implicit and the narrowing
  ceiling.
- A surface that observes a stale snapshot, a coalesced delta stream,
  a partial scope, or a policy-limited projection narrows its claim and
  never implies exact current truth.
