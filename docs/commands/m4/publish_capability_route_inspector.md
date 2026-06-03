# Publish the capability-route inspector across deep links, approvals, tunnels, remote targets, provider handoffs, and replay-safe command flows

## Overview

This document describes the stable proof packet that publishes the
[`CapabilityRouteInspector`] across six externally routed or high-risk flow
families. The packet is canonical for the M4 stable line in the route/origin
and high-risk-action-safety lane.

## Scope

The packet covers:

- **Deep-link flows** — protocol-handler and share-link invocations
- **Approval flows** — preview-gated, approval-lined write-capable paths
- **Tunnel flows** — SSH, dev-tunnel, reverse-tunnel, and provider-tunnel routed actions
- **Remote-target flows** — actions whose effects land on a remote workspace or helper
- **Provider-handoff flows** — browser-companion and provider-callback handoffs
- **Replay-safe command flows** — rerun, recipe, and macro automation paths

## Record model

### `CapabilityRouteInspectorPacket`

The top-level export-safe record binds:

- one [`CapabilityRouteInspector`] (from `stabilize_client_origin_route_class`)
- one [`FlowPublicationRecord`] per required flow class
- one [`ReapprovalPolicyRecord`] enforcing drift-triggered reapproval
- one [`LineagePreservationRecord`] preserving the inspector end to end
- one [`KeyboardReachabilityRecord`] making the inspector keyboard-reachable
- one [`PublicationSurfaceRow`] per required command surface
- evidence-export lineage refs

### Flow classes

| Token | Meaning |
|-------|---------|
| `deep_link` | Protocol-handler or share-link invocation |
| `approval` | Preview-gated, approval-lined path |
| `tunnel` | SSH, dev-tunnel, reverse-tunnel, provider-tunnel |
| `remote_target` | Effects land on a remote workspace or helper |
| `provider_handoff` | Browser-companion or provider-callback handoff |
| `replay_safe_command` | Rerun, recipe, or macro automation path |

### Drift classes

| Token | Meaning |
|-------|---------|
| `route_drift` | Route or provider changed since last grant |
| `target_drift` | Target identity changed since last grant |
| `policy_drift` | Policy epoch advanced since last grant |
| `host_drift` | Host or workspace boundary changed since last grant |
| `approval_expiry` | Approval expiry passed |

## Invariants

1. **Inspector reachability** — The inspector is reachable from every claimed-stable flow without a debug or admin toggle.
2. **Lineage preservation** — One machine-readable lineage object survives from preview through execution, audit, support export, and shiproom proof.
3. **Drift forces reapproval** — Any route drift, target drift, policy drift, host drift, or approval expiry forces visible reapproval instead of silent replay.
4. **Reversibility** — Browser and provider handoffs stay typed and reversible; opening externally may not widen authority beyond what the inspector disclosed.
5. **Keyboard reachability** — The inspector is reachable from review sheets, command previews, and diagnostic/support surfaces via keyboard.

## Artifact refs

- Support export: `artifacts/commands/m4/publish_capability_route_inspector/support_export.json`
- Markdown summary: `artifacts/commands/m4/publish_capability_route_inspector/summary.md`
- Schema: `schemas/commands/capability_route_inspector.schema.json`
- Fixtures: `fixtures/commands/m4/publish_capability_route_inspector/`
