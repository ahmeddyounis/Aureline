# External-drift cases (save safety fixtures)

This directory is a small, reviewer-facing corpus for **external drift**
detection in the staged save pipeline.

The intent is to keep the vocabulary aligned with:

- `schemas/runtime/vfs_save_envelope.schema.json` (`SaveOutcome` tokens),
- `fixtures/runtime/vfs_decision_examples/external_change_conflict.json`, and
- `fixtures/fs/conflict_save_cases/` (review-choice matrix expectations).

These fixtures are descriptive inputs/notes for the save lane; they do not
currently drive an automated harness.

