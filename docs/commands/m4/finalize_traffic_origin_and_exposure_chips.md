# Finalize traffic-origin and exposure chips, tunnel, port, and publish-target explainability

This stable lane makes every tunnel-open, port-forward, and publish-target
action origin-explicit and exposure-explicit. For one command or network-action
family on a claimed stable row, the packet binds the traffic-origin chip set,
exposure chip set, tunnel/port/publish-target explainability records, and
cross-surface chip parity into one export-safe artifact — so UI, CLI, AI,
support export, deep-link, browser handoff, and provider-action surfaces all
read the same traffic-origin and exposure truth rather than inferring authority
from context. The runtime owner is
`aureline_commands::finalize_traffic_origin_and_exposure_chips`.

An exposure claim that cannot be read off a stable chip or explainability record
is not a stable exposure claim. The packet makes every traffic-origin class,
exposure class, tunnel, port, and publish target inspectable and attributable
from every entry surface, and refuses any row where a surface widens traffic
authority, hides origin or exposure, or claims the Stable lane while it is
narrowed below it.

## Contract

The packet does **not** re-derive the descriptor, registry, invocation, result,
authority, high-risk hardening, command-parity, or client-origin/route-class
models. The
`aureline_commands::stabilize_client_origin_route_class::ClientOriginRouteClassPacket`
owns the capability-route inspector, approval scope, and cross-surface inspector
parity. This packet binds those by their canonical schema refs and adds the
traffic-origin chip, exposure chip, tunnel, port, and publish-target
explainability invariants the stable line needs:

- **Traffic-origin chip set** — for each declared traffic-origin class
  (`local_process`, `loopback_client`, `tunnel_ingress`, `port_forward_ingress`,
  `publish_target_relay`, `provider_callback`, `browser_companion_relay`) a chip
  is visible in preview, approval, active-session status, and support export,
  bound to the named origin and never inferred from ambient context.
- **Exposure chip set** — for each exposed endpoint the exposure class
  (`unexposed`, `localhost_only`, `port_forwarded`, `tunnel_exposed`,
  `publicly_published`, `provider_managed`, `enterprise_gateway`) is surfaced in
  the chip object, never implicit. Any elevation from a narrower class to a
  wider one forces visible reapproval; silent class elevation is a policy
  violation.
- **Tunnel explainability** — for each active or pending tunnel a single
  structured record names the tunnel kind (`ssh_forward`, `dev_tunnel`,
  `reverse_tunnel`, `provider_tunnel`), the target-port ref (opaque), the
  traffic-origin class, the exposure class, the approval scope ref, and the
  `drift_forces_reapproval` invariant. The record is reachable from preview,
  approval, active-session UI, CLI, AI-tool run, and support export without a
  debug or admin toggle. A tunnel record where `drift_forces_reapproval` is
  false, or that is not disclosed in chip, preview, or support export, is a
  `tunnel_record_guards_broken` violation.
- **Port explainability** — for each forwarded or exposed port a single
  structured record names the port ref (opaque, never the raw port number), the
  protocol class (`tcp`, `http`, `https`, `ws`, `wss`, `udp`), the exposure
  class, and the traffic-origin class. A port record not disclosed in chip or
  support export is a `port_record_guards_broken` violation.
- **Publish-target explainability** — for each publish target a single
  structured record names the target class (`static_host`, `container_registry`,
  `managed_service`, `external_host`, `provider_push`), the exposure class, the
  traffic-origin class, the approval scope ref, and the spend-posture token. A
  publish-target record not disclosed in chip or support export, or missing an
  approval scope ref or spend-posture token, is a
  `publish_target_record_guards_broken` violation.
- **Cross-surface chip parity** — the same traffic-origin and exposure chips are
  reachable from every claimed stable surface (menu/button, keybinding, palette,
  CLI/headless, AI tool, voice, recipe, deep link, browser companion). Any
  surface that drops origin-class disclosure, exposure-class disclosure, or
  network explainability on a stable, reachable row is a
  `chip_surface_parity_broken` violation. Any surface narrowed below Stable must
  not claim the Stable lane.
- **No authority widening** — no surface may widen traffic authority beyond what
  the descriptor claims. A `no_authority_widening: false` on a stable surface is
  a `chip_surface_parity_broken` violation.
- **Export boundary** — the packet carries opaque refs, state tokens, and coarse
  classes only. Raw port numbers, raw endpoint hostnames, raw tunnel URLs, raw
  credentials, exact cost figures, and billing-account ids must not appear in the
  export. Any field carrying such material is a `raw_material_in_export`
  violation.

## Schemas and artifacts

- Schema: `schemas/commands/finalize_traffic_origin_and_exposure_chips.schema.json`
- Descriptor contract: `docs/commands/command_descriptor_contract.md`
- Parity contract: `docs/commands/invocation_result_and_parity_contract.md`
- Support export: `artifacts/commands/m4/finalize_traffic_origin_and_exposure_chips/support_export.json`
- Summary: `artifacts/commands/m4/finalize_traffic_origin_and_exposure_chips/summary.md`
- Fixtures: `fixtures/commands/m4/finalize_traffic_origin_and_exposure_chips/`
