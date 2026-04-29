# Device-permission, capture-boundary, and embedded-origin honesty contract

This contract freezes the shared vocabulary Aureline uses to make
**device permissions**, **mic/capture state**, **transcript review**, and
**embedded-surface boundaries** legible before voice, dictation, screen
capture, and embedded authentication flows widen. The goal is boundary
honesty: before capture begins (or an embedded surface requests it),
users can tell **what is about to be captured**, **where processing
happens**, **what retention / export posture applies**, and **how to
revoke or move the flow to a system-browser boundary**.

Companion artifacts:

- [`/schemas/auth/device_permission_row.schema.json`](../../schemas/auth/device_permission_row.schema.json)
  defines `device_permission_row_record`, the required device-permission
  row shared by settings, capture preflight sheets, status inspectors,
  and support/export previews.
- [`/schemas/auth/mic_state.schema.json`](../../schemas/auth/mic_state.schema.json)
  defines `mic_state_record`, the persistent mic-state pill / strip
  shown while capture is possible or active.
- [`/fixtures/auth/capture_boundary_cases/`](../../fixtures/auth/capture_boundary_cases/)
  contains worked scenarios that compose this contract with the existing
  voice/dictation, auth handoff, and embedded-surface boundary records.

Upstream contracts this document composes with (it does not replace):

- [`/docs/ux/voice_and_dictation_contract.md`](../ux/voice_and_dictation_contract.md)
  for dictation vs command mode separation, mic indicator invariants,
  provider handoff disclosure, transcript correction-before-commit, and
  transcript export/redaction rules.
- [`/docs/auth/system_browser_callback_packet.md`](./system_browser_callback_packet.md)
  for system-browser and device-code handoff packets, callback
  correlation, and the rule that embedded auth is not a silent fallback.
- [`/docs/auth/credential_state_and_secret_prompt_contract.md`](./credential_state_and_secret_prompt_contract.md)
  for the shared actor/scope/storage vocabulary used by provider consent
  and handoff prompts.
- [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  for owner/origin chrome, impersonation bans, system-browser-first auth,
  and the native-reserved surface list.
- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)
  for canonical `command_id` identity, preview/approval posture, and
  undo/audit parity rules voice and capture lanes must inherit.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` §5.36–5.37 and §10.11–10.13 for
  system-browser-first auth, device/OS integration, explicit retention
  and export truth, and “what data exists, where it lives, how long it
  survives” honesty requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6 and §23.7
  for system-browser-first identity boundaries and voice/dictation as an
  explicit interaction mode with disclosed locality and retention.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.14 and §8.14 for
  handoff cards, device-code fallback, explicit “where processing
  happens” disclosure, and parity with the command/trust/undo model.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §21.14 and §23.x drills for
  embedded-webview origin bars, “open in browser” fallback, and
  confirmation/review expectations for voice-driven privileged actions.

If this document disagrees with the upstream sources, those sources win
and this contract plus its schemas and fixtures must update in the same
change.

## Principles

1. **Capture is explicit.** Mic/camera/screen capture must never feel
   “always on” or implied. Every capture-capable surface advertises its
   state before the first capture begins.
2. **Boundaries are named.** “Local”, “system browser”, and “embedded
   webview” are distinct boundaries with distinct guarantees and
   limitations; surfaces must not blur them.
3. **Where processing happens is disclosed.** Before capture begins, the
   user can tell whether processing is local-only or involves a remote
   provider / managed service, and whether any transcript is retained.
4. **Retention and export are truthful.** Capture, transcripts, and
   provider-backed processing carry an explicit retention/export note.
   Deletion/export flows preserve the distinction between metadata and
   raw captured content.
5. **Revoke and repair are first-class.** Every permission or consent
   state names a safe revoke path and an OS/system-settings route when
   applicable; “try again” is not enough.
6. **Parity with the command lane is mandatory.** Voice-triggered and
   capture-triggered actions resolve to the same `command_id`, trust
   gates, preview/approval posture, and undo model as keyboard/mouse/CLI
   equivalents.
7. **Embedded surfaces cannot impersonate trust.** Embedded webviews may
   request host-native review/permission surfaces, but they may not host
   native-reserved security/auth/verification UI or password collection
   as a silent fallback.

## Required surfaces (frozen)

Every capture-capable flow must be representable using the following
surfaces. Implementations may vary visually, but the fields and rules
below are required.

### 1) Device-permission row (`device_permission_row_record`)

The device-permission row answers, without ambiguity:

- **What device class is involved** (microphone, camera, screen capture,
  or other declared class).
- **What permission state applies** (not requested, granted, denied,
  restricted, unavailable, policy blocked).
- **Who is asking** (actor class + surface ref) and whether the request
  is user-initiated, extension-initiated, or policy-initiated.
- **Where processing happens** (local-only vs remote/managed/provider),
  with an export-safe disclosure label.
- **Retention / export note** that differentiates metadata-only rows
  from durable/raw capture material.
- **Revoke and repair actions**: revoke route, open system settings
  route, and (when applicable) an “open in system browser” escape hatch
  for auth/provider flows.

Rules:

1. A device-permission row MUST be renderable before capture begins and
   MUST remain inspectable while capture is active.
2. “Permission required” copy MUST include the device class, the
   actor/surface, and the OS/system-settings repair route when denial is
   OS-scoped.
3. A row representing remote/provider capture MUST include the locus
   disclosure label; pretending “it’s local” while audio leaves the
   device is non-conforming.

### 2) Mic-state pill (`mic_state_record`)

The mic-state pill is the persistent indicator that keeps capture honest
in the moment. It carries:

- a closed **state class** whose display label is one of:
  `Idle`, `Listening`, `Muted`, `Processing`, `Needs confirmation`,
  `Unavailable`, `Policy blocked`;
- the current **actor** and **invocation surface**;
- the **processing locus disclosure** label (local-only vs remote);
- the **transcript correction path** ref (when a transcript is expected);
- a **stop/mute** action label and an optional **open detail** route.

Rules:

1. If capture is active, the mic-state pill MUST be visible.
2. `Listening` MUST NOT be shown unless the system is actually capturing
   audio (not merely “voice feature enabled”).
3. `Needs confirmation` MUST be shown when a voice-triggered privileged
   action is awaiting preview/confirmation before commit.
4. `Policy blocked` is distinct from `Unavailable`; policy blocks must
   name the policy owner/ref in the detail view.

### 3) Transcript strip (review + correction)

The transcript strip is the capture-adjacent review surface that allows
correction before commit and safe export after capture.

Rules:

1. Dictation vs command mode MUST be clear; the strip must not imply a
   spoken command will be inserted as text without stating the mode.
2. Privileged voice actions MUST require preview/confirmation and a
   correction-before-commit window, reusing the same command preview and
   undo rules as keyboard-driven equivalents.
3. Transcript export MUST be explicit and redaction-aware; exporting a
   transcript implicitly through a support bundle or log is forbidden.

### 4) Browser/device-code handoff card

When capture depends on a remote provider consent or an auth boundary,
the surface uses a handoff card that quotes the existing handoff packets
instead of inventing new copy.

Rules:

1. System-browser is the default. Embedded webviews MUST NOT be the
   silent fallback for sign-in or consent.
2. Device-code fallback MUST remain distinguishable from browser handoff
   and MUST preserve local-work continuity language while the handoff is
   pending.
3. The card MUST include provider identity (domain label), expiry, and
   a cancel/retry path.

### 5) Embedded-webview origin bar (owner/origin chrome)

Every embedded webview-like surface includes a host-owned origin bar.

Rules:

1. The bar MUST disclose owner (host vs extension/service), origin class
   (embedded vs system browser), and an open-in-browser escape hatch.
2. The bar MUST disclose capability limitations (for example, “cannot
   collect credentials here”, “high-risk approvals open in a host-native
   sheet”).
3. Embedded content MUST NOT impersonate native security messaging,
   update verification, trust prompts, irreversible confirmations, or
   auth UI.

### 6) Capture/export review

Any export or deletion flow that touches mic capture, transcript
material, or provider-backed processing MUST provide a capture/export
review step that distinguishes:

- device permission rows (metadata about permission/consent),
- mic-state rows (live state),
- transcripts (text content),
- capture artifacts (audio/screen/camera material),
- provider processing disclosures (where the capture went), and
- retention posture (what remains after export/delete).

Rules:

1. A “delete” action MUST state whether it is deleting a local-only copy,
   a managed copy, or only metadata; “deleted” cannot mean “unknown”.
2. Export MUST be previewable and redaction-aware, and MUST provide
   omission reasons for policy-blocked or held content.

## Change control

- Adding a new `device_class`, `permission_state_class`, `mic_state_class`,
  or required surface field is additive-minor and requires updates to
  this contract, the relevant schema(s), and at least one fixture in
  `/fixtures/auth/capture_boundary_cases/` in the same change.
- Repurposing an existing vocabulary value is breaking and requires
  security/trust and accessibility review.
- Any change that weakens system-browser-first auth, hides capture state,
  or removes the revoke/system-settings repair path is non-conforming
  and must be accompanied by updated fixtures proving the new behavior
  remains honest.

