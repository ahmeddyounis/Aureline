# Truth-class, degraded-state, and claim-propagation vocabulary

This document is the authoritative narrative for Aureline's shared
truth-class and degraded-state vocabulary. It exists so that product
surfaces, docs, support exports, release packets, claim manifests, and
public-proof packets converge on **one cross-surface truth model**
rather than inventing per-surface dialects for how durable, derived,
cached, partial, or inferred state is described.

Machine-readable companions:

- [`/artifacts/governance/truth_class_matrix.yaml`](../../artifacts/governance/truth_class_matrix.yaml)
  — matrix of truth classes, examples, labeling rules, and allowed
  projections.
- [`/artifacts/governance/claim_propagation_rules.yaml`](../../artifacts/governance/claim_propagation_rules.yaml)
  — propagation rules binding degraded-state tokens to required copy
  fields, required route channels, and the worst-supporting-truth-wins
  composition rule.

Related upstream contracts (already seeded and this vocabulary does not
replace any of them):

- [`/docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  and
  [`/schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)
  — frozen source-class, version-match, freshness, client-scope,
  service-contract, and degraded-state-cause vocabularies for
  docs/Help/About/service-health. This document re-uses those tokens
  rather than minting parallel ones.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — canonical `lifecycle_state`, `support_class`, `freshness_class`, and
  `client_scope` enums.
- [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  and
  [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — claim-row publication contract and seeded downgrade triggers.
- [`/docs/governance/public_surface_truth_map.md`](./public_surface_truth_map.md)
  and
  [`/artifacts/governance/source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml)
  — canonical owner map for public/support-facing truth rows.
- [`/docs/governance/drift_blocking_rules.md`](./drift_blocking_rules.md)
  — severity classes and same-change-set rules for truth drift.
- [`/docs/search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)
  and
  [`/artifacts/search/result_truth_labels.yaml`](../../artifacts/search/result_truth_labels.yaml)
  — search-readiness vocabulary, whose `heuristic`, `hybrid`, and
  `partial_index` tokens are the search-side expression of the derived
  and partial classes seeded here.

## Purpose

A surface should be able to answer two questions mechanically, without
relying on human-written copy:

1. **What class of truth is this row?** Is it durable user-authored
   state, workspace/VCS state, a runtime observation, a derived index,
   session/collaboration state, or an AI inference?
2. **Is the row currently degraded, and if so, how?** Is it warming,
   cached, partial, stale, offline, policy-blocked, limited,
   unsupported, experimental, or awaiting re-test?

Every publication surface (Help/About, service-health, Start Center,
docs, support exports, claim manifests, release evidence) must resolve
both questions using the tokens in this document. Generic copy such as
"error", "unavailable", "temporarily unavailable", "loading", or "try
again" is **forbidden on protected paths** when a more precise state is
known.

## Truth classes

The truth class answers *what kind of statement is this row*. Every
class names: the durability guarantee, the canonical owner family, the
allowed degraded states, the allowed projections, and the required
labeling discipline.

| Truth class | Durability | Canonical owner family | Typical rows |
|---|---|---|---|
| `user_authored_durable_truth` | Durable on local disk (plus any opted-in managed copy); authoritative by definition. | user workspace, settings, notes, drafts | buffer contents, settings changes, notes, unsaved edits that persist across restarts |
| `workspace_vcs_truth` | Durable and version-controlled in a VCS the user owns. | working tree, VCS index, VCS refs | working-tree files, staged changes, branches, commits, submodules |
| `runtime_observed_truth` | Point-in-time observation of a process, socket, filesystem, or external system. Not durable. | runtime probes, observers, diagnostics | process status, port state, docker/container state, filesystem mount state, network reachability |
| `derived_indexed_truth` | Function of durable truth plus indexer state. Reproducible; must be labeled as derived. | index, cache, generated reference | symbol index, search index, docs pack index, lockfile-derived graph, generated API reference |
| `session_collaboration_truth` | Ephemeral state scoped to a session, window, tab, or co-edit channel. | session store, presence service, co-edit channel | focus, cursor, selection, presence, breadcrumb trail, unshared local view state |
| `ai_inferred_truth` | Inferred by a model from inputs; provisional by nature; requires citation when a claim is public. | AI explanation overlay, assistant answers, rank-based suggestions | AI explanation overlay content, assistant summaries, suggested edits, risk estimates |

### Labeling rules (MUST)

1. A surface **MUST** render the truth class of every row it shows,
   either as a typed badge, a field in the underlying record, or a
   known projection defined in the truth-class matrix. Mixing two truth
   classes into a single compound ("live state") is forbidden.
2. `user_authored_durable_truth` **MUST NOT** be rendered with
   `warming`, `stale`, `cached`, or `offline` degraded tokens; those
   tokens describe derived or observed state. A durable user row is
   either present or not present.
3. `derived_indexed_truth` and `runtime_observed_truth` **MUST** carry
   a freshness token (`authoritative_live`, `warm_cached`,
   `degraded_cached`, `stale`, `unverified`) and, when below
   `authoritative_live`, a typed `degraded_state_cause`.
4. `ai_inferred_truth` **MUST** cite at least one authoritative anchor
   (a row from one of the other truth classes or a published claim
   row). AI answers are never claim-bearing on their own.
5. `session_collaboration_truth` **MUST NOT** be promoted into a claim
   row or a support-export row without first being projected through a
   durable or derived owner.
6. Export and support surfaces **MUST** preserve the originating truth
   class of every row; narrowing the class (for example, exporting a
   derived summary as if it were durable) is forbidden.

## Degraded-state vocabulary

The degraded-state token answers *how is this row currently weaker than
its ideal posture*. Ten tokens are frozen below. Every token names its
required copy intent, its mandatory recovery expectation, and its
cross-surface compatibility with upstream freshness/lifecycle/service
vocabularies. Surfaces **MUST** pick exactly one token when a row is
degraded; composition is not allowed (see "Worst-supporting-truth-wins"
below).

| Token | Meaning | Required copy intent | Recovery expectation |
|---|---|---|---|
| `Warming` | Owner is not yet ready; this is the first-reach-for state, not a failure. | Name what is warming and the expected ready signal. | A known completion path; show progress where available. |
| `Cached` | A prior successful reach is being shown while the owner is unreachable or not yet re-contacted. | Name the age class (warm_cached vs degraded_cached). | Refresh hook or route to repair. |
| `Partial` | Only part of the requested scope is present; the rest is missing or not yet computed. | Name the covered scope and the excluded scope. | Expand-scope or finish-index hook. |
| `Stale` | A prior value is being shown past its freshness floor. | Name the staleness cause and last-known-good time. | Refresh, resync, or route to repair. |
| `Offline` | Owner is unreachable and the surface has fallen back to local-only truth. | Name the unreachable owner and local-only scope. | Retry and route to service-health. |
| `PolicyBlocked` | Row is withheld by admin policy, kill switch, or deployment-profile rule. | Name the policy or kill-switch source. | Policy explainer or admin-escalation route. |
| `Limited` | Row is narrower than the declared claim for a non-policy reason (evidence narrower, scope reduced, feature gated). | Name the narrowing reason. | Route to the known-limit note and/or repair hook. |
| `Unsupported` | Row applies to a client-scope, platform, or deployment profile that excludes the rendering surface. | Name the excluding scope. | Redirect to a supported surface or document the exclusion. |
| `Experimental` | Row is behind a preview/labs posture and may narrow further. | Name the preview posture. | Experiment register row or feature-flag policy. |
| `RetestPending` | Prior evidence was invalidated (rebuild, re-index, policy change); row is awaiting re-verification. | Name the retrigger and what will refresh. | Re-test job, re-index trigger, or re-verification gate. |

### Composition with upstream vocabularies

Each token composes with already-frozen vocabularies rather than
replacing them:

- `Warming`, `Cached`, `Stale`, `Offline`, `RetestPending` are
  expressions of the `freshness_class` enum
  (`authoritative_live`, `warm_cached`, `degraded_cached`, `stale`,
  `unverified`) from
  `schemas/governance/capability_lifecycle.schema.json` — this document
  does not mint a parallel freshness scale.
- `PolicyBlocked` composes with `disabled_by_policy` (capability
  lifecycle) and the `policy_blocked` service-contract state from the
  docs/help/service-health truth schema.
- `Unsupported` composes with `client_scope` (capability lifecycle) and
  `client_scope_excludes_surface` (docs/help degraded-state cause).
- `Experimental` composes with `lifecycle_state` values `labs` and
  `preview` and with the `experimental` claim posture.
- `Limited` composes with the `limited` claim posture and with
  `required_evidence_narrower_than_claim` / `known_limit_missing` /
  `compatibility_row_degraded` downgrade triggers.
- `Partial` composes with the search `partial_index` and
  `stale_index_served` readiness states and with exports that carry a
  partial-result disclosure.

## Claim propagation rules

Publication channels treated as projections (not sources) of the same
row:

- **Help/About** — version-match, build-channel, and capability rows
  render truth class, freshness, and degraded-state tokens directly; a
  generic "available" chip is forbidden.
- **Service health** — emits only
  `schemas/docs/help_status_badge.schema.json#service_contract_state`
  tokens. A per-feature "available"/"having trouble" chip is
  forbidden. Each row names the degraded-state token and the
  `degraded_state_cause`.
- **Start Center** — first-run, reopen-with-pending-restore, warming,
  partial, offline, and unsupported startup rows must name the
  degraded-state token from the copy-review matrix (see
  `artifacts/ux/startup_state_copy_review.yaml`), and they must not
  overclaim readiness.
- **Docs** — pack cards, version-match badges, and freshness badges
  render `source_class`, `version_match_state`, and `freshness_class`
  from the existing badge vocabulary; they may not invent a parallel
  "current" / "outdated" scale.
- **Support exports** — every row preserves the originating truth class
  and the currently-applicable degraded-state tokens. Downgrades
  propagate; widening is forbidden.
- **Claim manifest** — `effective_claim_posture` mirrors the
  degraded-state token set: `experimental`, `limited`,
  `policy_disabled`, and `replacement_grade` postures map to
  `Experimental`, `Limited`, `PolicyBlocked`, and (respectively) a
  replacement-routed row.
- **Release evidence** — release packets surface degraded tokens on any
  claim-bearing row whose evidence is `Stale`, `Partial`, `Cached`,
  `RetestPending`, or narrower than the claim. Packets fail closed when
  the row-level token is broader than the underlying evidence.

### Worst-supporting-truth-wins rule

When a row's public posture depends on multiple evidence sources (for
example, a Help/About capability card depends on: a capability
lifecycle row, the docs-pack version-match, a service-health ping, a
claim row, and a compatibility row), the rendered degraded-state token
is the **worst** of the contributing tokens under this ordering
(rightmost is strongest; surfaces MUST render the leftmost that applies
— lower-strength tokens lose to higher-strength ones):

```
authoritative_live
  >  Warming
  >  Cached
  >  RetestPending
  >  Partial
  >  Stale
  >  Offline
  >  Experimental
  >  Limited
  >  Unsupported
  >  PolicyBlocked
```

- A row that is `Cached` upstream but `Offline` downstream renders
  `Offline`.
- A row that is `Experimental` (upstream) and `PolicyBlocked`
  (downstream) renders `PolicyBlocked`.
- A row that would be `authoritative_live` but whose claim evidence is
  `narrower_than_claim` renders `Limited`.

This rule is mandatory for Help/About, service-health, Start Center,
docs, support exports, claim manifests, and release evidence. Other
surfaces may be narrower than the composed worst-supporting truth; they
may never be broader.

### Forbidden generic copy

On any protected path (Help/About, service-health, docs-pack header,
Start Center entry/restore row, support-export row, claim-row copy,
release-evidence row), the following generic copy is **forbidden** when
a more precise state is known:

- "error"
- "failure" / "failed" (without naming the failure class)
- "unavailable" / "temporarily unavailable"
- "having trouble"
- "loading" (without a `Warming` or `RetestPending` token)
- "offline" (as a bare word, without naming the unreachable owner)
- "try again" (without the recovery route)

If the more precise state is not yet known (a seed gap), the surface
renders `unresolved_axis` and routes to the owning repair hook, rather
than fabricating a generic message.

## Acceptance checklist

A reviewer auditing a protected surface can confirm conformance with
this vocabulary in three passes:

1. **Truth-class pass.** Open the row. Can you name its class from the
   truth-class matrix without reading surface-local copy? If yes, which
   owner family is named? If no, the row fails acceptance.
2. **Degraded-state pass.** If the row is below ideal posture, which
   exact token applies? Is the worst-supporting-truth-wins rule
   honored? Is the copy intent (naming the cause) honored? If not, the
   row fails acceptance.
3. **Machine-readable pass.** Do the truth-class, degraded-state, and
   required copy fields appear as typed values (not free text) in the
   underlying record or packet? Can docs, support templates, and future
   badges reuse them without parsing prose? If not, the row fails
   acceptance.

## Out of scope

Per the originating spec, this document freezes vocabulary and the
matrix. **Wiring every runtime surface to the vocabulary is out of
scope**; it is pulled forward through each surface's own freeze work
referencing this file and its two machine-readable companions.
