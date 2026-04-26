# Crash-Loop Card Fixtures

Worked JSON fixtures for
[`/schemas/ux/restore_fidelity.schema.json`](../../../schemas/ux/restore_fidelity.schema.json)
and
[`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../../../docs/ux/crash_loop_and_restore_fidelity_contract.md).

| Fixture | Scenario |
| --- | --- |
| [`extension_host_crash_loop.json`](./extension_host_crash_loop.json) | Repeated extension-host startup failure escalates to safe mode and suspect-extension disable. |
| [`remote_target_expired_layout_only.json`](./remote_target_expired_layout_only.json) | Display and remote/managed-session drift lower restore to layout-only with placeholders. |
| [`corrupt_state_evidence_only.json`](./corrupt_state_evidence_only.json) | Corrupt restore state falls back to evidence-only while journals and forensics remain inspectable. |

Each fixture keeps evidence refs distinct from restored UI claims and
uses explicit action records for what is preserved, discarded, or
deferred.
