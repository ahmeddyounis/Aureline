# Capture-boundary honesty fixtures

Worked-example fixtures for the capture-boundary honesty contract frozen
in [`/docs/auth/device_permission_and_capture_boundary_contract.md`](../../../docs/auth/device_permission_and_capture_boundary_contract.md).

This fixture set focuses on the *host-owned boundary rows* that make it
possible for UI, support/export, and embedded surfaces to stay honest
about:

- device permissions (microphone/camera/screen),
- whether the mic is actually listening vs idle/muted/processing,
- where processing happens (local vs remote/provider/managed),
- what retention/export posture applies, and
- how to revoke or escape to a system-browser boundary.

The scenarios compose this contract with existing vocabulary:

- voice/dictation privacy and confirmation flows under `/fixtures/ux/voice_cases/`,
- embedded-surface origin bars under `/fixtures/ux/embedded_boundary_cases/`, and
- system-browser/device-code handoff packets under `/fixtures/auth/callback_and_lock_state_cases/`.

## Index

| Fixture | What it proves |
|---|---|
| `local_dictation_preflight.yaml` | Local dictation preflight remains explicit about OS permission, processing locus, and no-retention posture. |
| `remote_provider_consent_preflight.yaml` | Remote provider consent is disclosed before listening starts, with an explicit system-browser escape hatch. |
| `privileged_voice_action_needs_confirmation.yaml` | High-impact spoken actions surface `Needs confirmation` until preview/confirmation completes. |
| `device_code_fallback_handoff.yaml` | Device-code fallback stays distinguishable from system-browser handoff and preserves local continuity. |
| `capture_export_review.yaml` | Capture/transcript export remains explicit about redaction and what is exported vs omitted. |
| `embedded_origin_disclosure.yaml` | Embedded origin bars disclose owner/origin and forbid in-webview impersonation of native auth/security surfaces. |

