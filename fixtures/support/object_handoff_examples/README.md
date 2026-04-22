# Object handoff examples

These fixtures are worked object-specific issue and support handoff
records that exercise the governed contract in
[`/schemas/support/object_handoff_packet.schema.json`](../../../schemas/support/object_handoff_packet.schema.json).

## Route records

| Fixture | Key behavior shown |
|---|---|
| [`public_issue_tracker_supportability.json`](./public_issue_tracker_supportability.json) | public supportability route using a browser-reviewed community/public issue destination |
| [`official_support_portal_private_partner.json`](./official_support_portal_private_partner.json) | private partner/support route using the official support portal with device-code fallback |
| [`security_private_triage.json`](./security_private_triage.json) | trust-sensitive private triage route that stays local/incident-linked until explicit security escalation |
| [`docs_feedback_public_issue.json`](./docs_feedback_public_issue.json) | docs-feedback route preserving docs-truth posture and public-summary expectation |
| [`local_export_only_follow_up.json`](./local_export_only_follow_up.json) | offline/local-only publish-later route that snapshots the future target explicitly |

## Packet records

| Fixture | Covered source surface / object |
|---|---|
| [`extension_runtime_quarantine.json`](./extension_runtime_quarantine.json) | extension detail page launching a supportability report from a quarantined runtime host |
| [`workflow_bundle_browser_blocked.json`](./workflow_bundle_browser_blocked.json) | workflow bundle detail saving an offline follow-up packet after browser handoff is blocked |
| [`update_candidate_version_mismatch.json`](./update_candidate_version_mismatch.json) | update screen escalation preserving candidate/build/docs truth |
| [`trust_warning_remote_boundary.json`](./trust_warning_remote_boundary.json) | trust warning launching a private security/support handoff with incident linkage |
| [`docs_help_anchor_feedback.json`](./docs_help_anchor_feedback.json) | docs/help feedback preserving source destination descriptor and docs anchor |
| [`migration_import_partial_manual_review.json`](./migration_import_partial_manual_review.json) | migration flow handoff carrying session/outcome/report refs and recovery context |
| [`command_detail_wrong_target.json`](./command_detail_wrong_target.json) | command detail sheet preserving invocation, route, target, and boundary truth |
| [`generated_artifact_lineage_gap.json`](./generated_artifact_lineage_gap.json) | generated artifact row preserving lineage and path-truth cues |
| [`imported_artifact_provenance_conflict.json`](./imported_artifact_provenance_conflict.json) | imported artifact row preserving import-source and partial-fidelity context |
