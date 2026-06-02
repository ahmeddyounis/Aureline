# Hardened safe-mode runtime profile, retained capabilities, and support guidance

This document defines the hardened safe-mode runtime profile that promotes the
beta safe-mode contract into a stable, typed, and exportable system with
explicit retained capabilities, accessibility posture declarations, and
recovery-ladder integration.

## What this row owns

- The [`HardenedSafeModeProfile`] record that names every narrowing as a typed
  `(host|service|surface, disposition_class, reason_class, summary)` row and
  adds three new row sets:
  - [`RetainedCapabilityRecord`] — one per required capability with a closed
    [`RetainedCapabilityClass`], a reviewer-safe rationale, and a user-facing
    support-guidance string.
  - [`AccessibilityPostureRow`] — one per touched capability or surface per
    dimension (keyboard, screen-reader, IME/grapheme/bidi, zoom, high-contrast,
    reduced-motion) with a closed [`AccessibilityPostureClass`] and explanation.
  - [`RecoveryLadderBindingRow`] — one per required rung with a closed
    [`RecoveryLadderRungClass`], a [`HardenedSafeModeSupportClass`] label,
    and evidence refs.
- The [`HardenedSafeModeEvaluator`] that validates the profile, retained
  capabilities, accessibility postures, and recovery-ladder bindings, and
  projects one [`HardenedSafeModeSupportPacket`] per profile.
- The boundary schema at
  [`/schemas/support/harden_the_safe_mode_runtime_profile_retained_capabilities.schema.json`](../../schemas/support/harden_the_safe_mode_runtime_profile_retained_capabilities.schema.json).
- The protected fixture corpus at
  [`/fixtures/support/m4/harden_the_safe_mode_runtime_profile_retained_capabilities/`](../../fixtures/support/m4/harden_the_safe_mode_runtime_profile_retained_capabilities/).

## Acceptance and how this row meets it

- **Safe mode declares exactly which hosts, services, and surfaces are disabled
  or narrowed and why.** Every `declared_hosts`, `declared_services`, and
  `declared_surfaces` row carries an opaque id, a closed `*_class`, a closed
  `disposition_class`, a closed `reason_class`, and a reviewer-safe
  `narrowing_summary`. The evaluator refuses a profile with empty hosts, empty
  services, or empty/duplicate ids; the schema mirrors the same shape so the
  chrome and the headless export cannot disagree on what was disabled.
- **Every required retained capability is admitted exactly once with
  support guidance.** The evaluator requires the eight
  [`RetainedCapabilityClass::REQUIRED`] variants, each with a non-empty
  `rationale` and non-empty `support_guidance`. The canonical fixtures also
  mark every capability as `explicitly_tested: true`.
- **Accessibility is validated on every touched surface and capability, not as
  a post-pass.** The evaluator requires six [`AccessibilityDimensionClass`]
  rows per touched surface and per retained capability. A missing dimension
  is refused; an empty `explanation` is refused.
- **Recovery-ladder bindings cover every required rung exactly once.** The
  evaluator requires seven [`RecoveryLadderRungClass::REQUIRED`] variants,
  each with a non-empty `rung_summary` and at least one `evidence_ref`.
- **Safe-mode entry and exit preserve user-owned state and avoid destructive
  resets.** The profile's `destructive_resets_present` field is pinned to
  `false`. The `preserved_state_classes` list must contain
  `user_authored_files`. The support packet excludes raw private material and
  ambient authority by default.

## Failure-drill posture

The evaluator fails closed before widening any narrowing:

- A profile that declares a destructive reset is refused.
- A profile that drops `user_authored_files` preservation is refused.
- A profile without a `doctor.finding.*` ref or without any narrowed
  hosts/services is refused.
- Duplicate `host_id`, `service_id`, or `surface_id` values are refused.
- A missing or duplicate retained capability class is refused.
- A retained capability with empty `rationale` or `support_guidance` is refused.
- A missing accessibility posture for any touched surface/capability and
  dimension is refused.
- A missing or duplicate recovery-ladder rung is refused.
- A recovery-ladder binding with empty `rung_summary` or missing `evidence_refs`
  is refused.

## First consumers

- The implementation lives in
  [`crates/aureline-support/src/harden_the_safe_mode_runtime_profile_retained_capabilities/mod.rs`](../../../crates/aureline-support/src/harden_the_safe_mode_runtime_profile_retained_capabilities/mod.rs).
- The primary evaluator is `HardenedSafeModeEvaluator`.

## Related contracts

- [`safe_mode_beta.md`](../m3/safe_mode_beta.md) — the beta safe-mode profile
  this row hardens.
- [`safe_mode_profile.schema.json`](../../schemas/support/safe_mode_profile.schema.json) —
  the beta boundary schema.
- [`recovery_ladder_alpha.md`](../recovery_ladder_alpha.md) — the parent rung
  contract for safe mode.
- [`recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json) —
  the closed recovery-action vocabulary.

## Out of scope for this row

- Live runtime enforcement (host launcher, service supervisor, panel gating) —
  those bindings land with the runtime-host and chrome consumers that quote
  this profile.
- Time-bounded auto-exit from safe mode after a clean launch — the hardened
  surfaces only explicit, user-confirmed return paths.
- Cross-tenant policy-forced safe-mode reconciliation; the
  `policy_forced_profile` fixture covers single-tenant entry and exit.
