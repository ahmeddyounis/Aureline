# M5 interchange import/export validators

These are the per-family **import/export validator descriptors** for the M5 interchange families. They are rendered from one source — the interchange-conformance register at `artifacts/contracts/m5-interchange-conformance.json` — by `tools/regenerate_m5_interchange_conformance.py` and checked by `tools/validate_m5_interchange_conformance.py`. Each descriptor names the family's contract version, conformance class, the real emitted artifact its cross-surface runner exercises, the consumer surfaces that must agree, and the stable, copy-safe reason codes an interchange failure reports.

## Validators

| Family | Class | Emitted artifact | Reason codes |
| --- | --- | --- | --- |
| [`request_api_collections.json`](request_api_collections.json) | `round_trip_write_back` | [`request_api_collections.json`](../../fixtures/contracts/m5-interchange/emitted/request_api_collections.json) | `unsupported_contract_version`, `schema_validation_failed`, `trust_widening_blocked`, `round_trip_mismatch` |
| [`notebook_parity_exports.json`](notebook_parity_exports.json) | `compare_only` | [`notebook_parity_exports.json`](../../fixtures/contracts/m5-interchange/emitted/notebook_parity_exports.json) | `schema_validation_failed`, `round_trip_mismatch`, `unknown_field_unpreserved` |
| [`docs_packets.json`](docs_packets.json) | `import_validation_only` | [`docs_packets.json`](../../fixtures/contracts/m5-interchange/emitted/docs_packets.json) | `missing_required_provenance`, `schema_validation_failed`, `redaction_class_conflict` |
| [`trace_profile_replay_exports.json`](trace_profile_replay_exports.json) | `round_trip_write_back` | [`trace_profile_replay_exports.json`](../../fixtures/contracts/m5-interchange/emitted/trace_profile_replay_exports.json) | `unsupported_contract_version`, `corrupt_or_truncated_payload`, `round_trip_mismatch` |
| [`support_bundles.json`](support_bundles.json) | `import_validation_only` | [`support_bundles.json`](../../fixtures/contracts/m5-interchange/emitted/support_bundles.json) | `missing_required_provenance`, `schema_validation_failed`, `redaction_class_conflict` |
| [`portable_state_packages.json`](portable_state_packages.json) | `round_trip_write_back` | [`portable_state_packages.json`](../../fixtures/contracts/m5-interchange/emitted/portable_state_packages.json) | `unsupported_contract_version`, `trust_widening_blocked`, `round_trip_mismatch`, `unknown_field_unpreserved` |

## How a validator fails or narrows an import

Each validator validates the emitted artifact against its family's contract schema, confirms the contract version is supported, confirms required provenance is present, refuses to widen trust, and — for a round-trip family — confirms the artifact round-trips without dropping unknown fields. Any failure reports a stable reason code from the closed vocabulary, never a raw parser exception. A failed required check on a release-blocking family holds promotion; a downgraded check narrows the family.
