# M5 pipeline / dependency / finding component fixtures

These fixtures are the first-consumer examples for the frozen matrix at
`artifacts/design/m5-pipeline-dependency-finding-component-matrix.md`.

| Fixture | Schema | Scenario |
| --- | --- | --- |
| `pipeline_run_row.json` | `schemas/ui/m5-pipeline-run-row.schema.json` | A partial provider-owned CI run preserves provider/run identity, trigger, branch/change relation, artifact counts, freshness, and rerun/cancel authority across review, project health, companion, support, and release proof. |
| `annotation_row.json` | `schemas/ui/m5-annotation-row.schema.json` | A stale scanner annotation remains visible in code, review, health, companion, support, and release surfaces while suppressed until review, preserving provider/scanner disclosure, manifest anchor, stale handoff, severity, confidence, freshness, remediation, and open-details action without silently retargeting. |
| `dependency_row.json` | `schemas/ui/m5-dependency-row.schema.json` | A transitive dependency row preserves manifest scope, version delta, advisory count, license action, and changelog action. |
| `manifest_diff_card.json` | `schemas/ui/m5-manifest-diff-card.schema.json` | A staged manifest diff previews scripts/hooks and peer/runtime constraint changes while naming checkpoint, rollback, and apply boundary. |
| `security_finding_card.json` | `schemas/ui/m5-security-finding-card.schema.json` | A critical no-fix-yet finding keeps severity, confidence, freshness, exception-expired suppression, exposure, and remediation visible. |

Validate with:

```sh
python3 - <<'PY'
import json
from pathlib import Path
from jsonschema import Draft202012Validator

pairs = [
    ("schemas/ui/m5-pipeline-run-row.schema.json", "fixtures/ui/m5-pipeline-dependency-finding-components/pipeline_run_row.json"),
    ("schemas/ui/m5-annotation-row.schema.json", "fixtures/ui/m5-pipeline-dependency-finding-components/annotation_row.json"),
    ("schemas/ui/m5-dependency-row.schema.json", "fixtures/ui/m5-pipeline-dependency-finding-components/dependency_row.json"),
    ("schemas/ui/m5-manifest-diff-card.schema.json", "fixtures/ui/m5-pipeline-dependency-finding-components/manifest_diff_card.json"),
    ("schemas/ui/m5-security-finding-card.schema.json", "fixtures/ui/m5-pipeline-dependency-finding-components/security_finding_card.json"),
]
for schema_path, fixture_path in pairs:
    schema = json.loads(Path(schema_path).read_text())
    fixture = json.loads(Path(fixture_path).read_text())
    Draft202012Validator.check_schema(schema)
    Draft202012Validator(schema).validate(fixture)
PY
```
