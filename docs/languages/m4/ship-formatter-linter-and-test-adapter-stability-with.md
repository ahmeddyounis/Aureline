# Adapter stability: formatter, linter, and test-adapter degraded-provider boundary truth

This document is the reviewer contract for the stable adapter
stability truth packet certifying the formatter, linter, and
test-adapter lanes across the launch wedges. The packet ships
formatter / linter / test-adapter stability with degraded-provider
truth so the editor language pack, framework pack panel, language
settings/help, CLI/headless inspector, support export, release proof
index, Help/About proof card, and the conformance dashboard all read
one record.

- Boundary schema: `schemas/language/adapter_stability_truth.schema.json`
- Stable artifact: `artifacts/language/m4/adapter_stability_truth_packet.json`
- Fixture corpus: `fixtures/language/m4/adapter_stability_truth_packet/`
- Implementation: `crates/aureline-language/src/adapter_stability_truth_packet/`

## What the packet pins

Every row binds a closed `adapter_lane_class`,
`adapter_stability_row_class`, `support_class`,
`adapter_capability_class`, `degraded_provider_class`,
`adapter_outcome_class`, `launch_wedge_class`, `evidence_class`,
`known_limit_class`, `downgrade_automation_class`, and
`adapter_stability_confidence_class` plus an `evidence_refs` array
and a `disclosure_ref` whenever the row is narrowed below
launch-stable, declares a non-`none_declared` known limit, or binds
a non-`none` downgrade automation.

### Adapter lanes (`adapter_lane_class`)

| Lane | Meaning |
| --- | --- |
| `formatter_lane` | Formatter adapter (format-on-save, format-on-demand, format-diff). |
| `linter_lane` | Linter adapter (lint-on-save, lint-on-demand, lint quick-fix). |
| `test_adapter_lane` | Test adapter (discover, run, filter, report). |

Every required lane MUST appear in the packet `covered_lanes` and MUST
carry at least one row.

### Row classes (`adapter_stability_row_class`)

| Row class | Meaning |
| --- | --- |
| `adapter_stability_quality` | Lane headline qualification row. |
| `adapter_capability_truth` | Binds exactly one adapter capability (discover, execute, report). |
| `degraded_provider_admission` | Binds a `degraded_provider_class`. |
| `adapter_outcome_admission` | Binds an `adapter_outcome_class`. |
| `launch_wedge_coverage` | Binds a `launch_wedge_class` touchpoint. |
| `unsupported_gap` | Precisely labeled unsupported gap on a lane. |
| `known_limit` | Disclosed known-limit row attached to a lane. |
| `downgrade_automation` | Downgrade-automation rule row attached to a lane. |

### Adapter-capability coverage (`adapter_capability_class`)

A lane that claims `launch_stable` MUST carry an
`adapter_capability_truth` row for each of:

1. `discover` — locating the formatter binary, lint rules, or test
   targets on the launch wedge.
2. `execute` — running the adapter against the workspace.
3. `report` — surfacing diff, diagnostics, or test results to consumer
   surfaces.

A missing capability row narrows the lane below stable; the validator
emits `missing_adapter_capability_coverage`.

### Degraded-provider admission

A lane that claims `launch_stable` MUST carry a
`degraded_provider_admission` row for each of:

- `provider_healthy`
- `provider_degraded_warned`
- `provider_unavailable`

`provider_misconfigured` and `provider_timed_out` are additional closed
states that lanes MAY admit to cover further degraded-provider truth.
Surfaces project these rows verbatim; they do not paraphrase provider
state locally. A missing required degraded-provider state row narrows
the lane below stable; the validator emits
`missing_degraded_provider_coverage`.

### Adapter outcome and launch-wedge coverage

A lane that claims `launch_stable` MUST also carry one
`adapter_outcome_admission` row binding an `adapter_outcome_class` and
at least one `launch_wedge_coverage` row binding a `launch_wedge_class`
(`python_wedge`, `typescript_javascript_wedge`, `rust_wedge`,
`go_wedge`, `java_kotlin_wedge`, `c_cpp_wedge`).

### Closed support and confidence vocabularies

`support_class` is closed to:
`launch_stable | launch_stable_below | beta_grade_only | preview_only | unsupported | support_unbound`.

`adapter_stability_confidence_class` is closed to:
`high_confidence | medium_confidence | low_confidence`.

A row that claims `launch_stable` at `low_confidence` is narrowed
below stable until evidence grows.

### Closed evidence, known-limit, downgrade-automation vocabularies

See `schemas/language/adapter_stability_truth.schema.json` for the
authoritative enums. Every row binds exactly one value from each
vocabulary. `evidence_unbound`, `limit_unbound`, `automation_unbound`,
`state_unbound`, `outcome_unbound`, and `support_unbound` never
qualify stable.

### Disclosure refs

A row MUST surface a `disclosure_ref` whenever it:

- claims a support class below `launch_stable`,
- declares a known limit other than `none_declared`, or
- binds a downgrade automation other than `none`.

A missing disclosure ref is a blocker.

### Boundary safety

Every row MUST set `raw_source_material_excluded`, `secrets_excluded`,
and `ambient_authority_excluded` to `true`. The packet is
metadata-only — it never admits raw formatter or linter output, raw
test logs, source bodies, secrets, or ambient credentials past the
boundary.

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
  downgrade-automation, evidence, degraded-provider, or
  adapter-outcome class is unbound;
- a lane claiming `launch_stable` is missing any of the three required
  adapter capabilities (`discover`, `execute`, `report`), any of the
  three required degraded-provider states (`provider_healthy`,
  `provider_degraded_warned`, `provider_unavailable`), an
  `adapter_outcome_admission` row, or a `launch_wedge_coverage` row;
- a row narrowed below `launch_stable`, declaring a
  non-`none_declared` known limit, or binding a non-`none` downgrade
  automation drops its `disclosure_ref`;
- a binding-typed row (adapter capability, degraded provider, adapter
  outcome, launch wedge) drops its binding, or a non-binding row
  binds one;
- raw formatter output, linter output, test logs, source bodies,
  secrets, or ambient credentials slip past the boundary;
- any required consumer projection is missing or collapses the lane,
  row-class, support-class, adapter-capability, degraded-provider,
  adapter-outcome, launch-wedge, known-limit, downgrade-automation, or
  evidence-class vocabulary; or
- stored `promotion_state` disagrees with the derived findings.

## Fixture corpus

`fixtures/language/m4/adapter_stability_truth_packet/` ships:

| Fixture | What it proves |
| --- | --- |
| `baseline_stable.json` | Baseline stable posture across all three adapter lanes. |
| `launch_stable_with_unbound_evidence_blocks_stable.json` | A row claiming `launch_stable` with unbound evidence is refused. |
| `missing_capability_for_launch_stable_blocks_stable.json` | A lane claiming `launch_stable` missing a required adapter capability is refused. |
| `missing_degraded_provider_state_blocks_stable.json` | A lane claiming `launch_stable` missing a required degraded-provider state is refused. |
| `narrowed_row_missing_disclosure_ref_blocks_stable.json` | A row narrowed below `launch_stable` without a disclosure ref is refused. |
| `projection_collapses_degraded_provider_vocabulary_blocks_stable.json` | A consumer projection collapsing the degraded-provider vocabulary is refused. |
| `raw_source_material_blocks_stable.json` | A row admitting raw source bodies past the boundary is refused. |

Each fixture pins `record_kind`, `case_name`, `scenario`, the full
input packet, and the expected promotion state, finding count, row
count, and token sets the materialized packet must produce.
