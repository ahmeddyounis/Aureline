# M5 Issue-Report-Builder-Step Primitive

- Packet: `m5-support-issue-report-builder-step-primitive:stable:0001`
- Label: `M5 issue-report-builder-step primitive: human-readable summary, ordered reproduction steps, selected and excluded evidence classes with their metadata/environment-adjacent/code-adjacent/high-risk data class, redaction posture, derived step posture, per-class local-boundary disposition, and bounded reveal-boundary/preview-local-only/edit-selection/review-redaction/share/export actions with a same-weight local-only preview`
- Support-intake consumers: 5 (5 stable)
- Step postures: share_blocked, no_evidence_selected, redaction_review_required, local_only_preview, ready_to_share
- Step actions: reveal_evidence_boundary, preview_local_only, edit_evidence_selection, review_redaction, share_report, export_step
- Evidence classes: doctor_finding, crash_forensics, repair_transaction, activity_timeline, environment_snapshot, user_note
- Data-risk classes: metadata_only, environment_adjacent, code_adjacent, high_risk
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Support-intake consumers

- **Support Center Builder**: `stable`
  - Owner: Support center builder owner
  - Scope: The support-center report builder renders the shared issue-report builder step so a describe-symptom step carrying a human-readable summary, ordered reproduction steps, and only activity-timeline and environment-snapshot evidence is ready to share with a user note held excluded, and an attach-evidence step selecting code-adjacent Doctor findings and crash forensics under a full-metadata posture forces a redaction review before anything leaves the local boundary
  - Worked steps: 2
    - `builder:support-center:describe-symptom` (`describe_symptom`) → `ready_to_share` (crosses `true`, review `false`, local-only `true`)
    - `builder:support-center:attach-evidence` (`attach_evidence`) → `redaction_review_required` (crosses `false`, review `true`, local-only `true`)
- **Recovery Center Builder**: `stable`
  - Owner: Recovery center builder owner
  - Scope: The recovery-center report builder renders the shared issue-report builder step so a review-redaction step selecting a repair transaction and crash forensics under a credentials-scrubbed posture is ready to share — proving code-adjacent evidence can cross the local boundary once redacted — and a choose-scenario step with nothing selected yet names its no-evidence-selected posture rather than pretending a report is ready
  - Worked steps: 2
    - `builder:recovery-center:review-redaction` (`review_redaction`) → `ready_to_share` (crosses `true`, review `false`, local-only `true`)
    - `builder:recovery-center:choose-scenario` (`choose_scenario`) → `no_evidence_selected` (crosses `false`, review `false`, local-only `true`)
- **Doctor Handoff Builder**: `stable`
  - Owner: Doctor handoff builder owner
  - Scope: The Project Doctor handoff report builder renders the shared issue-report builder step so a confirm-scope step previewed locally only keeps its Doctor finding and activity-timeline evidence on the device until a share is requested, and a submit-or-export step selecting a high-risk user note under a full-metadata posture requires a redaction review before it can leave the local boundary
  - Worked steps: 2
    - `builder:doctor-handoff:confirm-scope` (`confirm_scope`) → `local_only_preview` (crosses `false`, review `false`, local-only `true`)
    - `builder:doctor-handoff:submit-or-export` (`submit_or_export`) → `redaction_review_required` (crosses `false`, review `true`, local-only `true`)
- **Headless / CLI Builder**: `stable`
  - Owner: Headless CLI builder owner
  - Scope: The headless / CLI report builder renders the shared issue-report builder step so a submit-or-export step selecting only an activity timeline under a bodies-omitted posture is ready to share without a desktop UI, and an attach-evidence step whose export is blocked by policy still names its share-blocked posture and keeps the same-weight local-only preview instead of faking a share
  - Worked steps: 2
    - `builder:headless-cli:submit-or-export` (`submit_or_export`) → `ready_to_share` (crosses `true`, review `false`, local-only `true`)
    - `builder:headless-cli:attach-evidence` (`attach_evidence`) → `share_blocked` (crosses `false`, review `false`, local-only `true`)
- **Support Packet Export**: `stable`
  - Owner: Support packet export owner
  - Scope: The support-packet export surface renders the shared issue-report builder step so a submit-or-export step selecting a code-adjacent Doctor finding under a policy-restricted posture is ready to share with the user note and activity timeline held excluded, and a review-redaction step selecting a high-risk user note under a full-metadata posture requires a redaction review — reconstructing the same summary, repro, evidence, and redaction truth a support reviewer reads
  - Worked steps: 2
    - `builder:support-export:submit-or-export` (`submit_or_export`) → `ready_to_share` (crosses `true`, review `false`, local-only `true`)
    - `builder:support-export:review-redaction` (`review_redaction`) → `redaction_review_required` (crosses `false`, review `true`, local-only `true`)
