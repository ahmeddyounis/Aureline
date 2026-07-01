# M5 lifecycle-object certification: status-surface, status-code, last-failure-reason, and recovery-affordance truth on every long-lived M5 object

Generated from the seeded packet in
[`crate::m5_lifecycle_object_certification`](../../crates/aureline-shell/src/m5_lifecycle_object_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_object_certification -- markdown > \
  artifacts/lifecycle/m5-lifecycle-object-certification.md
```

- Packet id: `m5-lifecycle-object-certification:stable:0001`
- Source schema ref: `schemas/lifecycle/m5-lifecycle-object-certification.schema.json`
- Certifies matrix packet: `m5-lifecycle-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required bindings: `primary_status_surface`, `exportable_status_code`, `last_failure_reason`, `named_recovery_affordance`
- Object families certified: 13
- Green (full binding): 9
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Object family | Status | Status surface | Status code | Last-failure reason | Recovery affordance | Headless | Waiver |
| ------------- | ------ | -------------- | ----------- | ------------------- | ------------------- | -------- | ------ |
| Workspace / window session | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Installed extension / capability | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Remote / tunnel session | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Live collaboration session | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| AI assistant action | `yellow` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `disclosed_generic_reason` | `named_recovery_present` | `true` | — |
| Update / rollback lifecycle | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Notebook kernel runtime | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Request / API run | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Preview / live-server session | `yellow` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `disclosed_reduced_recovery` | `true` | — |
| Pipeline / task run | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Data / database session | `green` | `bound_to_one_primary_surface` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Profiler / trace capture | `yellow` | `bound_to_one_primary_surface` | `disclosed_partial_export` | `controlled_reason_reported` | `named_recovery_present` | `true` | — |
| Companion / paired device session | `yellow` | `disclosed_surface_relocation` | `stable_code_exports_everywhere` | `controlled_reason_reported` | `named_recovery_present` | `true` | `waiver:companion-surface-relocation:0001` |

## Auto-narrowed rows

- `ai_action` (`yellow`) — When a policy or upstream control blocks an AI action before the specific reason class is resolved, the action discloses a generic but still-controlled last-failure reason class rather than raw text or a missing reason, so the AI object is narrowed and disclosed.
- `preview_session` (`yellow`) — When a preview session's live-server dependency is unavailable, the object offers a disclosed reduced rebuild affordance that requires the dependency to return before the full rebuild is possible, while still naming a path forward, so the preview object is narrowed and disclosed.
- `profiler_capture` (`yellow`) — An in-flight or headless profiler capture exports a disclosed coarse status code on a subset of surfaces until the capture is finalized, while still naming the same controlled state, so the profiler object is narrowed and disclosed rather than losing its exportable code.
- `companion_session` (`yellow`) — When a paired companion device drops, the companion presence badge is unavailable, so the session's lifecycle state is relocated to a disclosed, waivered still-visible activity-center reconnect prompt rather than disappearing, so the companion object is narrowed and disclosed while a named reconnect stays reachable.

## Exact object causes

- `ai_action` — `upstream_dependency_narrowed` (disclosed: `true`) — The object falls back to a disclosed generic but still-controlled last-failure reason class when the specific class is not yet available, so the reason is narrowed and disclosed rather than raw or missing.
- `preview_session` — `upstream_dependency_narrowed` (disclosed: `true`) — The object offers a disclosed reduced recovery affordance while still naming a path forward, so recovery is narrowed and disclosed rather than absent.
- `profiler_capture` — `upstream_dependency_narrowed` (disclosed: `true`) — The object's status code exports in a disclosed reduced form on a subset of surfaces while still naming the same controlled state, so the export is narrowed and disclosed rather than lost.
- `companion_session` — `upstream_dependency_narrowed` (disclosed: `true`) — The object's canonical primary status surface is unavailable, so its state is relocated to a disclosed, waivered still-visible fallback surface rather than disappearing; the relocation is disclosed and the single-surface binding is restored when the dependency returns.

## Active waivers

- `waiver:companion-surface-relocation:0001` (`companion_session`, owner: Companion owner, expires `2026-09-30T00:00:00Z`) — When a paired companion device drops, the companion presence badge is unavailable, so the session's lifecycle state is relocated to a disclosed, still-visible activity-center reconnect prompt in the primary window rather than vanishing; the relocation is disclosed, never silent, and the single-surface binding is restored when the device reconnects.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_object_certification -- validate
cargo test -p aureline-shell --test m5_lifecycle_object_certification_fixtures
```
