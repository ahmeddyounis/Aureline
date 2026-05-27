# Refactor transaction model: preview, validate, apply, and rollback boundary truth

This document is the reviewer contract for the stable refactor
transaction truth packet certifying the rename, extract-function,
inline-symbol, move-symbol, update-imports, and cross-file-signature-change
launch-language refactor lanes. The packet finalizes the refactor
transaction model and the preview / validate / rollback corpus so the
editor language pack, framework pack panel, language settings/help,
CLI/headless inspector, support export, release proof index, Help/About
proof card, and the conformance dashboard all read one record.

- Boundary schema: `schemas/language/refactor_transaction_truth.schema.json`
- Stable artifact: `artifacts/language/m4/refactor_transaction_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/refactor_transaction_truth_packet/`
- Implementation: `crates/aureline-language/src/refactor_transaction_truth_packet/`

## What the packet pins

Every row binds a closed `refactor_class_lane_class`,
`refactor_transaction_row_class`, `support_class`,
`transaction_phase_class`, `preview_completeness_class`,
`validation_outcome_class`, `rollback_path_class`,
`launch_language_class`, `evidence_class`, `known_limit_class`,
`downgrade_automation_class`, and `refactor_transaction_confidence_class`
plus an `evidence_refs` array and a `disclosure_ref` whenever the row
is narrowed below launch-stable, declares a non-`none_declared` known
limit, or binds a non-`none` downgrade automation.

### Refactor-class lanes (`refactor_class_lane_class`)

| Lane | Meaning |
| --- | --- |
| `rename_symbol_lane` | Symbol / identifier rename across files. |
| `extract_function_lane` | Extract function / method / helper. |
| `inline_symbol_lane` | Inline a symbol back into its call sites. |
| `move_symbol_lane` | Move a symbol, file, module, or export. |
| `update_imports_lane` | Rewrite import statements / export barrels. |
| `cross_file_signature_change_lane` | Change a callable signature and update cross-file call sites. |

Every required lane MUST appear in the packet `covered_lanes` and MUST
carry at least one row.

### Row classes (`refactor_transaction_row_class`)

| Row class | Meaning |
| --- | --- |
| `refactor_transaction_quality` | Lane headline qualification row. |
| `transaction_phase_truth` | Binds exactly one transaction phase (preview, validate, apply, rollback). |
| `preview_outcome_admission` | Binds a `preview_completeness_class`. |
| `validation_hook_admission` | Binds a `validation_outcome_class`. |
| `rollback_drill_admission` | Binds a `rollback_path_class`. |
| `launch_language_coverage` | Binds a `launch_language_class` touchpoint. |
| `unsupported_gap` | Precisely labeled unsupported gap on a lane. |
| `known_limit` | Disclosed known-limit row attached to a lane. |
| `downgrade_automation` | Downgrade-automation rule row attached to a lane. |

### Transaction-phase coverage (`transaction_phase_class`)

A lane that claims `launch_stable` MUST carry a `transaction_phase_truth`
row for each of:

1. `preview`
2. `validate`
3. `apply`
4. `rollback`

A missing phase row narrows the lane below stable; the validator emits
`missing_transaction_phase_coverage`.

### Preview / validate / rollback admission

A lane that claims `launch_stable` MUST also carry:

- one `preview_outcome_admission` row binding a
  `preview_completeness_class`,
- one `validation_hook_admission` row binding a
  `validation_outcome_class`, and
- one `rollback_drill_admission` row binding a `rollback_path_class`.

These three rows are the preview / validate / rollback corpus admissions
for the lane. Surfaces project these rows verbatim; they do not paraphrase
preview completeness, validation outcomes, or rollback paths locally.

### Closed support and confidence vocabularies

`support_class` is closed to:
`launch_stable | launch_stable_below | beta_grade_only | preview_only | unsupported | support_unbound`.

`refactor_transaction_confidence_class` is closed to:
`high_confidence | medium_confidence | low_confidence`.

A row that claims `launch_stable` at `low_confidence` is narrowed
below stable until evidence grows.

### Closed evidence, known-limit, downgrade-automation vocabularies

See `schemas/language/refactor_transaction_truth.schema.json` for the
authoritative enums. Every row binds exactly one value from each
vocabulary. `evidence_unbound`, `limit_unbound`, `automation_unbound`,
`preview_unbound`, `outcome_unbound`, `rollback_unbound`, and
`support_unbound` never qualify stable.

### Disclosure refs

A row MUST surface a `disclosure_ref` whenever it:

- claims a support class below `launch_stable`,
- declares a known limit other than `none_declared`, or
- binds a downgrade automation other than `none`.

A missing disclosure ref is a blocker.

### Boundary safety

Every row MUST set `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded` to `true`. The packet is metadata-only
— it never admits raw source bodies, refactor diffs, generated artifact
bodies, secrets, or ambient credentials past the boundary.

## Consumer projections

The packet certifies eight required projections, one per consumer
surface (`editor_language_pack`, `framework_pack_panel`,
`language_settings`, `cli_headless`, `support_export`,
`release_proof_index`, `help_about`, `conformance_dashboard`). Each
projection MUST preserve every closed vocabulary verbatim, point at
the packet `packet_id`, support JSON export, and exclude raw private
material and ambient authority. Any projection that collapses a
vocabulary is refused; the validator emits
`*_vocabulary_collapsed` and the packet blocks stable.

## Refused promotions

The validator refuses to certify a stable packet when:

- a row claims `launch_stable` while its support, known-limit,
  downgrade-automation, evidence, preview-completeness,
  validation-outcome, or rollback-path class is unbound;
- a lane claiming `launch_stable` is missing any of the four required
  transaction phases, the `preview_outcome_admission`,
  `validation_hook_admission`, or `rollback_drill_admission` rows;
- a row narrowed below `launch_stable`, declaring a
  non-`none_declared` known limit, or binding a non-`none` downgrade
  automation drops its `disclosure_ref`;
- a binding-typed row (transaction phase, preview outcome, validation
  hook, rollback drill, launch language) drops its binding, or a
  non-binding row binds one;
- raw source bodies, refactor diffs, secrets, or ambient credentials
  slip past the boundary;
- any required consumer projection is missing or collapses the lane,
  row-class, support-class, transaction-phase, preview-completeness,
  validation-outcome, rollback-path, launch-language, known-limit,
  downgrade-automation, or evidence-class vocabulary; or
- stored `promotion_state` disagrees with the derived findings.

## Fixture corpus

`fixtures/language/m4/refactor_transaction_truth_packet/` ships:

| Fixture | What it proves |
| --- | --- |
| `baseline_stable.json` | Baseline stable posture across all six lanes. |
| `launch_stable_with_unbound_evidence_blocks_stable.json` | A row claiming `launch_stable` with unbound evidence is refused. |
| `missing_transaction_phase_for_launch_stable_blocks_stable.json` | A lane claiming `launch_stable` missing a required transaction phase is refused. |
| `narrowed_row_missing_disclosure_ref_blocks_stable.json` | A row narrowed below `launch_stable` without a disclosure ref is refused. |
| `projection_collapses_rollback_path_vocabulary_blocks_stable.json` | A consumer projection collapsing the rollback-path vocabulary is refused. |
| `raw_source_material_blocks_stable.json` | A row admitting raw source bodies past the boundary is refused. |

Each fixture pins `record_kind`, `case_name`, `scenario`, the full
input packet, and the expected promotion state, finding count, row
count, and token sets the materialized packet must produce.
