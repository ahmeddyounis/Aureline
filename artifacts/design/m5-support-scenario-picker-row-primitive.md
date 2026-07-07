# M5 Support-Scenario-Picker-Row Primitive

- Packet: `m5-support-scenario-picker-row-primitive:stable:0001`
- Label: `M5 support-scenario-picker-row primitive: stable scenario family, user-facing symptom cue, claimed launch/deployment/profile scope, bound Doctor finding family, derived picker posture, and bounded reveal/start-diagnosis/start-local-only/confirm-scope/export actions with a same-weight local-only route`
- Support-intake consumers: 5 (5 stable)
- Picker postures: focused_file_scenario, workspace_scenario, account_or_device_scenario, remote_service_scenario, unmapped_scenario, scenario_diagnosis_blocked
- Picker actions: reveal_scenario_lineage, start_diagnosis, start_local_only_diagnosis, confirm_scope, export_scenario
- Scenario families: crash_recovery, performance_health, extension_conflict, data_integrity, connectivity_sync, uncategorized_scenario
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Support-intake consumers

- **Doctor Intake**: `stable`
  - Owner: Doctor intake owner
  - Scope: The Project Doctor intake surface renders the shared support-scenario picker row so a startup crash-recovery scenario scoped to a single file names its stable scenario family, user-facing symptom, claimed scope, and bound startup-health finding with a scenario-coded start-diagnosis action, and a performance-health scenario scoped to the workspace binds its index-integrity finding — both keeping a same-weight local-only diagnosis route
  - Worked rows: 2
    - `scenario:execution-context-startup-crash` (`crash_recovery` / `single_file`) → `focused_file_scenario` (start `true`, local-only `true`, mapped `true`)
    - `scenario:performance-workspace-slowdown` (`performance_health` / `workspace`) → `workspace_scenario` (start `true`, local-only `true`, mapped `true`)
- **Support Center Intake**: `stable`
  - Owner: Support center intake owner
  - Scope: The support-center intake surface renders the shared support-scenario picker row so an extension/host regression scenario whose scope reaches the device/host binds its extension-fault finding and confirms scope before diagnosis, and a state-corruption / schema-drift / low-disk recovery scenario scoped to the account binds its storage-pressure finding — neither understating its incident scope
  - Worked rows: 2
    - `scenario:extension-host-regression` (`extension_conflict` / `device_host`) → `account_or_device_scenario` (start `true`, local-only `true`, mapped `true`)
    - `scenario:state-corruption-low-disk` (`data_integrity` / `account`) → `account_or_device_scenario` (start `true`, local-only `true`, mapped `true`)
- **Recovery Center Intake**: `stable`
  - Owner: Recovery center intake owner
  - Scope: The recovery-center intake surface renders the shared support-scenario picker row so a connectivity/sync scenario covering network/CA/proxy/mirror failure and remote/route/collaboration mismatch scoped to a remote service binds its sync-connectivity finding and confirms scope, and a crash-recovery scenario whose scope is still unknown is treated as remote-reaching until confirmed
  - Worked rows: 2
    - `scenario:network-mirror-collab-mismatch` (`connectivity_sync` / `remote_service`) → `remote_service_scenario` (start `true`, local-only `true`, mapped `true`)
    - `scenario:crash-loop-undetermined-scope` (`crash_recovery` / `unknown_scope`) → `remote_service_scenario` (start `true`, local-only `true`, mapped `true`)
- **Headless / CLI Intake**: `stable`
  - Owner: Headless CLI intake owner
  - Scope: The headless / CLI intake surface renders the shared support-scenario picker row so an uncategorized scenario not yet mapped to a committed Doctor finding family is named once as unmapped and starts diagnosis by gathering evidence, and a state-corruption scenario scoped to a single file resolves to a focused scenario-coded start — proving the same scenario grammar works without a desktop UI
  - Worked rows: 2
    - `scenario:uncategorized-intake` (`uncategorized_scenario` / `workspace`) → `unmapped_scenario` (start `true`, local-only `true`, mapped `false`)
    - `scenario:schema-drift-single-file` (`data_integrity` / `single_file`) → `focused_file_scenario` (start `true`, local-only `true`, mapped `true`)
- **Support Packet Export**: `stable`
  - Owner: Support packet export owner
  - Scope: The support-packet export surface renders the shared support-scenario picker row so a scenario whose scenario-coded live diagnosis is blocked by policy still reads its scenario family, symptom, scope, and finding and keeps the same-weight local-only diagnosis route without ever faking a blocked scenario-coded start, and a performance scenario scoped to the account confirms scope — the same row a support reviewer reads elsewhere
  - Worked rows: 2
    - `scenario:trust-policy-identity-block` (`extension_conflict` / `workspace`) → `scenario_diagnosis_blocked` (start `false`, local-only `true`, mapped `true`)
    - `scenario:performance-account-index` (`performance_health` / `account`) → `account_or_device_scenario` (start `true`, local-only `true`, mapped `true`)
