# Route Exposure Class Matrix

This document publishes the canonical **route exposure classes** for every
shareable or externally reachable path in Aureline (preview share links,
forwarded endpoints, tunnels / port-forwards, provider callbacks, and other
route-brokered paths).

The goal is to make exposure, TTL, and approval semantics **mechanical**:
surfaces and exports must not infer or paraphrase reachability ad hoc.

Companion artifacts:

- [`/artifacts/network/route_classes.yaml`](../../artifacts/network/route_classes.yaml)
  is the machine-readable route exposure class matrix.
- [`/fixtures/network/route_class_examples/`](../../fixtures/network/route_class_examples/)
  contains worked examples of allowed, downgraded, and blocked flows.

Important: `route_exposure_class` is **not** the same as the transport
`route_class` in `transport_decision_record` (direct loopback vs proxy vs
relay). `route_exposure_class` describes **who/what can reach a route** and
what gating applies.

## Contract Rules

1. Any shareable, preview, callback, or tunnel route MUST resolve exactly one
   `route_exposure_class` from
   [`/artifacts/network/route_classes.yaml`](../../artifacts/network/route_classes.yaml).
2. If a route cannot be mapped deterministically, it MUST be blocked as
   unknown (deny-by-default) rather than guessed.
3. `public_preview` is denied by default. It MAY be admitted only when a
   higher-trust flow steps it up (explicit approval ticket, signed org bundle,
   or managed policy admission) and the route remains timeboxed and revocable.
4. `machine_callback` routes MUST validate correlation to a prior packet /
   ticket / intent envelope before mutating local state. They are not human
   share links.
5. Support/audit exports reconstruct exposure semantics from:
   `route_exposure_class` + the per-class required fields. They MUST NOT carry
   raw URLs, raw tokens, or raw callback bodies.

## Route Exposure Classes

| `route_exposure_class` | Allowed audience (default) | TTL posture (default) | Typical uses | Default approval expectation | Badge requirements (minimum) | Revocation path (must exist when shareable) | Required support / audit fields |
|---|---|---|---|---|---|---|---|
| `loopback_local` | Same-device operator only | `ephemeral_session_only` | Local preview, local loopback callback, local-only forwards | `end_user_implicit` | local-only / same-device termination disclosed | Stop session / stop server (local) | `route_exposure_class`, termination class, route handle ref |
| `org_shared` | Signed-in users at workspace/org/tenant scope | `policy_window` | Org/tenant share links, managed workspace endpoints | `signed_org_bundle_approved` or `admin_policy_approved` | authenticated + scope disclosed | Revoke handle + policy authority | `route_exposure_class`, audience scope ref, auth class, revoke handle ref, policy epoch refs |
| `external_guest` | Authenticated external viewer(s) (invite/allowlist) | `short_window` | External share via controlled relay, cross-boundary review links | `user_consent_recorded` (often requires step-up) | external audience disclosed + auth posture disclosed | Revoke handle + expiry | `route_exposure_class`, guest/audience ref (opaque), auth class, expiry, revoke handle ref |
| `public_preview` | Anyone with the link/token (no identity guarantee) | `short_window` | Public preview links or public tunnel endpoints | `exception_required` (step-up required) | public link disclosure + data-sensitivity disclosure | Revoke handle + explicit expiry | `route_exposure_class`, expiry, approval evidence ref, revoke handle ref, policy source |
| `machine_callback` | Provider/webhook/machine actor (non-human) | `short_window` | OAuth loopback return, webhook delivery, device-code completion | `approval_not_applicable` (but correlation is mandatory) | callback origin + correlation disclosure | Disable callback route handle; revoke packet/ticket | `route_exposure_class`, callback envelope ref, correlation ref, replay posture, originating packet/ticket ref |

The TTL and approval vocabulary above reuses the stable token sets already used
in other network contracts (for example `effective_route_state_record`). The
route exposure class, however, is the primary key for exposure semantics.

## Mapping Rules

This section defines the canonical mapping from concrete flows to
`route_exposure_class`. Mappers MUST NOT consult UI copy or free-form
descriptions.

### Forwarded endpoints (`forwarded_endpoint_record`)

Source vocabulary:

- `schemas/remote/forwarded_endpoint.schema.json`:
  - `endpoint_class`
  - `visibility_class`

Mapping:

1. If `endpoint_class` is one of:
   `auth_callback_route`, `webhook_callback_route`, `machine_to_machine_callback`,
   then `route_exposure_class = machine_callback` regardless of `visibility_class`.
2. Otherwise map by `visibility_class`:
   - `local_only` → `loopback_local`
   - `workspace_only` / `organization_only` / `tenant_only` / `persistent_environment` → `org_shared`
   - `same_device_lan` → `external_guest` (non-identity LAN sharing is treated as guest-level exposure)
   - `public_ephemeral` → `public_preview` (still denied by default unless stepped up)
   - `blocked` / `visibility_unknown_requires_review` → `blocked_unknown_requires_review`

### Tunnel / port-forward publish flows (`tunnel_publish`)

Tunnel and port-forward publication is treated as a route exposure decision:

- The publishable object is the `forwarded_endpoint_record` (reverse tunnel,
  port forward, browser preview route, managed workspace tunnel).
- The transport permission class is typically `tunnel_publish` and the audit
  payload MUST carry `route_exposure_class`.
- The canonical exposure class is the result of the forwarded-endpoint mapping
  above; publish flows MUST NOT invent a separate exposure vocabulary.

### Preview share links (`preview_share_link_record`)

Source vocabulary:

- `schemas/preview/preview_share_link.schema.json`:
  - `share_audience_class`
  - `share_destination_class`
  - `share_auth_class`

Mapping:

- `share_audience_class = workspace_local_only_no_share` → `loopback_local`
- `share_audience_class` in `{workspace_signed_in_only, organization_signed_in_only, tenant_signed_in_only}` → `org_shared`
- `share_audience_class` in `{temporary_external_link_audience, one_time_external_link_audience}`:
  - `share_destination_class = approved_third_party_relay` → `external_guest`
  - `share_destination_class` in `{temporary_external_browser_link, one_time_external_browser_link}` → `public_preview`
- `share_audience_class` in `{policy_blocked_no_share_audience, not_shareable_inherent_surface}` → `blocked_unknown_requires_review`

### Provider-linked callback flows

Provider callbacks and webhooks are never treated as human share links:

- Any flow described as a browser return, webhook delivery, device-code
  completion, or machine-to-machine callback MUST resolve to
  `route_exposure_class = machine_callback`.
- The callback handler MUST reject callbacks that cannot be correlated to a
  live packet/ticket/intent envelope (deny-by-default).

### Unknowns

When any required source fields are missing, contradictory, or outside the
known vocabulary, the route MUST be blocked as unknown. It MUST NOT be mapped
to a weaker class such as `loopback_local`.
