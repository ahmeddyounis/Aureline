# Remote Attach and Forwarded Endpoint Cases

Worked fixtures for
[`/docs/remote/attach_tunnel_port_forward_contract.md`](../../../docs/remote/attach_tunnel_port_forward_contract.md)
and the boundary schemas at
[`/schemas/remote/attach_session.schema.json`](../../../schemas/remote/attach_session.schema.json)
and
[`/schemas/remote/forwarded_endpoint.schema.json`](../../../schemas/remote/forwarded_endpoint.schema.json).

Every fixture uses opaque refs for targets, hosts, ports, routes,
actors, auth sessions, tickets, and browser-handoff packets. Raw URLs,
raw hostnames, raw IPs, raw ports, raw paths, raw query strings,
cookies, headers, bearer tokens, and secret values do not appear.

| Fixture | Main state | Coverage |
|---|---|---|
| `active_remote_preview_local_only.yaml` | `active` / `local_only` | Remote attach with service discovery and a browser preview that remains local-only. |
| `reconnect_degraded_read_only.yaml` | `reconnecting` / `suspended_reconnect` | Network loss inside reconnect window; mutations cancelled, read-only state preserved. |
| `public_tunnel_policy_denied.yaml` | `policy_denied` | Public tunnel request denied before exposure; no silent widening. |
| `stale_target_retarget_collision.yaml` | `retarget_review_required` / `blocked_collision` | Target witness changed and collides with an existing endpoint. |
| `provider_unavailable_browser_handoff.yaml` | `provider_unavailable` | Browser handoff cannot complete because the route provider is unavailable. |
| `paused_managed_workspace_no_traffic.yaml` | `paused` | Managed workspace pause leaves endpoints suspended with no traffic. |
