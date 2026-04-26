# Voice, dictation, and speech-privacy contract

This document freezes the cross-surface contract every Aureline
**speech provider entry**, **local speech pack**, **speech provider
handoff disclosure**, **grammar / intent resolver**, **editor
integration bridge**, **voice-mode session**, **voice-mode
transition**, **mic-indicator state**, **speech capture event**,
**transcript correction**, and **transcript export** resolves
through before any voice or dictation lane appears in the shell.
The goal is a speech-input surface that stays an **explicit
interaction mode** with disclosed audio locality, disclosed
provider handoff, disclosed retention, and full parity with the
keyboard-driven command lane — never a hidden assistant magic
that bypasses the command, trust, or undo model.

The machine-readable schemas live at:

- [`/schemas/ux/speech_provider_entry.schema.json`](../../schemas/ux/speech_provider_entry.schema.json)
- [`/schemas/ux/voice_mode_state.schema.json`](../../schemas/ux/voice_mode_state.schema.json)
- [`/schemas/ux/speech_privacy_ledger.schema.json`](../../schemas/ux/speech_privacy_ledger.schema.json)

The companion fixtures live under:

- [`/fixtures/ux/voice_cases/`](../../fixtures/ux/voice_cases/)

This contract is normative for the speech-provider abstraction,
local-pack verification, handoff disclosure, voice-mode separation
(dictation vs command), persistent mic-state cues, continuous-
listening opt-in, raw-audio / transcript retention defaults,
correction-before-commit posture, command preview and confirmation
posture for privileged actions, transcript-export redaction posture,
fallback behavior when a speech pack is unavailable, and command-
graph parity (undo stack, command_id identity, trust model,
accessibility announcements). Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, or milestone document, those sources win and this
document plus its companion schemas and fixtures update in the same
change. Where a downstream voice, dictation, or transcript surface
mints a parallel vocabulary, this contract wins and the surface is
non-conforming.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference:

- [`/docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md),
  [`/schemas/commands/command_descriptor.schema.json`](../../schemas/commands/command_descriptor.schema.json),
  and
  [`/schemas/commands/command_registry_entry.schema.json`](../../schemas/commands/command_registry_entry.schema.json)
  — canonical `command_id`, alias lifecycle, capability scope class,
  preview class, approval posture, enablement decision, disabled-
  reason vocabulary. Every voice-triggered action, every dictation-
  initiated palette open, every grammar / intent resolver row, and
  every command-preview confirmation lands on a stable `command_id`
  on the registry. No private mutation path, no voice-only verb
  outside the registry, and no inline edit lane outside the editor
  integration bridge are admissible.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  and
  [`/schemas/security/trust_decision_packet.schema.json`](../../schemas/security/trust_decision_packet.schema.json)
  — `trust_state`, `trust_decision_record`, restricted-mode posture.
  A voice-mode session inherits the workspace trust state of its
  invocation surface; speech cannot widen trust, cannot bypass a
  restricted-mode block, and cannot mint an inline trust grant.
- [`/docs/ai/provider_model_registry_contract.md`](../ai/provider_model_registry_contract.md),
  [`/schemas/ai/provider_registry.schema.json`](../../schemas/ai/provider_registry.schema.json),
  and the AI model registry — `provider_class`, `execution_locus_class`,
  `transport_class`, `auth_mode_class`, `retention_stance_class`,
  `region_posture_class`, `pack_origin_class`,
  `pack_verification_state_class`. Speech providers re-project the
  same provenance lanes (local in-process, local sandbox, local
  companion, BYOK direct, enterprise-gateway-brokered, vendor-
  hosted-first-party-managed, extension-provided, mocked, disabled);
  a speech route that mints a parallel provenance vocabulary is
  non-conforming.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — `browser_handoff_packet`. A speech route that opens a connected-
  provider authorization or sign-in flow rides this packet; raw URL
  launches from a voice command are forbidden.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  and
  [`/schemas/ux/interaction_safety.schema.json`](../../schemas/ux/interaction_safety.schema.json)
  — preview / apply / revert phases, typed permission prompts,
  representation-labeled copy / export, focus return, authority
  class, consequence class. A voice-issued privileged action lands
  on the same preview-required / approval-required posture a
  keyboard-issued command lands on; voice cannot collapse a preview
  phase, downgrade a consequence class, or mint a private "I just
  said it" approval.
- [`/docs/accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md)
  — keyboard-completeness, focus-order, screen-reader announcements,
  IME / dead-key / bidi / AltGr / emoji input. Dictation rides the
  shell's text-input normalization lane; speech-triggered command
  announcements ride the same screen-reader announcement lane as
  keyboard-issued commands.
- [`/docs/ux/localization_and_locale_pack_contract.md`](./localization_and_locale_pack_contract.md)
  — locale-pack manifest, source-language fallback. A grammar /
  intent resolver pack and a speech ASR pack each disclose locale
  fallback through the locale-pack lanes; silent locale degradation
  is non-conforming.
- [`/docs/ux/learning_and_presentation_contract.md`](./learning_and_presentation_contract.md)
  — onboarding, learning-mode, command-tip pack, presentation-mode
  layout anchors. A "voice tour" or a "say this to try voice"
  command tip rides the guided-tour and command-tip-pack lanes; no
  private voice-only teaching shell is admissible.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — `deployment_profile_id` vocabulary so air-gapped, managed-cloud,
  and restricted envelopes inherit suppression posture mechanically.
  Air-gapped envelopes admit only `local_in_process_speech`,
  `local_sandbox_process_speech`, and `local_companion_service_speech`
  speech routes; remote and vendor-hosted speech routes resolve
  `disabled_no_speech_provider` or `blocked_by_policy`.

## Who reads this contract

- **Speech provider authors and pack publishers** — to declare a
  provider entry's `speech_provider_class`, `execution_locus_class`,
  `transport_class`, `auth_mode_class`, `retention_stance_class`,
  `region_posture_class`, `supported_feature_class`, and
  `allowed_data_class` set; to declare a local speech pack's
  `pack_origin_class` and `pack_verification_state_class`; and to
  declare a grammar / intent resolver's mapping back to canonical
  `command_id_ref`s.
- **Voice-mode authors** — to emit `voice_mode_session_record`s
  that name the active mode (idle, dictation, command, continuous-
  listening), the persistent mic-indicator state, the activation
  class (push-to-talk hold, push-to-talk toggle, wake phrase
  continuous, manual command activation), the command-preview
  posture, the transcript-correction posture, and the accessibility
  announcement class for the session. Voice mode is one explicit,
  inspectable state; conflating dictation and command into a single
  ambiguous "you said something" lane is non-conforming.
- **Editor integration bridge authors** — to wire dictated text and
  speech-triggered commands into the same undo stack, the same
  command_id identity, the same trust model, and the same
  accessibility announcement lane the keyboard route uses. A
  bridge row that mints a private "voice undo" stack, a private
  "voice apply" mutation lane, or a private "voice trust" grant is
  non-conforming.
- **Privacy ledger authors** — to emit `speech_capture_event_record`s
  for every audio capture window, `transcript_correction_record`s
  for every transcript edit before commit, and `transcript_export_record`s
  for every export, each carrying the audio locality class, the
  transcript locality class, the export redaction class, and the
  consent class. Audio that leaves the device, transcripts that are
  retained, and exports that are not redacted MUST each be visible
  on the ledger before they happen.
- **Support, admin-envelope, and policy authors** — to suppress
  voice surfaces that cannot be honoured under the current envelope
  (policy-disabled speech, retention blocked, region blocked,
  consent missing, locale unavailable, imported pack unverified,
  trust restricted) with a typed denial reason rather than silent
  omission.

## Why this exists

Without this contract, speech and dictation drift the fastest into
hidden assistant magic:

- a "tap to speak" affordance opens a remote ASR endpoint without
  disclosing that audio is leaving the device, breaking the local-
  first promise;
- a wake-phrase listener stays on after the user thought they
  toggled it off because no persistent mic-indicator row was
  declared, so the user has no inspectable cue for "is the mic
  capturing right now?";
- the same say-that triggers both dictation (text into the editor)
  and a command (apply that refactor) depending on hidden context,
  so the user cannot tell whether their words are about to become
  text or about to mutate the workspace;
- a voice-issued "delete this folder" jumps the preview / apply /
  revert phase a keyboard-issued delete would have ridden, because
  voice surfaces a private apply lane;
- a transcript with a misheard secret value is auto-committed into
  the editor before the user could correct it, leaving the secret
  in the buffer history with no governed redaction lane;
- a "share session" action exports the transcript verbatim into a
  support bundle without a redaction class, so the support bundle
  carries raw spoken content;
- a continuous-listening "always on" feature is enabled by default
  on first launch, with no explicit opt-in lane and no per-session
  inspection;
- a remote provider's speech endpoint widens its retention contract
  silently between two sessions, and the user has no provenance
  row to compare against;
- when a local speech pack is unavailable, the surface silently
  falls back to a remote vendor without disclosure, so the user
  thinks they are still local-only;
- a voice command bypasses the workspace trust state and runs in a
  restricted workspace anyway, because the speech route did not
  read the trust packet;
- a dictated edit lands without a screen-reader announcement, so
  an assistive-technology user has no parity cue with the keyboard
  lane;
- a voice-issued action does not show up in the same undo stack as
  keyboard actions, so undo cannot reach it.

This contract closes all of those gaps by declaring three object
families — speech provider entries / local packs / handoff
disclosures / grammar resolvers / editor integration bridges on
one schema, voice-mode sessions / transitions / mic-indicator
states / invariants on a second schema, and speech capture events
/ transcript corrections / transcript exports / privacy-ledger
manifests on a third schema — plus one closed denial-reason set
per schema and one explicit binding to the command-registry, AI
provider-registry, workspace-trust, shell-interaction-safety,
accessibility, localization, and connected-provider-handoff truth
models. Voice and dictation remain a thin, inspectable layer over
canonical workspace surfaces.

## 1. Record kinds

### 1.1 `speech_provider_entry_record`, `local_speech_pack_entry_record`, `speech_provider_handoff_disclosure_record`, `grammar_intent_resolver_record`, `editor_integration_bridge_record`

Frozen on
[`/schemas/ux/speech_provider_entry.schema.json`](../../schemas/ux/speech_provider_entry.schema.json).

- One **`speech_provider_entry_record`** per registered speech
  (ASR / TTS / hybrid) provider. Names the `speech_provider_class`,
  the `execution_locus_class`, the `transport_class`, the
  `auth_mode_class`, the `retention_stance_class`, the
  `region_posture_class`, the `supported_feature_class` set, the
  `allowed_data_class` set, the deployment-profile envelope, the
  policy context, the redaction class, and the optional
  `local_speech_pack_ref` for offline serving.
- One **`local_speech_pack_entry_record`** per local speech pack.
  Names the `pack_origin_class`, the `pack_verification_state_class`,
  the locale, the deployment-profile envelope, and the trust
  posture; an unverified pack MUST NOT serve speech turns.
- One **`speech_provider_handoff_disclosure_record`** per
  invocation. Minted **before** the call so the same provenance
  axes (provider identity, locus, transport, auth mode, retention,
  region, allowed data classes, declared mode set) are inspectable
  at the point of intent. Silent widening of any axis after the
  disclosure is non-conforming.
- One **`grammar_intent_resolver_record`** per grammar / intent
  resolver. Names a closed set of `voice_intent_record`s, each
  resolving to exactly one canonical `command_id_ref` (or denying
  with a typed reason). A grammar entry that resolves to a
  non-canonical verb suppresses; voice cannot mint commands.
- One **`editor_integration_bridge_record`** per bridge between a
  voice-mode session and the editor / command lane. Declares the
  `undo_stack_parity_class`, the `command_id_parity_class`, the
  `trust_model_parity_class`, the `accessibility_announcement_class`,
  and the optional editor-canvas `surface_ref`. A bridge that
  diverges from the keyboard lane is non-conforming.

### 1.2 `voice_mode_session_record`, `voice_mode_transition_record`, `mic_indicator_state_record`, `voice_mode_invariant_manifest_record`

Frozen on
[`/schemas/ux/voice_mode_state.schema.json`](../../schemas/ux/voice_mode_state.schema.json).

- One **`voice_mode_session_record`** per voice session. Names the
  `voice_mode_class` (idle, dictation, command, continuous-
  listening), the `activation_class` (push-to-talk hold, push-to-
  talk toggle, wake-phrase continuous, manual command activation),
  the `mic_indicator_class`, the `command_preview_class`, the
  `transcript_correction_posture`, the `accessibility_announcement_class`,
  the bound `speech_provider_entry_ref`, the bound
  `editor_integration_bridge_ref`, the optional
  `grammar_intent_resolver_ref`, the bound `trust_decision_ref` (so
  the session inherits the workspace trust state), the deployment-
  profile envelope, the policy context, and the redaction class.
- One **`voice_mode_transition_record`** per transition between
  voice-mode classes (idle → dictation, dictation → command,
  command → idle, opt-in to continuous listening, opt-out from
  continuous listening, blocked by policy, blocked by envelope).
- One **`mic_indicator_state_record`** per render of the persistent
  mic-state cue. Names the `mic_indicator_class`, the bound
  session ref, the indicator visibility, and the layout target
  (settings_pane, primary_editor adjunct, status zone, etc.).
- One **`voice_mode_invariant_manifest_record`** aggregator
  declaring the ten const-true invariants for a voice-mode session
  set.

### 1.3 `speech_capture_event_record`, `transcript_correction_record`, `transcript_export_record`, `speech_privacy_ledger_manifest_record`

Frozen on
[`/schemas/ux/speech_privacy_ledger.schema.json`](../../schemas/ux/speech_privacy_ledger.schema.json).

- One **`speech_capture_event_record`** per audio capture window.
  Names the `audio_locality_class`, the `transcript_locality_class`,
  the `consent_class`, the bound voice-mode session ref, the bound
  speech-provider entry ref, and the policy context. Captures
  whose locality or consent class would otherwise drift after the
  fact MUST be re-emitted as a new event rather than mutated in
  place.
- One **`transcript_correction_record`** per correction-before-
  commit event. Names the `correction_origin_class` (user inline
  correction, user palette correction, transcript committed
  without correction by user choice, transcript committed without
  correction blocked by envelope), the bound capture-event ref,
  and the optional `command_descriptor_ref` so a correction that
  changed the resolved command intent is auditable.
- One **`transcript_export_record`** per export. Names the
  `export_redaction_class`, the destination class (support bundle,
  portable profile package, evidence packet, no-export local-only),
  the bound capture-event refs, the consent class for the export,
  and the policy context.
- One **`speech_privacy_ledger_manifest_record`** aggregator
  declaring the eight const-true invariants for a privacy-ledger
  set.

## 2. Closed vocabularies (frozen)

All vocabularies below are frozen as closed enums on the schema
files. Adding a new value is additive-minor and bumps the
companion `*_schema_version`; repurposing a value is breaking
and requires a new decision row.

### 2.1 Provider / pack / resolver / bridge vocabularies (`speech_provider_entry.schema.json`)

| Vocabulary                            | Values                                                                                                                                                                                                                              |
|---------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `speech_provider_class`               | `first_party_self_hosted_speech`, `first_party_managed_speech`, `connected_provider_vendor_speech`, `connected_provider_self_hosted_speech`, `byok_speech`, `enterprise_gateway_brokered_speech`, `extension_provided_speech`, `mocked_test_speech`, `disabled_no_speech_provider` |
| `execution_locus_class`               | `local_in_process_speech`, `local_sandbox_process_speech`, `local_companion_service_speech`, `byok_remote_vendor_direct_speech`, `byok_remote_self_hosted_direct_speech`, `enterprise_gateway_brokered_locus_speech`, `vendor_hosted_first_party_managed_speech`, `extension_provided_locus_speech`, `mocked_test_locus_speech`, `disabled_no_locus_speech` |
| `transport_class`                     | `in_process_call`, `local_stdio_pipe`, `local_unix_domain_socket`, `local_named_pipe`, `local_http_loopback`, `local_grpc_loopback`, `remote_https`, `remote_grpc_over_tls`, `extension_mediated_transport`, `mocked_test_transport`, `no_transport_disabled` |
| `auth_mode_class`                     | `no_auth_local`, `byok_user_credential`, `oauth_user_authorized`, `mtls_enterprise`, `enterprise_gateway_brokered_auth`, `extension_provided_auth`, `mocked_test_auth`, `no_auth_disabled` |
| `retention_stance_class`              | `no_audio_retained_no_transcript_retained`, `ephemeral_audio_local_only_no_transcript_retained`, `transcript_retained_local_only`, `transcript_retained_redacted_in_support_bundle`, `transcript_retained_provider_per_contract`, `retention_blocked_by_policy`, `retention_unavailable_in_envelope` |
| `region_posture_class`                | `local_machine_only_no_region`, `local_region_user_machine`, `customer_managed_region`, `vendor_managed_region_named`, `vendor_managed_region_unspecified`, `region_unavailable_in_envelope` |
| `supported_feature_class`             | `dictation_supported`, `command_mode_supported`, `push_to_talk_supported`, `continuous_listening_supported`, `transcript_correction_supported`, `redaction_on_capture_supported`, `tts_readback_supported`, `none_supported_disabled` |
| `data_class`                          | `raw_audio_bytes`, `transcript_text`, `partial_transcript_text`, `command_intent_payload`, `voice_print_or_speaker_embedding`, `none_disabled` |
| `pack_origin_class`                   | `first_party_signed_pack`, `vendor_signed_pack_distributed_via_first_party`, `byok_user_imported_pack`, `enterprise_managed_pack`, `extension_provided_pack`, `mocked_test_pack` |
| `pack_verification_state_class`       | `signature_verified`, `signature_pending`, `signature_failed`, `signature_unsupported_disabled`, `signature_unavailable_offline_only_self_signed` |
| `pre_invocation_disclosure_kind`      | `local_capture_only_no_handoff`, `byok_remote_handoff_disclosed`, `enterprise_gateway_handoff_disclosed`, `vendor_hosted_handoff_disclosed`, `extension_provided_handoff_disclosed`, `handoff_blocked_by_policy`, `handoff_unavailable_in_envelope` |
| `voice_intent_resolution_class`       | `resolves_to_canonical_command_id`, `resolves_to_dictation_text_only`, `resolves_to_no_command_disambiguation_required`, `resolution_denied_uncanonical_verb`, `resolution_denied_low_confidence`, `resolution_blocked_by_envelope` |
| `undo_stack_parity_class`             | `same_undo_stack_as_keyboard_lane`, `dedicated_dictation_segment_within_unified_undo_stack`, `undo_unavailable_inert_metadata_only`, `parity_blocked_by_envelope` |
| `command_id_parity_class`             | `same_command_ids_as_keyboard_lane`, `command_ids_re_exported_with_voice_aliases`, `parity_blocked_by_envelope` |
| `trust_model_parity_class`            | `inherits_workspace_trust_state_strict`, `inherits_workspace_trust_state_with_envelope_narrowing`, `parity_blocked_by_envelope` |
| `accessibility_announcement_class`    | `announces_via_screen_reader_lane`, `announces_via_keyboard_announcement_lane`, `announces_via_visual_indicator_only`, `announcement_blocked_by_envelope` |

### 2.2 Voice-mode vocabularies (`voice_mode_state.schema.json`)

| Vocabulary                            | Values                                                                                                                                                                                                                              |
|---------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `voice_mode_class`                    | `idle_microphone_off`, `dictation_mode_active`, `command_mode_active`, `continuous_listening_active_user_opted_in`, `voice_mode_blocked_by_policy`, `voice_mode_blocked_by_envelope` |
| `activation_class`                    | `push_to_talk_held`, `push_to_talk_toggle`, `wake_phrase_continuous_user_opted_in`, `manual_command_activation`, `activation_blocked_by_policy`, `activation_unavailable_in_envelope` |
| `mic_indicator_class`                 | `persistent_indicator_visible_capture_active`, `persistent_indicator_visible_capture_idle`, `persistent_indicator_hidden_capture_disabled`, `persistent_indicator_unavailable_degraded` |
| `command_preview_class`               | `preview_required_for_privileged_actions`, `preview_optional_for_reversible_local_actions`, `preview_skipped_for_inert_metadata_only`, `preview_blocked_by_envelope` |
| `transcript_correction_posture`       | `correction_required_before_commit`, `correction_optional_before_commit`, `correction_unavailable_capture_only`, `correction_blocked_by_envelope` |
| `voice_mode_transition_kind`          | `idle_to_dictation`, `idle_to_command`, `dictation_to_command`, `command_to_dictation`, `dictation_to_idle`, `command_to_idle`, `opt_in_continuous_listening`, `opt_out_continuous_listening`, `transition_blocked_by_policy`, `transition_blocked_by_envelope` |

### 2.3 Privacy-ledger vocabularies (`speech_privacy_ledger.schema.json`)

| Vocabulary                            | Values                                                                                                                                                                                                                              |
|---------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `audio_locality_class`                | `local_machine_only_in_process`, `local_machine_only_sandbox_process`, `local_machine_only_companion_service`, `sent_to_byok_remote_vendor_direct`, `sent_to_byok_remote_self_hosted_direct`, `sent_to_enterprise_gateway`, `sent_to_vendor_hosted_first_party_managed`, `audio_blocked_by_policy`, `audio_unavailable_in_envelope` |
| `transcript_locality_class`           | `transcript_local_only`, `transcript_returned_from_remote_provider_local_persisted`, `transcript_returned_from_remote_provider_ephemeral`, `transcript_blocked_by_policy`, `transcript_unavailable_in_envelope` |
| `consent_class`                       | `explicit_user_opt_in_each_session`, `explicit_user_opt_in_remembered_per_profile`, `implicit_user_initiated_push_to_talk`, `no_consent_required_local_inert_metadata_only`, `consent_blocked_by_policy`, `consent_unavailable_in_envelope` |
| `correction_origin_class`             | `user_inline_correction`, `user_palette_correction`, `transcript_committed_without_correction_user_choice`, `transcript_committed_without_correction_blocked_by_envelope`, `correction_skipped_inert_metadata_only` |
| `export_redaction_class`              | `redacted_metadata_only`, `redacted_full_transcript_blocked`, `redacted_partial_with_disclosure`, `exported_unredacted_with_explicit_consent`, `export_blocked_by_policy`, `export_unavailable_in_envelope` |
| `export_destination_class`            | `support_bundle_metadata_safe_default`, `portable_profile_package_redacted`, `evidence_packet_redacted`, `no_export_local_only`, `export_blocked_by_policy_destination`, `export_destination_unavailable_in_envelope` |

### 2.4 Shared deployment / redaction / policy

`deployment_profile_id`, `redaction_class`, and `policy_context`
re-export verbatim from
[`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
and the shell-interaction-safety / settings-resolver lanes; this
contract mints no parallel envelope vocabulary.

## 3. Speech provider entries, local packs, handoff disclosure, grammar / intent resolvers, editor integration bridges

Rules (frozen):

1. **Every invocation discloses provider identity before the call.**
   A speech turn that lands without a matching
   `speech_provider_handoff_disclosure_record` minted at or before
   the invocation moment is non-conforming
   (`denial_reason = speech_provider_handoff_undisclosed`). The
   disclosure carries provider identity, locus, transport, auth
   mode, retention stance, region posture, allowed data classes,
   and supported features; silent widening of any axis after the
   disclosure is non-conforming
   (`denial_reason = speech_provider_retention_widened_after_disclosure`).
2. **Local packs are signature-verified before they serve.** A
   `local_speech_pack_entry_record` whose
   `pack_verification_state_class` is `signature_pending`,
   `signature_failed`, or
   `signature_unsupported_disabled` MUST NOT be cited as the
   serving pack on a `speech_provider_entry_record`
   (`denial_reason = speech_provider_unverified_pack_admitted`).
3. **Air-gapped envelopes admit only local loci.** A
   `speech_provider_entry_record` whose `deployment_profile_refs`
   contain `air_gapped` or `air_gapped_mirror_only` MUST resolve
   `execution_locus_class` to one of `local_in_process_speech`,
   `local_sandbox_process_speech`, or
   `local_companion_service_speech`
   (`denial_reason = speech_provider_envelope_mismatch`).
4. **Grammar entries resolve to canonical commands.** A
   `voice_intent_record` on a `grammar_intent_resolver_record`
   whose `voice_intent_resolution_class` is
   `resolves_to_canonical_command_id` MUST populate
   `command_id_ref`; a row whose resolution class is
   `resolution_denied_uncanonical_verb` denies with
   `speech_provider_grammar_resolves_to_uncanonical_command`.
   Grammar cannot mint commands outside the registry.
5. **Editor integration bridges enforce parity.** A
   `editor_integration_bridge_record` whose
   `undo_stack_parity_class` is anything other than
   `same_undo_stack_as_keyboard_lane` or
   `dedicated_dictation_segment_within_unified_undo_stack` is
   non-conforming
   (`denial_reason = speech_provider_action_opens_private_mutation_path`).
   `command_id_parity_class` MUST be
   `same_command_ids_as_keyboard_lane` or
   `command_ids_re_exported_with_voice_aliases`; a bridge that
   mints a private "voice verb" lane is non-conforming.
6. **No private mutation paths.** A grammar / intent resolution
   that mutates the workspace MUST resolve through a canonical
   `command_id_ref`; a resolver that opens an inline edit lane
   outside the editor integration bridge is non-conforming
   (`denial_reason = speech_provider_action_opens_private_mutation_path`).

## 4. Voice-mode separation, indicators, activation, preview, correction, parity

Rules (frozen):

1. **Dictation and command modes are separate.** A
   `voice_mode_session_record` carries exactly one
   `voice_mode_class` at any moment, and a transition between
   modes emits a `voice_mode_transition_record` with one of the
   closed `voice_mode_transition_kind` values
   (`denial_reason = voice_mode_dictation_and_command_conflated`).
2. **Continuous listening is opt-in.** A session whose
   `voice_mode_class = continuous_listening_active_user_opted_in`
   MUST carry an `activation_class = wake_phrase_continuous_user_opted_in`
   and a corresponding consent ledger entry on the privacy ledger
   (`denial_reason = voice_mode_continuous_listening_without_opt_in`).
3. **Persistent mic indicator is visible during active capture.**
   While a session is in any `*_mode_active` state, its bound
   `mic_indicator_state_record` MUST resolve
   `mic_indicator_class = persistent_indicator_visible_capture_active`
   (`denial_reason = voice_mode_indicator_hidden_during_active_capture`).
4. **Command preview is required for privileged actions.** A
   command-mode session whose resolved command's
   `capability_scope_class` is `recoverable_durable_mutation`,
   `externally_visible_mutation`, `irreversible_high_blast_mutation`,
   `credential_or_secret_bearing`, `managed_workspace_control`, or
   `policy_authoring_or_waiver` MUST carry
   `command_preview_class = preview_required_for_privileged_actions`
   (`denial_reason = voice_mode_command_preview_skipped_for_privileged_action`).
5. **Transcript correction window precedes commit.** A
   dictation-mode session whose
   `transcript_correction_posture = correction_required_before_commit`
   MUST emit at least one `transcript_correction_record` per
   transcript segment that lands in the editor; a transcript that
   commits without a correction window denies with
   `voice_mode_transcript_commit_without_correction_window`.
6. **Undo stack parity holds.** A `voice_mode_session_record`'s
   bound `editor_integration_bridge_ref` MUST resolve to a bridge
   whose `undo_stack_parity_class` is one of the admitted parity
   values; a session whose voice-issued mutations do not appear on
   the keyboard undo stack denies with
   `voice_mode_undo_stack_diverged_from_keyboard_lane`.
7. **Trust model parity holds.** A session inherits the workspace
   trust state of its bound `trust_decision_ref`; voice cannot
   widen trust, cannot bypass restricted mode, and cannot grant
   trust inline. A session whose
   `trust_model_parity_class = parity_blocked_by_envelope` MUST
   suppress mutating voice commands.
8. **Accessibility announcement parity holds.** A voice-issued
   command MUST emit the same screen-reader / keyboard
   announcement a keyboard-issued command of the same intent
   would have emitted; a session that omits the announcement
   denies with `voice_mode_accessibility_announcement_missing`.

## 5. Audio locality, transcript locality, retention, correction, export, consent

Rules (frozen):

1. **Audio that leaves the device is disclosed.** A
   `speech_capture_event_record` whose `audio_locality_class` is
   any `sent_to_*` value MUST be paired with a
   `speech_provider_handoff_disclosure_record` whose
   `pre_invocation_disclosure_kind` is the matching `*_handoff_disclosed`
   value
   (`denial_reason = speech_privacy_audio_left_device_without_disclosure`).
2. **Transcripts retained beyond the declared retention class
   deny.** A capture whose bound provider's
   `retention_stance_class` is
   `no_audio_retained_no_transcript_retained` or
   `ephemeral_audio_local_only_no_transcript_retained` MUST NOT
   emit a transcript-export record naming a destination class
   that retains transcript bodies
   (`denial_reason = speech_privacy_transcript_retained_beyond_declared_class`).
3. **Unredacted export requires explicit consent.** A
   `transcript_export_record` whose
   `export_redaction_class = exported_unredacted_with_explicit_consent`
   MUST carry a matching consent class on the bound capture event;
   silent unredacted export denies with
   `speech_privacy_export_unredacted_without_consent`.
4. **Privileged commits go through correction.** A capture event
   whose bound voice-mode session resolves a privileged command
   MUST emit a `transcript_correction_record` with
   `correction_origin_class` other than
   `transcript_committed_without_correction_user_choice` and other
   than `correction_skipped_inert_metadata_only`
   (`denial_reason = speech_privacy_correction_skipped_for_privileged_commit`).
5. **Consent class is declared, not inferred.** A capture event
   that omits `consent_class` denies with
   `speech_privacy_consent_class_inferred_not_declared`. Implicit
   push-to-talk MUST resolve to
   `implicit_user_initiated_push_to_talk` rather than be inferred
   from absence.
6. **Fallback when speech pack is unavailable.** A
   `speech_provider_entry_record` whose `local_speech_pack_ref`
   resolves to a pack with `pack_verification_state_class` not in
   `signature_verified` MUST resolve `speech_provider_class` to
   `disabled_no_speech_provider` and the surface MUST suppress
   voice activation rather than silently fall back to a remote
   route. A grammar / intent resolver that nevertheless attempts
   to dispatch denies with
   `speech_provider_grammar_resolves_to_uncanonical_command`.

## 6. Evidence-hook reserves

Five evidence-hook reserves are admitted on speech-provider,
local-pack, handoff-disclosure, grammar-resolver, editor-bridge,
voice-mode session, transition, mic-indicator, capture-event,
correction, export, and manifest records so a later audit can be
qualified without redefining object shape:

- `speech_handoff_audit` — reserves space for a packet that walks
  every handoff disclosure back to its provider entry and reports
  any axis-widening between the two.
- `local_pack_continuity` — reserves space for a packet that
  reports whether the local speech pack a provider entry resolves
  against has remained continuous (no missed revision, no
  signature regression, no offline-only-without-disclosure window).
- `voice_mode_parity_review` — reserves space for a packet that
  reports the undo-stack, command_id, trust-model, and
  accessibility-announcement parity for a voice-mode session
  against the keyboard lane.
- `transcript_correction_audit` — reserves space for a packet that
  proves every privileged commit on a session went through a
  correction-before-commit window.
- `privacy_bounded_export_artifact` — reserves space for a packet
  that reports what transcript-export artifacts a privacy ledger
  retained, under what export-redaction class, with no raw
  transcript body leaked.

The reserve is additive-optional; a row that does not need a hook
omits it and does not bump the schema version.

## 7. Invariants

Every `voice_mode_invariant_manifest_record` MUST declare the
following const-true invariants:

1. `dictation_and_command_modes_are_separate_states`
2. `continuous_listening_requires_explicit_opt_in`
3. `persistent_mic_indicator_visible_during_active_capture`
4. `command_preview_required_for_privileged_voice_actions`
5. `transcript_correction_window_precedes_privileged_commit`
6. `voice_actions_share_undo_stack_with_keyboard_lane`
7. `voice_actions_resolve_to_canonical_command_ids`
8. `voice_actions_inherit_workspace_trust_state`
9. `voice_actions_emit_keyboard_parity_accessibility_announcements`
10. `evidence_hooks_reserved_for_voice_mode_parity_review`

Every `speech_privacy_ledger_manifest_record` MUST declare the
following const-true invariants:

1. `audio_leaving_device_disclosed_at_or_before_capture`
2. `transcript_retention_matches_provider_retention_stance`
3. `unredacted_export_requires_explicit_consent`
4. `privileged_commit_passes_through_correction_window`
5. `consent_class_declared_not_inferred`
6. `local_pack_unverified_forces_voice_suppression_not_silent_remote_fallback`
7. `transcript_export_redaction_class_matches_destination`
8. `evidence_hooks_reserved_for_privacy_bounded_export_artifact`

## 8. Acceptance mapping

| Acceptance clause                                                                                                                                  | Resolved by                                                                                                                                                  |
|----------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Reviewers can tell whether audio stays local or leaves the device.                                                                                | §2.1 `execution_locus_class` / `transport_class`, §2.3 `audio_locality_class`, §3 rule 1, §5 rule 1; fixtures `push_to_talk_local_dictation.yaml`, `remote_speech_provider_explicit_consent.yaml`. |
| Reviewers can tell whether the system is in dictation vs command mode.                                                                            | §2.2 `voice_mode_class` / `voice_mode_transition_kind`, §4 rule 1; fixtures `command_mode_privileged_action_confirmation.yaml`, `push_to_talk_local_dictation.yaml`. |
| Reviewers can tell whether a browser or provider handoff occurred before using the feature.                                                       | §2.1 `pre_invocation_disclosure_kind`, §1.1 handoff disclosure record, §3 rule 1; fixture `remote_speech_provider_explicit_consent.yaml`.                  |
| Reviewers can tell whether transcripts will be retained or exported before using the feature.                                                     | §2.1 `retention_stance_class`, §2.3 `transcript_locality_class` / `export_redaction_class` / `export_destination_class`, §5 rules 2, 3; fixture `transcript_export_with_redaction.yaml`. |
| Voice cannot create privileged actions or hidden edit paths that bypass existing command, trust, or undo models.                                  | §3 rules 4, 5, 6; §4 rules 4, 6, 7; §7 voice-mode invariants 4, 6, 7, 8; fixture `command_mode_privileged_action_confirmation.yaml`.                       |
| Fixtures cover at least: push-to-talk local dictation, remote speech provider with explicit consent, command-mode confirmation for privileged action, transcript export with redaction, fallback when speech pack is unavailable. | Fixture set in `/fixtures/ux/voice_cases/` (see §10).                                                                                                       |

## 9. Denial reasons

The denial-reason sets are reserved on the three companion schemas.
A shell that would otherwise render a non-conforming row MUST deny
with the matching reason rather than silently fall back:

- `speech_provider_entry.schema.json`:
  `speech_provider_unverified_pack_admitted`,
  `speech_provider_handoff_undisclosed`,
  `speech_provider_retention_widened_after_disclosure`,
  `speech_provider_envelope_mismatch`,
  `speech_provider_action_opens_private_mutation_path`,
  `speech_provider_grammar_resolves_to_uncanonical_command`,
  `speech_provider_schema_version_lagging`.
- `voice_mode_state.schema.json`:
  `voice_mode_dictation_and_command_conflated`,
  `voice_mode_continuous_listening_without_opt_in`,
  `voice_mode_indicator_hidden_during_active_capture`,
  `voice_mode_command_preview_skipped_for_privileged_action`,
  `voice_mode_transcript_commit_without_correction_window`,
  `voice_mode_undo_stack_diverged_from_keyboard_lane`,
  `voice_mode_accessibility_announcement_missing`,
  `voice_mode_schema_version_lagging`.
- `speech_privacy_ledger.schema.json`:
  `speech_privacy_audio_left_device_without_disclosure`,
  `speech_privacy_transcript_retained_beyond_declared_class`,
  `speech_privacy_export_unredacted_without_consent`,
  `speech_privacy_correction_skipped_for_privileged_commit`,
  `speech_privacy_consent_class_inferred_not_declared`,
  `speech_privacy_schema_version_lagging`.

## 10. Worked examples

Fixtures under
[`/fixtures/ux/voice_cases/`](../../fixtures/ux/voice_cases/)
cover:

1. **Push-to-talk local dictation** —
   `push_to_talk_local_dictation.yaml`. A
   `speech_provider_entry_record` whose
   `execution_locus_class = local_in_process_speech`,
   `audio_locality_class = local_machine_only_in_process`, paired
   with a `voice_mode_session_record` in `dictation_mode_active`
   under `push_to_talk_held` activation. No handoff is disclosed
   because no audio leaves the device; the editor integration
   bridge declares `same_undo_stack_as_keyboard_lane`.
2. **Remote speech provider with explicit consent** —
   `remote_speech_provider_explicit_consent.yaml`. A
   `speech_provider_entry_record` whose
   `execution_locus_class = byok_remote_vendor_direct_speech`,
   paired with a `speech_provider_handoff_disclosure_record` whose
   `pre_invocation_disclosure_kind = byok_remote_handoff_disclosed`
   and a `speech_capture_event_record` whose
   `audio_locality_class = sent_to_byok_remote_vendor_direct` and
   `consent_class = explicit_user_opt_in_each_session`.
3. **Command-mode confirmation for privileged action** —
   `command_mode_privileged_action_confirmation.yaml`. A
   `voice_mode_session_record` in `command_mode_active` resolving
   a privileged command (capability scope =
   `recoverable_durable_mutation`) through a
   `grammar_intent_resolver_record` and a
   `voice_intent_record` whose
   `voice_intent_resolution_class = resolves_to_canonical_command_id`,
   with `command_preview_class = preview_required_for_privileged_actions`
   and a `transcript_correction_record` proving the correction-
   before-commit window.
4. **Transcript export with redaction** —
   `transcript_export_with_redaction.yaml`. A
   `transcript_export_record` whose
   `export_redaction_class = redacted_partial_with_disclosure` and
   `export_destination_class = support_bundle_metadata_safe_default`,
   paired with the bound capture-event refs and a
   `speech_privacy_ledger_manifest_record` declaring the eight
   const-true invariants.
5. **Fallback when speech pack is unavailable** —
   `speech_pack_unavailable_fallback.yaml`. A
   `local_speech_pack_entry_record` whose
   `pack_verification_state_class = signature_failed`, paired with
   a `speech_provider_entry_record` resolving
   `speech_provider_class = disabled_no_speech_provider`, and a
   `voice_mode_session_record` in
   `voice_mode_blocked_by_envelope` with no activation. The shell
   suppresses voice activation rather than silently falling back
   to a remote route.
6. **Voice-mode invariant manifest** —
   `voice_mode_invariant_manifest.yaml`. A
   `voice_mode_invariant_manifest_record` declaring the ten const-
   true invariants and binding the parity bridge, the indicator,
   the session, and the transition rows.

## 11. Adding a new vocabulary value

Adding a new `speech_provider_class`, `execution_locus_class`,
`transport_class`, `auth_mode_class`, `retention_stance_class`,
`region_posture_class`, `supported_feature_class`, `data_class`,
`pack_origin_class`, `pack_verification_state_class`,
`pre_invocation_disclosure_kind`, `voice_intent_resolution_class`,
`undo_stack_parity_class`, `command_id_parity_class`,
`trust_model_parity_class`, `accessibility_announcement_class`,
`voice_mode_class`, `activation_class`, `mic_indicator_class`,
`command_preview_class`, `transcript_correction_posture`,
`voice_mode_transition_kind`, `audio_locality_class`,
`transcript_locality_class`, `consent_class`,
`correction_origin_class`, `export_redaction_class`,
`export_destination_class`, `evidence_hook_class`, or
`denial_reason` is **additive-minor** and bumps the relevant
`*_schema_version`. Repurposing an existing value is **breaking**
and requires a new decision row on the launch decision register.
A consumer surface that resolves a value it does not recognize
MUST deny with the matching `*_schema_version_lagging` reason
rather than silently map to a default.

## 12. Out of scope at this revision

- Implementing concrete ASR / TTS providers, training local
  models, or shipping a voice / dictation UX in M0. This contract
  freezes the boundary; the actual audio capture pipeline, the
  ASR / TTS adapters, the wake-phrase detector, the inline
  transcript correction surface, and the per-session privacy
  ledger UI land in later milestones and ride this contract.
- Final visuals (mic-indicator chip, transcript correction popover,
  voice command preview pane). The design-system style guide owns
  those.
- Final wake-phrase, hotword, or activation-shortcut bindings.
  The keybinding resolver and the platform-input matrix own those.
- Final policy-bundle definitions that disable continuous
  listening, remote speech routes, transcript retention, or
  unredacted export. The identity / policy-bundle contracts own
  those; this contract names the typed suppression cause.
