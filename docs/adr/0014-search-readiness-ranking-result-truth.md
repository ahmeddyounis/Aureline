# ADR 0014 — Search readiness, ranking-reason, hidden-scope, result-truth, and deep-link drift vocabulary

- **Decision id:** D-0020 (see `artifacts/governance/decision_index.yaml#D-0020`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-10-01
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** product_scope_review
- **Related requirement ids:** none

## Context

The command palette, full search, symbol jump, docs search, graph
overlay, AI-explanation overlay, and support-export surfaces all
answer the same family of questions about the same product
instance: is the index ready; which rows are exact graph facts,
which are imported, which are heuristic, and which are hybrid;
which rows were hidden and why; how many rows were hidden; is this
result set stale or partial and why; did a stored deep link still
resolve to the same entity; and — when the semantic service is
degraded or unavailable — is this a lexical-only result set. If
each surface invents its own answer — its own "still indexing"
copy, its own "approximate" label, its own "semantic unavailable"
tooltip, its own "link broken" fallback — then the palette reader,
the docs reader, the symbol-jump caller, the AI explainer, and the
support-export consumer each see a different cut of the same
truth and a parity audit between palette, full search, symbol
jump, and docs search degenerates into a hand-mapping exercise.

The source documents
(`.t2/docs/Aureline_Technical_Architecture_Document.md`,
`.t2/docs/Aureline_Technical_Design_Document.md`,
`.t2/docs/Aureline_PRD.md`,
`.t2/docs/Aureline_UI_UX_Spec_Document.md`,
`.t2/docs/Aureline_Milestones_Document.md`) treat readiness
(`Loaded` / `ManifestKnown` / `Cached` / `Unavailable`;
`hot_set_ready` / `partial_index` / `warm_index` / `stale_index`),
result truth (`exact graph fact`, `imported or mirrored fact`,
`heuristic summary`, `structural fallback`), lexical-first
ranking, hidden-result count, partial-index disclosure, scope /
filter notes, and deep-link remap / drift corpora as first-order
product contracts. The PRD MUSTs are explicit: "the product MUST
become useful before the full repo is indexed", "operations that
cannot see the full relevant scope MUST label partial truth
rather than pretending global completeness", and "every graph-
backed claim must distinguish exact graph fact, imported fact,
heuristic summary, and fallback structure". What is still
missing is the shared vocabulary that pins readiness, ranking
reason, hidden scope, result truth, partial-index cause,
semantic fallback, and deep-link drift into one typed packet
every search / navigation surface projects and every downstream
consumer reads.

The freeze matters now, ahead of the ranking, graph-overlay,
docs-search, AI-explanation-citation, bookmark / session-restore,
and support-export lanes landing: if those lanes proliferate
before a shared vocabulary is frozen, each will mint its own
("indexing..." on one surface; "still catching up" on another;
"some results hidden" on a third; a silently broken deep link on
a fourth), and downstream ranking-explainability, parity-audit,
and claim-manifest tooling will have nothing mechanical to
consume. This ADR closes `D-0020` (search readiness,
ranking-reason, hidden-scope, result-truth, and deep-link drift
vocabulary) so the ranking, graph-overlay, docs-search,
AI-explanation-citation, bookmark / session-restore, support-
export, and palette / symbol-jump lanes can instrument against
one contract.

This ADR rides alongside the ADR-0001 identity modes (the
`managed_admin_surface` client scope and the `policy_blocked`
denial ride that envelope), the ADR-0004 RPC transport (search
packets, session records, deep-link records, and audit events
cross as typed payloads; raw query bodies, raw document bodies,
raw symbol definitions, and raw URLs never do), the ADR-0005
subscription envelope (every rendered search row carries an
authority class and a freshness hint), the ADR-0007 secret
broker (no raw secret material reaches search rows), the
ADR-0008 settings resolver (admin policy may narrow semantic
scoring, narrow source classes, raise the freshness floor, or
disable specific surfaces — MAY NOT silently widen), the
ADR-0009 execution-context model (every emitted packet names an
`execution_context_id` and a `scope_filter_class` drawn from the
workset / scope vocabulary), the ADR-0011 capability-lifecycle
vocabulary (the `freshness_class`, `client_scope`, and
`redaction_class` axes are re-exported without modification),
and the ADR-0013 docs / Help / service-health truth-source
contract (the `citation_anchor_record` shape and the `source_class`
vocabulary back every `imported` or `docs_search` result; the
`browser_handoff_packet` envelope is the only path any search
row uses to leave the product). This ADR does not redefine those
contracts; it pins the search-specific axes and the record
shapes they project into.

A live ranking engine, the actual index implementation, the graph
store, the semantic embedding service, and the session-restore /
bookmark storage layer are explicitly out of scope at this
milestone; this freeze establishes the vocabulary those later
surfaces will honour.

## Decision

Aureline freezes one **readiness vocabulary** for every search /
navigation surface; one **result-truth-class vocabulary** for
every result row (`exact`, `imported`, `heuristic`, `hybrid`);
one **ranking-reason-class vocabulary** enumerating every
contributor to the ranking; one **hidden-scope vocabulary** for
rows withheld from the rendered result set; one
**partial-truth-cause vocabulary** for stale-or-partial
explanations; one **semantic-fallback-state vocabulary** for
lexical-only / degraded / supplement / disabled-by-policy modes;
one **deep-link-drift-state vocabulary** for bookmark / session
/ AI-evidence / cross-surface navigation links; one
**`search_result_packet_record`** that every palette row, full-
search row, symbol-jump row, docs-search row, graph-overlay row,
AI-explanation overlay row, and support-export row projects; and
a paired **`search_session_record`** and **`search_deep_link_record`**
so a later parity audit can compare surfaces field-for-field
without inventing search-local copy.

Every result row rendered on a search / navigation surface
resolves to exactly one `search_result_packet_record`; the packet
binds one readiness state, one result-truth class, at least one
ranking-reason class, one semantic-fallback state, one scope /
filter class, one freshness class, one running-build identity
ref, one policy context, one redaction class, and — when any row
is hidden — a typed `hidden_result_disclosure` with a count and
at least one hidden-scope reason. Every non-`fully_indexed`
readiness, every `heuristic` or `hybrid` truth class, every
semantic-fallback state below `semantic_available_as_supplement`,
and every non-null hidden-result disclosure MUST carry a typed
`stale_or_partial_explanation` whose `human_readable_summary`
quotes the frozen copy in
`docs/search/search_readiness_vocabulary.md` verbatim.

All rules below are stated in terms of contract, vocabulary, and
event names rather than specific crates so surface changes are
hygiene, not re-litigation.

### Source-of-truth ownership (frozen)

Each surface family has exactly one canonical owner. A surface
that re-renders a row owned by another family MUST quote the
owner's packet rather than mint its own; a surface that mints
its own copy of a row another family owns is non-conforming.

| Surface family                             | Canonical owner                                                                                      | Authority class on the subscription envelope (ADR-0005) |
|--------------------------------------------|------------------------------------------------------------------------------------------------------|---------------------------------------------------------|
| Command palette                            | `palette_command_registry` (host process) — registry of command graph + recent-use bias.             | `derived_knowledge`                                     |
| Full text / path search                    | `search_indexer` (host process) — authoritative lexical / structural index over the loaded scope.    | `derived_knowledge`                                     |
| Symbol jump / go-to-symbol                 | `symbol_jump_resolver` (host process) — resolves a symbol query into a graph neighbourhood.          | `derived_knowledge`                                     |
| Graph overlay / navigation                 | `graph_authority` (host process) — authoritative graph store, owns `exact` truth for graph facts.    | `derived_knowledge`                                     |
| Docs search                                | `docs_pack_registry` (ADR-0013) — docs-pack manifest registry.                                       | `derived_knowledge`                                     |
| AI explanation overlay search              | `derived_explanation_session` (ADR-0013) — never authoritative; MUST cite a `citation_anchor_record`. | `derived_knowledge` (must cite)                         |
| Support-export search summary              | `support_export_pipeline` (ADR-0013) — quotes owners above; never re-mints their packets.            | `derived_knowledge`                                     |
| Deep-link / bookmark resolution            | `deep_link_resolver` (host process) — single resolver across palette, symbol-jump, full-search, and docs-search deep links. | `derived_knowledge`                                     |

Rules (frozen):

1. The owners above are the **only** authorities for their rows.
   Surfaces that render these rows subscribe to them; copy-only
   shadow surfaces are forbidden, mirroring the ADR-0008
   "one registry, no shadows" posture and the ADR-0013
   "no shadow docs" rule.
2. A derived explanation (AI) that quotes a search row MUST cite
   at least one `citation_anchor_record` (ADR-0013); an AI-
   explanation row that cannot cite is denied with
   `derived_explanation_uncited` at the ADR-0013 boundary.
3. The `graph_authority` is the only authority that may emit
   `result_truth_class = exact` against a graph-backed claim.
   The `search_indexer` MAY emit `result_truth_class = exact`
   against a lexical / path claim inside its loaded scope.
   Every other owner MUST emit `imported`, `heuristic`, or
   `hybrid` unless it quotes the graph authority inline.
4. The `deep_link_resolver` is the single resolver for bookmark,
   session-restore, AI-evidence anchor, and cross-surface
   navigation links. Per-surface resolvers MAY NOT mint parallel
   drift vocabularies.

### Readiness-state vocabulary (frozen)

Every packet names exactly one readiness state from the closed
set below. The set is closed; adding a state is additive-minor
and bumps `search_result_schema_version`; repurposing a state is
breaking and requires a new decision row.

| State                  | Meaning                                                                                                                  | Partial-truth disclosure required? |
|------------------------|--------------------------------------------------------------------------------------------------------------------------|------------------------------------|
| `not_indexed`          | The surface has not started indexing this scope. Only `imported` facts (from manifests / packs) may render.              | Yes                                |
| `hot_set_ready`        | The hot set (changed files, current build targets, nearby tests) is indexed; the rest is not.                            | Yes                                |
| `partial_index`        | A subset of the loaded scope is indexed; the index is still growing.                                                     | Yes                                |
| `warm_index`           | Most of the loaded scope is indexed; a minor background refresh is still running.                                        | Yes                                |
| `fully_indexed`        | The loaded scope is fully indexed inside the refresh window.                                                             | No                                 |
| `stale_index`          | Index was fresh but the refresh window has elapsed without a re-verify.                                                  | Yes                                |
| `reindexing`           | The index is being rebuilt from scratch (schema bump, manifest change, recovery).                                        | Yes                                |
| `index_unavailable`    | The surface cannot reach its indexer at all (crash, policy block, remote shard unreachable).                             | Yes                                |

Rules (frozen):

1. `fully_indexed` is the only state that renders a result set as
   complete for the requested scope; every other state MUST
   surface a typed `stale_or_partial_explanation`.
2. The readiness state is computed on render; it is not cached
   past the packet's freshness window without a re-verify.
3. `index_unavailable` denies mutating navigation (e.g. "open
   matching symbol") and routes to a repair hook; silent
   fallback to an empty result set is forbidden.

### Result-truth-class vocabulary (frozen)

Every row names exactly one result-truth class. The set is closed.

| Class       | Meaning                                                                                                                                           | Admissible owner(s)                                                       |
|-------------|---------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------|
| `exact`     | Proven from the current graph or lexical index for this scope.                                                                                    | `graph_authority`, `search_indexer`, `palette_command_registry`, `symbol_jump_resolver` |
| `imported`  | Fact pulled from a mirrored pack, docs pack, manifest, or provider overlay. Stable under drift only through its `source_revision_ref`.            | `docs_pack_registry`, `search_indexer` (for manifest-derived facts), provider overlay |
| `heuristic` | Probabilistic, naming-similarity, or semantic-embedding derived. MUST NOT render alone as authoritative.                                          | any semantic-capable owner, `search_indexer` for fuzzy matches            |
| `hybrid`    | Combines at least two of the above and MUST enumerate its contributors in `ranking_reason_classes`.                                               | any owner that actually combines contributors                             |

Rules (frozen):

1. `heuristic` and `hybrid` rows MUST carry a
   `stale_or_partial_explanation` and MUST render the typed
   disclosure on the primary surface (not in a tooltip).
2. A row whose `ranking_reason_classes` includes
   `semantic_embedding_neighbour` MUST have `result_truth_class
   = heuristic` or `hybrid`, never `exact`.
3. `imported` rows MUST carry at least one
   `citation_anchor_refs` entry (ADR-0013); an `imported` row
   without an anchor is non-conforming.
4. The palette, symbol-jump, and full-search surfaces render
   `exact` rows before any `imported`, `heuristic`, or
   `hybrid` row on the same query by default (lexical-first
   ranking). Admin policy MAY narrow this ordering further but
   MAY NOT silently widen it.

### Ranking-reason-class vocabulary (frozen)

Every packet enumerates at least one ranking-reason class. The
set is closed. Per-surface free-form reason strings are
forbidden.

Classes (frozen):

- `exact_name_match`
- `exact_path_match`
- `lexical_prefix_match`
- `lexical_fuzzy_match`
- `symbol_kind_prior`
- `graph_neighbourhood_hop`
- `recent_file_bias`
- `recent_edit_bias`
- `frequent_use_bias`
- `imported_fact_match`
- `heuristic_summary_cluster`
- `semantic_embedding_neighbour`
- `docs_anchor_match`
- `palette_command_canonical`
- `symbol_jump_declaration_anchor`
- `structural_fallback`
- `policy_hidden_placeholder`

Rules (frozen):

1. `structural_fallback` names the parser / symbol-inferred
   fallback used when the graph could not confirm a relation.
   Surfaces emitting `structural_fallback` MUST NOT claim
   `result_truth_class = exact`.
2. `policy_hidden_placeholder` is emitted only on rows whose
   presence is disclosed but whose content is hidden (see
   `hidden_scope_reason`). A `policy_hidden_placeholder` row
   renders a typed count and repair hook; it MUST NOT render
   the hidden content.
3. A `hybrid` truth class MUST list at least two contributor
   classes in `ranking_reason_classes`.

### Hidden-scope vocabulary (frozen)

Every non-zero hidden-result count MUST enumerate at least one
reason from the closed set below. A surface MAY group hidden
rows under a single reason; it MUST NOT silently drop the count.

- `trust_state_excludes_surface`
- `policy_narrows_source`
- `sparse_scope_excludes_root`
- `outside_loaded_scope`
- `excluded_by_user_filter`
- `client_scope_excludes_surface`
- `provider_overlay_unauthorised`
- `pack_quarantined`
- `redaction_narrowed`
- `remote_shard_unreachable`

Rules (frozen):

1. When the surface cannot compute an exact count, it MAY emit
   an upper bound and set `count_is_approximate = true`.
   Surfaces MUST NOT round a known count to zero.
2. Zero hidden rows is a valid packet; the surface still
   records the disclosure as null. A known non-zero count with
   a null disclosure is non-conforming.
3. `policy_narrows_source` and `trust_state_excludes_surface`
   denials carry the policy epoch and an ADR-0011 repair hook;
   silent removal of the hidden-result count is forbidden.

### Partial-truth-cause vocabulary (frozen)

Every readiness state other than `fully_indexed` MUST carry at
least one cause from the closed set below on its
`stale_or_partial_explanation`.

- `hot_set_only`
- `manifest_only`
- `cache_only`
- `outside_loaded_scope`
- `indexing_in_progress`
- `semantic_service_unavailable`
- `remote_shard_unreachable`
- `provider_overlay_unreachable`
- `policy_blocked_shard`
- `freshness_floor_unmet`
- `stale_index_served`

Rules (frozen):

1. Every cause comes with a `human_readable_summary` quoted
   verbatim from the frozen copy table in
   `docs/search/search_readiness_vocabulary.md`. A surface that
   paraphrases the copy is non-conforming.
2. Multiple causes are allowed; a surface MUST emit every cause
   that actually applies, not just the most visible one.

### Semantic-fallback-state vocabulary (frozen)

Every packet names exactly one semantic-fallback state. The set
is closed.

- `semantic_unavailable_lexical_only` — default on palette /
  symbol-jump and on any surface whose semantic service is
  unreachable.
- `semantic_degraded_lexical_preferred` — semantic service is
  reachable but slow, partial, or rate-limited; lexical rows
  render first.
- `semantic_available_as_supplement` — semantic service is
  healthy; semantic rows may appear alongside lexical rows. The
  only state under which `result_truth_class = hybrid` with a
  semantic contributor is admissible.
- `semantic_disabled_by_policy` — admin policy has narrowed
  semantic scoring off on this surface; surface MUST render
  a typed disclosure.
- `semantic_not_applicable` — the surface contract does not
  include semantic scoring (e.g. palette command rows).

Rules (frozen):

1. `semantic_unavailable_lexical_only` is the canonical non-
   semantic fallback; every surface MUST be able to serve rows
   under this state (no surface may degrade to "no results"
   when semantic is unavailable).
2. `semantic_disabled_by_policy` carries the policy epoch and
   a typed repair hook (`request_admin_policy_change`).

### Deep-link-drift-state vocabulary (frozen)

Every deep-link resolution on bookmark restore, session
restore, AI evidence packet, support export, or cross-surface
navigation carries exactly one drift state. The set is closed.

- `resolved_exact`
- `resolved_remapped`
- `resolved_ambiguous`
- `target_missing`
- `target_moved`
- `target_renamed`
- `target_branch_drifted`
- `target_policy_blocked`
- `target_scope_excluded`
- `index_not_ready_for_target`
- `unresolvable`

Rules (frozen):

1. `resolved_remapped` links MUST carry a non-empty
   `remap_chain_refs` pointing at the predecessor
   `deep_link_id`(s) so a parity audit can reconstruct the
   rename chain without re-resolving.
2. `target_branch_drifted` distinguishes the case where the
   target still exists but under a different branch / revision
   from the saved link; surfaces MUST NOT silently follow a
   branch-drifted link without disclosing the drift.
3. `target_policy_blocked` and `target_scope_excluded` carry
   the policy epoch and a repair hook; they MUST NOT silently
   collapse to `target_missing`.
4. `resolved_ambiguous` renders the candidate set (ids only)
   and a repair hook (`request_admin_policy_change` or
   `refresh_freshness`); silent choice of one candidate is
   forbidden.

### Required parity fields (frozen)

The fields below MUST be present on every emitted
`search_result_packet_record` so a later parity audit between
the palette, full search, symbol jump, docs search, graph
overlay, AI explanation overlay, and support-export surfaces can
compare them mechanically without inventing search-local copy:

- `surface_class`,
- `readiness_state`,
- `result_truth_class`,
- `ranking_reason_classes` (non-empty),
- `semantic_fallback_state`,
- `scope_filter_class`,
- `freshness_class`,
- `client_scopes` (non-empty),
- `hidden_result_disclosure` (or explicit null),
- `stale_or_partial_explanation` (when required by the rules
  above),
- `running_build_identity_ref`,
- `policy_context`,
- `redaction_class`.

A packet that omits any of these fields when required is
non-conforming.

### Per-surface projection requirements (frozen)

Each surface MUST project the vocabulary into its packet using
the fields below. A surface that renders a row without the
required projection is non-conforming.

| Surface                       | Required projected fields                                                                                                                                                                                                                                      | Required disclosure                                                                                                                             |
|-------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------|
| `command_palette`             | All required parity fields. `semantic_fallback_state` is `semantic_not_applicable` by default; `result_truth_class` is `exact` (`palette_command_canonical`) or `imported` (command contributed by an extension manifest).                                   | Hidden palette rows (policy-blocked commands) render a typed `policy_hidden_placeholder` count; silent omission is forbidden.                   |
| `full_search`                 | All required parity fields plus at least one `ranking_reason_class` from the lexical / graph / semantic families. Lexical-first ranking is the default.                                                                                                        | Partial-index readiness MUST render a typed disclosure on the primary surface (not in a tooltip); hidden-result counts MUST render inline.      |
| `symbol_jump`                 | All required parity fields plus `symbol_jump_declaration_anchor` on any exact declaration row; `structural_fallback` on any row without a graph confirmation.                                                                                                  | `target_branch_drifted` and `target_renamed` deep-link drift states MUST render an inline drift chip.                                           |
| `docs_search`                 | All required parity fields plus a non-empty `citation_anchor_refs` array for every row; `result_truth_class = imported` when the row is backed by a docs pack; `vendor_overrides_project` disclosure (ADR-0013) when applicable.                               | Cached / stale / signature-unverified docs packs render a typed partial-truth cause and repair hook.                                            |
| `graph_overlay`               | All required parity fields plus `graph_neighbourhood_hop` in `ranking_reason_classes`; `result_truth_class = exact` only when the `graph_authority` confirms.                                                                                                  | `structural_fallback` rows render a typed disclosure.                                                                                           |
| `ai_explanation_overlay`      | All required parity fields plus a non-empty `citation_anchor_refs` pointing at authoritative anchors; `result_truth_class` is `imported` or `hybrid`; AI rows MUST NOT claim `exact` on their own.                                                             | A row without an authoritative anchor is denied under ADR-0013 (`derived_explanation_uncited`).                                                 |
| `support_export`              | All required parity fields plus a quoted owner packet (never re-minted); readiness / truth / hidden-scope / deep-link records are carried verbatim; a missing required field denies the export with `parity_field_missing`.                                    | The support-export redaction envelope applies; raw query bodies, raw document bodies, and raw URLs are stripped before export.                  |

Rules (frozen):

1. Chip collapsing is a UI freedom, not a record-shape freedom.
   A surface that folds readiness / truth / hidden-scope /
   partial-truth into one chip MUST keep the underlying fields
   separately addressable in its packet so parity audits,
   support bundles, and AI explanations can read each axis
   independently.
2. The `deep_link_resolver` is single-source. Per-surface
   resolvers MAY compute a local candidate set but MUST hand the
   resolution to the resolver; a surface that mints its own
   drift vocabulary is non-conforming.
3. The `graph_authority` is single-source for `exact` graph
   facts. A row that claims `result_truth_class = exact` on a
   graph-backed topic without quoting the graph authority is
   non-conforming.
4. Lexical-first ranking is the default across the palette,
   full search, and symbol jump. Admin policy MAY narrow the
   ordering further; admin policy MAY NOT silently widen
   semantic rows past the default cutoff.

### Audit events (frozen)

Every emission, every readiness transition, every hidden-result
count update, every semantic fallback engagement / restoration,
every deep-link resolution, and every denial emits a structured
event on the `search` audit stream. Events carry the packet /
session / link id, the surface class, the readiness state, the
result-truth class, the semantic fallback state, the deep-link
drift state (when applicable), and the policy context. Events
MUST NOT carry raw query bodies, raw document bodies, raw symbol
definitions, or raw URLs.

| Event id                                        | Fires when                                                                                            |
|-------------------------------------------------|-------------------------------------------------------------------------------------------------------|
| `search_query_opened`                           | A `search_session_record` opened.                                                                     |
| `search_result_packet_emitted`                  | A `search_result_packet_record` was emitted on a surface.                                             |
| `search_result_packet_refused`                  | Emission refused (required parity field missing, schema version lagging, etc.).                       |
| `search_readiness_state_changed`                | Readiness state transitioned (`partial_index` -> `warm_index`, etc.).                                 |
| `search_partial_index_disclosed`                | A packet surfaced a partial-index disclosure (first emission with a partial-truth cause).             |
| `search_semantic_fallback_engaged`              | Semantic service transitioned out of `semantic_available_as_supplement`.                              |
| `search_semantic_restored`                      | Semantic service returned to `semantic_available_as_supplement`.                                      |
| `search_hidden_result_count_updated`            | A packet's hidden-result count changed materially.                                                    |
| `search_result_truth_class_downgraded`          | A row's truth class downgraded (`exact` -> `hybrid`, `hybrid` -> `heuristic`, etc.).                  |
| `search_deep_link_resolved`                     | A deep link resolved successfully (`resolved_exact` / `resolved_remapped` / `resolved_ambiguous`).    |
| `search_deep_link_drifted`                      | A deep link resolved with a drift-carrying state (any non-`resolved_exact`).                          |
| `search_deep_link_unresolvable`                 | A deep link could not be resolved (`target_missing`, `unresolvable`).                                 |
| `search_scope_narrowed_by_policy`               | Admin policy narrowed the scope / source class on a surface.                                          |
| `search_result_schema_version_bumped`           | `search_result_schema_version` was bumped.                                                            |

### Denial posture (frozen)

When a search surface cannot render a packet safely it denies.
Denial is typed, visible, auditable, and repairable. Silent
downgrade to an empty result set is forbidden.

Denial reasons (frozen):

- `readiness_state_unresolved`
- `result_truth_class_unresolved`
- `ranking_reason_missing`
- `hidden_scope_count_unresolved`
- `partial_truth_cause_missing`
- `semantic_fallback_state_unlabelled`
- `deep_link_drift_unresolved`
- `surface_class_excludes_row`
- `client_scope_excludes_surface`
- `policy_blocked`
- `raw_query_body_forbidden_on_boundary`
- `raw_document_body_forbidden_on_boundary`
- `search_result_schema_version_lagging`

Denials fail closed. They MUST NOT silently retry, MUST NOT
substitute a different truth class or readiness state, and MUST
emit the corresponding audit event.

### Process-boundary constraints (frozen)

1. `search_result_packet_record`, `search_session_record`,
   `search_deep_link_record`, and `search_audit_event_record`
   instances cross the RPC boundary as typed payloads
   (ADR-0004). Raw query bodies, raw document bodies, raw
   symbol definitions, and raw URLs never cross.
2. The `palette_command_registry`, `search_indexer`,
   `symbol_jump_resolver`, `graph_authority`, and
   `deep_link_resolver` are authoritative in the host process.
   Extensions, AI tool calls, recipes, and remote helpers read
   these surfaces only through the shared subscription
   envelope (ADR-0005) with authority class `derived_knowledge`
   and a declared freshness hint.
3. Remote-agent attach surfaces a remote-scoped search view
   whose `client_scope` is `remote_agent`; the host surface
   renders a typed disclosure for any row whose client scope
   excludes the remote agent.
4. Crash dumps and core files MUST NOT inherit unresolved
   search packets; a crash that lands mid-render discards the
   packet rather than persisting a partial axis set.
5. Mutation-journal entries, save manifests, support bundles,
   and evidence packets name `packet_id`,
   `query_session_id_ref`, `deep_link_id`, and
   `running_build_identity_ref` only; they MUST NOT embed raw
   query bodies, raw document bodies, or raw URLs.
6. AI tool calls MUST NOT cache search rows past the packet's
   freshness window without re-resolving; a cached row that
   outlives its anchors is denied under ADR-0013
   (`derived_explanation_uncited`).

### Redaction defaults (frozen)

Every surface that emits observable state declares a redaction
class; the broker-owned redaction pass (ADR-0007) runs before
bytes reach any persistent or exportable sink.

| Sink                                 | Default inclusion (packet / session / link / event fields)                                                                                                                     |
|--------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | `packet_id`, `query_session_id_ref`, `surface_class`, `readiness_state`, `result_truth_class`, `semantic_fallback_state`, `deep_link_drift_state`, audit-event ids. No raw query, document, or URL bodies. |
| `traces_local`                       | Same as `logs_local`; span names MUST NOT include raw query or document bodies.                                                                                                |
| `support_bundle`                     | Full per-axis values, full hidden-scope enumeration, full deep-link drift enumeration, full citation-anchor enumeration. Raw bodies excluded.                                  |
| `evidence_packet`                    | Release-relevant fields: `running_build_identity_ref`, `readiness_state`, `result_truth_class`, full `citation_anchor_refs`. Raw bodies never included.                        |
| `ai_context_capture`                 | `packet_id`, `readiness_state`, `result_truth_class`, `ranking_reason_classes`, `citation_anchor_refs`, `stale_or_partial_explanation`. Raw bodies and raw URLs never captured.|
| `recipe_manifest`                    | `packet_id`, `running_build_identity_ref`, `deep_link_id` only. Raw bodies and raw URLs forbidden.                                                                             |
| `profile_export` / `sync`            | Same as `recipe_manifest`.                                                                                                                                                      |
| `crash_dump`                         | Opt-in only; redaction scan precedes packaging; denied by default for packets whose `policy_context` references a managed policy bundle.                                       |
| `mutation_journal_entry`             | `packet_id`, `query_session_id_ref`, `deep_link_id`, `surface_class`, `readiness_state`, `result_truth_class`, audit-event id. No raw bodies or raw URLs.                      |
| `save_manifest` (ADR-0006)           | Same as `mutation_journal_entry`.                                                                                                                                               |
| `claim_manifest`                     | Full per-axis values, full citation-anchor enumeration. Raw bodies never included.                                                                                              |
| `terminal_transcript`                | `packet_id` and `surface_class` only; raw URLs require boundary-labelled confirmation before capture.                                                                          |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

### Linkage to neighbouring contracts (frozen)

- **Execution-context and workset / scope vocabulary
  (ADR-0009).** The `scope_filter_class` field on every packet
  is drawn from the scope vocabulary frozen by ADR-0009. The
  `policy_context.execution_context_id` re-uses the ADR-0009
  id.
- **Capability-lifecycle vocabulary (ADR-0011).** The
  `freshness_class`, `client_scope`, and `redaction_class`
  enumerations are re-exported without modification; this ADR
  does not redefine them.
- **Docs / Help / service-health truth contract (ADR-0013).**
  The `citation_anchor_refs` on every `imported`,
  `docs_search`, or `ai_explanation_overlay` row name
  `citation_anchor_record` ids from ADR-0013. The
  `browser_handoff_packet` envelope (ADR-0010, re-exported
  through ADR-0013) is the only path a search row uses to
  leave the product; raw URL launches from search surfaces are
  forbidden.
- **Subscription envelope (ADR-0005).** Every packet rides the
  shared envelope with authority class `derived_knowledge` and
  a declared freshness hint.
- **Settings resolver (ADR-0008).** Admin policy MAY narrow
  which source classes contribute to ranking, narrow which
  surfaces expose semantic scoring, raise the freshness floor
  on an individual surface, or force a step-up authenticator
  on a `browser_handoff` from a docs-search row. Policy MAY
  NOT silently widen.

### Schema-of-record posture (frozen)

Rust types in the eventual search / navigation crate are the
source of truth. The JSON Schema export at
`schemas/search/search_result_truth.schema.json` is the cross-
tool boundary every non-owning surface reads. Adding a new
readiness state, result-truth class, ranking-reason class,
hidden-scope reason, deep-link drift state, partial-truth
cause, semantic-fallback state, surface class, audit-event id,
or denial reason is additive-minor and bumps
`search_result_schema_version`; repurposing a value is breaking
and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, ADR 0010, ADR 0011, ADR 0012, and ADR 0013.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- A live ranking engine implementation (scoring weights,
  tuning, A/B infrastructure). The vocabulary here reserves
  the ranking-reason axis; the engine lands later.
- The actual index implementation (lexical index, graph store,
  semantic embedding service, manifest pipeline). The
  vocabulary here pins readiness, truth class, hidden scope,
  partial-truth causes, and semantic fallback states; the
  index lane fills in the bodies.
- The session-restore / bookmark storage layer. The
  vocabulary here reserves the deep-link record shape and the
  drift vocabulary; the storage lane wires it to persistent
  state.
- The citation / symbol-reference packet body (ADR-0013
  reserves `citation_anchor_record`; this ADR consumes it).
- AI explanation infrastructure (model selection, prompt
  templates, provider routing). This ADR pins the disclosure
  and citation contract AI rows must honour; the explanation
  pipeline rides ADR-0013.

These lines move only by opening a new decision row, not by
editing this ADR.

### Tradeoff summary

| Axis                          | Chosen stack                                                                                                                                                                            | Best rejected alternative                                                                            | Why chosen wins                                                                                                                                  |
|-------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| **Source-of-truth ownership** | Eight named owners (palette command registry, search indexer, symbol-jump resolver, graph authority, docs-pack registry, derived-explanation session, support-export pipeline, deep-link resolver) with no shadow surfaces | Each surface family owns its own index and reconciles offline                                        | Per-surface copies recreate palette / search / symbol-jump / docs-search drift; one canonical owner per family makes parity audits mechanical.   |
| **Readiness vocabulary**      | Eight closed readiness states with a partial-truth-disclosure rule on every non-`fully_indexed` state                                                                                    | Free-form "still loading" copy                                                                       | Free-form copy is not machine-readable; typed states let parity audits, support exports, and AI overlays consume the axis.                       |
| **Result-truth axis**         | Four closed classes (`exact`, `imported`, `heuristic`, `hybrid`) with admissible-owner rules                                                                                             | One generic "confidence" float                                                                       | A float hides whether a row is graph-proven, manifest-imported, or semantic-guessed; users and parity audits cannot tell them apart.             |
| **Ranking-reason axis**       | Seventeen closed reason classes emitted per-row; free-form reason strings forbidden                                                                                                      | Free-form "why this result" tooltip                                                                  | Tooltips are not machine-readable; typed classes make explainability and ranking-engine A/B testing mechanical.                                  |
| **Hidden-scope axis**         | Ten closed hidden-scope reasons with a typed disclosure carrying a count and an approximate flag                                                                                         | Silent omission of hidden rows                                                                       | Silent omission hides trust / policy narrowing from the user and from parity audits; typed disclosure makes it visible and repairable.           |
| **Deep-link drift axis**      | Eleven closed drift states with a remap-chain record                                                                                                                                     | "Broken link" fallback with no typed cause                                                           | Breaks rename / move / branch-drift / policy-blocked apart so the repair hook is specific and the support export can reconstruct the chain.      |
| **Semantic fallback axis**    | Five closed states with a mandatory lexical-only guarantee                                                                                                                               | "Semantic unavailable" tooltip                                                                       | The lexical-only guarantee is the PRD MUST; the typed states make the degraded / supplement / policy-blocked cases first-class.                  |
| **Packet record shape**       | One `search_result_packet_record` shared by palette, full search, symbol jump, docs search, graph overlay, AI overlay, and support export                                                | Per-surface packet records                                                                           | Per-surface records recreate the drift this ADR closes; one record makes parity audits mechanical.                                               |
| **Schema of record**          | Rust types in the eventual search / navigation crate; JSON Schema export at `schemas/search/search_result_truth.schema.json`                                                             | External IDL + codegen at this milestone                                                              | No second-language consumer yet; the JSON Schema export reserves a clean integration point.                                                      |
| **Citation reuse**            | Quote ADR-0013 `citation_anchor_record` ids on `imported`, `docs_search`, and `ai_explanation_overlay` rows                                                                               | Mint a parallel search-specific anchor vocabulary                                                     | Two anchor vocabularies guarantee drift; reuse keeps docs / help / search / AI overlays parsing the same tokens.                                 |

Each row carries reopen triggers. A support-bundle finding that
a surface rendered `fully_indexed` while the indexer reported
`partial_index`, a parity-audit finding that palette and full
search disagree on `result_truth_class`, an AI-overlay finding
that a derived explanation rendered without a citation anchor, a
deep-link finding that a bookmark silently followed a rename
without disclosure, a semantic-service finding that lexical-only
was unreachable during an outage, or a support-export finding
that a required parity field was missing reopens the relevant
row.

## Consequences

- **Frozen:** the source-of-truth ownership table — eight named
  owners (palette command registry, search indexer, symbol-jump
  resolver, graph authority, docs-pack registry, derived-
  explanation session, support-export pipeline, deep-link
  resolver) — and the rule forbidding copy-only shadow
  surfaces.
- **Frozen:** the readiness-state vocabulary (`not_indexed`,
  `hot_set_ready`, `partial_index`, `warm_index`,
  `fully_indexed`, `stale_index`, `reindexing`,
  `index_unavailable`) and the rule that every non-
  `fully_indexed` readiness carries a typed stale-or-partial
  explanation.
- **Frozen:** the result-truth-class vocabulary (`exact`,
  `imported`, `heuristic`, `hybrid`) and the admissible-owner
  rules (`graph_authority` / `search_indexer` /
  `palette_command_registry` / `symbol_jump_resolver` may emit
  `exact`; `heuristic` and `hybrid` MUST carry a partial-truth
  disclosure; `imported` MUST cite an authoritative anchor).
- **Frozen:** the ranking-reason-class vocabulary (seventeen
  classes) and the rule that every row enumerates every class
  that contributed.
- **Frozen:** the hidden-scope vocabulary (ten reasons) and the
  typed disclosure with a count, an approximate flag, and at
  least one reason.
- **Frozen:** the partial-truth-cause vocabulary (eleven
  causes) and the rule that surfaces quote the frozen
  human-readable copy verbatim.
- **Frozen:** the semantic-fallback-state vocabulary (five
  states) and the lexical-only guarantee.
- **Frozen:** the deep-link-drift-state vocabulary (eleven
  states) and the remap-chain rule for `resolved_remapped`
  links.
- **Frozen:** the `search_result_packet_record` shape (with
  required parity fields enumerated), the
  `search_session_record` shape, the `search_deep_link_record`
  shape, and the rule that chip collapsing is a UI freedom but
  record addressability is mandatory.
- **Frozen:** the per-surface projection requirements for the
  command palette, full search, symbol jump, docs search,
  graph overlay, AI explanation overlay, and support export.
- **Frozen:** the audit-event ids on the `search` audit stream
  and the denial-reason set. Silent downgrade to an empty
  result set is forbidden; denials fail closed.
- **Frozen:** process-boundary constraints. Raw query bodies,
  raw document bodies, raw symbol definitions, and raw URLs
  never cross RPC; search records cross as typed payloads.
- **Frozen:** the schema of record is Rust types in the
  eventual search / navigation crate; the boundary schema
  lives at `schemas/search/search_result_truth.schema.json`;
  no external IDL or codegen toolchain at this milestone.
- **Permitted:** adding a new readiness state, truth class,
  ranking-reason class, hidden-scope reason, drift state,
  partial-truth cause, semantic-fallback state, surface
  class, audit-event id, or denial reason is additive-minor
  with a schema bump. Repurposing any existing value is
  breaking and requires a new decision row.
- **Permitted:** admin policy MAY narrow which source classes
  contribute to ranking on a surface, narrow semantic
  scoring, raise the freshness floor, force a step-up
  authenticator on a browser handoff from a docs-search row,
  or quarantine a pack. Policy MAY NOT silently widen beyond
  the frozen rules.
- **Permitted:** surfaces MAY collapse multiple axes into a
  single chip for dense rendering, provided the underlying
  packet retains each axis as a separately addressable
  field.
- **Follow-up:** the ranking, graph-overlay, docs-search,
  AI-explanation-citation, bookmark / session-restore, and
  support-export lanes instrument against this vocabulary
  before claiming search parity.
- **Follow-up:** the docs-browser lane (ADR-0013) consumes
  the `citation_anchor_refs` on `imported` and
  `docs_search` rows.
- **Follow-up:** the session-restore / bookmark storage lane
  fills in the `search_deep_link_record` persistence.
- **Ratifies:** the ADR-0013 `citation_anchor_record` is the
  only anchor vocabulary search surfaces quote. The ADR-0011
  `freshness_class`, `client_scope`, and `redaction_class`
  vocabularies are re-exported without modification. The
  ADR-0009 workset / scope vocabulary is re-exported as the
  search `scope_filter_class`. The ADR-0008 "one registry,
  no shadows" rule for settings and the ADR-0013 "no shadow
  docs" rule are mirrored here as the "no shadow search"
  rule.

## Alternatives considered

- **Per-surface result records.** Rejected: per-surface
  records recreate the palette / full-search / symbol-jump /
  docs-search drift this ADR is closing; a parity audit
  becomes a hand-mapping exercise rather than a mechanical
  comparison.
- **Free-form "still indexing" / "some results hidden"
  copy.** Rejected: free-form copy is not machine-readable;
  parity audits, support exports, and AI overlays cannot
  consume it. The typed readiness / partial-truth-cause /
  hidden-scope vocabularies make each state enumerable and
  repairable.
- **One generic `confidence_score` float per row.**
  Rejected: collapses exact / imported / heuristic / hybrid
  truth classes, hides which rows can be authoritative, and
  forces per-surface rules ("if confidence > 0.8 then
  ...") to be re-derived on every surface.
- **Mint a search-specific freshness vocabulary.**
  Rejected: guarantees drift between the lifecycle ADR and
  the search vocabulary. Re-exporting the ADR-0011 axes
  keeps one vocabulary across the protected surfaces.
- **Mint a parallel citation anchor for search.** Rejected:
  ADR-0013 already freezes `citation_anchor_record`;
  minting a parallel shape would force every redaction pass
  and every audit consumer to learn two shapes. Quoting the
  existing record is sufficient.
- **Allow palette / symbol-jump to fail when semantic is
  unavailable.** Rejected: the PRD MUST is explicit — the
  product MUST become useful before the full repo is
  indexed. The `semantic_unavailable_lexical_only` state
  pins this guarantee into the record.
- **Silent follow of renamed / moved deep links.**
  Rejected: silent follow hides drift from the user and
  from session-restore parity audits. The typed
  `resolved_remapped` state with a `remap_chain_refs`
  array makes the drift addressable.
- **External IDL + generator for search records.**
  Rejected: same argument ADR 0004 through ADR 0013 make —
  an IDL without a second-language consumer costs more than
  it buys; the JSON Schema export reserves the integration
  point.
- **Defer to a later milestone.** Rejected: the
  default-if-unresolved narrowing on `D-0020` (no shared
  readiness / truth / ranking-reason / hidden-scope /
  deep-link-drift / partial-index / semantic-fallback
  vocabulary) would force the ranking, graph-overlay, docs-
  search, AI-explanation-citation, bookmark / session-
  restore, support-export, and palette / symbol-jump lanes
  to land with incompatible assumptions that downstream
  tooling could not reconcile.

The `D-0020` `narrow` default-if-unresolved posture would
have locked the product to per-surface, hand-maintained
search badges with free-form "still indexing" copy, no
shared readiness axis, no typed truth-class set, no
ranking-reason enumeration, no hidden-result count
contract, no deep-link drift vocabulary, no partial-index
disclosure, no semantic fallback axis, and no parity-audit
contract. Accepting this ADR replaces that narrowing with
the readiness vocabulary, the result-truth classes, the
ranking-reason enumeration, the hidden-scope vocabulary,
the partial-truth-cause set, the semantic-fallback state
set, the deep-link drift states, the packet / session /
link records, the per-surface projection requirements,
the audit-event list, and the denial posture above; the
narrowing default does not apply.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — "the product MUST become
  useful before the full repo is indexed. Hot files,
  changed files, current build targets, and nearby tests
  get priority"; normative MUST / SHOULD language on
  search public truth and partial-scope labelling.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  readiness vocabulary (`Loaded`, `ManifestKnown`,
  `Cached`, `Unavailable`); warm / cold / warm-index
  posture; degradation rules (`semantic-service restarts,
  partial indexing, and stale-result labeling are allowed,
  but silent contradiction is not`).
- `.t2/docs/Aureline_Technical_Design_Document.md` — "operations
  that cannot see the full relevant scope MUST label partial
  truth rather than pretending global completeness";
  search-session lane seeds (`bookmark remap/drift corpora`
  across session restore, branch drift, sparse scope);
  hidden-result count posture.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — result-truth
  provenance ("every graph-backed claim must distinguish
  exact graph fact, imported fact, heuristic summary, and
  fallback structure"); scope-banner rules (`Current repo`,
  `Selected workset`, `Full workspace`, `Remote cache`,
  `Outside current scope`); `No results` vs `No results
  in this workset` vs `Index not built for excluded roots`
  vs `Blocked by trust or policy`; breadcrumb honesty
  rules under stale / incomplete symbol data.
- `.t2/docs/Aureline_Milestones_Document.md` — "ranking-
  reason packet + partial-index drill + deep-link remap
  corpus" milestone deliverable posture.
- `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
  — subscription envelope with authority class and
  freshness hint re-exported here.
- `docs/adr/0009-execution-context-and-scope.md` — workset
  / scope vocabulary re-exported as the search
  `scope_filter_class`.
- `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  — `freshness_class`, `client_scope`, and
  `redaction_class` vocabularies re-exported here.
- `docs/adr/0013-docs-help-service-health-truth.md` —
  `citation_anchor_record` shape quoted here; no-shadow-
  docs rule mirrored as no-shadow-search.

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0020`
- RFC: none.
- Boundary schema:
  `schemas/search/search_result_truth.schema.json`
- Vocabulary + copy-guidance doc:
  `docs/search/search_readiness_vocabulary.md`
- Worked-example corpus:
  `artifacts/search/result_truth_labels.yaml`
- Fixture corpus:
  `fixtures/search/result_truth_examples/*.json`
- Citation anchor vocabulary consumed:
  `docs/adr/0013-docs-help-service-health-truth.md` and
  `schemas/docs/help_status_badge.schema.json`.
- Lifecycle / freshness / client-scope vocabulary this ADR
  re-exports:
  `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  and `schemas/governance/capability_lifecycle.schema.json`.
- Workset / scope vocabulary re-exported as
  `scope_filter_class`:
  `docs/adr/0009-execution-context-and-scope.md` and
  `schemas/runtime/execution_context.schema.json`.
- Affected lanes: `governance_lane:product_scope_review`,
  `governance_lane:docs_public_truth`,
  `governance_lane:support_export`,
  `governance_lane:design_system_seeds`.

## Supersession history

First acceptance. No supersession.
