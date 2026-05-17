# M3 beta incident-workspace handoff template

The beta incident-workspace handoff packet is the single escalation
artifact a blocked user, support intake, and security triage read
without a separate private template. Every emitted packet pins:

- the **workspace identity** that preserves the user's authored
  files;
- the **target** (exact-build identity and deployment profile);
- the **degraded state** that opened the escalation;
- one or more typed **findings** (Project Doctor, extension bisect,
  safe-mode profile, crash envelope, records governance, runtime
  replay);
- one or more **evidence artifacts** with an explicit **custody
  class** so local-only artifacts, managed copies, and held records
  are never silently merged;
- one or more **recovery options** the user still has;
- a **claim state** block carrying the closed downgrade tokens the
  scenario corpus's drill harness propagates;
- a pinned **privacy baseline** (`raw_private_material_excluded =
  true`, `ambient_authority_excluded = true`).

The implementation lives in
[`crates/aureline-support/src/incident_workspace_beta/mod.rs`](../../../crates/aureline-support/src/incident_workspace_beta/mod.rs).
The JSON-schema boundary lives at
[`schemas/support/incident_workspace_beta_packet.schema.json`](../../../schemas/support/incident_workspace_beta_packet.schema.json).
The protected fixture corpus lives at
[`fixtures/support/m3/incident_packets/`](../../../fixtures/support/m3/incident_packets/).
The reviewer-facing artifact summary lives at
[`artifacts/support/m3/incident_workspace_packet.md`](../../../artifacts/support/m3/incident_workspace_packet.md).
The protected drill test lives at
[`crates/aureline-support/tests/incident_workspace_beta_packet.rs`](../../../crates/aureline-support/tests/incident_workspace_beta_packet.rs).

## What this row owns

- The closed `HandoffConsumerClass` vocabulary
  (`support_intake_only`, `security_triage_only`,
  `support_intake_and_security_triage`) that lets one packet route to
  support, security, or both consumers without separate private
  templates.
- The closed `EvidenceCustodyClass` vocabulary that distinguishes
  local-only artifacts, managed copies, held records (legal hold,
  security hold), exported-to-support artifacts, and
  withheld-pending-user-review artifacts.
- The closed `DegradedStateClass`, `FindingClass`,
  `EvidenceArtifactClass`, `RecoveryOptionClass`, and
  `ClaimDowngradeToken` vocabularies that bound every row the packet
  emits.
- The `IncidentWorkspaceBetaPacketCorpus` loader and the
  `validate_packet_record` entry point that refuse a packet missing
  required fields, dropping the user-authored-files baseline,
  admitting raw private material, or skipping the
  `held_record_blocks_export` / `managed_copy_pending_admin_review`
  downgrade tokens when their custody classes are attached.

## Acceptance and how this row meets it

- **Incidents can be handed off with one packet that names
  workspace, target, degraded state, findings, and evidence
  artifacts.** Every emitted packet pins
  `workspace_identity`, `target`, `degraded_state`, `findings`, and
  `evidence_artifacts` as required fields; the validator refuses a
  packet missing any of them.
- **Security and support consume the same packet without separate
  private templates.** The `handoff_consumer_classes` array admits
  `support_intake_only`, `security_triage_only`, and the joint
  `support_intake_and_security_triage` value. The joint fixture at
  `fixtures/support/m3/incident_packets/joint_security_support_held_record.yaml`
  proves the joint lane is bound to the same packet shape both
  consumers read.
- **The packet distinguishes local-only artifacts, managed copies,
  and held records.** Every evidence artifact row carries a closed
  `EvidenceCustodyClass`. The validator refuses a packet that
  attaches a held record without the matching
  `held_record_blocks_export` downgrade token and a packet that
  attaches a managed copy without the
  `managed_copy_pending_admin_review` downgrade token, so the
  custody truth and the claim-state truth cannot diverge silently.

## Failure-drill posture

The validator fails closed:

- A packet whose `workspace_identity.preserves_user_authored_files`
  is `false` is refused
  (`packet.workspace_identity.preserves_user_authored_files`).
- A packet whose
  `privacy_baseline.raw_private_material_excluded` or
  `privacy_baseline.ambient_authority_excluded` is `false` is
  refused.
- A packet whose `findings`, `evidence_artifacts`, or
  `recovery_options` lists are empty is refused.
- A packet whose `handoff_consumer_classes` names a security route
  but omits `open_security_private_triage` from `recovery_options`
  is refused (`packet.recovery_options.security_route_missing`).
- A packet that attaches a held record but drops
  `held_record_blocks_export` from `claim_state.downgrade_tokens` is
  refused
  (`packet.claim_state.held_record_token_required`).
- A packet that attaches a managed copy but drops
  `managed_copy_pending_admin_review` from
  `claim_state.downgrade_tokens` is refused
  (`packet.claim_state.managed_copy_token_required`).
- A packet whose `references.doc_ref`,
  `references.schema_ref`, or
  `references.scenario_corpus_doc_ref` does not pin the canonical
  paths is refused.

## Joint security/support handoff

When `handoff_consumer_classes` includes
`security_triage_only` or `support_intake_and_security_triage`,
the packet MUST list `open_security_private_triage` as a recovery
option. This guarantees the security review lane has a typed entry
point bound to the same packet the support intake reads. The joint
fixture also demonstrates that:

- Held records do not embed raw payload bytes; the security triage
  lane opens against the held record by reference under its custody
  class.
- The records-governance packet ref pins the legal hold so support
  and security share one chain-of-custody truth.
- The support-bundle preview remains reopenable so the user reviews
  the redacted rows that are not under hold.

## First consumers

- The `aureline-support` `incident_workspace_beta` module is the
  canonical loader for the protected fixture corpus and the
  validation entry point used by support intake and security triage.
- The M3 drill harness report at
  [`artifacts/support/m3/drill_harness_report.md`](../../../artifacts/support/m3/drill_harness_report.md)
  shares the closed claim-downgrade vocabulary so the same red /
  yellow / stale signals show up across the scenario corpus and the
  incident-workspace beta packet.
- The alpha
  [`incident_workspace_contract.md`](../../ops/incident_workspace_contract.md)
  remains the source of truth for the operational incident-workspace
  record; this beta packet composes with it by reference rather than
  re-flattening the operational shape.

## Related contracts

- [Support scenario corpus](support_scenario_corpus.md) — the seven
  beta lane scenarios the packet's findings and evidence reference.
- [Support intake contract](../support_intake_and_escalation_contract.md)
  — the support intake side of the handoff.
- [Recovery ladder alpha](../recovery_ladder_alpha.md) — the parent
  recovery contract every beta packet plugs into.
- [Records governance beta](records_governance_beta.md) — the
  chain-of-custody packet the held-record lane composes with.

## Out of scope for this row

- Hosted ticket intake, cross-tenant case management, or upload
  transport. The packet is the metadata-safe escalation artifact;
  transport stays with the support-export pipeline.
- Live mutation of the workspace, the extension marketplace state,
  the credential store, or any held record. Recovery options route
  the user to the owning beta-lane evaluator, never to a hidden
  destructive reset.
- Live measurement of escalation latency. The alpha diagnosis-latency
  scorecard remains the source of truth for the alpha latency lane.
