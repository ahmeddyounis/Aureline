# Provider Account/Session and Mapping Continuity Drills

This page is the reviewer-facing guide to the provider-account/target-mapping
continuity drill corpus. Where
[`provider_account_and_mapping_truth.md`](provider_account_and_mapping_truth.md)
describes the steady-state truth (who is acting, which target, local/queued/live
posture), this corpus proves that truth survives **failure**, **recovery**, and
**durable reopen** for every marketed beta provider lane.

## Sources

- Schema: `schemas/providers/provider_account_mapping_drill_case.schema.json`
- Fixtures: `fixtures/providers/m3/account_scope_and_mapping_corpus/`
- Report: `artifacts/providers/m3/account_scope_and_mapping_report.md`
- Per-lane continuity packet:
  `artifacts/providers/m3/account_scope_and_mapping_continuity_matrix.json`
- Validator: `ci/check_provider_mapping_corpus.py`
- Entry point: `scripts/ci/run_provider_mapping_corpus.sh`

## The three lane-failing invariants

Each drill case is rejected — the lane fails closed — if any of the following
would happen:

1. **A queued draft silently vanishes.** Queued drafts, local drafts, queued
   transitions, and evidence stay durable with retry/export available.
2. **A narrowed session still appears writable.** Stale, limited-scope,
   read-only, offline-capture, and publish-later-only sessions degrade to a
   local draft, a queued publish-later item, or a read-only inspection with a
   named next-safe action — never a live mutation.
3. **A mapping changes without a visible review.** A remap or invalidation
   always surfaces a visible review before the new target is used.

## Identity that survives

Every drill pins the stable `provider_id` / `account_id` / `mapping_id` triple
and echoes it verbatim through support export, activity-center reopen, and
restart/restore. None of those surfaces may carry a raw access token, raw
delegated-token body, raw provider payload, or raw provider URL.

## Drill classes

| Drill class | What a reviewer should see |
| --- | --- |
| `board_project_remap` | The remap is held for review; the issue update is not silently re-pointed. |
| `stale_token` | The comment is a local draft with a reconnect path; nothing looks writable. |
| `installation_grant_withdrawal` | The mapping invalidates; queued evidence drops to a local draft with an admin path. |
| `policy_locked_mapping` | An attempted remap is refused with an admin path; the locked target is kept. |
| `offline_capture` | The incident handoff is queued for publish-later with evidence retained. |
| `browser_blocked_handoff` | A blocked handoff offers a truthful fallback and a local draft. |
| `publish_later_replay` | The queued action replays on reconnect; the queued draft is preserved. |
| `queued_draft_export_import` | The queued draft survives an export/import round trip with identity intact. |

## Running the drills

```sh
scripts/ci/run_provider_mapping_corpus.sh
```

This validates every fixture against the schema, enforces the three
lane-failing invariants and the identity-continuity invariants, checks the
enum-only corpus matrix and the per-lane continuity packet against the
fixtures, and (when Cargo is available) re-validates the seeded beta page. The
corpus is intentionally small — eight cases — so it runs as per-PR smoke
coverage; nightly runs add the Cargo re-validation step.

## Beta scorecard reference

A beta scorecard for a claimed provider lane can cite the per-lane row in
`artifacts/providers/m3/account_scope_and_mapping_continuity_matrix.json`: each
lane lists its drill cases and asserts identity survives all three durable
surfaces and that every case fails closed.
