# M5 workflow-bundle manifests

This document is the contract for the `m5_workflow_bundle_manifests_packet`. The canonical packet
is checked in at `artifacts/workspace/m5/m5-workflow-bundle-manifests.json`, validated by
`schemas/workspace/m5-workflow-bundle-manifests.schema.json`, and backed by the typed model in the
`aureline-workspace` crate (`m5_workflow_bundle_manifests`).

## What the packet governs

Every M5 launch wedge — notebook, data/API, profiler, framework-pack, docs, companion,
sync-handoff, and opened local folder — is composed as a real, versioned **workflow bundle**
instead of an ad hoc pile of templates, extensions, docs, and setup instructions. Each
[`WorkflowBundleManifest`] captures the minimum cohesive experience for a named **persona**,
**stack**, and **archetype** and answers:

- **What the bundle is made of** — a single diffable `components` list. Every component carries a
  `component_kind` drawn from one closed set spanning every content category: `extension`,
  `profile_preset`, `layout_preset`, `settings_preset`, `task_recipe`, `launch_recipe`,
  `debug_recipe`, `docs_pack`, `tour_pack`, `template_ref`, `scaffold_ref`, and
  `migration_mapping`. The categories stay distinct so discovery and review never collapse a docs
  pack into a recipe or a scaffold into a migration.
- **What lifecycle it depends on** — each component records a `lifecycle_stage` of `stable`,
  `preview`, `labs`, `policy_gated`, `mirror_only`, or `bounded_platform`. A non-stable stage is a
  lifecycle-sensitive dependency marker.
- **What certification it aims at** — a `certification_target` of `certified`, `managed_approved`,
  `community_reviewed`, `imported_pending_review`, or `local_draft`.
- **Where it is in its own lifecycle** — a `publication_state` of `draft`, `published`, or
  `deprecated`.

## A bundle is the minimum cohesive experience

A manifest is not a loose bag of presets. Every manifest must capture at least an **extension set**
and at least one **runnable recipe** (task, launch, or debug) —
[`WorkflowBundleManifest::has_minimum_cohesive_experience`]. Anything less is not a cohesive
workflow and is rejected by `validate()`.

## Non-stable dependencies are declared, never hidden

A bundle may depend on a Preview, Labs, policy-gated, mirror-only, or bounded-platform capability —
but it must say so. Each non-stable component:

- is **review-gated** (`requires_review: true`), and
- is rolled up into the manifest's `dependency_markers` (the distinct non-stable stages it depends
  on, sorted).

A manifest that depends on any non-stable capability must set
`discloses_non_stable_dependencies: true`; the disclosure flag and the dependency markers are both
recomputed and checked, so a hidden Preview or mirror-only dependency is a validation failure. A
bundle may still aim at a `certified` target while disclosing a non-stable dependency (for example,
a certified framework pack that openly carries a Preview AI-assist extension) — it can never bury
the dependency.

## Diffable, mirrorable, export-safe — never opaque

Every manifest holds `diffable`, `mirrorable`, and `export_safe` true, and holds the
`opaque_binary_state` guardrail false. Every component is `diffable` too. Opaque binary bundle
state, hidden installers, and private registry search recipes are forbidden on these claimed paths:
one manifest can always be diffed against another, mirrored offline, and exported into a support
packet. The packet is metadata-only — every field is a typed state, a count, or an opaque ref, and
it carries no credential bodies, raw provider payloads, raw local paths, or bundle binary contents.

## Targets never out-rank publication

A `draft` manifest may only claim a `local_draft` target
([`WorkflowBundleManifest::target_within_publication`]); a bundle that claims any stronger posture
must be `published`. Only a `certified` target presents as certified
([`WorkflowBundleManifest::presents_as_certified`]). Any non-certified target, any non-stable
dependency, any review-gated component, and any `deprecated` manifest must carry a caveat, so the
weaker posture is never silent.

## One manifest, many consumers

The same packet drives discovery, review, diagnostics, and claim publication. Each manifest carries
a `start_center_ref`, a `migration_center_ref`, a `bundle_detail_ref`, plus `diagnostics_ref`,
`support_export_ref`, `help_surface_ref`, and `release_evidence_ref`, alongside
`manifest_provenance_ref`, `certification_evidence_ref`, and `migration_provenance_ref`. Start
center, migration center, bundle detail pages, and release/help surfaces ingest this one
manifest-backed object model instead of cloning status text.

## How it is validated

The typed model parses the embedded packet and runs `validate()`, which checks the closed
vocabularies, full wedge coverage, the diffable/mirrorable/export-safe and non-opaque guardrails,
the non-stable disclosure recomputation and dependency markers, per-component consistency
(diffable, review-gating of non-stable stages, and migration `from`/`to` populated exactly for
migration mappings), the minimum-cohesive-experience floor, the target-within-publication rule,
complete provenance, required caveats, and the recomputed summary. The unit tests in
`crates/aureline-workspace/src/m5_workflow_bundle_manifests/tests.rs` assert the embedded packet
validates clean and that every wedge, certification target, component kind, and lifecycle stage is
exercised.

[`WorkflowBundleManifest`]: ../../../crates/aureline-workspace/src/m5_workflow_bundle_manifests/mod.rs
[`WorkflowBundleManifest::has_minimum_cohesive_experience`]: ../../../crates/aureline-workspace/src/m5_workflow_bundle_manifests/mod.rs
[`WorkflowBundleManifest::target_within_publication`]: ../../../crates/aureline-workspace/src/m5_workflow_bundle_manifests/mod.rs
[`WorkflowBundleManifest::presents_as_certified`]: ../../../crates/aureline-workspace/src/m5_workflow_bundle_manifests/mod.rs
