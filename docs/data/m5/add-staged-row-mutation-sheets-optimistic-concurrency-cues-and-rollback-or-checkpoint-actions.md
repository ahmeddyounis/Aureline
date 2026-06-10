# Staged row-mutation sheets, optimistic-concurrency cues, and rollback or checkpoint actions

## Scope

This document describes the canonical M5 qualification packet for staged row-mutation sheets, optimistic-concurrency control cues, rollback actions, and checkpoint actions in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/add_staged_row_mutation_sheets_optimistic_concurrency_cues_and_rollback_or_checkpoint_actions/mod.rs`
- Schema: `schemas/data/add-staged-row-mutation-sheets-optimistic-concurrency-cues-and-rollback-or-checkpoint-actions.schema.json`
- Checked-in packet: `artifacts/data/m5/add-staged-row-mutation-sheets-optimistic-concurrency-cues-and-rollback-or-checkpoint-actions.json`
- Fixtures: `fixtures/data/m5/add_staged_row_mutation_sheets_optimistic_concurrency_cues_and_rollback_or_checkpoint_actions/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Staged row-mutation sheet | stable | stable | Shows pending inserts, updates, and deletes with preview, confirmation, and rollback path before apply. |
| Optimistic-concurrency cue | stable | stable | Shows version stamps, stale-read warnings, and resolution actions before commit. |
| Rollback action | stable | stable | Discloses single-row, sheet-level, session-level, and checkpoint-scoped undo with confirmation where required. |
| Checkpoint action | preview | preview | Visible and restorable but is still below stable pending full session-scope parity. |
| Mutation session | preview | preview | Shows sheet state and target context but does not yet show full auth-scope detail in preview. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.

## Redaction and privacy

- Staged sheets never include raw row values or raw primary keys in exported packets.
- Optimistic-concurrency cues use opaque version refs instead of raw timestamps or sequence numbers.
- Rollback and checkpoint actions never include raw transaction IDs or raw database connection strings.
- Support-bundle-safe exports use `full_redaction`, `metadata_only`, or `safe_preview` classes only.

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
