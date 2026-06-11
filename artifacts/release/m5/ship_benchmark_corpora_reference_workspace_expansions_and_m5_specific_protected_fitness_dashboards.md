# M5 Fitness-Surface Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5 fitness-surface register.

- **Canonical JSON**: `artifacts/release/m5/ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards.json`
- **Schema**: `schemas/governance/ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards.schema.json`
- **Typed consumer**: `crates/aureline-release/src/ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards/mod.rs`
- **Validation capture**: `artifacts/release/captures/ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards_validation_capture.json`
- **Generator**: `gen_fitness_surface_register.py`

The register is the single source of truth for the M5 fitness surfaces (benchmark corpora, reference-workspace expansions, protected fitness dashboards, and the fitness functions that guard them), the per-lane fitness scorecard, the disclosed corpus provenance and trust tier of each, owner manifests, downgrade automation, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_fitness_surface_register.py` from the repository root after changing the fitness-surface lanes.
