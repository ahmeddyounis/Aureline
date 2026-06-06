# Shared Terminal Debug Control Plane

Shared terminal and debugger control is separate from ordinary collaboration presence. Stable surfaces must show who controls input, the target context, grant expiry or restore state, and the active paste/secret/debug guardrails.

## Stable Behavior

- Presence, cursor follow, support follow, and presenter follow never imply shell or debugger authority.
- Mutating terminal/debugger control requires an explicit grant with issuer, accepter, scope, expiry, revoke path, and target context.
- Only one active driver can hold mutating control for a sensitive surface.
- Browser, mobile, support, join, rejoin, and restore paths default to view-only unless a fresh grant is accepted.
- Restore never replays prior input, signals, debug evaluates, attach actions, or hidden control state.

## User-Visible State

Shared-control chrome should render:

- current driver or view-only state
- presenter or follow/breakaway state when applicable
- target terminal, debugger, or support context
- grant expiry or fresh-grant-required restore state
- paste, secret, signal, debug-evaluate, environment reveal, and live-attach guardrails
- request handoff, revoke, open audit drawer, and open-on-desktop actions where applicable

## Evidence

The canonical packet is `artifacts/collab/m4/shared-terminal-debug-control-plane.json`. Consumers should ingest that packet directly rather than cloning status text. Support exports should carry lineage metadata and guardrail outcomes without raw terminal or debugger content unless a separate explicit evidence profile permits it.
