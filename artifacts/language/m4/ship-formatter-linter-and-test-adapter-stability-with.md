# Adapter stability — formatter / linter / test-adapter artifact

This artifact is the human-readable companion to the stable adapter
stability truth packet. The boundary contract, vocabularies, and
refused-promotion rules live in
`docs/languages/m4/ship-formatter-linter-and-test-adapter-stability-with.md`;
this file pins the stable artifact references and the M4
launch-stable posture for the formatter, linter, and test-adapter
lanes across the launch wedges.

## Stable references

- Boundary schema: `schemas/language/adapter_stability_truth.schema.json`
- Stable packet artifact: `artifacts/language/m4/adapter_stability_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/adapter_stability_truth_packet/`
- Implementation contract: `crates/aureline-language/src/adapter_stability_truth_packet/`
- Reviewer doc: `docs/languages/m4/ship-formatter-linter-and-test-adapter-stability-with.md`

## Claimed lanes (M4 launch-stable)

| Lane | Claim |
| --- | --- |
| `formatter_lane` | Launch-stable across the formatter discover/execute/report loop. |
| `linter_lane` | Launch-stable across the linter discover/execute/report loop. |
| `test_adapter_lane` | Launch-stable across the test-adapter discover/execute/report loop. |

Each lane carries:

- one `adapter_stability_quality` row binding the headline support
  class;
- three `adapter_capability_truth` rows binding `discover`, `execute`,
  and `report`;
- three `degraded_provider_admission` rows binding `provider_healthy`,
  `provider_degraded_warned`, and `provider_unavailable`;
- one `adapter_outcome_admission` row binding an
  `adapter_outcome_class`;
- one `launch_wedge_coverage` row binding the launch wedge under
  proof.

## Required consumer projections

Every required surface preserves the packet verbatim:

| Surface | What it shows |
| --- | --- |
| `editor_language_pack` | The lane adapter posture on the editor language pack. |
| `framework_pack_panel` | The lane adapter posture on the framework pack panel. |
| `language_settings` | The lane adapter posture in language settings/help. |
| `cli_headless` | The lane adapter posture from CLI/headless inspection. |
| `support_export` | The lane adapter posture in support exports. |
| `release_proof_index` | The lane adapter posture on the release proof index. |
| `help_about` | The lane adapter posture on the Help/About proof card. |
| `conformance_dashboard` | The lane adapter posture on the conformance dashboard. |

A surface that collapses any closed vocabulary is refused; the
validator emits `*_vocabulary_collapsed` and the packet blocks stable.

## Refusing promotion

The packet refuses to certify stable when any of the conditions
enumerated in the reviewer doc fire. The fixture corpus
(`fixtures/language/m4/adapter_stability_truth_packet/`) ships a
baseline stable case plus six narrowed-below-stable cases proving
that the validator refuses each refused promotion class.

## Boundary safety

The packet is metadata-only. Every row sets
`raw_source_material_excluded`, `secrets_excluded`, and
`ambient_authority_excluded` to `true`. Raw formatter output, linter
output, test logs, source bodies, secrets, and ambient credentials
never cross the boundary; any row that admits private material blocks
stable promotion.
