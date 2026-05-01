# Learning/presentation evidence packet fixtures

Worked examples for
[`/schemas/learning/learning_presentation_packet.schema.json`](../../../schemas/learning/learning_presentation_packet.schema.json),
the boundary schema for the learning/presentation evidence packet
contract frozen in
[`/docs/learning/learning_presentation_evidence_packet.md`](../../../docs/learning/learning_presentation_evidence_packet.md).

Each fixture is a single record (or audit event) that exercises one
or more contract sections. The fixtures pin opaque ids and typed
vocabulary only; raw speaker-note bodies, raw audience identity,
raw glossary or docs pack bodies, raw URLs, raw absolute paths, and
raw imported teaching pack payloads MUST NOT appear.

| Fixture                                                  | Window                       | Audience                | Significance              |
|----------------------------------------------------------|------------------------------|-------------------------|---------------------------|
| `milestone_close_informational.yaml`                     | `milestone_close_window`     | `engineering_internal`  | `informational`           |
| `accessibility_audit_reduced_motion.yaml`                | `accessibility_audit_window` | `accessibility_audit`   | `informational`           |
| `release_train_parity_assertion.yaml`                    | `release_train_window`       | `release_readiness`     | `release_bearing`         |
| `claim_narrowing_imported_pack_unverified.yaml`          | `ad_hoc_review_window`       | `enterprise_audit`      | `claim_narrowing`         |
| `claim_widening_blocked_pack_mirrored.yaml`              | `release_train_window`       | `release_readiness`     | `claim_widening_blocked`  |
| `audit_denial_export_redaction_floor_unmet.yaml`         | n/a (audit event)            | n/a                     | n/a                       |
