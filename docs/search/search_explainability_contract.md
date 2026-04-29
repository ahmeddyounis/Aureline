# Search Explainability Panel Contract

Search explainability is a product surface, not a support-only diagnostic. A
user looking at quick open, global search, docs search, or graph-backed
discovery must be able to inspect why a row appeared, why it ranked where it
did, what scope was actually searched, and what was hidden or blocked without
leaving the current result surface.

Machine-readable companions:

- [`/schemas/search/search_explainability_panel.schema.json`](../../schemas/search/search_explainability_panel.schema.json)
  defines the renderer-facing `search_explainability_panel_record`.
- [`/fixtures/search/search_explainability_cases/`](../../fixtures/search/search_explainability_cases/)
  contains worked panel records for partial-index, hidden-by-filter,
  policy-limited, provider-limited, and recentness-boosted cases.

This contract composes with:

- [`search_query_session_contract.md`](./search_query_session_contract.md) for
  canonical query-session, result-identity, and explanation-capture ownership.
- [`search_readiness_vocabulary.md`](./search_readiness_vocabulary.md) for
  readiness, freshness, result-truth, hidden-scope, and partial-truth copy.
- [`query_planner_contract_seed.md`](./query_planner_contract_seed.md) for
  planner-pass, shard-contribution, result-set, and source-mix identity.
- [`../ux/quick_open_contract.md`](../ux/quick_open_contract.md) and
  [`../ux/search_result_contract.md`](../ux/search_result_contract.md) for the
  row and focus contracts that open the panel.

If those source contracts already define an enum or copy token, this contract
quotes it. It does not mint surface-local synonyms for ranking reasons,
hidden-scope reasons, freshness classes, or result-truth classes.

## Panel Scope

A search explainability panel answers four questions from the result surface:

| Question | Required panel contents |
|---|---|
| Why did this result appear or rank here? | Ordered ranking reasons, explanation signal classes, source classes, boosted-term refs, recentness or frequency cues, and non-numeric weight buckets. |
| What sources contributed? | Source mix by lane or source class, contribution role, freshness/readiness per source, provider/imported/local posture, and citation refs when imported or docs-backed. |
| What was not shown? | Visible, matching, hidden, hidden-by-filter, policy-blocked, provider-limited, and unknown counts as separate fields. |
| How complete is this result set? | Readiness, freshness, scope class, result-truth class, semantic fallback, stale/partial explanation, approximation notes, and index/graph/docs epoch refs. |

The panel may render compactly, but those fields remain available to keyboard,
screen-reader, CLI explain, export-preview, and support-redacted projections
according to the visibility rules below.

## Canonical Refs

Every panel record cites the canonical owners that produced it:

- `query_session_id_ref`;
- `result_identity_id_ref` when the panel explains a row;
- `result_set_id_ref` and `planner_pass_id_ref`;
- `search_result_packet_ref` when a rendered row exists;
- `explanation_capture_refs` for reusable ranking and omission signals.

The panel may cache labels for presentation, but it must not become a new truth
owner. Reopened panels reconcile against the same query session and result
identity rather than recomputing local explanations from raw provider payloads.

## Same-Surface Inspection

The panel opens inside the surface that displayed the result list.

| Source surface | Same-surface host |
|---|---|
| Quick open | quick-open detail panel or inline inspector |
| Global or file search | search panel detail area |
| Docs search | docs search detail area |
| Graph-backed discovery | graph discovery panel or graph result inspector |
| AI context picker | picker detail area |
| CLI explain | inline structured output tied to the same query/session refs |
| Support export preview | export review panel quoting the same refs |

Rules:

1. A hidden-result count, policy-blocked count, provider-limited count,
   freshness chip, reason chip, or keyboard inspect command must open the panel
   without routing the user to settings, a debug console, or an unrelated admin
   page as the only explanation path.
2. The panel must always provide a return path to the result list or selected
   row.
3. Widen-scope, clear-filter, retry-provider, request-policy-change, rebuild,
   or export actions may be offered from the panel, but the explanation itself
   stays inspectable before any such action runs.
4. Policy and provider repair hooks may hand off to a managed or external
   surface only after the current panel has disclosed the count, reason,
   owner boundary, and support-safe summary.

## Count Truth

Counts are separate axes:

- visible results;
- matching results, when known;
- hidden results;
- hidden-by-filter results;
- policy-blocked results;
- provider-limited results; and
- unknown results.

Hidden is not blocked. A hidden result is withheld from the rendered list under
a hidden-scope reason. A policy-blocked result or action is known or inferred
but cannot be opened, cited, mutated, exported, or revealed under the current
policy/trust/access state. A provider-limited result is missing or partial
because a connected provider, remote shard, or imported lane is unauthorized,
unreachable, stale, or capped.

Approximate counts remain labeled as upper bounds or provider-limited counts.
Unknown counts remain unknown; they must not be flattened to zero.

## Ranking And Source Mix

Ranking reasons use the frozen `ranking_reason_class` vocabulary and reusable
`explanation_signal_class` values from the explanation-capture contract.
Panels may show friendly labels or icons, but the record keeps the canonical
classes.

Required ranking detail:

- ordered reason entries;
- source classes that contributed to each reason;
- boosted-term refs and term boost classes when a user, surface, context, or
  recentness boost affected ranking;
- recentness/frequency signals when they affected ordering;
- non-numeric weight buckets such as primary signal, tie-breaker, recency bias,
  or semantic supplement; and
- explanation capture entry refs when the detail came from an existing capture.

Numeric private rank weights are forbidden. A debug payload may carry opaque
planner refs and bucket names, but support-only debug material is never the sole
answer to why a row ranked, disappeared, or was blocked.

## Visibility Boundaries

Always visible when material:

- primary ranking reasons;
- boosted-term and recentness indicators;
- source mix;
- freshness and scope class;
- hidden-result count;
- policy-blocked count;
- provider-limited count; and
- approximation or partial-index notes.

Detail visible:

- all reusable explanation signals;
- source refs, shard refs, planner-pass refs, and capture refs;
- policy owner boundary refs;
- provider authority refs;
- epoch refs; and
- repair hook refs.

Opt-in debug:

- shard snapshot refs;
- normalized term hash refs;
- planner stage refs;
- non-numeric rank weight buckets;
- duplicate-collapse rationale; and
- provider latency buckets.

Forbidden:

- raw query text;
- raw document bodies;
- raw symbol definitions;
- raw absolute paths;
- raw provider payloads;
- raw URLs;
- secrets;
- numeric private ranking weights;
- labels or paths for hidden results; and
- support-only unexplained heuristics.

## Export And Evidence

Exported search evidence captures the panel state by refs and safe summary
fields. It must include:

- query session, result identity, result set, planner pass, result packet, and
  explanation capture refs;
- visible ranking reason classes and explanation signal classes;
- freshness, scope, semantic fallback, and result-truth classes;
- hidden, blocked, provider-limited, approximate, and unknown count summaries;
- stale or partial explanation refs or frozen copy; and
- the export policy and redaction class used.

Exports must not include raw query text, hidden result labels, hidden paths,
raw provider payloads, raw source bodies, raw URLs, secrets, or numeric weights.
If the panel cannot prove the export-safe posture, export is denied rather than
silently omitting required explanation state.

## Surface Parity

Quick open, global search, docs search, and graph-backed discovery may use
different providers, result densities, and UI layouts, but they reuse the same
explanation terms:

| Surface | Required parity behavior |
|---|---|
| Quick open | Shows ranking, recentness, hot-set, hidden, policy, and provider truth inside quick open. |
| Global search | Shows scope, partial-index, hidden-by-filter, policy-blocked, source-mix, and approximation truth inside the search panel. |
| Docs search | Shows docs-pack/source class, citation/freshness, mirrored/live/local posture, policy-blocked counts, and context-sharing limits inside docs search. |
| Graph-backed discovery | Shows graph readiness, entity/source mix, structural fallback, imported/heuristic distinction, policy-hidden lanes, and graph epoch refs inside the graph surface. |

Provider-specific facts can appear as source refs or provider authority refs,
but provider-specific explanation vocabulary must map to the reusable
explanation signal classes before it reaches the panel.

## Non-Conforming Behavior

The following must fail closed or render an explanatory row:

- a reason chip or hidden-count chip with no same-surface panel;
- a non-zero hidden, blocked, or provider-limited count hidden behind logs,
  settings, or support-only tools;
- a policy-blocked count that lacks policy epoch, owner boundary, and
  support-safe summary refs;
- a graph-backed result that downgrades to lexical or structural fallback
  without updating source mix and approximation notes;
- an exported panel summary that drops ranking reasons or omission counts while
  claiming complete evidence; or
- any explanation whose only meaningful detail is a secret heuristic, numeric
  rank weight, raw provider payload, or support-only artifact.

## Fixture Acceptance

A fixture under `fixtures/search/search_explainability_cases/` is conforming
when a reviewer can determine, without backend-specific jargon:

- which query session, result identity, result set, planner pass, and
  explanation captures own the panel;
- why the current result ranked where it did;
- which terms, recentness signals, and sources affected ranking;
- what counts were visible, hidden, blocked, provider-limited, approximate, or
  unknown;
- how freshness, scope, result truth, and partial-index state were disclosed;
- how the user returns to the result list and inspects policy/scope truth from
  the same surface; and
- which evidence fields can be exported without leaking hidden details.
