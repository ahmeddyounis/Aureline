# Docs Authoring Suggestions and Stale-Link / Stale-Example Review (open-raw / open-source escapes)

- Packet: `packet:m5:docs_authoring_review:retry_backoff_guide`
- Session: docs authoring + review: the networking retry/backoff guide
- Promotion: `stable` (0 findings)
- Items: 3 | Degradations: 1

## Items

- [authoring_suggestion] `item:authoring_suggestion:retry_backoff_guide_intro` (Authoring suggestion: tighten the retry/backoff intro) — trust `first_party_authored` — first_party_doc / exact_build_match / authoritative_live / local / high
  - Suggestion: apply `apply_available` (trigger `manual_authoring`)
  - Review: [fresh_ok/advisory]
  - Captured/live: live | Cited: true
  - Escapes: open-raw `open-raw:docnode:project:guides/retry_with_backoff#intro` / open-source `open-source:repo:docs/guides/retry_with_backoff.md`
- [stale_link_review] `item:stale_link_review:retry_backoff_runbook_link` (Stale-link review: the runbook link redirects) — trust `signed_docs_pack` — signed_docs_pack / compatible_minor_drift / warm_cached / imported_pack / medium
  - Suggestion: apply `preview_required` (trigger `broken_link_detected`)
  - Review: [stale_link_redirected/advisory]
  - Captured/live: live | Cited: true
  - Escapes: open-raw `open-raw:docnode:project:guides/retry_with_backoff#runbook` / open-source `open-source:pack:ops/runbooks/retry_backoff_runbook.md`
- [stale_example_review] `item:stale_example_review:retry_backoff_example` (Stale-example review: the backoff example re-validated) — trust `first_party_authored` — first_party_doc / exact_build_match / authoritative_live / local / high
  - Suggestion: apply `suggestion_only` (trigger `stale_example_detected`)
  - Review: [fresh_ok/advisory]
  - Captured/live: live | Cited: true
  - Escapes: open-raw `open-raw:docnode:project:guides/retry_with_backoff#example` / open-source `open-source:repo:crates/aureline-net/src/retry.rs`

## Degradations

- [link_checker_offline/advisory]: the live link checker was offline for one external host; the redirected link verdict is served from the last snapshot
