# Scanner Import Beta Fixtures

These fixtures exercise the shared scanner-import lane for SARIF and a
structured scanner JSON shape that does not require a native SARIF producer.

- `structured_import_request.json` binds the structured payload to tool,
  target, revision, baseline, suppression, waiver, and local-confirmation
  state.
- `structured_scanner_output.json` contains the scanner-shaped run and result
  rows consumed by the runtime normalizer.

The runtime tests also reuse the SARIF fixture in `fixtures/quality/sarif_alpha`
to prove the older import path now emits the same review, CLI, support, and
release packet labels.
