# M3 default-redacted support/incident export fixtures

The seed corpus for the default-redacted support and incident export
profile lives here. Every case is loaded by
[`aureline_support::export_review`](../../../../crates/aureline-support/src/export_review/mod.rs)
and validated against
[`schemas/support/export_redaction_profile.schema.json`](../../../../schemas/support/export_redaction_profile.schema.json).

The cases pin the protected default-redacted posture:

- exact-build identity, scenario family, doctor finding codes, and
  channel/platform identity stay embedded;
- crash manifest ids and symbolication report ids cross the boundary by
  reference, never as raw payloads;
- repair history rows cross the boundary by id, never as transcript
  bodies;
- code-adjacent attachments, raw dumps, raw transcripts, raw logs, full
  shell history, and secret-bearing material stay excluded or
  retained-local-only unless the user accepts an explicit broaden-
  evidence-classes review;
- the local-only save/copy path is always available and equal in
  prominence with any upload/handoff path;
- the reopen manifest preserves what was included, what was excluded,
  build identity, destination class, and whether the export ever left
  the machine.

## Index

| Fixture | Posture |
|---|---|
| `default_redacted_profile.yaml` | the local-only default-redacted export profile for an extension-crash claimed row |
| `vendor_handoff_profile.yaml` | an operator-selected vendor handoff that still keeps raw payloads referenced by id |
| `broaden_evidence_review_required.yaml` | a profile where the user widens code-adjacent capture only after an explicit review marker |
| `reopen_manifest_local_only.yaml` | the reopen manifest emitted after a local-only review (never exported) |
| `reopen_manifest_vendor_handoff.yaml` | the reopen manifest emitted after a vendor case handoff with raw payloads referenced by id |
