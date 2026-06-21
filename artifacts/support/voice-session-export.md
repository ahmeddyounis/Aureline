# Voice Session Support Export & Redaction

- Packet: `voice-support-export:stable:0001`
- Label: `Voice Session Support Export & Redaction`
- Minted: `2026-06-20T00:00:00Z`
- Sessions: 7 (3 with a failure, 3 with a blocked action, 1 with provider drift)
- Data minimization: raw audio in telemetry = false, raw transcript in telemetry = false, raw audio in crash packets = false, sensitive transcript in logs = false

## Session diagnostics

- **voice-session:local-dictation-clean:0001** — On-device dictation in the editor, nothing retained
  - mode = `dictation_mode_active`, provider = `on_device_local`, locality = `local_on_device`
  - retention = `no_audio_no_transcript_retained`, audio = `no_audio_retained`, export = `no_transcript_export`, policy = `user_controlled`
  - confidence = `high`, failure = `none`, blocked = `none`, drift = `no_drift_observed`
  - raw audio excluded = true, raw transcript excluded = true, keyboard fallback = true
- **voice-session:hosted-command-confirm-held:0001** — Disclosed hosted command session; a high-impact command was held for confirmation
  - mode = `command_mode_active`, provider = `approved_remote_disclosed`, locality = `hosted_remote_disclosed`
  - retention = `transcript_retained_redacted_in_support_bundle`, audio = `ephemeral_audio_local_only`, export = `explicit_user_export_redacted`, policy = `enterprise_policy_managed`
  - confidence = `medium`, failure = `none`, blocked = `high_impact_command_held_for_confirmation`, drift = `no_drift_observed`
  - raw audio excluded = true, raw transcript excluded = true, keyboard fallback = true
- **voice-session:provider-unreachable-fell-back-local:0001** — A hosted provider became unreachable; the session fell back to the on-device engine
  - mode = `command_mode_active`, provider = `on_device_local`, locality = `local_on_device`
  - retention = `no_audio_no_transcript_retained`, audio = `no_audio_retained`, export = `no_transcript_export`, policy = `user_controlled`
  - confidence = `medium`, failure = `hosted_provider_unreachable_fell_back_local`, blocked = `none`, drift = `provider_downgraded_to_local`
  - raw audio excluded = true, raw transcript excluded = true, keyboard fallback = true
- **voice-session:low-confidence-aborted:0001** — Dictation aborted because recognition confidence stayed below the usable threshold
  - mode = `dictation_mode_active`, provider = `on_device_local`, locality = `local_on_device`
  - retention = `ephemeral_audio_local_only_no_transcript_retained`, audio = `ephemeral_audio_local_only`, export = `no_transcript_export`, policy = `user_controlled`
  - confidence = `low`, failure = `recognition_low_confidence_aborted`, blocked = `none`, drift = `no_drift_observed`
  - raw audio excluded = true, raw transcript excluded = true, keyboard fallback = true
- **voice-session:policy-blocked:0001** — Policy blocks voice in this context; the keyboard path stays available
  - mode = `voice_mode_blocked_by_policy`, provider = `provider_disabled`, locality = `processing_unavailable`
  - retention = `retention_blocked_by_policy`, audio = `audio_capture_blocked`, export = `export_blocked_by_policy`, policy = `policy_blocked`
  - confidence = `none`, failure = `policy_blocked_capture`, blocked = `continuous_listening_blocked_by_policy`, drift = `no_drift_observed`
  - raw audio excluded = true, raw transcript excluded = true, keyboard fallback = true
- **voice-session:enterprise-relay-managed:0001** — Enterprise relay session with provider-contract retention; raw content still stays out of support
  - mode = `command_mode_active`, provider = `enterprise_relay_managed`, locality = `hosted_remote_disclosed`
  - retention = `transcript_retained_provider_per_contract`, audio = `audio_retained_provider_per_contract`, export = `provider_contract_retained`, policy = `enterprise_policy_managed`
  - confidence = `high`, failure = `none`, blocked = `none`, drift = `no_drift_observed`
  - raw audio excluded = true, raw transcript excluded = true, keyboard fallback = true
- **voice-session:dictation-target-unsupported:0001** — Dictation targeted a surface that does not accept dictated text; the action was held
  - mode = `dictation_mode_active`, provider = `on_device_local`, locality = `local_on_device`
  - retention = `no_audio_no_transcript_retained`, audio = `no_audio_retained`, export = `no_transcript_export`, policy = `user_controlled`
  - confidence = `medium`, failure = `none`, blocked = `dictation_target_surface_unsupported`, drift = `no_drift_observed`
  - raw audio excluded = true, raw transcript excluded = true, keyboard fallback = true

## Transcript export decisions

- `excluded_by_default` (metadata_only_support_export): Transcripts are excluded from support exports by default — only metadata classes are captured
  - redaction applied = false, reviewed by user = false, segments = 0
- `redacted_included_after_explicit_review` (explicit_user_export_redacted): User reviewed and exported 3 transcript segments with redaction applied
  - redaction applied = true, reviewed by user = true, segments = 3
  - redacted spans = 4 (email_address, long_numeric_sequence, absolute_path, credential_token)
- `blocked_by_policy` (export_blocked_by_policy): Transcript export is blocked by policy in this context
  - redaction applied = false, reviewed by user = false, segments = 0
- `no_transcript_available` (no_transcript_export): No transcript was produced for this session
  - redaction applied = false, reviewed by user = false, segments = 0
