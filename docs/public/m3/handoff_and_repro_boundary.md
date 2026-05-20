# Public/private handoff review and reproduction-packet boundary

This document is the M3 contract for the public/private handoff lane. It governs
how the issue, security-disclosure, docs-feedback, RFC/discussion, and
community-support handoffs let the user **review what will be shared, with whom,
and under which visibility boundary — before the report leaves the shell** — and
how the drafted work survives when a browser, network, or target permission
fails.

The contract has three artifacts:

- `schemas/public/handoff_target_review.schema.json` — boundary schema for one
  handoff target row.
- `schemas/public/repro_packet_preview.schema.json` — boundary schema for one
  reproduction-packet preview.
- `crates/aureline-shell/src/handoff_review/` — the Rust projection and
  validator that the handoff lanes consume.

Positive and negative fixtures live in
`fixtures/public/m3/handoff_and_repro_preview/`; the Rust test
`crates/aureline-shell/tests/handoff_review_preview_fixtures.rs` loads every
fixture and asserts the contract still rejects the documented drift.

## Why one review sheet

When a user files an issue, asks the community for help, sends docs feedback, or
discloses a vulnerability, the destinations have very different visibility:
a public issue is world-readable forever, a security disclosure must stay on a
private encrypted channel, a support intake is private-but-redacted, and a
vendor route leaves the project entirely. Without a single review sheet, the
lanes blur together — a security report can be coerced onto the public tracker,
a public issue can carry a security-scoped payload, and a blocked browser can
silently drop everything the user typed.

The review sheet — `HandoffReviewSheet` — bundles three pieces the user reviews
together:

1. a **handoff target review** (where it goes and how visible it is),
2. a **reproduction-packet preview** (exactly what will be shared), and
3. a **draft-continuity** block (what survives if the handoff is blocked).

## Handoff target review

Each target row carries typed axes:

| Axis | Vocabulary |
|---|---|
| `visibility_class` | `official_public`, `official_private`, `security_disclosure`, `community`, `third_party_vendor` |
| `route_class` | `public_issue`, `security_disclosure`, `docs_feedback`, `rfc_discussion`, `community_support` |
| `network_browser_requirement_class` | `offline_capture_preview`, `system_browser_public_browse`, `system_browser_authenticated_plane`, `encrypted_security_channel`, `vendor_or_third_party_call` |
| `data_exit_boundary_class` | reused from the About/help/community destination contract |

Honesty rules enforced by the schema and the Rust validator:

- **A route may only target a visibility from its allowed set.** Security
  disclosures resolve only to `security_disclosure` or `official_private`
  visibility — never a public target. Public issue, docs feedback, and
  RFC/discussion resolve to `official_public` or `community`; community support
  resolves to `community` or `official_private`. The lanes never blur together
  or coerce the user into a public target by accident.
- **Visibility pins the data exit and the network requirement.**
  `security_disclosure` visibility carries `security_payloads_only` data exit and
  the encrypted (or offline) channel. `third_party_vendor` visibility carries a
  vendor/external data exit and the vendor (or offline) network call.
  `official_public` and `community` visibility never carry a security- or
  support-scoped data exit.
- **Every handoff offers a safe fallback.** Each target row cites at least one
  `safe_fallback_refs[]` entry so a blocked route degrades to a labeled path
  (the local docs pack, a saved draft) instead of dead-ending.
- **Every handoff lane attaches a versioned build-context export.** The same
  `build_context_exports[]` block the About/help/community surfaces publish rides
  the issue/report/disclosure lane, so the user never has to infer scope from a
  raw URL and never pastes a screenshot.

## Reproduction-packet preview

The preview the user confirms before share carries typed axes:

| Axis | Vocabulary |
|---|---|
| `redaction_posture_class` | `fully_redacted_public_safe`, `redacted_support_scoped`, `security_channel_only`, `metadata_refs_only` |
| `selected_diagnostics[].kind_class` | build identity, environment capsule, redacted log tail, sanitized config snapshot, repro steps text, anchor object ref, performance trace |
| `attachments[].kind_class` | build-context export block, redacted log bundle, minimal repro project, sanitized config bundle, anchor object snapshot |
| `anchor_identity` | the exact anchor + object the report is about |

Honesty rules enforced by the schema and the Rust validator:

- **Raw payloads never ride a shareable preview.** `raw_secrets_excluded` and
  `raw_screenshots_excluded` are always `true`, and every attachment is
  `redaction_applied`.
- **At least one diagnostic is selected.** A preview that includes nothing is
  rejected; the summary lists which diagnostics are in and which are out.
- **The report names a precise object.** `anchor_identity` cites the exact
  anchor and object refs (and a reviewable label), so the report points at a
  real location rather than a fuzzy description.
- **Preview before share.** `preview_confirmed_before_share` gates the handoff:
  the system browser only opens after the user has confirmed the preview.

## Sheet cross-validation

`HandoffReviewSheet` validates each constituent record, then enforces the joins
that make the review trustworthy:

- the redaction posture is **safe for the target visibility** — a public target
  may not carry a support- or security-scoped payload;
- the preserved draft mirrors the **chosen target class** and the **chosen
  redaction posture**, so a blocked handoff resumes against the same boundary;
- the share is gated on **preview confirmation** before the browser opens;
- a **selected fallback** is one the target review actually offered.

## Draft continuity: no silent loss

When the browser handoff is blocked, offline, or policy-denied, the
draft-continuity block proves the work survives:

| Outcome | Requirement |
|---|---|
| `opened_in_system_browser` | requires `preview_confirmed_before_share = true` |
| `browser_blocked`, `offline`, `policy_denied`, `target_permission_denied` | must preserve intent, keep the draft text, and offer **both** `export_packet` and `save_draft_local` |

`silent_loss` is never allowed, regardless of outcome. The block records the
preserved draft text ref, the preserved attachment refs, the preserved target
class, and the preserved redaction posture, plus the closed set of preservation
actions (`export_packet`, `save_draft_local`, `retry_when_online`,
`copy_refs_to_clipboard`, `discard`). A blocked handoff that offers only
`discard` is rejected.

The sheet also renders a deterministic plaintext block that support exports and
reviewer-facing previews quote without inventing their own ordering or
vocabulary.

## What is out of scope in M3

This contract does not build a hosted ticketing system, a discussion client, or
a synced cloud-draft service inside Aureline. The handoff lanes are descriptive
and resilient only — they declare visibility, preview the packet, and preserve
the draft locally; they do not add arbitrary external social/marketing share
targets or long-lived synced cloud drafts beyond the current M3 handoff lanes.
M3 owns the boundary and the review, not a hosted backend.
