# M5 device-permission & capture review

Set: `m5_device_permission_set:default`

## Device-permission rows

| Record | Device | State | Actor | Processing | Retention | Capture active? |
| --- | --- | --- | --- | --- | --- | --- |
| `device_permission_row:microphone` | Microphone | Granted — idle | You | `local_on_device` | `ephemeral_audio_local_only_no_transcript_retained` | false |
| `device_permission_row:camera` | Camera | Not yet requested | Operating system | `processing_unavailable` | `no_audio_retained_no_transcript_retained` | false |
| `device_permission_row:screen_capture` | Screen capture | Granted — in use | You | `local_on_device` | `no_audio_retained_no_transcript_retained` | true |
| `device_permission_row:system_audio_capture` | System audio | Granted — idle | Connected provider | `hosted_remote_disclosed` | `transcript_retained_provider_per_contract` | false |
| `device_permission_row:clipboard` | Clipboard | Blocked by policy | Administrator policy | `processing_unavailable` | `no_audio_retained_no_transcript_retained` | false |

## Mic-state pills

| Pill | State | Processing | Correction | Scope | Preview required? |
| --- | --- | --- | --- | --- | --- |
| `mic_state_pill:idle` | Idle | `local_on_device` | `correction_optional_before_commit` | `inert_metadata_only` | false |
| `mic_state_pill:listening` | Listening | `local_on_device` | `correction_optional_before_commit` | `reversible_local_mutation` | false |
| `mic_state_pill:muted` | Muted | `local_on_device` | `correction_optional_before_commit` | `inert_metadata_only` | false |
| `mic_state_pill:processing` | Processing | `hosted_remote_disclosed` | `correction_required_before_commit` | `reversible_local_read` | false |
| `mic_state_pill:needs_confirmation` | Needs confirmation | `local_on_device` | `correction_required_before_commit` | `recoverable_durable_mutation` | true |
| `mic_state_pill:unavailable` | Unavailable | `processing_unavailable` | `correction_unavailable_capture_only` | `inert_metadata_only` | false |
| `mic_state_pill:policy_blocked` | Policy blocked | `processing_unavailable` | `correction_blocked_by_envelope` | `inert_metadata_only` | false |

## Capture/export reviews

| Review | Redaction | Data exit | Delete? | Export? |
| --- | --- | --- | --- | --- |
| `capture_export_review:voice_session` | Raw never exported | `no_payload_leaves_product` | true | false |
| `capture_export_review:screen_and_clipboard` | Redacted before export | `redacted_support_packet` | true | true |
| `capture_export_review:device_inventory` | Metadata refs only | `no_payload_leaves_product` | false | true |

Capture is never always-on by default, local processing is never claimed when a provider is in the path, and high-impact spoken commands ride the same preview/confirmation gate with transcript correction required before commit.
