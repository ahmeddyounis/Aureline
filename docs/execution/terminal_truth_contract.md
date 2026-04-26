# Terminal session, command-boundary, paste-safety, and transcript / export truth contract

This document freezes how Aureline treats the terminal as a first-class
execution surface — with explicit host boundary, shell-integration
confidence, replay limits, and export / redaction truth — before the
PTY, shell-integration injection, terminal renderer, clipboard bridge,
or transcript-export paths begin to ship implementation.

Every terminal-touching surface (the terminal panel itself, the
status / inspector chips, the run / debug surface that seeds a session,
the support / export bundle that captures one, the AI explanation that
quotes one, the collaboration shared-control surface that admits a
guest into one, and any restored or imported transcript surface) reads
the same record family. There is one terminal-session truth record,
one boundary / input / paste packet family, one transcript-export
review record, one restore-no-rerun attestation, one clipboard-bridge
review record, and one linkification review record — not a separate
hidden authority path per surface.

Machine-readable companions:

- [`/schemas/execution/terminal_session.schema.json`](../../schemas/execution/terminal_session.schema.json)
  — `terminal_session_record`, the truth record every consumer reads
  when it needs to answer "is this a live shell, a basic PTY, a
  captured transcript, or a restored snapshot, and what authority does
  it carry?". Carries the host-scope badge, the local-vs-remote
  boundary cue posture, the shell-integration quality class, the cwd
  resolution class, the shared-control state, the live-or-transcript
  state, the alt-screen / raw-mode state, the active capability-limit
  set, the transcript-capture posture, the scrollback bound, and the
  refs to the execution-context root, the workspace-trust state, the
  identity-mode envelope, the admin-policy epoch, the command-dispatch
  descriptor that opened the session, the remote-agent or container
  target identity, and the collaboration shared-terminal-control row.
- [`/schemas/execution/command_boundary_marker.schema.json`](../../schemas/execution/command_boundary_marker.schema.json)
  — three live-session packet families that ride alongside the
  terminal-session record:
  `command_boundary_marker_record` (where one command begins and ends,
  with typed boundary-confidence, exit-status confidence, and the
  honest scrollback-line-range posture);
  `input_request_record` (typed prompts for paste, secret-mode, or
  interactive-line entry under timeout / expiry); and
  `paste_safety_packet_record` (typed review of host-initiated paste,
  remote clipboard bridge, and OSC-52 round-trip paste before bytes
  commit, with a typed run-intent attribution that routes through the
  command-dispatch descriptor rather than letting paste mint authority).
- [`/schemas/execution/terminal_export_review.schema.json`](../../schemas/execution/terminal_export_review.schema.json)
  — four review packets that keep open / copy / share / rerun distinct
  actions on a transcript: the `transcript_export_review_record` (raw-
  vs-rendered representation, scrollback-bound class, redaction class,
  approval-ticket gate for broadened or unbounded captures); the
  `restore_no_rerun_attestation_record` (a restored pane never silently
  regains write authority — a fresh session with its own
  command-dispatch descriptor is the only path that may); the
  `clipboard_bridge_review_record` (host ↔ remote / OSC-52 round-trip
  clipboard with target-identity witness); and the
  `linkification_review_record` (an OSC-8 hyperlink, a path token, or
  a URL token in the transcript may be opened, copied, or shared but
  never executed without a typed command-dispatch descriptor).
- [`/fixtures/execution/terminal_cases/`](../../fixtures/execution/terminal_cases/)
  — worked YAML fixtures covering the required scenarios.

This contract composes with and does not replace:

- [`/schemas/terminal/session_restore_metadata.schema.json`](../../schemas/terminal/session_restore_metadata.schema.json)
  — the terminal-protocol seed (ADR-0021). The PTY-owner-class,
  protocol-conformance-level, shell-integration-signal-kind,
  shell-family, terminal-session lifecycle-state, terminal-restore-
  level, terminal-restore-decision, clipboard-write-class, clipboard-
  suppression-class, and terminal-denial-reason vocabularies are
  frozen there; this contract narrows them into review-time truth and
  never re-mints them.
- [`/docs/runtime/execution_context_vocabulary.md`](../runtime/execution_context_vocabulary.md)
  and
  [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — execution-context root, target identity, workspace-trust state,
  identity-mode envelope, sandbox posture, and policy epoch (ADR-0009,
  ADR-0018, ADR-0001). Every terminal-session record cites this root.
- [`/docs/execution/context_inspector_packet.md`](./context_inspector_packet.md)
  — execution-context snapshot, diff, and inspector view. The
  terminal-session-seed snapshot is one of the three surfaces that
  packet binds together.
- [`/docs/security/secret_broker_contract.md`](../security/secret_broker_contract.md)
  — credential-handle classes (ADR-0007). Secret-mode prompts,
  paste-then-run-with-secret review, and transcript-capture pauses
  cite credential-handle class refs from this boundary.
- [`/docs/commands/command_dispatch_contract.md`](../commands/command_dispatch_contract.md)
  — command-dispatch descriptor (ADR-0016). The descriptor mints run
  intent; a paste, a hyperlink, a transcript line, or rerun metadata
  never does.
- [`/docs/security/shell_interaction_safety_contract.md`](../security/shell_interaction_safety_contract.md)
  — high-risk preview classes (multiline_terminal_paste, paste_then_run,
  remote_clipboard_bridge, secret_access, bidi_or_invisible_formatting_
  reveal, confusable_identifier_reveal, rich_active_content_render).
  Paste-safety, clipboard-bridge, transcript-export, and linkification
  packets cite these refs rather than minting parallel preview classes.
- [`/docs/collaboration/session_authority_contract.md`](../collaboration/session_authority_contract.md)
  — shared-terminal-control metadata. Shared-control state on the
  terminal-session record re-projects that row; this contract never
  infers shared-control from presenter / follow state.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md)
  — quarantine, restart, and freshness floors. Capability-limit
  classes and live-or-transcript transitions read those rules rather
  than inventing private "retrying" folklore states.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  — shared `Limited`, `Stale`, `Blocked`, and downgrade language; the
  capability-limit and degraded-posture chips here reuse it.
- `.t2/docs/Aureline_PRD.md`,
  `.t2/docs/Aureline_Technical_Design_Document.md`, and
  `.t2/docs/Aureline_UI_UX_Spec_Document.md`. If those documents
  disagree with this contract, those upstream documents win and this
  contract plus the companion schemas update in the same change.

## Why freeze this now

The terminal is the easiest place in an IDE for product truth to slip:

- a captured or restored transcript can be presented exactly like a
  live shell, and an unsuspecting user (or an AI assist) can ask it to
  rerun a previous command — silently, on stale state;
- a remote, container, devcontainer, or managed-workspace session can
  render against the same chrome as a local session, and the user can
  type or paste destructive commands at the wrong host without ever
  seeing a local-vs-remote cue;
- a session whose shell has not advertised any integration signals can
  display speculative command-boundaries, prompt zones, and exit-status
  badges that look like ground truth — and propagate that speculation
  into rerun affordances and AI explanations;
- a paste-then-run shortcut can attach `Enter` to a multi-line clipboard
  payload and commit a script that no preview ever rendered;
- an OSC-52 round-trip can hand a remote shell silent read access to
  the host clipboard, or write back into the host clipboard from the
  remote side, without the user ever consenting to clipboard bridging;
- an alt-screen full-screen program (a paginator, an editor, a TUI)
  can capture mouse and keyboard while host chrome accelerators look
  alive but no longer route to the host;
- a transcript export can leak ANSI escape payloads, hyperlink targets,
  prompt-zone hints, environment variables, and credentials into a
  support bundle that was meant to be redaction-aware; and
- an OSC-8 hyperlink, a path token, or a URL token in the transcript
  can be treated as a command — opening a network resource, mutating a
  file, or invoking an arbitrary handler — without ever passing through
  the command-dispatch boundary.

This contract makes those differences explicit before any apply path
ships. A live shell, a basic PTY, a captured transcript, and a restored
snapshot are four typed states with four different authority surfaces.
High-risk paste, remote clipboard bridging, and transcript export are
reviewable rather than implicit. Open, copy, share, and rerun remain
distinct actions — restored transcripts never silently regain write
authority, and a transcript link is never a command.

## Scope

Frozen at this revision:

- the terminal-session truth record's host-scope badge (nine-value
  vocabulary covering local desktop, SSH remote, remote / managed
  workspace agent, container / devcontainer, provider-side remote
  agent, compatibility-bridge remote, and host-scope-unknown), the
  local-vs-remote boundary-cue visibility class (six-value), the
  shell-integration quality class (six-value, projecting the ADR-0021
  protocol-conformance-level plus an observed-signal confidence
  dimension), the cwd resolution class (six-value, distinguishing
  live-cwd / last-known-cwd / unknown / unresolvable / redacted /
  unknown-pending-review), the shared-control state class (six-value,
  re-projected from the collaboration session-authority contract's
  shared-terminal-control row), the live-or-transcript state class
  (nine-value, separating live-active / live-idle / live-suspended /
  live-lost-transport from transcript-captured / transcript-restored /
  transcript-restored-with-hints / transcript-imported), the
  session-user-impact label class (eight-value, reusing the entry-
  restore truth-audit user-impact set), the alt-screen state class,
  the raw-mode state class, the capability-limit class
  (ten-value, covering alt-screen scrollback / search / grid limits,
  raw-mode line-buffer / echo limits, bracketed-paste-disabled paste
  review, mouse-capture host-chrome pin, IME composition, output buffer
  cap, transcript-capture pause, session quarantine, and
  unknown-pending-review), and the transcript-capture posture class
  (seven-value, covering disabled / metadata-only / support-bundle /
  broadened-with-approval-ticket / paused-by-alt-or-raw /
  paused-by-secret-class / unknown);
- the command-boundary marker's boundary-confidence class (six-value),
  boundary host-scope class (five-value), command-line class
  (six-value, coarse only — exact bytes never project), exit-status
  class (six-value, including exit_status_unobserved_no_shell_integration
  as the honest answer when the shell did not emit command_end),
  optional duration in monotonic milliseconds, and the
  transcript-line-range class (five-value, honest about scrollback-
  bound truncation);
- the input-request record's input-request kind class (seven-value,
  covering visible-echo line entry, secret / password modes, yes-no
  confirmation, paste intent, rerun intent, and unknown-pending-review),
  the secret-mode class (four-value), the typed timeout / expiry
  monotonic deadline, the typed result class (eight-value), and the
  result-summary class that resolves to no_summary_secret_or_redacted
  on every secret-mode prompt;
- the paste-safety packet's paste-origin class (six-value, covering
  host-initiated paste from clipboard history / drag-drop / command
  palette, remote-originated paste via clipboard bridge, shell-
  originated OSC-52 round-trip paste, and unknown-pending-review),
  paste-payload line / payload class (eight-value, with executable-
  intent and credential-handle and high-risk-preview intersection
  rows), and paste-safety decision class (twelve-value, covering
  admit-after-preview-no-run / admit-after-preview-explicit-run /
  held-pending-high-risk-preview / held-pending-secret-class / held-
  pending-remote-clipboard-bridge / held-pending-responsive-fallback /
  denied-by-trust / denied-by-policy / denied-by-secret-class / denied-
  by-quarantine / denied-by-alt-or-raw / unknown);
- the transcript-export-review record's transcript-action class
  (five-value, keeping open / copy / share / rerun / unknown distinct),
  export-representation class (six-value, raw-bytes-with-ansi /
  rendered-text-no-ansi / rendered-with-redaction-markers / rendered-
  sanitized-no-high-risk / metadata-and-hashes-only / unknown),
  scrollback-bound class (four-value, with unbounded gated by an
  approval-ticket ref), transcript-export-review decision class
  (eleven-value), and redaction class at export;
- the restore-no-rerun attestation's restore-authority state class
  (four-value, covering restored-inspect-only / restored-with-hints-
  inspect-only / fresh-session-with-explicit-user-initiation /
  unknown), the user-impact label class, and the invariant booleans
  `auto_rerun_forbidden` (always true), `input_admission_forbidden_
  until_fresh_session`, and `clipboard_bridge_admission_forbidden_
  until_fresh_session` — the only path back to write authority is a
  fresh terminal-session record bound to a fresh command-dispatch
  descriptor;
- the clipboard-bridge-review record's clipboard-bridge direction
  class (seven-value, covering host ↔ remote bridge in both
  directions, host ↔ local-shell OSC-52 round-trip in both directions,
  host ↔ remote-clipboard-bridge in both directions, and unknown), the
  clipboard-bridge decision class (nine-value, with default-disabled
  and restored-transcript-inspect-only as typed denials);
- the linkification-review record's link-target class (seven-value,
  covering safe-scheme URL / unsafe-scheme / canonical workspace path /
  outside-workspace path / OSC-8 with safe target / OSC-8 with
  unverified target / unknown), and linkification decision class
  (ten-value, with `rerun_intent_forbidden_link_is_not_a_command` as
  the typed refusal when a transcript link is treated as a command);
- the closed denial-reason vocabularies per record family (eighteen
  terminal-session denial reasons, eighteen command-boundary / input-
  request / paste-safety denial reasons, twenty-nine transcript-
  export / restore-no-rerun / clipboard-bridge / linkification denial
  reasons including the contract-level `open_copy_share_rerun_actions_
  must_remain_distinct` invariant);
- the matched audit-event vocabularies per record family (terminal-
  session published / revoked / state-transitioned / capability-limit
  entered / capability-limit exited / transcript-capture posture
  narrowed / broadened, plus the silent-paint / silent-auto-rerun /
  silent-boundary-cue-collapse / silent-cwd-widening / silent-shell-
  integration-widening / audit-denial denials; command-boundary
  published / revoked / confidence-narrowed; input-request opened /
  closed-with-typed-result / timed-out / silent-secret-capture
  forbidden; paste-safety admitted / held / denied / silent-admit
  forbidden / silent-run-intent forbidden; transcript-export admitted /
  held / denied / silent-admit forbidden; restore-no-rerun published /
  fresh-session-minted / silent-input-admission forbidden / silent-
  bridge-admission forbidden; clipboard-bridge admitted / held /
  denied / silent-admit-on-restored-transcript forbidden; linkification
  open admitted / copy admitted / share admitted / open held / denied /
  rerun-intent-on-link forbidden; the contract-level open-copy-share-
  rerun-collapsed forbidden denial; and the family audit-denial-emitted
  rows);
- the additionalProperties = false posture on every record; raw PTY
  bytes, raw escape-sequence payloads, raw command lines, raw
  environment bodies, raw clipboard bytes, raw secret values, raw URLs
  / hyperlink targets, raw absolute paths, and raw shell prompts MUST
  NOT cross any of these boundaries — records carry refs and counts
  only.

Out of scope at this revision:

- implementing PTY support (forks, ConPTY, ssh-channel multiplexing,
  remote-agent PTY brokers);
- implementing the shell-integration injection mechanism, the OSC
  allowlist allowlister, the bracketed-paste handler, the alt-screen
  / raw-mode detector, the linkifier, or the renderer;
- implementing transcript capture, scrollback storage, or the support /
  export packager;
- the final UI affordances for chips, badges, capability-limit popovers,
  paste-review modals, transcript-export review surfaces, restore-no-
  rerun attestation surfaces, clipboard-bridge review surfaces, or
  linkification review surfaces;
- ranking or prioritising paste-safety review items;
- the conflict-resolution policy when an external change races a live
  session (named here through the live-or-transcript state class but
  the reconciler is its own decision row).

## 1. Terminal-session truth record

The terminal-session record is the truth surface every other terminal-
touching record cites. Each session emits exactly one record kind per
moment of capture; subsequent state transitions emit fresh records and
never edit prior ones in place. Adding a new state, a new badge, a new
capability-limit class, or a new posture is additive-minor and bumps
`terminal_session_truth_schema_version`.

### 1.1 Host-scope badge and the local-vs-remote boundary cue

`host_scope_badge_class` is the load-bearing field. It decides which
target-identity ref is required, which boundary cue MUST be visible,
which clipboard-bridge directions are admissible, and which capability
limits a renderer MUST surface.

The vocabulary is more specific than the ADR-0021 `pty_owner_class`:
`local_host_desktop_pty` and `container_local_pty` both project
`pty_owner_class = host_desktop`, but the badge distinguishes them so
the renderer can paint the matching cue. Every non-local badge MUST
resolve `boundary_cue_visibility_class` to a visible row; the typed
`boundary_cue_must_be_visible_but_responsive_fallback_collapsed` value
is the only admissible degraded posture and forces the cue into the
responsive-fallback header rather than silently dropping it.

`host_scope_badge_class = host_scope_unknown_requires_review` fails
closed. A session in this state forbids every mutating live action and
MUST resolve to a typed denial under
`terminal_session_record_host_scope_unknown_requires_review` rather
than minting a record without a target identity.

### 1.2 Shell-integration quality

`shell_integration_quality_class` is a confidence label, not an
authority claim. The five non-unknown values pair the ADR-0021
`protocol_conformance_level` with an observed-signal confidence
dimension (`shell_integration_signal_count_observed`):

- `no_shell_integration_baseline_only` — baseline PTY, no integration
  signals, no signal evidence required;
- `osc_allowlist_only_no_signals` — the OSC allowlist is admitted but
  the shell has not emitted any integration signal yet;
- `shell_integration_optional_partial_signals` — at least one signal
  observed but the full prompt-begin / prompt-end / command-begin /
  command-end set is incomplete;
- `shell_integration_optional_full_signals` — the full set has been
  observed at least once;
- `shell_integration_unknown_pending_first_signal` — the session has
  advertised optional integration but no signal has arrived yet.

Every quality class outside `shell_integration_optional_full_signals`
MUST degrade command-boundary navigation, prompt-zone focus return,
and exit-status badges to a typed `not_available_without_shell_
integration` chip rather than silently disappearing.

### 1.3 Cwd resolution

`cwd_resolution_class` answers how confident the session is in its
current working directory. `live_cwd_token_observed` is admissible
only when at least one `cwd_change` shell-integration signal has
arrived in the session's lifetime; `last_known_cwd_token_only` is the
honest answer for an idle / suspended / lost-transport session that
has a prior token; `cwd_unknown_no_signals_yet` is the honest answer
for a baseline session whose shell has not advertised cwd. The
session MUST NOT paint a stale cwd as live; the `cwd_token_ref`
field is required for the first two values and MUST be null for the
others.

### 1.4 Live-or-transcript state and the rerun gate

`live_or_transcript_state_class` separates the four authority surfaces
the contract is built around:

- `live_session_active_with_running_pty`,
  `live_session_idle_with_running_pty`,
  `live_session_suspended_pty_may_run`, and
  `live_session_lost_transport_reconnect_pending` are the live rows.
  Only the first two may carry `session_user_impact_label_class =
  live_session_continued`; the suspended / lost-transport rows MUST
  resolve user-impact to a degraded label and MUST set
  `auto_rerun_admitted_on_this_record = false`;
- `transcript_captured_inspect_only_no_pty`,
  `transcript_restored_evidence_only_no_rerun`,
  `transcript_restored_with_hints_evidence_only_no_rerun`, and
  `transcript_imported_from_support_bundle_inspect_only` are the
  transcript rows. They forbid every mutating live-session action,
  forbid `live_session_continued`, and pin
  `auto_rerun_admitted_on_this_record = false`. The only way back to
  write authority is a fresh terminal-session record minted under a
  fresh command-dispatch descriptor (the
  `restore_no_rerun_attestation_record` is the typed bridge);
- `live_or_transcript_state_unknown_requires_review` fails closed
  and forbids auto-rerun.

### 1.5 Shared-control state

`shared_control_state_class` re-projects the collaboration session-
authority contract's shared-terminal-control row. Solo sessions
resolve to `shared_control_not_offered_solo_session`; shared sessions
inherit the inbound or outbound view-only or temporary-typing-grant
posture. Renderers MUST NOT infer shared control from presenter /
follow state under the follow-and-presenter contract; control is an
explicit grant. Every non-solo / non-unknown shared-control state
MUST cite the matching `shared_terminal_control_metadata_ref`.

### 1.6 Capability limits

`active_capability_limits` is a set of typed reasons the session may
report at any moment. Multiple limits may be active concurrently
(for example, alt-screen plus IME composition plus a transcript-
capture pause). Each entry narrows a specific affordance — scrollback
navigation, find-in-scrollback, paste, host chrome accelerators, IME,
output paging, transcript capture, or mutating actions — to the
matching typed chip. Renderers MUST NOT silently drop the affordance.

The `capability_limit_unknown_requires_review` value fails closed.

### 1.7 Transcript-capture posture and scrollback bounds

`transcript_capture_posture_class` decides whether a transcript exists
at all and under what redaction. Capture pauses (`paused_alt_screen_
or_raw_mode_active`, `paused_secret_class_intersected`) are typed
chips, not silent gaps. The `broadened` value MUST cite an approval-
ticket ref; the secret-class pause MUST cite at least one credential-
handle class ref. `scrollback_bound_lines_max` and `scrollback_bytes_
estimate_max` are honest bounds — null when capture is disabled,
otherwise the declared cap. The exact bytes never project.

## 2. Command-boundary marker, input-request, and paste-safety

Three live-session packets ride alongside the terminal-session record.
They share the `command_boundary_marker_schema_version` const and a
common denial-reason / audit-event vocabulary so a reviewer can read
all three with the same eyes.

### 2.1 Command-boundary markers

`boundary_confidence_class` is the load-bearing field. The five non-
unknown values are typed degraded postures relative to a confirmed
shell-integration signal pair:

- `boundary_confirmed_by_shell_integration_signal_pair` — both
  command_begin and command_end signals were observed; their refs are
  required;
- `boundary_inferred_from_prompt_zone_only` — the host inferred
  boundaries from prompt_begin / prompt_end without paired command
  signals;
- `boundary_inferred_from_carriage_return_only_low_confidence` — the
  host inferred boundaries from CR / LF only, no prompt signals;
- `boundary_inferred_from_command_dispatch_only_no_shell_signal` — the
  command-dispatch descriptor opened the boundary, the shell emitted
  no signal; the descriptor ref is required;
- `boundary_unconfirmed_no_shell_integration` — no signal evidence at
  all; the renderer MUST narrow command-boundary navigation, exit-
  status badges, and rerun affordances to the matching chip rather
  than implying confirmed boundaries.

`exit_status_class = exit_status_unobserved_no_shell_integration` is
the honest answer when the shell did not emit `command_end`; in that
state, `exit_status_value` and `duration_monotonic_ms` MUST both be
null. Claiming an exit status without the matching signal is a typed
denial under `command_boundary_marker_must_not_claim_exit_status_
observed_without_shell_signal`.

`transcript_line_range_class` reports honestly how the marked region
sits relative to the parent session's scrollback bound: within the
bound, partially truncated, fully truncated pending review, paused
under a redaction class, or unknown. The exact line numbers never
project.

### 2.2 Input requests

`input_request_record` is the typed prompt the host opens against the
user when a running shell or command-dispatch action needs a paste
intent, a rerun intent, a yes / no confirmation, an interactive line,
or a secret-mode entry. Every input request carries a monotonic
`opened_at` and a monotonic `expires_at`; once the deadline passes the
host MUST close the prompt with `result_timed_out_no_value_provided`,
and reopening requires a new `input_request_record` (no implicit
authority extension).

Secret-mode prompts (`interactive_secret_mode_no_echo_request`,
`interactive_password_mode_managed_echo_mask`) MUST suppress local
echo, MUST forbid transcript capture of the entered bytes, MUST cite
at least one `credential_handle_class_refs` entry, and MUST resolve
`result_summary_class` to `no_summary_secret_or_redacted`. The
typed `result_user_submitted_secret_no_summary` value forbids
summarising the entered bytes anywhere; only the input-request id and
the typed result class project.

### 2.3 Paste safety

`paste_safety_packet_record` is the typed review of host-initiated
paste, remote clipboard bridging, and OSC-52 round-trip paste before
bytes commit. The contract's load-bearing rule is that a paste never
mints run authority by itself: the only admit decision that may run
the pasted bytes is `paste_admitted_after_preview_run_intent_explicit`,
and that decision MUST cite a `run_intent_command_dispatch_descriptor_
ref`. Every other admit decision pins
`paste_admitted_after_preview_no_run_until_user_confirms`; held and
denied decisions MUST null the run-intent ref entirely.

Remote-originated paste (`remote_originated_paste_via_clipboard_
bridge`) and shell-originated OSC-52 round-trip paste MUST cite a
`target_identity_witness_ref`. Payloads that intersect the credential-
handle class set MUST cite at least one credential-handle class ref
and MUST resolve to `paste_held_pending_secret_class_review` or
`paste_denied_by_secret_class_intersection` — they cannot admit.
Payloads that intersect a high-risk preview class MUST cite at least
one preview-class ref. Pastes attempted under workspace-trust
restricted, admin-policy deny, session quarantine, or alt-screen /
raw-mode active resolve to the matching typed denial rather than a
silent drop.

## 3. Transcript / export review, restore-no-rerun, clipboard bridge, linkification

The four review records keep open / copy / share / rerun distinct
actions on a transcript. The `open_copy_share_rerun_actions_must_
remain_distinct` denial reason is the contract-level invariant that
fails closed when any record attempts to collapse two of the four
actions into one decision.

### 3.1 Transcript export review

`transcript_export_review_record` is the typed share route. The
record's `transcript_action_class` MUST be
`share_action_via_typed_export_route` — open / copy / rerun route
through the matching action records.

`export_representation_class` makes the raw-vs-rendered choice
explicit:

- `raw_bytes_with_ansi_escape_preserved_redaction_aware` preserves
  ANSI / VT-100 escape payloads with secret-class redaction applied;
- `rendered_text_no_ansi_escape_redaction_aware` projects the visible
  text only;
- `rendered_text_with_redaction_markers_only` projects only redaction
  markers (no transcript text body);
- `rendered_text_sanitized_no_high_risk_classes` strips high-risk
  preview classes (BiDi, invisible formatting, confusables, rich
  active content);
- `structured_packet_metadata_and_hashes_only` carries no transcript
  bytes — only opaque ids, counts, and hashes.

`scrollback_bound_class = scrollback_unbounded_capture_pending_admin_
signed_opt_in_only` is reserved and MUST cite an approval-ticket ref.
`transcript_export_review_decision_class = export_admitted_after_
preview_with_broadened_capture_approval_ticket` MUST also cite an
approval-ticket ref. Admitted exports MUST cite a preview record ref.
Held / denied decisions MUST cite a typed denial-reason class. Secret-
class denials MUST cite at least one credential-handle class ref.

### 3.2 Restore-no-rerun attestation

`restore_no_rerun_attestation_record` is the typed bridge from a
captured / restored / imported transcript back to a live shell. It
carries three boolean invariants:

- `auto_rerun_forbidden` — MUST always be `true`. A record that sets
  this `false` is non-conforming regardless of any other field;
- `input_admission_forbidden_until_fresh_session` — MUST be `true` on
  every inspect-only attestation row;
- `clipboard_bridge_admission_forbidden_until_fresh_session` — MUST be
  `true` on every inspect-only attestation row.

The only path that may relax the latter two is `restore_authority_
state_class = fresh_session_minted_with_explicit_user_initiation`, and
even then the record MUST cite both a `fresh_terminal_session_record_
ref` (the freshly minted live session) and a `fresh_command_dispatch_
descriptor_ref` (the descriptor that opened it). The freshly minted
session has its own `session_record_id`; it never inherits the
restored pane's authority.

### 3.3 Clipboard-bridge review

`clipboard_bridge_review_record` is the typed review of host ↔ remote /
OSC-52 round-trip clipboard transfers. The `transcript_action_class`
on this record MUST be either `copy_action_to_host_clipboard_with_
representation_label` or `share_action_via_typed_export_route` — open
or rerun on a clipboard payload is a paste packet on
`schemas/execution/command_boundary_marker.schema.json` or a command-
dispatch descriptor, never this record.

Every direction in the seven-value vocabulary MUST cite a
`target_identity_witness_ref` — host ↔ remote, OSC-52 round-trip in
either direction, and remote-clipboard-bridge directions all require
the witness so the renderer can paint the host-vs-remote cue.

`bridge_denied_on_restored_transcript_inspect_only` is the typed
refusal that fires whenever the parent terminal-session record is in
any `transcript_*` `live_or_transcript_state_class` — a restored
transcript never silently regains clipboard-bridge authority.

### 3.4 Linkification review

`linkification_review_record` is the typed review of an OSC-8
hyperlink, a path token, or a URL token surfaced inside the transcript.
The record's `transcript_action_class` MUST be one of `open_action_
inspect_only_no_run`, `copy_action_to_host_clipboard_with_
representation_label`, or `share_action_via_typed_export_route` —
`rerun_action_through_command_dispatch_descriptor_only` is forbidden
on this record.

The exact URL / path / hyperlink target bytes never project. Only the
typed `link_target_class` (seven-value, with `url_safe_scheme_https_
or_mailto_or_steam_safe`, `url_unsafe_scheme_requires_review`,
`path_token_canonical_workspace_path`, `path_token_outside_workspace_
requires_review`, `osc_8_hyperlink_with_safe_target`, `osc_8_hyperlink_
with_unverified_target_requires_review`, and `link_target_class_
unknown_requires_review`) plus an opaque `link_target_summary_ref`
project.

`linkification_decision_class = rerun_intent_forbidden_link_is_not_a_
command` is the typed refusal that fires whenever a transcript surface
attempts to attribute rerun authority to a hyperlink or a path token;
rerun authority lives only on a command-dispatch descriptor. Unsafe-
scheme, unverified OSC-8, outside-workspace path, and unknown target
classes MUST resolve to a held / denied decision; admitted decisions
MUST cite a preview record ref.

## 4. Restore-no-rerun, clipboard bridge, linkification — invariant table

The four invariants the contract guarantees on a restored or exported
transcript:

| Invariant | Where it lives | Failure mode when missing |
|---|---|---|
| Restored panes never auto-rerun | `restore_no_rerun_attestation_record.auto_rerun_forbidden = true` and `terminal_session_record.auto_rerun_admitted_on_this_record = false` on every transcript-state row | A user (or an AI assist) reruns a stale command against new state |
| Restored panes never silently admit input | `restore_no_rerun_attestation_record.input_admission_forbidden_until_fresh_session = true` until a fresh session is minted | The restored pane behaves like a live shell but writes to a transcript that no longer maps to a PTY |
| Restored panes never silently admit clipboard bridge | `restore_no_rerun_attestation_record.clipboard_bridge_admission_forbidden_until_fresh_session = true` until a fresh session is minted | OSC-52 round-trip or remote-clipboard-bridge writes leak into the host clipboard from a transcript with no live target |
| Open / copy / share / rerun stay distinct | `open_copy_share_rerun_actions_must_remain_distinct` denial; `transcript_action_class` constraint on every review record | A "open this transcript line" action silently runs the line, or a "share" action silently copies raw bytes that the user did not authorise |

## 5. Out-of-band attestations and audit events

Every record family carries a closed `audit_event_id` vocabulary so a
reviewer can read the chronology without raw bodies. The terminal-
session family covers published / revoked / state-transitioned /
capability-limit entered / exited / transcript-capture posture
narrowed / broadened, plus the silent-paint / silent-auto-rerun /
silent-boundary-cue-collapse / silent-cwd-widening / silent-shell-
integration-widening / audit-denial denials. The boundary / input /
paste family covers boundary published / revoked / confidence-narrowed,
input-request opened / closed-with-typed-result / timed-out / silent-
secret-capture forbidden, and paste-safety admitted / held / denied /
silent-admit forbidden / silent-run-intent forbidden. The transcript /
restore / bridge / linkification family covers admitted / held /
denied for each route, the silent-admit forbiddens, the silent-input-
admission and silent-bridge-admission forbiddens on the restore
attestation, the rerun-intent-on-link forbidden, and the contract-
level `open_copy_share_rerun_actions_collapsed_forbidden_denial`. Raw
bodies, raw command lines, raw paste bytes, raw URLs, raw paths, raw
secret values, and raw clipboard bytes MUST NOT appear on any audit
event.

## 6. Versioning

Each schema declares its own `*_schema_version` const at value `1`.
Adding a new enum member, a new optional field, or a new sub-record
kind is additive-minor and bumps the matching version const.
Repurposing an existing enum member or removing one is breaking and
requires a new decision row. Adding a new record kind to the
top-level `oneOf` of a schema is additive-minor as long as it does
not change the semantics of an existing record kind. Adding a new
schema file under `schemas/execution/` is additive-minor under the
execution-family rules in
[`/artifacts/governance/schema_families.yaml`](../../artifacts/governance/schema_families.yaml).
