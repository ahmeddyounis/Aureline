# Extension conformance kit and validator

This page is the reviewer-facing entrypoint for the beta extension
conformance kit. The kit turns the SDK, manifest, permission, lifecycle,
runtime, and rollback contracts into a repeatable command that extension
authors can run locally and maintainers can run in CI before publication.

The governed artifacts are:

- Validator CLI:
  [`tools/extensions/m3/validator_cli/aureline_extension_validator.py`](../../../tools/extensions/m3/validator_cli/aureline_extension_validator.py)
- Author manifest schema:
  [`schemas/extensions/beta_extension_manifest.schema.json`](../../../schemas/extensions/beta_extension_manifest.schema.json)
- Lifecycle metadata schema:
  [`schemas/extensions/lifecycle_metadata.schema.json`](../../../schemas/extensions/lifecycle_metadata.schema.json)
- Canonical lifecycle packet:
  [`artifacts/extensions/m3/lifecycle_metadata_packet.json`](../../../artifacts/extensions/m3/lifecycle_metadata_packet.json)
- Validator report schema:
  [`schemas/extensions/conformance_kit_report.schema.json`](../../../schemas/extensions/conformance_kit_report.schema.json)
- Fixture suite:
  [`fixtures/extensions/m3/conformance_kit/suite.json`](../../../fixtures/extensions/m3/conformance_kit/suite.json)
- CI compatibility packet:
  [`artifacts/compat/m3/extension_conformance_kit_report.json`](../../../artifacts/compat/m3/extension_conformance_kit_report.json)

The validator is intentionally local and dependency-light. It reads JSON
manifests, emits stable JSON reports, and returns a non-zero exit code
when blocker checks fail or when the committed suite report drifts.

## What the validator checks

The validator emits one `extension_conformance_report` for a manifest
or one `extension_conformance_suite_report` for a fixture suite. Checks
are grouped into five suites:

| Suite | Contract checked | Blocks when |
|---|---|---|
| `manifest_shape` | Required manifest fields, schema version, package id, publisher id, semver, runtime origin, host family | Required identity or shape is missing, publisher is opaque, or runtime vocabulary is unknown |
| `permission_declarations` | Declared permission scopes, purpose text, targets, trust-mode behavior, review requirement, network endpoint class, secret-handle posture | Privileged scope lacks purpose, prompt copy, review gating, endpoint class, or handle-only secret posture |
| `lifecycle_metadata` | Lifecycle state, activation triggers, activation budget, degraded path, disable support, rollback target | Activation, degraded path, disable, or rollback truth is missing |
| `compatibility_targets` | SDK line, SDK semver, lifecycle metadata refs, host ABI window, WIT worlds or external-host contract, Aureline version range, platform rows, support class, bridge state | Host/runtime claims do not match the SDK beta contract or the compatibility target is incomplete |
| `conformance_fixtures` | Install, activation, permission prompt, degraded path, and disable or rollback fixture coverage | Required scenario coverage is missing or fixture refs are empty |

The report carries a closed `result_class`: `pass`, `warn`, or `fail`.
Failed blocker checks emit `unsupported_pending_qualification`; passing
reports emit `compatible_on_declared_targets`.

## Fixture coverage

The checked-in suite covers both positive and negative rows:

| Case | Purpose |
|---|---|
| `wasm_install_activation_permission_prompt_pass` | Valid Wasm component manifest covering install, lazy activation, prompt copy, degraded behavior, disable, and rollback |
| `external_host_degraded_disable_rollback_pass` | Valid supervised external-host manifest covering helper launch, network review, degraded behavior, disable, and rollback |
| `permission_prompt_missing_purpose_fail` | Negative permission prompt fixture: privileged network permission lacks purpose text |
| `compatibility_target_mismatch_fail` | Negative compatibility fixture: Wasm runtime claims an external-host contract family |
| `rollback_missing_checkpoint_fail` | Negative lifecycle fixture: rollback support is claimed without a last-known-good target |

The suite report is green only when every fixture produces its expected
result and the aggregate fixture set covers all required scenario
classes.

## CLI usage

Validate one manifest:

```text
python3 tools/extensions/m3/validator_cli/aureline_extension_validator.py \
  --repo-root . \
  validate-manifest \
  --manifest fixtures/extensions/m3/conformance_kit/wasm_install_activation_permission_prompt_pass.json \
  --format json
```

Validate the committed suite and fail on report drift:

```text
python3 tools/extensions/m3/validator_cli/aureline_extension_validator.py \
  --repo-root . \
  validate-suite \
  --suite fixtures/extensions/m3/conformance_kit/suite.json \
  --report artifacts/compat/m3/extension_conformance_kit_report.json \
  --check
```

Refresh the compatibility packet after an intentional change:

```text
python3 tools/extensions/m3/validator_cli/aureline_extension_validator.py \
  --repo-root . \
  validate-suite \
  --suite fixtures/extensions/m3/conformance_kit/suite.json \
  --report artifacts/compat/m3/extension_conformance_kit_report.json
```

Validate the lifecycle metadata packet and fail on report drift:

```text
python3 tools/extensions/m3/validator_cli/aureline_extension_validator.py \
  --repo-root . \
  validate-lifecycle-packet \
  --packet artifacts/extensions/m3/lifecycle_metadata_packet.json \
  --report artifacts/compat/m3/extension_lifecycle_metadata_report.json \
  --check
```

## Relationship to existing contracts

The validator does not replace the existing Rust-owned records. It is
the external author and CI entrypoint that checks an authored manifest
before those records are emitted or ingested:

- Manifest and publisher baseline:
  [`docs/extensions/m1_permission_and_publisher_baseline.md`](../m1_permission_and_publisher_baseline.md)
- Permission manifest and re-consent delta:
  [`docs/extensions/m3/permission_manifest_beta.md`](./permission_manifest_beta.md)
- Runtime admission contract:
  [`docs/extensions/m3/runtime_v1_beta.md`](./runtime_v1_beta.md)
- SDK starter pack:
  [`docs/extensions/m3/sdk_v1/README.md`](./sdk_v1/README.md)
- SDK publication and conformance-result envelope:
  [`docs/extensions/sdk_publication_contract.md`](../sdk_publication_contract.md)

## Degraded paths

A manifest passes degraded-path checks only when it declares a typed
degraded behavior and explicitly preserves core editing. The validator
accepts these behavior classes:

- `read_only_degrade`
- `disable_background_work`
- `disable_until_review`
- `quarantine_pending_review`

The disabled or degraded extension must remain inspectable. The manifest
must also declare disable support that preserves user state and a
rollback target that points at a last-known-good extension version.

## CI contract

The CI form is the suite command with `--check`. It compares the
generated report to
`artifacts/compat/m3/extension_conformance_kit_report.json` byte for
byte, so changing validator behavior or fixtures requires an explicit
packet refresh in the same change.
