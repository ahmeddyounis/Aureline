# Scanner Import Alpha Fixtures

These fixtures exercise the scanner-import alpha lane:

- `scanner_output.sarif.json` is the bounded SARIF-shaped input.
- `import_request.json` binds the raw payload to tool, target, revision,
  baseline, suppression, waiver, and local-confirmation state.

The expected normalized output keeps scanner findings read-only, preserves
raw payloads by opaque ref, emits `new`, `resolved`, `persisting`,
`suppressed`, `waived`, and `unmapped` delta states, and excludes raw source
paths or scanner bodies from support-export projections.
