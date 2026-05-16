# Extension publication pipeline beta

This document is the reviewer-facing contract for publishing a beta
extension artifact into a registry lane. The machine-readable source is
[`schemas/extensions/publication_pipeline.schema.json`](../../../schemas/extensions/publication_pipeline.schema.json),
the headless entrypoint is
[`tools/extensions/m3/publish_extension.py`](../../../tools/extensions/m3/publish_extension.py),
and the checked packet lives under
[`artifacts/extensions/m3/publication_pipeline/`](../../../artifacts/extensions/m3/publication_pipeline/).

The pipeline is intentionally narrow: it packages one extension artifact,
binds it to a signer, attaches provenance and compatibility metadata,
promotes it through the governed registry channels, and preserves the
previous installable artifact for rollback. It does not add marketplace
ranking, recommendations, vanity listing fields, or new permission
classes.

## Required packet truth

Every publication packet carries:

| Field family | Required truth |
|---|---|
| Version metadata | extension identity, package id, publisher id, extension version, manifest schema version, SDK line, host version range, support class, bridge state |
| Artifact metadata | artifact ref, registry manifest ref, permission manifest ref, runtime contract ref, digest algorithm, digest, byte size |
| Signer metadata | signer ref, key fingerprint, signature ref, signature class, signed content address, signing timestamp |
| Provenance metadata | builder id, source manifest ref, source revision ref, build run ref, conformance report ref, SDK release bundle ref, subject content address |
| Compatibility metadata | compatibility report ref, host contract family refs, capability world refs, target platforms, support class, bridge state |
| Promotion steps | monotone channel movement, evidence refs, approver refs, signature ref, and `preserves_artifact_identity = true` |
| Rollback plan | previous version, previous registry manifest ref, previous content address, rollback manifest ref, and preservation flags |
| Failure guard | staging catalog ref, target catalog ref, guarded write class, catalog-after-verification rule, revocation-after-commit rule, zero orphaned revocation states |

The publication record refuses unsigned artifacts, missing provenance,
missing compatibility metadata, digest drift between package/signature/
provenance/promotion rows, rollback plans that do not preserve the prior
installable artifact, unsafe partial writes, and orphaned revocation
state.

## Headless usage

Validate the checked fixture suite:

```text
python3 tools/extensions/m3/publish_extension.py \
  --repo-root . \
  validate-fixtures \
  --fixtures-dir fixtures/extensions/m3/publication_pipeline \
  --report artifacts/compat/m3/extension_publication_pipeline_report.json \
  --check
```

Validate the checked publication packet:

```text
python3 tools/extensions/m3/publish_extension.py \
  --repo-root . \
  validate-packet \
  --packet artifacts/extensions/m3/publication_pipeline/publication_pipeline_record.json
```

Rebuild the checked packet from the sample manifest and sample package:

```text
python3 tools/extensions/m3/publish_extension.py \
  --repo-root . \
  build-packet \
  --manifest fixtures/extensions/m3/conformance_kit/wasm_install_activation_permission_prompt_pass.json \
  --artifact fixtures/extensions/m3/publication_pipeline/sample_artifacts/wasm_notes_1.0.0-beta.1.aurext \
  --out-dir artifacts/extensions/m3/publication_pipeline \
  --previous-version 1.0.0-beta.0 \
  --previous-registry-manifest-ref registry_manifest:dev.aureline.samples/wasm-notes:1.0.0-beta.0:public \
  --previous-digest-hex 40cb1a93fd5e1b3463d0fbac6bbde42564da49bc6aa163dc3b49dd8f9e21e7b2 \
  --previous-digest-size-bytes 68120 \
  --source-revision-ref git:dev.aureline.samples/wasm-notes@fixture \
  --build-run-ref build_run:extensions:wasm-notes:2026-05-16 \
  --conformance-report-ref conformance_report:dev.aureline.samples/wasm-notes:1.0.0-beta.1:pass \
  --security-review-ref security_review:dev.aureline.samples/wasm-notes:1.0.0-beta.1 \
  --mirror-rehearsal-ref mirror_rehearsal:dev.aureline.samples/wasm-notes:1.0.0-beta.1 \
  --rollback-drill-ref rollback_drill:dev.aureline.samples/wasm-notes:1.0.0-beta.0-to-1.0.0-beta.1 \
  --sdk-release-bundle-ref sdk_release_bundle:aureline.sdk.beta:1.0.0-beta.1 \
  --force
```

The tool writes sidecar metadata first and writes `catalog_snapshot.json`
last. A refused publication exits non-zero and does not write a catalog
snapshot, which prevents a half-published catalog row from becoming the
source of install truth.

## Checked outputs

| Output | Purpose |
|---|---|
| `publication_pipeline_record.json` | canonical packet evaluated by the Rust model and CLI |
| `publication_support_export.json` | metadata-safe projection for support and partner handoff |
| `registry_manifest_row.json` | registry row with content address, signature ref, compatibility, and revocation snapshot |
| `promotion_rows.json` | channel-promotion rows that preserve artifact identity |
| `rollback_manifest.json` | previous installable artifact and rollback manifest refs |
| `catalog_snapshot.json` | final catalog pointer written only after metadata validation succeeds |

## Fixture drills

The fixture suite covers:

| Fixture | Expected result |
|---|---|
| `ready_signed_provenance_rollback_safe.json` | signed, provenance-bound, compatibility-backed, production promotion with rollback |
| `refused_missing_rollback_target.json` | production promotion is refused because the prior installable artifact is missing |
| `refused_identity_mutation.json` | promotion is refused because the promotion digest differs from the signed artifact digest |

The Rust crate mirrors the same decisions through
`aureline_extensions::evaluate_extension_publication_pipeline`, and the
support export is produced through
`aureline_extensions::project_extension_publication_support_export`.
