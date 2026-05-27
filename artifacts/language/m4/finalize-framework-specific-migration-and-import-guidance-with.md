# Framework migration and import guidance — launch bundle artifact

This artifact is the human-readable companion to the stable framework
migration and import guidance truth packet. The boundary contract,
vocabularies, and refused-promotion rules live in
`docs/languages/m4/finalize-framework-specific-migration-and-import-guidance-with.md`;
this file pins the stable artifact references and the M4
launch-stable posture for the framework migration guidance, import
guidance, and unsupported-gap labeling lanes across the launch
bundles.

## Stable references

- Boundary schema: `schemas/language/framework_migration_import_truth.schema.json`
- Stable packet artifact: `artifacts/language/m4/framework_migration_import_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/framework_migration_import_truth_packet/`
- Implementation contract: `crates/aureline-language/src/framework_migration_import_truth_packet/`
- Reviewer doc: `docs/languages/m4/finalize-framework-specific-migration-and-import-guidance-with.md`

## Claimed lanes (M4 launch-stable)

| Lane | Claim |
| --- | --- |
| `framework_migration_guidance_lane` | Launch-stable for framework-specific migration guidance across the launch bundles. |
| `import_guidance_lane` | Launch-stable for import-statement / module-resolution migration guidance across the launch bundles. |
| `unsupported_gap_labeling_lane` | Launch-stable for precise unsupported-gap labeling across the launch bundles. |

Each lane carries:

- one `migration_guidance_quality` row binding the headline support
  class;
- five `outcome_label_truth` rows binding `exact_match`,
  `translated_match`, `partial_match`, `shimmed_match`, and
  `unsupported_gap`;
- one `rollback_checkpoint_admission` row binding
  `checkpoint_preserved`;
- one `diagnostic_preservation_admission` row binding
  `diagnostics_preserved`;
- one `launch_bundle_coverage` row binding the launch bundle under
  proof.

## Required consumer projections

Every required surface preserves the packet verbatim:

| Surface | What it shows |
| --- | --- |
| `editor_language_pack` | The migration-guidance posture on the editor language pack. |
| `framework_pack_panel` | The migration-guidance posture on the framework pack panel. |
| `language_settings` | The migration-guidance posture in language settings/help. |
| `cli_headless` | The migration-guidance posture from CLI/headless inspection. |
| `support_export` | The migration-guidance posture in support exports. |
| `release_proof_index` | The migration-guidance posture on the release proof index. |
| `help_about` | The migration-guidance posture on the Help/About proof card. |
| `conformance_dashboard` | The migration-guidance posture on the conformance dashboard. |

A surface that collapses any closed vocabulary is refused; the
validator emits `*_vocabulary_collapsed` and the packet blocks stable.

## Refusing promotion

The packet refuses to certify stable when any of the conditions
enumerated in the reviewer doc fire. The fixture corpus
(`fixtures/language/m4/framework_migration_import_truth_packet/`)
ships a baseline stable case plus five narrowed-below-stable cases
proving that the validator refuses each refused promotion class.

## Boundary safety

The packet is metadata-only. Every row sets
`raw_source_material_excluded`, `secrets_excluded`, and
`ambient_authority_excluded` to `true`. Raw imported artifact
bodies, source bodies, dependency manifests, secrets, and ambient
credentials never cross the boundary; any row that admits private
material blocks stable promotion.
