# about_and_boundary_truth — M3 fixture corpus

Positive and negative fixtures for the public about-destination and
capability-boundary-card contracts published in M3. The corpus is loaded by
the `public_truth_about_and_boundary_fixtures` test in
`crates/aureline-shell/tests/`.

## Layout

```
positive/    Page snapshots that MUST validate.
negative/    Page snapshots that MUST fail validation with a typed reason.
```

Each fixture is a JSON serialization of the
`aureline_shell::public_truth::AboutAndBoundaryTruthPage` record. Adding a
new positive case adds a passing row; adding a negative case asserts the
contract still rejects that drift.

## Drift axes covered

- Destination class — `Official public`, `Official private`, `Community`,
  `Third-party / vendor`.
- Destination role — source repository, issue tracker, discussion / RFC
  forum, governance charter, contributing guide, security and support
  intake, status page, docs/help, release notes/packet, marketplace index,
  upgrade-or-hosted, sponsorship, community-handoff router, local-only
  fallback, mirror/archive.
- Route state — current, redirected, archived, replaced, decommissioned,
  unreachable-probed. Dead routes carry an explicit replacement or
  local-only fallback.
- Account requirement — none, optional, required-for-view, required-for-
  write, required-for-subscribe, required-for-premium-hosted.
- Support prominence — troubleshooting-first, support-first, source-first,
  parity-with-upgrade, below-upgrade. Support-oriented surfaces never sink
  below upgrade.
- Local-only parity — account-optional local parity, local-only-only,
  hosted-only-no-local-fallback, mixed-local-optional-account.
- Boundary card posture — local-open, account-optional local-open,
  managed first-party, self-hosted customer-operated, mirrored offline,
  premium hosted, third-party vendor, community-operated.
- Upgrade-honesty rule — local-path-visible, local-path-hidden-violation
  (rejected), no-local-path-applicable.
