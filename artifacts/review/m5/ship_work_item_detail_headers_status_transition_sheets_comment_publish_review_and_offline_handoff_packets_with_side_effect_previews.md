# Work-Item Detail Headers, Transition Sheets, Comment Publish Review, and Offline Handoff Packets

- Packet: `work-item-mutation-review:stable:0001`
- Surface: `M5 work-item mutation review side-effect previews`
- Detail headers: 2
- Transition sheets: 2 (1 blocked)
- Comment publish reviews: 2 (1 deferred or local)
- Offline handoff packets: 2
- Proof freshness SLO: 168 hours (last refresh: 2026-06-12T22:35:00Z)

## Detail headers

- **AUR-241** (`issue_tracker boundary · GitHub Issues`): In progress · Ready for review, owner `A. Rahman`, sync `provider_authoritative · live_authoritative_fresh · published_observed_authoritative`, open `Open externally`
- **AUR-242** (`issue_tracker boundary · GitHub Issues`): Blocked · policy review pending, owner `Unassigned`, sync `policy_blocked · unverifiable_provider_unreachable · draft_only`, open `Open provider settings`

## Transition sheets

- **AUR-241**: `In progress` -> `Ready for review` via `publish_now`; fallback `preserve the local draft on failure`
- **AUR-242**: `Blocked` -> `Triaged locally` via `local_draft`; fallback `policy requires offline capture until workspace trust recovers`

## Comment publish review

- **Publish comment for AUR-241**: `Local draft preserved for review` / `publish now to the provider`; visibility `provider work-item timeline`; fallback `preserve the local draft on failure`
- **Queue comment with transition for AUR-242**: `Queued comment remains local until publish later` / `queue for publish later`; visibility `provider timeline via publish-later queue`; fallback `queue the comment for publish later`

## Offline handoff packets

- **handoff:aur-242:offline**: acceptance `not_submitted_local_capture_only`, drain `captured_pending_drain`, expiry `2026-06-19T22:35:00Z`, target `publish_target:github:aur-242`
- **handoff:aur-242:imported**: acceptance `imported_handoff_evidence_only_no_provider_path`, drain `exported_pending_external_apply`, expiry `2026-06-19T22:35:00Z`, target `publish_target:imported:aur-242`
