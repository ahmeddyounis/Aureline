# Docs Version/Freshness Findings Fixtures

Each fixture is a case file with a `record_kind` of
`docs_version_freshness_findings_case`, a `scenario` describing what the case
proves, a packet `input`, and an `expect` block naming the derived promotion
state and the validation finding kinds the validator must raise. The integration
test materializes each `input` and asserts the promotion state and expected
findings, so these fixtures pin the guardrails the canonical support export keeps
green.

## baseline_stable.json

The baseline packet certifies `stable`: the controlled version/freshness
vocabulary (exact, nearby, project_specific, mirrored, cached, stale,
policy_blocked, browser_handoff_required) renders with distinct badges and
distinct confidence treatments, carries stale-example and broken-link findings
with suppress/compare/open-current-source actions, and is reused without drift by
every claimed consumer surface.

## cached_shares_exact_confidence_blocks_stable.json

A cached card claims the exact-current confidence treatment. The validator raises
`card_confidence_collapsed` and blocks stable because cached or nearby-version
documentation must never render with the same confidence as exact current
documentation.

## version_mismatch_hidden_blocks_stable.json

A nearby-version card hides the active and viewed versions. The validator raises
`version_disclosure_missing` and blocks stable because a version-mismatch surface
must show both the active code/package version and the viewed docs version.

## broken_link_finding_blocks_stable.json

A finding is raised to blocking severity. The packet blocks promotion, proving
stale-example and broken-link findings are actionable review items that can gate
the stable claim while keeping stable object identity.

## finding_actions_dropped_blocks_stable.json

A finding drops its compare action. The validator raises `finding_actions_missing`
and blocks stable because every finding must preserve its
suppress/compare/open-current-source actions.

## finding_orphan_blocks_stable.json

A finding references a card absent from the packet. The validator raises
`finding_orphan` and blocks stable because every finding must attach to a real
card with stable object identity.

## state_distinction_collapsed_blocks_stable.json

A consumer surface collapses the distinct state badges into one generic info
badge. The validator raises `state_distinction_collapsed` and blocks stable
because browser_handoff_required, cached, mirrored, and project_specific must
never collapse into one badge.

## vocabulary_coverage_incomplete_blocks_stable.json

The cards drop the browser_handoff_required state. The validator raises
`vocabulary_coverage_missing` and blocks stable because the controlled vocabulary
must stay whole and every state must be a real, reachable badge.

## policy_blocked_reason_missing_blocks_stable.json

A policy-blocked card drops its reason. The validator raises `state_reason_missing`
and blocks stable because a policy-blocked or browser-handoff-required state must
name why the answer is not rendered inline.
