# API request profile certification

Aureline certifies the API request lane across every way you can run a request —
not just on your desktop. A request profile is a way of sending: locally, from the
CLI, to a remote host, into a container, against a managed workspace, through the
browser companion, or from a collection you reopened offline or from a mirror.

Certification is **evidence-bound** and **honest**: a green send button never
hides a stale schema, a drifted persisted operation, or an origin that changed.

## What is certified

For each profile, the certification covers:

- **Collections** — request definitions stay text-first and versionable, and
  export keeps redaction classes without persisting raw secrets.
- **Contract and GraphQL freshness** — a stale or unavailable schema is labeled,
  and a live-validated send is blocked rather than silently sent as raw text.
- **Request-origin truth** — the origin is explicit, and a rerun whose resolved
  origin changed is held for review before dispatch.
- **Persisted-operation continuity** — a drifted or deprecated persisted
  operation blocks the send behind rerun, regenerate, or cancel — never a silent
  raw fallback.
- **History retention** — metadata-only retention stays the safe default;
  storing redacted or full payloads needs an explicit reviewed choice.
- **Auth-source labeling** — the auth scheme and secret source are named without
  exposing the secret.

## How a profile can narrow

A profile that claims more confidence than its proof supports **narrows
automatically** — it drops from *stable* to *preview* instead of overstating
itself. This happens when:

- proof is missing or stale,
- a contract schema is stale or unavailable,
- a request origin changed,
- a persisted operation drifted or was deprecated,
- a collection reopened offline and could not refresh its contract, or
- a profile would otherwise overclaim validation confidence or origin stability.

For example, a collection reopened from an **offline mirror** keeps its portable
definitions but narrows its live-validation claim to *preview*, because offline it
cannot prove the contract is still live. The snapshot is labeled, never shown as
live.

## Trust boundaries

Managed-workspace and browser-companion sends never inherit your desktop-local
trust or naming. Their origins are isolated, and their auth and history carry the
managed or companion identity explicitly.

## Where you see it

Certification state appears on the request profile scorecard, the compatibility
report, release-center promotion, service-health diagnostics, support and export
bundles, and Help/About. Support and export bundles carry certification state with
redaction classes only — never raw URLs, secrets, or payloads.
