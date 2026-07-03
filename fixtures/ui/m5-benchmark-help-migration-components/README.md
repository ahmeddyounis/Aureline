# M5 benchmark / help / migration component fixtures

These fixtures are the first-consumer examples for the frozen matrix at
`artifacts/design/m5-benchmark-help-migration-component-matrix.md`.

| Fixture | Schema | Scenario |
| --- | --- | --- |
| `benchmark_evidence_card.json` | `schemas/ui/m5-benchmark-evidence-card.schema.json` | Stale lab/reference benchmark evidence narrows to retest pending while preserving workflow, measured value versus budget, corpus, environment, as-of date, caveats, and copy/export fields. |
| `benchmark_evidence_card_self_capture.json` | `schemas/ui/m5-benchmark-evidence-card.schema.json` | A self capture stays separate from reference proof and exports its self-capture caveat. |
| `benchmark_evidence_card_design_partner.json` | `schemas/ui/m5-benchmark-evidence-card.schema.json` | A design-partner result stays limited by redacted environment/corpus truth. |
| `benchmark_evidence_card_community.json` | `schemas/ui/m5-benchmark-evidence-card.schema.json` | A community report stays methodology-only until verified against reference proof. |
| `benchmark_evidence_card_imported.json` | `schemas/ui/m5-benchmark-evidence-card.schema.json` | Imported evidence stays retest-pending until environment truth is verified. |
| `about_service_health_card.json` | `schemas/ui/m5-about-service-health-card.schema.json` | Cached service-health status stays local-first and visible as cached rather than live. |
| `support_package_card.json` | `schemas/ui/m5-support-package-card.schema.json` | A package saved locally is not represented as submitted support. |
| `importer_diff_row.json` | `schemas/ui/m5-importer-diff-row.schema.json` | A bridge-required import row keeps source/target values, translated result, reason note, actions, export-safe ids, compatibility, and checkpoint/restore context. |
| `importer_review_table.json` | Shared typed fixture | Outcome-grouped review table proving `imported`, `mapped`, `skipped`, `manual_review`, `bridge_required`, and `unsupported` rows across settings, shortcuts, extensions, tasks, and workspace metadata remain visible after apply/export. |
| `community_handoff_tile_official_public.json` | `schemas/ui/m5-community-handoff-tile.schema.json` | An official public release issue template discloses world-readable visibility, version-specific release context, and copyable issue-template context. |
| `community_handoff_tile_official_authenticated.json` | `schemas/ui/m5-community-handoff-tile.schema.json` | An official authenticated support handoff remains inspectable when browser launch is blocked. |
| `community_handoff_tile.json` | `schemas/ui/m5-community-handoff-tile.schema.json` | A community-owned destination is disclosed before exit and keeps a local fallback. |
| `community_handoff_tile_vendor.json` | `schemas/ui/m5-community-handoff-tile.schema.json` | A vendor-owned extension support route is distinct from Aureline official support and keeps offline copy continuity. |
| `community_handoff_tile_local_only.json` | `schemas/ui/m5-community-handoff-tile.schema.json` | A local-only migration draft discloses cached/offline version context and never opens an external destination. |

Validate with:

```sh
python3 - <<'PY'
import json
from pathlib import Path
from jsonschema import Draft202012Validator

pairs = [
    ("schemas/ui/m5-benchmark-evidence-card.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card.json"),
    ("schemas/ui/m5-benchmark-evidence-card.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card_self_capture.json"),
    ("schemas/ui/m5-benchmark-evidence-card.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card_design_partner.json"),
    ("schemas/ui/m5-benchmark-evidence-card.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card_community.json"),
    ("schemas/ui/m5-benchmark-evidence-card.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card_imported.json"),
    ("schemas/ui/m5-about-service-health-card.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/about_service_health_card.json"),
    ("schemas/ui/m5-support-package-card.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/support_package_card.json"),
    ("schemas/ui/m5-importer-diff-row.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/importer_diff_row.json"),
    ("schemas/ui/m5-community-handoff-tile.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile_official_public.json"),
    ("schemas/ui/m5-community-handoff-tile.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile_official_authenticated.json"),
    ("schemas/ui/m5-community-handoff-tile.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile.json"),
    ("schemas/ui/m5-community-handoff-tile.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile_vendor.json"),
    ("schemas/ui/m5-community-handoff-tile.schema.json", "fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile_local_only.json"),
]
for schema_path, fixture_path in pairs:
    schema = json.loads(Path(schema_path).read_text())
    fixture = json.loads(Path(fixture_path).read_text())
    Draft202012Validator.check_schema(schema)
    Draft202012Validator(schema).validate(fixture)
PY
```
