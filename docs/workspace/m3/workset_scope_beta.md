# Workset-scope beta truth and broad-action admission

Beta layer on top of the alpha `workset_artifact_record` model in
[`scope_truth_packet.md`](../scope_truth_packet.md). The alpha layer froze
the durable scope artifact, the chip projection, and the widen-diff record.
The beta layer hardens that foundation for the four cross-surface lanes the
spec calls out — **search**, **graph**, **refactor**, **AI**, **export**
plus the **support packet** — so every consumer reads one closed vocabulary
and a single typed admission decision for each broad action.

The machine-readable schema lives at:

- [`/schemas/workspace/workset_scope_beta.schema.json`](../../../schemas/workspace/workset_scope_beta.schema.json)

The canonical fixtures live under:

- [`/fixtures/workspace/m3/scope_truth/`](../../../fixtures/workspace/m3/scope_truth/)

The Rust types are exported from `aureline_workspace::worksets::beta`. The
integration test
[`crates/aureline-workspace/tests/workset_scope_beta.rs`](../../../crates/aureline-workspace/tests/workset_scope_beta.rs)
replays every fixture and proves the closed acceptance states.

## 1 Beta truth contract

The beta truth is a `WorksetScopeBetaTruth` payload derived from one
`WorksetArtifactRecord` and one `BetaConsumerSurface`. It is the record a
search, graph, refactor, AI, export, or support consumer reads when it
decides whether to execute a broad action.

The truth quotes:

- the `workset_ref` and `stable_scope_id` from the underlying artifact — a
  consumer never mints a parallel scope id for the same scope;
- the `scope_class` / `scope_mode` from the alpha vocabulary
  (`current_repo`, `selected_workset`, `sparse_slice`, `full_workspace`,
  `policy_limited_view`);
- the `included_roots` (one row per declared root, with root kind, result
  state, and presentation label) and the `excluded_roots` (one row per
  workspace root that is not in scope, with a typed reason);
- the `include_patterns` and `exclude_patterns` carried verbatim from the
  artifact;
- one `BroadActionAdmission` per broad-action class — the admission
  vocabulary is closed and every class appears exactly once;
- the `lineage` chain (the active artifact plus, for `policy_limited_view`,
  the underlying workset and any parent reference);
- a `portability_lineage_preserved` flag that explains whether the lineage
  chain encodes enough portability truth for an export consumer to replay
  the same scope.

A truth records `outside_current_scope_marker_visible = true` only when it
describes a row (search hit, graph edge, preview) whose owning root is not
in the active workset. Outside-scope truths MUST carry an explain note and
MUST block every destructive / exfiltrating broad action with
`blocked_by_outside_scope`.

## 2 Broad-action admission ladder

The closed broad-action class vocabulary is:

- `search_query`
- `graph_traversal`
- `refactor_apply`
- `ai_apply`
- `export_artifact`
- `support_archive`

Each admission carries one of the closed decision tokens:

- `allowed` — the action may run against the active scope without further
  narrowing. Reason field MUST be empty.
- `narrowed_to_scope` — the action runs but is constrained to the active
  scope's declared roots and patterns. A typed reason is required.
- `blocked_by_policy` — a policy overlay (admin, trust, license) denies the
  action. A typed reason is required.
- `blocked_by_portability` — the artifact's portability class denies the
  action (e.g. managed-provider locked exports). A typed reason is
  required.
- `blocked_by_sparse_partial` — the scope is partial and the action cannot
  be replayed truthfully (e.g. ephemeral session before warming).
- `blocked_by_outside_scope` — the truth describes an outside-scope row and
  the action would silently widen beyond the active workset.

Frozen rules:

1. `search_query` and `graph_traversal` are read-only lanes and stay
   `allowed` for the active scope; an outside-scope truth still allows the
   read so the row stays visible, while destructive lanes block.
2. `refactor_apply` and `ai_apply` MUST `narrowed_to_scope` or block
   whenever the scope is partial (readiness below `ready`, any included
   root not `loaded`, or `scope_class` narrowed below the workspace).
3. `refactor_apply`, `ai_apply`, and `export_artifact` MUST block on
   `policy_limited_view` (`blocked_by_policy`) — the underlying workset is
   the only path that may widen.
4. `export_artifact` MUST block on `managed_provider_locked` artifacts
   (`blocked_by_portability`) regardless of policy.
5. `support_archive` MAY `narrowed_to_scope` under a policy-limited view so
   the hidden-member count is preserved; the hidden member list is never
   exported.
6. `outside_current_scope` truths block every destructive / exfiltrating
   class with `blocked_by_outside_scope` and a typed reason.

## 3 Lineage preservation

The `lineage` field is the chain of `ScopeLineageEntry` records every
consumer carries with a truth. The chain starts with the active artifact
(`lineage[0].workset_ref == workset_ref`) and walks upward through:

- the artifact's `policy_limitation.underlying_workset_ref`, when the
  active scope is `policy_limited_view`;
- the artifact's `parent_workset_ref`, when one is declared.

Frozen rules:

1. Support / export packets MUST quote the lineage verbatim. A bundle that
   flattens a sparse / policy-limited workset into a workspace-wide truth
   is non-conforming.
2. `policy_limited_view` truths MUST include the underlying workset as
   `lineage[1]` with its `narrowing_cause` preserved.
3. Lineage entries carry the readiness, portability class, and narrowing
   cause for the ancestor — a downstream consumer reopens the same scope
   without re-deriving these from a side channel.

## 4 Hidden-member accounting

For `policy_limited_view`, the truth records an excluded-root entry with
reason `policy_hidden` and the policy ref, but never enumerates the hidden
members. The hidden member count is preserved on the underlying artifact's
`policy_limitation.hidden_member_count`; the beta truth references it via
the lineage chain rather than copying the list. Admin-policy and
license-or-export-control narrowing causes MUST NOT expose the hidden
list.

## 5 Outside-current-scope behavior

When a row (search hit, graph edge, preview, refactor preview range) is
owned by a root not in the active workset, the producing surface calls
`WorksetArtifactRecord::project_beta_truth_outside_scope` and:

- sets `outside_current_scope_marker_visible = true`;
- writes an explain note that names the outside root and the user-visible
  cue (e.g. "Quick-open jumped into a sibling repo without a widen
  review");
- rewrites every destructive / exfiltrating admission to
  `blocked_by_outside_scope` with reason `outside_workset_roots`.

The chrome surfaces the marker on the row itself; widening is an explicit
action gated by a `scope_widen_diff_record` (alpha contract), never a
silent admission.

## 6 First consumer surfaces

The first consumer wired here is the
`aureline_workspace::WorksetScopeBetaSupportExport` packet. Triage
reopens a reporter's scope by replaying the support-export against the
same artifact: every consumer surface (search, graph, refactor, AI,
export, support_packet) is bundled as one truth with the same
`workset_ref` / `stable_scope_id`, and the lineage chain is preserved on
the packet so the reviewer never has to re-derive scope from a side
channel.

Subsequent consumers (live search shell, refactor preview, AI context
inspector, export writer) project the same `WorksetScopeBetaTruth` from
their active artifact + observation inputs; they do not invent parallel
scope vocabulary.
