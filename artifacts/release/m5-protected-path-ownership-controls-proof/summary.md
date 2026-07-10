# Protected-path rows and ownership cards

- Packet: `m5-protected-path-ownership-controls:stable:0001`
- Surface: `M5 protected-path rows and ownership cards: protection reason, owner source, advisory-versus-authoritative enforcement, backup coverage, and escalation continuity across claimed governed surfaces`
- Protected-path rows: 6 (3 advisory or local estimate)
- Ownership cards: 6 (3 not clean coverage)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Protected-path rows

- **.github/workflows/release.yml** — reason `Release workflow is provider branch-protected: changes require an owner approval`, enforcement `provider_branch_protection` → `provider_authoritative`, freshness `currently_evaluated`, rule source `branch_protection_rule`
- **crates/aureline-crypto/**** — reason `Cryptography core is CODEOWNERS-guarded: a security owner review is required`, enforcement `provider_resolved_codeowners` → `provider_authoritative`, freshness `imported`, rule source `codeowners_rule`
- **schemas/public/**** — reason `Public schema surface is locally protected: a public-surface diff review is required`, enforcement `local_manifest_enforced` → `locally_authoritative`, freshness `currently_evaluated`, rule source `manifest_entry`
- **docs/public/**** — reason `Public docs are advisory-guarded: an owner review is suggested but not enforced`, enforcement `local_manifest_advisory` → `advisory_only`, freshness `stale`, rule source `protected_path_policy`
- **config/*.toml** — reason `Config files matched a heuristic protection pattern: treat as an estimate`, enforcement `local_heuristic_match` → `local_estimate`, freshness `never_evaluated`, rule source `no_rule_source`
- **services/billing/**** — reason `Billing service owner inferred from recent authorship: not a recorded rule`, enforcement `inferred_from_authorship` → `local_estimate`, freshness `unknown`, rule source `no_rule_source`

## Ownership cards

- **crates/aureline-crypto/**** — owner `security-team`, source `codeowners_entry`, enforcement `provider_authoritative`, coverage `covered_with_backup`, continuity `continuous`
- **schemas/public/**** — owner `api-platform-team`, source `ownership_manifest`, enforcement `locally_authoritative`, coverage `covered_with_backup`, continuity `continuous`
- **services/billing/**** — owner `billing-team`, source `dri_registry`, enforcement `advisory_only`, coverage `backup_missing`, continuity `degraded_backup_missing`
- **tools/legacy/**** — owner `unassigned-role`, source `unresolved`, enforcement `local_estimate`, coverage `unresolved`, continuity `unresolved_continuity`
- **services/restricted/**** — owner `policy-restricted-role`, source `inferred_authorship`, enforcement `local_estimate`, coverage `policy_hidden`, continuity `policy_limited`
- **.github/workflows/release.yml** — owner `release-eng-team`, source `codeowners_entry`, enforcement `provider_authoritative`, coverage `covered_with_backup`, continuity `continuous`
