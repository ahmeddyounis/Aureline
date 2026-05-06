# Quality-scenario case stubs

This directory contains **worked case stubs** for the quality-attribute scenario library published in:

- `artifacts/architecture/quality_scenario_rows.yaml`
- `artifacts/qe/quality_scenario_to_lane_map.yaml`

The case files are intentionally lightweight: they do not implement the benchmark or QE lanes. They exist
so future lane tooling can:

1) resolve a `qas:*` scenario id to the current fixture/corpus anchors,
2) produce deterministic “what evidence backs this scenario?” joins, and
3) keep scenario IDs stable even as underlying fixtures evolve.

Every file in this directory MUST:

- declare `schema_version: 1`,
- cite exactly one `scenario_id: qas:*`,
- cite the canonical scenario row register and lane map by path, and
- avoid planning identifiers in ids and titles.

