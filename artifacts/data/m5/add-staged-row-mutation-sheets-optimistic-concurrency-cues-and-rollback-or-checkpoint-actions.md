# Staged row-mutation sheets, optimistic-concurrency cues, and rollback or checkpoint actions — Artifact Summary

## Packet identity

- `packet_id`: `m5_041_staged_row_mutation_qualification:v1`
- `as_of`: `2026-06-09`
- `schema_version`: `1`
- `record_kind`: `staged_row_mutation_sheets_optimistic_concurrency_cues_and_rollback_or_checkpoint_actions`

## Surfaces

5 promoted surfaces, 3 stable, 2 narrowed.

## Rows

- 5 staged row-mutation sheets (insert, update, delete, upsert, batch)
- 5 optimistic-concurrency cues (no conflict, stale read, version mismatch, concurrent write, policy blocked)
- 4 rollback actions (single row, sheet level, session level, to checkpoint)
- 4 checkpoint actions (pre-mutation, pre-commit, post-commit, automatic)

## Validation

The packet passes `StagedRowMutationQualificationPacket::validate()` with zero violations.

## Downgrade behavior

All surfaces have `downgrade_if_missing: true`. If proof artifacts are stale or removed, stable claims narrow to `preview` automatically.
