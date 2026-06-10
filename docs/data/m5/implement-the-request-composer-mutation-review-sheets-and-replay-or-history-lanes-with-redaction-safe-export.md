# Request composer, mutation-review sheets, replay and history lanes, and redaction-safe export

## Scope

This document describes the canonical M5 qualification packet for request composers, mutation-review sheets, history lanes, replay configurations, and redaction-safe exports in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export/mod.rs`
- Schema: `schemas/data/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.schema.json`
- Checked-in packet: `artifacts/data/m5/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.json`
- Fixtures: `fixtures/data/m5/implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Request composer | stable | stable | Shows method, target, body mode, auth mode, and variable source before send. |
| Mutation-review sheet | stable | stable | Shows target, side-effect class, auth scope, confirmation requirement, and replay consequences before send. |
| History lane | preview | preview | Local-first and redactable but does not yet show full auth-scope detail in preview. |
| Replay lane | stable | stable | Discloses exact rerun, current context, review-only, and blocked modes with attestation where required. |
| Export review | stable | stable | Shows redaction class, portable format, and support-bundle safety before any portable handoff. |
| Response viewer | preview | preview | Shows stream states and export redaction but is still below stable pending full UI parity. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.

## Redaction and privacy

- Export rows never include raw secrets or raw response bodies.
- History lanes default to `local_first` retention.
- Support-bundle-safe exports use `full_redaction`, `metadata_only`, or `safe_preview` classes only.

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
