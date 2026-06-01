# Optional AI-adjacent surface qualification audit fixtures

This directory contains the protected fixtures for the optional AI-adjacent
surface qualification audit lane. Each fixture demonstrates one surface family
qualification row and can be round-tripped through
`OptionalAiAdjacentSurfaceAuditPacket::validate`.

## Files

| File | Surface family | Qualification |
|---|---|---|
| `notebook_qualification.json` | `notebook` | `limited` |
| `voice_qualification.json` | `voice` | `experimental` |
| `browser_companion_qualification.json` | `browser_companion` | `limited` |
| `preview_designer_qualification.json` | `preview_designer` | `limited` |
| `background_branch_automation_qualification.json` | `background_branch_automation` | `limited` |
| `qualification_matrix.json` | all families | audit packet |

The boundary schema is
`schemas/ai/optional-ai-surface-qualification.schema.json`.

Run the full suite with:

```sh
cargo test -p aureline-ai audit_optional_ai_adjacent_surfaces
```
