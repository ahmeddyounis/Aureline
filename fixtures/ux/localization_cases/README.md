# Localization, locale-pack, translation-governance, and source-language fallback fixtures

These fixtures exercise the contract frozen in
[`/docs/ux/localization_and_locale_pack_contract.md`](../../../docs/ux/localization_and_locale_pack_contract.md)
against the three boundary schemas:

- [`/schemas/ux/message_catalog_entry.schema.json`](../../../schemas/ux/message_catalog_entry.schema.json)
- [`/schemas/ux/locale_pack_manifest.schema.json`](../../../schemas/ux/locale_pack_manifest.schema.json)
- [`/schemas/ux/locale_fallback_state.schema.json`](../../../schemas/ux/locale_fallback_state.schema.json)

Each YAML file is a single record; a `# yaml-language-server: $schema=...`
header pins the editor to the correct boundary schema.

| Fixture                                                      | Record kind                       | Why it's here                                                                                                                                  |
|--------------------------------------------------------------|-----------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| `partially_translated_docs_pack.yaml`                        | `locale_fallback_state_record`    | Acceptance: partially translated docs pack; pt-BR with en-US fill, fallback disclosed.                                                         |
| `extension_locale_overlay_for_extension_command_label.yaml`  | `message_catalog_entry_record`    | Acceptance: extension locale overlay; namespaced overlay pinned to a host-resolved canonical command id.                                       |
| `locale_pack_signature_failed.yaml`                          | `locale_fallback_state_record`    | Acceptance: locale-pack signature failure; ja-JP signature failure forces source-language-only with the typed denial reason.                   |
| `source_language_fallback_for_recovery_banner.yaml`          | `locale_fallback_state_record`    | Acceptance: source-language fallback; safety-critical recovery banner falls through fr-CA → en-US with command identity preserved.             |
| `translated_cli_help_with_locale_neutral_json.yaml`          | `message_catalog_entry_record`    | Acceptance: translated CLI help with locale-neutral JSON; flag and path placeholders preserved, `cli_locale_neutral_output_flag` mandatory.    |
| `command_label_built_in_canonical.yaml`                      | `message_catalog_entry_record`    | Supporting: built-in canonical command label, frozen-after-review, glossary-anchored.                                                          |
| `locale_pack_manifest_mirrored_signed.yaml`                  | `locale_pack_manifest_record`     | Supporting: mirrored, signed pt-BR pack manifest declaring partial coverage on glossary terms.                                                 |

The fixtures share an `__fixture__` block that names the scenario,
exercised axes, and contract sections; tooling that lints fixture
metadata reads those keys uniformly with the other fixture corpora
under `fixtures/ux/`.
