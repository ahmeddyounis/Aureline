# AI patch evidence + rollback across TS project boundaries (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:ai_patch_evidence_across_projects`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`

## Scenario goal

Prove that AI-assisted patches over a TS/JS workspace remain:

- attributable (what was changed, why, and against which scope);
- reviewable (preview-first posture, with exclusions explicit); and
- rollbackable (checkpoint exists and is referenced).

This scenario does not endorse any provider. It only defines the evidence
and rollback contract a “patch” must satisfy before it can be treated as
trustworthy work.

## Required truth and disclosures

- Evidence replayability and provenance requirements:
  - `docs/ai/evidence_replayability_contract.md`
- Review-assist and diff/patch publish contracts (preview, redaction,
  outdated-scope handling):
  - `docs/ai/review_assist_publish_contract.md`
- Workspace refactor preview/apply/rollback rules:
  - `docs/editor/refactor_and_replace_transaction_contract.md`
- Generated artifacts remain protected unless explicit override posture is
  declared:
  - `docs/architecture/generated_artifact_safe_edit_policy.md`

## Known-limit expectations

- If the AI patch workflow cannot validate affected symbols/files under the
  declared scope (for example partial index), certification requires a
  known-limit note that narrows the claim to validated scopes only.

