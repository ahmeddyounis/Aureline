# Provider-versus-local processing fixtures

Worked-example fixtures for the processing-disclosure packet frozen by:

- the capture / export review matrix
  ([`/artifacts/auth/capture_export_review_matrix.yaml`](../../../artifacts/auth/capture_export_review_matrix.yaml)),
- the device-permission store audit
  ([`/artifacts/auth/device_permission_store_audit.md`](../../../artifacts/auth/device_permission_store_audit.md)),
  and
- the device-permission and capture-boundary contract
  ([`/docs/auth/device_permission_and_capture_boundary_contract.md`](../../../docs/auth/device_permission_and_capture_boundary_contract.md)).

This fixture set focuses on the *processing-disclosure* row that
distinguishes:

1. **Local processing** — capture stays on the device; no provider
   handoff occurs and `processing_locus_class = local_device_only`.
2. **Trusted enterprise managed processing** — capture leaves the
   device for an enterprise-managed processing service inside the
   tenant boundary; `processing_locus_class =
   enterprise_managed_service`.
3. **Third-party provider processing** — capture leaves the device
   for a BYOK or vendor-hosted provider outside the tenant boundary;
   `processing_locus_class = byok_remote_provider` or
   `vendor_hosted_service`.

Each fixture also exercises the matrix's three independent consent
lanes (`consent_to_capture`, `consent_to_export`,
`consent_to_provider_processing`) and the matching irreversible-
share warning class.

The scenarios compose with existing vocabulary:

- the device-permission row and mic-state pill schemas under
  [`/schemas/auth/`](../../../schemas/auth/),
- the speech-privacy ledger schema under
  [`/schemas/ux/speech_privacy_ledger.schema.json`](../../../schemas/ux/speech_privacy_ledger.schema.json),
  and
- the existing capture-boundary fixtures under
  [`/fixtures/auth/capture_boundary_cases/`](../capture_boundary_cases/).

## Index

| Fixture | Processing class | What it proves |
|---|---|---|
| `local_only_air_gapped_capture.yaml` | `local_device_only` (air-gapped envelope) | Capture stays on the device and every export destination is `unavailable_in_envelope`; the `sent_to_*` audio locality is forbidden by the envelope. |
| `enterprise_managed_processing_capture.yaml` | `enterprise_managed_service` | Capture leaves the device for an enterprise-managed processing service inside the tenant boundary; the irreversible-share warning is the managed-service class and delete routes through admin. |
| `byok_third_party_provider_capture.yaml` | `byok_remote_provider` | Capture leaves the device for a BYOK third-party provider; the irreversible-share warning is the third-party class and delete routes through the provider. Three consent lanes are all explicit. |
| `provider_consent_blocked_no_silent_fallback.yaml` | `byok_remote_provider` (consent missing / revoked) | Provider consent is missing; capture denies and the surface MUST NOT silently fall back to a local pack or a different provider. |
| `screen_capture_third_party_share.yaml` | `byok_remote_provider` (screen capture) | Screen capture leaves the device for a third-party provider; the strongest irreversible-share warning class fires before the share is confirmed. |
| `policy_blocked_export_visible_repair.yaml` | `enterprise_managed_service` (export blocked by policy) | Capture is granted but export is policy-blocked; the surface fails closed visibly with the `policy_owner_ref` quoted, and capture remains reviewable while export is unavailable. |

## Conformance notes

- Every fixture is export-safe: no raw audio bytes, no raw frames, no
  raw screen captures, no raw transcripts, no raw URLs, no raw
  provider tokens, and no raw credential material cross the boundary.
- Every `processing_locus_class` other than `local_device_only`
  carries a paired `speech_provider_handoff_disclosure_ref` on the
  bound `speech_capture_event_record` and a paired
  `consent_to_provider_processing` row on the matrix.
- Every `policy_blocked` row quotes the `policy_owner_ref` and a
  visible repair path; "blocked" never means "unknown".
- Air-gapped envelopes forbid every `sent_to_*` audio locality; the
  fixture proves the envelope keeps capture reviewable while every
  export destination renders `unavailable_in_envelope`.
- The processing-disclosure row is *separate* from consent-to-
  capture and consent-to-export. A fixture that collapses them is
  non-conforming.
