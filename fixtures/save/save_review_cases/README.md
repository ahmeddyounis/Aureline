# Save-review sheet cases (save safety fixtures)

This directory is a small, reviewer-facing corpus for the **save-review
sheet** shown when a save attempt is refused due to external drift or other
review-required outcomes.

These fixtures are descriptive inputs/notes for the save lane and are consumed
by deterministic unit tests in `crates/aureline-shell/src/save_review/`. They do
not require a live shell run.

Vocabulary anchors:

- `docs/ux/editor_external_change_contract.md` (choice keys and guardrails)
- `artifacts/fs/save_review_choice_matrix.yaml` (forbidden reason codes)
- `schemas/runtime/vfs_save_envelope.schema.json` (`SaveOutcome` tokens)

