# Remote Preview Route Lifecycle, Expiry, Target Identity, and Preview/Runtime Trust Disclosure

- Packet: `remote-preview-route:stable:0001`
- Schema: `schemas/review/add-remote-preview-route-lifecycle-expiry-target-identity-and-preview-runtime-trust-disclosure.schema.json`
- Support export: `artifacts/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/support_export.json`
- Contract doc: `docs/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure.md`
- Fixtures: `fixtures/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/`
- Producer: `aureline_review::current_remote_preview_route_export`

## Coverage

- **Route lifecycle** carries, per route, the durable review anchor, the
  lifecycle phase (`requested`, `provisioning`, `live`, `expiring_soon`,
  `expired`, `revoked`, `failed`, `unknown_phase_provider_owned`), and a bound
  list of lifecycle-event transitions, so the route's progression is explicit and
  reviewable. Every route carries at least one lifecycle event, and
  `unknown_phase_provider_owned` is never flattened into a known phase.
- **Expiry** records, per route, the expiry state, the TTL, an honest expiry
  label, and an auto-revoke flag. A time-bounded route must carry a positive TTL,
  a non-empty label, and `auto_revoke_on_expiry`; a `no_expiry_unbounded` route is
  blocked (`blocked_no_expiry_not_time_bounded`) rather than served, so every
  served route stays time-bounded.
- **Target identity** records, per route, the target identity label, the run the
  preview was built from, and the commit/branch identity, so a remote preview can
  never hide which change a reviewer is looking at.
- **Preview / runtime trust disclosure** records, per route, the host identity
  (class, label, origin disclosed), the runtime trust class, the network egress
  class, whether the runtime executes untrusted code, and whether its writes are
  disclosed. Untrusted remote content, unrestricted/unknown egress, or an unknown
  trust class each require an explicit attention reason before the route is opened.
- **Provider-mode mutation modes** are limited to the three that reach upstream
  host state — `publish_now` (cites an approval ticket), `open_in_provider` (cites
  a browser-handoff packet), and `deferred_publish` (cites a publish-later queue
  item). The local-only `local_draft` mode is intentionally absent because
  publishing a remote preview route cannot remain local.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: the lifecycle phase is explicit; every served route is
time-bounded with enforced auto-revoke; the target identity, host identity,
preview/runtime trust, and network egress are all disclosed; every mutating route
is attributable and records an audit row; the mutation mode cites the grant it
depends on; no route creates hidden write scope; downgrade narrows the claim
instead of hiding the lane; and stale or underqualified rows block promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`route_attribution_missing`, `route_expiry_unbounded`, `route_expired_or_revoked`,
`runtime_trust_undisclosed`, `host_identity_undisclosed`, `trust_narrowing`, and
`upstream_dependency_narrowed`.

## Boundary

Raw preview URLs, raw host names, raw run / log bodies, raw provider payloads, raw
absolute paths, raw author email addresses, credentials, and live provider
responses never cross this boundary. The packet carries only metadata, lifecycle
phases, expiry states, host classes, trust classes, network egress classes,
mutation modes, blocked classes, reviewable labels, and contract references. Every
route, expiry, host, and trust disclosure stays attributable and reviewable before
any upstream effect fires.
