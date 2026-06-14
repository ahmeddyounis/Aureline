# Persisted-operation detail, drift review, and no-unsafe-fallback send rules

## Scope

This document describes the persisted-operation detail, hash/id drift-check,
contract-version-review, and no-unsafe-fallback send-rule records that make a
request's persisted-operation binding a first-class fact rather than hidden
metadata. Each detail row keeps the local operation name, the opaque remote id
or hash, the contract version, the breaking-risk note, the binding/drift class,
and the open-contract action inspectable across the detail panel, request
composer, CLI/headless output, support export, and Help/About surfaces. When a
persisted-operation id or hash drifts, is deprecated, or is removed, the
companion drift-review sheets surface clear rerun, regenerate, and cancel choices
and block the send instead of silently falling back to raw local-text execution.

The records reuse the canonical matrix vocabulary (`contract_kind`,
`persisted_operation_binding_state`, `retention_mode`) and reference the frozen
API-collection matrix as a verified upstream packet rather than minting a local
synonym set. The finer `persisted_operation_drift_class` adds the
hash-versus-id-versus-deprecation distinction this lane requires while mapping
one-to-one onto the frozen binding states.

## Truth sources

- Implementation: `crates/aureline-api/src/add_persisted_operation_detail_hash_or_id_drift_checks_contract_version_review_and_no_unsafe_fallback_send_rules/mod.rs`
- Schema: `schemas/data/add-persisted-operation-detail-hash-or-id-drift-checks-contract-version-review-and-no-unsafe-fallback-send-rules.schema.json`
- Checked-in packet: `artifacts/data/m5/add-persisted-operation-detail-hash-or-id-drift-checks-contract-version-review-and-no-unsafe-fallback-send-rules.json`
- Fixtures: `fixtures/data/m5/add_persisted_operation_detail_hash_or_id_drift_checks_contract_version_review_and_no_unsafe_fallback_send_rules/`
- Upstream matrix: `artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json`

## Locked vocabulary

| Term | Family | Meaning |
|---|---|---|
| `current`, `deprecated`, `hash_drift`, `id_drift`, `removed` | drift class | The five binding/drift classes a persisted-operation detail can carry. |
| `bound_current`, `persisted_operation_drift` | binding state | The frozen matrix states the drift classes resolve under; `current`/`deprecated` map to `bound_current`, the rest to `persisted_operation_drift`. |
| `send_persisted_bound`, `block_pending_review` | send decision | Send the bound persisted operation, or block until the drift/deprecation is reviewed. |
| `rerun_reviewed_binding`, `regenerate_persisted_id`, `cancel`, `reviewed_raw_downgrade` | review choice | The choices a drift-review sheet offers; only the reviewed raw downgrade can produce raw execution. |

## Consumer surfaces

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Persisted-operation detail panel | stable | stable | Shows local name, remote id/hash, contract version, breaking-risk note, drift state, and open-contract action. |
| Persisted-operation drift review sheet | stable | stable | Shows prior/resolved id/hash and contract versions and offers rerun, regenerate, and cancel choices, blocking the send on a material mismatch. |
| Request composer send rule | stable | stable | Sends a current binding and blocks a drifted or deprecated binding until reviewed instead of sending raw text. |
| CLI and headless persisted-operation line | stable | stable | Prints the detail fields and refuses a material mismatch without an explicit reviewed-downgrade acknowledgement. |
| Support export persisted-operation truth | stable | stable | Carries detail and review-choice truth with metadata-only retention, never raw operation text, bodies, headers, or secrets. |
| Help and About persisted-operation contract | stable | stable | Describes the detail fields, drift classes, contract-version review, and the no-unsafe-fallback send contract. |

## Detail and send rules

- Every detail keeps the local name, opaque remote id/hash, contract version,
  drift state, and open-contract action visible; server-bound identity is never
  hidden when a request depends on it.
- A detail's `drift_class` always resolves under its canonical
  `binding_state`; the finer class never diverges from the frozen matrix states.
- A `current` binding sends the persisted operation by its server-bound id
  (`send_persisted_bound`) without review.
- A `deprecated`, `hash_drift`, `id_drift`, or `removed` binding requires review,
  shows a breaking-risk note, and the enforced send decision is
  `block_pending_review`.
- A material mismatch (`hash_drift`, `id_drift`, `removed`) blocks the send until
  reviewed and never silently falls back to raw local-text execution.
- A raw send after a material mismatch is only reachable through an explicit,
  acknowledged `reviewed_raw_downgrade` choice; no other review choice produces
  raw execution.
- Drift-review and compare UX never widens request-history retention toward
  unsafe body or header capture; history stays metadata-only or
  redacted-replayable.
- The detail lane references the frozen API-collection matrix as a verified
  upstream packet; the matrix remains the source of binding-state, contract, and
  no-silent-raw-fallback truth.
