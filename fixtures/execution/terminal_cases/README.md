# Terminal session, boundary, paste-safety, and transcript / export worked cases

These fixtures are short, reviewable scenarios that anchor the contract
frozen in
[`/docs/execution/terminal_truth_contract.md`](../../../docs/execution/terminal_truth_contract.md)
and validated by:

- [`/schemas/execution/terminal_session.schema.json`](../../../schemas/execution/terminal_session.schema.json)
- [`/schemas/execution/command_boundary_marker.schema.json`](../../../schemas/execution/command_boundary_marker.schema.json)
- [`/schemas/execution/terminal_export_review.schema.json`](../../../schemas/execution/terminal_export_review.schema.json)

Each fixture is one record (a `terminal_session_record`, a
`command_boundary_marker_record`, an `input_request_record`, a
`paste_safety_packet_record`, a `transcript_export_review_record`, a
`restore_no_rerun_attestation_record`, a `clipboard_bridge_review_
record`, or a `linkification_review_record`) rendered as a worked
scenario. The set exists so a reviewer can read the four authority
surfaces — live shell, basic PTY, captured transcript, restored
snapshot — and the four reviewable actions — open, copy, share,
rerun — across one corpus rather than reverse-engineering per-surface
prose.

## Scope rules

- Fixtures validate against their named schema. They carry the
  matching `*_schema_version: 1` const.
- Fixtures MUST NOT encode raw PTY bytes, raw escape-sequence payloads,
  raw command lines, raw env bodies, raw clipboard bytes, raw secret
  values, raw URLs / hyperlink targets, raw absolute paths, or raw
  shell prompts. Only class labels, frozen tokens, opaque ids,
  hashes, and counts are admissible.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- Approval-ticket, target-identity-witness, container-target-identity,
  shared-terminal-control-metadata, command-dispatch-descriptor,
  credential-handle-class, high-risk-preview-class, preview-record,
  trust-state, identity-mode, admin-policy-epoch, link-target-summary,
  and audit-event id refs are opaque.

## Index

### Terminal-session truth records

| Fixture | Host scope | Live state | Capability limits | Key coverage |
|---|---|---|---|---|
| [`live_local_shell_with_boundaries.yaml`](./live_local_shell_with_boundaries.yaml) | `local_host_desktop_pty` | `live_session_active_with_running_pty` | none | live local Zsh with `shell_integration_optional_full_signals`; `auto_rerun_admitted_on_this_record = false` |
| [`remote_degraded_pty_no_shell_integration.yaml`](./remote_degraded_pty_no_shell_integration.yaml) | `ssh_remote_host_pty` | `live_session_active_with_running_pty` | none | remote Bash 3.x with `no_shell_integration_baseline_only`; remote-agent target-identity witness ref required and present; renderer narrows to `not_available_without_shell_integration` chips |
| [`alt_screen_capture_limit.yaml`](./alt_screen_capture_limit.yaml) | `local_host_desktop_pty` | `live_session_active_with_running_pty` | three concurrent: scrollback paused, search disabled / no grid capture, transcript-capture paused | alt-screen paginator with `cbreak_mode_per_keystroke_local_echo_on`; `transcript_capture_paused_alt_screen_or_raw_mode_active` |
| [`restored_transcript_no_rerun.yaml`](./restored_transcript_no_rerun.yaml) | `local_host_desktop_pty` (preserved from prior session) | `transcript_restored_evidence_only_no_rerun` | none | `auto_rerun_admitted_on_this_record = false`; `prior_work_preserved_but_not_rerun`; `cwd_resolution_class = last_known_cwd_token_only` |

### Command-boundary, input-request, and paste-safety packets

| Fixture | Record kind | Decision / result | Key coverage |
|---|---|---|---|
| [`command_boundary_marker_confirmed.yaml`](./command_boundary_marker_confirmed.yaml) | `command_boundary_marker_record` | `boundary_confirmed_by_shell_integration_signal_pair` / `exited_zero_success` | both `command_begin` and `command_end` signal refs; `transcript_lines_within_scrollback_bound` |
| [`input_request_secret_mode_no_summary.yaml`](./input_request_secret_mode_no_summary.yaml) | `input_request_record` | `result_user_submitted_secret_no_summary` | secret-mode prompt with credential-handle class ref; `result_summary_class = no_summary_secret_or_redacted`; typed `expires_at` deadline |
| [`paste_safety_held_pending_secret_class.yaml`](./paste_safety_held_pending_secret_class.yaml) | `paste_safety_packet_record` | `paste_held_pending_secret_class_review` | `payload_intersects_credential_handle_class`; held packet — `run_intent_command_dispatch_descriptor_ref = null`; credential-handle class ref required |

### Transcript / export, restore-no-rerun, clipboard-bridge, linkification

| Fixture | Record kind | Decision | Key coverage |
|---|---|---|---|
| [`transcript_export_redaction_reviewed.yaml`](./transcript_export_redaction_reviewed.yaml) | `transcript_export_review_record` | `export_admitted_after_preview_within_redaction_class` | `share_action_via_typed_export_route`; `rendered_text_no_ansi_escape_redaction_aware`; `scrollback_within_declared_bound`; preview record ref required |
| [`restore_no_rerun_attestation_fresh_session.yaml`](./restore_no_rerun_attestation_fresh_session.yaml) | `restore_no_rerun_attestation_record` | `fresh_session_minted_with_explicit_user_initiation` | `auto_rerun_forbidden = true` (const); fresh terminal-session record ref AND fresh command-dispatch descriptor ref required; only path back to write authority |
| [`clipboard_bridge_denied_on_restored_transcript.yaml`](./clipboard_bridge_denied_on_restored_transcript.yaml) | `clipboard_bridge_review_record` | `bridge_denied_on_restored_transcript_inspect_only` | restored transcript never silently regains clipboard-bridge authority; `target_identity_witness_ref` required for remote-clipboard-bridge direction |
| [`linkification_rerun_intent_forbidden.yaml`](./linkification_rerun_intent_forbidden.yaml) | `linkification_review_record` | `rerun_intent_forbidden_link_is_not_a_command` | a transcript hyperlink is never a command; rerun authority lives only on a command-dispatch descriptor; unverified OSC-8 target |

## Coverage contract

The fixture set MUST keep:

- at least one `terminal_session_record` covering each of the four
  authority surfaces — live shell with full shell-integration signals,
  remote degraded baseline-only PTY with a target-identity witness,
  alt-screen / capability-limit-active live session, and a restored
  transcript that forbids auto-rerun;
- at least one `command_boundary_marker_record` showing
  `boundary_confirmed_by_shell_integration_signal_pair` with both
  command-begin and command-end signal refs and a `transcript_line_
  range_class` honestly within scrollback;
- at least one `input_request_record` in secret mode, with
  `result_summary_class = no_summary_secret_or_redacted`, a
  credential-handle class ref, and a typed `expires_at` deadline;
- at least one `paste_safety_packet_record` whose payload intersects a
  credential-handle class, resolves to
  `paste_held_pending_secret_class_review`, and pins
  `run_intent_command_dispatch_descriptor_ref = null`;
- at least one `transcript_export_review_record` admitted after
  preview within the declared redaction class, with
  `transcript_action_class = share_action_via_typed_export_route` and
  the chosen `export_representation_class` projecting redaction-aware
  rendered text rather than raw ANSI;
- at least one `restore_no_rerun_attestation_record` covering the
  fresh-session path that mints a new terminal-session record AND a
  new command-dispatch descriptor;
- at least one `clipboard_bridge_review_record` denying the bridge on
  a restored transcript with the typed
  `bridge_denied_on_restored_transcript_inspect_only` decision; and
- at least one `linkification_review_record` denying rerun-intent on a
  transcript link with the typed
  `rerun_intent_forbidden_link_is_not_a_command` decision.

Removing a layer of coverage is a breaking change.

## Pre-implementation note

At this milestone there is still no PTY broker, no shell-integration
injector, no terminal renderer, no clipboard bridge, and no transcript-
export packager wired up. These fixtures remain pre-implementation
governance artifacts.
