# Search, graph, and docs support-export parity — stable contract

Status: Stable lane proof for the support-export parity of search,
graph, docs, retrieval-debug, operator-truth inspector, and query-session
export packets.

This document is the reviewer-facing contract for the stable
support-export parity truth packet. The packet is the single source
of truth that the search shell, graph topology canvas, docs/help,
AI context, review workspace, CLI/headless inspector, support export,
and the release proof index all read; surfaces MUST NOT mint local
copies or paraphrase export posture.

## What the packet asserts

For each governed *lane class × export packet* row, the packet asserts:

1. The **lane class** — one of `search_export`,
   `graph_topology_export`, `docs_handoff_export`,
   `operator_truth_inspector`, `retrieval_debug`,
   `query_session_export`. Every certified packet MUST carry at
   least one row for each of the six required lane classes.
2. The **export packet class** — one of
   `search_collection_snapshot`, `graph_topology_snapshot`,
   `docs_handoff_packet`, `operator_truth_packet`,
   `retrieval_inspector_packet`, `query_session_packet`. The
   validator rejects rows whose packet class does not carry the
   declared lane.
3. The **query-session ref** — every row preserves a
   `query_session_id_ref` so AI context assembly, docs lookup,
   CLI/headless search, and support replay all reuse the same
   query-session object instead of inventing private candidate lists.
4. The **count summary** — visible/selected/included rows plus
   `omitted_result_count`, `hidden_by_current_scope_rows`,
   `hidden_by_policy_rows`, `hidden_by_remote_cache_rows`, and a
   `count_is_partial` flag. The validator rejects internally
   inconsistent summaries (selected > visible, or
   `omitted_flag` disagreeing with `omitted_result_count`).
5. The **redaction class** — one of `literal_local_only`,
   `hashes_scope_and_result_refs`,
   `metadata_only_no_query_material`, `policy_withheld`, or
   `explicit_literal_consent`. The validator rejects
   `literal_local_only` and `explicit_literal_consent` as the
   default export class: those classes only apply with an explicit
   user opt-in.
6. The **live-vs-captured class** — one of `current_live_results`,
   `captured_snapshot`, `live_rerun_required`,
   `scope_changed_since_capture`, `empty_because_scope_changed`.
7. The **downgrade state** — one of `none`,
   `narrowed_query_redacted`, `truncated_results`,
   `policy_withheld`, `provider_unavailable`,
   `scope_changed_since_capture`.
8. The **confidence class** — `high`, `medium`, `low`, or
   `heuristic`.
9. The **disclosure ref** — every row carries a repo-relative
   reference to the disclosure shown to the user, plus at least one
   `evidence_refs` entry that proves the export claim.

### Operator-truth inspector rows

Operator-truth inspector rows MUST carry an
`operator_reconstruction_proof` block with:

- `reconstruction_id` — stable id for the reconstruction.
- `why_row_existed_ref` — repo-relative ref a reviewer can read to
  understand why the row existed in the original view.
- `what_was_hidden_ref` — repo-relative ref explaining what was
  hidden or withheld from the row.
- `approximate_count_disclosure_ref` — repo-relative ref explaining
  how counts were approximated.
- `raw_query_text_excluded` — boolean; MUST be true. Raw query text
  is never reconstructed by the inspector.

The validator rejects an operator-truth row that drops any of these
fields with `operator_truth_missing_reconstruction`.

### Query-session export rows (deep links and handoff packets)

Query-session export rows MUST carry a `deep_link_scope_binding`
block with:

- `deep_link_intent_ref` — repo-relative ref to the saved search
  intent (query classification + scope binding, never literal text).
- `scope_metadata_ref` — repo-relative ref to the scope metadata the
  link reopens against.
- `requires_recipient_resolution` — boolean; MUST be true. The
  recipient resolves the link against their own permissions, trust
  posture, and current workspace state.
- `frozen_certainty_excluded` — boolean; MUST be true. The deep
  link MUST NOT carry frozen certainty about current results.

The validator rejects rows that drop the binding with
`query_session_missing_deep_link_binding`, that drop the intent or
scope metadata refs with `deep_link_drops_scope_metadata`, or that
flip either boolean to false with
`deep_link_freezes_recipient_certainty`.

## Boundary safety

Every row carries `raw_query_text_excluded`, `secrets_excluded`, and
`ambient_authority_excluded`. The validator emits
`raw_query_text_present`, `secrets_present`, or
`ambient_authority_present` as a blocker for any row that flips one of
those booleans to false. Search export packets MUST default to
`hashes_scope_and_result_refs`, `metadata_only_no_query_material`, or
`policy_withheld`; the validator emits
`default_export_too_permissive` if a row defaults to literal export.

## Closed vocabulary

**Lane classes** — `search_export`, `graph_topology_export`,
`docs_handoff_export`, `operator_truth_inspector`,
`retrieval_debug`, `query_session_export`.

**Export packet classes** — `search_collection_snapshot`,
`graph_topology_snapshot`, `docs_handoff_packet`,
`operator_truth_packet`, `retrieval_inspector_packet`,
`query_session_packet`.

**Redaction classes** — `literal_local_only`,
`hashes_scope_and_result_refs`,
`metadata_only_no_query_material`, `policy_withheld`,
`explicit_literal_consent`.

**Live-vs-captured classes** — `current_live_results`,
`captured_snapshot`, `live_rerun_required`,
`scope_changed_since_capture`, `empty_because_scope_changed`.

**Downgrade states** — `none`, `narrowed_query_redacted`,
`truncated_results`, `policy_withheld`, `provider_unavailable`,
`scope_changed_since_capture`.

**Consumer surfaces** — `search_shell`, `graph_topology`,
`docs_help`, `ai_context`, `review_workspace`, `cli_headless`,
`support_export`, `release_proof_index`.

**Finding kinds** — see `schemas/search/support_export_parity_truth.schema.json`
for the closed list. Notable invariants:

- `raw_query_text_present`, `secrets_present`,
  `ambient_authority_present` — boundary safety.
- `default_export_too_permissive` — default redaction class is
  literal/consent-only.
- `operator_truth_missing_reconstruction` — operator-truth row
  drops its reconstruction proof.
- `query_session_missing_deep_link_binding`,
  `deep_link_drops_scope_metadata`,
  `deep_link_freezes_recipient_certainty` — query-session deep
  links violate the scope-metadata-only invariant.
- `missing_consumer_projection`, `consumer_projection_drift`,
  `redaction_vocabulary_collapsed`,
  `live_vs_captured_vocabulary_collapsed`,
  `downgrade_vocabulary_collapsed`, `query_session_refs_dropped`,
  `lane_vocabulary_collapsed`,
  `export_packet_class_vocabulary_collapsed` — surface drift.

## Required consumer projections

The packet REQUIRES one preserved projection per surface:
`search_shell`, `graph_topology`, `docs_help`, `ai_context`,
`review_workspace`, `cli_headless`, `support_export`, and
`release_proof_index`. Each projection MUST keep the lane class,
export-packet class, redaction class, live-vs-captured class,
downgrade state, and query-session refs verbatim, MUST support JSON
export, and MUST exclude raw private material and ambient authority.

## Companion artifacts

- Schema: `schemas/search/support_export_parity_truth.schema.json`
- Checked-in packet: `artifacts/search/m4/support_export_parity_truth_packet.json`
- Fixture corpus: `fixtures/search/m4/support_export_parity_truth_packet/`
- Reviewer artifact: `artifacts/search/m4/ship-search-graph-and-docs-support-export-parity.md`
- Rust contract: `crates/aureline-graph/src/support_export_parity_truth_packet/mod.rs`
- Replay tests: `crates/aureline-graph/tests/support_export_parity_truth_packet.rs`

Related contracts:

- `schemas/search/search_export_snapshot.schema.json` — the
  search-export snapshot contract this packet binds.
- `schemas/search/query_session.schema.json` — the canonical
  query-session contract referenced by `query_session_id_ref`.
- `schemas/search/retrieval_inspector.schema.json` — the retrieval
  inspector packet shape the `retrieval_debug` lane carries.
- `schemas/search/search_operator_truth_packet.schema.json` — the
  operator-truth packet shape the `operator_truth_inspector` lane
  carries.
- `docs/support/operator_truth_packet.md` — the operator-truth
  packet reconstruction contract.

## Anchored normative sources

- `.t2/docs/Aureline_Technical_Architecture_Document.md` — search,
  graph, and docs export-honesty rules.
- `.t2/docs/Aureline_Technical_Design_Document.md` — query-session
  retention and deep-link reopen rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — operator-truth
  inspector reconstruction rules and export-export parity rules.
- `.t2/docs/Aureline_PRD.md` Appendix P FR-SEARCH-002 — search-row
  scope/freshness/provenance disclosure rules.

If any of those sources disagree with this document, the source wins
and this document plus the schema and fixtures MUST be updated in the
same change.

#search-export

The `search_export` lane row preserves the export of a search
collection snapshot: query-session ref, planner-pass ref, selected
result refs, included result refs, count summary, redaction class
(default `hashes_scope_and_result_refs`), and live-vs-captured class.
The default redaction posture excludes raw query text and provider
filters from support exports.

#graph-topology

The `graph_topology_export` lane row preserves the export of a graph
topology snapshot: included node and edge refs, count summary,
redaction class (`metadata_only_no_query_material`), and any
`truncated_results` downgrade.

#docs-handoff

The `docs_handoff_export` lane row preserves the export of a docs
handoff packet: selected and included docs refs, count summary, and
redaction class. Docs handoffs are typically `current_live_results`
because docs lookups are deterministic against current docs packs.

#operator-truth

The `operator_truth_inspector` lane row preserves the operator-truth
inspector reconstruction. The row carries a reconstruction proof so
a reviewer can explain why a row existed, what was hidden, and how
counts were approximated, without exposing raw queries by default.

#operator-truth-why

`why_row_existed_ref` points to the evidence ref a reviewer reads to
understand the row's source signal (planner pass, lane combination,
scope binding).

#operator-truth-hidden

`what_was_hidden_ref` points to the evidence ref explaining what was
hidden by policy, scope, or remote unavailability.

#operator-truth-counts

`approximate_count_disclosure_ref` points to the evidence ref
explaining how counts were approximated when the underlying summary
is partial.

#retrieval-debug

The `retrieval_debug` lane row preserves the retrieval inspector
packet's lane contributions (lexical, vector, graph, fused),
readiness, freshness, embedder identity, route policy, and
provenance anchors without raw vectors or source bodies.

#query-session

The `query_session_export` lane row preserves a query-session deep
link. The deep link is metadata-only: a saved intent ref plus scope
metadata. The recipient re-resolves the link against their own
permissions, trust posture, and current workspace state; the link
never freezes recipient certainty about current results.
