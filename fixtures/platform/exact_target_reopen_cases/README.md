# Exact-target reopen cases

Worked `deep_link_intent_record` fixtures that prove OS-facing reopen paths:

- resolve to the **exact target** truthfully, or
- **fail closed** with bounded recovery actions (review sheet, locate, cached context, activity center, default browser restart),
- without silent redirects, last-writer-wins handler assumptions, or raw-path/raw-URL leakage.

These cases complement:

- `fixtures/platform/deep_link_replay_cases/` (replay-deny proofs)
- `fixtures/platform/system_affordance_cases/` (broader system-affordance fixtures)

The schema of record is `schemas/platform/deep_link_intent.schema.json`.

## Fixture rules

- `record_kind` is `deep_link_intent_record`.
- No raw URLs, raw callback bodies, raw absolute paths, or secret material appears; use opaque ids only.
- On success, `degraded_reasons` includes `none`.
- On failure, the record must deny or degrade with a typed reason and one bounded fallback.
