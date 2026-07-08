# M5 structured-artifact review-component consumer fixtures

Protected fixtures for the closing consumer-adoption lane
`add_shared_diff_toolbar_merge_sheet_review_workspace_help_support_and_export_consumers_so_artifact_review_components_keep_mode_risk_and_provenance_language_aligned`
in `crates/aureline-review`.

Each fixture is an `ArtifactReviewComponentConsumerPacket` validated against
[`schemas/ui/m5-structured-artifact-review-component-consumer.schema.json`](../../../schemas/ui/m5-structured-artifact-review-component-consumer.schema.json)
and by the module's `validate()` — the same nine components bound to the diff
toolbar, merge sheet, review workspace, Help surface, support packet, and exported
view, proving that the same artifact object presents identical canonical-source,
mode, risk, and provenance language across surfaces.

| Fixture | Scenario |
| --- | --- |
| `structured_fidelity_narrowed.json` | Some objects narrow to `structured_fidelity_narrowed` (partial structure / untrusted render); the raw / export-safe fallback stays explicit and every parity facet is preserved. |
| `raw_fallback_and_redaction.json` | Some objects fall back to `raw_fallback_disclosed` (schema unrecognized) or `redaction_narrowed` (content withheld); each narrowing is disclosed through its banner and note. |

Regenerate the checked-in support export, summary, and fixtures after a contract
change:

```sh
GEN_ARTIFACT_REVIEW_CONSUMER_ARTIFACTS=1 cargo test -p aureline-review --lib \
  regenerate_artifact_review_consumer_artifacts
```
