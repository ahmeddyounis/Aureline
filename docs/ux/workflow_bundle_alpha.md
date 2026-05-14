# Workflow Bundle Alpha Lifecycle

This note ties the existing workflow-bundle contracts to the first Start
Center lifecycle consumer. It does not create new bundle semantics; it names
which checked-in artifacts the alpha surface reads.

## Source Artifacts

- `schemas/workflow/bundle_manifest.schema.json` owns bundle identity,
  source class, lifecycle posture, mirror/offline posture, and component
  inventory.
- `schemas/workflow/bundle_change_preview.schema.json` owns
  install/update review, side-effect disclosure, and rollback checkpoint
  linkage.
- `schemas/workflow/bundle_drift_row.schema.json` owns local overrides,
  missing artifacts, version drift, resolve actions, and remove-bundle
  safety rows.
- `schemas/workflow/bundle_compatibility_scorecard.schema.json` owns the
  support-safe compatibility scorecard rows consumed by Start Center,
  migration handoff, docs/help, support export, and CLI summaries.
- `schemas/workflow/bundle_drift_packet.schema.json` groups drift-row refs
  and remove-review refs into one support-safe packet.

## First Consumer

`crates/aureline-shell/src/start_center/bundles/mod.rs` is the first runtime
consumer. It joins:

- `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`
- `artifacts/bundles/python_launch_bundle_alpha.yaml`
- `artifacts/compat/workflow_bundle_scorecard_sample.json`
- `artifacts/compat/bundle_drift_packet_sample.json`

The resulting Start Center lifecycle row exposes bundle ID, persona/stack
label, signer/source, channel, compatible Aureline range, scorecard status,
archetype refs, install/update/remove review refs, rollback checkpoint policy,
drift states/actions, explicit template/scaffold refs, and support-safe export
refs.

## Invariants

- Scorecard status uses the closed vocabulary: Certified, Managed approved,
  Community, Imported, Local draft, Partial, Retest pending, Blocked, and
  Status unknown.
- Template/scaffold refs remain explicit and mirrorable. The scorecard points
  at the template seed and generated-lineage contract and marks opaque
  generation behavior as disallowed.
- Drift actions preserve local state. Keep local, Adopt bundle, Compare, and
  Rebase to bundle are visible; durable adoption still routes through a
  bundle change preview.
- Remove-bundle review lists removable assets and retained local overrides.
  User-authored content is not exported from the support-safe packet.
- Support exports carry refs and redaction classes, not raw workspace files,
  raw source payloads, secrets, or signing material.
