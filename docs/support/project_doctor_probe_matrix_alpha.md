# Project Doctor alpha probe matrix

This matrix publishes the first executable Project Doctor alpha probe lane.
The runtime lives in
[`/crates/aureline-doctor/src/probes`](../../crates/aureline-doctor/src/probes)
and the protected cases live in
[`/fixtures/support/project_doctor_alpha`](../../fixtures/support/project_doctor_alpha).

The alpha probes consume existing surface-owned evidence. They do not open
files, run repository code, clear indexes, mutate trust, touch credentials,
reattach routes, or replay restore work. Any mutation is represented as a
repair preview, recovery path, runbook handoff, support bundle ref, or
escalation packet ref.

## Runtime contract

Each scenario is a `project_doctor_alpha_probe_scenario` and each result is a
`project_doctor_alpha_finding`. A finding is valid only when it carries:

- one stable finding code and rule id;
- the probe id and probe version that emitted it;
- one alpha failure family;
- severity, confidence, and diagnosis posture;
- at least one redaction-safe evidence ref;
- a read-only attestation with all mutating side effects forbidden; and
- either an exact recovery path ref or an escalation packet ref.

The support/export projection is
`project_doctor_alpha_support_export`; it contains only finding ids, finding
codes, family tokens, probe versions, evidence refs, support bundle refs, and
recovery/escalation refs.

## Alpha family coverage

| Alpha family | Probe id | Bounded finding | Evidence owner | First recovery or escalation path |
|---|---|---|---|---|
| `entry_open` | `doctor.probe.entry.open_readiness` | `doctor.finding.entry_open.target_unavailable` | admission checkpoint / recent-work intent | `recovery.entry.locate_or_open_minimal` |
| `toolchain_detection` | `doctor.probe.toolchain.detection` | `doctor.finding.toolchain_missing_required_component` | execution-context resolver | `repair.preview.execution_context.reresolve` |
| `search_index_readiness` | `doctor.probe.search.index_readiness` | `doctor.finding.search_index.readiness_stalled` | search indexed-lane state and activity row | `repair.preview.search_index.rebuild` |
| `trust_policy` | `doctor.probe.trust.policy_gate` | `doctor.finding.trust_policy_blocked` | restricted-mode trust/policy packet | `repair.preview.trust.approval_review` |
| `git_baseline` | `doctor.probe.git.baseline_status` | `doctor.finding.git_baseline.repository_unavailable` | Git status snapshot | `recovery.git.open_baseline_details` |
| `provider_auth` | `doctor.probe.provider.auth_state` | `doctor.finding.provider_auth.credential_expired` | credential-state and provider registry records | `repair.preview.provider_auth.renew_handle` |
| `restore_continuity` | `doctor.probe.restore.continuity` | `doctor.finding.restore_continuity.replay_blocked` | restore hydration / provenance records | `recovery.restore.open_without_replay` |

## Proof path

Run:

```sh
cargo test -p aureline-doctor
cargo test -p aureline-support project_doctor
```

The first command validates fixture coverage, read-only safety, exact
recovery/escalation refs, and support/export safety. The second command proves
the support crate consumes the runtime projection without scraping UI text or
copying the finding vocabulary.
