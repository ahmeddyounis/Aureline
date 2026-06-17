# Review artifact — saved-query governance, retention, and signed deep links

Packet id: `search.m5.saved_query_governance.v1`

This artifact is the reviewer-facing summary of the saved-query governance layer
for the M5 search and navigation lane. It is produced from the seeded packet and
governs query material: raw query text stays local-only by default, and sync,
share, and support export carry redacted metadata only.

## What this lane delivers

- Governed saved-query rows that bind the canonical `SavedQuery`,
  `ScopePackBinding`, and `QueryHistoryEntry` verbatim, with a captured-vs-current
  `ScopeDriftDisclosure` so a reopen, migration, or scope drift is always
  disclosed, never a silent semantic break.
- Signed search deep links that wrap the canonical `SearchDeepLink` with a
  tamper-evident content signature over the disclosed intent, completeness note,
  scope, freshness, and return path. The link reopens intent under the
  recipient's own permissions; it never implies live current certainty and never
  widens access.
- A local-versus-synced retention matrix that proves raw query text never syncs
  by default and leaves the device only under explicit user opt-in with literal
  consent, while hashes, scope metadata, parsed grammar, and result refs follow
  their own per-data-class posture.
- A desktop saved-query/history/deep-link projection
  (`crates/aureline-shell/src/saved_query_governance/mod.rs`) that reuses the
  governed artifacts, renders the captured-vs-current scope truth, refuses to
  reopen a tampered or authority-widening link, and never surfaces raw query text.

## Acceptance evidence

| Acceptance criterion | Evidence |
| --- | --- |
| Saved queries and query history survive reopen, migration, and scope drift without silent semantic breakage | Every row sets `survives_reopen/migration/scope_drift = true` and `scope_drift.silent_semantic_break = false`; one row carries `migrated_from_previous_version` and one discloses a `current_scope_changed_rebind_required` drift that reruns before claiming truth. |
| Search deep links disclose scope, freshness, and partiality and preserve a supportable return path | Each signed link carries a non-empty `return_anchor_ref`, a disclosed `completeness_note`, a `freshness_disclosure` that reopens intent (never live/frozen certainty), and a content signature that verifies the disclosure. |
| Raw query text is not synced, exported, or retained beyond policy by default | Only the local-only-private row keeps a literal, confined to local-only privacy/sync/redaction; the `raw_query_text` retention row is `synced_by_default = false`; and the redacted support export carries no raw query text. |

## Guardrails enforced (fail-closed)

- Raw query text must stay confined to a local-only artifact and never sync or
  export by default; any widening carries an explicit basis.
- A reopen, migration, or scope drift must be disclosed; `silent_semantic_break`
  is always `false`.
- A signed deep link's content signature must verify; tampering with the disclosed
  intent, scope, freshness, or return path breaks the signature.
- A deep link must never imply live current certainty and never widen access —
  shared intent is not shared authority.
- No consumer projection may widen authority or carry raw query text.

## Redacted export variant

`redacted_export.json` proves that the support/sync export copy carries no raw
query text: the local-only literal is removed and its query text mode drops to
`hash_only`, while hashes, scope metadata, captured-vs-current scope truth, and
the signed deep links are preserved unchanged.

## Sources

- Contract doc: `docs/search/saved-query-governance.md`
- Schema: `schemas/search/saved-query-governance.schema.json`
- Fixtures: `fixtures/search/m5/saved-query-retention/`
- Model + tests: `crates/aureline-search/src/saved_query_governance/`
- Desktop consumer + tests: `crates/aureline-shell/src/saved_query_governance/`
