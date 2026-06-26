# Boundary-wording copy-parity lint

`check_copy_parity.py` is the CI-facing gate for the boundary-wording catalog. It
reads the checked-in support export
(`artifacts/content/m5-boundary-wording-proof/support_export.json`) and fails when
hosted/managed/premium/self-hosted/local-only/BYOK/trial wording drifts across
surfaces or is dishonest about the actual product boundary.

It re-derives, in pure Python, the same rules the Rust catalog
(`aureline_shell::content::boundary_wording`) enforces, so release/docs/help/UI review
can fail on copy-parity or boundary-honesty drift **even when the underlying feature
code still works**:

- **Copy parity** — every surface that renders the same `concept_id` must agree on
  the boundary term, the support metadata, the identity/network/data/export/rollback
  postures, the local-capability posture, and the disclosed alternatives.
- **No overstatement** — a term can never claim more local independence than the
  actual product boundary provides.
- **No false vendor dependence** — a managed/paid claim with a local-capable core must
  disclose a local/BYOK/self-hosted alternative.
- **Machine-anchored moves** — a claim that narrows or widens a boundary must
  reference compatibility/support metadata.
- **Export/rollback retained** — a managed/paid introduction keeps an export and
  rollback route, and upgrade/account/help surfaces disclose the alternatives.

When `jsonschema` is installed it also validates the export against
`schemas/content/m5-boundary-wording.schema.json`.

## Usage

```sh
# Lint the checked-in export (exit 1 on any drift/dishonesty).
python3 scripts/content/m5-copy-parity-lint/check_copy_parity.py

# Write a machine-readable report.
python3 scripts/content/m5-copy-parity-lint/check_copy_parity.py --report parity_report.json

# Prove the lint catches injected drift without a checked-in bad fixture.
python3 scripts/content/m5-copy-parity-lint/check_copy_parity.py --self-test
```

The export and fixtures are minted from truth by
`aureline_shell_m5_boundary_wording`; never hand-edit them.
