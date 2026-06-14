# Persisted operations, drift checks, and safe sends

Some requests don't send raw text — they send a **persisted operation**: a
server-bound id or hash that points at an operation the server already knows.
Aureline treats that binding as a visible fact, not hidden metadata, and never
silently sends raw text in its place when the binding changes.

## What the detail panel shows

For a persisted-operation request you always see:

- the **local name** of the operation,
- the **server-bound id or hash** it resolves to,
- the **contract version** it targets,
- a **breaking-risk note** when something needs attention, and
- an **open-contract** action to inspect the contract.

## Drift classes

A binding falls into one of five classes:

- **Current** — the operation text and the persisted id match. The request sends
  the persisted operation directly.
- **Deprecated** — the id still resolves, but the server marks it deprecated. The
  send is held for review so you can acknowledge it or move to the replacement.
- **Hash drift** — the local operation text changed, so its hash no longer
  matches the persisted id.
- **Id drift** — the server rotated the persisted id, so the saved binding no
  longer resolves.
- **Removed** — the server no longer recognizes the persisted id at all.

Hash drift, id drift, and removal are **material mismatches**: the bound id no
longer resolves.

## What happens on drift

When a binding drifts or is deprecated, the send is **blocked until you review**
the change. The review sheet shows the prior and resolved id/hash and contract
versions, and offers clear choices:

- **Rerun the reviewed binding** — proceed after confirming the operation,
- **Regenerate the persisted id** — re-derive the id/hash from the current text,
- **Cancel** the send, or
- **Reviewed raw downgrade** — explicitly acknowledge sending the raw local text.

For a material mismatch, the **reviewed raw downgrade is the only way** to send
raw text, and it always requires an explicit acknowledgement. The request never
falls back to raw execution on its own.

## Trust and safety

Request history is never widened toward unsafe body or header retention just to
support a drift review or compare. Support exports carry the local name, the
opaque id/hash, the contract version, the drift class, and the review choices —
never raw operation text, bodies, headers, or secrets.
