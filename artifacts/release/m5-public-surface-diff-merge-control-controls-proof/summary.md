# Public-surface diff cards and merge-control banners

- Packet: `m5-public-surface-diff-merge-control-controls:stable:0001`
- Surface: `M5 public-surface diff cards and merge-control banners: surface class, stability label, schema-or-command delta disclosure, blocker reason, bypass policy, and migration-note continuity across claimed release-bearing changes`
- Public-surface diff cards: 6 (3 stable breaking)
- Merge-control banners: 6 (5 with a current blocker)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Public-surface diff cards

- **Remove the deprecated `aureline verify` command** — stability `stable`, change `removal`, parity `provider_confirmed`, evidence `machine_generated_diff`
- **Tighten the run-config schema to reject unknown keys** — stability `stable`, change `breaking`, parity `machine_generated_local`, evidence `migration_guide`
- **Add an optional `dry_run` field to the SDK plan surface** — stability `beta`, change `compatible`, parity `provider_confirmed`, evidence `compatibility_report`
- **Deprecate the legacy `job.enqueued` message id** — stability `experimental`, change `deprecation`, parity `local_estimate`, evidence `changelog_entry`
- **Clarify the compatibility claim wording without behavior change** — stability `internal`, change `compatible`, parity `stale_relative_to_head`, evidence `no_diff_evidence`
- **Change `aureline run` to fail fast on missing config** — stability `stable`, change `breaking`, parity `not_evaluated_here`, evidence `machine_generated_diff`

## Merge-control banners

- **Merge blocked: a required check is failing** — blocker `required_check_failing`, protection `provider_enforced`, bypass `no_bypass_allowed`, parity `provider_confirmed`
- **Merge blocked: a required review is missing** — blocker `required_review_missing`, protection `ruleset_enforced`, bypass `admin_bypass_allowed`, parity `provider_confirmed`
- **Merge estimate: a branch-protection rule may block** — blocker `branch_protection_rule`, protection `advisory_only`, bypass `emergency_bypass_allowed`, parity `local_estimate`
- **Merge blocked: a ruleset violation was recorded** — blocker `ruleset_violation`, protection `not_configured`, bypass `bypass_used`, parity `stale_relative_to_head`
- **Merge gate not evaluated: a merge conflict may exist** — blocker `merge_conflict`, protection `provider_enforced`, bypass `no_bypass_allowed`, parity `not_evaluated_here`
- **Merge allowed: the provider confirms the gate is clear** — blocker `no_blocker`, protection `provider_enforced`, bypass `no_bypass_allowed`, parity `provider_confirmed`
