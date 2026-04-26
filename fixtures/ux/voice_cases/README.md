# Voice, dictation, and speech-privacy worked fixtures

Worked-example fixtures for the voice, dictation, and speech-privacy
contract frozen in
[`/docs/ux/voice_and_dictation_contract.md`](../../../docs/ux/voice_and_dictation_contract.md)
and validated by the three companion schemas:

- [`/schemas/ux/speech_provider_entry.schema.json`](../../../schemas/ux/speech_provider_entry.schema.json)
- [`/schemas/ux/voice_mode_state.schema.json`](../../../schemas/ux/voice_mode_state.schema.json)
- [`/schemas/ux/speech_privacy_ledger.schema.json`](../../../schemas/ux/speech_privacy_ledger.schema.json)

Each YAML file is a scenario manifest: a `__fixture__` envelope plus
a `records:` array whose items each validate as one of the record
kinds declared by the relevant schema's `oneOf`. Records carry a
per-record `$schema` pin so a heterogeneous fixture can route
each record to the correct schema. Every fixture carries only
opaque ids, typed vocabulary, short privacy-safe labels,
monotonic placeholder timestamps, and opaque policy-bundle refs —
no raw URLs, no raw absolute paths, no raw audio bodies, no raw
transcript text, no raw API keys, no raw OAuth tokens, and no raw
cryptographic material.

## Index

| Fixture                                              | Schema(s)                                                                                                       | Exercises                                                                                                                  |
|------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------|
| `push_to_talk_local_dictation.yaml`                  | `speech_provider_entry`, `voice_mode_state`, `speech_privacy_ledger`                                            | Local in-process dictation, push-to-talk activation, no handoff disclosed because no audio leaves the device.             |
| `remote_speech_provider_explicit_consent.yaml`       | `speech_provider_entry`, `voice_mode_state`, `speech_privacy_ledger`                                            | BYOK remote speech provider with handoff disclosure and `explicit_user_opt_in_each_session` consent.                      |
| `command_mode_privileged_action_confirmation.yaml`   | `speech_provider_entry`, `voice_mode_state`, `speech_privacy_ledger`                                            | Command-mode privileged action with `preview_required_for_privileged_actions` and a transcript correction before commit. |
| `transcript_export_with_redaction.yaml`              | `speech_privacy_ledger`                                                                                         | Transcript export with `redacted_partial_with_disclosure` to a metadata-safe support bundle, plus the eight invariants.   |
| `speech_pack_unavailable_fallback.yaml`              | `speech_provider_entry`, `voice_mode_state`                                                                     | `signature_failed` local pack forces voice suppression; provider resolves to `disabled_no_speech_provider`.                |
| `voice_mode_invariant_manifest.yaml`                 | `voice_mode_state`                                                                                              | Voice-mode invariant manifest declaring all ten const-true invariants over the session set.                                |

Every fixture cites the contract sections it exercises and binds
each axis by reference rather than redefinition.
