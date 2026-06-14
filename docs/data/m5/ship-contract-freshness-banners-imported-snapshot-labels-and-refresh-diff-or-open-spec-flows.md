# Contract freshness banners, imported-snapshot labels, and refresh/diff/open-spec flows

## Scope

This document describes the schema/contract freshness banners and the refresh,
diff, and open-spec flows that render the frozen API-collection matrix contract
truth as a real consumer wherever request validation or completion depends on a
contract snapshot. Each banner keeps the source service, snapshot date, freshness
state, mirror/offline note, and refresh/open-details actions inspectable so a
GraphQL or other contract-linked request never lets a stale, cached, or imported
snapshot masquerade as a live contract behind a green send button.

The banners reuse the canonical matrix vocabulary (`contract_kind`,
`contract_source_class`, `contract_freshness_state`, `offline_mirror_behavior`,
`request_origin_kind`, `retention_mode`) and the composer export-redaction
vocabulary (`export_redaction`) rather than minting local synonyms, and they add
banner-specific vocabularies for severity class, banner action, refresh mode, and
open-spec target.

## Truth sources

- Implementation: `crates/aureline-api/src/ship_contract_freshness_banners_imported_snapshot_labels_and_refresh_diff_or_open_spec_flows/mod.rs`
- Schema: `schemas/data/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.schema.json`
- Checked-in packet: `artifacts/data/m5/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.json`
- Fixtures: `fixtures/data/m5/ship_contract_freshness_banners_imported_snapshot_labels_and_refresh_diff_or_open_spec_flows/`
- Upstream matrix: `artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json`

## Locked vocabulary

| Term | Family | Meaning |
|---|---|---|
| `live_contract`, `cached_schema`, `schema_stale`, `imported_snapshot`, `contract_unavailable` | freshness state | Contract snapshot freshness; stale and unavailable narrow any live claim. |
| `informational`, `advisory`, `stale_warning`, `imported_notice`, `unavailable_block` | banner severity | Visual severity derived one-to-one from the freshness state. |
| `refresh`, `diff`, `open_spec`, `open_details` | banner action | Actions a banner can offer; every banner offers at least `refresh` and `open_details`. |
| `fetch_live`, `revalidate_cache`, `reimport_snapshot` | refresh mode | How a refresh flow re-resolves the contract. |
| `inline_schema_view`, `external_spec_doc`, `provider_console` | open-spec target | Where an open-spec flow takes the user. |

## Consumer surfaces

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Request composer freshness banner | stable | stable | The composer banner shows source, snapshot date, freshness, and mirror note with refresh and open-details before a request is sent. |
| Completion and validation provider banner | stable | stable | Completion depends on the contract snapshot, so the provider attaches a banner that narrows confidence on cached, stale, or imported schema. |
| Browser companion request banner | stable | stable | Companion requests can drift from desktop-local state, so freshness is never hidden and managed/companion origins never inherit local trust. |
| CLI and headless freshness banner | stable | stable | Headless output prints the freshness line so a blocked or stale contract never reads as a clean send. |
| Support export freshness banner | stable | stable | Support exports carry banner truth with redaction-safe postures and the snapshot identity diff and open-spec replay. |
| Help and About freshness contract | stable | stable | Help/About describe the freshness vocabulary, imported-snapshot label, and flow contract. |

## Narrowing and honesty rules

- A banner's `severity` must be the canonical mapping of its
  `contract_freshness_state`; the two never disagree across surfaces.
- A banner whose freshness is `schema_stale` or `contract_unavailable` narrows
  any live claim and never appears equivalent to a live contract.
- An `imported_snapshot` banner is always explicitly labeled and never reads as
  live truth.
- Freshness is never hidden from browser-companion or managed-request surfaces;
  managed and companion origins never inherit desktop-local trust or naming.
- Refresh flows preserve local request context and local edits, require an
  acknowledgement before retargeting an origin, and never silently fall back to
  raw execution when contract risk changed.
- Diff flows preserve version and snapshot identity, keep support-export parity,
  and never force unsafe body/header retention or drop local request context to
  support compare UX.
- Open-spec flows preserve snapshot identity and support-export parity and never
  drop local request context.
- The banners reference the frozen API-collection matrix as a verified upstream
  packet; the matrix remains the source of contract source, freshness, and origin
  truth.
