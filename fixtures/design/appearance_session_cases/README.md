# Appearance-session fixtures

Worked fixtures for the appearance-session, token-overlay, and live
follow-system state contract frozen in
[`/docs/design/appearance_session_contract.md`](../../../docs/design/appearance_session_contract.md).

Each YAML file conforms to one of the boundary schemas:

- [`/schemas/design/appearance_session.schema.json`](../../../schemas/design/appearance_session.schema.json)
  — `appearance_session_record`, `live_follow_system_policy_record`,
  and `appearance_session_revision_event_record`.
- [`/schemas/design/token_overlay.schema.json`](../../../schemas/design/token_overlay.schema.json)
  — `token_overlay_record`.

The fixtures exist so settings UI, theme switcher, support exporter,
QA capture, docs/help, and migration flows can all read one packet
shape instead of inventing local appearance-state checklists. Each
example resolves every required field.

## Fixtures

- [`steady_state_follow_system_dark_signal.yaml`](./steady_state_follow_system_dark_signal.yaml)
  — `appearance_session_record`. Steady-state session with
  `follow_system` posture and an OS theme signal applied under
  `live_apply_no_review`. No preview, no checkpoint, no overlay.
- [`live_follow_system_policy_default_profile.yaml`](./live_follow_system_policy_default_profile.yaml)
  — `live_follow_system_policy_record`. Per-axis policy showing
  `live_apply_no_review` for theme/accent, revertable for contrast and
  density, confirm-required for text scale and full theme switch, and
  policy-blocked for nothing in the default profile.
- [`live_preview_revertable_checkpoint_session.yaml`](./live_preview_revertable_checkpoint_session.yaml)
  — `appearance_session_record` in `preview_live` with a revertable
  checkpoint and rollback ref. The session cites a token overlay whose
  entries are mostly inherited.
- [`os_contrast_signal_revertable_checkpoint_event.yaml`](./os_contrast_signal_revertable_checkpoint_event.yaml)
  — `appearance_session_revision_event_record`. OS contrast change
  applied under `live_apply_with_revertable_checkpoint`; the event
  records the checkpoint mint.
- [`user_theme_switch_confirm_required_event.yaml`](./user_theme_switch_confirm_required_event.yaml)
  — `appearance_session_revision_event_record`. User-driven
  theme-package switch applied under `confirm_review_required` after a
  user confirm action.
- [`workspace_overlay_overridden_deprecated_unmapped.yaml`](./workspace_overlay_overridden_deprecated_unmapped.yaml)
  — `token_overlay_record`. Workspace overlay with one `overridden`,
  one `deprecated`, one `unmapped`, and one `inherited` entry; the
  fallback chain ends in an `inert_placeholder` for the unmapped slot.
- [`policy_blocked_text_scale_overlay.yaml`](./policy_blocked_text_scale_overlay.yaml)
  — `token_overlay_record`. Policy-managed overlay blocked because the
  text-scale axis is `policy_blocked`; entry validation is
  `blocked_policy`.

## Intended usage

- **Schema conformance:** the YAML shape is the contract of record.
- **Settings / support / QA review:** reviewers walk from a session
  record to its policy and overlay refs without negotiating field
  names.
- **Migration evidence:** support exports cite the same revision event
  shape every surface emits.
- **Conformance gates:** future runners diff the canonical state
  against an implementation or capture without reinterpretation.
