# Fixtures: M5 workflow-bundle manifests

This directory contains fixture metadata for the `m5_workflow_bundle_manifests_packet`.

The canonical full corpus is checked in at:

`artifacts/workspace/m5/m5-workflow-bundle-manifests.json`

and is validated by `schemas/workspace/m5-workflow-bundle-manifests.schema.json` and the typed
model in the `aureline-workspace` crate (`m5_workflow_bundle_manifests`).

## Coverage

- One versioned workflow-bundle manifest per M5 launch wedge: `notebook_workspace`,
  `data_and_api_workspace`, `profiler_workspace`, `framework_pack_workspace`, `docs_workspace`,
  `companion_workspace`, `sync_handoff_workspace`, and `local_folder_workspace`. Every wedge is
  covered (`covers_every_wedge`), so no wedge falls back to an ad hoc combination of templates,
  extensions, docs, and setup instructions.
- All twelve component kinds are exercised and stay distinct: `extension`, `profile_preset`,
  `layout_preset`, `settings_preset`, `task_recipe`, `launch_recipe`, `debug_recipe`, `docs_pack`,
  `tour_pack`, `template_ref`, `scaffold_ref`, and `migration_mapping`. Every manifest captures a
  minimum cohesive experience (at least an extension set and a runnable recipe).
- All six lifecycle stages are exercised: `stable` everywhere plus one disclosed non-stable
  dependency each of `policy_gated` (data/API), `bounded_platform` (profiler), `preview`
  (framework-pack), `mirror_only` (companion), and `labs` (sync-handoff). Each non-stable component
  is review-gated, and the manifest discloses it via `discloses_non_stable_dependencies` and
  `dependency_markers`.
- All five certification targets are exercised: `certified` (notebook, framework-pack),
  `managed_approved` (data/API), `community_reviewed` (profiler, docs, local-folder),
  `imported_pending_review` (companion), and `local_draft` (sync-handoff). Only the two `certified`
  manifests present as certified.
- All three publication states are exercised: `published` (six), `draft` (sync-handoff), and
  `deprecated` (local-folder). The draft manifest claims only a `local_draft` target.
- Migration mappings carry a populated `migration_from`/`migration_to` pair (docs, sync-handoff);
  no other component kind carries those fields.
- Every manifest and every one of the 34 components is `diffable`; every manifest is `mirrorable`
  and `export_safe`; `opaque_binary_state` is `false` everywhere. Every non-certified, non-stable,
  review-gated, or deprecated manifest carries a caveat.

## How it is validated

The typed model parses the embedded packet and runs `validate()`, which checks the closed
vocabularies, full wedge coverage, the diffable/mirrorable/export-safe and non-opaque guardrails,
the non-stable disclosure recomputation and dependency markers, per-component consistency, the
minimum-cohesive-experience floor, the target-within-publication rule, complete provenance, required
caveats, and the recomputed summary. The unit tests in
`crates/aureline-workspace/src/m5_workflow_bundle_manifests/tests.rs` assert the embedded packet
validates clean and that every wedge, certification target, component kind, and lifecycle stage is
exercised.
