# Support-bundle consent sheets

Before any support bundle leaves your machine, Aureline shows a **consent sheet**: a reviewable
breakdown of exactly what the export would include, what it excludes, what your administrator's policy
locks out, and where the bundle would go. The contract and fail-closed gate are owned by the
`aureline-support` crate (`m5_support_bundle_consent`); the canonical packet is checked in at
`artifacts/support/m5/m5-support-bundle-consent.json` and validated against
`schemas/support/m5-support-bundle-consent.schema.json`.

## What every sheet shows

- **Included / excluded / policy-locked counts by data class.** Each of the four diagnostic data
  classes — `metadata_only`, `environment_adjacent`, `code_adjacent`, and `high_risk` — carries its own
  counts, so you can see what is being included, what is held back, and what an administrator policy
  locked out, rather than trusting an opaque package.
- **The visible schema version.** The bundle's schema version is shown, and a sheet warns when it is
  stale relative to the current schema.
- **A retention note and destination class.** The sheet names where the bundle would go
  (`local_only_review`, `vendor_case_handoff`, `user_initiated_upload`, `managed_admin_handoff`, or
  `private_security_channel`) and how long that destination retains it.
- **Class-safe redaction toggles.** Where policy allows, you can broaden or tighten how a class is
  handled. Secret-bearing (`high_risk`) content stays excluded by default and never offers an
  off-machine-exportable toggle.
- **A first-class local-save path.** Saving the bundle locally is always offered and is never less
  prominent than an upload or formal-support send.

## The consent gate

A fail-closed **consent gate** decides how each sheet may be presented. The published presentation is
the weaker of two ceilings:

| Input | Ceiling |
| --- | --- |
| Consent status `review_ready` | `review_ready` |
| Consent status `policy_narrowed` / `redaction_adjusted` | `narrowed_review` |
| Consent status `send_blocked` | `send_blocked` |
| Schema `current` | `review_ready` |
| Schema `stale` | `narrowed_review` |

So a policy lock, a redaction override, a stale schema, or content that cannot leave the machine can
never read as a clean "ready to export" sheet. Two invariants hold regardless of presentation:

1. **Local-save is never out-shouted by a send path.** A sheet whose local-save path is less prominent
   than a send path fails the gate.
2. **Secret-bearing content never leaves the machine silently.** A `high_risk` class included on a
   destination that leaves the machine blocks the send (`send_blocked`) and warns before any packet
   leaves.

## The scenarios this corpus proves

| Sheet | Destination | Status | Presentation |
| --- | --- | --- | --- |
| `local-only-review` | local save only | review_ready | **review_ready** |
| `vendor-case-upload` | vendor case | review_ready | **review_ready** |
| `managed-policy-locked` | managed admin handoff | policy_narrowed | **narrowed_review** |
| `redaction-override-upload` | user upload | redaction_adjusted | **narrowed_review** |
| `stale-schema-vendor` | vendor case | review_ready | **narrowed_review** |
| `send-blocked-retained-local` | user upload | send_blocked | **send_blocked** |

- **Local-save-only** is a clean, first-class, fully reviewable path — not a degraded fallback.
- **A send-safe upload** keeps the equally prominent local-save path beside it.
- **A policy lock** excludes code-adjacent sections and shows them as locked, not hidden, and offers a
  policy-change path.
- **A broadened redaction** is surfaced as an override that narrows the sheet, never applied silently.
- **A stale schema** warns before sending even when the contents are otherwise send-safe.
- **Unsafe content** (a section marked retained-local-only staged for upload) blocks the send before
  anything leaves, and keeps local-save as the primary path.

## One vocabulary across surfaces

The desktop Support Center, the CLI / headless export review (`aureline support export review`), the
formal support-handoff packet, and the support export of the review itself all bind to this one
registry. Each preserves the same consent vocabulary and object ids, keeps local-save first-class, and
narrows with the gate, so the answer to "what is in this export?" is identical everywhere.

## Regenerating this packet

This packet is checked in alongside the registry it documents. When the consent registry changes,
update the packet, schema, reviewer artifact, and fixtures together, then re-run the gate:

```sh
cargo test -p aureline-support m5_support_bundle_consent
cargo run -p aureline-support --example dump_m5_support_bundle_consent
```
