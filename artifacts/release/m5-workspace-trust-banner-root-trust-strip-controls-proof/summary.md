# M5 Workspace-Trust-Banner and Root-Trust-Strip Controls

- Packet: `m5-workspace-trust-banner-root-trust-strip-controls:stable:0001`
- Label: `M5 workspace-trust-banner and root-trust-strip controls with object identity, trust class, grant source, policy epoch, narrowed-capability, per-root trust, and mixed-root honesty`
- Consumer surfaces: 5
- Trust scopes: trusted_workspace, trusted_root, restricted_workspace, mixed_root, policy_blocked, scope_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **workspace_trust_ui**: `stable`
  - Owner: Workspace trust owner
  - Scope: The workspace-trust UI renders one banner naming the trusted object, trust class, grant source, and policy epoch, and one root-trust strip per root so a mixed-root workspace never reads as blanket trust
  - Banner examples: 2 / strip examples: 2
- **settings_ui**: `stable`
  - Owner: Settings trust owner
  - Scope: The settings trust pane reuses the same trust/root vocabulary, names the narrowed capability a restricted workspace removes, and degrades honestly when the grant source is undisclosed
  - Banner examples: 2 / strip examples: 2
- **safe_mode_ui**: `stable`
  - Owner: Safe mode owner
  - Scope: Safe mode shows the policy-blocked banner with its policy epoch and the per-root trust strip, degrading honestly when a policy epoch or per-root trust cannot be resolved
  - Banner examples: 2 / strip examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved banner and strip truth, so a mixed-root workspace collapsed into uniform trust or a missing trust-detail path is visible in evidence rather than hidden
  - Banner examples: 2 / strip examples: 2
- **product_ui**: `stable`
  - Owner: In-product trust owner
  - Scope: In-product surfaces reuse the same trust/root vocabulary a user sees in the workspace-trust UI, always offering the command-backed detail path and degrading honestly when object or root identity is unstated
  - Banner examples: 4 / strip examples: 2
