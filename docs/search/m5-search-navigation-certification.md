# Search and navigation certification

The [search and navigation qualification](m5-search-navigation-qualification.md)
index freezes **what** every claimed M5 search surface must prove. This
certification index answers the next question — **is that proof current?** — and
narrows the claim automatically the moment it is not.

Certification is organized by the four search/docs/graph **depth lanes**, each
of which certifies a distinct claim:

| Lane | Certified claim | Evidence packet |
| --- | --- | --- |
| `query_session_identity` | Durable query session and stable result identity across virtualization, preview, and reopen. | `search.m5.query_session_first_consumers.v1` |
| `ranking_explainability` | Inspectable ranking reasons; withheld-latency, policy-hidden, and partial-index candidates stay visible. | `search.m5.ranking_explainability.v1` |
| `saved_query_privacy` | Saved queries, scope packs, history, and signed deep links keep raw query text local-only by default. | `search.m5.saved_query_governance.v1` |
| `navigation_continuity` | Breadcrumb, outline, bookmark, history, and peek continuity bind to canonical anchors with visible drift states. | `search.m5.navigation_continuity.v1` |

Each lane cites its **own** evidence packet, boundary schema, reviewer doc,
review artifact, fixture corpus, and record kind — a lane may never borrow an
adjacent lane's proof. The contract and downgrade automation are owned by the
`aureline-search` crate (`m5_search_navigation_certification`). The canonical
index is checked in at
`fixtures/search/m5/m5-search-navigation-certification/packet.json` and validated
against `schemas/search/m5-search-navigation-certification.schema.json`.

## Evidence freshness

Each lane row carries the freshness of its evidence and a recheck deadline. Lane
evidence must be regenerated and re-certified within
**30 days**; once it falls outside the window the lane fails closed
automatically.

| Freshness | Requires re-test? | Meaning |
| --- | --- | --- |
| `fresh` | No | Regenerated within the recheck window and validates against the lane's current schema. |
| `stale` | Yes | Older than the recheck window; the lane fails closed to `retest_pending`. |
| `superseded` | Yes | Validates an older schema version than the lane now publishes; the lane fails closed to `retest_pending`. |
| `missing` | Yes | Absent or unreadable; the lane fails closed to `unsupported`. |

## Fail-closed certification states

| State | Meaning |
| --- | --- |
| `certified` | Fresh evidence proves the lane claim with every claim surface in parity. |
| `retest_pending` | Evidence is stale or schema-superseded; the claim is held pending a re-test. |
| `limited` | A degraded source lane or a broken cross-surface parity narrows the lane to a partial claim. |
| `unsupported` | Evidence is missing, or a consumer binding is broken; the broad claim is not currently sustainable. |

A lane stays `certified` only with `fresh` evidence, a non-degraded source state,
and every claim surface (product, CLI/headless, docs/help, support export) in
parity. When more than one downgrade condition fires, the strictest narrowing
wins.

## Auto-narrowing

| Trigger | Effect |
| --- | --- |
| `evidence_stale` | Evidence is older than the recheck window; the lane fails closed to `retest_pending`. |
| `schema_version_drift` | Evidence validates an older schema version than the lane publishes; the lane fails closed to `retest_pending`. |
| `degraded_source_state` | The source lane packet is itself degraded; the lane fails closed to `limited`. |
| `surface_parity_break` | A claim surface stops projecting the lane's certified truth; the lane fails closed to `limited` and the overclaiming surface is held back. |
| `evidence_missing` | Evidence is absent or unreadable; the lane fails closed to `unsupported`. |
| `consumer_binding_missing` | A consumer stops ingesting the certification by reference; the broad claim fails closed to `unsupported`. |

## Cross-surface parity

Every lane carries a parity audit over the four claim surfaces — product,
CLI/headless, docs/help, and support export. An **in-parity** surface projects
the lane's published certification state; an **out-of-parity** surface is one
still projecting a greener state than the lane proves. A single overclaiming
surface drops the whole lane to `limited`, so release and public-truth surfaces
cannot keep a broader search/docs/graph claim green when one surface drifts.

## One index, every consumer

Help/About, docs/help search, support export, and the claim-publication manifest
all ingest this one packet by reference and preserve the lane row ids, lane
tokens, certification state, evidence freshness, recheck deadline, stale-proof
tokens, and downgrade-rule ids verbatim. None of them maintains a parallel
certification badge, so a stale, limited, or unsupported lane narrows everywhere
at once.

## Regenerating the evidence

```sh
cargo run -q -p aureline-search --example dump_m5_search_navigation_certification_packet -- canonical \
  > fixtures/search/m5/m5-search-navigation-certification/packet.json
cargo test -p aureline-search m5_search_navigation_certification
```
