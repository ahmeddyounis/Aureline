# M5 request/data component fixtures

These fixtures are the first-consumer examples for the frozen matrix at
`artifacts/design/m5-request-data-component-matrix.md`.
The release certification bundle is
`artifacts/release/m5-request-data-component-proof/proof_packet.json`, the
support projection is
`artifacts/release/m5-request-data-component-proof/support_export.json`, and
the fixture-to-consumer manifest is
`fixtures/ui/m5-request-data-components/component_manifest.json`.

| Fixture | Schema | Scenario |
| --- | --- | --- |
| `request_editor_header.json` | `schemas/ui/m5-request-editor-header.schema.json` | A browser-runtime GraphQL mutation is narrowed to mutation review while preserving target, origin, environment, auth storage, run/cancel state, last-run summary, and variable/auth inspector refs. |
| `variable_resolution_inspector.json` | `schemas/ui/m5-variable-resolution-inspector.schema.json` | Layered variable resolution shows workspace, runtime, and secret-broker layers, redacted preview state, override scope, and export scope without exposing raw secrets. |
| `auth_sheet.json` | `schemas/ui/m5-auth-sheet.schema.json` | A browser/device-code auth sheet exposes scheme, secret source class, token lifetime, expiry posture, handoff state, and policy notes without raw tokens or verification codes. |
| `response_tabset.json` | `schemas/ui/m5-response-tabset.schema.json` | REST response tabs keep summary, body, headers/cookies, assertions, timeline, and browser trust separate. |
| `request_history_row.json` | `schemas/ui/m5-request-history-row.schema.json` | Request history rows preserve timestamp, environment, origin scope, status/result class, assertion state, retention/redaction, replay, export, compare, and contract badge refs without raw secrets or unsafe payloads. |
| `contract_source_badge.json` | `schemas/ui/m5-contract-source-badge.schema.json` | Contract/source badges link requests to OpenAPI/GraphQL/gRPC or imported/manual sources with version/snapshot and freshness truth across editor, history, handoff, compare, CLI, and support projections. |
| `connection_picker_row.json` | `schemas/ui/m5-connection-picker-row.schema.json` | A managed warehouse connection is read-only and permission-limited while preserving schema tree freshness and auth storage mode. |
| `result_grid.json` | `schemas/ui/m5-result-grid.schema.json` | A typed grid discloses returned-only row scope, loaded ranges, truncation, virtualization, null/binary/JSON rendering, filter/sort locus, redaction review, and copy/export actions. |
| `query_history_row.json` | `schemas/ui/m5-query-history-row.schema.json` | A database query-history row keeps service label, origin, statement class, duration, row/affected counts, success/error state, retention mode, replay posture, and result/plan refs. |
| `explain_plan_pane.json` | `schemas/ui/m5-explain-plan-pane.schema.json` | An imported estimated plan remains stale, carries warnings and a safe source-query link, and cannot masquerade as an actual executed plan. |
| `component_manifest.json` | Matrix manifest | Maps auth sheet, environment picker, schema tree, and query-history rows to canonical component families and narrowed behaviors. |

Each schema-backed fixture carries `reduced_capability_banner` and
`provider_handoff_notes` so narrower consumers disclose missing send, replay,
mutate, or raw-export authority without changing canonical labels or dropping
export fields.

Validate with:

```sh
python3 - <<'PY'
import json
from pathlib import Path
from jsonschema import Draft202012Validator

pairs = [
    ("schemas/ui/m5-request-editor-header.schema.json", "fixtures/ui/m5-request-data-components/request_editor_header.json"),
    ("schemas/ui/m5-variable-resolution-inspector.schema.json", "fixtures/ui/m5-request-data-components/variable_resolution_inspector.json"),
    ("schemas/ui/m5-auth-sheet.schema.json", "fixtures/ui/m5-request-data-components/auth_sheet.json"),
    ("schemas/ui/m5-response-tabset.schema.json", "fixtures/ui/m5-request-data-components/response_tabset.json"),
    ("schemas/ui/m5-request-history-row.schema.json", "fixtures/ui/m5-request-data-components/request_history_row.json"),
    ("schemas/ui/m5-contract-source-badge.schema.json", "fixtures/ui/m5-request-data-components/contract_source_badge.json"),
    ("schemas/ui/m5-connection-picker-row.schema.json", "fixtures/ui/m5-request-data-components/connection_picker_row.json"),
    ("schemas/ui/m5-sql-run-bar.schema.json", "fixtures/ui/m5-request-data-components/sql_run_bar.json"),
    ("schemas/ui/m5-result-grid.schema.json", "fixtures/ui/m5-request-data-components/result_grid.json"),
    ("schemas/ui/m5-query-history-row.schema.json", "fixtures/ui/m5-request-data-components/query_history_row.json"),
    ("schemas/ui/m5-explain-plan-pane.schema.json", "fixtures/ui/m5-request-data-components/explain_plan_pane.json"),
]
for schema_path, fixture_path in pairs:
    schema = json.loads(Path(schema_path).read_text())
    fixture = json.loads(Path(fixture_path).read_text())
    Draft202012Validator.check_schema(schema)
    Draft202012Validator(schema).validate(fixture)

row_schema = json.loads(Path("schemas/ui/m5-schema-object-row.schema.json").read_text())
Draft202012Validator.check_schema(row_schema)
row_validator = Draft202012Validator(row_schema)
schema_rows = json.loads(Path("fixtures/ui/m5-request-data-components/schema_object_rows.json").read_text())
for row in schema_rows["rows"]:
    row["reduced_capability_banner"] = schema_rows["reduced_capability_banner"]
    row["provider_handoff_notes"] = schema_rows["provider_handoff_notes"]
    row_validator.validate(row)
PY
```
