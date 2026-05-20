# Provider account-scope and target-mapping continuity report

This report is the deterministic, support- and partner-facing summary of the M3
provider-account/target-mapping continuity drill corpus. It is generated from
the checked-in drill fixtures under
[`fixtures/providers/m3/account_scope_and_mapping_corpus/`](../../../fixtures/providers/m3/account_scope_and_mapping_corpus/)
and validated against
[`schemas/providers/provider_account_mapping_drill_case.schema.json`](../../../schemas/providers/provider_account_mapping_drill_case.schema.json)
by
[`ci/check_provider_mapping_corpus.py`](../../../ci/check_provider_mapping_corpus.py),
run from
[`scripts/ci/run_provider_mapping_corpus.sh`](../../../scripts/ci/run_provider_mapping_corpus.sh).

It pairs with:

- the per-lane continuity packet at
  [`artifacts/providers/m3/account_scope_and_mapping_continuity_matrix.json`](account_scope_and_mapping_continuity_matrix.json),
- the reviewer drills doc at
  [`docs/providers/m3/provider_account_and_mapping_drills.md`](../../../docs/providers/m3/provider_account_and_mapping_drills.md),
- the account/session and mapping truth landing page at
  [`docs/providers/m3/provider_account_and_mapping_truth.md`](../../../docs/providers/m3/provider_account_and_mapping_truth.md),
- the seeded beta page in
  [`crates/aureline-provider/src/project_mapping/mod.rs`](../../../crates/aureline-provider/src/project_mapping/mod.rs).

## Why this corpus exists

The account-scope beta made provider *authority* honest and the target-mapping
beta made provider *targeting* honest. This corpus keeps both honest under
failure and across durable surfaces, before stable planning assumes these
hosted lanes are trustworthy. Every marketed beta provider lane carries current
evidence that stale credentials, policy-locked mappings, offline capture,
limited-scope sessions, and publish-later recovery behave truthfully instead of
degrading into silent mutation failure or dropped local work.

Three lane-failing invariants are enforced on every drill case:

1. A queued draft never silently vanishes.
2. A narrowed session never still appears writable.
3. A mapping never changes without a visible review.

And the stable provider/account/mapping identity triple (`provider_id`,
`account_id`, `mapping_id`) survives **support export**, **activity-center
reopen**, and **restart/restore** without leaking raw credentials.

## 1 Drill cases

| Fixture | drill_class | provider_lane | profile | session_state | publish_posture | fail_closed |
| --- | --- | --- | --- | --- | --- | --- |
| `board_project_remap_held_for_review.json` | `board_project_remap` | `issue_or_work_item` | `connected` | `live` | `local_draft` | yes |
| `stale_token_blocks_live_mutation.json` | `stale_token` | `issue_or_work_item` | `mirror_only` | `stale_credential` | `local_draft` | yes |
| `installation_grant_withdrawal_invalidates_mapping.json` | `installation_grant_withdrawal` | `publish_later` | `enterprise_managed` | `read_only` | `local_draft` | yes |
| `policy_locked_mapping_blocks_remap.json` | `policy_locked_mapping` | `review_decision` | `connected` | `live` | `local_draft` | yes |
| `offline_capture_queues_incident_handoff.json` | `offline_capture` | `incident_handoff` | `offline` | `offline_capture` | `queued_publish_later` | yes |
| `browser_blocked_handoff_offers_fallback.json` | `browser_blocked_handoff` | `review_decision` | `connected` | `limited_scope` | `local_draft` | yes |
| `publish_later_replay_preserves_queue.json` | `publish_later_replay` | `publish_later` | `enterprise_managed` | `publish_later_only` | `queued_publish_later` | yes |
| `queued_draft_export_import_round_trip.json` | `queued_draft_export_import` | `incident_handoff` | `mirror_only` | `offline_capture` | `queued_publish_later` | yes |

The corpus covers all four provider lanes (`issue_or_work_item`,
`review_decision`, `incident_handoff`, `publish_later`) and all four account
profiles (`connected`, `mirror_only`, `offline`, `enterprise_managed`).

## 2 What each drill class proves

- **`board_project_remap`** — a detected board/project remap holds the action as
  a local draft and surfaces a visible review of the new target; the mapping is
  never silently re-pointed.
- **`stale_token`** — a stale credential drops the action to a local draft with
  a reconnect path; the session is never shown as writable and the queued draft
  is retained.
- **`installation_grant_withdrawal`** — withdrawing the installation grant
  invalidates the mapping and drops the queued evidence to a read-only-blocked
  local draft with an admin path; the draft is not lost.
- **`policy_locked_mapping`** — a managed policy lock refuses an attempted remap
  and holds the action as a local draft with an admin path rather than silently
  re-pointing it.
- **`offline_capture`** — offline capture queues the action for publish-later
  with evidence retained; the session never appears writable while offline.
- **`browser_blocked_handoff`** — a blocked system-browser handoff degrades to a
  truthful fallback and a local draft; the limited-scope session is never shown
  as writable.
- **`publish_later_replay`** — a publish-later-only grant replays the queued
  action after reconnect with the queued draft preserved across replay.
- **`queued_draft_export_import`** — a queued draft is exported and re-imported
  with the exact identity triple preserved and no raw credentials in the packet.

## 3 Identity continuity (fail closed, never silent)

Every drill echoes its `provider_id` / `account_id` / `mapping_id` triple
verbatim through three durable surfaces and asserts no raw credentials are
present:

| Surface | Asserted by |
| --- | --- |
| Support export | `continuity.support_export` + `expected.identity_survives_support_export` |
| Activity-center reopen | `continuity.activity_center_reopen` + `expected.identity_survives_activity_center_reopen` |
| Restart / restore | `continuity.restart_restore` + `expected.identity_survives_restart_restore` |

The validator fails the lane when any surface drops or rewrites the triple, when
a continuity surface carries raw credentials, when a narrowed/stale session is
not fail-closed, when a queued draft is not retained, or when a mapping change
is not surfaced for review.

## 4 Verification

```sh
scripts/ci/run_provider_mapping_corpus.sh
```

The script runs the deterministic corpus validator
(`ci/check_provider_mapping_corpus.py`) and, when a Cargo toolchain is
available, re-validates the seeded beta page via
`cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta -- validate`.
For per-PR smoke coverage the validator runs on its own in well under a second;
nightly runs add the Cargo re-validation.
