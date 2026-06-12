# M5 workflow-bundle manifests — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-workflow-bundle-manifests.json`. The full contract lives in
`docs/workspace/m5/m5-workflow-bundle-manifests.md`; the typed model lives in the
`aureline-workspace` crate (`m5_workflow_bundle_manifests`).

This artifact freezes one versioned **workflow-bundle manifest** per M5 launch wedge. Each manifest
captures the minimum cohesive experience for a named persona, stack, and archetype as a single
diffable component list, declares its lifecycle-sensitive dependencies instead of hiding them, and
stays diffable, mirrorable, and export-safe.

## Manifest roll-up (as of 2026-06-11)

| Bundle | Version | Wedge | Target | Publication | Components | Dependency markers |
| --- | --- | --- | --- | --- | --- | --- |
| `m5-wbm:notebook` | 1.0.0 | notebook | certified | published | 5 | — |
| `m5-wbm:data-and-api` | 1.2.0 | data/API | managed_approved | published | 5 | policy_gated |
| `m5-wbm:profiler` | 0.9.0 | profiler | community_reviewed | published | 4 | bounded_platform |
| `m5-wbm:framework-pack` | 2.1.0 | framework-pack | certified | published | 4 | preview |
| `m5-wbm:docs` | 1.1.0 | docs | community_reviewed | published | 4 | — |
| `m5-wbm:companion` | 0.7.0 | companion | imported_pending_review | published | 4 | mirror_only |
| `m5-wbm:sync-handoff` | 0.3.0 | sync-handoff | local_draft | **draft** | 4 | labs |
| `m5-wbm:local-folder` | 1.4.0 | local folder | community_reviewed | **deprecated** | 4 | — |

## What the manifests prove

- **Real versioned bundles, not ad hoc combinations.** Each wedge resolves to one versioned
  manifest with a typed component list spanning extension sets, presets, recipes, docs/tour packs,
  template/scaffold refs, and migration mappings — every component kind in the closed set is
  exercised.
- **Minimum cohesive experience.** Every manifest carries at least an extension set and a runnable
  recipe (task, launch, or debug), so no bundle is an empty pile of presets.
- **Non-stable dependencies are disclosed.** Five manifests depend on a non-stable capability —
  one each of `policy_gated`, `bounded_platform`, `preview`, `mirror_only`, and `labs`. Each sets
  `discloses_non_stable_dependencies` and rolls the stage into `dependency_markers`, and each
  non-stable component is review-gated. The certified framework-pack bundle openly carries a
  Preview AI-assist extension — the dependency is visible to discovery, review, and export, never
  buried.
- **Diffable and never opaque.** Every manifest and every one of the 34 components is `diffable`;
  every manifest is `mirrorable` and `export_safe`; `opaque_binary_state` is `false` everywhere.
- **Targets never out-rank publication.** The draft sync-handoff bundle claims only a `local_draft`
  target; the other targets (`certified` ×2, `managed_approved`, `community_reviewed` ×3,
  `imported_pending_review`) all sit on published or deprecated manifests. Only the two `certified`
  bundles present as certified.
- **Weaker posture is never silent.** Every non-certified, non-stable-dependent, review-gated, or
  deprecated manifest carries a caveat.
- **One object model, many consumers.** Every manifest carries start-center, migration-center,
  bundle-detail, diagnostics, support-export, help-surface, and release-evidence refs plus manifest,
  certification, and migration provenance, so discovery, review, diagnostics, and claim publication
  ingest the same packet.

## How it is validated

The typed model parses the embedded packet and runs `validate()`, which checks the closed
vocabularies, full wedge coverage, the diffable/mirrorable/export-safe and non-opaque guardrails,
the non-stable disclosure recomputation and dependency markers, per-component consistency, the
minimum-cohesive-experience floor, the target-within-publication rule, complete provenance, required
caveats, and the recomputed summary. See
`crates/aureline-workspace/src/m5_workflow_bundle_manifests/tests.rs`.
