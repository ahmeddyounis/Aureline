# Generated artifacts + lineage honesty (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:generated_artifacts_safe_handling`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.npm_pnpm_toolchain`

## Scenario goal

Prove that generated artifacts in TS/JS workflows do not masquerade as
canonical source across core surfaces (search/open/review/AI/export/support).

This scenario covers (non-exhaustive):

- lockfiles and resolved manifests;
- build outputs (for example `dist/`, framework output caches);
- generated declarations and source-map artifacts;
- preview/runtime projections.

## Required truth and disclosures

- Generated-artifact edit posture is governed by one shared posture record
  and one shared vocabulary:
  - `docs/architecture/generated_artifact_safe_edit_policy.md`
  - `schemas/generated/artifact_edit_posture.schema.json`
- Drift/regeneration cases exist so reviewers can see downgrade behavior:
  - `fixtures/generated/drift_regeneration_manifest.yaml`

## Evidence hooks

- Support bundles must preserve the same provenance/edit-posture fields:
  - `docs/support/support_bundle_contract.md`

## Known-limit expectations

- Any “safe edit” exception (for example a lockfile that becomes
  round-trip-safe under a declared editor) must land as an explicit policy
  and known-limit note before it can widen certified wording.

