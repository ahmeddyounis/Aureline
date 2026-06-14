# Request history, retention, and compare

Aureline keeps a **request history** so you can see, diff, and re-run earlier
requests. History is a governed record, not a hidden log: every row shows the
same facts, and storing more than safe metadata is always an explicit, reviewed
choice.

## What each history row shows

For every past request you always see:

- **when** it ran (the timestamp),
- the **environment** it ran in (local, development, staging, production, or
  managed),
- the **origin scope** — local, remote, a container service, a managed
  workspace, or the browser companion — and whether that origin **changed**
  since the last run,
- the **result class** (success, redirect, client error, server error, transport
  error, blocked, timed out, or cancelled),
- the **assertion state** (all passed, mixed, any failed, not evaluated, or no
  assertions),
- the **retention mode** for the row, and
- **compare** and **export** actions.

A failing or mixed assertion result is always shown — it is never hidden behind a
green status.

## How much is kept (retention)

By default, history keeps **metadata only** — method, target, status, timing, and
the facts above. Bodies, headers, and full results are **not** stored unless you
ask for them.

To keep more, you make an explicit, reviewed **retention selection** with a
redaction posture:

- **Metadata only** — the safe default; nothing sensitive is stored.
- **Redacted replayable** — bodies are kept for replay with secrets and sensitive
  fields redacted.
- **Full capture (opt-in)** — full bodies and headers are kept; this is only
  reached through an explicit reviewed selection, and you choose whether secrets
  are redacted or the capture stays **local-only** and is never exported.

Full capture is never the path of least resistance: history is never widened to
unsafe body or header retention on its own.

## Comparing two runs

The **compare view** diffs two history rows — on status and timing, on redacted
bodies, on assertion results, or on header metadata. Compare always works on what
was **already retained safely**. It never widens retention just to enable the
diff, never carries raw secrets, and always keeps each run's origin and
environment identity.

## Exporting history

Exports keep the row's origin and environment identity and never include raw
secret values. A fully redacted, metadata-only, or safe-preview export is safe
for support bundles. An **unredacted local-only** export can carry a raw body for
local diagnosis, but it is never shared in a support bundle.

## Trust and safety

Managed-workspace and browser-companion requests keep their own origin and never
inherit desktop-local trust or naming assumptions, so a localhost name and a
managed or companion target are never confused in your history.
