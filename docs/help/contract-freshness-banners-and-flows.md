# Contract freshness banners

Whenever request validation or completion depends on a contract — a GraphQL
schema, an OpenAPI description, or a plugin-provided contract — Aureline shows a
**freshness banner** so you always know whether you are working against live
truth or a snapshot.

## What each banner shows

- **Source service** — the named service the contract came from.
- **Snapshot date** — when the contract you are using was captured.
- **Freshness** — `live_contract`, `cached_schema`, `schema_stale`,
  `imported_snapshot`, or `contract_unavailable`. Stale and unavailable
  contracts never look like a live contract.
- **Mirror / offline note** — whether an offline mirror is kept and what happens
  offline.
- **Actions** — every banner offers **Refresh** and **Open details**. Banners
  backed by a snapshot also offer **Diff** and **Open spec**.

Imported snapshots are always labeled as snapshots. A stale schema or an
unavailable contract is never hidden behind a green send button.

## Refresh, diff, and open-spec

- **Refresh** re-resolves the contract — fetch live, revalidate the cache, or
  re-import a snapshot — **without dropping your in-progress request**. If the
  refresh would retarget the origin, you are asked to confirm first. A failed
  refresh never silently falls back to running the raw request.
- **Diff** compares two snapshots (for example, your stale snapshot against the
  live schema). Both version labels and snapshot identities are preserved, and
  comparing never widens what request history retains.
- **Open spec** opens the contract — inline, as an external document, or in the
  provider's console — at the exact snapshot you are using.

## Trust and safety

Browser-companion and managed requests can drift from your desktop-local state,
so their freshness is always shown and they never inherit desktop-local trust or
naming. Diff and open-spec exports are redaction-safe and carry the snapshot
identity, never raw bodies, headers, or secrets.
