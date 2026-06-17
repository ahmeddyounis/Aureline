# Shiproom review packet — support-bundle consent

This packet is the shiproom- and release-center-facing view of the support-bundle consent registry. It
does not maintain its own summary: the claim scope below is read from the canonical packet and narrows
automatically when a sheet carries a policy lock, a redaction override, a stale schema, or content that
cannot leave the machine.

## Canonical inputs

- Packet: `artifacts/support/m5/m5-support-bundle-consent.json`
- Reviewer artifact: `artifacts/support/m5/m5-support-bundle-consent.md`
- Schema: `schemas/support/m5-support-bundle-consent.schema.json`
- Companion doc: `docs/help/support/m5-support-bundle-consent.md`
- Fixtures: `fixtures/support/m5/m5-support-bundle-consent/`
- Typed model + gate: `aureline-support` crate, `m5_support_bundle_consent`

- Claim publishable: **yes**
- Review-ready sheets: `2`
- Narrowed sheets: `3`
- Send-blocked sheets: `1`
- Local-save first-class on every sheet: `yes`

## Claim scope

| Sheet | Destination | Status | Presentation | Included / Excluded / Locked |
| --- | --- | --- | --- | --- |
| `local-only-review` | local_only_review | review_ready | **review_ready** | 9 / 2 / 0 |
| `vendor-case-upload` | vendor_case_handoff | review_ready | **review_ready** | 9 / 2 / 0 |
| `managed-policy-locked` | managed_admin_handoff | policy_narrowed | **narrowed_review** | 7 / 1 / 1 |
| `redaction-override-upload` | user_initiated_upload | redaction_adjusted | **narrowed_review** | 9 / 2 / 0 |
| `stale-schema-vendor` | vendor_case_handoff | review_ready | **narrowed_review** | 9 / 2 / 0 |
| `send-blocked-retained-local` | user_initiated_upload | send_blocked | **send_blocked** | 8 / 2 / 0 |

## Sign-off gate

Promotion of the consent registry holds unless all of the following are true on the current packet
(`M5SupportBundleConsent::validate()` returns no violations):

1. Every sheet carries all four data classes with their included / excluded / policy-locked counts, a
   visible schema version, a retention note, a destination, its one-step explain entry, and the
   CLI / headless equivalent object.
2. Every sheet's `consent_status`, `presentation`, `downgrade_reasons`, `local_save_first_class`
   attestation, and `blocked_before_send` flag equal the recomputed fail-closed gate.
3. The local-save path is offered, enabled, and at least as prominent as every send path on every
   sheet — no upload-first sheet may bury it.
4. Secret-bearing classes are excluded by default, never offer an off-machine-exportable toggle, and
   never reach a send destination on a sheet that is not blocked.
5. No raw secret bodies, clipboard history, or raw payloads are carried.
6. The four consumer bindings (Support Center, CLI / headless, formal support handoff, support export)
   are present and reuse this packet's consent vocabulary and object ids, each keeping local-save
   first-class.

This packet projects from the canonical support-bundle consent truth source; it does not restate the
consent vocabulary in its own words.
