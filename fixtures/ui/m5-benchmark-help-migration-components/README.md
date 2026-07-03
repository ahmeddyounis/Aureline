# M5 benchmark / help / migration component fixtures

These fixtures are the first-consumer examples for the frozen matrix at
`artifacts/design/m5-benchmark-help-migration-component-matrix.md`.

| Fixture | Schema | Scenario |
| --- | --- | --- |
| `benchmark_evidence_card.json` | `schemas/ui/m5-benchmark-evidence-card.schema.json` | Stale benchmark evidence narrows to retest pending while preserving workflow, budget, corpus, hardware, and copy/export fields. |
| `about_service_health_card.json` | `schemas/ui/m5-about-service-health-card.schema.json` | Cached service-health status stays local-first and visible as cached rather than live. |
| `support_package_card.json` | `schemas/ui/m5-support-package-card.schema.json` | A package saved locally is not represented as submitted support. |
| `importer_diff_row.json` | `schemas/ui/m5-importer-diff-row.schema.json` | A bridge-required import row keeps compatibility and checkpoint/restore context. |
| `community_handoff_tile.json` | `schemas/ui/m5-community-handoff-tile.schema.json` | A community-owned destination is disclosed before exit and keeps a local fallback. |

Validate with:

```sh
python3 - <<'PY'
import json
from pathlib import Path
from jsonschema import Draft202012Validator

pairs = {
    "schemas/ui/m5-benchmark-evidence-card.schema.json": "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card.json",
    "schemas/ui/m5-about-service-health-card.schema.json": "fixtures/ui/m5-benchmark-help-migration-components/about_service_health_card.json",
    "schemas/ui/m5-support-package-card.schema.json": "fixtures/ui/m5-benchmark-help-migration-components/support_package_card.json",
    "schemas/ui/m5-importer-diff-row.schema.json": "fixtures/ui/m5-benchmark-help-migration-components/importer_diff_row.json",
    "schemas/ui/m5-community-handoff-tile.schema.json": "fixtures/ui/m5-benchmark-help-migration-components/community_handoff_tile.json",
}
for schema_path, fixture_path in pairs.items():
    schema = json.loads(Path(schema_path).read_text())
    fixture = json.loads(Path(fixture_path).read_text())
    Draft202012Validator.check_schema(schema)
    Draft202012Validator(schema).validate(fixture)
PY
```

