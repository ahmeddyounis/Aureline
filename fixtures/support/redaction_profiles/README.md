# Support bundle redaction profiles

These fixtures seed the preview/export profiles consumed by
[`schemas/support/support_bundle.schema.json`](../../../schemas/support/support_bundle.schema.json)
as `support_bundle_redaction_profile_record`.

Each profile demonstrates how a governed support bundle decides:

- what can embed by default;
- what exports as metadata-only or by stable reference;
- what stays local-only unless a later review or opt-in widens it;
- what requires manual review before export; and
- what remains excluded by default or excluded always.

The profiles intentionally quote the same vocabulary as the bundle
schema: `handling_class`, `redaction_class`, `artifact_kind_class`,
`data_class`, and `record_class_id`.

## Index

| Fixture | Main posture |
|---|---|
| [`local_first_default.yaml`](./local_first_default.yaml) | default local-first support export that keeps raw high-risk captures on-device |
| [`operator_escalation_review.yaml`](./operator_escalation_review.yaml) | operator-reviewed escalation profile that still requires review before widening code-adjacent or raw high-risk evidence |
