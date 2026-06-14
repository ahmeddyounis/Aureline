# Request-history rows and export-safe compare

## Scope

This document describes the request-history records that upgrade request history
from a convenience replay log into a governed object model. Each history row
keeps the execution timestamp, the named environment, the origin scope (local,
remote, container, managed-workspace, or browser-companion), the origin drift
state, the status/result class, the aggregate assertion state, the retention
mode, and the redaction posture inspectable across the request-history panel, the
browser-companion and managed-workspace history surfaces, the compare view, the
retention settings, CLI/headless output, support export, and Help/About surfaces.

Metadata-only retention is the safe default. Storing redacted-replayable or full
payloads, results, or headers requires an explicit, reviewed retention selection
with a declared redaction posture, never a convenience toggle. Compare stays
export-safe: it operates on what was already retained safely and never widens
retention toward unsafe body or header capture, and history rows and export
packets never drop origin or environment identity.

The records reuse the canonical frozen vocabulary (`contract_kind`,
`request_origin_kind`, `request_origin_drift_state`, `retention_mode`) and
reference the frozen API-collection matrix as a verified upstream packet, the
named-environment vocabulary from the request-list views lane, the assertion
outcome vocabulary from the response-viewer lane, and the export redaction
vocabulary from the composer history and redaction-safe export lane, rather than
minting a local synonym set.

## Truth sources

- Implementation: `crates/aureline-api/src/implement_request_history_rows_with_environment_origin_scope_assertion_state_redaction_or_retention_mode_and_export_safe_compare/mod.rs`
- Schema: `schemas/data/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.schema.json`
- Checked-in packet: `artifacts/data/m5/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.json`
- Fixtures: `fixtures/data/m5/implement_request_history_rows_with_environment_origin_scope_assertion_state_redaction_or_retention_mode_and_export_safe_compare/`
- Upstream matrix: `artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json`

## Locked vocabulary

| Term | Family | Meaning |
|---|---|---|
| `success`, `redirected`, `client_error`, `server_error`, `transport_error`, `blocked`, `timed_out`, `cancelled` | result class | The status/result class a history row can carry. |
| `no_assertions`, `all_passed`, `mixed_results`, `any_failed`, `not_evaluated` | assertion state | The aggregate assertion state; maps onto the response-viewer assertion outcome vocabulary. |
| `metadata_only`, `redacted_replayable`, `opt_in_full_capture` | retention mode | The history retention modes; `metadata_only` is the safe default. |
| `redact_all`, `redact_secrets`, `no_redaction_local_only` | redaction posture | How stored content is redacted; `no_redaction_local_only` is never exported. |
| `status_and_timing`, `redacted_bodies`, `assertion_results`, `header_metadata` | compare basis | What a compare diffs across two history rows. |
| `full_redaction`, `metadata_only`, `safe_preview`, `unredacted_local_only` | export redaction | The export redaction class for a compare or export. |

## Consumer surfaces

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Request-history panel | stable | stable | Shows timestamp, environment, origin scope, result class, assertion state, retention mode, and compare/export actions per row. |
| Browser-companion request history | stable | stable | Same columns, with browser-companion origin isolating desktop-local trust. |
| Managed-workspace request history | stable | stable | Same columns, with managed origin and environment isolating desktop-local trust. |
| History compare view | stable | stable | Diffs two history rows on already-retained safe data without widening retention. |
| History retention settings | stable | stable | Keeps metadata-only as the default and requires an explicit reviewed selection before storing more. |
| CLI and headless request-history output | stable | stable | Prints the row columns without raw bodies, headers, or secrets. |
| Support export request-history truth | stable | stable | Carries the row columns with redaction-safe content, never raw bodies, headers, or secrets. |
| Help and About request-history contract | stable | stable | Describes the columns, the safe default, the reviewed upgrade, and export-safe compare. |

## History, retention, and compare rules

- Every history row keeps the timestamp, environment, origin scope, origin drift
  state, result class, assertion state, and retention mode visible; origin and
  environment identity are never dropped from a row or an export packet.
- The aggregate assertion state agrees with the row's pass/fail counts, so a
  failing or mixed run never reads as a clean pass behind a green status.
- `metadata_only` retention is the safe default and pairs with the `redact_all`
  posture; `redacted_replayable` pairs with `redact_secrets`; `opt_in_full_capture`
  pairs with `redact_secrets` or `no_redaction_local_only`.
- Any retention beyond safe metadata-only requires an explicit, reviewed
  retention selection; full-capture storage of bodies/headers is only reachable
  through an opt-in selection and is never the path of least resistance.
- Managed-workspace and browser-companion origins isolate desktop-local trust and
  never inherit local naming assumptions.
- Compare operates on already-safe retention and never forces unsafe body/header
  capture; an export-safe compare carries no raw secrets and is never the
  unredacted-local-only class.
- Exports retain origin and environment identity, never carry raw secret values,
  and only carry a raw body under the unredacted-local-only class, which is never
  support-bundle safe.
- The history lane references the frozen API-collection matrix as a verified
  upstream packet; the matrix remains the source of origin, retention, and
  contract truth.
