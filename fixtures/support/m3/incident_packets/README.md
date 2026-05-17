# M3 beta incident-workspace handoff packet corpus

This corpus seeds the protected fixtures the beta incident-workspace
handoff template consumes. Every fixture mirrors the
[`IncidentWorkspaceBetaPacket`](../../../../crates/aureline-support/src/incident_workspace_beta/mod.rs)
record kind, the JSON-schema boundary at
[`schemas/support/incident_workspace_beta_packet.schema.json`](../../../../schemas/support/incident_workspace_beta_packet.schema.json),
and the reviewer doc at
[`docs/support/m3/incident_handoff_template.md`](../../../../docs/support/m3/incident_handoff_template.md).

The corpus covers the three custody lanes the packet must distinguish
verbatim:

| Custody lane | Fixture | Handoff consumer class |
| ------------ | ------- | ---------------------- |
| Local-only artifacts (no managed copy, no hold) | `safe_mode_crash_loop_local_only.yaml` | `support_intake_only` |
| Managed copy reachable through managed admin | `extension_quarantine_managed_copy.yaml` | `support_intake_only` |
| Held records (legal and security hold) consumed by both lanes | `joint_security_support_held_record.yaml` | `support_intake_and_security_triage` |

Every fixture preserves the same baseline:

- `workspace_identity.preserves_user_authored_files = true`;
- `privacy_baseline.raw_private_material_excluded = true` and
  `privacy_baseline.ambient_authority_excluded = true`;
- one finding row per attributable lane (Project Doctor, extension
  bisect, safe-mode, crash envelope, records governance, runtime
  replay) — the schema requires at least one and forbids empty rows;
- `claim_state.downgrade_tokens` align with the custody classes
  attached: managed copies require `managed_copy_pending_admin_review`
  and held records require `held_record_blocks_export`;
- the reviewer doc and schema refs are pinned verbatim.

Adding a fixture requires preserving the three custody lanes and
keeping the `IncidentWorkspaceBetaPacketCorpus::validate` checks
green; the integration test at
[`crates/aureline-support/tests/incident_workspace_beta_packet.rs`](../../../../crates/aureline-support/tests/incident_workspace_beta_packet.rs)
re-proves every fixture on disk and round-trips the YAML through
serde.
