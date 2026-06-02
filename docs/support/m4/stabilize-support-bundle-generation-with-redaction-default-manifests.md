# Stabilized support-bundle generation with redaction-default manifests and chain-of-custody fields

This document defines the M4 stable lane that stabilizes support-bundle
generation into a typed, versioned, redacted-by-default, and chain-of-custody-
auditable system.

## What this row owns

- The [`StabilizedSupportBundleManifest`] record that carries:
  - Schema version, included/excluded classes, destination class, retention note,
    redaction profile ref, and chain-of-custody fields.
  - A clear [`SupportBundleGenerationMode`] distinction between
    `ordinary_redaction_default` and `high_fidelity_incident_capture`.
  - [`ConsentEscalationClass`] so high-fidelity capture never widens by default.
  - [`RecoveryLadderHookBinding`] rows for every required recovery rung.
- The [`StabilizedSupportBundleEvaluator`] that validates manifests and projects
  metadata-safe [`StabilizedSupportBundleSupportPacket`] rows.
- The boundary schema at
  [`/schemas/support/stabilize_support_bundle_generation_with_redaction_default_manifests.schema.json`](../../schemas/support/stabilize_support_bundle_generation_with_redaction_default_manifests.schema.json).
- The protected fixture corpus at
  [`/fixtures/support/m4/stabilize-support-bundle-generation-with-redaction-default-manifests/`](../../fixtures/support/m4/stabilize-support-bundle-generation-with-redaction-default-manifests/).

## Acceptance and how this row meets it

- Every stabilized manifest carries schema version, included/excluded classes,
  destination, retention, redaction profile, and chain-of-custody.
- Ordinary redaction-default export and high-fidelity incident capture are
  clearly distinguished. High-fidelity capture requires either
  `explicit_user_consent` or `admin_policy_override`; `not_required` is only
  valid for ordinary exports.
- High-risk and code-adjacent data classes are not included in ordinary exports.
  When included under high-fidelity capture, proper consent escalation is
  enforced.
- Chain-of-custody entries are mandatory, strictly sequenced, and carry actor,
  action, location, and note fields.
- Recovery-ladder hooks cover all eight required rungs; every hook preserves
  user-owned state and carries a non-empty blast-radius description.
- Local-only destination requires `supports_offline_inspection=true` so the
  export remains useful without upload assumptions.
- Support packets are metadata-only and quote the same schema, doc, and artifact
  refs so intake surfaces reconstruct the exact contract.

## Failure-drill posture

- Schema-version and record-kind mismatches are rejected.
- Empty manifest ids, build ids, exact-build refs, redaction profile refs, or
  retention notes are rejected.
- Ordinary exports with consent escalation refs or incident scenarios are rejected.
- High-fidelity captures missing consent escalation refs or incident scenarios
  are rejected.
- Empty included/excluded class lists are rejected.
- Non-monotonic chain-of-custody sequences, empty actor refs, or empty notes
  are rejected.
- Missing or duplicate recovery-ladder hooks are rejected.
- Hooks claiming `preserves_user_state=true` with destructive blast-radius
  keywords are rejected.
- Local-only destination without offline inspection support is rejected.

## First consumers

- The implementation lives in
  [`crates/aureline-support/src/stabilize_support_bundle_generation_with_redaction_default_manifests/mod.rs`](../../../crates/aureline-support/src/stabilize_support_bundle_generation_with_redaction_default_manifests/mod.rs).
- The primary evaluator is [`StabilizedSupportBundleEvaluator`].

## Related contracts

- [`support_bundle_contract.md`](../m3/support_bundle_contract.md) — the M3
  support-bundle contract this row stabilizes.
- [`support_bundle_manifest.schema.json`](../../schemas/support/support_bundle_manifest.schema.json) —
  the existing support-bundle manifest schema.
- [`export_redaction_profile.schema.json`](../../schemas/support/export_redaction_profile.schema.json) —
  the default-redacted export profile.
- [`record_class.schema.json`](../../schemas/support/record_class.schema.json) —
  records-governance and chain-of-custody schema.

## Out of scope for this row

- Live byte-level redaction implementation, upload transport, or hosted intake.
- Full diagnostic artifact matrix expansion.
- Live Project Doctor probe execution or repair application.
