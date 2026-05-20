# Provider account-scope and target-mapping continuity drill corpus

This corpus turns the provider-account/session and board/project mapping truth
proven by the seeded beta page (`fixtures/providers/m3/account_scope_and_mapping/`)
into a regression-resistant **failure / recovery / continuity** lane. Each
fixture is one deterministic drill case that extends a seeded mapping-review
row into a degraded scenario and pins the behaviour CI must keep honest.

Each case validates against
`schemas/providers/provider_account_mapping_drill_case.schema.json` and is
checked by `ci/check_provider_mapping_corpus.py`, run from
`scripts/ci/run_provider_mapping_corpus.sh`.

## What each drill case pins

- **Identity that survives.** The stable provider/account/mapping identity
  triple (`provider_id`, `account_id`, `mapping_id`) is echoed verbatim through
  three durable surfaces — support export, activity-center reopen, and
  restart/restore — so support and reviewer surfaces can always name exactly
  which account acted and which target a mutation would touch.
- **Fail-closed degradation.** Stale, narrowed, read-only, offline, and
  publish-later-only sessions never present as writable; the row degrades to a
  local draft, a queued publish-later item, or a read-only inspection with a
  concrete next-safe action.
- **Durable local work.** Queued drafts, local drafts, queued transitions, and
  evidence attachments stay durable with retry/export available; a queued draft
  never silently vanishes.
- **Visible remap review.** A mapping change is never applied silently; it
  surfaces a visible review before the new target is used.
- **No raw secrets.** No raw access token, raw delegated-token body, raw
  provider payload, or raw provider URL appears in any case or continuity
  surface.

## Required drill classes (one fixture each)

| Drill class | Fixture | Provider lane | Profile |
| --- | --- | --- | --- |
| `board_project_remap` | `board_project_remap_held_for_review.json` | `issue_or_work_item` | `connected` |
| `stale_token` | `stale_token_blocks_live_mutation.json` | `issue_or_work_item` | `mirror_only` |
| `installation_grant_withdrawal` | `installation_grant_withdrawal_invalidates_mapping.json` | `publish_later` | `enterprise_managed` |
| `policy_locked_mapping` | `policy_locked_mapping_blocks_remap.json` | `review_decision` | `connected` |
| `offline_capture` | `offline_capture_queues_incident_handoff.json` | `incident_handoff` | `offline` |
| `browser_blocked_handoff` | `browser_blocked_handoff_offers_fallback.json` | `review_decision` | `connected` |
| `publish_later_replay` | `publish_later_replay_preserves_queue.json` | `publish_later` | `enterprise_managed` |
| `queued_draft_export_import` | `queued_draft_export_import_round_trip.json` | `incident_handoff` | `mirror_only` |

The corpus covers all four provider lanes (`issue_or_work_item`,
`review_decision`, `incident_handoff`, `publish_later`) and all four account
profiles (`connected`, `mirror_only`, `offline`, `enterprise_managed`).

## Companion artifacts

- `corpus_matrix.json` — the enum-only matrix pinning the drill classes, lanes,
  profiles, and degraded states this corpus covers (no labels, no targets).
- `artifacts/providers/m3/account_scope_and_mapping_report.md` — the
  support- and partner-facing report.
- `artifacts/providers/m3/account_scope_and_mapping_continuity_matrix.json` —
  the per-lane continuity packet beta scorecards reference.
- `docs/providers/m3/provider_account_and_mapping_drills.md` — the reviewer
  drills doc.

The seeded beta page these drills extend remains authoritative in
`crates/aureline-provider/src/project_mapping/mod.rs` and is re-validated by the
same CI entry point via
`cargo run -p aureline-provider --bin aureline_provider_target_mapping_beta -- validate`.
