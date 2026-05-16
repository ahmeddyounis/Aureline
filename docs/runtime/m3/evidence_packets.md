# Runtime evidence packets

Beta runtime contract that binds task, test, debug, and runtime evidence to
the canonical execution-context provenance object. The packet is the
support, incident, and replay envelope every claimed beta runtime lane
emits so reviewers and partners never have to reconstruct target, toolchain,
or policy truth from free-form logs.

## Why one packet, not four

The task-event lane, the test runner, the debug supervisor, and the trace
replay lane all need the same answer when an evidence artefact lands in a
support bundle: *which execution context produced this row, against which
target, with which toolchain, under which policy epoch, and is it still
compatible with the freshly resolved context?* Forking four envelopes per
lane would let one lane drift its vocabulary while another lags. The
[`RuntimeEvidencePacket`](../../../crates/aureline-runtime/src/provenance/evidence_packet.rs)
record carries one closed `lane` token, one closed `evidence_kind` token,
one redaction-safe
[`ExecutionEventProvenance`](../../../crates/aureline-runtime/src/provenance/mod.rs)
projection, and one subject reference per lane artefact.

## Replay-compatibility comparator

A consumer that wants to replay or compare a captured packet against a
freshly resolved context calls
[`RuntimeEvidencePacket::compare_with_context`]. The comparator emits a
typed [`RuntimeEvidenceReplayComparison`] with a closed
`compatibility` token and a closed `incompatibility_reasons` vocabulary.
Reviewers and shell surfaces quote these tokens verbatim:

| Compatibility class                          | Permits replay | Notes                                                        |
|----------------------------------------------|----------------|--------------------------------------------------------------|
| `compatible_replay`                          | yes            | Target, toolchain, capsule, policy, trust all match.         |
| `compatible_minor_drift`                     | yes            | Capsule and/or policy epoch advanced cleanly (`in_sync`).    |
| `incompatible_target_id_changed`             | no             | Replay target id differs.                                    |
| `incompatible_target_class_changed`          | no             | Replay target class differs.                                 |
| `incompatible_toolchain_changed`             | no             | Toolchain class or identity drift.                           |
| `incompatible_capsule_drift`                 | no             | Capsule drift state regressed or hash diverged while drifting. |
| `incompatible_policy_epoch_regressed`        | no             | Replay policy epoch went backwards.                          |
| `incompatible_trust_state_downgraded`        | no             | Trust state lower than captured packet.                      |
| `incompatible_redaction_class`               | no             | Captured packet's redaction posture is not metadata-safe.    |
| `unknown_requires_review`                    | no             | Comparator could not classify; reviewer must intervene.      |

Every comparison also carries one or more
`incompatibility_reasons`: `target_id_drift`, `target_class_drift`,
`toolchain_class_drift`, `toolchain_id_drift`,
`environment_capsule_hash_drift`,
`environment_capsule_drift_state_regressed`,
`policy_epoch_regressed`, `trust_state_downgraded`,
`redaction_class_unsafe`, or `clean_forward_drift`. The
`clean_forward_drift` reason is recorded only on the
`compatible_minor_drift` outcome so the disclosure remains explicit.

## Redaction guarantee

The packet stamps `redaction_class = metadata_safe_default` and inherits
the redaction-safe properties of the embedded provenance projection.
Reviewers and support flows embed the packet verbatim into bundles without
re-validating that raw env, raw command lines, or unmanaged credentials
were stripped. The seeded scenarios round-trip through serde with no raw
secret markers in their JSON form.

## Boundary

- Cross-tool schema: [`/schemas/runtime/evidence_packet.schema.json`](../../../schemas/runtime/evidence_packet.schema.json)
- Closed vocabularies: [`/artifacts/runtime/m3/evidence_packets/closed_vocabularies.yaml`](../../../artifacts/runtime/m3/evidence_packets/closed_vocabularies.yaml)
- Checked-in fixtures: [`/fixtures/runtime/m3/evidence_packets/`](../../../fixtures/runtime/m3/evidence_packets/)
- Integration test: [`crates/aureline-runtime/tests/evidence_packets_beta.rs`](../../../crates/aureline-runtime/tests/evidence_packets_beta.rs)
- First consumer: [`crates/aureline-shell/src/runtime/evidence_packet.rs`](../../../crates/aureline-shell/src/runtime/evidence_packet.rs)

## Seeded scenarios

Reviewer and partner replays of
[`seeded_runtime_evidence_packet_support_export`](../../../crates/aureline-runtime/src/provenance/evidence_packet.rs)
reproduce these scenarios byte-for-byte:

1. `local_task_compatible` — local-host task event; identical replay
   context. Comparator returns `compatible_replay`.
2. `local_test_policy_advanced_clean` — local-host test attempt against
   policy epoch 2; replay context advances cleanly to policy epoch 3.
   Comparator returns `compatible_minor_drift` with the
   `clean_forward_drift` reason recorded.
3. `container_debug_capsule_drift` — devcontainer debug session captured
   in_sync; replay context advances capsule drift state to
   `manually_diverged` with a divergent hash. Comparator returns
   `incompatible_capsule_drift` with
   `environment_capsule_drift_state_regressed` reason.
4. `managed_runtime_trust_downgraded` — managed-workspace runtime trace
   evidence captured with `trusted`; replay context downgrades trust to
   `restricted`. Comparator returns
   `incompatible_trust_state_downgraded` with `trust_state_downgraded`
   reason.

## Acceptance

- Runtime evidence packets include execution-context refs, target
  identity, toolchain lineage, and policy epoch without leaking secrets.
- Debug, test, and task artefacts can be compared or replayed against
  their original context with explicit incompatibility labels.
- Incident and support workflows quote the closed compatibility and
  reason tokens directly instead of free-form log strings.
