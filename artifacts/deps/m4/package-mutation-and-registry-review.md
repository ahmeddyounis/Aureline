# Proof packet: Package mutation and registry review

Artifact: `artifacts/deps/m4/package-mutation-and-registry-review.json`

## Purpose

This packet is the canonical package-mutation and registry-review truth for the
stable dependency-manager lane. It keeps package search, manifest scope,
requested and resolved package identity, registry or mirror auth, lockfile
impact, script/native-build risk, validation packs, grouped updates, support
exports, and recovery rows on one vocabulary.

The packet currently claims stable rows for `cargo` and `node_pnpm` only. Other
ecosystems should stay inspect-only or handoff-only until resolver safety,
registry/mirror auth, and lockfile semantics are proven.

## Vocabulary

| Domain | Tokens |
|---|---|
| Ecosystem | `cargo`, `node_pnpm` |
| Operation | `add`, `update`, `remove`, `resolve` |
| Search state | `results_available`, `no_results`, `auth_required`, `mirror_stale`, `offline_snapshot_only`, `unknown_or_stale` |
| Source kind | `registry`, `workspace_local`, `path`, `vcs`, `policy_pinned`, `offline_snapshot_only`, `unknown_or_stale` |
| Dependency relation | `direct`, `transitive`, `workspace_local`, `path`, `vcs`, `policy_pinned`, `offline_snapshot_only`, `unknown_or_stale` |
| Registry source | `public_registry`, `private_registry`, `enterprise_mirror`, `local_cache`, `offline_snapshot` |
| Credential mode | `anonymous`, `os_store`, `token`, `browser_or_device_sign_in`, `policy_inherited` |
| Registry freshness | `current`, `stale`, `auth_required`, `offline_snapshot_only`, `unknown` |
| Registry reachability | `reachable`, `unreachable`, `auth_required`, `policy_blocked`, `offline` |
| Lockfile impact | `direct_bump`, `security_patch`, `grouped_refresh`, `lockfile_refresh_only`, `major_version_pilot`, `workspace_wide_convergence` |
| Script/native-build risk | `none_known`, `package_scripts`, `native_build`, `new_egress`, `policy_blocked`, `unknown` |
| Source actor | `manual`, `ai_suggested`, `automation_suggested` |
| Write posture | `inspect_only`, `review_required`, `apply_blocked`, `applied_after_review` |

## Surface contract

| Surface | Contract |
|---|---|
| Package browser/search | Shows ecosystem, current manifest scope, search state, requested package, available/resolved cue, registry source, and auth state. |
| Workspace scope bar | Shows root/member/module scope, active manifest path, lockfile coupling, registry inheritance, and change-scope action. |
| Package detail sheet | Separates direct, transitive, workspace-local, path, VCS, policy-pinned, offline-snapshot-only, and unknown/stale package state. |
| Operation review sheet | Previews affected manifests and lockfiles, requested vs resolved identity, peer/runtime shifts, registry/auth state, lockfile impact, script/native-build risk, validation pack, and rollback checkpoint. |
| Registry/mirror auth panel | Shows source class, credential mode, freshness, reachability, policy lock, and redacted source label. Raw secrets are excluded from exports. |
| Operation history/recovery lane | Records timestamp, result class, operation scope, recovery action, and support-export ref after review or apply. |
| CLI/headless inspect | Uses the same packet vocabulary as UI and support export. |

## Fixture coverage in the canonical packet

| Scenario | Operation id | Proof |
|---|---|---|
| Manual Cargo add | `operation:cargo:add:anyhow` | Direct bump, requested `^1`, resolved `1.0.86`, root manifest and `Cargo.lock` previewed. |
| Automated Cargo security patch | `operation:cargo:update:openssl-security` | Automation-suggested update remains review-required and exposes native-build risk. |
| AI-suggested pnpm grouped update | `operation:node:update:react-group` | AI lands in the same review sheet with grouped-update, auth, validation, and rollback detail. |
| Lockfile-only refresh | `operation:node:resolve:lockfile-refresh` | Applied-after-review history preserves lockfile-only impact and rollback checkpoint. |
| No results | `operation:cargo:update:major-pilot-no-results` | `no_results` is apply-blocked and distinct from auth, stale mirror, and offline snapshot. |
| Mirror stale | `operation:node:update:workspace-convergence` | Stale mirror remains visible with workspace-wide lockfile impact. |
| Auth required | `operation:node:add:private-auth-required` | Auth-required state blocks apply and routes recovery to the registry auth panel. |
| Offline snapshot only | `operation:cargo:remove:offline-snapshot` | Offline-snapshot remove stays review-required with offline validation and rollback. |

## Summary

- 8 operation review rows.
- 5 review-required rows and 2 apply-blocked rows.
- 3 AI or automation-suggested rows, all review-required.
- 5 registry auth panels covering every credential mode.
- 8 lockfile impact rows covering every stable lockfile-impact class.
- 2 grouped-update plans and 3 operation history/recovery rows.

## Owner sign-off

- `team:dependency_tools` — signed off 2026-06-04 for package mutation review.
- `team:security` — signed off 2026-06-04 for credential redaction and script/native-build risk vocabulary.
- `team:release_engineering` — signed off 2026-06-04 for support-export and recovery lane parity.
