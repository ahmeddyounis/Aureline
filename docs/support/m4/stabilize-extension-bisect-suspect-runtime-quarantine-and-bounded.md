# Stabilized extension-bisect, suspect-runtime quarantine, and bounded repair orchestration

This document defines the stabilized orchestration profile that binds extension
bisect, suspect-runtime quarantine, and bounded repair into one typed, truthful,
and narrow recovery contract for the M4 stable lane.

## What this row owns

- The [`StabilizedOrchestrationProfile`] record that binds three recovery
  subsystems:
  - [`ExtensionBisectBinding`] — cites the bisect session, steps, finding,
    restore, and support packet with a closed [`ExtensionBisectStatusClass`].
  - [`SuspectRuntimeQuarantineBinding`] — cites the quarantine record, lane,
    owner, reason, expiry, clear/reenable actions, and evidence refs with a
    closed [`QuarantineReasonClass`].
  - [`BoundedRepairBinding`] — cites the repair transaction, preview, outcome,
    blast radius, compensation, and status with closed
    [`BoundedRepairBlastRadiusClass`], [`BoundedRepairCompensationClass`], and
    [`BoundedRepairStatusClass`].
- The [`StabilizedOrchestrationEvaluator`] that validates the profile,
  bindings, retained capabilities, accessibility postures, and recovery-ladder
  bindings, and projects one [`StabilizedOrchestrationSupportPacket`] per
  profile.
- The boundary schema at
  [`/schemas/support/stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded.schema.json`](../../schemas/support/stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded.schema.json).
- The protected fixture corpus at
  [`/fixtures/support/m4/stabilize-extension-bisect-suspect-runtime-quarantine-and-bounded/`](../../fixtures/support/m4/stabilize-extension-bisect-suspect-runtime-quarantine-and-bounded/).

## Acceptance and how this row meets it

- **Extension bisect, suspect-runtime quarantine, and bounded repair are bound
  into one typed orchestration profile.** Every profile carries non-empty refs
  for all three bindings. The evaluator refuses a profile with empty session
  refs, missing quarantine owners, or missing repair transactions.
- **The orchestration never declares destructive resets.** The
  `destructive_resets_present` field is pinned to `false`. The evaluator refuses
  any profile that sets it to `true`.
- **User-authored files and all durable state classes are preserved.** The
  `preserved_state_classes` list must contain all eight required variants
  including `user_authored_files`, `extension_state_store`, and
  `runtime_quarantine_store`.
- **Every required retained capability is admitted exactly once with support
  guidance.** The evaluator requires the eight
  [`RetainedCapabilityClass::REQUIRED`] variants, each with a non-empty
  `rationale` and non-empty `support_guidance`.
- **Accessibility is validated on every touched surface and capability, not as
  a post-pass.** The evaluator requires at least one accessibility posture row;
  canonical fixtures cover six dimensions per touched target.
- **Recovery-ladder bindings cover every required rung exactly once.** The
  evaluator requires nine [`RecoveryLadderRungClass::REQUIRED`] variants,
  including the new `extension_bisect` and `bounded_repair` rungs, each with a
  non-empty `rung_summary` and at least one `evidence_ref`.
- **Support packets are metadata-only and export-safe.** The support packet
  excludes raw private material and ambient authority by default, pins
  `destructive_resets_present` to `false`, and carries the doc and schema refs.

## Failure-drill posture

The evaluator fails closed before widening any scope:

- A profile that declares a destructive reset is refused.
- A profile that drops any required preserved state class is refused.
- A profile with empty `doctor_finding_ref` or `support_packet_ref` is refused.
- An extension-bisect binding with empty `session_ref`, `finding_ref`,
  `restore_ref`, or `support_packet_ref` is refused.
- A quarantine binding with empty `quarantine_ref`, `lane_ref`, `owner_ref`,
  `clear_action_ref`, `reenable_action_ref`, or empty `evidence_refs` is refused.
- A repair binding with empty `transaction_ref`, `preview_ref`, or `outcome_ref`
  is refused.
- A missing or duplicate retained capability class is refused.
- A retained capability with empty `rationale` or `support_guidance` is refused.
- A missing or duplicate recovery-ladder rung is refused.
- A recovery-ladder binding with empty `rung_summary` or missing `evidence_refs`
  is refused.

## First consumers

- The implementation lives in
  [`crates/aureline-support/src/stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded/mod.rs`](../../../crates/aureline-support/src/stabilize_extension_bisect_suspect_runtime_quarantine_and_bounded/mod.rs).
- The primary evaluator is `StabilizedOrchestrationEvaluator`.

## Related contracts

- [`extension_bisect_beta.md`](../m3/extension_bisect_beta.md) — the beta
  extension-bisect contract this row stabilizes.
- [`extension_bisect.schema.json`](../../schemas/support/extension_bisect.schema.json) —
  the beta boundary schema for extension bisect.
- [`safe_mode_beta.md`](../m3/safe_mode_beta.md) — the beta safe-mode profile.
- [`recovery_ladder_alpha.md`](../recovery_ladder_alpha.md) — the parent rung
  contract for recovery.
- [`repair_transaction_contract.md`](../repair_transaction_contract.md) — the
  repair transaction contract.

## Out of scope for this row

- Live runtime enforcement of extension bisect, quarantine, or repair — those
  bindings land with the runtime-host and chrome consumers that quote this
  profile.
- Auto-resolution or auto-rollback of quarantined lanes — the stabilized
  surfaces only explicit, user-confirmed or policy-reviewed actions.
- Cross-tenant policy-forced orchestration reconciliation; the
  `policy_forced_orchestration` fixture covers single-tenant entry and exit.
