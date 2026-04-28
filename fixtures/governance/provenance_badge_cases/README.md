# Provenance badge contract worked cases

These fixtures validate the contract in
[`/docs/governance/provenance_badge_contract.md`](../../../docs/governance/provenance_badge_contract.md)
and the schema at
[`/schemas/governance/provenance_badge.schema.json`](../../../schemas/governance/provenance_badge.schema.json).

Every JSON file is a single `provenance_badge_record`. The records are
pre-implementation governance artifacts: they carry opaque refs,
controlled vocabulary values, privacy-safe notes, and export projections,
but no raw artifact bytes, raw signatures, raw SBOM bodies, raw notice
text, raw registry URLs, raw advisory payloads, raw private mirror
endpoints, or raw customer identifiers.

| Fixture | Source class | Scenario |
|---|---|---|
| [`signed_official_release_asset.json`](./signed_official_release_asset.json) | `official` | Signed official release artifact with verified signature, attestation, SBOM, notices, and current trust root |
| [`mirrored_official_offline_bundle.json`](./mirrored_official_offline_bundle.json) | `mirrored` | Official artifact consumed from an offline mirror with origin verification, receipt refs, and trust-root continuity |
| [`side_loaded_local_archive.json`](./side_loaded_local_archive.json) | `side_loaded` | User-supplied local archive with checksum evidence, unverified signature, missing attestation, and unsupported status |
| [`third_party_import_notice_partial.json`](./third_party_import_notice_partial.json) | `third_party_import` | Mirrored third-party import with dependency/import ledger refs, allowed license, partial notice inventory, and upstream health watch |
| [`stale_review_advisory_history.json`](./stale_review_advisory_history.json) | `official` | Official artifact whose review is stale and whose advisory history remains visible after remediation |
| [`unknown_provenance_unsupported.json`](./unknown_provenance_unsupported.json) | `unknown_provenance` | Unknown provenance record that keeps unsupported, unknown, missing, and not-reviewed states explicit |
