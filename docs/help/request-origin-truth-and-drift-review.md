# Request origin and rerun drift review

Every request in Aureline shows **where it runs**. `localhost`, a container
service name, and a private DNS name mean different things depending on the
execution path, so the origin is always explicit and never silently changes
between runs.

## Origin classes

A request resolves through one of five execution paths, each with its own trust
boundary:

- **Local desktop** — a loopback service on your machine. Only this path keeps
  desktop-local trust.
- **SSH** — a host reached over an SSH tunnel. `localhost` here is the remote
  host, not your desktop.
- **Container** — a compose or container service name resolved inside a runtime.
- **Managed workspace** — a managed or cloud-hosted tenant endpoint.
- **Browser companion** — a browser-companion runtime, often against a private
  DNS target.

SSH, container, managed, and browser-companion origins never inherit your
desktop-local trust or naming.

## What you see before send

The composer and request list show the **origin class** and the **target
identity**. If a saved request now resolves to a different host, lane, or trust
boundary than the last run, an **origin-changed warning** appears and the
request is held for review.

## Rerun review

When you rerun a saved request, the rerun sheet distinguishes two modes:

- **Rerun exactly** re-dispatches against the *exact* origin and snapshot that
  were recorded. Nothing re-resolves, so the origin cannot drift.
- **Rerun with current context** re-resolves the origin through your current
  environment. This can drift — a private DNS name may have rebound, a service
  may have moved, or the lane may have changed.

If a rerun-with-current-context resolves to a changed origin, dispatch is
**blocked until you review** the enumerated changes — host identity, origin lane,
trust boundary, port or service, or a private-DNS rebinding — and acknowledge
them.

## Trust and safety

Request history is never widened toward unsafe body or header retention just to
support rerun or compare. Origin truth in support exports carries the origin
class, target identity, trust boundary, and enumerated changes — never raw
bodies, headers, or secrets.
