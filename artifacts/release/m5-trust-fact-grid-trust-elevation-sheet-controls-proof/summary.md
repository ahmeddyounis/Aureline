# M5 Trust-Fact-Grid and Trust-Elevation-Sheet Controls

- Packet: `m5-trust-fact-grid-trust-elevation-sheet-controls:stable:0001`
- Label: `M5 trust-fact-grid and trust-elevation-sheet controls with actor, object, scope, policy source, capability delta, reduced-mode alternative, lasting-versus-one-time effect, and no-ambient-grant honesty`
- Consumer surfaces: 5
- Trust scopes: trusted_workspace, trusted_root, restricted_workspace, mixed_root, policy_blocked, scope_unknown
- Effect classes: lasting_until_revoked, one_time_this_session, single_action_only, effect_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **workspace_trust_ui**: `stable`
  - Owner: Workspace trust owner
  - Scope: The workspace-trust UI renders one trust-fact grid naming actor, object, scope, policy source, capability, and per-root trust, and one elevation sheet reviewing a workspace-scoped and a root-scoped grant before approval so a trusted root never reads as a trusted workspace
  - Grid examples: 2 / sheet examples: 2
- **settings_ui**: `stable`
  - Owner: Settings trust owner
  - Scope: The settings trust pane reuses the same field and delta grammar, names the capability delta a restricted grant changes, and degrades honestly when the capability delta is left unnamed
  - Grid examples: 2 / sheet examples: 2
- **safe_mode_ui**: `stable`
  - Owner: Safe mode owner
  - Scope: Safe mode shows the policy-blocked grid with its policy epoch and a mixed-root elevation reviewing the reduced-mode alternative, degrading honestly when the trust scope cannot be resolved or the reduced-mode path is hidden
  - Grid examples: 2 / sheet examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved grid and sheet truth, so a mixed-root workspace collapsed into uniform trust, an approval implying ambient scope, or a missing trust-detail path is visible in evidence rather than hidden
  - Grid examples: 2 / sheet examples: 2
- **product_ui**: `stable`
  - Owner: In-product trust owner
  - Scope: In-product surfaces reuse the same field and delta grammar a user sees in the workspace-trust UI, always offering the command-backed detail path and degrading honestly when object, actor, grant source, or effect duration is unstated
  - Grid examples: 4 / sheet examples: 5
