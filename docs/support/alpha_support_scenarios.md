# External Alpha Support Scenarios

This document explains the external alpha support-scenario scorecard and
fixture corpus. The machine source is
[`/artifacts/support/diagnosis_latency_scorecard_alpha.yaml`](../../artifacts/support/diagnosis_latency_scorecard_alpha.yaml);
scenario fixtures live under
[`/fixtures/support/seeded_scenarios_alpha/`](../../fixtures/support/seeded_scenarios_alpha/).

## Contract

The scorecard measures diagnosis latency from `support_scenario_started`
to `first_actionable_result_packet_emitted`. The stop event is valid only
when the result packet includes a stable finding code, a result packet ref,
a support bundle ref, exact-build identity, redaction posture, and a safe
next action.

The six alpha families are:

| Family | Fixture | First actionable result |
| --- | --- | --- |
| `first_run` | `first_run_entry_open_target_unavailable.yaml` | Doctor finding for unavailable entry target |
| `search_index` | `search_index_readiness_stalled.yaml` | Doctor finding for stalled search/index readiness |
| `trust_policy` | `trust_policy_denied_capability.yaml` | Doctor finding for denied capability |
| `restore_continuity` | `restore_replay_blocked.yaml` | Doctor finding for blocked restore replay |
| `provider_auth` | `provider_credential_expired.yaml` | Doctor finding for expired provider credential |
| `crash_loop` | `crash_loop_extension_quarantine.yaml` | Recovery decision for safe mode and quarantine |

Each row covers the Python service and TypeScript/JavaScript web launch
wedges. Every row includes a headless Doctor path, and the crash-loop row
also includes the safe-mode Support Center path.

## First Consumer

The first support/export consumer is
[`crates/aureline-support/src/scenario_scorecard/mod.rs`](../../crates/aureline-support/src/scenario_scorecard/mod.rs).
It parses the scorecard and corpus, validates coverage and redaction
rules, then emits two projections from the same rows:

- `support_scenario_scorecard_support_packet`
- `support_scenario_scorecard_dashboard`

Both projections carry `scorecard_id` and `scorecard_row_id`, so support
packets and review dashboards can join back to one scorecard instead of
copying free-form notes.

## Guardrails

The corpus is seed evidence, not a claim that live alpha measurements are
green. Rows use `seeded_pending_measurement` until a measured run supplies
current latency samples. The support packet path remains metadata-safe:
raw credential handles, raw dumps, source files, full shell history, and
raw provider payloads are excluded by default.

No scenario offers a destructive reset shortcut. Repairs or recovery
actions that touch durable state stay behind reviewed preview, checkpoint,
or escalation paths owned by the existing Project Doctor, recovery ladder,
repair transaction, and support bundle contracts.

## Verification

Run:

```sh
cargo test -p aureline-support --test diagnosis_latency_scorecard_alpha
```

This validates scenario-family coverage, the diagnosis-latency window,
support-packet and dashboard joins, exact-build requirements, and default
redaction posture.
