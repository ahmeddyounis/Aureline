# M5 interchange conformance report

Cross-surface import/export conformance summary for the M5 interchange families. It is rendered from one source — the interchange-conformance register at `artifacts/contracts/m5-interchange-conformance.json` — by `tools/regenerate_m5_interchange_conformance.py`, so support, release-center, and claim-publication packets resolve one interchange truth per family instead of restating field semantics. If this report and the register disagree, the register wins and both are regenerated together.

- Register: `artifacts/contracts/m5-interchange-conformance.json`
- Validator manifest: `validators/m5-interchange/manifest.json`
- Validators: `validators/m5-interchange/`
- Emitted-artifact corpus: `fixtures/contracts/m5-interchange/emitted/`
- Current as of: `2026-06-19`

## Promotion decision

**CLEAR** — No release-blocking M5 interchange family has a failing required conformance dimension; every named import/export family is conformant in its declared conformance class.

## Family conformance

| Family | Direction | Class | Version | Label | State | Decision | Consumers agree |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `request_api_collections` | round_trip | `round_trip_write_back` | v1 | beta | conformant | clear | yes |
| `notebook_parity_exports` | import_validation | `compare_only` | v1 | beta | conformant | clear | yes |
| `docs_packets` | import_validation | `import_validation_only` | v1 | beta | conformant | clear | yes |
| `trace_profile_replay_exports` | round_trip | `round_trip_write_back` | v1 | beta | conformant | clear | yes |
| `support_bundles` | import_validation | `import_validation_only` | v1 | stable | conformant | clear | yes |
| `portable_state_packages` | round_trip | `round_trip_write_back` | v1 | beta | conformant | clear | yes |

## Conformance dimensions

Each family is scored on one cell per dimension. Every cell is required; a failed required cell on a release-blocking family holds promotion, while a downgraded cell narrows the family without inheriting an adjacent family's claim.

| Dimension | What it proves |
| --- | --- |
| `emitted_artifact_present` | Real emitted artifact present |
| `import_export_validator` | Import/export validator wired |
| `round_trip_or_compare` | Round-trip or declared compare-only behavior proven |
| `provenance_preserved` | Required provenance preserved |
| `trust_not_widened` | Trust not silently widened |
| `cross_surface_agreement` | Cross-surface consumer agreement |
| `stable_reason_codes` | Stable, copy-safe failure reason codes |

## Stable import/export reason codes

An interchange failure reports one of these stable, copy-safe reason codes instead of a raw parser exception or a generic corruption message:

| Reason code | Copy-safe diagnostic |
| --- | --- |
| `unsupported_contract_version` | This artifact declares a contract version this build does not support. Export it again from a compatible build, or upgrade before importing. |
| `missing_required_provenance` | This artifact is missing required provenance (source surface, build identity, or record class). It cannot be imported without the provenance that proves where it came from. |
| `schema_validation_failed` | This artifact does not match the published contract schema for its family. No fields were imported; nothing was changed. |
| `trust_widening_blocked` | Importing this artifact would widen its trust (for example, promoting a managed or limited-trust record to a durable local one). The import was blocked; re-run it with an explicit trust decision. |
| `round_trip_mismatch` | Re-exporting this artifact after import did not reproduce it byte-for-byte. The import was blocked to avoid silent data loss. |
| `corrupt_or_truncated_payload` | This artifact is truncated or corrupt and could not be read. Re-export it; nothing was imported. |
| `unknown_field_unpreserved` | This artifact carries fields a round-trip would drop. The import was blocked so unknown fields are preserved rather than silently lost. |
| `redaction_class_conflict` | This artifact's redaction class conflicts with the destination's policy. The import was blocked; export it again at a compatible redaction class. |

## Counts

- Families: 6 (2 release-blocking, 2 catalog-linked)
- Conformance: 6 conformant, 0 narrowed, 0 failed
- Classes: 3 round-trip write-back, 3 compare-only / import-validation-only
- Dimensions: 42 evaluated (42 pass, 0 downgrade, 0 fail)

## How it stays honest

- A catalog-linked family's `lifecycle_label` equals the published contract family's label, so an interchange claim can never run ahead of the contract.
- A family the source docs scope to compare-only or inspect-only carries a `compare_only` or `import_validation_only` conformance class; write-back is not forced and the runner proves the scoped behavior instead.
- Import does not silently widen trust or strip required provenance, and a round-trip family preserves unknown fields; the negative fixtures prove each rejection path.
