# Voice processing, provider routing, and retention

Voice is an **explicit, privacy-bounded input mode** in Aureline. This doc is the
human-readable face of how a claimed voice profile picks a speech provider,
switches language/acoustic profiles, processes audio locally or on a disclosed
hosted engine, controls transcript retention and export, and gates all of that
behind policy and entitlement — without ever silently widening authority or
reducing privacy.

The machine truth is the voice provider routing packet built by
`crates/aureline-shell/src/voice_provider_routing/`. Help/About, settings,
admin, diagnostics, support-export, and release surfaces should ingest that
packet rather than cloning provider/locality/retention text by hand.

## What is explicit product state

For every claimed voice profile, these are inspectable before capture and are
never inferred silently:

- **Provider and processing locality.** Which speech provider is active and
  whether it processes audio `local_on_device` or `hosted_remote_disclosed`. A
  hosted provider always declares a disclosed transport and is audit-capable.
- **Language / acoustic profile.** The active language tag (for example
  `en-US`), the acoustic profile class (default, noise-adapted, accent-adapted,
  or near-field), and whether the backing language pack is bundled, downloaded,
  available for download, hosted-only, or unavailable.
- **Transcript retention and export controls.** The retention mode, the
  audio-retention class, and the transcript-export posture, plus whether the user
  can change them and a precise disclosure label. Raw transcripts are excluded
  from support/telemetry by default and any support export is redacted first.
  This is the object the
  [`retention-and-export` schema](../../schemas/voice/retention-and-export.schema.json)
  describes.
- **Policy and entitlement state.** Whether voice is user-controlled,
  enterprise-managed, or policy-blocked, and whether the entitlement required by
  a provider path is held, needs an upgrade, was revoked, or could not be
  verified.

## Local-first defaults

With no specific provider requested, voice routes to the designated on-device
default and discloses local-only processing — no audio or transcript leaves the
device. Remote or enterprise-managed processing is never the default: it requires
an explicit request, a compatible policy state, and disclosure of the retention
and export change before capture.

## Switching never hides a retention or export change

When the user (or an admin policy) switches the active provider or language
profile, the resolved outcome records whether the active provider, locality,
retention mode, export posture, or language profile differs from what was
requested. The chrome surfaces those deltas — for example, opting in to a hosted
provider discloses that transcripts move from "no transcript retained" to
"retained redacted" and that export becomes an explicit, redacted user action.
A switch never swaps a retention or export posture silently.

## Denials are explicit, never a quiet widening

A policy or entitlement denial of a hosted or broader provider **never** falls
back to a broader or less private provider. The resolver only ever moves toward
on-device processing. The possible outcomes on a denial are:

- **Blocked, explicitly.** Voice is held off with a precise reason (for example,
  policy blocks voice, or a revoked entitlement with no on-device fallback). The
  keyboard and command palette remain fully available — voice is never a dead
  end.
- **Downgraded to a strictly more-private engine.** When an on-device default is
  available, a denied hosted request is held on the local engine with an explicit
  downgrade note. The active locality is at least as private as the requested
  one; it is never less private.

The same applies to unavailable providers and language packs:

- A requested provider that is unavailable downgrades to the on-device default,
  or blocks if none exists.
- A requested language pack that is not present on-device falls back to the
  on-device baseline language profile, disclosed explicitly, rather than reaching
  for a hosted engine to satisfy the language request.

Every downgrade or block carries a precise, non-generic disclosure label — never
a bare "unavailable" or "error".

## Guardrails

- Provider, locality, retention, export, language, policy, and entitlement state
  are always inspectable on a claimed voice profile.
- Local-first defaults stay visible; hosted/enterprise processing requires
  explicit disclosure and a compatible policy state.
- Switching provider or language never hides a retention or export change.
- Policy and entitlement denials block explicitly or downgrade to a strictly
  more-private engine — never a silent fallback to a broader or less private
  provider.
- Audio and transcript data never route outside the declared provider, locality,
  and retention model.
- A keyboard fallback is always available.

## Out of scope

This lane governs provider selection, language/profile metadata, retention/export
state, and policy/entitlement gating. It does not introduce unmanaged
third-party speech marketplaces or unqualified language matrices, and it does not
change the underlying ASR/TTS models.
