# Quick-open result-row, readiness, and focus contract

Quick open is a protected navigation surface. It is a governed view over
the search planner, command graph, recent-location store, route registry,
workspace scope, and deep-link resolver. It is not a private repaint of
search internals, and it must not hide readiness, scope, target, or policy
truth behind a fast fuzzy list.

Machine-readable companions:

- [`/schemas/search/quick_open_result.schema.json`](../../schemas/search/quick_open_result.schema.json)
  defines quick-open session and result-row records.
- [`/schemas/search/quick_open_explanation.schema.json`](../../schemas/search/quick_open_explanation.schema.json)
  defines the same-surface explanation records used by reason chips,
  hidden-result disclosures, policy details, provider details, target
  details, and focus-return inspection.
- [`/fixtures/search/quick_open_rows/`](../../fixtures/search/quick_open_rows/)
  contains seed session, row, and explanation records.

This contract composes with the search result-truth vocabulary
([`docs/search/search_readiness_vocabulary.md`](../search/search_readiness_vocabulary.md)),
the planner / result-fusion contract
([`docs/search/query_planner_contract_seed.md`](../search/query_planner_contract_seed.md)),
the command-palette row contract
([`docs/commands/palette_row_contract.md`](../commands/palette_row_contract.md)),
the navigation and saved-query contract
([`docs/navigation/navigation_and_saved_query_contract.md`](../navigation/navigation_and_saved_query_contract.md)),
and the navigation / escalation rules
([`docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)).
If those contracts define a vocabulary axis already, quick open quotes
that axis rather than minting a local synonym.

## Scope

Frozen here:

- quick-open session state, result-row anatomy, source-lane labels, target
  identity, host and route cues, and source refs;
- readiness and limited-state banners for hot-set-only, partial-index,
  stale, outside-current-workset, hidden-by-filter, policy-limited, and
  provider-limited states, including what remains usable in each state;
- preselection, keyboard navigation, preview, open, cancel, detail
  inspection, and reopen-focus rules; and
- same-surface explanation hooks for ranking reasons, hidden-result counts,
  policy-blocked counts, provider-limited counts, target truth, scope truth,
  and focus-return traceability.

Out of scope:

- final scoring weights, fuzzy-matcher implementation, provider ranking
  quality, index storage, and the live quick-open UI;
- broad command-palette semantics already owned by the command contracts;
  and
- durable saved-query storage beyond the planner / navigation records that
  quick-open rows reference.

## Canonical source chain

Every quick-open row must cite the records that produced it. A row may
materialize short labels for rendering, but it must also carry opaque refs
to the owning truth records.

| Source | Owns | Quick open may project |
|---|---|---|
| `search_result_packet_record` | readiness, result truth, ranking-reason classes, freshness, hidden-result disclosure, policy context | source lane, reason chips, freshness chips, hidden counts, search packet ref |
| `result_fusion_record` | shard contributions, duplicate collapse, result-set identity, planner-pass refs | provenance and rank explanation |
| `search_deep_link_record` / navigation artifact | target drift, remap state, history / bookmark / recent-place continuity | target identity, route cue, reopen target |
| command palette row / command descriptor | command identity, enablement, docs/help, shortcut and automation truth | command or route rows only |
| workspace scope / workset truth | current scope, hidden counts, outside-scope state, policy narrowing | scope banner and widen/reset actions |
| provider overlay state | provider availability, authorization, rate limit, offline/cache state | provider-limited banner and retry / reauth action |

Rules:

1. Quick open must not create independent command, path, symbol, provider,
   or policy truth. It quotes the owning record and adds placement,
   preselection, and focus semantics.
2. Rows may stream before the full index is ready only when their readiness
   state and partial-truth cause are visible on the quick-open surface.
3. Remote or imported rows remain accelerators unless the upstream search
   truth record marks them imported or hybrid and cites the required source
   anchor.
4. A row with hidden content may render a hidden placeholder and a count,
   but it must not render the hidden label, path, symbol, or provider
   payload.

## Result-row anatomy

Every row carries these fields:

| Field group | Required contents |
|---|---|
| Row identity | `quick_open_row_id`, row kind, query session ref, result-set ref, row availability, schema version, minted timestamp |
| Display projection | primary label, secondary label, icon / row kind, source lane, source-lane label ref, optional matched range refs |
| Ranking | `ranking_reason_classes`, primary reason ref, result-fusion ref, search packet ref, duplicate-collapse provenance when present |
| Readiness and freshness | search readiness state, freshness class, semantic fallback state, result-truth class, stale / partial explanation when required |
| Scope and policy | scope filter class, client scopes, policy context, hidden-result disclosure, visible banner state |
| Target identity | target kind, canonical target ref, optional path identity ref, optional symbol anchor ref, optional route ref, deep-link drift/ref, host class |
| Host / route cues | current workspace, current file, current workset, outside current workset, remote workspace, docs pack, provider overlay, command graph, or settings surface |
| Focus contract | invoking focus token, open / preview / cancel / inspect outcomes, reopen preselection reason, and nearest-safe-ancestor fallback |
| Explanation hook | same-surface explanation ref, detail availability, and count summary refs where applicable |

### File rows

File rows open a workspace file, generated artifact, or buffer anchor. They
must preserve target truth through the VFS / path-truth contract:

- `target_kind` is `workspace_file`, `buffer_anchor`, or
  `generated_artifact`.
- `canonical_target_ref` points at the file identity, not a raw absolute
  path.
- `path_identity_ref` is non-null when the row can prove a current VFS
  identity.
- host cue is normally `editor`; split or alternate open changes placement
  only, not target identity.
- hot-set-only rows must render the hot-set banner and must not imply full
  workspace completeness.

### Symbol rows

Symbol rows navigate to a declaration, definition, outline node, route,
test, docs-linked symbol, or graph object.

- `target_kind` is `graph_symbol`, `buffer_anchor`, or `docs_anchor`.
- `symbol_anchor_ref` is non-null when the row targets a symbol.
- `result_truth_class = exact` is allowed only when the upstream symbol /
  graph authority confirms the anchor.
- `structural_fallback` rows remain heuristic and disclose that graph
  confirmation is pending.
- Preview opens the target in a non-committing preview route and preserves
  focus return to quick open unless the user explicitly moves focus into
  the preview.

### Recent-place rows

Recent-place rows restore a previous navigation location, not just a path
string.

- `target_kind` is `recent_location`, `buffer_anchor`, or another concrete
  target kind from the navigation contract.
- The row cites the navigation-history or bookmark artifact that created
  the recent place.
- If the target drifted, quick open renders the drift cue and opens only
  through the deep-link resolver result.
- Reopening quick open after a recent-place open preselects the current
  target when it still resolves; otherwise it falls back to the first
  enabled row and keeps the drift explanation inspectable.

### Routed rows

Routed rows include command routes, settings routes, docs anchors,
provider resources, task/test/debug routes, terminal zones, and output
viewer objects.

- `route_ref` names the command / route / destination object.
- `host_class` names where the route opens.
- `route_cues` make cross-host or external handoff visible before the user
  commits.
- Policy-blocked or provider-limited routed rows remain selectable when
  explainability matters; they expose the same-surface explanation and next
  safe action instead of disappearing.

## Readiness and limited-state banners

The quick-open banner is a primary-surface contract. It may be compact, but
it must be reachable by keyboard and screen reader, and its explanation
must open inside quick open.

| Banner state | Surface meaning | What still works |
|---|---|---|
| `hot_set_only` | Rows are drawn from hot files, open files, recent edits, or nearby symbols. The full workspace tail is not represented. | Typing, opening visible rows, previewing visible rows, inspecting why results are partial, and cancelling back to the invoking surface. |
| `partial_index` | Some indexed scope is ready and some is still warming or rebuilding. | Existing visible rows, previews, hidden-count inspection, and index progress / rebuild detail. |
| `stale` | Cached or previously fresh data is served outside the freshness window. | Opening stale-labeled rows when target identity still resolves, preview, refresh / rebuild action, and detail inspection. |
| `outside_current_workset` | Matching rows exist outside the selected workset or sparse slice. | Visible in-scope rows, explanation of omitted scope, explicit widen / switch-workset action. |
| `hidden_by_filter` | Rows were removed by the current typed query, filter token, or surface mode. | Clear filter, inspect hidden count, keep typing, and cancel. |
| `policy_limited` | Trust or admin policy narrowed the source, target, or provider class. | Visible allowed rows, policy explanation, request / open admin route where allowed, and support-safe count export. |
| `provider_limited` | Connected provider data is unauthorized, offline, rate-limited, stale, or unavailable. | Workspace-backed rows, cached provider rows when labeled, provider detail, retry / reauth path when allowed, and cancel. |

Rules:

1. `No results` is reserved for exact empty sets. If any banner state above
   applies, the empty state must name the limiting condition instead.
2. A non-zero hidden count must render on the quick-open surface, not only
   in logs, settings, or support export.
3. A provider-limited or policy-limited banner may collapse to a chip, but
   the detail panel must still expose count, reason, owner boundary, and
   repair hook refs.

## Preselection and keyboard navigation

Quick open is keyboard-first:

- Focus enters the query field on open.
- The selected row is a stable row id, not a list index. Streaming,
  filtering, virtualization, and corrective replacement must preserve the
  selected row when the row still exists.
- Initial preselection order is:
  1. exact current buffer or current target,
  2. prior row from the same query session when reopened,
  3. recent place matching the current host,
  4. first enabled row,
  5. first selectable explanatory row when all actionable rows are blocked.
- Arrow keys move by visible row order. Page keys move by viewport. Home /
  End move to first / last visible row. Disabled explanatory rows remain
  reachable when they carry a policy, provider, or hidden-result detail.
- `Enter` opens the primary target when enabled or opens the required
  explanation / preview path when the row is blocked.
- Alternate gestures change placement only. They must not widen scope,
  skip review, or silently retarget.
- `Esc` cancels and returns focus according to the focus contract.

## Open, preview, cancel, and reopen focus

Every quick-open session mints a focus-return token when it opens. The
token names the invoking surface and the nearest safe ancestor. Focus rules:

| User action | Focus outcome |
|---|---|
| Open enabled row | Focus moves to the opened target host after the target resolves. The row's target ref and host cue must match the actual opened host. |
| Open blocked row | Focus stays in quick open and the same-surface explanation opens. No target host is focused. |
| Preview row | Focus remains in quick open by default. If the user tabs or clicks into the preview, closing preview returns to the selected row. |
| Inspect detail | Focus moves to the detail panel inside quick open, then returns to the selected row on close. |
| Cancel | Focus returns to the invoking surface. If that surface disappeared, focus returns to the nearest safe ancestor from the token. |
| Reopen quick open | Query field takes focus, and preselection resolves against current target truth rather than stale row position. |

Rules:

1. Opening a result may not silently retarget to a different host, root,
   provider, or route. If target truth changed, the row must be corrected,
   downgraded, or blocked with a drift explanation.
2. Focus return is part of the record. A support packet or accessibility
   replay should be able to reconstruct whether focus moved to target,
   stayed in quick open, or returned to the invoker.
3. Preview and explanation are non-committing. They may inspect target,
   rank, hidden count, policy, or provider state; they may not widen
   authority or open external routes without the row's normal gates.

## Same-surface explanation hooks

Quick open must answer "why this row?", "why not that row?", and "what
scope am I actually seeing?" without sending the user to settings or a
debug page.

An explanation record can be opened from:

- a row reason chip;
- a readiness or scope banner;
- a hidden-result count;
- a policy or provider chip;
- a target / host cue; or
- a keyboard shortcut for the selected row.

The explanation record carries:

- ranking trace: ordered reasons, source lanes, weight classes, shard refs,
  and whether a reason is a primary signal, tie-breaker, recency bias, or
  policy-required inclusion;
- scope truth: current scope class, current scope ref, captured scope ref
  when relevant, result-set ref, hidden counts, and outside-workset counts;
- policy and provider truth: policy-blocked count, provider-limited count,
  owner boundary, repair hook refs, and whether counts are approximate;
- target truth: target kind, canonical target ref, host class, route cues,
  deep-link drift state, and remap / unresolved refs; and
- focus trace: origin focus, open / preview / cancel / inspect outcomes,
  and nearest-safe-ancestor fallback.

The same explanation object is the source for support export, accessibility
narration, AI evidence citations, and power-user detail. Those consumers
quote the record; they do not rederive ranking, hidden counts, or focus
outcomes.

## Non-conforming behavior

The following must fail closed or render an explanatory row:

- a visible row without `search_result_packet_ref` or equivalent routed
  source ref;
- a row whose source lane says hot set while readiness renders as fully
  indexed;
- a non-zero hidden-result count with no same-surface explanation;
- a policy-blocked, provider-limited, or outside-workset result hidden
  with no count;
- a preview or alternate-open gesture that changes target authority;
- a stale or drifted recent place opened as if it were exact; or
- a cancel / preview / explanation close path that drops focus without the
  recorded fallback.

