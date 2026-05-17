# Workflow Bundle Review Beta

This document defines the workspace-owned review packet for workflow-bundle install, update, drift, override, remove, rollback, diagnostics, support export, and CLI/headless parity.

Companion artifacts:

- Schema: [`schemas/workspace/workflow_bundle_review.schema.json`](../../../schemas/workspace/workflow_bundle_review.schema.json)
- Rust module: [`crates/aureline-workspace/src/bundles/mod.rs`](../../../crates/aureline-workspace/src/bundles/mod.rs)
- Fixtures: [`fixtures/workspace/m3/workflow_bundle_review/`](../../../fixtures/workspace/m3/workflow_bundle_review/)
- Compatibility examples: [`artifacts/compat/m3/workflow_bundle_examples/`](../../../artifacts/compat/m3/workflow_bundle_examples/)

## Purpose

Workflow bundles already have governed manifest, change-preview, drift-row, and compatibility scorecard artifacts. This packet is the workspace boundary that composes those records into one reviewable unit before any durable mutation happens.

The packet is intentionally support-safe. It carries opaque refs, component classes, ownership classes, evidence refs, checkpoint refs, and closed vocabulary values. It does not carry raw settings values, local paths, source content, extension binaries, secrets, command bodies, or signing material.

## Required Review Surfaces

Every packet preserves the same bundle identity across:

- Start Center and bundle detail
- install/update review
- drift banner and override review
- remove/rollback review
- diagnostics
- support export
- CLI/headless summaries
- docs/workspace references

The install/update review must show a diff before mutation across extension set, profile preset, surface preset, settings/token, task recipe, launch recipe, debug recipe, docs pack, tour pack, template/scaffold ref, migration mapping, and certification target.

## Source Classes

The packet keeps these source classes distinct:

- `certified`
- `managed_approved`
- `community`
- `imported`
- `local_draft`

Consumers may share layout patterns, but they must not collapse these into a generic bundle badge. Certification and support copy must use `effective_badge_class`, not the source badge alone.

## Drift And Overrides

Drift is recorded at field, package, task, component, certification-evidence, or mirror/offline-pack granularity. The drift surface does not authorize mutation by itself.

Enabled `resolve.adopt_bundle` and `resolve.rebase_to_bundle` actions must route to a `bundle_change_preview` ref. Local overrides must carry a retained override ref when the drift state is `local_override`.

## Removal And Rollback

Remove review classifies every asset as `bundle_owned`, `user_owned`, `shared_user_overlay_on_bundle`, or `mixed_unknown_provenance`.

User-owned assets are not safe to delete. Shared overlays either preserve the user overlay or require explicit co-resident user-data review. Bundle-owned assets may be removed only after they were explicitly shown in the review. Rollback checkpoint refs stay attached to the review packet and to support export.

## Guardrails

Bundles may recommend providers, remote modes, or templates. They may not silently widen workspace trust, network egress, policy scope, or approval defaults. The packet encodes those facts as booleans so UI, CLI, diagnostics, and support export cannot diverge.

## Verification

Run:

```sh
cargo test -p aureline-workspace --test workflow_bundle_review_beta
```

The test suite loads every fixture, projects it through the Rust validator, and exercises downgrade, ownership, raw-export, guardrail, and drift-routing failure cases.
