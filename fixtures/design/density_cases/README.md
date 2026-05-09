# Density contract fixtures

Density cases bind the frozen density vocabulary (`compact`, `standard`, `comfortable`) to the geometry token ledger (`size.*`, `space.*`).

These fixtures are validated by:

- `schemas/design/density_case.schema.json`
- `tools/ci/validate_density_cases.py`

They exist to keep first‑party shell surfaces aligned on:

- which row/control sizing tokens are selected per density mode; and
- which spacing tokens are used for panel padding, zone insets, and gutters.

