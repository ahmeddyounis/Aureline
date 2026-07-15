# M5 Setting-Definition and Effective-Setting Registries

- Packet: `m5-setting-definition-and-effective-setting-registries:stable:0001`
- Label: `M5 setting-definition and effective-setting registries with one stable setting-definition object resolving per setting, stable setting IDs staying non-recycled, the sensitivity posture disclosed before any sensitive setting is surfaced, canonical / accessible / audit resolution-form coverage, and the complete resolved-value / shadow-chain / lock-state / validation-status / restart-state / capability-availability / last-applied-revision effective-setting object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Setting-definition types: boolean_setting, enum_setting, number_setting, path_setting, secret_reference_setting, type_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **settings_resolver**: `stable`
  - Owner: Settings-resolver owner
  - Scope: The settings resolver resolves the boolean setting definition to one stable object — declared type, stable setting ID, allowed scopes, declared default, migration aliases, restart posture, sensitivity class, and capability dependencies — from the shared registry and derives the effective setting for the winning scope; a definition object missing its declared default and an effective setting that masks a locked value without disclosing its lock source degrade honestly instead of reading as a clean pass
  - Setting-definition entries: 2 / effective-setting entries: 2
- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the enum setting definition and renders the user-scope effective setting while keeping the shadow chain of scopes that lost visible; a resolution-form gap on a definition entry and on an effective setting is caught before a screenshot can reintroduce a false-truth reading
  - Setting-definition entries: 2 / effective-setting entries: 2
- **sync_service**: `stable`
  - Owner: Sync-service owner
  - Scope: The sync service reports the number setting definition and the workspace-scope effective setting without manual reconstruction; a stable setting ID recycled into a different meaning is caught as an ID recycle before it can drift a scope
  - Setting-definition entries: 2 / effective-setting entries: 1
- **policy_service**: `stable`
  - Owner: Policy-service owner
  - Scope: The policy service resolves the path setting definition while disclosing its sensitivity posture and bound to the registry; a definition that is a hand-copied per-entry assumption and an effective setting on an unclassified scope degrade honestly
  - Setting-definition entries: 2 / effective-setting entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved setting-definition and effective-setting truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied settings table
  - Setting-definition entries: 2 / effective-setting entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved setting-definition and effective-setting truth, so a hand-copied constant, an unstated registry token, an ID recycle, or a hidden shadow chain is visible in evidence rather than hidden behind a screenshot
  - Setting-definition entries: 2 / effective-setting entries: 1
