# Fixtures: supportability handoff packets

This directory contains fixture metadata for the `m5_supportability_handoff_packets` packet family.

The canonical full corpus is checked in at:

`artifacts/support/m5/m5-supportability-handoff-packets.json`

It is the one authoritative supportability handoff registry; the typed model and fail-closed handoff /
share gate live in the `aureline-support` crate (`m5_supportability_handoff_packets`).

## Coverage

- **One escalation object, joining every source class.** Across the corpus every component kind is
  exercised — `finding_code`, `repair_id`, `crash_artifact`, `install_advisory_state`,
  `credential_state_descriptor`, `environment_summary`, `precedence_summary`, and
  `restore_provenance_record` — and every data class appears (`metadata`, `diagnostic_summary`,
  `environment_descriptor`, `credential_state`, `crash_artifact_reference`, `user_content_excerpt`). Each
  component carries its `source_ref` and `lineage_ref`, so the exact-build, finding-code, and repair-id
  lineage is preserved.
- **All three handoff modes are exercised:** `local_self_diagnosis` (`local-self-diagnosis-no-upload`),
  `team_share` (`team-share-redacted`, `blocked-user-escalation`), and `formal_support_handoff`
  (`formal-support-handoff`, `policy-locked-export`), each with its own allowed data classes and default
  redaction posture.
- **The required scenarios are all present:** a no-upload local handoff (`local-self-diagnosis-no-upload`),
  a team-share vs formal-support delta (`team-share-redacted` and `formal-support-handoff`, which differ on
  the credential-state descriptor), a policy-locked export (`policy-locked-export`), and a blocked-user
  escalation (`blocked-user-escalation`).
- **The three presentations and four statuses are each exercised:** `ready_to_share`
  (`local-self-diagnosis-no-upload`), `narrowed` (`team-share-redacted`, `formal-support-handoff`,
  `policy-locked-export`), and `send_blocked` (`blocked-user-escalation`); and the statuses
  `ready_to_share`, `redaction_narrowed`, `policy_locked`, and `send_blocked`. All five downgrade reasons
  and all four component dispositions are exercised across the corpus.
- **The gate is exercised in every direction:** one packet is fully ready to share (proving the gate is not
  a blanket flag); a redacted or excluded component narrows a packet and is never hidden; a policy-locked
  data class is withheld and named; a downgraded restore-provenance lineage is labeled rather than implied;
  and an unsafe content excerpt staged for a send mode blocks the send before anything leaves. Each
  packet's `status`, `presentation`, `downgrade_reasons`, `lineage_complete` attestation, and
  `blocked_before_send` flag equal the recomputed gate, so the Support Center, CLI / headless, issue-report
  flow, support drill packet, and support-export surfaces ingest one registry, keep data classes visible,
  and narrow with it.
