# Stabilize client-origin, target-context, route class, capability-route inspection, and expiry ownership disclosure on provider and remote flows

This stable lane makes provider and remote flows route-explicit and attributable.
For one provider or remote flow on a claimed stable row, the packet binds the
capability-route inspector, the client-origin and target-context disclosure, the
route class and capability boundary, the approval scope and expiry ownership
record, and the cross-surface parity into one export-safe artifact — so UI, CLI,
AI, support export, deep-link, browser handoff, tunnel, and provider-action
surfaces all read the *same* route truth instead of inferring authority from
context or assuming a previous approval still holds. The runtime owner is
`aureline_commands::stabilize_client_origin_route_class`.

A route claim that cannot be read off a stable inspector object is not a stable
route claim. The packet makes every route decision, capability boundary,
approval scope, expiry, and revalidation trigger inspectable and attributable
from every entry surface, and refuses any row where a surface widens capability,
hides the route class, or claims the Stable lane while it is narrowed below it.

## Contract

The packet does **not** re-derive the descriptor, registry, invocation, result,
authority, or command-parity models. The
`aureline_commands::stabilize_command_contract::CommandContractStabilizationPacket`,
`aureline_commands::harden_high_risk_command::HighRiskCommandHardeningPacket`,
and `aureline_commands::finalize_command_parity::CommandParityFinalizationPacket`
own those contracts. This packet binds them by their canonical schema refs and
adds the route-origin and capability-disclosure invariants the stable line needs:

- **Capability-route inspector** — a single `CapabilityRouteInspector` object
  naming the client origin class (`local_ui`, `cli_headless`, `deep_link`,
  `browser_companion`, `ai_tool_call`, `recipe_automation`, `tunnel_endpoint`,
  `provider_action`), the target context class (`local_workspace`,
  `remote_workspace`, `managed_provider`, `external_provider`,
  `tunnel_exposed_endpoint`, `browser_handoff`), the route class (`local`,
  `browser_handoff`, `managed`, `enterprise_gateway`, `tunnel_forwarded`,
  `provider_action_callback`, `byok`), the capability boundary class, and the
  approval scope, expiry, ownership, and revalidation trigger coverage. The
  inspector is reachable without a debug or admin toggle from every claimed
  stable entry point.
- **Expiry and ownership disclosure** — the approval scope record names the
  scope owner, the opaque expiry token, and enforces that an expired approval
  forces visible reapproval rather than silent replay. Both `disclosed_in_preview`
  and `disclosed_in_support_export` must be true for a stable row.
- **Revalidation trigger coverage** — the inspector enumerates all seven triggers
  (`approval_epoch_drift`, `route_or_provider_drift`, `target_identity_drift`,
  `capability_boundary_drift`, `policy_epoch_advanced`, `approval_expired`,
  `explicit_revalidation_requested`). Any drift on a claimed stable row forces
  visible reapproval; silent replay is a `inspector_guards_broken` violation.
- **Cross-surface inspector parity** — the same capability-route inspector object
  is reachable from every claimed stable surface (menu/button, keybinding,
  palette, CLI/headless, AI tool, voice, recipe, deep link, browser companion).
  Any surface that drops route-class disclosure, target-identity disclosure,
  capability-boundary disclosure, approval-scope-and-expiry disclosure, or
  policy checks on a stable, reachable row is an `inspector_surface_parity_broken`
  violation. Any surface narrowed below Stable must not claim the Stable lane.
- **No capability widening** — no surface may widen the capability boundary
  beyond what the descriptor claims. A `no_capability_widening: false` on a
  stable surface is an `inspector_surface_parity_broken` violation.
- **Export boundary** — the packet carries opaque refs, state tokens, and coarse
  classes only. Raw provider URLs, raw credentials, exact token counts, exact
  cost amounts, and billing-account ids stay outside the support boundary.

## Artifact

The canonical support export lives at
`artifacts/commands/m4/stabilize_client_origin_route_class/support_export.json`.
The Markdown summary lives at
`artifacts/commands/m4/stabilize_client_origin_route_class/summary.md`.
The protected fixture directory is
`fixtures/commands/m4/stabilize_client_origin_route_class/`.

These artifacts are referenced by the stable proof index and should be ingested
by dashboards, docs, Help/About surfaces, and support exports instead of cloning
status text.
