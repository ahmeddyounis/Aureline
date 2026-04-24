# Public-proof packet fixtures

Worked fixtures for the public-proof packet contract.

Each fixture is one record conforming to
`schemas/qe/public_proof_packet.schema.json`. Fixtures carry only
opaque ids, opaque refs, monotonic placeholder timestamps, and
redaction-aware labels — no raw URLs, raw hostnames, raw account
handles, raw project identifiers, or raw credential material.

Every scoreboard family seeded in
`artifacts/qe/workflow_bundle_ids.yaml#scoreboard_families` has at
least one fixture here so the contract is exercised end-to-end
without running the product.

## Fixtures

| File | Scoreboard family | Packet shape | Result class |
|---|---|---|---|
| `bootstrap_entry_parity_ts_web_first_useful_edit.json` | `bootstrap_and_entry_parity_scoreboard` | `bootstrap_entry_parity_packet` | `pass_full_proof` |
| `migration_fidelity_ts_web_dry_run.json` | `migration_fidelity_scoreboard` | `migration_fidelity_packet` | `pass_with_known_limit` |
| `task_run_test_debug_parity_python_pytest.json` | `task_run_test_debug_parity_scoreboard` | `task_run_test_debug_parity_packet` | `narrow_claim_before_publish` |
| `extension_bridge_parity_python_package_manager.json` | `extension_or_package_bridge_parity_scoreboard` | `extension_or_package_bridge_parity_packet` | `retest_pending` |
| `workflow_bundle_archetype_proof_rust_self_host.json` | `workflow_bundle_or_archetype_proof_scoreboard` | `workflow_bundle_or_archetype_proof_packet` | `pass_on_narrow_scope` |
| `benchmark_public_proof_ts_web_head_to_head.json` | `benchmark_and_public_proof_packet_scoreboard` | `benchmark_public_proof_packet` | `pass_full_proof` |
| `docs_support_copy_alignment_ts_web_drift_detected.json` | `docs_known_limits_support_copy_alignment_scoreboard` | `docs_known_limits_support_copy_alignment_packet` | `fail_claim_blocked` |

## What each fixture exercises

- **Bootstrap / entry parity** — the first-useful-edit no-account lane
  on the TypeScript launch wedge; `no_limits_declared`; docs / help
  match exact build.
- **Migration fidelity** — a dry-run import against the reference
  workspace with a scope-caveat known limit; narrows to
  `pass_with_known_limit`.
- **Task / run / test / debug parity** — Python pytest parity row that
  narrows to `narrow_claim_before_publish` with a
  `persona_workflow_floor_broken` active downgrade reason.
- **Extension / package bridge parity** — Python package-manager bridge
  row pending retest; carries `compatibility_row_degraded` and
  `compatibility_report_missing_or_stale` active downgrades.
- **Workflow-bundle / archetype proof** — Rust self-host archetype row
  narrowed to `pass_on_narrow_scope` (self-host wording only).
- **Benchmark / public-proof** — TypeScript wedge head-to-head
  benchmark with a `competitor_settings` block and
  `public_head_to_head_comparison` posture.
- **Docs / known-limits / support-copy alignment** — TypeScript wedge
  docs drift detected, packet fails and blocks claim projection with
  `docs_version_match_unmet` active.
