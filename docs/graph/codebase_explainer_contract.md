# Cited codebase explainer, omission ledger, and open-evidence action contract

This document is the cross-tool contract every codebase-explainer
surface reads when it explains a node, an edge, a query, a topology
view, an impact view, or an ownership card with cited evidence.
Surfaces covered: review-pack explainer overlay, AI evidence-explainer
panel, support-bundle explainer packet, public query-family explainer,
navigation-deep-link explainer, and the IDE inline-explainer overlay.

The machine-readable schema lives at:

- [`/schemas/graph/codebase_explainer_packet.schema.json`](../../schemas/graph/codebase_explainer_packet.schema.json)

Companion fixtures live under:

- [`/fixtures/graph/codebase_explainer_cases/`](../../fixtures/graph/codebase_explainer_cases/)

This contract is normative. It layers on top of the workspace-graph
seed
([`/docs/graph/workspace_graph_seed.md`](workspace_graph_seed.md)),
the provenance / confidence / source-anchor-drift contract
([`/docs/graph/provenance_and_confidence_contract.md`](provenance_and_confidence_contract.md)),
and the topology-map / impact-explorer / ownership-card contract
([`/docs/graph/topology_and_impact_contract.md`](topology_and_impact_contract.md)).
It does not restate identity rules, freshness / confidence / anchor-
drift rules, or topology / impact / ownership rules. Where this
document disagrees with the seed, the seed wins on identity rules
and this document is updated in the same change. Where it disagrees
with the provenance / confidence contract, that contract wins on
freshness, confidence, and anchor drift. Where it disagrees with the
topology / impact contract, that contract wins on loaded-scope
state, hidden counts, and topology / impact / ownership identity.
Where a downstream explainer surface mints its own claim, citation,
or omission vocabulary, this document wins and the surface is
non-conforming.

## Why freeze this now

A codebase explainer that renders a sentence like "buffer mutation
flushes through the VFS save queue" without citing the file range,
the symbol, or the topology edge that proves it has already lied
once. An explainer that omits the imported-vendor crate that
contributed the helper — and renders the prose as if the helper were
local — has lied a second time. An explainer that compresses
"three covered ownership edges had their anchors deleted, two
inferred-transitive impact rows are AI-assisted narrative, and one
helper lives in a hidden policy projection" into the single phrase
"some results are partial" has lied a third time. An explainer
whose "Open in source" action launches a string search instead of
resolving the source_anchor_slot the citation already pinned has
lied a fourth time.

Freezing the cited explainer packet, the typed claim disposition,
the typed omission ledger, and the closed open-evidence action
vocabulary in machine-readable form makes those four honesty
contracts enforceable rather than aspirational. The same node, the
same edge, the same imported root, and the same drifted anchor render
the same chips on the explainer panel that they render on the
topology map and the impact explorer because every surface reads the
same identity layer and the same provenance / confidence contract.

The second hazard is divergence between explainer surfaces. Review-
pack explainers, AI evidence panels, support-bundle packets, and the
public query-family explainer have historically rendered three
private claim vocabularies for "this is what we are saying", three
private vocabularies for "this is what we know", and three private
phrases for "we omitted some things". Pinning the claim-disposition
vocabulary, the omission-class vocabulary, the citation-anchor
vocabulary, and the export-format vocabulary in this document makes
the vocabularies one.

## Scope

- Freeze one `codebase_explainer_packet_record` covering subject,
  active scope, loaded-scope state, freshness/provenance strip,
  claim rows, omission ledger, open-evidence actions, hidden-count
  summary, supporting-artifact refs, review handoffs, and export
  hooks.
- Freeze one `claim_row_record` covering claim class, claim
  disposition, claim text, citation refs, rolled-up confidence /
  floor reason / conflict state, AI-inference segment marker,
  AI-evidence packet ref, omission-ledger entry ref, and supporting
  artifacts.
- Freeze one `citation_ref_entry` reusing the seed's
  `explainer_citation_slot.citation_class`, `source_anchor_slot`
  identity, `graph_node_record.node_id`, and `graph_edge_record.edge_id`
  verbatim.
- Freeze one `omission_ledger_entry` covering omission class,
  omission text, subject count, imported-root status, anchor-drift
  state, policy view ref, supporting artifacts, and the
  export-preservation flag.
- Pin the closed `claim_class`, `claim_disposition`,
  `omission_class`, `offered_action_class`, `export_format_class`,
  and `review_handoff_class` vocabularies so the same packet drives
  every explainer surface and every export hook without renaming
  identity.
- Reuse `graph_node_record.node_id`, `graph_edge_record.edge_id`,
  `source_anchor_slot` identity, `provenance_stamp_record.stamp_id`,
  `confidence_rollup_record.rollup_id`,
  `source_anchor_drift_record.drift_record_id`,
  `topology_map_view_record.view_id`,
  `impact_explorer_view_record.view_id`, and
  `ownership_card_record.card_id` verbatim.

## Out of scope

- AI-generated explainers as a feature. The
  `narrative_ai_assisted` disposition, the `ai_inference_segment`
  marker, and the `ai_evidence_packet_ref` requirement are how the
  schema *handles* AI-assisted segments when they exist; the
  upstream AI inference pass that produces the segment is a separate
  workstream.
- Ranking logic for which evidence to cite. The schema requires
  cited evidence; the producer that picks *which* citations to attach
  to a claim is a separate decision and outside this contract.
- Final explainer UI composition (canvas treatment, typography,
  motion, or interaction polish). The legend names which axes the
  surface MUST disclose; the visual treatment is a UX decision
  separate from this schema.
- Graph topology query planner, the indexing engine, and the
  cited-explainer renderer. Those land later; this contract freezes
  only the record shapes a renderer reads.

## 1. Codebase explainer packet

Every codebase-explainer surface emits exactly one
`codebase_explainer_packet_record`. The packet answers ten questions
in one shape:

| Field                          | Question answered                                                               |
|--------------------------------|---------------------------------------------------------------------------------|
| `explained_subject`            | What is this packet about (node / edge / query / topology view / impact view / ownership card)? |
| `active_scope_class` + `active_scope_ref` | Which workset / scope artifact was the explainer drawn under?       |
| `loaded_scope_state`           | Is the loaded set fully loaded, partial, sparse, policy-limited, imported-only, warming, stale, or evidence-missing? |
| `freshness_provenance_strip`   | Per-entry freshness / extraction-or-inference mode / imported-root status / rolled-up confidence / anchor-drift state. |
| `claim_rows`                   | One or more typed claims, each cited or explicitly downgraded / omitted / AI-assisted. |
| `omission_ledger`              | First-class typed omission entries; never compressed into "partial results". |
| `hidden_count_summary`         | Hidden-by-workset / hidden-by-policy / not-loaded / evidence-missing / imported-root / omission / AI-segment / refusal counts. |
| `offered_actions`              | Open-evidence actions that resolve through stable graph and source-anchor ids. |
| `offered_review_handoffs`      | Review-pack, support-bundle, AI-evidence packet, topology, impact, and ownership-card jumps. |
| `offered_export_hooks`         | Per-format declarations of which chips and which ledger survive the export. |

Plus pointer fields linking the packet to the matching topology /
impact / ownership surfaces (`topology_view_ref`, `impact_view_ref`,
`ownership_card_ref`) so the same node renders the same chips in
every direction.

### 1.1 The `claim_rows[]` layer is authoritative

The packet MAY render an optional `summary_text` for top-of-pane
prose, but the summary MUST NOT introduce a claim that is not also
asserted (or omitted) on `claim_rows[]`. The row layer is the
authoritative claim layer. A surface that renders unsupported prose
in the summary while leaving the row layer empty is non-conforming.

### 1.2 The `freshness_provenance_strip[]` is required

`freshness_provenance_strip[]` MUST carry `minItems: 1`. The strip
rides every claim row and the packet header so the same node renders
the same freshness / confidence / anchor-drift / imported-root chips
on the explainer that it renders on the topology map, the impact
explorer, and the ownership card.

### 1.3 Loaded-scope state is honest

A packet that narrowed the scope MUST NOT carry `loaded_scope_state
== fully_loaded`. The schema's `allOf` rules force every
`hidden_count_summary` count field to `0` when `fully_loaded` and
force `imported_root_count >= 1` when `imported_root_only_loaded`.

## 2. Claim rows

Every claim is one `claim_row_record`. Each row answers four
questions:

| Field                | Question answered                                                                  |
|----------------------|------------------------------------------------------------------------------------|
| `claim_class`        | Which axis of the codebase does this claim describe (subject / relation / lineage / ownership / impact / imported-root / anchor-drift / missing-evidence / workset-scope / runtime-role / dependency / generated-artifact / policy-projection / AI-inference segment / refusal)? |
| `claim_disposition`  | Is this claim cited, partially cited (imported / inferred / drifted / outside-workset), AI-assisted narrative, a typed downgrade / refusal, or an omission carried for completeness? |
| `claim_text`         | The verbatim sentence the surface renders. For downgraded / omitted dispositions the text names the refusal / omission instead of asserting the claim. |
| `citation_refs[]`    | Zero or more typed citations whose ids reuse the seed's identity layer verbatim.   |

### 2.1 `claim_class`

Closed vocabulary. Each row names exactly one class so reviewers
can dispatch on it without parsing prose. The vocabulary covers
the axes the spec names: `describes_subject`, `describes_relation`,
`describes_lineage`, `describes_ownership`, `describes_impact`,
`describes_imported_root`, `describes_anchor_drift`,
`describes_missing_evidence`, `describes_workset_scope`,
`describes_runtime_role`, `describes_dependency`,
`describes_generated_artifact`, `describes_policy_projection`,
`describes_ai_inference_segment`, `describes_refusal`.

### 2.2 `claim_disposition`

Closed vocabulary. Each row carries exactly one disposition. The
schema's `allOf` rules pair the disposition to the citation count,
the AI-inference flag, the imported-root status, and the anchor-
drift state:

| Disposition                                   | Required gates                                                                |
|-----------------------------------------------|-------------------------------------------------------------------------------|
| `cited_direct_evidence`                       | `citation_refs.minItems = 1`; `ai_inference_segment = false`.                 |
| `cited_partial_imported`                      | At least one citation with `imported_root_status != not_imported`; confidence caps at `medium`; export hook with `preserves_imported_root_chip = true`. |
| `cited_partial_inferred`                      | At least one citation; confidence MAY cap at `medium`; surface renders the inferred chip. |
| `cited_with_anchor_drift`                     | At least one citation with `anchor_drift_state != anchor_present_no_drift`; export hook with `preserves_anchor_drift_chip = true`. |
| `cited_outside_active_workset`                | Cited but the citation lies outside the active workset / scope; the loaded-scope note discloses the narrowing. |
| `narrative_ai_assisted`                       | `ai_inference_segment = true`; `ai_evidence_packet_ref` non-null; confidence caps at `medium`; export hook with `preserves_ai_inference_segment_chip = true`. |
| `downgraded_refusal_insufficient_evidence`    | `citation_refs` MAY be empty; `omission_ledger_entry_ref` non-null; the omission ledger names why; `claim_text` names the refusal, not the underlying claim. |
| `downgraded_refusal_policy_blocked`           | Same as above; the omission entry's `policy_view_ref` resolves the policy-side review handoff. |
| `downgraded_refusal_evidence_missing`         | Same as above; the omission entry names the missing-evidence axis. |
| `omitted_outside_workset`                     | Carried for completeness; `omission_ledger_entry_ref` non-null. |
| `omitted_policy_hidden`                       | Same; the omission entry's `omission_class = hidden_by_policy_projection`.    |
| `omitted_evidence_missing`                    | Same; the omission entry names the missing-evidence axis.                     |
| `omitted_imported_unverified`                 | Same; the omission entry's `omission_class = imported_root_unverified`.       |
| `omitted_ai_inference_segment_redacted`       | Same; the omission entry's `omission_class = ai_inference_segment_redacted`.  |

The `cited_*` dispositions are gated on `citation_refs.minItems = 1`
by the schema. The `downgraded_refusal_*` and `omitted_*`
dispositions are gated on a non-null `omission_ledger_entry_ref` so
the gap is reviewable. `narrative_ai_assisted` is gated on
`ai_inference_segment = true`, a non-null `ai_evidence_packet_ref`,
and `rolled_up_confidence in {medium, low, unknown}`.

### 2.3 Confidence chip on claims

Each row carries `rolled_up_confidence`, `floor_reason`, and
`conflict_state` (re-export of the rollup vocabulary). The trio is
the per-claim "confidence class" the spec names: a single rolled-up
level plus the typed reason the rollup landed there plus the typed
conflict (if any). The optional `confidence_rollup_ref` points at
the `confidence_rollup_record` so a reviewer can replay the rollup
deterministically.

## 3. Citation refs

Every citation is one `citation_ref_entry`. Each entry reuses the
seed's identity layer verbatim:

- `citation_class` — re-export of
  `explainer_citation_slot.citation_class` (`symbol_definition`,
  `file_range`, `doc_entry`, `generated_artifact_lineage`,
  `codeowners_rule`, `topology_edge`, `provider_resource`,
  `mutation_journal_entry`, `imported_bundle_entry`,
  `replay_capture_entry`).
- `subject_kind` — `graph_node_record`, `graph_edge_record`,
  `explainer_citation_slot`, `source_anchor_slot`,
  `supporting_artifact_ref`, `topology_map_view_record`,
  `impact_explorer_row_record`, or `ownership_card_record`.
- `citation_ref` — opaque ref reused verbatim from the underlying
  record's id.
- Optional `node_id_ref`, `edge_id_ref`, and `source_anchor_ref`.
- `freshness_class`, `imported_root_status`, `anchor_drift_state`,
  `rolled_up_confidence`, and a required `provenance_stamp_ref`.

The schema's `allOf` rules enforce two invariants per citation:

1. `anchor_drift_state != anchor_present_no_drift` MUST set
   `anchor_drift_record_ref` to a non-null opaque ref so the drift
   event is auditable.
2. `freshness_class != authoritative` MUST carry a typed
   `stale_reason`.

Open-evidence actions resolve through the citation entry's
`source_anchor_ref`, `node_id_ref`, `edge_id_ref`, or
`citation_ref`; never through an ambiguous string search.

## 4. Omission ledger

Every disclosed gap is one `omission_ledger_entry`. The ledger is
the spec's required disclosure that imported roots, outside-workset
facts, unavailable edges, and AI-assisted excluded segments do not
silently disappear.

### 4.1 `omission_class`

Closed vocabulary. The vocabulary covers each axis the spec names:

| Class                                          | Disclosed axis                                                              |
|------------------------------------------------|-----------------------------------------------------------------------------|
| `outside_active_workset`                       | The subject lies outside the active workset.                                |
| `outside_active_scope`                         | The subject lies outside the active sparse / policy / review scope.         |
| `hidden_by_policy_projection`                  | The subject is hidden by a policy projection; `policy_view_ref` resolves the policy-side review handoff. |
| `imported_root_unverified`                     | An imported root contributor was not cross-validated.                       |
| `imported_root_not_loaded`                     | An imported root is not currently loaded.                                   |
| `evidence_missing_anchor_deleted`              | A covered anchor was deleted with no replacement.                           |
| `evidence_missing_anchor_drift`                | A covered anchor drifted (moved / regenerated / imported / unknown).        |
| `evidence_missing_lineage_pending`             | A generated-artifact lineage record is missing or pending.                  |
| `evidence_missing_replay_capture`              | A replay capture entry is missing.                                          |
| `downgraded_refusal_insufficient_evidence`     | The claim was refused because cited evidence was insufficient.              |
| `downgraded_refusal_policy_blocked`            | The claim was refused because a policy projection blocked the assertion.    |
| `downgraded_refusal_evidence_missing`          | The claim was refused because evidence was missing.                         |
| `ai_inference_segment_excluded`                | An AI-assisted segment was excluded (not asserted in prose).                |
| `ai_inference_segment_redacted`                | An AI-assisted segment was redacted on export.                              |
| `generated_artifact_lineage_unknown`           | A generated artifact's lineage is unknown.                                  |
| `dependency_resolution_unknown`                | A `depends_on` resolution could not be reconstructed.                       |
| `claim_blocked_redaction`                      | A claim was blocked by redaction policy.                                    |
| `workset_scope_narrowed_excluded`              | A subject was excluded by workset scope narrowing.                          |
| `warming_index_pending`                        | A producer is warming and the subject is not yet hydrated.                  |
| `stale_producer_skipped`                       | A stale producer was skipped.                                               |

### 4.2 Required preservation through export

`export_preservation_required = true` is forced on every
`downgraded_refusal_*` and `ai_inference_segment_*` omission and is
admissible on every other class. The packet's `offered_export_hooks`
MUST set `preserves_omission_ledger = true` on at least one entry
whenever any claim row carries a `downgraded_refusal_*` /
`omitted_*` disposition (enforced by the packet's `allOf`). The
ledger is never compressed into a generic "partial results" phrase
on export.

### 4.3 Subject counts cross the boundary; ids may not

`subject_count_known = true` requires a non-null integer count;
`subject_count_known = false` carries `null`. The exact subject ids
are admissible only when the underlying policy / workset permits;
otherwise the count crosses the boundary and the ids stay behind the
policy view.

## 5. Open-evidence actions

`offered_actions[]` MUST carry `minItems: 5` and MUST contain at
least the spec's minimum action set:

- `open_source_at_anchor` — resolves through a citation's
  `source_anchor_ref`. Never an ambiguous search.
- `open_docs_at_citation` — resolves through a citation whose
  `citation_class == doc_entry`.
- `inspect_graph_relation` — resolves through a citation's
  `edge_id_ref` or through a `cited_edge_id_refs[]` entry.
- `export_explainer_packet` — resolves through `packet_id`.
- `flag_missing_evidence` — resolves through an
  `omission_ledger_entry.omission_id`.

The full vocabulary covers `inspect_topology_view`,
`inspect_impact_view`, `inspect_ownership_card`,
`inspect_provenance_stamp`, `inspect_confidence_rollup`,
`inspect_anchor_drift_record`, `request_widen_scope_with_review`,
`request_policy_review_admin_only`, `open_review_pack`,
`open_support_bundle`, `open_ai_evidence_packet`, `copy_packet_id`,
`copy_claim_id`, `copy_citation_id`, `copy_omission_id`,
`copy_node_id`, and `copy_edge_id`.

## 6. Hidden-count summary

`hidden_count_summary` mirrors the topology / impact summaries and
adds three explainer-only axes:

| Axis                                              | Disclosed                                                       |
|---------------------------------------------------|-----------------------------------------------------------------|
| `omission_known` / `omission_count`               | Total `omission_ledger` rows; drives "N omitted axes disclosed".|
| `ai_inference_segment_known` / `ai_inference_segment_count` | Count of claim rows whose `ai_inference_segment = true`. |
| `downgraded_refusal_known` / `downgraded_refusal_count`     | Count of claim rows whose disposition is `downgraded_refusal_*`. |

The summary emits counts even when `*_known = false`; surfaces
render "unknown" rather than silently showing `0`.

## 7. Export hooks

Every entry in `offered_export_hooks[]` carries six explicit
preservation flags:

| Flag                                       | True when the export carries the chip / ledger verbatim.                        |
|--------------------------------------------|---------------------------------------------------------------------------------|
| `preserves_omission_ledger`                | Every omission entry survives.                                                  |
| `preserves_citation_anchors`               | Per-citation `citation_ref` / `source_anchor_ref` / `line_range` survives.      |
| `preserves_confidence_chip`                | `rolled_up_confidence` + `floor_reason` + `conflict_state` survives.            |
| `preserves_imported_root_chip`             | Per-citation `imported_root_status` survives.                                   |
| `preserves_anchor_drift_chip`              | Per-citation `anchor_drift_state` survives.                                     |
| `preserves_ai_inference_segment_chip`      | Per-claim `ai_inference_segment` plus `ai_evidence_packet_ref` survives.        |

| Format                              | Lossless of the ledger and chips |
|-------------------------------------|----------------------------------|
| `review_pack_explainer_packet`      | yes                              |
| `support_bundle_explainer_packet`   | yes                              |
| `ai_evidence_explainer_packet`      | yes                              |
| `json_explainer_snapshot`           | yes                              |
| `yaml_explainer_snapshot`           | yes                              |
| `markdown_explainer_brief`          | no — surface MUST disclose the loss |
| `csv_claim_table`                   | no — surface MUST disclose the loss |

The packet's `allOf` rules force at least one export hook to set
`preserves_omission_ledger = true` whenever any claim row carries a
`downgraded_refusal_*` or `omitted_*` disposition; force at least
one hook to set `preserves_imported_root_chip = true` whenever any
citation is imported; force at least one hook to set
`preserves_anchor_drift_chip = true` whenever any citation has anchor
drift; force at least one hook to set
`preserves_ai_inference_segment_chip = true` whenever any claim's
`ai_inference_segment = true`.

## 8. Surface rules

These rules apply to every surface that renders, logs, exports, or
reasons about the cited codebase explainer.

1. **No surface invents private claim, citation, or omission
   identity.** Every `claim_id`, `citation_id`, and `omission_id` is
   stable per packet; every `citation_ref`, `node_id_ref`,
   `edge_id_ref`, `source_anchor_ref`, `provenance_stamp_ref`,
   `confidence_rollup_ref`, `anchor_drift_record_ref`,
   `topology_view_ref`, `impact_view_ref`, and `ownership_card_ref`
   reuses the upstream id verbatim.
2. **No surface invents private claim or omission vocabularies.**
   Surfaces render `claim_class`, `claim_disposition`, and
   `omission_class` verbatim.
3. **Cited evidence is gated by `citation_refs.minItems = 1`.**
   `cited_*` dispositions cannot appear without at least one
   citation; the schema's `allOf` rules enforce the gate.
4. **Downgraded refusal is gated by a non-null
   `omission_ledger_entry_ref`.** A row that says "we refuse to
   assert this" cannot quietly disappear; the ledger names why.
5. **AI-assisted segments are typed and reviewable.** A row with
   `ai_inference_segment = true` MUST set
   `claim_disposition = narrative_ai_assisted` (or
   `omitted_ai_inference_segment_redacted`), MUST carry a non-null
   `ai_evidence_packet_ref`, and MUST cap confidence at `medium`. At
   least one export hook MUST set
   `preserves_ai_inference_segment_chip = true`.
6. **Imported citations carry the imported-root chip.** A citation
   with `imported_root_status != not_imported` renders the chip
   verbatim and at least one export hook preserves it.
7. **Anchor drift remains typed and reviewable.** A citation with
   `anchor_drift_state != anchor_present_no_drift` carries the
   typed drift chip plus a non-null `anchor_drift_record_ref`; at
   least one export hook preserves the drift chip.
8. **Loaded-scope state is honest.** The packet never claims
   `fully_loaded` while disclosing hidden rows; the schema's
   `allOf` rules enforce the consistency.
9. **Open-evidence actions resolve through stable ids.** Every
   action's `action_target_ref` resolves through the matching
   schema's id family. Surfaces never launch ambiguous string
   searches when an opaque ref is available.
10. **The omission ledger survives export.** A surface that claims
    a `review_pack_explainer_packet` / `support_bundle_explainer_packet` /
    `ai_evidence_explainer_packet` export carries the ledger while
    actually compressing it into a generic "partial results"
    phrase is non-conforming.

## 9. Worked examples

Each example references a companion fixture under
[`/fixtures/graph/codebase_explainer_cases/`](../../fixtures/graph/codebase_explainer_cases/).
The fixtures are human-authored YAML and validate against the
schema under JSON Schema Draft 2020-12.

### 9.1 Direct graph evidence

A symbol-relation explainer over the workspace graph: "the buffer
crate calls the text crate's `apply_edit` symbol from
`buffer/commands.rs`". One claim row at
`claim_disposition = cited_direct_evidence`,
`rolled_up_confidence = high`, two citations (one `symbol_definition`
on the `apply_edit` symbol, one `file_range` on the call site). The
omission ledger is empty; `loaded_scope_state = fully_loaded`.

See
[`direct_graph_evidence_explainer.yaml`](../../fixtures/graph/codebase_explainer_cases/direct_graph_evidence_explainer.yaml).

### 9.2 Partial imported evidence

An ownership-edge explainer where the helper symbol is contributed by
an imported signed upstream bundle. One claim row at
`claim_disposition = cited_partial_imported`,
`rolled_up_confidence = medium`,
`floor_reason = imported_unverified_caps_at_medium`. One citation
points at the imported bundle entry; one citation points at the
local owned-by edge. One omission ledger entry of class
`imported_root_unverified` records that the upstream bundle has not
been cross-validated. The export hook declares
`preserves_imported_root_chip = true`.

See
[`partial_imported_evidence_explainer.yaml`](../../fixtures/graph/codebase_explainer_cases/partial_imported_evidence_explainer.yaml).

### 9.3 Downgraded refusal because citations are insufficient

An impact-row explainer where the available evidence is a single
inferred-transitive impact row whose anchor was deleted; the
producer refuses to assert "the renamed helper is impacted". One
claim row at
`claim_disposition = downgraded_refusal_insufficient_evidence`,
`rolled_up_confidence = low`,
`floor_reason = missing_anchor_pulls_to_low`,
empty `citation_refs`, non-null `omission_ledger_entry_ref`. The
omission ledger entry records
`omission_class = downgraded_refusal_insufficient_evidence`,
`export_preservation_required = true`. The export hook declares
`preserves_omission_ledger = true`.

See
[`downgraded_refusal_insufficient_evidence_explainer.yaml`](../../fixtures/graph/codebase_explainer_cases/downgraded_refusal_insufficient_evidence_explainer.yaml).

## 10. Acceptance

- Every claim row can be traced to one or more citations whose ids
  reuse the seed's identity layer verbatim or is explicitly marked
  inferred / imported / drifted / AI-assisted / downgraded / omitted
  via `claim_disposition` plus a non-null
  `omission_ledger_entry_ref`. Unsupported claims cannot appear as
  authoritative prose because the schema's `allOf` rules enforce the
  gate.
- Open-evidence actions resolve through stable graph and source-
  anchor ids (`citation_ref`, `node_id_ref`, `edge_id_ref`,
  `source_anchor_ref`, `provenance_stamp_ref`,
  `confidence_rollup_ref`, `anchor_drift_record_ref`,
  `topology_view_ref`, `impact_view_ref`, `ownership_card_ref`,
  `policy_view_ref`) instead of launching ambiguous searches.
- The omission ledger survives review-pack / support-bundle /
  AI-evidence packet exports; the export-hook entries declare
  `preserves_omission_ledger`, `preserves_citation_anchors`,
  `preserves_confidence_chip`, `preserves_imported_root_chip`,
  `preserves_anchor_drift_chip`, and
  `preserves_ai_inference_segment_chip` explicitly so a lossy
  format cannot quietly compress the ledger into "partial results".
- The fixtures cover direct-evidence, partial-imported, and
  downgraded-refusal cases and validate against the schema.

## 11. Changing this vocabulary

- **Additive-minor.** Adding a new `claim_class`, new
  `claim_disposition`, new `omission_class`, new
  `offered_action_class`, new `export_format_class`, new
  `review_handoff_class`, new `citation_subject_kind`, new
  `offered_action_target_kind`, or new hidden-count axis lands here,
  in the schema, and in the fixtures in the same change. The change
  must cite the motivating fixture.
- **Repurposing** an existing value is breaking. It opens a new
  decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The workspace-graph seed wins on identity rules; the provenance /
  confidence contract wins on freshness, confidence, and anchor
  drift; the topology / impact contract wins on loaded-scope state,
  hidden counts, and topology / impact / ownership identity. This
  document and its schema are updated in the same change when those
  documents change.

## Linked artifacts

- Workspace-graph seed:
  [`docs/graph/workspace_graph_seed.md`](workspace_graph_seed.md),
  [`schemas/graph/workspace_graph_seed.schema.json`](../../schemas/graph/workspace_graph_seed.schema.json).
- Provenance / confidence / source-anchor-drift contract:
  [`docs/graph/provenance_and_confidence_contract.md`](provenance_and_confidence_contract.md),
  [`schemas/graph/provenance_stamp.schema.json`](../../schemas/graph/provenance_stamp.schema.json),
  [`schemas/graph/confidence_rollup.schema.json`](../../schemas/graph/confidence_rollup.schema.json).
- Topology-map / impact-explorer / ownership-card contract:
  [`docs/graph/topology_and_impact_contract.md`](topology_and_impact_contract.md),
  [`schemas/graph/topology_map_view.schema.json`](../../schemas/graph/topology_map_view.schema.json),
  [`schemas/graph/impact_reason.schema.json`](../../schemas/graph/impact_reason.schema.json).
- Codebase-explainer packet schema:
  [`schemas/graph/codebase_explainer_packet.schema.json`](../../schemas/graph/codebase_explainer_packet.schema.json).
- Codebase-explainer case fixtures:
  [`fixtures/graph/codebase_explainer_cases/`](../../fixtures/graph/codebase_explainer_cases/).
- Cross-repo result-group contract (hidden-count vocabulary):
  [`schemas/workspace/cross_repo_result_group.schema.json`](../../schemas/workspace/cross_repo_result_group.schema.json).
- Workset / scope contract:
  [`schemas/workspace/workset_artifact.schema.json`](../../schemas/workspace/workset_artifact.schema.json).
