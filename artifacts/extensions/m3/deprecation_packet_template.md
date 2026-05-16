# Extension SDK/Public-Interface Deprecation Packet

Use this packet for any beta, stable, or LTS SDK/public-interface row
that enters `deprecated` or `retired` lifecycle state. The source row
must also be present in
`artifacts/extensions/m3/lifecycle_metadata_packet.json`.

## Packet Metadata

| Field | Value |
|---|---|
| Deprecation packet id | `deprecation_packet:<surface>:<version>` |
| Lifecycle row ref | `lifecycle_row:<surface>` |
| Affected surface ref | `sdk_v1_api_surface:*`, `manifest_schema:*`, `manifest_field:*`, `wit_world:*`, `permission_vocabulary:*`, `publication_pipeline:*`, or `bridge_profile:*` |
| Surface owner | `@owner` |
| Stability label | `deprecated` or `retired` |
| Deprecated since | `<version>` |
| Last writable version | `<version or channel>` |
| Removal target | `<version>` or `<YYYY-MM-DD>` |

## Replacement

| Field | Value |
|---|---|
| Replacement surface ref | `<surface ref>` |
| No direct replacement reason | `<only when no replacement exists>` |
| Migration guide ref | `<docs path or packet ref>` |
| Alias or downgrade behavior | `<reader/writer/alias/rollback behavior>` |

## Impact

| Audience | Required note |
|---|---|
| Extension authors | What code, manifest, WIT, permission, or publication metadata changes. |
| Users/admins | Whether installed extensions keep working, degrade, require consent, or stop activating. |
| Registry/mirrors | Whether ingest, promotion, mirror import, or rollback behavior changes. |
| Support/export | Which support export fields, compatibility rows, and deprecation notices must appear. |

## Verification

| Check | Evidence |
|---|---|
| Lifecycle packet row updated | `artifacts/extensions/m3/lifecycle_metadata_packet.json#lifecycle_row:<surface>` |
| Schema or fixture updated | `<schema ref>` / `<fixture ref>` |
| Compatibility report updated | `artifacts/compat/m3/extension_compatibility_report.md` |
| Publication tooling still gates missing metadata | `python3 tools/extensions/m3/publish_extension.py validate-fixtures ...` |
| Lifecycle packet validation passes | `python3 tools/extensions/m3/validator_cli/aureline_extension_validator.py validate-lifecycle-packet ...` |

## Release Text

State the old surface, the replacement or no-direct-replacement posture,
the support overlap, and the removal target. Do not mention planning
identifiers or internal task names.
