# Richer M5 Prompt Composer Fixtures

This directory contains protected fixtures for the richer prompt-composer lane.

## Fixtures

- `rich_composer_draft_patch.yaml` — Full draft-patch intent with all seven attachment source classes, two pinned contexts (one stale), two omitted contexts with restoration paths, and a warning budget strip.
- `rich_composer_ask_with_omitted_context.yaml` — Ask intent with cross-repo scope-excluded omitted context and a large-file budget omission, demonstrating read-only-context constraints.

## Validation

Every fixture is expected to parse as a valid `RicherPromptComposerPacket` and pass `validate_self` when loaded.
