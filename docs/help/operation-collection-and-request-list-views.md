# Operation collections and request lists

Aureline groups API requests into **operation collections** and renders them as a
keyboard-navigable tree with a request list. Every row stays explicit so large
workspaces remain legible and reviewable.

## What each request row shows

- **Protocol class** — `rest`, `graphql`, `grpc`, or `websocket`.
- **Environment** — the named environment (`local`, `development`, `staging`,
  `production`, `managed`) plus an explicit resolved-target label. Environment
  identity is never reduced to a friendly name alone.
- **Contract / source badge** — where the contract came from (live, cached,
  imported snapshot, plugin-provided, or unavailable) and its freshness.
- **Last-run state** — `never_run`, `succeeded`, `failed`,
  `blocked_pending_review`, or `stale_needs_resend`. Stale schemas and
  persisted-operation drift never hide behind a green `succeeded`.
- **Retention mode** — how history is kept (text-first, metadata-only,
  redacted-replayable, or opt-in full capture).
- **Provenance** — local-only history, imported snapshot, provider-linked
  contract, or managed/shared artifact.
- **Actions** — open detail, inspect, and export. Exports are redaction-safe and
  never carry raw bodies, headers, or secrets.

## Saved views

Saved views can be **private** (local to your desktop) or **workspace-shared**.
Their filters are stored as reviewable text, never opaque binary state. Shared
views never inherit desktop-local trust.

## Trust and safety

Managed and shared requests resolve to managed environments that never inherit
desktop-local trust or naming. Collections and saved views stay text-first and
diffable so they review cleanly in version control.
