# M5 public-contract example payloads

This directory holds worked example payloads for the M5 public-contract matrix.

- [`contract_row_example.json`](contract_row_example.json) — a single
  `m5_public_contract_matrix` **row** showing the canonical shape of one
  inventoried family: its contract form, stability lane, reader/writer posture,
  packaging need, claim/published labels, per-form publication requirements,
  validator suite, release-packet linkage, and active gap reasons. It validates
  against `#/$defs/row` of
  `schemas/public/m5-contracts/m5_public_contract_matrix.schema.json`.

The full worked example is the canonical matrix itself:
`artifacts/contracts/m5-stability-lifecycle-map.json`, validated by
`tools/validate_m5_public_contract_matrix.py` against the same schema. The flat
projections live alongside it at
`artifacts/contracts/m5-public-contract-inventory.csv` and
`artifacts/contracts/m5-public-contract-matrix.md`.
