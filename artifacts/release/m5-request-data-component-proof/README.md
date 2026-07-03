# M5 request/data component proof

This proof packet freezes the request/data component matrix for M5 reusable
request-editor, response-viewer, connection-browser, result-grid, and
explain-plan surfaces.

Primary files:

- `artifacts/design/m5-request-data-component-matrix.md`
- `artifacts/release/m5-request-data-component-proof/proof_packet.json`
- `artifacts/release/m5-request-data-component-proof/support_export.json`
- `fixtures/ui/m5-request-data-components/component_manifest.json`
- `fixtures/ui/m5-request-data-components/`
- `tools/ci/m5/request_data_component_check.py`

The proof narrows any consumer that fails controlled vocabulary parity, reduced
capability disclosure, secret redaction parity, copy/export parity, or
estimated-versus-actual plan truth.

M05-794 parity is part of the frozen proof:

- Keyboard traversal, screen-reader labeling, accessible text/table fallback,
  and 200% zoom/high-density behavior are required for every first consumer.
- CLI/headless and support exports join request runs, result sets, history
  rows, and plan objects through schema refs and stable support join ids.
- Missing, stale, or policy-blocked auth-source class, origin boundary, schema
  freshness, plan freshness, or export-redaction posture narrows the public
  claim and exports the same degraded-state reasons shown in the GUI.
