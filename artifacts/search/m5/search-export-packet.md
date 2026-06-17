# Review artifact — search export packets, redaction, replay, and consent

Packet id: `search.m5.search_export_governance.v1`

This artifact is the reviewer-facing summary of the search-export governance
layer for the M5 search and incident support lanes. It is produced from the
seeded packet and makes search export and support replay first-class,
privacy-aware artifacts rather than screenshots, copied text, or debug-only
logs: support exports and incident packets default to hashes, scope summaries,
result refs, omission counts, and reason summaries, and literal query text
requires explicit higher-trust consent.

## What this lane delivers

- Governed export rows that bind the canonical `SearchExportPacket` verbatim —
  its query-session ref, selected and included result refs, loaded/hidden counts,
  redaction mode, snapshot truth, and evidence refs — to the export class it ships
  under (local replay, support bundle, incident packet, managed analytics) and the
  literal-query consent gate that governs it.
- A literal-query consent gate that confines literal query text to a consented,
  local-only replay packet. Support bundles and incident packets are hash-only by
  default, managed analytics carries no query material at all, and nothing that
  leaves the device ever carries the literal.
- A replay-safety disclosure per packet proving it is a captured snapshot or a
  disclosed scope drift — never a live rerun — that preserves search intent and
  provenance and reruns before claiming current truth.
- Consumer projections proving the desktop shell, CLI/headless inspect, support
  export, and managed analytics paths read the same export packets under the same
  privacy rules — preserving redaction mode, count and omission disclosure, and
  replay safety — instead of raw UI state or screenshots.

## Acceptance evidence

| Acceptance criterion | Evidence |
| --- | --- |
| Support exports and incident packets can explain what search ran, what was selected, and what was omitted without storing literal query text by default | Every row that leaves the device sets `literal_query_text_included = false` and carries no `query_text`, while preserving the query-session ref, included result refs, count summary, omission flags, and evidence refs; the support-bundle row discloses `omitted_result_count` and `hidden_by_current_scope`. |
| Replay-safe packets preserve intent and provenance without claiming live current results | Each packet's `replay_safety` sets `claims_live_current_results = false`, `preserves_intent_and_provenance = true`, `result_semantics ≠ current_live_results`, and `snapshot_truth ≠ live_rerun`; the incident row discloses a `scope_changed_since_capture` drift that requires a rerun. |
| Search privacy rules stay consistent across desktop, CLI, support export, and managed analytics paths | All four consumer projections ingest the one packet id and set `preserves_redaction_mode`, `preserves_count_and_omission_disclosure`, `preserves_replay_safety`, `reuses_same_export_packets`, `literal_query_text_excluded`, and `ambient_authority_excluded` to true. |
| Support-bundle inclusion of query text is bound to explicit consent gates and higher-trust export classes | Literal text appears only on the local-replay row under `query_text_elevated` consent; `permits_literal_query_text` is false for every class that leaves the device, and `managed_analytics` carries neither literal nor hash material. |

## Guardrails enforced (fail-closed)

- Literal query text must stay confined to a consented local-only replay packet
  and never leave the device; the embedded packet is independently re-checked for
  export safety.
- Literal inclusion requires explicit `query_text_elevated` consent and a class
  that permits it; managed analytics carries no query material at all.
- Every packet must be a captured snapshot or a disclosed drift, never a live
  rerun, and never claim live current results; a drift requires a rerun first.
- Partial or omitted packets must preserve their omitted/truncated flags, and
  every packet must carry evidence refs and the query-session ref for audit.
- No consumer projection may widen authority, drop disclosure, or carry literal
  query text.

## Redacted export variant

`redacted_export.json` proves that the support/incident export copy carries no
literal query text: the local-only literal is removed and its query-text mode
drops to `hash_only`, while hashes, scope metadata, result refs, counts, omission
disclosure, and the replay-safety truth are preserved unchanged.

## Sources

- Contract doc: `docs/search/search-export-packet.md`
- Schema: `schemas/search/search-export-packet.schema.json`
- Fixtures: `fixtures/search/m5/support-export/`
- Model + tests: `crates/aureline-search/src/search_export_governance/`
- Per-export packet model: `crates/aureline-search/src/session_ledger/`
