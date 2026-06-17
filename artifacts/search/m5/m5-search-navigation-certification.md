# M5 Search and Navigation Certification Review

This review packet certifies that the four M5 search/docs/graph depth lanes —
query-session identity, ranking explainability, saved-query privacy, and
navigation continuity — carry **current** evidence. It sits above the
[qualification matrix](../../../docs/search/m5-search-navigation-qualification.md),
which freezes *what* each surface must prove, and answers whether that proof is
current, narrowing the claim fail-closed when freshness or cross-surface parity
slips.

## Evidence

| Evidence | Path |
| --- | --- |
| Rust packet | `crates/aureline-search/src/m5_search_navigation_certification/mod.rs` |
| Boundary schema | `schemas/search/m5-search-navigation-certification.schema.json` |
| Reviewer doc | `docs/search/m5-search-navigation-certification.md` |
| Canonical fixture | `fixtures/search/m5/m5-search-navigation-certification/packet.json` |
| Qualification matrix | `schemas/search/m5-search-navigation-qualification.schema.json` |
| Query-session identity evidence | `schemas/search/query-session-first-consumers.schema.json` |
| Ranking explainability evidence | `schemas/search/ranking-explainability.schema.json` |
| Saved-query privacy evidence | `schemas/search/saved-query-governance.schema.json` |
| Navigation continuity evidence | `schemas/search/navigation-continuity.schema.json` |

## Review Findings

| Area | Result |
| --- | --- |
| Lane coverage | All four search/docs/graph depth lanes carry exactly one certification row and one cross-surface parity audit. |
| Own-proof evidence | Each lane cites its own evidence packet id, schema, doc, artifact, fixture corpus, and record kind; no lane borrows an adjacent lane's proof, and every lane cites a distinct evidence packet. |
| Evidence freshness | Each lane row carries its evidence freshness (`fresh` / `stale` / `missing` / `superseded`) and a recheck deadline; only `fresh` does not force a re-test. |
| Fail-closed derivation | The published certification state is derived strictly from freshness, source-degradation, and parity; a green claim on stale, missing, or superseded evidence cannot validate. |
| Closed state vocabulary | The four-state vocabulary (`certified`, `retest_pending`, `limited`, `unsupported`) is frozen; the strictest narrowing wins when multiple triggers fire. |
| Cross-surface parity | A per-lane audit catches any product, CLI/headless, docs/help, or support-export surface that overclaims a greener state than the lane proves, dropping the lane to `limited`. |
| Downgrade automation | Stale evidence, schema drift, a degraded source lane, a broken parity, missing evidence, or a missing consumer binding can no longer keep a broad search/docs/graph claim green. |
| Shared consumer contract | Help/About, docs/help, support export, and the claim-publication manifest all ingest the same packet id and preserve the same row fields verbatim. |
| Export safety | The certification remains metadata-only and by-reference; raw query text, source bodies, provider payloads, and secrets stay outside this boundary. |

## Current posture

- All four lanes certify on fresh evidence in the canonical index; each cites its
  own checked-in evidence packet and a recheck deadline 30 days out.
- Degraded fixtures demonstrate the fail-closed auto-narrowing the claim depends
  on:
  - `retest_pending_stale_evidence.json` — stale navigation-continuity evidence
    fails the lane closed to `retest_pending` while the other lanes stay
    `certified`.
  - `limited_degraded_source.json` — a degraded ranking-explainability source with
    a support-export surface still projecting `certified` fails the lane closed to
    `limited` and holds the overclaiming surface back.
  - `unsupported_missing_evidence.json` — missing saved-query-privacy evidence
    fails the lane closed to `unsupported`.

## Regenerating this evidence

```sh
cargo run -q -p aureline-search --example dump_m5_search_navigation_certification_packet -- canonical \
  > fixtures/search/m5/m5-search-navigation-certification/packet.json
cargo run -q -p aureline-search --example dump_m5_search_navigation_certification_packet -- retest_pending \
  > fixtures/search/m5/m5-search-navigation-certification/retest_pending_stale_evidence.json
cargo run -q -p aureline-search --example dump_m5_search_navigation_certification_packet -- limited \
  > fixtures/search/m5/m5-search-navigation-certification/limited_degraded_source.json
cargo run -q -p aureline-search --example dump_m5_search_navigation_certification_packet -- unsupported \
  > fixtures/search/m5/m5-search-navigation-certification/unsupported_missing_evidence.json
cargo test -p aureline-search m5_search_navigation_certification
```
