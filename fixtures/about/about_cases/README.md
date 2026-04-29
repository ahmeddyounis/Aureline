# About-card and reproducibility-packet fixture cases

These fixtures validate the contract in
[`/docs/about/about_provenance_and_boundary_contract.md`](../../../docs/about/about_provenance_and_boundary_contract.md)
and the schemas at:

- [`/schemas/about/about_card.schema.json`](../../../schemas/about/about_card.schema.json)
- [`/schemas/about/reproducibility_packet.schema.json`](../../../schemas/about/reproducibility_packet.schema.json)

Every YAML file is either a single `about_card_record` or a single
`reproducibility_packet_record`, plus a `__fixture__` prelude. The
records are pre-implementation governance artifacts: they carry opaque
refs, controlled vocabulary values, privacy-safe sentences, and
typed states only. Raw signatures, raw attestations, raw key material,
raw SBOM bodies, raw notice text, raw URLs, raw issue-template bodies,
raw advisory payloads, raw private mirror endpoints, raw policy bodies,
raw tenant identifiers, and raw user identifiers do not appear.

| Fixture | Record | Scenario |
|---|---|---|
| [`open_local_independent_build.yaml`](./open_local_independent_build.yaml) | `about_card_record` | Open-source local-independent build: stable channel, signature verified, no managed or optional services active, public-tracker / public-RFC / private-security / private-support routes wired. |
| [`managed_with_optional_services.yaml`](./managed_with_optional_services.yaml) | `about_card_record` | Managed-cloud build with optional managed AI provider and optional symbol-service active for the workflow; auto-narrowed from the declared open posture. |
| [`mirrored_offline_air_gapped_build.yaml`](./mirrored_offline_air_gapped_build.yaml) | `about_card_record` | Air-gapped customer-mirror build: mirrored official offline posture active, current customer-mirror freshness, public lanes pointed at the public-summary-on-fix posture. |
| [`reproducibility_packet_export.yaml`](./reproducibility_packet_export.yaml) | `reproducibility_packet_record` | Frozen `regulated_review_export` packet for the air-gapped build, with offline-bundle receipts, SBOM refs, source-bundle symbols, and `regulated_review_restricted` redaction. |

Case ids are stable and quoted by the contract document at
`docs/about/about_provenance_and_boundary_contract.md`.
