# Refactor transaction model — preview / validate / rollback artifact

This artifact is the human-readable companion to the stable refactor
transaction truth packet. The boundary contract, vocabularies, and
refused-promotion rules live in
`docs/languages/m4/finalize-the-refactor-transaction-model-plus-preview-validate.md`;
this file pins the stable artifact references and the M4 launch-stable
posture for the claimed launch-language refactor classes.

## Stable references

- Boundary schema: `schemas/language/refactor_transaction_truth.schema.json`
- Stable packet artifact: `artifacts/language/m4/refactor_transaction_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/refactor_transaction_truth_packet/`
- Implementation contract: `crates/aureline-language/src/refactor_transaction_truth_packet/`
- Reviewer doc: `docs/languages/m4/finalize-the-refactor-transaction-model-plus-preview-validate.md`

## Claimed lanes (M4 launch-stable)

| Lane | Claim |
| --- | --- |
| `rename_symbol_lane` | Launch-stable across the rename loop. |
| `extract_function_lane` | Launch-stable across the extract-function loop. |
| `inline_symbol_lane` | Launch-stable across the inline-symbol loop. |
| `move_symbol_lane` | Launch-stable across the move-symbol loop. |
| `update_imports_lane` | Launch-stable across the update-imports loop. |
| `cross_file_signature_change_lane` | Launch-stable across the cross-file signature-change loop. |

Each lane carries:

- one `refactor_transaction_quality` row binding the headline support
  class;
- four `transaction_phase_truth` rows binding `preview`, `validate`,
  `apply`, and `rollback`;
- one `preview_outcome_admission` row binding a `preview_completeness_class`;
- one `validation_hook_admission` row binding a `validation_outcome_class`;
- one `rollback_drill_admission` row binding a `rollback_path_class`;
- one `launch_language_coverage` row binding the launch language under
  proof.

## Required consumer projections

Every required surface preserves the packet verbatim:

| Surface | What it shows |
| --- | --- |
| `editor_language_pack` | The lane refactor posture on the editor language pack. |
| `framework_pack_panel` | The lane refactor posture on the framework pack panel. |
| `language_settings` | The lane refactor posture in language settings/help. |
| `cli_headless` | The lane refactor posture from CLI/headless inspection. |
| `support_export` | The lane refactor posture in support exports. |
| `release_proof_index` | The lane refactor posture on the release proof index. |
| `help_about` | The lane refactor posture on the Help/About proof card. |
| `conformance_dashboard` | The lane refactor posture on the conformance dashboard. |

A surface that collapses any closed vocabulary is refused; the
validator emits `*_vocabulary_collapsed` and the packet blocks stable.

## Refusing promotion

The packet refuses to certify stable when any of the conditions
enumerated in the reviewer doc fire. The fixture corpus
(`fixtures/language/m4/refactor_transaction_truth_packet/`) ships a
baseline stable case plus five narrowed-below-stable cases proving
that the validator refuses each refused promotion class.

## Boundary safety

The packet is metadata-only. Every row sets
`raw_source_material_excluded`, `secrets_excluded`, and
`ambient_authority_excluded` to `true`. Raw source bodies, refactor
diffs, generated artifact bodies, secrets, and ambient credentials
never cross the boundary; any row that admits private material blocks
stable promotion.
