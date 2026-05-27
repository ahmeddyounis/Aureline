# Framework migration and import guidance with unsupported-gap labeling for launch bundles

This document is the reviewer contract for the stable framework
migration and import guidance truth packet certifying the framework
migration guidance, import guidance, and unsupported-gap labeling
lanes across the launch bundles. The packet ships exact, translated,
partial, shimmed, and unsupported outcome labels generated from real
imported artifacts and preserves rollback checkpoints and diagnostics
when mapping fails, so the editor language pack, framework pack panel,
language settings/help, CLI/headless inspector, support export, release
proof index, Help/About proof card, and the conformance dashboard all
read one record.

- Boundary schema: `schemas/language/framework_migration_import_truth.schema.json`
- Stable artifact: `artifacts/language/m4/framework_migration_import_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/framework_migration_import_truth_packet/`
- Implementation: `crates/aureline-language/src/framework_migration_import_truth_packet/`

## What the packet pins

Every row binds a closed `migration_lane_class`,
`framework_migration_row_class`, `support_class`,
`outcome_label_class`, `rollback_checkpoint_class`,
`diagnostic_preservation_class`, `launch_bundle_class`,
`evidence_class`, `known_limit_class`,
`downgrade_automation_class`, and
`framework_migration_confidence_class` plus an `evidence_refs` array
and a `disclosure_ref` whenever the row is narrowed below
launch-stable, declares a non-`none_declared` known limit, or binds
a non-`none` downgrade automation.

### Migration lanes (`migration_lane_class`)

| Lane | Meaning |
| --- | --- |
| `framework_migration_guidance_lane` | Framework upgrade or replacement guidance for a launch bundle. |
| `import_guidance_lane` | Import-statement / module-resolution migration for a launch bundle. |
| `unsupported_gap_labeling_lane` | Precisely labeling features without a stable migration mapping. |

Every required lane MUST appear in the packet `covered_lanes` and MUST
carry at least one row.

### Row classes (`framework_migration_row_class`)

| Row class | Meaning |
| --- | --- |
| `migration_guidance_quality` | Lane headline qualification row. |
| `outcome_label_truth` | Binds exactly one outcome label (`exact_match`, `translated_match`, `partial_match`, `shimmed_match`, `unsupported_gap`). |
| `rollback_checkpoint_admission` | Binds a `rollback_checkpoint_class`. |
| `diagnostic_preservation_admission` | Binds a `diagnostic_preservation_class`. |
| `launch_bundle_coverage` | Binds a `launch_bundle_class` touchpoint. |
| `unsupported_gap` | Precisely labeled unsupported gap on a lane. |
| `known_limit` | Disclosed known-limit row attached to a lane. |
| `downgrade_automation` | Downgrade-automation rule row attached to a lane. |

### Outcome-label coverage (`outcome_label_class`)

A lane that claims `launch_stable` MUST carry an `outcome_label_truth`
row for each of the five outcome labels the spec demands from real
imported artifacts:

1. `exact_match` — the imported artifact maps to a stable target
   without translation.
2. `translated_match` — the imported artifact maps to a stable target
   via translation.
3. `partial_match` — only part of the imported artifact maps to a
   stable target.
4. `shimmed_match` — the imported artifact maps via a runtime shim or
   compatibility layer.
5. `unsupported_gap` — the imported artifact has no stable mapping;
   the gap is precisely labeled.

A missing outcome-label row narrows the lane below stable; the
validator emits `missing_outcome_label_coverage`. Implied
replacement-grade behavior never substitutes for an explicit
`unsupported_gap` row.

### Rollback-checkpoint and diagnostic-preservation admission

A lane that claims `launch_stable` MUST preserve rollback checkpoints
and diagnostics when mapping fails. Concretely:

- The lane MUST carry a `rollback_checkpoint_admission` row binding
  `checkpoint_preserved`. Lanes MAY admit additional checkpoint states
  (`checkpoint_with_diagnostics`, `checkpoint_pending`) to cover wider
  truth.
- The lane MUST carry a `diagnostic_preservation_admission` row
  binding `diagnostics_preserved`. Lanes MAY admit `diagnostics_partial`
  or `diagnostics_absent` to disclose narrower coverage with the
  matching downgrade automation and disclosure refs.

A missing required state narrows the lane below stable; the validator
emits `missing_rollback_checkpoint_coverage` or
`missing_diagnostic_preservation_coverage`.

### Launch-bundle coverage

A lane that claims `launch_stable` MUST also carry at least one
`launch_bundle_coverage` row binding a `launch_bundle_class`
(`python_launch_bundle`, `typescript_javascript_launch_bundle`,
`rust_launch_bundle`, `go_launch_bundle`, `java_kotlin_launch_bundle`,
`c_cpp_launch_bundle`). The launch bundle is the boundary truth that
binds the migration claim to a concrete launch wedge.

### Closed support and confidence vocabularies

`support_class` is closed to:
`launch_stable | launch_stable_below | beta_grade_only | preview_only | unsupported | support_unbound`.

`framework_migration_confidence_class` is closed to:
`high_confidence | medium_confidence | low_confidence`.

A row that claims `launch_stable` at `low_confidence` is narrowed
below stable until evidence grows.

### Closed evidence, known-limit, downgrade-automation vocabularies

See `schemas/language/framework_migration_import_truth.schema.json` for
the authoritative enums. Every row binds exactly one value from each
vocabulary. `evidence_unbound`, `limit_unbound`, `automation_unbound`,
`label_unbound`, `checkpoint_unbound`, `diagnostic_unbound`, and
`support_unbound` never qualify stable.

### Disclosure refs

A row MUST surface a `disclosure_ref` whenever it:

- claims a support class below `launch_stable`,
- declares a known limit other than `none_declared`, or
- binds a downgrade automation other than `none`.

A missing disclosure ref is a blocker.

### Boundary safety

Every row MUST set `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded` to `true`. The packet is
metadata-only — it never admits raw imported artifact bodies, source
bodies, dependency manifests, secrets, or ambient credentials past
the boundary.

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
  downgrade-automation, evidence, outcome-label, rollback-checkpoint,
  or diagnostic-preservation class is unbound;
- a lane claiming `launch_stable` is missing any of the five required
  outcome labels (`exact_match`, `translated_match`, `partial_match`,
  `shimmed_match`, `unsupported_gap`), the required
  `rollback_checkpoint_admission` row binding `checkpoint_preserved`,
  the required `diagnostic_preservation_admission` row binding
  `diagnostics_preserved`, or a `launch_bundle_coverage` row;
- a row narrowed below `launch_stable`, declaring a
  non-`none_declared` known limit, or binding a non-`none` downgrade
  automation drops its `disclosure_ref`;
- a binding-typed row (outcome label, rollback checkpoint, diagnostic
  preservation, launch bundle) drops its binding, or a non-binding
  row binds one;
- raw imported artifact bodies, source bodies, dependency manifests,
  secrets, or ambient credentials slip past the boundary;
- any required consumer projection is missing or collapses the lane,
  row-class, support-class, outcome-label, rollback-checkpoint,
  diagnostic-preservation, launch-bundle, known-limit,
  downgrade-automation, or evidence-class vocabulary; or
- stored `promotion_state` disagrees with the derived findings.

## Fixture corpus

`fixtures/language/m4/framework_migration_import_truth_packet/` ships:

| Fixture | What it proves |
| --- | --- |
| `baseline_stable.json` | Baseline stable posture across all three migration lanes. |
| `launch_stable_with_unbound_evidence_blocks_stable.json` | A row claiming `launch_stable` with unbound evidence is refused. |
| `missing_outcome_label_for_launch_stable_blocks_stable.json` | A lane claiming `launch_stable` missing a required outcome label is refused. |
| `narrowed_row_missing_disclosure_ref_blocks_stable.json` | A row narrowed below `launch_stable` without a disclosure ref is refused. |
| `projection_collapses_outcome_label_vocabulary_blocks_stable.json` | A consumer projection collapsing the outcome-label vocabulary is refused. |
| `raw_source_material_blocks_stable.json` | A row admitting raw source bodies past the boundary is refused. |

Each fixture pins `record_kind`, `case_name`, `scenario`, the full
input packet, and the expected promotion state, finding count, row
count, and token sets the materialized packet must produce.
