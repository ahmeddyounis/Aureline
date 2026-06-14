# M5 Stability Verdicts And Quarantine Records

- Packet: `stability-verdict-quarantine:stable:0001`
- Label: `M5 Stability Verdicts And Quarantine Records`
- Verdicts: 6 (1 imported-only) across 5 / 8 states
- Quarantines: 3 (2 active, 1 expired-reopened)
- Readiness: 4 row(s) fail readiness (gate blocked)

## Stability verdicts

- **verdict:stable:auth** [stable] confidence `high` → readiness `no_impact`, release `informational_recovered`
  - subject `test:auth::login_ok` (concrete_invocation), provenance `local_authoritative`
  - window: 8 observed (8 passed / 0 failed / 0 inconclusive)
- **verdict:flaky:checkout** [confirmed_flaky] confidence `moderate` → readiness `narrows_claim`, release `claim_narrowing_required`
  - subject `test:checkout::pay[*]` (parameterized_template), provenance `local_authoritative`
  - window: 10 observed (7 passed / 3 failed / 0 inconclusive)
- **verdict:quarantined:integration** [quarantined] confidence `moderate` → readiness `fails_readiness`, release `release_blocking`
  - subject `test:integration::sync[case-3]` (concrete_invocation), provenance `remote_authoritative`
  - window: 12 observed (5 passed / 5 failed / 2 inconclusive)
  - quarantine `quarantine:active:integration`
- **verdict:quarantined:legacy** [quarantined] confidence `low` → readiness `fails_readiness`, release `release_blocking`
  - subject `test:legacy::migrate` (concrete_invocation), provenance `local_authoritative`
  - window: 9 observed (2 passed / 6 failed / 1 inconclusive)
  - quarantine `quarantine:expired:legacy`
- **verdict:imported:smoke** [imported_only_unverified] confidence `low` → readiness `narrows_claim`, release `claim_narrowing_required`
  - subject `test:imported::smoke` (concrete_invocation), provenance `imported_read_only`
  - window: 3 observed (0 passed / 0 failed / 3 inconclusive)
- **verdict:stale:nightly** [stale_evidence] confidence `insufficient_evidence` → readiness `narrows_claim`, release `claim_narrowing_required`
  - subject `test:nightly::soak` (concrete_invocation), provenance `local_authoritative`
  - window: 4 observed (4 passed / 0 failed / 0 inconclusive)

## Quarantine records

- **quarantine:active:integration** [quarantine / active] reason `reproduced_flaky` owner `owner:integration-team`
  - expires `2026-12-01T00:00:00Z` restore `stable_window_required` → readiness `fails_readiness`, release `release_blocking`
- **quarantine:expired:legacy** [quarantine / expired_reopened] reason `known_failing` owner `owner:platform-team`
  - expires `2026-03-01T00:00:00Z` restore `manual_review_only` → readiness `fails_readiness`, release `release_blocking`
  - reopened attempt `attempt:legacy:reopen`
- **quarantine:mute:imported** [mute / active] reason `imported_incomparable` owner `owner:release-eng`
  - expires `2026-09-01T00:00:00Z` restore `owner_sign_off` → readiness `narrows_claim`, release `claim_narrowing_required`
