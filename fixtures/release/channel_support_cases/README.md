# Channel-and-support-window seed cases

These fixtures are the seed cases the channel-and-support-window
contract at
[`docs/release/channel_support_window_contract.md`](../../../docs/release/channel_support_window_contract.md)
defines. Each file is a `support_window_badge_record` instance that
projects onto one
[`support_class_rows.yaml`](../../../artifacts/release/support_class_rows.yaml)
row, one
[`channel_matrix.yaml`](../../../artifacts/release/channel_matrix.yaml)
`channel_row`, and the five required surfaces (About panel, update
center, issue / report packet, compatibility report, migration /
import workflow).

Every case:

- names one stable `badge_id`;
- binds one `channel_identity.channel_row_ref` and one
  `support_posture.support_class_row_ref`;
- declares the explicit `support_window` posture with concrete
  start / end / security-only timestamps when the support class is
  positive, and the not-applicable refusal state when the support
  class is one of the four refusal classes;
- carries a `scope_envelope` naming archetype, client class, OS
  family, deployment profile, and local-or-remote mode so the
  support claim is never unbounded;
- lists the `required_surface_set` rendering surfaces (always
  including the five required floor entries).

## Case list

- `stable_supported_general_availability_desktop.yaml` —
  stable / supported / rolling-window posture for the general-
  availability desktop build.
- `lts_certified_enterprise_calendar_window.yaml` —
  LTS / certified / explicit-LTS-window posture backed by a live
  certified-archetype report.
- `preview_experimental_pre_release_unverified.yaml` —
  preview / experimental / narrowed-pre-release-window posture with
  runtime-observation-only evidence.
- `not_certified_in_this_mode_remote_agent_attached.yaml` —
  refusal-class badge demonstrating that the four refusal states
  (not_certified_in_this_mode, not_configured, disabled_by_policy,
  not_supported) are distinct labels with distinct recovery paths.

Every case cites its channel-row, support-class row, scope envelope,
evidence path, and recovery guidance by stable ref so release,
support, docs, and About surfaces can pivot in O(1) from one badge
to the canonical channel and support evidence.
