# Diagnostic clustering, semantic-layer banners, freshness/scope labels, and detail sheets

Stable contract for the clustered diagnostic surface that keeps Problems and
in-context findings trustworthy across the M5 **notebook, framework, preview,
and generated-code** consumers. Where the sibling
[semantic-result arbitration packet](arbitration-inspectors-disagreement-detail-and-semantic-to-text-fallback-banners.md)
certifies the *answer* a definition/references/hierarchy/completion lane
returns, this packet certifies the *diagnostic cluster* those surfaces render:

- a **cluster identity** — the diagnostic source families (compiler, linter,
  language-server, framework, runtime, notebook, policy) that converged into
  one cluster, whether deduplication preserved per-provider detail,
  timestamps/epochs, suppression/baseline state, and related symbol/file
  evidence, whether the source families stayed differentiated rather than
  fusing into one undifferentiated row, and the route that opens the cluster
  detail sheet;
- a **semantic-layer banner** — which posture the surface explains it is in
  (semantic, graph-warm, syntax-only, cached, runtime-only, or partial) and the
  freshness and scope labels the cluster may claim; and
- a **fix offer** — whether a fix is offered, the acting provider and
  freshness/scope posture named alongside it, and (for a mutating fix) the typed
  preview completeness and rollback checkpoint required before any
  organize-imports, schema/codegen, AI-planned, or notebook/generated edit
  mutates source.

This document is the human-readable companion to the diagnostic-cluster truth
packet. The canonical record is checked in at
`artifacts/language/m5/diagnostic_cluster_semantic_layer_truth_packet.json` and
validated by the boundary schema at
`schemas/language/diagnostic_cluster_semantic_layer_truth.schema.json`. The
packet is owned by `aureline-language`
(`crates/aureline-language/src/diagnostic_cluster_semantic_layer_truth_packet/`),
and the protected fixture corpus lives at
`fixtures/language/m5/diagnostic_cluster_semantic_layer_truth_packet/`.

## Why this exists

A compiler error, a lint warning, a language-server hint, a framework-schema
violation, a failing test, a notebook-kernel traceback, and a policy finding
must cluster so the user is not buried — but the cluster is only trustworthy if
each finding's *provider, freshness, scope, suppression state, and related
evidence stays inspectable.* Collapsing that into one undifferentiated error row
hides exactly the truth a safe fix depends on: whether the row is a security
finding or a formatting nit, whether the evidence is live or stale, and whether
acting on it would rewrite generated source. This packet is the single source
that, per surface and cluster lane, names the converged providers, keeps the
loser of a disagreement inspectable, labels the semantic layer the surface may
claim, and refuses to offer a fix that hides its provider or skips the
launch-language refactor safety model.

## Reused vocabulary — not a local synonym set

The packet does **not** re-mint provider vocabulary. It reads the closed
provider-family, conflict, diagnostic-source, completeness, support, evidence,
known-limit, downgrade-automation, confidence, and consumer-surface vocabularies
frozen by the provider/refactor matrix packet
(`crates/aureline-language/src/provider_refactor_matrix_truth_packet/`) and adds
only the cluster-provenance, source-differentiation, detail-sheet,
semantic-layer banner, freshness, scope-label,
provider-disagreement-visibility, and fix-offer vocabulary the clustered
diagnostic surfaces need on top.

## What every row binds

| Group | Dimensions |
| --- | --- |
| Identity | `surface_class`, `cluster_lane_class`, `support_class` |
| Cluster identity | `diagnostic_source_classes`, `cluster_provenance_class`, `source_differentiation_class`, `preserves_per_provider_detail`, `preserves_timestamps_epochs`, `preserves_suppression_baseline`, `preserves_related_evidence`, `detail_sheet_route_class` |
| Semantic-layer banner | `semantic_layer_banner_class`, `freshness_class`, `scope_label_class` |
| Provider arbitration | `acting_provider_family_class`, `conflict_class`, `provider_disagreement_visibility_class` |
| Fix offer & refactor safety | `fix_offer_class`, `preview_completeness_class`, `rollback_checkpoint_ref` |
| Evidence & narrowing | `evidence_class`, `known_limit_class`, `downgrade_automation_class`, `confidence_class`, `evidence_refs`, `disclosure_ref` |
| Boundary | `raw_source_material_excluded`, `secrets_excluded`, `ambient_authority_excluded` |

## Claim narrowing — what makes a row narrow below stable

The validator narrows a packet below stable (it never silently publishes)
whenever a row would hide truth the source documents require to stay
inspectable. Each rule maps to a guardrail:

- **`cluster_provenance_collapsed`** — a multi-provider cluster dropped
  per-provider detail, timestamps/epochs, suppression/baseline state, or related
  evidence. Deduplication may merge, but it may not erase provenance.
- **`sources_fused_undifferentiated`** — runtime evidence, policy / security
  findings, and static analysis collapsed into one undifferentiated error row.
- **`losing_provider_collapsed`** — a provider disagreement collapsed into
  ranking-only output that drops the losing provider. The loser stays
  inspectable.
- **`opaque_detail_sheet_route`** — an opaque spinner standing in for a real
  detail-sheet route.
- **`detail_sheet_route_missing`** — a multi-provider or disagreeing cluster
  with no inspectable detail sheet to unpack it.
- **`semantic_layer_overclaimed`** — a `semantic` banner claimed on stale or
  otherwise non-live evidence; the banner must narrow to a degraded posture.
- **`overclaimed_scope_on_stale_evidence`** — a whole-workspace scope claimed on
  stale or non-semantic evidence; the scope must narrow to the scanned slice.
- **`fix_offered_without_provider_or_freshness`** — a fix offered without naming
  the acting provider and freshness/scope posture.
- **`mutating_fix_bypasses_preview`** — an organize-imports, schema/codegen,
  AI-planned, notebook/generated, or quick-fix mutation that bypasses typed
  preview completeness and a rollback checkpoint. This preserves — it never
  weakens — the launch-language refactor safety model while extending it to
  M5-only artifacts and framework packs.

Binding, disclosure, boundary, coverage, and consumer-projection rules round out
the validator; any blocker narrows the packet to `blocks_stable`, and a
certified-at-low-confidence row narrows to `narrowed_below_stable`.

## Consumers

Every required consumer surface
(`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`,
`conformance_dashboard`) carries a projection that preserves the packet
verbatim, so Help/About, release-center, support, and conformance surfaces read
the same cluster truth the editor shows. The support export
(`DiagnosticClusterSemanticLayerTruthSupportExport`) wraps the exact packet and
is export-safe only when the packet validates with no findings and no private
material crosses the boundary.

## Regeneration

The artifact and fixtures are generated from the real validator so they can
never drift from the materialized packet:

```
cargo run -p aureline-language --example dump_diagnostic_cluster_semantic_layer_truth_packet
```
