# Export-before-reset checklist fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/reliability/export_before_reset_contract.md`](../../../docs/reliability/export_before_reset_contract.md)
and validated by
[`/schemas/recovery/export_before_reset_checklist.schema.json`](../../../schemas/recovery/export_before_reset_checklist.schema.json)
and
[`/schemas/recovery/export_verification_result.schema.json`](../../../schemas/recovery/export_verification_result.schema.json).

Each scenario emits a `*.yaml` checklist record and a matching
`*.verification_result.yaml` result record. The checklist names what
will be deleted, retained, externally recoverable, and not-exported;
the verification-result record carries the per-row outcome and the
reset-authorization gate the destructive-action runner consumes.

**Scope rules**

- Fixtures validate against the export-before-reset checklist and
  verification-result schemas; they do not redefine recovery-scenario,
  continuity-status, restore-destination-review, support-bundle,
  repair-transaction, or scenario-picker vocabularies (those are
  cited by opaque ref).
- A new fixture MUST exercise at least one
  `scenario_family_class`, `reset_kind_class`, `artifact_class`,
  `external_recoverability_class`, `verification_result_class`,
  `follow_up_guidance_class`, `post_reset_state_class`, or
  `reset_authorization_state_class` value the existing set does not
  already cover, and MUST cite the contract section it motivates.
- Monotonic timestamps and stable ids are opaque; they read well
  rather than reflect any real clock.

**Index**

| Fixture | Scenario family | Reset kind | Reset-authorization state | Doc section |
|---|---|---|---|---|
| [`local_only_reset.yaml`](./local_only_reset.yaml) + [`.verification_result.yaml`](./local_only_reset.verification_result.yaml) | `credential_store_unreadable` | `credential_store_reset` | `authorized_for_reset` | §1, §2, §3, §4, §5, §6, §10, §13 |
| [`policy_blocked_export.yaml`](./policy_blocked_export.yaml) + [`.verification_result.yaml`](./policy_blocked_export.verification_result.yaml) | `profile_corruption` | `profile_reset` | `blocked_by_policy` | §1, §5, §7, §8, §9, §10, §16 |
| [`verified_export_before_reset.yaml`](./verified_export_before_reset.yaml) + [`.verification_result.yaml`](./verified_export_before_reset.verification_result.yaml) | `device_replacement` | `factory_reset` | `authorized_for_reset` | §1, §4, §6, §8, §10, §13, §15 |
| [`unsupported_evidence_class.yaml`](./unsupported_evidence_class.yaml) + [`.verification_result.yaml`](./unsupported_evidence_class.verification_result.yaml) | `seat_loss` | `factory_reset` | `blocked_unsupported_class` | §5, §6, §8, §9, §10, §13, §16 |
