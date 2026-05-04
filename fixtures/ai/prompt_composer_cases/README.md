# AI prompt-composer session, turn-draft, mention, attachment, slash-command, and pre-dispatch disclosure worked fixtures

This directory holds worked examples for the composer-session
side of the contract frozen in
[`/docs/ai/prompt_composer_contract.md`](../../../docs/ai/prompt_composer_contract.md)
and the schemas at
[`/schemas/ai/prompt_composer_session.schema.json`](../../../schemas/ai/prompt_composer_session.schema.json)
and
[`/schemas/ai/request_workspace_ref.schema.json`](../../../schemas/ai/request_workspace_ref.schema.json).

Every file is a multi-document YAML. The first document is a
`__fixture__` prelude summarising the scenario, the contract
sections exercised, and the typed classes hit. Each subsequent
document is one record conforming to the prompt-composer session
schema (or, for `request_workspace_ref` fields embedded inside a
descriptor, the request-workspace-ref schema).

No fixture embeds raw prompt text, raw retrieved-document bodies,
raw terminal / log bodies, raw generated artifact bytes, raw
request / response payloads, raw user-supplied text, raw
workspace paths, raw URLs, raw endpoint hostnames, raw API keys,
raw OAuth tokens, raw mTLS material, or raw credential material.
Every such field is an opaque ref or a structured redaction-safe
readout.

## Cases

| Scenario file                                       | Axis exercised                                                                                                                                                                                  |
|-----------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `empty_draft.yaml`                                  | Composer session opened, turn draft in `draft` state, no mentions, no attachments, no slash commands, no pre-dispatch disclosure yet. Pins the minimal valid composer session.                  |
| `single_file_ask.yaml`                              | Single `file_mention` + matching `file_slice_excerpt` produced by an `/explain` slash command. Pre-dispatch disclosure carries every required field with all four count fields at zero. Dispatched inline. |
| `cross_repo_ask_with_omitted_context.yaml`          | Cross-repo /explain. One mention is `omitted_outside_scope`, one is `omitted_under_budget`. Pre-dispatch disclosure shows `omitted_attachment_count = 2` verbatim.                                |
| `tainted_pasted_content.yaml`                       | User-pasted text fenced as `fenced_tainted_user_pasted` (quoted_as_data_only, full usage-constraint set). One log span attachment redacted under broker pass. Disclosure shows tainted=1, redacted=1. |
| `background_branch_agent_dispatch.yaml`             | `/background-branch-agent` dispatch with `branch_agent_dispatch_intent_disclosed` in disclosure_fields. Browser-handoff approval ticket. Turn draft transitions to `dispatched_branch_agent`.    |

Every fixture declares its scenario and the contract sections /
typed classes it exercises via the `__fixture__` prelude so later
coverage audits can confirm each vocabulary member is hit at
least once.

## Acceptance-criteria coverage

The seeded cases cover the three acceptance criteria named in the
spec:

- **Round-trip composer sessions, mentions, attachments, and
  request-workspace refs without redefining identity or trust
  semantics.** Every fixture binds the composer session, turn
  draft, mentions, attachments, slash command, and pre-dispatch
  disclosure with the same id namespaces the assembly-side records
  use (`composer_session_id`, `turn_draft_id`, `mention_id`,
  `attachment_id`); the `request_workspace_ref` field carries the
  typed `request_workspace_ref_record` re-exporting flow / scope /
  posture / lifecycle from the owning workspace verbatim.
- **Scope, target, and tainted-context status visible without
  hidden UI-only state.** Every fixture's pre-dispatch disclosure
  record lists the required `disclosure_fields` (scope, target
  context, active account / provider / model, route path, cost
  visibility, approval posture, request_workspace_ref) plus the
  four count fields (tainted, omitted, policy-blocked, redacted)
  even when the count is zero. `tainted_pasted_content.yaml`
  pins `tainted_attachment_count = 1`; the user-paste attachment
  carries `disposition_class = fenced_tainted_user_pasted`,
  `tainted_fence_strategy = quoted_as_data_only`, and the full
  `tainted_usage_constraints` set.
- **Composer output points at route receipts, spend receipts,
  and branch-agent dispatch packets without ad hoc glue
  fields.** Every dispatched draft binds
  `route_plan_ref`, `spend_plan_ref`,
  `route_receipt_ref`, `spend_receipt_ref` (and / or
  `branch_agent_dispatch_ref` / `review_handoff_ref`) at typed
  fields on the descriptor — and the same placeholder ids appear
  on the disclosure record so reviewers can cross-walk pre- and
  post-dispatch state.
  `background_branch_agent_dispatch.yaml` pins the
  `branch_agent_dispatch_placeholder_ref` on the disclosure and
  the matching `branch_agent_dispatch_ref` on the draft.

## Validation

Every fixture in this directory validates against
`schemas/ai/prompt_composer_session.schema.json` (and, for the
embedded `request_workspace_ref` fields, against
`schemas/ai/request_workspace_ref.schema.json`) under Draft
2020-12. A small Python harness (using
`jsonschema.Draft202012Validator` plus `PyYAML`) is sufficient
for local checks:

```python
import json
from pathlib import Path
import yaml
from jsonschema import Draft202012Validator
from referencing import Registry, Resource

repo = Path(__file__).resolve().parents[3]
session_schema = json.loads(
    (repo / "schemas/ai/prompt_composer_session.schema.json").read_text()
)
ref_schema = json.loads(
    (repo / "schemas/ai/request_workspace_ref.schema.json").read_text()
)
registry = Registry().with_resources([
    (session_schema["$id"], Resource.from_contents(session_schema)),
    (ref_schema["$id"], Resource.from_contents(ref_schema)),
])
validator = Draft202012Validator(session_schema, registry=registry)
for path in (repo / "fixtures/ai/prompt_composer_cases").glob("*.yaml"):
    docs = list(yaml.safe_load_all(path.read_text()))
    for doc in docs:
        if doc is None or "__fixture__" in doc:
            continue
        validator.validate(doc)
```
