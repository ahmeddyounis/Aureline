# Certify schema publication, WIT/OpenAPI packaging, validator coverage, and compatibility truth on every claimed M5 public artifact family

This is the closeout row for the M5 public-contract publication lane. Earlier rows publish the individual contract forms — the JSON Schema catalog, the OpenAPI catalog, the WIT publication, the reader/writer compatibility suite, and the interchange-conformance register — plus the publication matrix that records whether each family published its required forms and the contract-health register that enforces those forms with CI gates and a release-graph linkage. This row certifies the whole lane: it joins all of them into one certification packet that proves every claimed M5 public artifact family has the contract assets its claim requires, and narrows or withholds any family whose contract packages are missing, stale, or mirror-incomplete.

## What the certification packet binds

For every claimed family the packet records one row binding the family to its **published contract form**, its **lifecycle metadata** (explicit version field and stability label), its **example corpus**, its **validator coverage**, its **compatibility report**, and its **release-graph linkage** (release packet plus the one build identity the candidate shipped). Each pillar carries its own evidence state (`current`, `stale`, or `missing`), so a stale compatibility report narrows a family while its schema and validator pillars stay current.

## How narrowing works

The certification reuses the contract-health register's per-family gate evaluation and the publication matrix's lifecycle labels rather than minting a new vocabulary. A family certifies only when every required pillar is current and its published label matches its public claim. A family may never certify a greener label than its public claim; a family whose public claim already narrowed inherits that narrowing; and a release-blocking family missing a required pillar withholds certification and holds promotion.

## Current state

**Certification decision: HOLD.** Certification is held: one or more release-blocking M5 public artifact families have a missing required contract pillar (published contract form, lifecycle metadata, example corpus, validator coverage, compatibility report, or release-graph linkage). Publishing the missing contract evidence and rerunning the gate clears the hold.

- 16 claimed families (8 release-blocking).
- 15 certified, 0 narrowed, 1 withheld.
- 95 pillars current, 0 stale, 1 missing across 96 evaluated.

## Sources and consumers

- Register (truth source): `artifacts/certification/m5-public-contract-certification.json`
- Report: `artifacts/certification/m5-public-contract-certification.md`
- Shiproom dashboard: `shiproom/m5-public-contract-certification-dashboard.md`
- Help-center page: `docs/help/m5-public-contract-certification.md`
- JSON Schema: `schemas/public/m5-contracts/m5_public_contract_certification.schema.json`
- Validator: `tools/validate_m5_public_contract_certification.py`
- Regenerator: `tools/regenerate_m5_public_contract_certification.py`

The packet is consumed by claim-publication, release-center, support-center, and SDK/docs publication flows; it is referenced against the canonical M5 evidence index at `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`.
