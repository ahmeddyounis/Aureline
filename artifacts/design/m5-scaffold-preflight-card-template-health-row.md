# Scaffold preflight cards and template health rows

- Packet: `m5-scaffold-preflight-card-template-health-row-controls:stable:0001`
- Surface: `M5 scaffold preflight cards and template health rows: generated file counts, immediate-versus-deferred actions, blocked/warning/optional checks, and create-empty parity across claimed bootstrap surfaces`
- Scaffold preflight cards: 6 (1 blocked prerequisites)
- Template health rows: 6 (5 not current)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Scaffold preflight cards

- **Tooling present** — check `tooling_present`, result `passed` → `clear`, side effect `extension_install`, timing `deferred_for_later`, 24 files / 6 folders, deep link `template_manifest`
- **Dependency availability** — check `dependency_availability`, result `warning` → `advisory`, side effect `package_install`, timing `requires_confirmation`, 24 files / 6 folders, deep link `starter_registry_entry`
- **Network access** — check `network_access`, result `blocked` → `blocked_prerequisite`, side effect `remote_provisioning`, timing `blocked_until_resolved`, 15 files / 4 folders, deep link `policy_reference`
- **Workspace writable** — check `workspace_writable`, result `skipped_optional` → `optional_skipped`, side effect `script_execution`, timing `not_applicable`, 24 files / 6 folders, deep link `docs_anchor`
- **Host boundary** — check `host_boundary`, result `not_run` → `needs_attention`, side effect `dependency_restore`, timing `runs_immediately`, 24 files / 6 folders, deep link `docs_anchor`
- **Credential scope** — check `credential_scope`, result `unknown` → `needs_attention`, side effect `trust_prompt`, timing `requires_confirmation`, 15 files / 4 folders, deep link `policy_reference`

## Template health rows

- **Build health** — signal `build_health`, freshness `fresh` → `current`, severity `info`, fix `no_fix_needed`, deep link `docs_anchor`
- **Dependency freshness** — signal `dependency_freshness`, freshness `aging` → `aging`, severity `warning`, fix `auto_fix_available`, deep link `starter_registry_entry`
- **Security advisories** — signal `security_advisories`, freshness `stale` → `stale_or_expired`, severity `blocker`, fix `manual_fix_required`, deep link `docs_anchor`
- **Test status** — signal `test_status`, freshness `expired` → `stale_or_expired`, severity `warning`, fix `manual_fix_required`, deep link `docs_anchor`
- **Maintenance cadence** — signal `maintenance_cadence`, freshness `never_checked` → `never_checked`, severity `info`, fix `no_fix_needed`, deep link `docs_anchor`
- **Compatibility** — signal `compatibility`, freshness `unavailable` → `unavailable`, severity `blocker`, fix `auto_fix_available`, deep link `docs_anchor`
