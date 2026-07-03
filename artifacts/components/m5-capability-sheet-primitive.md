# M5 Capability-Sheet Primitive: Consequence Grouping, Transitive Scope, and Re-consent

- Packet: `m5-capability-sheet-primitive:stable:0001`
- Label: `M5 capability-sheet primitive: consequence grouping, transitive scope, reduced mode, and revoke / re-consent`
- Trust lanes: 6 (6 stable)
- Anatomy parts: actor_identity, purpose_text, consequence_grouped_requests, scope_choice, reduced_mode_option, approve_action, deny_action, detail_action, transitive_scope_disclosure
- Consequence classes: read_local_context, modify_workspace, execute_code, network_access, credential_access, system_control
- Scope states: requested_not_granted, granted_full_scope, granted_reduced_scope, transitive_scope_disclosed, re_consent_required, revoked_with_history
- Export fields: actor_identity_repr, consequence_class, capability_token, scope_state, transitive_origin_repr, reduced_mode_offered, revocable_from_settings
- Proof freshness SLO: 720 hours (last refresh: 2026-06-30T00:00:00Z)

## Trust lanes

- **Extension Install**: `stable`
  - Owner: Extension trust owner
  - Scope: The extension-install lane renders the shared capability sheet so an extension's requests read grouped by consequence — a full read of local context and a reduced-scope workspace modification — with the reduced grant disclosed and revocable from the extension trust surface, never a generic 'grant access?' prompt
  - Shell zone: `transient_overlay`
  - Worked sheets: 1
    - `extension:formatter-pack` — 2 request(s), 2 consequence group(s)
- **AI Tool Request**: `stable`
  - Owner: AI tool-gateway owner
  - Scope: The AI-tool lane renders the shared capability sheet so an execute-code request reads as requested-not-granted and a network capability pulled in transitively by a dependency is disclosed with its origin before approval, grouped by consequence rather than by tool API name
  - Shell zone: `transient_overlay`
  - Worked sheets: 1
    - `ai-tool:code-search` — 2 request(s), 2 consequence group(s), widens scope transitively
- **Provider Route**: `stable`
  - Owner: Connected-provider registry owner
  - Scope: The provider-route lane renders the shared capability sheet so a remembered network grant whose scope widened reads as re-consent-required rather than silently re-using the old grant, and the grant stays revocable from the provider trust surface
  - Shell zone: `transient_overlay`
  - Worked sheets: 1
    - `provider-route:hosted-model` — 1 request(s), 1 consequence group(s)
- **Remote Connector**: `stable`
  - Owner: Remote-connector trust owner
  - Scope: The remote-connector lane renders the shared capability sheet so revoking a credential-access grant reads as revoked-with-history — the change is kept in the chronology and the connector cannot silently keep the access
  - Shell zone: `transient_overlay`
  - Worked sheets: 1
    - `remote-connector:build-farm` — 1 request(s), 1 consequence group(s)
- **Automation Flow**: `stable`
  - Owner: Automation governance owner
  - Scope: The automation-flow lane renders the shared capability sheet so a policy pre-approved execute-code capability grants in full while a policy pre-denied system-control capability stays requested-not-granted and can never be approved locally — the policy pre-decisions are preserved on the sheet
  - Shell zone: `transient_overlay`
  - Worked sheets: 1
    - `automation-flow:nightly-sync` — 2 request(s), 2 consequence group(s)
- **Privileged Helper**: `stable`
  - Owner: Privileged-helper trust owner
  - Scope: The privileged-helper lane renders the shared capability sheet so an elevated system-control grant is approved in full and revocable from the helper trust surface, while a credential capability the helper pulls in transitively is disclosed with its origin before approval
  - Shell zone: `transient_overlay`
  - Worked sheets: 1
    - `privileged-helper:installer-service` — 2 request(s), 2 consequence group(s), widens scope transitively
