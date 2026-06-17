# Support-bundle consent — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/support/m5/m5-support-bundle-consent.json`. The full contract and gate semantics live in
`docs/help/support/m5-support-bundle-consent.md`; the typed model lives in the `aureline-support` crate
(`m5_support_bundle_consent`).

This registry gives every support-export scenario a **consent sheet** that shows what the export would
include, exclude, and policy-lock by data class, the visible schema version, the retention note, the
destination class, the class-safe redaction toggles, and — equal in prominence to any upload or
formal-support send — the local-save path. A fail-closed consent gate narrows or blocks any sheet whose
export would carry a policy lock, a silent redaction override, a stale schema, or content that cannot
safely leave the machine, and forbids an upload-first sheet from making local-save look secondary.

## Sheet roll-up (as of 2026-06-16)

| Sheet | Destination | Status | Presentation | Included / Excluded / Locked |
| --- | --- | --- | --- | --- |
| `local-only-review` | local_only_review | review_ready | **review_ready** | 9 / 2 / 0 |
| `vendor-case-upload` | vendor_case_handoff | review_ready | **review_ready** | 9 / 2 / 0 |
| `managed-policy-locked` | managed_admin_handoff | policy_narrowed | **narrowed_review** | 7 / 1 / 1 |
| `redaction-override-upload` | user_initiated_upload | redaction_adjusted | **narrowed_review** | 9 / 2 / 0 |
| `stale-schema-vendor` | vendor_case_handoff | review_ready | **narrowed_review** | 9 / 2 / 0 |
| `send-blocked-retained-local` | user_initiated_upload | send_blocked | **send_blocked** | 8 / 2 / 0 |

Two sheets present as fully review-ready (proving the gate is not a blanket flag), three narrow on a
policy lock, a redaction override, or a stale schema, and one blocks an upload before any packet leaves.
All four data classes are present on every sheet, and the local-save path is first-class on all six.

## The cases this corpus proves

### Local save is first-class — `local-only-review`

A local-save-only export is the safest path and presents transparently. It is the primary affordance,
retains nothing off-device, and is a fully reviewable destination — not a hidden fallback.

### Send-safe upload keeps local-save beside it — `vendor-case-upload`

A bundle attached to a vendor case carries only metadata, environment-adjacent, and redacted
code-adjacent summaries; secret-bearing content stays excluded. The local-save path is offered
co-equal to the upload, so the user is never pushed toward sending.

### Policy lock is shown, not hidden — `managed-policy-locked`

Administrator policy locks code-adjacent sections out of managed handoffs. The locked content is
counted as policy-locked and excluded, the sheet narrows with a `destination_policy_locked` reason, and
a policy-change path is named in the blockers.

### Redaction override is surfaced — `redaction-override-upload`

A code-adjacent section's redaction was broadened from a redacted summary to a sanitized snapshot. The
override is surfaced as a `redaction_override_applied` reason that narrows the sheet, never applied
silently. Secret-bearing content stays excluded even as other redaction is broadened.

### Stale schema warns before sending — `stale-schema-vendor`

The contents are send-safe, but the visible bundle schema is older than the current one. The sheet
narrows with a `stale_schema_warning` and asks the user to regenerate before sending.

### Unsafe content blocks the send — `send-blocked-retained-local`

A code-adjacent section marked retained-local-only is staged for upload. The gate blocks the send
(`export_blocked_unsafe_content`), warns before any packet leaves, and keeps local-save as the primary
path so the user is steered to the safe alternative.

## Sign-off gate

Promotion of the consent registry holds unless all of the following are true on the current packet
(`M5SupportBundleConsent::validate()` returns no violations):

1. Every sheet carries all four data classes with their included / excluded / policy-locked counts, a
   visible schema version, a retention note, a destination, and its one-step explain entry plus the
   CLI / headless equivalent object.
2. Every sheet's `consent_status`, `presentation`, `downgrade_reasons`, `local_save_first_class`
   attestation, and `blocked_before_send` flag equal the recomputed fail-closed gate.
3. The local-save path is offered, enabled, and at least as prominent as every send path on every sheet
   — no upload-first sheet may bury it.
4. Secret-bearing (`high_risk`) classes are excluded by default, never offer an off-machine-exportable
   toggle, and never reach a send destination on a sheet that is not blocked.
5. No raw secret bodies, clipboard history, or raw payloads are carried (`raw_material_excluded`).
6. The four consumer bindings (Support Center, CLI / headless, formal support handoff, support export)
   are all present and reuse this packet's consent vocabulary and object ids, each keeping local-save
   first-class.

## Regenerating this packet

This packet is checked in alongside the registry it reviews. When the consent registry changes, update
the packet, schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-support m5_support_bundle_consent
cargo run -p aureline-support --example dump_m5_support_bundle_consent
```
