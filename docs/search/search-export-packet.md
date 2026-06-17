# Search export packets, redaction, replay safety, and literal-query consent

This document describes the governance layer that makes search export and
support replay first-class, privacy-aware artifacts for the M5 search and
incident support lanes. Search and incident support flows routinely carry
sensitive query text, customer identifiers, hostnames, and policy terms, so
supportability must not quietly broaden retention of raw query text or result
bodies. Where [`session_ledger`] owns the per-export `SearchExportPacket` and the
privacy vocabulary, and [`saved_query_governance`] freezes the durable
saved-query artifacts, this layer freezes the export and replay posture into one
delivery-grade packet.

- Schema: `schemas/search/search-export-packet.schema.json`
- Packet model: `crates/aureline-search/src/search_export_governance/mod.rs`
- Per-export packet model: `crates/aureline-search/src/session_ledger/mod.rs`
- Fixtures: `fixtures/search/m5/support-export/`

## Governed export rows

Each `SearchExportRow` binds one canonical `SearchExportPacket` — reused verbatim
— to the export class it ships under and the literal-query consent gate that
governs it. The embedded packet preserves the query-session ref, the selected and
included result refs, the loaded/hidden count summary, the redaction mode, the
snapshot truth, and the evidence refs, so replay and audit read the same object
the live surface produced.

| Field | Meaning |
| --- | --- |
| `export_class` | The trust tier the packet ships under: local replay, support bundle, incident packet, or managed analytics. |
| `export_packet` | The canonical per-export `SearchExportPacket`, reused verbatim with its refs, counts, redaction mode, snapshot truth, and evidence refs. |
| `literal_query_consent` | The consent posture — `metadata_only` by default, `query_text_elevated` when the user opted into literal query text. |
| `literal_query_text_included` | True only when the embedded packet retains a literal; gated behind elevated consent and a permitting class. |
| `replay_safety` | The captured-vs-current truth proving the packet preserves intent and never claims live results. |

## Export classes and redaction defaults

| Export class | Leaves device | Default redaction | Literal query text |
| --- | --- | --- | --- |
| `local_replay` | No | Raw query local-only | Retained only under `query_text_elevated` consent |
| `support_bundle` | Yes | Query hash only | Never |
| `incident_packet` | Yes | Query hash only | Never |
| `managed_analytics` | Yes | Query material omitted | Never (no literal or hash) |

Support bundles and incident packets explain what search ran, what was selected,
and what was omitted using query hashes, scope summaries, result refs, omission
counts, and reason summaries. The literal query string is confined to a local
replay packet that stays on the device; it is never bundled into anything that
leaves the device, and managed analytics carries no query material at all.

## Literal-query consent gate

Literal query text requires explicit higher-trust consent
(`query_text_elevated`) and a class that permits retention. The gate is
fail-closed:

- `literal_query_text_included` must match the embedded packet's query text.
- Inclusion requires `query_text_elevated` consent and
  `export_class.permits_literal_query_text()`.
- Nothing that leaves the device may carry the literal, and managed analytics
  carries no literal or hash material.
- The embedded packet is independently re-checked for export safety, so a
  non-local destination can never carry raw query text.

## Replay safety

Every packet is replay-safe: it is a captured snapshot or a disclosed scope
drift, never a live rerun, and never claims live current results. A drifted scope
(`scope_changed_since_capture`) requires a rerun before any current-truth claim.
Replay/debug tooling and support replay consume these packets — not raw UI state
or screenshots — so a stale capture never masquerades as a live result.

## Consumers

The desktop shell, CLI/headless inspect, support export, and managed analytics
paths each ingest the one packet id and preserve the redaction mode, the count
and omission disclosure, and the replay safety. No consumer widens authority,
drops disclosure, or carries literal query text, so the same privacy rules hold
across every path.

## Redacted export

`SearchExportGovernancePacket::redact_for_export` removes all literal query text
(dropping the local replay packet's query-text mode to `hash_only`) while
preserving hashes, scope metadata, result refs, counts, omission disclosure, and
the replay-safety truth. `support_export` wraps that redacted copy as the bundle
a support case or incident packet ships.

[`session_ledger`]: ../../crates/aureline-search/src/session_ledger/mod.rs
[`saved_query_governance`]: ./saved-query-governance.md
