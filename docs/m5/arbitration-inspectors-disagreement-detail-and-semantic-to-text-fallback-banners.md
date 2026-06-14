# Arbitration inspectors, disagreement detail, and semantic-to-text fallback banners

Stable contract for the per-answer arbitration inspector that keeps
**definition, references, hierarchy, and completion** results trustworthy
across the M5 **search, docs, framework, notebook, and generated-source**
consumers. Where the provider-status surface packet certifies the reusable
strip, drawer, and provenance-pill UI objects, this packet certifies the
*result* those objects anchor:

- an **arbitration inspector** — which provider won, the basis it won on,
  whether the alternate (losing) providers stay inspectable, and the route
  that opens the detail;
- a **disagreement detail** — the conflict class, whether the conflict
  changes target identity, scope coverage, or refactor safety, and how that
  disagreement is made visible; and
- a **semantic-to-text fallback banner** — the result tier, the banner shown
  when a semantic answer degraded, the guarantee that *remains*, the
  guarantee that was *lost*, the scope the surface may still claim, any
  skipped-coverage gap, and (for a mutating follow-up) the typed preview
  completeness and rollback checkpoint that the launch-language refactor
  safety model still requires.

This document is the human-readable companion to the result-arbitration
truth packet. The canonical record is checked in at
`artifacts/language/m5/semantic_result_arbitration_truth_packet.json` and
validated by the boundary schema at
`schemas/language/semantic_result_arbitration_truth.schema.json`. The packet
is owned by `aureline-language`
(`crates/aureline-language/src/semantic_result_arbitration_truth_packet/`),
and the protected fixture corpus lives at
`fixtures/language/m5/semantic_result_arbitration_truth_packet/`.

## Why this exists

A definition, reference set, hierarchy, or completion result is only
trustworthy if the user can still tell *which provider answered, what the
other providers said, where the answer's confidence and completeness
changed, and when a semantic answer degraded to heuristic, file-local, or
text behavior.* Collapsing that into one generic semantic result hides
exactly the truth a safe code action depends on. This packet is the single
source that, per surface and lane, names the winning provider and its basis,
keeps the loser inspectable, makes a material conflict openable, and labels
every degraded answer with the guarantee it kept and the guarantee it gave
up.

## Reused vocabulary — not a local synonym set

The packet does **not** re-mint provider vocabulary. It reads the closed
provider-family, conflict, completeness, support, evidence, known-limit,
downgrade-automation, confidence, and consumer-surface vocabularies frozen
by the provider/refactor matrix packet
(`crates/aureline-language/src/provider_refactor_matrix_truth_packet/`) and
adds only the arbitration-inspector, disagreement-impact, and
fallback-banner vocabulary the result lanes need on top. It anchors the same
objects the provider-status surface packet
(`artifacts/language/m5/provider_status_surface_truth_packet.json`)
certifies.

## What every row binds

| Group | Dimensions |
| --- | --- |
| Identity | `result_surface_class`, `result_lane_class`, `support_class` |
| Arbitration inspector | `acting_provider_family_class`, `arbitration_basis_class`, `alternate_provider_visibility_class`, `inspector_route_class` |
| Disagreement detail | `conflict_class`, `disagreement_impact_class`, `disagreement_visibility_class` |
| Fallback banner | `result_tier_class`, `fallback_banner_class`, `retained_guarantee_class`, `lost_guarantee_class`, `claim_scope_class`, `coverage_gap_class` |
| Refactor safety | `anchor_action_class`, `preview_completeness_class`, `rollback_checkpoint_ref` |
| Evidence & narrowing | `evidence_class`, `known_limit_class`, `downgrade_automation_class`, `confidence_class`, `evidence_refs`, `disclosure_ref` |
| Boundary | `raw_source_material_excluded`, `secrets_excluded`, `ambient_authority_excluded` |

## Claim narrowing — what makes a row narrow below stable

The validator narrows a packet below stable (it never silently publishes)
whenever a row would hide truth the source documents require to stay
inspectable. Each rule maps to a guardrail:

- **`losing_provider_collapsed`** — a disagreement collapsed into
  ranking-only output that drops the losing provider. The loser and the
  downgrade reason must stay inspectable.
- **`disagreement_detail_path_missing`** — a conflict that changes target
  identity, scope coverage, or refactor safety with no visible detail path.
- **`silent_fusion_of_conflict`** — a target-identity conflict fused
  silently into an exact answer with no visible disagreement.
- **`opaque_inspector_route`** — an opaque spinner standing in for a real
  inspection route.
- **`fallback_banner_missing`** — a degraded answer with no fallback banner
  or with no recorded lost guarantee; a fallback banner must preserve the
  guarantees that remain and the guarantees that were lost.
- **`fallback_banner_on_exact_result`** — an exact answer mislabeled with a
  fallback banner or a lost guarantee.
- **`overclaimed_scope_on_lexical_evidence`** — a whole-workspace /
  all-results claim resting only on lexical or heuristic evidence; surfaces
  stop claiming safe-wide rename or all-references when only lexical evidence
  exists.
- **`whole_workspace_wording_with_coverage_gap`** — whole-workspace wording
  kept after excluded roots, unloaded slices, generated-only edges, or
  notebook cells were skipped.
- **`retained_guarantee_overstated`** — a text / lexical result advertising a
  retained semantic guarantee.
- **`mutating_anchor_bypasses_preview`** — a mutating follow-up that bypasses
  typed preview completeness and a rollback checkpoint. This preserves — it
  never weakens — the launch-language refactor safety model while extending
  it to M5-only artifacts and framework packs.

Binding, disclosure, boundary, coverage, and consumer-projection rules round
out the validator; any blocker narrows the packet to `blocks_stable`, and a
certified-at-low-confidence row narrows to `narrowed_below_stable`.

## Consumers

Every required consumer surface
(`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`,
`conformance_dashboard`) carries a projection that preserves the packet
verbatim, so Help/About, release-center, support, and conformance surfaces
read the same arbitration truth the editor shows. The support export
(`SemanticResultArbitrationTruthSupportExport`) wraps the exact packet and is
export-safe only when the packet validates with no findings and no private
material crosses the boundary.

## Regeneration

The artifact and fixtures are generated from the real validator so they can
never drift from the materialized packet:

```
cargo run -p aureline-language --example dump_semantic_result_arbitration_truth_packet
```
