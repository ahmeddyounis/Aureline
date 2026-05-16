# Extension Validator CLI

This directory contains the headless validator for beta extension
manifests and the checked-in conformance fixture suite.

## Validate one manifest

```text
python3 tools/extensions/m3/validator_cli/aureline_extension_validator.py \
  --repo-root . \
  validate-manifest \
  --manifest fixtures/extensions/m3/conformance_kit/wasm_install_activation_permission_prompt_pass.json \
  --format text
```

Use `--format json` for CI, registry ingest, or author tooling that
needs the machine-readable report.

## Validate the in-repo suite

```text
python3 tools/extensions/m3/validator_cli/aureline_extension_validator.py \
  --repo-root . \
  validate-suite \
  --suite fixtures/extensions/m3/conformance_kit/suite.json \
  --report artifacts/compat/m3/extension_conformance_kit_report.json \
  --check
```

Drop `--check` to refresh the report after intentionally changing the
suite, schema, or validator behavior.

## Contracts

- Author manifest schema:
  `schemas/extensions/beta_extension_manifest.schema.json`
- Validator report schema:
  `schemas/extensions/conformance_kit_report.schema.json`
- Reviewer guide:
  `docs/extensions/m3/conformance_kit_beta.md`
