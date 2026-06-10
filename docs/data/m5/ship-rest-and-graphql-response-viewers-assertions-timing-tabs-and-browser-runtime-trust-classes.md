# REST and GraphQL response viewers, assertions, timing tabs, and browser-runtime trust classes

## Scope

This document describes the canonical M5 qualification packet for REST and GraphQL response viewers, assertion panels, timing breakdown tabs, and browser-runtime trust classes in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes/mod.rs`
- Schema: `schemas/data/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.schema.json`
- Checked-in packet: `artifacts/data/m5/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.json`
- Fixtures: `fixtures/data/m5/ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| REST response viewer | stable | stable | Shows preview class, raw/structured views, assertion results, timing, and export redaction. |
| GraphQL response viewer | stable | stable | Shows preview class, raw/structured views, assertion results, timing, and export redaction. |
| Assertion panel | stable | stable | Shows pass/fail/error/skipped outcomes with request/response identity and export safety. |
| Timing tab | preview | preview | Shows phase breakdown but is still below stable pending full UI parity. |
| Browser-runtime trust panel | stable | stable | Shows trust class, surface kind, mutation review, and cross-origin disclosure. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.

## Redaction and privacy

- Response viewers never grant ambient execution rights to HTML/JS payloads.
- Response viewers enforce a body size limit (10 MB default).
- Assertion results are visible before export and support-bundle safe by default.
- Browser-runtime trust classes are visible in UI and exports.
- Mutating browser-runtime actions require explicit review.

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
