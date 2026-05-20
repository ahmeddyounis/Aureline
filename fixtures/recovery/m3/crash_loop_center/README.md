# Crash-loop recovery center beta fixtures

Protected `crash_loop_signal_record` corpus consumed by
[`crates/aureline-support/src/crash_loop_center/mod.rs`](../../../../crates/aureline-support/src/crash_loop_center/mod.rs)
and bound to the boundary schema at
[`schemas/support/crash_loop_recovery.schema.json`](../../../../schemas/support/crash_loop_recovery.schema.json).

Each `*.yaml` file is one detected restart-budget breach (or an explicit
recovery-center request). The evaluator
(`CrashLoopRecoveryCenterBeta::evaluate`) turns each signal into a bounded,
command-backed `crash_loop_recovery_center_record` and a metadata-safe
`crash_loop_recovery_support_packet_record`.

| Fixture | Trigger | Suspected fault domain | Session sensitivity | Covers |
| --- | --- | --- | --- | --- |
| `startup_extension_suspect.yaml` | startup budget exceeded | extension host | privileged or remote | no-silent-rerun gating, targeted extension disable, recovered draft |
| `reopen_profile_suspect.yaml` | reopen budget exceeded | workspace profile or layout | local mutating | targeted profile/layout disable, checkpoint-diff entry |
| `restore_replay_unsafe.yaml` | restore replay failed | restore continuity | local read-only | evidence-only restore class, draft + rollbackable state entries |
| `runtime_host_unknown.yaml` | runtime host budget exceeded | unknown | local mutating | no recent-change suspects, base choices only |
| `explicit_user_request.yaml` | explicit user request | cache or index | local read-only | center opened without a budget breach |

The corpus is `status: protected`: the fixtures encode the recovery contract
and must not be edited to make a failing run pass. See the manifest for the
required case assertions.
