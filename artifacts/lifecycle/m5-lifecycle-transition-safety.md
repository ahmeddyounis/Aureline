# M5 lifecycle transition safety: safe retry/cancel/rollback rules, transition attribution, checkpoint sequencing, and the protected local-editing fallback on every long-lived M5 object

Generated from the seeded packet in
[`crate::m5_lifecycle_transition_safety`](../../crates/aureline-shell/src/m5_lifecycle_transition_safety/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_transition_safety -- markdown > \
  artifacts/lifecycle/m5-lifecycle-transition-safety.md
```

- Packet id: `m5-lifecycle-transition-safety:stable:0001`
- Source schema ref: `schemas/lifecycle/m5-lifecycle-transition-safety.schema.json`
- Certifies matrix packet: `m5-lifecycle-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required transition dimensions: `safe_transition`, `transition_attribution`, `checkpoint_sequencing`, `local_fallback`
- Object families certified: 13
- Green (full safety): 9
- Yellow (auto-narrowed): 4
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Object family | Status | Safe transition | Attribution | Checkpoints | Local fallback | Headless | Waiver |
| ------------- | ------ | --------------- | ----------- | ----------- | -------------- | -------- | ------ |
| Workspace / window session | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Installed extension / capability | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Remote / tunnel session | `yellow` | `disclosed_reduced_transition_set` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Live collaboration session | `yellow` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `disclosed_reduced_fallback` | `true` | `waiver:collaboration-reduced-fallback:0001` |
| AI assistant action | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Update / rollback lifecycle | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Notebook kernel runtime | `yellow` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `disclosed_compacted_checkpoints` | `local_editing_protected_fallback` | `true` | — |
| Request / API run | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Preview / live-server session | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Pipeline / task run | `yellow` | `safe_retry_cancel_rollback_rules` | `disclosed_coarse_attribution` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Data / database session | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Profiler / trace capture | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |
| Companion / paired device session | `green` | `safe_retry_cancel_rollback_rules` | `actor_subsystem_attributed` | `required_checkpoints_enforced` | `local_editing_protected_fallback` | `true` | — |

## Auto-narrowed rows

- `remote_session` (`yellow`) — While a remote session is reconnecting, it exposes a disclosed reduced transition set — a pending cancel is deferred until the reconnect resolves so the tunnel is never left half-torn-down — while retry, rollback, and compensation stay safe, so the remote object is narrowed and disclosed rather than allowing an unsafe cancel.
- `collaboration_session` (`yellow`) — When a live collaboration session loses its shared connection, the object keeps a disclosed, waivered reduced local-editing fallback — local edits continue read-only against the last synced snapshot until the session rejoins — while still keeping a safe local path, so the collaboration object is narrowed and disclosed rather than losing local editing.
- `notebook_runtime` (`yellow`) — A fast notebook cell run presents its required queue / execute / render checkpoints in a disclosed compacted progress on the inline cell surface while still naming each milestone individually, so the notebook object is narrowed and disclosed rather than collapsing its checkpoints into an anonymous spinner.
- `pipeline_run` (`yellow`) — A fan-out pipeline run attributes an in-flight transition to a disclosed coarse stage group rather than the exact task actor until the specific task that drove the transition is resolved, while still naming a controlled subsystem, so the pipeline object is narrowed and disclosed rather than dropping attribution.

## Exact transition causes

- `remote_session` — `upstream_dependency_narrowed` (disclosed: `true`) — The object exposes a disclosed reduced set of safe transitions on a subset of surfaces — for example deferring cancel until a reconnect or checkpoint resolves — while retry, rollback, and compensation stay safe, so the transition set is narrowed and disclosed rather than unsafe.
- `collaboration_session` — `upstream_dependency_narrowed` (disclosed: `true`) — When the managed, collaborative, AI, or remote lane degrades, the object keeps a disclosed, waivered reduced local-editing fallback — for example continuing local edits read-only until the lane rejoins — while still keeping a safe local path, so the fallback is narrowed and disclosed rather than lost.
- `notebook_runtime` — `upstream_dependency_narrowed` (disclosed: `true`) — The object presents its required checkpoints in a disclosed compacted form on a compact surface while still naming each milestone individually, so the checkpoint sequence is narrowed and disclosed rather than collapsing into an anonymous spinner.
- `pipeline_run` — `upstream_dependency_narrowed` (disclosed: `true`) — The object attributes a transition to a disclosed coarse subsystem group rather than the exact actor until the specific attribution resolves, while still naming a controlled subsystem, so attribution is narrowed and disclosed rather than missing.

## Active waivers

- `waiver:collaboration-reduced-fallback:0001` (`collaboration_session`, owner: Collaboration owner, expires `2026-09-30T00:00:00Z`) — When a live collaboration session loses its shared connection, the object keeps a disclosed, still-safe local-editing fallback: local edits continue read-only against the last synced snapshot until the session rejoins, rather than blocking editing outright. The reduced fallback is disclosed, never silent, and the full read-write local fallback is restored the moment the collaboration lane rejoins.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_transition_safety -- validate
cargo test -p aureline-shell --test m5_lifecycle_transition_safety_fixtures
```
