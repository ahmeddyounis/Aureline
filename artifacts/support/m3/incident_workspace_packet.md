# M3 beta incident-workspace packet baseline

This artifact is the checked-in reviewer-facing baseline for the M3
beta incident-workspace handoff packet. The packet shape is the
single escalation artifact a blocked user, support intake, and
security triage read without a separate private template.

The packet is projected from the protected fixture corpus at
[`fixtures/support/m3/incident_packets/`](../../../fixtures/support/m3/incident_packets/)
by
[`aureline_support::incident_workspace_beta`](../../../crates/aureline-support/src/incident_workspace_beta/mod.rs)
via the protected integration test at
[`crates/aureline-support/tests/incident_workspace_beta_packet.rs`](../../../crates/aureline-support/tests/incident_workspace_beta_packet.rs).
The reviewer doc lives at
[`docs/support/m3/incident_handoff_template.md`](../../../docs/support/m3/incident_handoff_template.md).
The JSON-schema boundary lives at
[`schemas/support/incident_workspace_beta_packet.schema.json`](../../../schemas/support/incident_workspace_beta_packet.schema.json).

## Packet contract

- `record_kind`: `incident_workspace_beta_packet_record`
- `schema_version`: `1`
- `privacy_baseline.raw_private_material_excluded`: `true`
- `privacy_baseline.ambient_authority_excluded`: `true`
- `workspace_identity.preserves_user_authored_files`: `true`
- `references.doc_ref`: `docs/support/m3/incident_handoff_template.md`
- `references.schema_ref`: `schemas/support/incident_workspace_beta_packet.schema.json`
- `references.scenario_corpus_doc_ref`: `docs/support/m3/support_scenario_corpus.md`

## Custody lanes

| Custody class | Meaning |
|---|---|
| `local_only_artifact` | Artifact stays on the user's host; no managed copy exists. |
| `managed_copy_available` | A managed copy is reachable through the managed admin lane. |
| `held_record_under_legal_hold` | Record is under legal hold; export is governed. |
| `held_record_under_security_hold` | Record is held pending the security triage lane. |
| `exported_to_support_intake` | Artifact has already been exported to support intake. |
| `withheld_pending_user_review` | Artifact is withheld pending explicit user review. |

The validator refuses a packet that attaches a held record without
the `held_record_blocks_export` downgrade token, or a managed copy
without `managed_copy_pending_admin_review`, so the claim-state row
mirrors the custody row instead of drifting silently.

## Handoff consumer classes

| Consumer class | Routing |
|---|---|
| `support_intake_only` | Only the support intake lane reads the packet. |
| `security_triage_only` | Only the security triage lane reads the packet. |
| `support_intake_and_security_triage` | Both lanes read the same packet. |

When the security route is named, the packet MUST list the
`open_security_private_triage` recovery option so the security lane
has a typed entry point bound to the same packet.

## Per-fixture baseline rows

### `safe_mode_crash_loop_local_only.yaml` — local-only after a startup crash loop

- Packet id: `support.m3.incident_packet.safe_mode.crash_loop_local_only`
- Handoff consumers: `support_intake_only`
- Degraded state: `startup_crash_loop_detected`
- Custody classes attached: `local_only_artifact`
- Claim downgrade tokens: (none — every artifact is local-only)
- Recovery options: `enter_safe_mode`, `export_support_bundle`,
  `handoff_to_support`

### `extension_quarantine_managed_copy.yaml` — managed copy reachable through admin lane

- Packet id: `support.m3.incident_packet.extension_quarantine.managed_copy`
- Handoff consumers: `support_intake_only`
- Degraded state: `extension_regression_suspected`
- Custody classes attached: `local_only_artifact`,
  `managed_copy_available`
- Claim downgrade tokens: `managed_copy_pending_admin_review`
- Recovery options: `start_extension_bisect`, `open_repair_preview`,
  `handoff_to_support`

### `joint_security_support_held_record.yaml` — joint lane with held records

- Packet id: `support.m3.incident_packet.joint.security_support_held_record`
- Handoff consumers: `support_intake_and_security_triage`
- Degraded state: `support_export_only_no_remote`
- Custody classes attached: `local_only_artifact`,
  `held_record_under_security_hold`, `held_record_under_legal_hold`
- Claim downgrade tokens: `held_record_blocks_export`
- Recovery options: `handoff_to_support`,
  `open_security_private_triage`, `export_support_bundle`

## How to refresh

1. Run the protected drill-harness test:
   `cargo test -p aureline-support --test incident_workspace_beta_packet`.
2. The test reloads every fixture from disk, re-proves the
   validator, and round-trips the YAML through serde. If a fixture
   changes, re-emit this artifact in the same change so the reviewer
   baseline stays in lockstep with the corpus.
3. Adding a packet requires preserving the three custody lanes
   (local-only, managed copy, held record) so the joint
   security/support contract remains provable.

## Boundaries

- Raw logs, command lines, provider payloads, stack frames, secret
  material, and ambient authority do not cross the packet boundary.
- Held records (legal hold or security hold) are referenced under
  their hold custody class; raw payload bytes are never embedded.
- Hosted ticket intake and upload transport remain out of scope for
  this row; the packet is the metadata-safe escalation artifact, not
  the transport layer.
