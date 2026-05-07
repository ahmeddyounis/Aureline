# Dependency ingest + health checks

This directory provides the seed automation tooling for keeping the dependency
governance registers mechanically checkable and refreshable, without turning
the canonical registers into bot-owned copy tables.

Primary entry points:

- `tools/governance/dependency_ingest/check_dependency_health.py`
  - offline integrity checks used by `./ci/check_dependency_health.sh`
- `tools/governance/dependency_ingest/refresh_upstream_observations.py`
  - optional upstream metadata probe (networked) that emits an attributed JSON
    report for human review (does not rewrite registers)

The authoritative source rows remain:

- `artifacts/governance/dependency_register.yaml`
- `artifacts/governance/third_party_import_register.yaml`
- `artifacts/governance/upstream_health_scorecard.yaml`
- `artifacts/governance/release_notice_seed.yaml`

