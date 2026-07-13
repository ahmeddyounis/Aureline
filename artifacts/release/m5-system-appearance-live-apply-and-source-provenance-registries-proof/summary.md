# M5 System-Appearance Live-Apply and Appearance-Source-Provenance Registries

- Packet: `m5-system-appearance-live-apply-and-source-provenance-registries:stable:0001`
- Label: `M5 system-appearance live-apply and appearance-source-provenance registries with live-apply / restart-required / unsupported postures, canonical posture labels (applies live / restart required / not supported on this host), applied / canonical / accessible response-form coverage, preserved active-context continuity, and stable-ID / record-surface / source-signal provenance across shell, settings, docs, onboarding, CLI, and support surfaces`
- Consumer surfaces: 6
- Support postures: live_apply, restart_required, unsupported, posture_unclassified
- Response forms: applied_visual_reapply, canonical_posture_truth, accessible_announcement
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell reapplies the system theme live to the shell chrome from the shared appearance registry and records the active appearance source in settings; a hand-copied per-platform response and a source that is not recorded degrade honestly instead of reading as a clean pass
  - Response entries: 2 / provenance entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings preview reapplies the system theme live from the registry and records the source and posture in diagnostics; a live-apply entry that did not reapply live is caught as mislabeled for its posture
  - Response entries: 2 / provenance entries: 1
- **docs_help**: `stable`
  - Owner: Docs/help surface owner
  - Scope: Docs and help render the live theme response across the applied, canonical, and accessible response forms and record the source in the support export; a response and a provenance record that omit a response form degrade honestly so a diagnostics panel cannot reintroduce an incorrect posture
  - Response entries: 2 / provenance entries: 2
- **onboarding**: `stable`
  - Owner: Onboarding surface owner
  - Scope: Onboarding reapplies the live contrast response to the active editor from the registry while preserving active-context continuity; a change that resets local context and a record with an unclassified record surface degrade honestly
  - Response entries: 2 / provenance entries: 1
- **cli_export**: `stable`
  - Owner: CLI/export owner
  - Scope: The CLI export records the restart-required text-scale posture from the appearance registry and explains the narrower behavior; a restart-required change with no explained fallback degrades honestly instead of silently narrowing
  - Response entries: 2 / provenance entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved appearance-response and provenance truth, so a hand-copied response or an unstated registry token is visible in evidence rather than hidden behind a diagnostics panel
  - Response entries: 2 / provenance entries: 1
