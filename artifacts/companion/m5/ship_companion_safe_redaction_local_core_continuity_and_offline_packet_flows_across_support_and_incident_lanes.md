# Companion-Safe Redaction, Local-Core Continuity, and Offline Packet Flows Across Support and Incident Lanes

- Packet: `redaction-continuity-offline-packet-surface:stable:0001`
- Label: `Companion-Safe Redaction, Local-Core Continuity, and Offline Packet Flows Across Support and Incident Lanes`
- Sections: 4 | Redaction-policy rows: 3 | Continuity rows: 4 | Incident packets: 3 | Support packets: 3
- Exact desktop handoff for every item: yes
- Redaction provable or labeled: yes
- No payload body crosses boundary: yes
- Local incident-packet path available: yes
- Local support-packet path available: yes
- Packet completeness honestly qualified: yes
- Incident packets attributable: yes
- Local-core continuity preserved: yes
- Stale state honestly labeled: yes
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)
- Degraded: none

## Sections

- **redaction_policy**: `beta` / `staged_rollout` [read_only] (matrix lane `companion_notification`)
- **local_core_continuity**: `beta` / `staged_rollout` [read_only] (matrix lane `offboarding_continuity`)
- **offline_incident_packet**: `preview` / `early_access` [read_only] (matrix lane `incident_workspace`)
- **offline_support_packet**: `beta` / `staged_rollout` [read_only] (matrix lane `offboarding_continuity`)

## Redaction policy

- `redact:0001` [companion/notification_body/redacted_summary] (verified: yes) Companion notification bodies cross as a redacted summary; redaction verified (live) → `review_panel` (exact)
- `redact:0002` [support/support_diagnostics/metadata_only] (verified: yes) Support diagnostics cross as metadata only; redaction verified (cached) → `review_panel` (exact)
- `redact:0003` [incident/incident_evidence/reference_only] (verified: no) Incident evidence crosses as an opaque ref; redaction not yet verified; labeled (unknown) → `review_panel` (exact)

## Local-core continuity

- `cont:0001` [local_editing/local_core_authoritative] offline `yes` local_work_preserved `yes` Local editing is authoritative and works fully offline (live) → `review_panel` (exact)
- `cont:0002` [redaction_enforcement/local_core_authoritative] offline `yes` local_work_preserved `yes` Redaction enforcement runs in the local core and works offline (live) → `review_panel` (exact)
- `cont:0003` [offline_packet_replay/local_core_continues_degraded] offline `yes` local_work_preserved `yes` Offline packet replay continues from the local core in a degraded but usable mode (cached) → `review_panel` (exact)
- `cont:0004` [support_export_assembly/requires_provider_continuity] offline `no` local_work_preserved `yes` Provider-assembled support-export pieces require provider continuity; labeled; local work retained (unknown) → `review_panel` (exact)

## Offline incident packets

- `inc:0001` [evidence_timeline/local_ready/complete_verified] redaction `metadata_only` (verified: yes) attribution `yes` Incident evidence timeline assembled locally now; complete, verified; attributable; redaction verified (live) → `review_panel` (exact)
- `inc:0002` [runbook_execution/local_staging/complete_verified] redaction `reference_only` (verified: yes) attribution `yes` Runbook execution packet staging locally from the local core; complete, verified; attributable (cached) → `review_panel` (exact)
- `inc:0003` [incident_export_bundle/requires_provider_assembly/complete_unverified] redaction `reference_only` (verified: no) attribution `no` Provider-assembled incident export bundle; completeness, redaction, and attribution not yet verified; all labeled (unknown) → `review_panel` (exact)

## Offline support packets

- `supp:0001` [diagnostics_bundle/local_ready/complete_verified] redaction `metadata_only` (verified: yes) Diagnostics bundle assembled locally now; complete, verified; redaction verified (live) → `review_panel` (exact)
- `supp:0002` [proof_packet_export/local_staging/complete_verified] redaction `reference_only` (verified: yes) Proof-packet export staging locally from the local core; complete, verified; redaction verified (cached) → `review_panel` (exact)
- `supp:0003` [session_diagnostics/requires_provider_assembly/complete_unverified] redaction `reference_only` (verified: no) Provider-assembled session diagnostics; completeness and redaction not yet verified; labeled (unknown) → `review_panel` (exact)
