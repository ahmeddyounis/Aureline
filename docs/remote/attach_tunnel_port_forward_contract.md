# Remote Attach, Tunnel, Port-Forward, and Route-Truth Contract

This contract freezes the cross-surface objects Aureline uses for
remote attach sessions and forwarded endpoints. It connects remote
agent lifecycle, tunnel / port-forward rows, service discovery, and
browser-preview handoff into one route-truth model so later remote
workspace, browser-runtime, managed-preview, support-export, and CLI
surfaces do not invent parallel authority or endpoint fields.

Companion artifacts:

- [`/schemas/remote/attach_session.schema.json`](../../schemas/remote/attach_session.schema.json)
  defines the `remote_attach_session_record`.
- [`/schemas/remote/forwarded_endpoint.schema.json`](../../schemas/remote/forwarded_endpoint.schema.json)
  defines the `forwarded_endpoint_record`.
- [`/fixtures/remote/attach_cases/`](../../fixtures/remote/attach_cases/)
  contains worked attach, reconnect, tunnel, collision, policy-denial,
  paused, degraded, and browser-handoff cases.

This contract composes with, and does not replace:

- [`/docs/network/route_class_matrix.md`](../network/route_class_matrix.md)
  and
  [`/artifacts/network/route_classes.yaml`](../../artifacts/network/route_classes.yaml)
  for canonical route exposure classes (loopback, org-shared, guest, public
  preview, and machine callbacks).
- [`/docs/adr/0020-remote-agent-contract.md`](../adr/0020-remote-agent-contract.md)
  for remote-agent hello, heartbeat, reconnect, target-identity witness,
  and capability narrowing.
- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  and
  [`/artifacts/runtime/action_origin_target_labels.yaml`](../../artifacts/runtime/action_origin_target_labels.yaml)
  for action origin, target, route, exposure, route-change reason, and
  authority-linkage tokens.
- [`/docs/verification/target_and_host_boundary_packet.md`](../verification/target_and_host_boundary_packet.md)
  for target-confidence, host-boundary, wrong-target, and reapproval
  vocabulary.
- [`/docs/runtime/browser_runtime_contract.md`](../runtime/browser_runtime_contract.md)
  and
  [`/schemas/runtime/preview_route.schema.json`](../../schemas/runtime/preview_route.schema.json)
  for browser-runtime and preview-route records that cite forwarded
  endpoints by opaque ref.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  and
  [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  for approval and browser-handoff authority envelopes.

## Scope

Frozen here:

- one `remote_attach_session_record` shape for remote SSH, remote-agent,
  managed-workspace, provider-side, and browser-return callback attach
  sessions;
- one `forwarded_endpoint_record` shape for service discovery rows,
  local loopback forwards, remote port forwards, reverse tunnels,
  managed workspace tunnels, browser-preview routes, notebook endpoints,
  debugger sockets, auth callbacks, webhooks, and machine-to-machine
  callbacks;
- the lifecycle and downgrade states that make reconnect, retarget,
  paused, degraded, provider-unavailable, policy-denied, expired-
  approval, and capability-narrowed states visibly distinct and
  machine-readable;
- endpoint identity, collision handling, stale-target labeling,
  local-only versus shareable link disclosure, browser-handoff
  disclosure, and teardown / revocation semantics.

Out of scope:

- implementing a tunnel backend, route broker, remote agent, managed
  preview service, or browser automation runtime;
- storing raw hostnames, IP addresses, URLs, ports, paths, cookies,
  headers, bearer tokens, environment values, or secret bytes in these
  records;
- defining framework-specific preview behavior. Preview-specific state
  remains in the preview-route and browser-runtime contracts.

## Core Invariants

1. **Attach truth and endpoint truth are separate records.**
   An attach session names the target, auth source, capability posture,
   reconnect policy, and teardown policy. A forwarded endpoint names
   one reachable or discovered service and the exact route / exposure
   posture for that service. A surface that needs both cites both.
2. **No route can widen silently.**
   Moving from `local_only` to any shareable visibility, from read-only
   to mutating authority, or from one target witness to another requires
   a new authority linkage and an explicit route-change reason.
3. **Endpoint identity is opaque but stable.**
   Raw ports and URLs never become identity. Endpoint identity is the
   tuple of `endpoint_id`, `endpoint_generation`, `attach_session_ref`,
   source execution context, target witness, service ref, port / path
   handles, route handle, and collision key.
4. **Target termination is always disclosed.**
   A forwarded endpoint states whether traffic terminates on the local
   device, a remote agent, a managed workspace, a provider surface, or
   an unknown target that requires review.
5. **Secrets are reviewed before projection.**
   The records carry secret-review classes and credential-handle refs
   only. Query strings, cookies, headers, callback bodies, raw bearer
   tokens, and raw environment values never appear.
6. **Route truth survives reconnect and export.**
   Reconnect, stale target, retarget, provider outage, and policy
   narrowing update the lifecycle / downgrade fields; they do not
   rewrite prior origin, target, authority, or endpoint identity.
7. **Teardown is classed, not inferred.**
   Every attach session and endpoint declares what happens on session
   close, trust downgrade, policy epoch change, approval expiry, target
   identity change, provider unavailability, and reconnect-window
   expiry.

## Attach Session Record

`remote_attach_session_record` is the canonical attach row. It is
emitted when a user, CLI, automation path, managed control plane, or
browser-return callback attempts to attach to a remote or managed target.

Required field groups:

| Group | Purpose |
|---|---|
| `target_identity` | Target class, logical target ref, canonical target ref, materialized instance ref, target-identity witness ref, execution-context ref, host-boundary cue stack, reachability, stale-target label, and prior target when retargeting. |
| `route_truth` | Action origin, target, route, exposure, route-change reason, authority-linkage class, invocation-session id, and canonical route-truth ref. |
| `auth_context` | Auth source, credential projection, credential scope, broker / session refs, and secret-exposure review result. |
| `authority` | The ticket or token that admitted the attach: remote attach ticket, approval ticket, browser-handoff packet, managed-control-plane token, supervisor repair ticket, or read-only no-authority posture. |
| `capability_posture` | Requested, admitted, and narrowed capabilities plus typed narrowing reasons and the effective posture. |
| `service_discovery` | Whether services are undiscovered, discovering, active, stale, policy-blocked, provider-unavailable, or unsupported. |
| `browser_preview_handoff` | Whether a browser preview can stay in-product, needs a tunnel disclosure, needs a system-browser handoff, is blocked, or is provider-unavailable. |
| `downgrade_recovery` | The typed downgrade / recovery state, reapproval requirement, route-change reason, local-continuation allowance, cancelled mutation refs, and preserved read-only refs. |
| `teardown_policy` | Which triggers tear down or suspend associated endpoints and which revocation refs prove teardown. |

### Attach Lifecycle States

| State | Meaning | Mutation posture |
|---|---|---|
| `requested` | User or automation intent exists; target resolution has not completed. | No remote mutation. |
| `resolving_target` | Resolver is proving target identity and policy context. | No remote mutation. |
| `attaching` | Auth and capability negotiation are in flight. | No remote mutation until authority is admitted. |
| `active` | Target identity, policy, auth, and capability posture are current. | Admitted capabilities only. |
| `reconnecting` | Transport dropped inside the reconnect window. | In-flight mutations are cancelled; read-only subscriptions may resume only after identity match. |
| `paused` | User, admin, or lifecycle policy paused traffic without closing the session identity. | No endpoint traffic; local-only continuation only when declared. |
| `degraded` | Target is reachable but capability, freshness, latency, or service discovery is below the admitted floor. | Narrows to the declared effective posture. |
| `retarget_review_required` | Target witness changed, became unverifiable, or collided with another target. | Blocked until reapproval or user target selection. |
| `provider_unavailable` | Provider / control plane needed for the attach is unavailable. | Existing local edits continue if allowed; remote mutation blocked. |
| `policy_denied` | Policy denies attach or route creation. | Blocked. |
| `approval_expired` | The ticket that admitted the attach expired or was revoked. | Blocked until reissue. |
| `capability_narrowed` | A previously admitted capability is no longer admitted. | Narrows to read-only, inspect-only, or local-only as recorded. |
| `teardown_pending` | Close or revoke has started, but endpoint revocation evidence is still pending. | No new endpoints. |
| `closed` | Session tickets are revoked and associated endpoints are closed or exported as historical records. | None. |
| `quarantined` | Supervisor or trust policy quarantined the session. | Fresh attach required. |

## Forwarded Endpoint Record

`forwarded_endpoint_record` is the canonical row for one discovered or
reachable service. Service discovery rows use the same record shape as
live forwards so a discovered dev server cannot later become reachable
without preserving identity, target, authority, and secret-review state.

Required field groups:

| Group | Purpose |
|---|---|
| `identity` | Source execution context, service ref, service-discovery ref, target host ref, port / path handles, local bind handle, public route handle, protocol class, target witness, endpoint fingerprint, endpoint generation, and collision key. |
| `route_truth` | Action origin, target, route, exposure, route-change reason, and authority linkage for this endpoint. |
| `exposure` | Visibility, link scope, viewer state, data sensitivity, copy/share disclosure, audience ref, share-link handle ref, and TTL. |
| `auth_posture` | Auth source, authority ticket, credential projection, secret-review posture, and session / broker refs. |
| `collision` | Collision state, handling rule, colliding endpoint refs, selected resolution ref, and review requirement. |
| `target_freshness` | Target freshness, stale-target label, prior target ref, and reapproval requirement. |
| `browser_handoff_disclosure` | Whether handoff disclosure was shown, is pending, is blocked, or is unavailable; plus destination, termination, and viewer-access classes. |
| `teardown` | Current teardown state, trigger, revoke posture, revocation refs, expiry, and summary. |
| `downgrade_recovery` | Network loss, retarget, policy denial, expired approval, capability narrowing, provider unavailable, paused, degraded, stale-target, and closed recovery states. |

### Endpoint Lifecycle States

| State | Meaning |
|---|---|
| `proposed` | Endpoint was requested but is not reachable. |
| `pending_review` | Endpoint creation awaits user, policy, or secret-exposure review. |
| `active` | Endpoint is reachable with current target and authority. |
| `suspended_reconnect` | Attach transport is reconnecting; no traffic is forwarded until identity is revalidated. |
| `paused` | Session or lifecycle policy paused forwarding. |
| `degraded` | Endpoint is live but narrowed, stale, or below health budget. |
| `stale_target` | Endpoint still exists as a record but the target witness is stale or unverifiable. |
| `retarget_review_required` | Endpoint would terminate somewhere different and needs explicit review. |
| `policy_denied` | Policy denies the endpoint. |
| `approval_expired` | Ticket or approval expired. |
| `capability_narrowed` | Attach session no longer admits the endpoint capability. |
| `provider_unavailable` | Route broker, provider, or managed control plane is unavailable. |
| `revoked` | Endpoint was explicitly revoked. |
| `expired` | TTL or lifecycle window elapsed. |
| `closed` | Endpoint has no live route and no reopen path in the current session. |
| `blocked_collision` | Collision handling requires user / policy resolution before any live route. |
| `blocked_secret_review` | Secret exposure review blocked endpoint creation or copy/share. |

## Endpoint Identity and Collision Rules

1. `endpoint_id` is stable for the service intent. `endpoint_generation`
   increments whenever the route handle, target witness, local bind
   handle, or visibility class changes.
2. `endpoint_fingerprint_ref` is derived from opaque refs only. It may
   not include raw hostnames, raw IPs, raw ports, raw paths, raw URLs,
   raw query strings, or raw tokens.
3. A collision with the same target and same service may reuse an
   existing endpoint only when the authority linkage is identical and
   the exposure does not widen.
4. A collision with a different target, stale target, broader
   visibility, or missing authority is blocked as `blocked_collision`
   or `retarget_review_required`.
5. Public, tenant, organization, or persistent environment endpoints
   may not be auto-rebound to a different target. They require fresh
   authority and a new endpoint generation.
6. Local bind collisions prefer a new local bind handle when the
   endpoint remains local-only. They never reuse a public route handle.

## Visibility and Copy / Share Rules

| Visibility | Copy / share rule |
|---|---|
| `local_only` | The link is not shareable and must disclose same-device termination. `share_link_handle_ref` is null. |
| `same_device_lan` | Requires explicit LAN disclosure and cannot be represented as an organization or public route. |
| `workspace_only` | Requires workspace identity, auth posture, and revoke path. |
| `organization_only` / `tenant_only` | Requires org / tenant audience ref, auth class, authority ticket, TTL or policy review window, and audit stream. |
| `public_ephemeral` | Denied by default. Requires explicit step-up, TTL, audit, revoke path, data-sensitivity review, and public-link disclosure. |
| `persistent_environment` | Requires managed or provider authority, persistent environment identity, policy review, and admin revoke path. |
| `blocked` | Copy/share/open actions are disabled and the typed reason is exposed. |

Copying a raw tunnel URL, callback URL, or dev-server URL from outside
the endpoint record is non-conforming. Consumers copy or open a handle
resolved by the route broker at the narrowest boundary.

## Browser-Handoff Disclosure

Browser preview and dev-server flows must disclose:

- where traffic terminates (`local device`, `remote agent`, `managed
  workspace`, `provider`, or `unknown requires review`);
- who can reach the endpoint now;
- whether the viewer sees live state, a captured snapshot, mock/sample
  data, or stale mirrored state;
- whether cookies, local storage, headers, query strings, callbacks, or
  session auth affect the route;
- which authority ticket, browser-handoff packet, or managed token
  admitted the route;
- how to revoke or let the route expire.

Remote preview or dev-server flows that launch a system browser must
cite a `browser_handoff_packet_ref`. If the packet is absent, expired,
revoked, or mismatched to the target witness, the endpoint state is
`approval_expired`, `provider_unavailable`, `retarget_review_required`,
or `blocked_secret_review`; it is never rendered as a normal live
preview.

## Downgrade and Recovery Rules

| Trigger | Attach state | Endpoint state | Recovery |
|---|---|---|---|
| Network loss inside reconnect window | `reconnecting` | `suspended_reconnect` | Resume only after target witness match; mutations are not replayed. |
| Network loss outside reconnect window | `teardown_pending` or `closed` | `expired` or `closed` | Fresh attach required. |
| Target witness changed | `retarget_review_required` | `retarget_review_required` | User target selection or reapproval; endpoint generation increments. |
| Target witness stale / unverifiable | `degraded` or `retarget_review_required` | `stale_target` | Inspect-only until target is proven. |
| Policy denial | `policy_denied` | `policy_denied` | Policy repair or narrowed request. |
| Approval expired / revoked | `approval_expired` | `approval_expired` | Reissue ticket; no silent reuse. |
| Capability narrowed | `capability_narrowed` | `capability_narrowed` | Remove disallowed actions and preserve narrowing reason. |
| Provider unavailable | `provider_unavailable` | `provider_unavailable` | Local-only continuation if declared; wait or explicit handoff. |
| User/admin pause | `paused` | `paused` | Resume with same target witness or reopen through review. |
| Degraded health | `degraded` | `degraded` | Continue only at declared effective posture. |

Every downgrade writes the typed state to both the attach session and
affected endpoints. Support exports and route history quote those
records; they do not infer state from logs.

## Teardown Semantics

Endpoint teardown is triggered by one of:

- user disconnect or manual revoke;
- session close;
- trust downgrade;
- policy epoch change;
- approval expiry or revocation;
- target identity change;
- lifecycle pause;
- provider unavailable beyond the allowed window;
- network loss beyond the reconnect window;
- supervisor quarantine.

Teardown can produce `suspended_no_traffic`, `pending_revoke`,
`revoked`, `expired`, or `closed`. A route in teardown may remain in
history for diagnostics, support export, or audit, but it may not be
opened, copied, shared, or resumed unless a recovery action mints the
required authority and endpoint generation.

## Fixture Coverage

The fixture corpus covers:

- active local-only remote preview with no shareable link;
- network loss and reconnect narrowing that suspends endpoints while
  preserving route truth;
- managed public tunnel request denied by policy before exposure;
- stale target and collision requiring retarget review;
- provider-unavailable browser-handoff flow;
- paused managed workspace where endpoints enter no-traffic state.

Adding fixture rows is additive. Repurposing an existing fixture id is
breaking and requires a replacement fixture rather than rewriting the
meaning of the old one.
