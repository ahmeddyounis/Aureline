# Approver matrices and review-pack summaries

- Packet: `m5-approver-review-pack-controls:stable:0001`
- Surface: `M5 approver matrices and review-pack summaries: requirement source, satisfied-pending-waived-expired state, local-versus-provider evaluation parity, suppressed-check visibility, and freshness truth across claimed governed surfaces`
- Approver matrix rows: 6 (2 waived or expired)
- Review-pack summaries: 5 (4 not provider-authoritative)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Approver matrix rows

- **security-team** — requirement `branch_protection_rule`, state `satisfied`, parity `provider_authoritative`, evidence `provider_approval_record`
- **api-platform-team** — requirement `codeowners_rule`, state `satisfied`, parity `provider_authoritative`, evidence `ci_check_run`
- **release-review-role** — requirement `review_policy_rule`, state `pending`, parity `local_only`, evidence `local_evaluation_record`
- **qa-review-role** — requirement `manual_review_request`, state `pending`, parity `ci_only`, evidence `ci_check_run`
- **governance-owner-role** — requirement `review_policy_rule`, state `waived`, parity `not_evaluated_here`, evidence `waiver_record`
- **unassigned-review-role** — requirement `unresolved`, state `expired`, parity `stale_relative_to_head`, evidence `no_evidence_link`

## Review-pack summaries

- **pack:digest/release-provider** — base `base:main@a1b2c3` → head `head:feature@d4e5f6`, parity `provider_authoritative`, 1 suppressed check(s)
- **pack:digest/local-eval** — base `base:main@a1b2c3` → head `head:feature@d4e5f6`, parity `local_only`, 1 suppressed check(s)
- **pack:digest/ci-report** — base `base:main@a1b2c3` → head `head:feature@d4e5f6`, parity `ci_only`, 1 suppressed check(s)
- **pack:digest/not-evaluated** — base `base:main@a1b2c3` → head `head:feature@d4e5f6`, parity `not_evaluated_here`, 1 suppressed check(s)
- **pack:digest/stale** — base `base:main@a1b2c3` → head `head:feature@d4e5f6`, parity `stale_relative_to_head`, 0 suppressed check(s)
