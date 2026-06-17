# Saved queries, scope packs, history retention, and signed deep links

This document describes the governance layer that makes saved queries, scope
packs, query history, and search deep links as governed and portable as every
other M5 artifact. Where [`query_artifacts`] owns the row-level durable
artifacts and [`session_ledger`] owns the privacy and export vocabulary, this
layer freezes them into one delivery-grade governance packet.

- Schema: `schemas/search/saved-query-governance.schema.json`
- Packet model: `crates/aureline-search/src/saved_query_governance/mod.rs`
- Desktop consumer: `crates/aureline-shell/src/saved_query_governance/mod.rs`
- Fixtures: `fixtures/search/m5/saved-query-retention/`

## Governed saved-query rows

Each `GovernedSavedQueryRow` binds one canonical `SavedQuery` to its captured
`ScopePackBinding` and local `QueryHistoryEntry`, and adds the
captured-vs-current `ScopeDriftDisclosure` shown on reopen. The bound artifacts
share one scope-binding id and one saved-query id, so the saved-query list, the
history lane, and the share sheet inspect the same objects.

| Field | Meaning |
| --- | --- |
| `saved_query` | The durable saved query, reused verbatim with its privacy, sync, retention, redaction, and migration state. |
| `scope_pack` | The captured scope pack the saved query reopens against. |
| `history_entry` | The local query-history entry linked to the saved query. |
| `scope_drift` | Captured-vs-current scope truth; `silent_semantic_break` is always `false`. |

A saved query **survives reopen, migration, and scope drift without silent
semantic breakage**: a migrated artifact keeps its identity, and a drifted scope
rebinds and reruns visibly instead of silently presenting stale rows as current.
A reopen never claims `current_live_results` without a rerun.

## Signed deep links

A `SignedSearchDeepLink` wraps a canonical `SearchDeepLink` with a tamper-evident
content signature over its disclosed intent, completeness note, scope, freshness,
and return path:

| Field | Meaning |
| --- | --- |
| `intent_summary` | The intent the link reopens — never frozen result certainty. |
| `completeness_note` | Discloses partiality and freshness. |
| `freshness_disclosure` | `live_rerun_required`, `scope_changed_since_capture`, or `empty_because_scope_changed` — never live or frozen certainty. |
| `scope_disclosure` | The scope honesty state the recipient sees. |
| `return_anchor_ref` | The supportable return path focus returns to. |
| `signature_scheme` | `local_content_digest`, `workspace_signed_digest`, or `policy_signed_digest`. |
| `payload_digest` / `signature` | A deterministic content digest and key-scoped signature over the signed fields. |

The signature is a deterministic, verifiable content digest of the disclosure
fields scoped by the signing key. It is **not** a cryptographic identity proof;
it binds the disclosed scope, freshness, and partiality to the link so a
recipient — and the desktop chrome — can detect tampering before reopening. A
deep link reopens *search intent* under the recipient's own permissions: it
never implies live current certainty and never widens access. **Shared intent is
not shared authority.**

## Local-versus-synced retention

The retention matrix is the machine-readable proof that **raw query text stays
local-only by default**. Each row governs one query-material data class:

| Data class | Local default | Synced by default | Leaves the device only |
| --- | --- | --- | --- |
| `raw_query_text` | local-only literal | **no** | under explicit user opt-in with literal consent |
| `query_hash` | local-only hash | no | with an explicit workspace-share basis |
| `parsed_query_ast` | local-only, redaction-safe | no | as the metadata-only form |
| `scope_metadata` | local-only metadata | yes | with repo-provided read-only scope packs |
| `result_refs` | local-only ephemeral | no | inside a bounded, redacted support export |

Any data class that widens past the local default carries an explicit
`widening_basis`; raw query text never syncs by default and only leaves under
`explicit_user_opt_in` with the `explicit_literal_consent` redaction profile.

## Consumer reuse

The `product_ui`, `sync_portability`, and `support_export` consumers each ingest
the packet verbatim, preserve the privacy and sync class and the captured-vs-current
scope truth, reuse the same artifact objects, and **exclude raw query text**.
`SavedQueryGovernancePacket::redact_for_export` materializes the redacted copy a
support bundle ships — hashes, scope metadata, and result refs remain; the local
literal is removed.

## Guardrails enforced (fail-closed)

- Raw query text must stay confined to a local-only artifact (local-only
  privacy, local-only sync, literal-local-only redaction) and never sync or
  export by default.
- A reopen, migration, or scope drift must be disclosed; `silent_semantic_break`
  is always `false`, and a drifted scope must rerun before claiming truth.
- A signed deep link's content signature must verify; the link must preserve a
  return path, disclose completeness, never imply live current certainty, and
  never widen access.
- The retention matrix must govern every data class, keep every local copy
  local-only, and carry an explicit widening basis wherever a class syncs.
- No consumer projection may widen authority or carry raw query text.

[`query_artifacts`]: ../../crates/aureline-search/src/query_artifacts/mod.rs
[`session_ledger`]: ../../crates/aureline-search/src/session_ledger/mod.rs
