# Runtime replay packs

Support- and partner-facing envelope that lets reviewers reopen
transcripts, logs, artefacts, and context for a single runtime evidence
artefact (task event, test attempt, debug session, runtime trace) without
hand-building a log bundle. The pack carries one canonical fidelity label
and one canonical reopen decision so reviewer dashboards and CLI surfaces
quote tokens rather than free-form prose.

## Why a pack, not a pile of logs

A runtime evidence packet from
[`/docs/runtime/m3/evidence_packets.md`](../../runtime/m3/evidence_packets.md)
already carries lane, target, toolchain, capsule, policy, and trust truth
plus a replay comparator outcome. Support, design partners, and incident
flows need one extra layer: *given that comparator outcome and the
captured action's privilege class, what is the reviewer authorised to do*?
A pack joins:

- the underlying `RuntimeEvidencePacket`,
- the underlying `RuntimeEvidenceReplayComparison`,
- a closed set of opaque artefact references (transcript, runtime log,
  artifact blob, evidence packet, context provenance),
- a closed [`ReplaySubjectPrivilegeClass`] (`read_only`, `mutating`,
  `privileged`), and
- a derived [`ReplayFidelityClass`] + [`ReplayReopenDecisionClass`].

## Closed fidelity vocabulary

Reviewers and dashboards quote these tokens verbatim:

| Fidelity class | Permits runtime replay | Source comparator outcome |
|----------------|------------------------|----------------------------|
| `exact`        | yes (subject to privilege) | `compatible_replay` |
| `compatible`   | yes (subject to privilege) | `compatible_minor_drift` |
| `layout_only`  | no                         | `incompatible_capsule_drift`, `incompatible_policy_epoch_regressed`, `incompatible_toolchain_changed`, `incompatible_trust_state_downgraded` |
| `evidence_only`| no                         | `incompatible_target_id_changed`, `incompatible_target_class_changed`, `incompatible_redaction_class`, `unknown_requires_review` |

## Closed reopen-decision vocabulary

| Decision | Permits live rerun | Notes |
|----------|---------------------|-------|
| `allow_replay`           | yes | Read-only subject with exact or compatible fidelity. |
| `allow_inspect_no_rerun` | no  | Reviewer may open transcripts/logs/artefacts/context but the runtime MUST NOT silently fire the captured action. |
| `allow_evidence_only_view` | no | Only the evidence record itself is shown. Transcripts/logs MAY render but cannot be reopened on the live target. |
| `blocked`                  | no | Comparator could not classify the replay context, or the redaction posture is not metadata-safe. Reviewer must intervene before any reopen. |

## Privilege gating

A pack carries the captured-action privilege class. Mutating and
privileged subjects can never resolve to `allow_replay`, regardless of
fidelity — the reopen decision is downgraded to
`allow_inspect_no_rerun`. This is how the pack guarantees "no replay or
reopen flow silently reruns a privileged or mutating action".

Quick join table for the read-only subject:

| Fidelity \ Privilege | `read_only` | `mutating` | `privileged` |
|----------------------|-------------|------------|--------------|
| `exact`              | `allow_replay` | `allow_inspect_no_rerun` | `allow_inspect_no_rerun` |
| `compatible`         | `allow_replay` | `allow_inspect_no_rerun` | `allow_inspect_no_rerun` |
| `layout_only`        | `allow_inspect_no_rerun` | `allow_inspect_no_rerun` | `allow_inspect_no_rerun` |
| `evidence_only`      | `allow_evidence_only_view` | `allow_evidence_only_view` | `allow_evidence_only_view` |

When the underlying comparator returned `unknown_requires_review` or
`incompatible_redaction_class`, every cell collapses to `blocked`.

## Artefact references

The pack never embeds raw bytes. Each artefact entry carries one closed
[`ReplayPackArtefactClass`] token (`transcript_ref`, `runtime_log_ref`,
`artifact_blob_ref`, `evidence_packet_ref`, `context_provenance_ref`) plus
an opaque reference (id, hash digest, or URI) that the source support
export resolves back to bytes. Reviewers verify a pack is well-formed by
checking `covers_required_artefact_classes` — every pack carries
transcript, runtime log, evidence packet, and context provenance refs.
The `artifact_blob_ref` slot is optional because some lanes produce no
artefact blob.

## Redaction guarantee

Replay packs inherit the underlying evidence packet's
`metadata_safe_default` redaction class. The artefact entries carry opaque
refs only; the seeded support export round-trips through serde with no
raw secret markers (`BEARER`, `AWS_SECRET_ACCESS_KEY`, `SSH_PRIVATE_KEY`,
`LD_LIBRARY_PATH`) in its JSON form.

## Boundary

- Cross-tool schema: [`/schemas/runtime/runtime_replay_pack.schema.json`](../../../schemas/runtime/runtime_replay_pack.schema.json)
- Closed vocabularies: [`/artifacts/runtime/m3/replay_packets/closed_vocabularies.yaml`](../../../artifacts/runtime/m3/replay_packets/closed_vocabularies.yaml)
- Checked-in fixtures: [`/fixtures/runtime/m3/replay_packets/`](../../../fixtures/runtime/m3/replay_packets/)
- Implementation: [`crates/aureline-support/src/runtime_evidence/mod.rs`](../../../crates/aureline-support/src/runtime_evidence/mod.rs)
- Integration test: [`crates/aureline-support/tests/runtime_replay_packs.rs`](../../../crates/aureline-support/tests/runtime_replay_packs.rs)
- First consumer: [`crates/aureline-shell/src/runtime/replay_pack.rs`](../../../crates/aureline-shell/src/runtime/replay_pack.rs)

## Seeded scenarios

The `seeded_runtime_replay_pack_support_export` builder reproduces these
packs byte-for-byte:

1. `local_task_exact_read_only` — local task event; replay context fully
   matches; read-only captured action. Resolves to fidelity `exact`,
   reopen `allow_replay`.
2. `local_test_compatible_read_only` — local test attempt; replay context
   advances the policy epoch cleanly; read-only test attempt. Resolves to
   fidelity `compatible`, reopen `allow_replay`.
3. `container_debug_layout_only_mutating` — devcontainer debug session;
   replay context drifted on the environment capsule; mutating debug
   launch. Resolves to fidelity `layout_only`, reopen
   `allow_inspect_no_rerun`.
4. `managed_runtime_layout_only_privileged` — managed-workspace runtime
   evidence; replay context downgraded trust state; privileged managed
   dispatch. Resolves to fidelity `layout_only`, reopen
   `allow_inspect_no_rerun`.

## Acceptance

- Evidence packs can reopen transcripts, logs, artifacts, and context with
  `exact`, `compatible`, `layout_only`, or `evidence_only` fidelity labels.
- No replay or reopen flow silently reruns a privileged or mutating
  action: mutating and privileged subjects are gated to
  `allow_inspect_no_rerun` regardless of fidelity.
- Support and partner escalation happens from one bounded support-export
  bundle whose plaintext panel quotes the closed fidelity / privilege /
  reopen-decision / artefact-class tokens directly.
