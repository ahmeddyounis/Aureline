# Policy fixtures

Seed fixtures for policy-simulation diffs, remembered-decision
narrowing, waiver-expiry drift, and timezone-aware chronology bars.
These files are the policy-lane companion to the governed-record schema
fixtures under
[`/fixtures/governance/record_state_examples/`](../governance/record_state_examples/).

Files:

- [`simulation_diff_manifest.yaml`](./simulation_diff_manifest.yaml)
  — machine-readable case roster covering grant-to-deny, narrower-
  scope carry-forward, expired remembered decisions, future-effective
  policy changes, legal-hold interception, and audit-export chronology.
- [`chronology_bar_cases/`](./chronology_bar_cases/)
  — reviewer-facing chronology-bar rows preserving effective time,
  display timezone, actor identity, and ordering keys for later support
  and export packets.
- [`explain_and_diff_cases/`](./explain_and_diff_cases/)
  — admin-policy artifact and signed bundle-cache examples covering
  offline continuity, expired-bundle safe-default degrade, mirror import,
  and local policy-decision reconstruction.

Companion documentation:

- [`/docs/verification/policy_simulation_packet.md`](../../docs/verification/policy_simulation_packet.md)
  — narrative verification packet defining the input/output objects,
  severity labels, dashboard joins, and chronology-bar requirements.
- [`/artifacts/policy/waiver_expiry_dashboard_contract.yaml`](../../artifacts/policy/waiver_expiry_dashboard_contract.yaml)
  — dashboard field registry and drift-detection contract.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../../docs/policy/admin_policy_and_bundle_cache_contract.md)
  — local admin-policy artifact, signed bundle-cache, precedence,
  safe-default, and explain/export contract.
