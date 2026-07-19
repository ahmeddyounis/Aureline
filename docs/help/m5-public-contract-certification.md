# M5 public-contract certification

Aureline certifies every claimed public artifact family with a single certification packet. For each family the packet records its published contract form, its lifecycle metadata (version + stability label), its example payload corpus, the validator that guards it, its compatibility report, and the release packet and build identity it shipped with — then decides whether the family certifies its contract claim or has narrowed below the certification cutline.

## Where the packet lives

- Machine-readable register: `artifacts/certification/m5-public-contract-certification.json`
- Human-readable report: `artifacts/certification/m5-public-contract-certification.md`
- Shiproom dashboard: `shiproom/m5-public-contract-certification-dashboard.md`
- JSON Schema: `schemas/public/m5-contracts/m5_public_contract_certification.schema.json`

## How to read a certification row

- **Claim** is the marketed lifecycle label; **certified** is the label the family may actually carry. When they differ, the family narrowed.
- **Pillars** are the six contract assets a family must publish to certify a contract claim. A `missing` required pillar on a release-blocking family withholds certification and holds promotion.
- **State** is one of: `certified`, `narrowed_row_downgraded`, `narrowed_stale`, `narrowed_retest_pending`, or `withheld`.

## Inspect it locally

```sh
cargo run -q -p aureline-release \
  --bin aureline_release_certify_schema_publication_wit_openapi -- inspect command_descriptors
```

The certification packet is consumed by claim-publication, release-center, support-center, and SDK/docs publication flows; it joins the contract-health register (`artifacts/release/m5-contract-health.json`) and the publication matrix (`artifacts/contracts/m5-stability-lifecycle-map.json`) rather than restating their field semantics.
