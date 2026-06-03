# Proof packet: Harden the critical dependency register, fork/replace log, third-party import manifest, and REUSE/SPDX/notice coverage

Artifact: `artifacts/release/harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage.json`

## Proof summary

| Lane | State | Effective label | Gap reason |
|---|---|---|---|
| Critical dependency: upstream Rust | current | stable | — |
| Critical dependency: platform/system | current | stable | — |
| Fork/replace: forked crates | current | stable | — |
| Fork/replace: replaced upstream | narrowed_stale | beta | packet_freshness_breached |
| Third-party import: direct | current | stable | — |
| Third-party import: transitive | on_waiver | stable | — |
| REUSE/SPDX/notice: SPDX SBOM | narrowed_unbacked | beta | license_coverage_gap |
| REUSE/SPDX/notice: human-readable notices | current | stable | — |

## Packet freshness

- `packet:critical_dependency_upstream_rust` — captured 2026-05-28, current
- `packet:critical_dependency_platform_system` — captured 2026-05-28, current
- `packet:fork_replace_forked_crates` — captured 2026-05-28, current
- `packet:fork_replace_replaced_upstream` — captured 2026-05-01, breached (SLO target 30 days)
- `packet:third_party_import_direct` — captured 2026-05-28, current
- `packet:third_party_import_transitive` — captured 2026-05-28, current
- `packet:reuse_spdx_notice_spdx_sbom` — not captured, missing
- `packet:reuse_spdx_notice_human_readable` — captured 2026-05-28, current

## Evidence refs

- `artifacts/release/m3/critical_upstream_register.csv`
- `artifacts/release/platform_dependency_manifest.yaml`
- `artifacts/release/fork_replace_log.json`
- `artifacts/release/third_party_import_manifest.json`
- `artifacts/release/cargo.lock`
- `docs/release/third_party_notices.md`
- `schemas/governance/dependency_audit.schema.json`
- `schemas/governance/platform_dependency_audit.schema.json`
- `schemas/governance/fork_replace_log.schema.json`
- `schemas/governance/third_party_import_manifest.schema.json`
- `schemas/governance/notice_coverage.schema.json`

## Owner sign-off

- `team:release_engineering` — signed off 2026-05-28 for all current lanes
- Unsigned for narrowed lanes pending remediation

## Waiver

- `waiver:transitive_import_audit` — expires 2026-06-30, covering third-party import transitive audit gap

## Publication verdict

**hold** — fork/replace replaced upstream (packet freshness breached) and REUSE/SPDX/notice SPDX SBOM (license coverage gap) block promotion.
