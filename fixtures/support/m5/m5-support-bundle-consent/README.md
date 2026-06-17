# Fixtures: support-bundle consent sheets

This directory contains fixture metadata for the `m5_support_bundle_consent` packet.

The canonical full corpus is checked in at:

`artifacts/support/m5/m5-support-bundle-consent.json`

It is the one authoritative support-bundle consent registry; the typed model and fail-closed consent
gate live in the `aureline-support` crate (`m5_support_bundle_consent`).

## Coverage

- All four data classes — `metadata_only`, `environment_adjacent`, `code_adjacent`, and `high_risk` —
  are present on every sheet, each carrying its included / excluded / policy-locked counts. The
  secret-bearing `high_risk` class is excluded by default and never offers an off-machine-exportable
  toggle on any sheet.
- All five destination classes are exercised across the sheets (`local_only_review`,
  `vendor_case_handoff`, `user_initiated_upload`, `managed_admin_handoff`), and every sheet offers an
  enabled local-save path that is at least as prominent as any send path.
- The three presentations are each exercised: `review_ready` (`local-only-review`, `vendor-case-upload`),
  `narrowed_review` (`managed-policy-locked`, `redaction-override-upload`, `stale-schema-vendor`), and
  `send_blocked` (`send-blocked-retained-local`).
- The four consent statuses (`review_ready`, `policy_narrowed`, `redaction_adjusted`, `send_blocked`)
  and the four downgrade reasons (`destination_policy_locked`, `redaction_override_applied`,
  `export_blocked_unsafe_content`, `stale_schema_warning`) are each exercised.
- The named fixtures the source set calls out are all present: a policy-locked export
  (`managed-policy-locked`), a local-save-only flow (`local-only-review`), a redaction override
  (`redaction-override-upload`), and a stale-schema warning (`stale-schema-vendor`), plus a send-blocked
  unsafe-content case (`send-blocked-retained-local`).
- The gate is exercised in every direction: two sheets are fully review-ready (proving the gate is not
  a blanket flag); the policy lock keeps the locked content visible as excluded; the redaction override
  is surfaced rather than silent; the stale schema warns even when contents are send-safe; and the
  unsafe-content case blocks the send before any packet leaves while keeping local-save primary. Each
  sheet's `consent_status`, `presentation`, `downgrade_reasons`, `local_save_first_class` attestation,
  and `blocked_before_send` flag equal the recomputed gate, so the Support Center, CLI / headless,
  formal-support-handoff, and support-export surfaces ingest one registry and a narrowed or blocked
  export cannot read as a clean "ready to export" sheet.
