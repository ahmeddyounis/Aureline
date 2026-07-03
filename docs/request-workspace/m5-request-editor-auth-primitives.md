# M5 request editor header, environment picker, variable inspector, and auth sheet

The request editor header, environment picker, variable-resolution inspector,
and auth sheet are reusable UI contracts for every M5 surface that can send,
replay, or inspect a request. The canonical matrix is
`artifacts/design/m5-request-data-component-matrix.md`; first-consumer fixtures
live under `fixtures/ui/m5-request-data-components/`.

## Request header

The header must show the operation kind, target identity, execution origin,
environment picker, auth posture, capability state, run/cancel control state,
last-run state, last-run summary, variable inspector ref, and auth sheet ref
before a send can commit. Browser-runtime and managed consumers may narrow to
mutation review or inspect-only, but they keep the same origin, auth, and
last-run labels as the desktop request workspace.

## Environment and variable resolution

Environment pickers and variable inspectors keep the source layers visible:
workspace, profile, collection, request document, runtime override, secret
broker, managed policy, browser runtime, imported snapshot, and default value.
They also expose resolved/unresolved state, override scope, export scope, and a
redacted preview state. Raw secret values, raw tokens, raw cookies, verification
codes, and credential bodies are never valid resting UI or default export data.

## Auth sheet

Auth sheets render `scheme`, `secret_source_class`, `auth_storage_mode`,
`token_lifetime`, expiry posture, browser/device-code handoff state, policy
notes, redaction review state, and `raw_secret_exposed=false`. The sheet is
referenced from the request header so request workspace, browser-runtime panel,
CLI/headless inspect, support export, and release proof share the same auth
truth instead of rewording it.

## Verification

Validate the checked-in UI contracts with:

```bash
python3 - <<'PY'
import json
from pathlib import Path
from jsonschema import Draft202012Validator

pairs = [
    ("schemas/ui/m5-request-editor-header.schema.json", "fixtures/ui/m5-request-data-components/request_editor_header.json"),
    ("schemas/ui/m5-variable-resolution-inspector.schema.json", "fixtures/ui/m5-request-data-components/variable_resolution_inspector.json"),
    ("schemas/ui/m5-auth-sheet.schema.json", "fixtures/ui/m5-request-data-components/auth_sheet.json"),
]

for schema_path, fixture_path in pairs:
    schema = json.loads(Path(schema_path).read_text())
    fixture = json.loads(Path(fixture_path).read_text())
    Draft202012Validator.check_schema(schema)
    Draft202012Validator(schema).validate(fixture)
PY
```
