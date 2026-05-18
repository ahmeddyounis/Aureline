# Route Exposure Truth Report

Status: seeded
Schema version: 1
As of: 2026-05-18

## Scope

This report covers the governed route-object and exposure-review record model
that replaces convenience-toggle exposure with reviewable route truth for
port-forward, tunnel, preview-route, and exposed-service rows. The model is
the same one quoted by UI rows, audit events, support exports, and issue
reports — the `route_id` and controlled exposure label are stable across all
of those surfaces.

Two record kinds are governed:

- `route_object_record` — bounded route truth that names source service /
  process, host/workspace identity, opaque port/path handles, exposure
  class, audience, auth/TLS posture, expiry, last-access posture, viewer
  state, data sensitivity, copy/share disclosure, revocation, downgrade, and
  reopen behavior. The boundary schema is at
  [`/schemas/remote/route_object.schema.json`](../../../schemas/remote/route_object.schema.json).
- `exposure_review_record` — typed review sheet a widen, share, copy, or
  open step MUST surface before a route widens its audience. The boundary
  schema is at
  [`/schemas/remote/exposure_review.schema.json`](../../../schemas/remote/exposure_review.schema.json).

Both records reuse one closed `controlled_exposure_label` vocabulary so the
user-visible chip ("Local only", "Same device / LAN", "Authenticated org
route", "Signed preview link", "Public route") is identical to the token
logs, audits, and exports quote.

The Rust implementation lives at
[`/crates/aureline-remote/src/route_governance/`](../../../crates/aureline-remote/src/route_governance/).
The fixture corpus lives at
[`/fixtures/remote/m3/route_exposure_and_revocation/`](../../../fixtures/remote/m3/route_exposure_and_revocation/).

## Controlled exposure labels

| Label                       | Shareable | Publicly reachable | Auth profile expected                  |
|-----------------------------|:---------:|:------------------:|----------------------------------------|
| `local_only`                |     no    |        no          | `no_auth_loopback_only`                |
| `same_device_lan`           |   limited |        no          | `workspace_session_auth`               |
| `authenticated_org_route`   |    yes    |        no          | `workspace_session_auth` or `organization_sso` |
| `signed_preview_link`       |    yes    |       yes          | `signed_preview_link`                  |
| `public_route`              |    yes    |       yes          | `signed_preview_link` with TTL         |

Local-only routes cannot carry a share-link handle, cannot enable the share
action, and cannot use any auth source other than `no_auth_loopback_only`.
Public routes MUST carry an expiry timestamp and MUST use the
`public_link_step_up_required` copy disclosure. The validator surfaces these
rules in [`RouteObject::validate`](../../../crates/aureline-remote/src/route_governance/mod.rs).

## Fixture corpus

The corpus at
[`/fixtures/remote/m3/route_exposure_and_revocation/`](../../../fixtures/remote/m3/route_exposure_and_revocation/)
covers each controlled exposure label and the lifecycle states the chrome
must render honestly. Every case carries an `__fixture__` block with stable
expected values; the integration test in
[`/crates/aureline-remote/tests/route_exposure_and_revocation.rs`](../../../crates/aureline-remote/tests/route_exposure_and_revocation.rs)
loads each case, calls `RouteObject::validate` (and `ExposureReview::validate`
when a review is present), and checks the manifest expectations.

| Fixture | Route kind | Controlled label | Lifecycle | Revoke posture |
|--------|-----------|-----------------|-----------|----------------|
| `local_only_loopback_forward.yaml`                | `local_port_forward`           | `local_only`               | `active`        | `user_self_revoke`       |
| `same_device_lan_devcontainer_service.yaml`       | `devcontainer_exposed_service` | `same_device_lan`          | `active`        | `user_self_revoke`       |
| `authenticated_org_route_managed_workspace.yaml`  | `managed_workspace_tunnel`     | `authenticated_org_route`  | `active`        | `workspace_admin_revoke` |
| `signed_preview_link_browser_preview.yaml`        | `browser_preview_route`        | `signed_preview_link`      | `active`        | `user_self_revoke`       |
| `public_route_reverse_tunnel_expired.yaml`        | `reverse_tunnel`               | `public_route`             | `expired`       | `automatic_expiry_only`  |
| `revoked_org_route_with_stale_links.yaml`         | `managed_workspace_tunnel`     | `authenticated_org_route`  | `revoked`       | `workspace_admin_revoke` |
| `stale_target_remote_port_forward.yaml`           | `remote_port_forward`          | `same_device_lan`          | `stale_target`  | `user_self_revoke`       |

## Acceptance-criteria mapping

- *"Aureline never silently publishes or broadens a route because a preview,
  devcontainer, remote agent, or managed workspace became available."*
  Every controlled-exposure transition is governed by an
  `exposure_review_record` that names the proposed transition, audience,
  data sensitivity, idle timeout, reachability, and lingering local preview.
  The validator rejects denied/blocked reviews that admit a transition and
  rejects public widening that omits a TTL or idle timeout.
- *"Users can revoke or suspend routes without ambiguity and can later
  explain exactly which endpoint was exposed, to whom, for how long, and
  under what auth/expiry posture."*
  Every route carries a `revocation` block (teardown state, revoke posture,
  affected link refs, session impact summary, stale-shared-link state,
  reopen class) and a `last_access` block. The
  [`RevocationSummary::from_route`](../../../crates/aureline-remote/src/route_governance/mod.rs)
  helper derives a stable summary projection that UI rows, audits, and
  support exports quote verbatim.
- *"UI, logs, issue reports, and support exports refer to the same stable
  route identity and exposure class."*
  The `route_id` and `controlled_exposure_label` are opaque, additive-stable
  tokens on the record; the same values are quoted by the audit, support
  packet, and issue surfaces because no surface re-derives them.
- *"Stale, expired, or policy-blocked routes degrade honestly rather than
  lingering as apparently live endpoints."*
  The validator forces non-active lifecycle states to declare a non-active
  teardown state, requires `stale_target` rows to downgrade to
  `stale_target_inspect_only` or `retarget_pending_review`, and forbids
  `stale_target` rows from claiming a `live_service` viewer state. The
  `public_route_reverse_tunnel_expired.yaml` and
  `revoked_org_route_with_stale_links.yaml` fixtures exercise the expired
  and revoked-with-stale-links paths.

## Out of scope

This work does not introduce internet-facing deployment orchestration,
production ingress management, or generic cloud-console replacement. The
record model is bounded to typed route truth that the IDE can surface
honestly.

## Verification

```
cargo build -p aureline-remote
cargo test  -p aureline-remote
```

The test binary runs unit tests in
[`crates/aureline-remote/src/route_governance/tests.rs`](../../../crates/aureline-remote/src/route_governance/tests.rs)
and the fixture-driven integration test in
[`crates/aureline-remote/tests/route_exposure_and_revocation.rs`](../../../crates/aureline-remote/tests/route_exposure_and_revocation.rs).
