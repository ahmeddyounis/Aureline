# Target-Discovery Confidence and Explainability (Beta)

This lane promotes the alpha
[`target_confidence_card_record`](../target_confidence_alpha.md) into one beta
row that names where a target came from (native protocol, structured adapter,
heuristic parser, imported metadata, user declaration, or resolver
unavailable), how current that discovery is, which actions the target
supports, and a typed decision for every protected action. The promise: a
heuristic or imported row cannot masquerade as exact runnable truth on
protected build / run / test / debug actions, and the same explanation
visible in the run / test / debug surfaces is preserved in support exports.

The implementation lives in
[`/crates/aureline-runtime/src/target_discovery/`](../../../crates/aureline-runtime/src/target_discovery/)
and the cross-tool boundary at
[`/schemas/runtime/target_discovery_beta.schema.json`](../../../schemas/runtime/target_discovery_beta.schema.json).

## Why a beta layer

The alpha `target_confidence_card_record` collapsed two orthogonal axes into a
single `discovery_confidence_token`:

- **where the target came from** (native protocol vs structured adapter vs
  heuristic / fallback parser vs imported / out-of-band metadata vs user
  declaration);
- **how confident the resolver is in the current binding** (exact, probed,
  inferred, divergent, stale).

A target produced by a heuristic regex is structurally different from a target
produced by a typed manifest reader even when the resolver reports the same
confidence level. The beta layer splits the axes so launch surfaces can refuse
protected dispatch on a row whose target was guessed by a heuristic parser
without re-deriving the rule locally.

## Closed vocabularies

| Axis | Tokens |
| --- | --- |
| `discovery_source` | `native_protocol`, `structured_adapter`, `heuristic_parser`, `imported_metadata`, `user_declared`, `resolver_unavailable` |
| `discovery_freshness` | `fresh_probe`, `recent_within_session`, `imported_authoritative`, `stale_imported`, `unknown` |
| `supported_capability` | `run`, `test`, `debug_launch`, `debug_attach`, `build`, `inspect_only` |
| `protected_action` | `dispatch_run`, `dispatch_test`, `dispatch_debug_launch`, `dispatch_debug_attach`, `dispatch_build`, `export_artifact` |
| `protected_action_decision` | `allowed`, `requires_review`, `blocked_heuristic_target`, `blocked_imported_target`, `blocked_unsupported_capability`, `blocked_resolver_unavailable`, `blocked_freshness_stale` |

Adding a token in any axis is additive-minor and bumps
`target_discovery_beta_schema_version`. Repurposing an existing token is
breaking and requires a vocabulary RFC.

## Protected-action gating

Every beta row carries one decision per protected-action class. The closed
decision rules are:

1. `export_artifact` is always `allowed` — every typed row remains exportable
   as evidence even when dispatch is blocked.
2. `resolver_unavailable` rows → `blocked_resolver_unavailable` on every
   non-export action.
3. `heuristic_parser` rows → `blocked_heuristic_target` on every non-export
   action.
4. If the target does not advertise the action's required capability →
   `blocked_unsupported_capability`.
5. `imported_metadata` rows → `blocked_imported_target`.
6. `stale_imported` or `unknown` freshness → `blocked_freshness_stale`.
7. Helper-backed lane, non-`high` resolver confidence, or any divergence /
   inference reason → `requires_review`.
8. Otherwise → `allowed`.

This ordering is intentional: a structural mismatch (capability missing) is
reported before a refreshable concern (freshness), and source classes that can
never dispatch protected work are reported before remediable ones.

## Consumer surfaces

The first consumer is a deterministic shell projection:
[`/crates/aureline-shell/src/target_discovery_beta/`](../../../crates/aureline-shell/src/target_discovery_beta/).
Run, test, build, and debug pickers read the same projection so the same
discovery explanation visible in the chrome is the one that lands in support
exports.

`TargetDiscoveryBetaSupportExport` bundles:

- the beta coverage manifest pinning the closed vocabulary at export time;
- the beta projection (one row per execution context);
- the underlying alpha `TargetConfidenceCard` records (for cross-reference);
- alpha host-boundary rows and review rows;
- redaction-safe execution-context provenance plus support-export provenance
  events.

The boundary keeps raw paths, command lines, environment bodies, and secrets
out of the export.

## Fixtures

Protected fixtures live under
[`/fixtures/runtime/m3/target_confidence/`](../../../fixtures/runtime/m3/target_confidence/):

- `native_local_and_heuristic_remote.json` proves one local native row and
  one heuristic-discovered helper row preserve discovery source, freshness,
  capabilities, and gating through projection and support export.

## Verify

```sh
cargo test -p aureline-runtime target_discovery
cargo test -p aureline-runtime --test target_discovery_beta
cargo test -p aureline-shell target_discovery_beta
```
