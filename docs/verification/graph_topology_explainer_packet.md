# Graph topology, impact, cited-explainer, and scope-banner verification seed

This packet freezes one reviewer-facing story for graph-backed
understanding before Aureline ships strong codebase-intelligence
claims. It exists so review, support, public-proof, and QE lanes can
cite one corpus when they ask "is this topology view, this impact
row, and this cited explainer telling the same scope, provenance,
imported-root, and anchor-drift story under partial, imported, or
drifting workspace conditions?"

If this packet, the topology-explainer corpus, the imported-root
drift audit, and the underlying graph schemas disagree, the
machine-readable schemas and base contracts win and this packet must
update in the same change so review, support, and public-proof
surfaces read one story.

Companion artifacts:

- [`/fixtures/graph/topology_explainer_cases/`](../../fixtures/graph/topology_explainer_cases/)
  — partial-scope corpus tying one topology view, one impact view,
  one cited explainer, and one scope banner per proof case back to
  graph provenance, confidence, and source-anchor-drift records.
- [`/artifacts/graph/imported_root_drift_audit.yaml`](../../artifacts/graph/imported_root_drift_audit.yaml)
  — machine-readable drift audit naming every imported-root condition
  (anchor moved, deleted, replaced, partially loaded, unverified) and
  the topology / impact / explainer / search labels that MUST agree
  for that condition.
- [`/docs/graph/topology_and_impact_contract.md`](../graph/topology_and_impact_contract.md)
  — topology-map view, impact-explorer row, ownership-card record
  shapes this packet quotes rather than redefines.
- [`/docs/graph/codebase_explainer_contract.md`](../graph/codebase_explainer_contract.md)
  — cited explainer packet, claim disposition, omission ledger, and
  open-evidence action vocabulary this packet quotes rather than
  redefines.
- [`/docs/graph/provenance_and_confidence_contract.md`](../graph/provenance_and_confidence_contract.md)
  — provenance stamp, confidence rollup, freshness, and source-anchor
  drift vocabulary every claim row in this packet rolls up against.
- [`/docs/workspace/workset_scope_and_cross_repo_contract.md`](../workspace/workset_scope_and_cross_repo_contract.md)
  — workset switcher, scope banner, and cross-repo result-group
  vocabulary the proof cases quote so search and graph surfaces share
  one scope vocabulary.
- [`/fixtures/graph/topology_impact_cases/`](../../fixtures/graph/topology_impact_cases/),
  [`/fixtures/graph/codebase_explainer_cases/`](../../fixtures/graph/codebase_explainer_cases/),
  [`/fixtures/graph/provenance_cases/`](../../fixtures/graph/provenance_cases/),
  and
  [`/fixtures/workspace/workset_cross_repo_cases/`](../../fixtures/workspace/workset_cross_repo_cases/)
  — the upstream record-shape fixtures the proof cases quote by
  reference rather than re-mint.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md` — graph-backed understanding,
  imported-root and partial-workset honesty rules, and the
  cited-explainer and omission-ledger requirement register entries.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — workspace
  graph identity, provenance / confidence rollup, and source-anchor
  drift requirements.
- `.t2/docs/Aureline_Technical_Design_Document.md` — topology-map
  view, impact-explorer row, ownership-card, cited-explainer packet,
  and scope-banner record shapes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — topology, impact,
  cited-explainer, scope-banner, and cross-repo result-group surface
  rules.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: verification_packet
packet_id: verification.graph_topology_explainer.partial_scope_seed
evidence_id: evidence.graph.topology_explainer_partial_scope_seed
title: Graph topology, impact, cited-explainer, and scope-banner verification seed
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids: []
  claim_row_refs:
    - packet_row:graph_topology_explainer.surface_agreement_contract
    - packet_row:graph_topology_explainer.partial_scope_corpus
    - packet_row:graph_topology_explainer.imported_root_drift_audit
    - packet_row:graph_topology_explainer.proof_case_set
    - packet_row:graph_topology_explainer.omission_ledger_export_preservation
  covered_lanes:
    - release_evidence
    - support_export
    - governance_packets
    - docs_public_truth
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-05-04T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: commit:working_tree
  trigger_revision: graph_topology_explainer_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen workspace-graph seed, the
    provenance / confidence / source-anchor-drift contract, the
    topology-map / impact-explorer / ownership-card contract, the
    cited codebase explainer contract, and the workset-scope and
    cross-repo result-group contract. No graph engine, search engine,
    or topology UI implementation is claimed.
artifact_links:
  supporting_evidence_ids:
    - evidence.graph.topology_impact_cases
    - evidence.graph.codebase_explainer_cases
    - evidence.graph.provenance_cases
    - evidence.workspace.workset_cross_repo_cases
    - evidence.graph.imported_root_drift_audit
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/graph/topology_explainer_cases/
    - fixtures/graph/topology_impact_cases/
    - fixtures/graph/codebase_explainer_cases/
    - fixtures/graph/provenance_cases/
    - fixtures/workspace/workset_cross_repo_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/graph/topology_and_impact_contract.md
    - docs/graph/codebase_explainer_contract.md
    - docs/graph/provenance_and_confidence_contract.md
    - docs/workspace/workset_scope_and_cross_repo_contract.md
    - artifacts/graph/imported_root_drift_audit.yaml
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This packet freezes one reviewer-facing field set tying topology-map
views, impact-explorer rows, cited-explainer packets, and scope
banners back to a single graph provenance / confidence / anchor-drift
story. It does not claim the graph engine, the topology renderer, the
impact computation engine, or the explainer surface chrome are
implemented; it claims only that the partial-scope corpus, the
imported-root drift audit, and the four required proof cases now
exist and can be cited by QE and public-proof lanes without
per-feature graph truth.

## Claim coverage

| Packet row | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|
| `packet_row:graph_topology_explainer.surface_agreement_contract` | `seed_only` | `internal` | `evidence.graph.topology_impact_cases`, `evidence.graph.codebase_explainer_cases`, `evidence.workspace.workset_cross_repo_cases` | Pins the surfaces that must agree on `loaded_scope_state`, `hidden_count_summary`, `imported_root_status`, `anchor_drift_state`, and `rolled_up_confidence` for any single subject. |
| `packet_row:graph_topology_explainer.partial_scope_corpus` | `seed_only` | `internal` | `evidence.graph.topology_explainer_partial_scope_corpus` | Names the partial-scope corpus that backs every claim row in the packet. |
| `packet_row:graph_topology_explainer.imported_root_drift_audit` | `seed_only` | `internal` | `evidence.graph.imported_root_drift_audit` | Names the audit that holds imported-root labeling consistent across search, topology, impact, explainer, and ownership surfaces. |
| `packet_row:graph_topology_explainer.proof_case_set` | `seed_only` | `internal` | `evidence.graph.topology_explainer_partial_scope_corpus` | Names the four required proof cases (full-workspace topology, sparse-workset impact, imported-root cited explainer, insufficient-evidence downgrade). |
| `packet_row:graph_topology_explainer.omission_ledger_export_preservation` | `seed_only` | `internal` | `evidence.graph.codebase_explainer_cases` | Holds omission ledger entries verbatim through review-pack, support-bundle, AI-evidence, and public-proof exports rather than compressing into "partial results". |

## What this seed freezes

- **One surface-agreement contract** so the same subject (a node, an
  edge, an imported root, a drifted anchor) carries identical
  `loaded_scope_state`, `hidden_count_summary`, `imported_root_status`,
  `anchor_drift_state`, and `rolled_up_confidence` chips on the
  topology map, the impact explorer, the ownership card, the cited
  explainer, and the cross-repo result group / scope banner that hosts
  it.
- **One partial-scope corpus** covering sparse worksets,
  imported / generated roots, stale anchors, hidden-result groups,
  and ownership-domain gaps with a single proof-case record per
  scenario.
- **One imported-root drift audit** that types every moved, deleted,
  replaced, partially-loaded, or unverified root and pins the labels
  every search, topology, impact, and explainer surface MUST emit for
  that condition.
- **One four-case proof set** covering full-workspace topology (no
  drift, no imports, all chips authoritative), sparse-workset impact
  (hidden-by-workset and hidden-by-policy counts non-zero,
  imported-root count zero), imported-root cited explainer (imported
  signed upstream bundle with anchor-imported-unverified and a
  non-empty omission ledger), and insufficient-evidence downgrade
  (claim refused with non-empty omission ledger and zero citations).
- **One omission-ledger preservation rule** so review-pack /
  support-bundle / AI-evidence / public-proof exports carry the
  ledger verbatim rather than compressing it to "partial results".

## Surface-agreement field set

For every subject (graph node, graph edge, imported root, drifted
anchor, omission ledger entry) projected onto more than one graph-
backed surface, the surfaces MUST agree on this field set:

| Packet field | Source contract | Required cross-surface invariant |
|---|---|---|
| `loaded_scope_state` | topology-map view contract | Topology, impact, ownership, and explainer surfaces MUST emit the same value for the same view subject. |
| `active_scope_class` + `active_scope_ref` | workset-scope and cross-repo contract | Scope banner, topology view, impact view, ownership card, and explainer packet MUST quote the same workset id and the same scope class. |
| `hidden_count_summary.hidden_by_workset_count` | topology-map view contract | Search-result hidden count, topology hidden-by-workset count, impact hidden-by-workset count, and explainer hidden-by-workset count MUST agree for the same subject. |
| `hidden_count_summary.hidden_by_policy_count` | topology-map view contract | Identical agreement; the count crosses the boundary, the hidden ids do not. |
| `hidden_count_summary.imported_root_count` | topology-map view contract | Topology, impact, explainer, and cross-repo result-group surfaces MUST emit the same count for the same imported-root condition. |
| `imported_root_status` | provenance-and-confidence contract | Identical token across all surfaces (`not_imported`, `imported_signed_upstream_bundle`, `imported_unsigned_bundle`, `imported_local_only_bundle`, `imported_unknown_lineage`). |
| `anchor_drift_state` | provenance-and-confidence contract | Identical token across all surfaces (`anchor_present_no_drift`, `anchor_moved_with_match`, `anchor_moved_with_partial_match`, `anchor_deleted_no_replacement`, `anchor_imported_unverified`, `anchor_unknown_lineage`). |
| `rolled_up_confidence` + `floor_reason` | provenance-and-confidence contract | Identical chip across all surfaces; floor reason explains any cap. |
| `claim_disposition` (explainer only) | codebase-explainer contract | Disposition MUST be one of `cited_direct_evidence`, `cited_partial_imported`, `cited_inferred_with_anchor`, `downgraded_refusal_insufficient_evidence`; refusal cases MUST carry zero citations and a non-empty omission ledger entry. |
| `omission_class` (omission ledger only) | codebase-explainer contract | Imported-root unverified, evidence-missing-anchor-deleted, hidden-by-workset, hidden-by-policy, ai-inference-segment, and downgraded-refusal-insufficient-evidence omissions MUST be typed; surfaces MUST NOT compress to "partial results". |

Rules:

1. A topology view that names `loaded_scope_state = imported_root_only_loaded`
   MUST be hosted by a scope banner whose `hidden_result_summary`
   names imported roots; an explainer over the same subject MUST
   carry `imported_root_status = imported_signed_upstream_bundle` (or
   the equivalent imported class) on at least one citation. If any
   surface emits `not_imported` for the same subject, the packet
   fails.
2. A topology view that names `loaded_scope_state = policy_limited_loaded`
   MUST be hosted by a scope banner whose `policy_overlay` block
   names `hidden_member_list_visible = false`; an impact explorer
   over the same scope MUST carry `hidden_by_policy_count > 0`.
3. A subject with `anchor_drift_state = anchor_deleted_no_replacement`
   MUST surface as an evidence-missing condition on every surface
   that hosts it: the topology view's `hidden_count_summary.evidence_missing_count`
   MUST be ≥ 1, the explainer claim disposition MUST resolve to
   `downgraded_refusal_insufficient_evidence` (or an equivalent
   refusal), the omission ledger MUST carry `evidence_missing_anchor_deleted`,
   and the cross-repo result group MUST carry an `evidence_missing`
   row state.
4. A claim row at `claim_disposition = downgraded_refusal_insufficient_evidence`
   MUST carry zero `citation_refs` and at least one
   `omission_ledger_entry` of class
   `downgraded_refusal_insufficient_evidence`. Surfaces MUST never
   render the refusal as authoritative prose.
5. Every export hook on every surface MUST declare its
   `preserves_*` flags explicitly. The lossless review-pack export
   MUST carry `preserves_omission_ledger = true`,
   `preserves_imported_root_chip = true`, and
   `preserves_anchor_drift_chip = true`. Lossy exports MUST disclose
   the dropped chips on the export receipt.

## Partial-scope corpus

The corpus lives at
[`/fixtures/graph/topology_explainer_cases/`](../../fixtures/graph/topology_explainer_cases/).
Each proof case is one YAML document carrying a
`graph_topology_explainer_proof_case_record` (this packet's
record kind) that quotes one topology view, one impact view, one
cited explainer, and one scope banner by reference and projects the
expected surface-agreement field set verbatim.

| Proof case id | Primary scenario | Loaded-scope state | Imported-root count | Anchor-drift posture | Claim disposition |
|---|---|---|---|---|---|
| `graph.topology_explainer_proof.full_workspace_topology` | Full architecture-domain map across one workspace root with three crates and two depends_on edges; every chip authoritative; explainer cites two direct evidence rows | `fully_loaded` | 0 | `anchor_present_no_drift` | `cited_direct_evidence` |
| `graph.topology_explainer_proof.sparse_workset_impact` | Sparse runtime-domain map under a named workset narrowing to one service plus its kernel; eight workset-hidden, two policy-hidden; impact row over the same workset; explainer cites the workset-narrowed evidence and records the hidden counts as omissions | `policy_limited_loaded` | 0 | `anchor_present_no_drift` | `cited_direct_evidence` (with hidden-by-workset and hidden-by-policy omissions) |
| `graph.topology_explainer_proof.imported_root_cited_explainer` | Imported-bundle-rollover impact and ownership card under a vendor-imported-only loaded scope; explainer cites the imported bundle entry and the local CODEOWNERS rule; omission ledger names imported-root-unverified | `imported_root_only_loaded` | 1 | `anchor_imported_unverified` | `cited_partial_imported` |
| `graph.topology_explainer_proof.insufficient_evidence_downgrade` | Legacy_helper rename with deleted source anchor; topology / impact / explainer all agree; explainer refuses the strong claim; omission ledger carries the downgraded-refusal-insufficient-evidence and evidence-missing-anchor-deleted entries; export-preservation required | `evidence_missing_loaded` | 0 | `anchor_deleted_no_replacement` | `downgraded_refusal_insufficient_evidence` |

Each proof-case record carries:

- `proof_case_id` — stable id quoted by review packets and audits.
- `topology_view_ref`, `impact_view_ref`, `explainer_packet_ref`,
  `scope_banner_ref`, `cross_repo_result_group_ref` — references back
  to the canonical topology / impact / explainer / scope banner /
  cross-repo result-group fixtures, by id.
- `expected_loaded_scope_state`, `expected_active_scope_class`,
  `expected_active_scope_ref` — projected scope vocabulary.
- `expected_hidden_count_summary` — projected hidden counts.
- `expected_imported_root_status`, `expected_anchor_drift_state`,
  `expected_rolled_up_confidence`, `expected_floor_reason`,
  `expected_conflict_state` — projected provenance / confidence /
  drift chips.
- `expected_claim_disposition`, `expected_citation_count`,
  `expected_omission_ledger` — projected explainer outcome with the
  full omission ledger inline.
- `cross_surface_invariants` — list of named invariants the proof
  case asserts (e.g. `topology_and_explainer_loaded_scope_state_match`,
  `imported_root_count_matches_across_topology_impact_and_explainer`,
  `omission_ledger_preserved_in_review_pack_export`).
- `failure_signatures` — closed list of failure signatures the proof
  case is designed to catch (e.g. `topology_claims_full_load_when_explainer_omits_evidence`,
  `imported_root_chip_dropped_on_explainer`,
  `anchor_deleted_collapses_into_generic_no_results`).

## Imported-root drift audit

The audit lives at
[`/artifacts/graph/imported_root_drift_audit.yaml`](../../artifacts/graph/imported_root_drift_audit.yaml).
It enumerates every imported-root condition the contract recognises
and the labels every surface MUST emit for that condition. Each row
covers:

- `audit_row_id` — stable id quoted by QE and public-proof lanes.
- `imported_root_condition` — one of `imported_signed_upstream_bundle_anchor_resolves`,
  `imported_signed_upstream_bundle_anchor_unverified`,
  `imported_unsigned_bundle_anchor_unverified`,
  `imported_anchor_moved_with_match`,
  `imported_anchor_moved_with_partial_match`,
  `imported_anchor_deleted_no_replacement`,
  `imported_root_partially_loaded_pending_workset_widen`,
  `imported_root_unknown_lineage`,
  `imported_root_generated_artifact`.
- `expected_topology_label` — `imported_root_status`,
  `anchor_drift_state`, `rolled_up_confidence`, `floor_reason`, and
  `loaded_scope_state` the topology view MUST emit.
- `expected_impact_label` — same five tokens for the impact-explorer
  row.
- `expected_explainer_label` — `claim_disposition`,
  `omission_class`, `imported_root_status`, `anchor_drift_state`, and
  `rolled_up_confidence` the cited explainer packet MUST emit.
- `expected_search_label` — `cross_repo_result_group_row_class`,
  `in_scope_marker`, and `result_state_class` the cross-repo
  result-group / search surface MUST emit.
- `expected_scope_banner_label` — banner state, `partial_index_note`,
  and `hidden_result_summary.count_class` the scope banner MUST
  emit when this condition is present in the active scope.
- `failure_signatures` — closed list of failure signatures the audit
  catches (e.g. `imported_anchor_deleted_collapses_into_generic_no_results`,
  `imported_signed_upstream_bundle_renders_as_local_on_topology`,
  `imported_root_count_diverges_between_topology_and_explainer`).
- `proof_case_refs` — list of proof cases that exercise this
  condition.

## Evidence joins

| `evidence_id` | Family / source kind | Why it is linked here | Freshness note | Artifact ref |
|---|---|---|---|---|
| `evidence.graph.topology_impact_cases` | `verification_corpus` | Upstream topology-map view, impact-explorer view, and ownership-card record fixtures the proof cases quote. | current with packet revision 1 | [`fixtures/graph/topology_impact_cases/`](../../fixtures/graph/topology_impact_cases/) |
| `evidence.graph.codebase_explainer_cases` | `verification_corpus` | Upstream cited explainer packet fixtures the proof cases quote. | current with packet revision 1 | [`fixtures/graph/codebase_explainer_cases/`](../../fixtures/graph/codebase_explainer_cases/) |
| `evidence.graph.provenance_cases` | `verification_corpus` | Upstream provenance / confidence / source-anchor-drift record fixtures the proof cases roll up against. | current with packet revision 1 | [`fixtures/graph/provenance_cases/`](../../fixtures/graph/provenance_cases/) |
| `evidence.workspace.workset_cross_repo_cases` | `verification_corpus` | Upstream workset switcher, scope banner, and cross-repo result-group fixtures the proof cases quote so search and graph surfaces share one scope vocabulary. | current with packet revision 1 | [`fixtures/workspace/workset_cross_repo_cases/`](../../fixtures/workspace/workset_cross_repo_cases/) |
| `evidence.graph.topology_explainer_partial_scope_corpus` | `verification_corpus` | The four required proof cases. | current with packet revision 1 | [`fixtures/graph/topology_explainer_cases/`](../../fixtures/graph/topology_explainer_cases/) |
| `evidence.graph.imported_root_drift_audit` | `verification_corpus` | Imported-root drift audit pinning per-surface labels. | current with packet revision 1 | [`artifacts/graph/imported_root_drift_audit.yaml`](../../artifacts/graph/imported_root_drift_audit.yaml) |

## Corpus coverage

The proof-case corpus is the canonical row set. The reviewer-facing
summary follows the partial-scope corpus table above; the imported-
root drift audit lives in YAML and is the canonical source for
imported-root surface labels.

## Verification method

- **Verification classes used:** design review, corpus freeze,
  cross-surface label review, drift-audit row review.
- **Procedure summary:** quote the topology / impact / explainer
  contracts and the workset-scope contract; project a single proof-
  case record per scenario tying topology, impact, explainer, scope
  banner, and (when relevant) cross-repo result-group fixtures
  together; freeze the imported-root drift audit covering every
  imported-root condition recognised by the contracts; pair each
  proof case with the closed list of cross-surface invariants and
  failure signatures it asserts.
- **Automation refs:** no dedicated validator yet; proof-case YAML
  parses with the existing JSON / YAML check, and the drift audit is
  shaped so a follow-on validator can compare topology / impact /
  explainer / search labels for the same imported-root condition.

## Known gaps and waivers

- **Waiver refs:** `none`
- **Known-limit refs:** `none`
- **Migration-packet refs:** `none`
- **Explicit gaps:** the packet does not run a live graph engine,
  search engine, topology renderer, or explainer surface; it freezes
  the cross-surface vocabulary those engines will read.
- **Explicit gaps:** the packet does not enumerate every possible
  failure signature; it freezes the closed list the four proof cases
  exercise and the imported-root drift audit asserts. Adding new
  signatures is welcome; removing one is a breaking change.
- **Explicit gaps:** AI-suggested explainer narrative segments are
  represented as a marker on the claim row (`ai_inference_segment`)
  rather than as a separate proof case; the AI-assistance contract
  pins the segment-marker rules.

## Reviewer signoff

- **Reviewer / forum:** `not_yet_reviewed`
- **Decision:** `needs_follow_up`
- **Date:** `2026-05-04`
- **Reviewed claim rows:**
  `packet_row:graph_topology_explainer.surface_agreement_contract`,
  `packet_row:graph_topology_explainer.partial_scope_corpus`,
  `packet_row:graph_topology_explainer.imported_root_drift_audit`,
  `packet_row:graph_topology_explainer.proof_case_set`,
  `packet_row:graph_topology_explainer.omission_ledger_export_preservation`
- **Blocking refs:** `none`

## Refresh trigger

- **Named rerun trigger:** `graph_corpus_or_audit_revision_changed`
- **Expected freshness window:** refresh within `P30D` or whenever
  the topology-map view, impact-explorer row, ownership-card,
  cited explainer packet, scope banner, cross-repo result-group, or
  provenance / confidence / source-anchor-drift contracts change.
- **Next packet family to update with the same evidence ids:**
  release evidence and support-export packets covering graph-backed
  understanding claims, plus the public-proof packet for codebase
  intelligence.
