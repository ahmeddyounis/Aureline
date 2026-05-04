# Graph topology, impact, cited-explainer, and scope-banner proof cases

These fixtures back the verification packet at
[`/docs/verification/graph_topology_explainer_packet.md`](../../../docs/verification/graph_topology_explainer_packet.md)
and the imported-root drift audit at
[`/artifacts/graph/imported_root_drift_audit.yaml`](../../../artifacts/graph/imported_root_drift_audit.yaml).

Each proof case is one YAML document carrying a
`graph_topology_explainer_proof_case_record` that quotes one
canonical topology view, one impact view (where applicable), one
cited explainer packet, one scope banner, and (where applicable)
one cross-repo result group **by id**, and projects the surface-
agreement field set the verification packet pins.

Proof cases do **not** redefine the canonical record shapes. The
upstream record-shape fixtures are:

- [`/fixtures/graph/topology_impact_cases/`](../topology_impact_cases/)
  — topology-map view, impact-explorer view, and ownership-card
  records.
- [`/fixtures/graph/codebase_explainer_cases/`](../codebase_explainer_cases/)
  — cited codebase explainer packets.
- [`/fixtures/graph/provenance_cases/`](../provenance_cases/)
  — provenance / confidence / source-anchor-drift records.
- [`/fixtures/workspace/workset_cross_repo_cases/`](../../workspace/workset_cross_repo_cases/)
  — workset switcher, scope banner, and cross-repo result-group
  records.

**Index**

| Proof case | Primary scenario | Loaded scope | Imported-root count | Anchor-drift posture | Claim disposition |
|---|---|---|---|---|---|
| [`full_workspace_topology_proof.yaml`](./full_workspace_topology_proof.yaml) | Full architecture-domain map across one workspace root with three crates and two depends_on edges; every chip authoritative | `fully_loaded` | 0 | `anchor_present_no_drift` | `cited_direct_evidence` |
| [`sparse_workset_impact_proof.yaml`](./sparse_workset_impact_proof.yaml) | Sparse runtime-domain map with eight workset-hidden, two policy-hidden members | `policy_limited_loaded` | 0 | `anchor_present_no_drift` | `cited_direct_evidence` (with hidden-by-workset and hidden-by-policy omissions) |
| [`imported_root_cited_explainer_proof.yaml`](./imported_root_cited_explainer_proof.yaml) | vendor:acme:1.3.0 imported signed upstream bundle drives an impact row, an ownership card, and an explainer claim | `imported_root_only_loaded` | 1 | `anchor_imported_unverified` | `cited_partial_imported` |
| [`insufficient_evidence_downgrade_proof.yaml`](./insufficient_evidence_downgrade_proof.yaml) | legacy_helper rename with deleted source anchor; explainer refuses the strong claim | `evidence_missing_loaded` | 0 | `anchor_deleted_no_replacement` | `downgraded_refusal_insufficient_evidence` |

**Coverage contract**

This fixture set MUST cover, at minimum: a full-workspace topology
proof, a sparse-workset impact proof, an imported-root cited
explainer proof, and an insufficient-evidence downgrade proof.
Adding fixtures that exercise additional `loaded_scope_state`,
`imported_root_status`, `anchor_drift_state`, `claim_disposition`,
or `omission_class` combinations is welcome; removing a class this
directory already covers is a breaking change.

**Scope rules**

- Proof cases assert cross-surface agreement; they do not encode
  wire bytes or runtime execution-context envelopes.
- A new proof case MUST quote at least one upstream topology /
  impact / explainer / scope banner / cross-repo result-group
  fixture by id, project the expected surface-agreement field set,
  list named cross-surface invariants, and list the closed failure
  signatures the proof case is designed to catch.
- Workset ids, scope refs, view ids, packet ids, and drift record
  ids are quoted by reference; no proof case mints a parallel
  stable id over a subject that already has one.
- Filesystem-root refs are opaque pointers to ADR-0006 filesystem-
  identity records. Raw absolute paths, raw remote URLs, raw policy
  bodies, and raw symbol definitions never appear.
