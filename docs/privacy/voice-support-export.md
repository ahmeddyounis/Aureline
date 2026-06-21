# Voice session support export, telemetry posture, and transcript redaction

When a voice session does something a user needs explained — a hosted provider
falls back to the local engine, a high-impact command is held for confirmation,
recognition aborts on low confidence — support has to be able to **say what
happened** without a hidden debug switch and without ever ingesting raw audio or
raw transcript text. This doc is the human-readable face of how Aureline keeps
voice supportable while keeping it data-minimizing.

The machine truth is the voice support-export packet built by
`crates/aureline-support/src/voice_redaction/`. Diagnostics, support export,
Help/About, and release surfaces ingest that packet rather than cloning voice
state text by hand. Its boundary schema is
[`transcript-redaction-and-support-export`](../../schemas/voice/transcript-redaction-and-support-export.schema.json),
and the class tokens mirror the
[voice-session](../../schemas/voice/voice-session.schema.json) and
[retention-and-export](../../schemas/voice/retention-and-export.schema.json)
schemas exactly.

## No raw audio or transcript by default

The default support, telemetry, crash, and log paths carry **metadata class
tokens only** — never raw audio bytes and never transcript content:

- **Telemetry and crash packets** never carry raw audio. Logs never carry
  sensitive transcript content. The telemetry posture pins all four of those
  flags off and is rejected if any is set.
- **Each session diagnostics row** pins `raw_audio_excluded` and
  `raw_transcript_excluded` true. A row that flips either is invalid.
- What *is* captured is enough to explain the session: the mode (command vs
  dictation vs idle vs blocked), the provider class, the processing locality, the
  retention/audio/export posture, the policy state, an **aggregate recognition
  confidence class** (a coarse `high`/`medium`/`low` cue, never content), and the
  failure, blocked-action, and provider-drift classes.

## Diagnostics explain behavior without a debug switch

Each [session diagnostics row](../../schemas/voice/transcript-redaction-and-support-export.schema.json)
records, as typed class tokens:

- **Failure class** — why a session did not complete normally (for example
  `recognition_low_confidence_aborted`, `hosted_provider_unreachable_fell_back_local`,
  `policy_blocked_capture`).
- **Blocked-action class** — when a spoken action was intentionally held or denied
  rather than silently applied (for example
  `high_impact_command_held_for_confirmation`,
  `dictation_target_surface_unsupported`).
- **Provider-drift class** — the observed difference between the requested
  provider/locality/profile and the active one (for example
  `provider_downgraded_to_local`). By contract drift only ever moves toward a
  **more-private** posture; there is no drift token that widens authority or moves
  to a more remote provider.

Support can read these directly and narrate the session. No hidden flag changes
what gets recorded.

## Transcript export is explicit, reviewed, redacted, and bounded

Transcript text is excluded from support by default. The only path that includes
any transcript is an **explicit, user-reviewed, redacted, bounded** export, and
it is always visibly labeled. The export decision records its
`inclusion_state`:

- `excluded_by_default` — only metadata is captured; no text leaves.
- `redacted_included_after_explicit_review` — the user reviewed and chose a
  bounded set of segments, redaction was applied first, and the decision carries a
  user-visible label. Even here, **only a content-free redaction summary**
  (how many spans were masked, of which classes) travels into the support packet —
  never the text itself.
- `blocked_by_policy` — export is blocked in this context.
- `no_transcript_available` — the session produced no transcript.

Redaction masks emails, long numeric sequences, absolute paths, URLs, IPv4
addresses, and credential-like tokens before any text leaves the device. The
residual redacted text flows only to the user's own export destination; the
support packet keeps the summary.

## Guardrails

- Raw audio and sensitive transcript content stay out of telemetry, crash
  packets, logs, and support exports by default.
- Diagnostics capture mode, provider, locality, confidence class, and failure /
  blocked-action / provider-drift classes — enough to explain a session without
  raw content.
- Transcript export is explicit, reviewed, redacted, bounded, and visibly
  labeled; only a content-free redaction summary enters support.
- Supportability convenience never widens retention by default.
- Provider drift only ever moves toward a more-private posture.
- The keyboard / command-palette fallback is always available.

## Out of scope

This lane governs the support, telemetry, and diagnostics **boundary** for voice
sessions and the redaction of explicitly exported transcripts. It does not change
provider selection or the live retention model (see
[voice processing, provider routing, and retention](./voice-processing-and-retention.md)),
and it never emits sensitive speech content into logs, analytics, or crash
packets.
