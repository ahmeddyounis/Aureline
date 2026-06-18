# Docs Suggestion Panel (diff-first proposals)

- Packet: `packet:m5:docs_suggestion_panel:retry_backoff_release`
- Panel: docs suggestion panel: the retry/backoff release docs sweep
- Promotion: `stable` (0 findings)
- Suggestions: 5 | Degradations: 1

## Suggestions

- [readme] `suggestion:readme:retry_backoff_api_contract` (Document the new max_elapsed retry parameter) — target `README → Configuration#configuration` — trigger `api_contract_change`
  - Chips: high / authoritative_live / exact_build_match / local
  - Proposal: `diff_hunks` (1 hunks, +6/-1)
  - Actions: apply `apply_available` | open-evidence `open-evidence:api-contract:retry_with_backoff#max_elapsed` | open-source `open-source:repo:crates/aureline-net/src/retry.rs` | dismiss true | save true
  - Provenance: `first_party_verified` | disposition `pending` | cited true
- [changelog] `suggestion:changelog:retry_backoff_release_entry` (Add a changelog entry for the retry/backoff change) — target `CHANGELOG → Unreleased#unreleased` — trigger `release_metadata_change`
  - Chips: high / authoritative_live / exact_build_match / local
  - Proposal: `new_section_diff` (1 hunks, +3/-0)
  - Actions: apply `apply_available` | open-evidence `open-evidence:release-metadata:next-channel#retry_backoff` | open-source `open-source:repo:artifacts/release/next_channel.yaml` | dismiss true | save true
  - Provenance: `first_party_verified` | disposition `applied` | cited true
- [help] `suggestion:help:retry_backoff_symbol_rename` (Update the renamed with_full_jitter symbol in help) — target `Help → Retry and backoff → Builder API#builder-api` — trigger `symbol_rename`
  - Chips: high / authoritative_live / exact_build_match / local
  - Proposal: `diff_hunks` (2 hunks, +2/-2)
  - Actions: apply `apply_available` | open-evidence `open-evidence:symbol-rename:RetryPolicy::with_jitter` | open-source `open-source:repo:crates/aureline-net/src/retry.rs` | dismiss true | save true
  - Provenance: `first_party_verified` | disposition `pending` | cited true
- [tutorial] `suggestion:tutorial:retry_backoff_failing_example` (Fix the failing step-3 backoff example) — target `Tutorial → Resilient networking → Step 3: add backoff#step-3-add-backoff` — trigger `failing_example`
  - Chips: medium / warm_cached / compatible_minor_drift / local
  - Proposal: `example_replace_diff` (1 hunks, +4/-4)
  - Actions: apply `preview_required` | open-evidence `open-evidence:failing-example:resilient-networking#step-3` | open-source `open-source:repo:crates/aureline-net/examples/backoff.rs` | dismiss true | save true
  - Provenance: `first_party_verified` | disposition `saved_for_later` | cited true
- [help] `suggestion:help:retry_backoff_runbook_link` (Repoint the redirected operations runbook link) — target `Help → Retry and backoff → Operations runbook#operations-runbook` — trigger `broken_link`
  - Chips: medium / warm_cached / compatible_minor_drift / imported_pack
  - Proposal: `link_repoint_diff` (1 hunks, +1/-1)
  - Actions: apply `preview_required` | open-evidence `open-evidence:broken-link:ops/runbooks/retry_backoff_runbook` | open-source `open-source:pack:ops/runbooks/retry_backoff_runbook.md` | dismiss true | save true
  - Provenance: `imported` | disposition `pending` | cited true

## Degradations

- [link_checker_offline/advisory]: the live link checker was offline for one external host; the broken-link suggestion is served from the last snapshot
