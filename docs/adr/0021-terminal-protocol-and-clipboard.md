# ADR 0021 — Terminal protocol, shell-integration signals, clipboard trust, and session-restore metadata

- **Decision id:** D-0026 (see `artifacts/governance/decision_index.yaml#D-0026`)
- **Status:** Proposed — this is an ADR seed. It reserves the PTY-ownership and terminal-session lifecycle, a minimum ANSI/VT-100 baseline, the shell-integration signal set, the clipboard-trust posture (including OSC-52), and the session-restore-metadata record shape so the command router, the platform adapter, the trust / policy packs, the support-export lane, and any future terminal-emulator lane at a later milestone cannot invent them ad hoc. Full freeze lands in a successor ADR once the open questions in §Open questions are closed.
- **Decision date:** pending
- **Freeze deadline:** 2026-12-15
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council (co-required with security_trust_review because the OSC-52 clipboard posture, the remote-clipboard-bridge rule, and the no-auto-rerun-on-restore rule carry the trust / policy invariants the trust-review remit already owns)
- **Related requirement ids:** `none`

## Context

Aureline's protected paths already assume the terminal is a first-class
surface. ADR-0007 froze the credential-handle projection and the
clipboard / OSC-52 posture for secret-bearing material. ADR-0009 froze
the execution-context object model that every shell / task-touching
capability resolves before it fires. ADR-0011 froze the five-axis
capability-lifecycle markers that project `terminal_manual_open` and
`terminal_repo_recipe_launch` rows. ADR-0016 froze the command-dispatch
boundary and the text-input normalization rules for `terminal_canvas`.
ADR-0018 froze the workspace-trust posture that gates terminal opening
in restricted mode. ADR-0019 seeded the `terminal-observe` host world
and the `terminal_observe_budget_class`. ADR-0020 seeded the
remote-agent session contract, including the placement row
`near_code_services.terminal` and the in-flight-action handling
vocabulary that governs what a reconnect may and may not replay.

The UI/UX and clipboard contracts already reserve terminal-shaped
fields. `docs/ux/clipboard_history_contract.md` §5.2 and §9 name
`multiline_terminal_paste`, `remote_clipboard_bridge`, and
`paste_then_run` as high-risk preview classes and name terminal
history as `evidence_only_no_rerun`. `docs/ux/entry_restore_truth_audit.md`
§6.2 pins `terminal_transcripts_not_rerun` on every restore prompt
that has not received a `live_session_continued` decision. The
`wit/aureline/terminal-observe.wit` interface already forbids launch
and input-injection capabilities on the observe world.

What none of these rows yet names is **the contract the local or
remote terminal MUST speak to claim PTY ownership, to publish shell-
integration signals without requiring injected scripts, to write to
the system clipboard from inside the session, and to carry its state
across a restore without re-executing prior destructive commands**.
Without a typed seed, every lane that touches a terminal — the shell
command router's `terminal_manual_open` / `terminal_repo_recipe_launch`
rows, the supervisor's `terminal_host` fault domain, the install /
attach lane's terminal-backed wizards, the support-export path's
terminal-transcript capture, the mutation-journal lane's
`evidence_only_no_rerun` claim, the session-restore sheet's
`terminal_transcripts_not_rerun` blocker, and any future emulator
replacement — would have to invent its own PTY lifecycle, its own
shell-integration parser, its own OSC-52 policy, and its own restore
decision. That is exactly the fragmentation this ADR seed forbids.

This ADR rides alongside ADR-0001 (identity-mode envelope inherited on
every session), ADR-0004 (terminal records cross RPC as typed payloads;
raw PTY bytes, raw escape sequences, raw clipboard bytes, and raw
command lines never do), ADR-0005 (terminal views ride the shared
subscription envelope with authority class `derived_knowledge` and a
declared freshness hint), ADR-0007 (secret-bearing copy from a
terminal selection honours the alias-only / reveal-on-demand posture;
OSC-52 writes of secret classes are denied by default), ADR-0008
(admin-policy narrowing is the orthogonal ceiling over every terminal
capability), ADR-0009 (execution-context resolution precedes any
terminal launch; the execution-context root is the cwd anchor), ADR-0011
(`terminal_manual_open` and `terminal_repo_recipe_launch` lifecycle
rows project through the five-axis markers), ADR-0016 (terminal paste,
clipboard, and command routing stay on the command-dispatch boundary),
ADR-0018 (trust-decision packet gates terminal opening in restricted
mode; repo-owned recipes do not auto-launch), ADR-0019 (the terminal-
observe world's budget and world-admission rules stay frozen; this
ADR names the local session those worlds observe), and ADR-0020
(remote-agent sessions that carry a terminal placement row resolve
this contract for the local protocol and this ADR for the local
session; remote-agent authority rules remain the ceiling on remote
terminals).

A production terminal emulator does not land at this milestone. What
this seed reserves is the **PTY ownership rules**, the **terminal-
session lifecycle states**, the **protocol baseline**, the
**shell-integration signal vocabulary**, the **clipboard-trust
rules**, the **session-restore metadata record**, and the **no-auto-
rerun posture** so the successor ADR has concrete records and invariants
to compose against rather than prose.

## Decision

Aureline reserves six record families — **terminal-session open
record**, **terminal-session lifecycle transition record**,
**shell-integration signal record**, **terminal-clipboard event
record**, **session-restore metadata record**, and **terminal-audit
event record** — plus a frozen vocabulary for PTY ownership, lifecycle
state, protocol conformance level, shell-integration signal kind,
clipboard-write class, restore decision, trust / policy suppression,
and denial reason. Every vocabulary below is opened as an enumerable
set whose initial members are frozen by this seed and whose additions
are additive-minor with a `terminal_protocol_schema_version` bump.
Repurposing any named item is breaking and requires a new decision
row.

The intent is deliberately narrower than the successor ADR. This seed
freezes **shape, names, invariants, and refusal posture**, not the
concrete emulator, not a renderer-side grid model, not a specific OSC
allow-list catalogue, and not a default keymap.

### PTY ownership and lifecycle

The host owns PTY lifecycle. A session MUST resolve through the
command-dispatch boundary from ADR-0016 and the execution-context root
from ADR-0009 before a PTY is allocated; a session launched via a
repo-owned recipe MUST resolve `terminal_repo_recipe_launch` against
the trust-decision packet from ADR-0018 first.

Reserved `pty_owner_class` values:

- `host_desktop` — PTY pair allocated and owned by the desktop shell
  process; the renderer is a display surface only.
- `remote_agent_primary` — PTY pair allocated by the remote-agent
  session per ADR-0020 `near_code_services.terminal` placement row;
  the desktop observes through the subscription envelope.
- `managed_workspace_agent` — PTY pair allocated by a managed-
  workspace agent per ADR-0020 `managed_workspace_agent_only`
  placement row; the desktop observes through the subscription
  envelope.
- `provider_side_remote_agent` — PTY pair allocated inside a provider-
  side session; every capability world denies mutation by default per
  ADR-0010 unless an approval ticket is present.
- `compatibility_bridge_remote` — PTY pair allocated through a
  compatibility-bridge remote per ADR-0019 bridge profiles.

Reserved `terminal_session_lifecycle_state` values:

- `session_requested` — command-dispatch route admitted, trust-decision
  packet evaluated, execution-context root resolved; allocation pending.
- `session_starting` — PTY pair allocated, shell binary spawned,
  protocol baseline advertised.
- `session_active` — PTY active, shell integration negotiated if
  available.
- `session_idle` — no output for longer than the declared idle
  threshold; no state change.
- `session_suspended` — UI detached (pane hidden / window minimised /
  device asleep); the PTY may still run if the OS permits.
- `session_lost_transport` — remote-owned PTY's transport dropped;
  reconnect window open per ADR-0020 reconnect-decision record.
- `session_reconnected_same_identity` — reconnect admitted with
  `target_identity_witness_match = matched`; UI state re-bound;
  prior commands NOT replayed.
- `session_reconnected_identity_changed` — reconnect admitted but
  witness changed; fresh PTY required; prior UI state becomes
  `evidence_only_no_rerun`.
- `session_kill_requested` — user or policy invoked terminate;
  SIGTERM / signal equivalent sent.
- `session_kill_confirmed` — process group exited; PTY closed.
- `session_restart_requested` — user invoked restart; prior state
  becomes `evidence_only_no_rerun`; a fresh PTY session is opened
  with the same command-dispatch parameters only if the user confirms.
- `session_closed` — orderly close (user, policy, or restart).
- `session_quarantined` — supervisor projected quarantine (protocol
  violation, resource budget exceeded, credential leak detected).

Rules (frozen):

1. **Kill and restart are host controls.** The command-dispatch route
   `terminal.kill`, `terminal.signal`, and `terminal.restart` are the
   only paths that terminate or recycle a session. A shell escape
   sequence MUST NOT be interpreted as an authoritative kill / restart.
2. **Suspended sessions do not silently resume input.** A session that
   re-enters `session_active` from `session_suspended`, from
   `session_lost_transport`, or from session-restore never replays
   latent input, latent paste buffers, or latent OSC sequences.
3. **Every session cites an execution-context root.** The cwd, the
   environment capsule, and the toolchain activator ride the
   ADR-0009 context-snapshot record; a terminal that cannot resolve
   the root refuses to open with `execution_context_root_unresolved`.
4. **Remote terminals remain remote-owned.** A terminal whose
   `pty_owner_class` is `remote_agent_primary`,
   `managed_workspace_agent`, `provider_side_remote_agent`, or
   `compatibility_bridge_remote` does not downgrade to a local PTY
   on reconnect. If the remote-agent session cannot be restored, the
   prior transcript remains as `evidence_only_no_rerun` and the user
   chooses a fresh local session explicitly.

### Protocol baseline (no shell injection assumed)

The terminal session MUST support a minimum protocol baseline without
requiring shell-specific init scripts. This baseline is what every
claimed conformance row leans on; anything above it is additive and
marked as such.

| Layer | Baseline requirement | Notes |
|---|---|---|
| Character encoding | UTF-8 in and out, with BOM and non-UTF input preserved as bytes until a declared decoder runs | Encoding normalization never drops bytes |
| Control sequences | ANSI / VT-100 core (CSI, SGR, cursor motion, erase, scrolling region) | No reliance on proprietary extensions |
| Line discipline | cooked and raw PTY modes per POSIX termios (or Windows ConPTY equivalent) | Windows sessions resolve through ConPTY |
| Input | key events, IME composition (per ADR-0016), bracketed paste, mouse events when enabled | Bracketed paste is the default on |
| Output size | explicit per-session cap on pending write buffer; over-cap writes pause the producer | Prevents unbounded memory growth |
| Window size | SIGWINCH / resize propagation from the shell zone to the PTY | Resize is a host signal, not shell-minted |

Reserved `protocol_conformance_level` values:

- `baseline_only` — the session implements the baseline above and no
  shell-integration signals; every shell-integration-dependent feature
  (command boundaries, prompt zones, rerun metadata, cwd tracking)
  degrades to a typed `not_available_without_shell_integration` state.
- `baseline_plus_osc_allowlist` — the session honours a frozen
  allow-list of OSC sequences (window title, hyperlinks with
  safe-preview, clipboard writes subject to §Clipboard trust) without
  requiring an injected script.
- `baseline_plus_shell_integration_optional` — the session is willing
  to consume shell-integration signals when a shell emits them; it
  MUST continue to function at `baseline_plus_osc_allowlist` when the
  shell does not.
- `baseline_plus_shell_integration_required` — a future conformance
  tier reserved for surfaces that refuse to open without shell
  integration. Not admitted at this milestone.

Rules (frozen):

1. **Shell injection is additive, never required.** A `baseline_only`
   or `baseline_plus_osc_allowlist` session MUST remain usable for
   interactive shell work, scrollback, copy, and export. Features that
   need shell integration (command-boundary navigation, rerun, prompt-
   zone focus return, exit-status badges) degrade to typed
   `not_available_without_shell_integration` chips.
2. **Baseline parity across claimed shells.** Conformance fixtures
   MUST pin `baseline_only` behaviour for **Bash**, **Zsh**, **Fish**,
   **PowerShell**, and **cmd.exe** without injected scripts. Claiming
   a feature on one of these shells that only works with injection is
   non-conforming.
3. **Unknown escape sequences fail safe.** A sequence outside the
   baseline and outside the declared allow-list is dropped, logged,
   and counted against the session's protocol-violation budget. A
   sequence MUST NOT terminate the session silently.

### Shell-integration signal contract

Shell integration is a typed channel from the shell to the session.
Every signal is a `shell_integration_signal_record`; free-form parsing
is forbidden.

Reserved `shell_integration_signal_kind` values:

- `prompt_begin` — shell is about to render a prompt; marks a prompt
  zone boundary.
- `prompt_end` — shell finished rendering a prompt; command entry
  begins.
- `command_begin` — shell is about to execute a command.
- `command_end` — shell finished executing a command; carries exit
  status and monotonic duration.
- `cwd_change` — shell's current working directory changed; carries
  the new cwd path token under ADR-0006 path-identity rules.
- `virtual_environment_hint` — shell reports a virtual environment
  activation or deactivation (venv, conda, nix-shell, managed toolchain).
- `rerun_metadata` — shell offers structured rerun metadata for the
  last command (the command tokens, the monotonic timestamp, the
  exit status). This is **evidence**, never an authority to rerun.
- `session_title_hint` — shell offers a human-legible session title.
- `protocol_level_advertised` — shell advertises its integration
  level; host narrows.

Reserved fields on every signal record:

| Field | Notes |
|---|---|
| `signal_id` | Opaque, stable id (safe to log). |
| `session_id` | The terminal session this signal belongs to. |
| `signal_kind` | One of the kinds above. |
| `sequence` | Monotonic per-session sequence number. |
| `observed_at` | Monotonic timestamp at the host. |
| `shell_family_declared` | Declared shell family (`bash`, `zsh`, `fish`, `powershell`, `cmd`, `other`). Informational only; the host does not trust shell self-report for authority. |
| `payload_refs` | Ordered refs to payload sub-records (cwd token, exit-status record, rerun metadata, virtual-env hint). |
| `protocol_level_observed` | Conformance level the host observed on this session. |
| `redaction_class` | Redaction posture on any payload (`metadata_and_hashes_only` default). |

Rules (frozen):

1. **Signals are evidence.** A signal MAY be used to render a prompt
   zone, a command boundary, or a virtual-env chip; it MUST NOT be
   used to invoke a command, elevate trust, or bypass a preview gate.
2. **Rerun metadata is evidence, not authority.** `rerun_metadata`
   lets the user re-execute via the command-dispatch boundary with
   explicit consent; the session NEVER reruns a command purely because
   rerun metadata exists.
3. **Cwd changes honour path identity.** The cwd token resolves
   through ADR-0006 canonical path identity; remote cwds resolve
   through the remote-agent target identity.
4. **Absent signals are a typed degraded state.** When a shell emits
   no integration signals, command-boundary navigation, prompt-zone
   focus return, and exit-status badges render
   `not_available_without_shell_integration` rather than silently
   disappearing.
5. **Raw command text and raw prompt bodies never cross RPC.**
   Command tokens ride as opaque refs with redaction envelope; raw
   prompt bytes are not retained past the session boundary.

### Clipboard-trust rules

A terminal session MAY participate in copy, paste, and OSC-52
clipboard operations subject to the trust / policy posture below.
Every clipboard-affecting action emits a
`terminal_clipboard_event_record`.

Reserved `terminal_clipboard_write_class` values:

- `host_initiated_copy` — user invoked copy via the command-dispatch
  boundary; default allowed.
- `host_initiated_paste` — user invoked paste via the command-dispatch
  boundary; routed through paste review per the clipboard-history
  contract.
- `osc52_write_from_local_shell` — shell inside a local `host_desktop`
  session requested an OSC-52 clipboard write.
- `osc52_write_from_remote_agent` — shell inside a remote-owned
  session requested an OSC-52 clipboard write.
- `osc52_read_request` — shell requested an OSC-52 clipboard read.
- `remote_clipboard_bridge_write` — a remote-agent session requested
  a clipboard bridge write through the host's clipboard adapter.
- `remote_clipboard_bridge_read` — a remote-agent session requested
  a clipboard bridge read.

Reserved `terminal_clipboard_suppression_class` values:

- `allowed_with_preview` — default for `osc52_write_from_local_shell`
  and `remote_clipboard_bridge_write`: the write is held pending a
  preview chip; the user confirms or silently ignores per the UX
  clipboard contract.
- `allowed_without_preview` — reserved for host-initiated copy on
  `representation_class = raw` selections that do not cross a
  high-risk preview class.
- `suppressed_by_trust` — the workspace-trust posture
  (`restricted` / `trust_revoked`) denies OSC-52 writes and remote-
  clipboard-bridge writes; the session shows a typed suppression
  chip rather than silently dropping.
- `suppressed_by_policy` — an admin policy pack denies the class;
  the chip names the policy source.
- `suppressed_by_secret_class` — the payload intersects a credential-
  handle class from ADR-0007 (`signing_key_material`, `device_secret`,
  `provider_session`, `ephemeral_operation_token`,
  `ssh_key_material`, `client_certificate`); denied regardless of
  trust state.
- `suppressed_by_content_class` — the payload intersects a high-risk
  preview class (`bidi_or_invisible_formatting_reveal`,
  `confusable_identifier_reveal`, `rich_active_content_render`) and
  the session's responsive-fallback posture cannot show the required
  preview chip.
- `denied_read` — OSC-52 read is denied by default; the session MUST
  NOT return clipboard contents into the shell without an explicit
  user-initiated read command through the command-dispatch boundary.

Rules (frozen):

1. **OSC-52 reads are denied by default.** A shell-initiated
   clipboard read MUST NOT silently return bytes into the PTY. The
   default posture is `denied_read`; any change requires a trust-
   scoped, policy-permitted, per-session user grant.
2. **Remote clipboard bridge is allow-with-preview at best.** A
   remote-agent-originated clipboard write crosses the host adapter
   only after a preview chip names the target-identity witness and
   the payload summary. The `remote_clipboard_bridge` high-risk
   preview class from the clipboard-history contract stays frozen.
3. **Secret classes are never clipboard-bridgeable.** Credential-
   handle reveal, token preview, and managed-secret inspector rows
   resolve through the ADR-0007 reveal contract only; the terminal
   clipboard path denies the write with `suppressed_by_secret_class`.
4. **Suppression is auditable.** Every suppressed event emits a
   `terminal_clipboard_event_record` with the `suppression_class`
   and the policy / trust ref; a silent drop is non-conforming.
5. **Policy-managed clipboard posture rides the admin-policy narrowing
   ceiling.** An admin policy pack MAY narrow OSC-52 writes, remote-
   clipboard-bridge writes, or OSC-52 reads to `never`. Policy MAY
   NOT widen the defaults.
6. **Boundary cues.** The session surface MUST render a visible
   local-vs-remote boundary cue whenever the `pty_owner_class` is
   not `host_desktop`. OSC-52 writes originating from a remote-owned
   session surface MUST NOT be rendered as local-shell writes.

### Session-restore metadata and no-auto-rerun posture

Every terminal session produces a `terminal_session_restore_metadata_record`
when the session state is captured for restore. The schema of record
is `schemas/terminal/session_restore_metadata.schema.json`. The record
carries **what the UI may restore** and **what MUST NOT be re-executed**.

Reserved `terminal_restore_level` values:

- `restore_ui_only` — restore the pane / tab topology, the visible
  title, and a placeholder for the prior transcript. No transcript
  bytes are materialised on restore; the pane opens as
  `session_requested` and asks the user to approve a fresh session.
- `restore_ui_with_transcript` — restore the transcript (bounded
  scrollback) alongside the pane topology, tagged as
  `evidence_only_no_rerun`. The pane is read-only until the user
  approves a fresh session.
- `restore_ui_with_transcript_and_hints` — as above plus the frozen
  shell-integration hints (the last observed cwd, the last virtual-
  env hint, the last exit status). Hints are evidence only; the
  session still opens fresh.
- `restore_declined_by_policy` — admin policy denied restore; the
  pane opens empty with a policy chip.
- `restore_declined_by_trust` — workspace trust is `restricted` or
  `trust_revoked`; restore declined.
- `restore_declined_by_missing_root` — the execution-context root is
  missing or unreachable; the pane opens with the typed missing-root
  placeholder per the entry-restore object model.

Reserved `terminal_restore_decision` values:

- `restore_approved_user_initiated_fresh_session` — the user accepted
  restore of UI state and approved opening a fresh session.
- `restore_approved_evidence_only` — the user accepted UI + transcript
  restore without opening a fresh session; the pane remains
  `evidence_only_no_rerun`.
- `restore_declined` — the user declined restore.
- `restore_declined_automatic` — automatic decline (policy, trust, or
  missing root).

Rules (frozen):

1. **No auto-rerun. Ever.** A restored terminal pane MUST NOT
   re-execute any prior command, replay any prior input, resume any
   prior running process, or continue any in-flight mutation. A
   restored pane without an explicit user-initiated fresh session
   remains `evidence_only_no_rerun`.
2. **Restore level is declared.** Every restored pane declares
   exactly one `terminal_restore_level`; the UI chip names it
   verbatim. Implying a higher level than delivered is non-
   conforming per `docs/ux/entry_restore_truth_audit.md` §6.2.
3. **Transcripts carry no executable authority.** A transcript row
   in a restored pane MUST NOT be invocable by hover, by double-
   click, by Enter, or by any default accelerator. Rerun affordances
   cite `rerun_metadata` through the command-dispatch boundary and
   require an explicit preview.
4. **Remote sessions never locally auto-re-open.** A pane whose
   `pty_owner_class` was remote and whose remote-agent session
   failed to reconnect inside the assigned reconnect window MUST NOT
   open a local replacement session silently. The user chooses.
5. **Credential-handle material is never persisted for restore.**
   Live secret reveals, OSC-52 writes of credential classes, and
   ephemeral tokens are not in the restore record by construction.
   A restore record that carries any raw credential material is
   non-conforming.
6. **Restore metadata crosses RPC as a typed record.** Raw PTY bytes,
   raw command lines, raw environment bytes, and raw prompt bodies
   never cross the restore boundary. Transcripts that are restored
   are redacted per the `redaction_class` posture and quoted by ref.
7. **User impact is named.** A pane blocked on restore names
   `terminal_transcripts_not_rerun` as its user-impact class per
   the entry-restore truth audit.

### Denial-reason vocabulary

Terminal-session denials fail closed; a silent downgrade to a best-
effort open or a best-effort restore is forbidden. Every denial
cites exactly one of the following.

- `terminal_pty_owner_class_unknown`
- `terminal_pty_allocation_failed`
- `terminal_execution_context_root_unresolved`
- `terminal_trust_state_denies_open`
- `terminal_policy_pack_denies_open`
- `terminal_protocol_violation_budget_exceeded`
- `terminal_protocol_level_required_not_advertised`
- `terminal_shell_integration_required_not_available`
- `terminal_clipboard_write_suppressed_by_trust`
- `terminal_clipboard_write_suppressed_by_policy`
- `terminal_clipboard_write_suppressed_by_secret_class`
- `terminal_clipboard_write_suppressed_by_content_class`
- `terminal_clipboard_read_denied_by_default`
- `terminal_remote_clipboard_bridge_denied`
- `terminal_restore_declined_by_policy`
- `terminal_restore_declined_by_trust`
- `terminal_restore_declined_by_missing_root`
- `terminal_restore_auto_rerun_forbidden`
- `terminal_remote_session_failed_no_local_fallback`
- `terminal_raw_body_forbidden_on_boundary`

### Audit-event vocabulary

Raw PTY bytes, raw escape-sequence payloads, raw command lines, raw
environment bytes, raw clipboard bytes, and raw shell prompts MUST NOT
appear on any audit event. Every event is an opaque, typed id.

- `terminal_session_opened`
- `terminal_session_open_denied`
- `terminal_session_closed`
- `terminal_session_kill_requested`
- `terminal_session_restart_requested`
- `terminal_session_quarantined`
- `terminal_session_reconnect_same_identity`
- `terminal_session_reconnect_identity_changed`
- `terminal_shell_integration_advertised`
- `terminal_shell_integration_signal_emitted`
- `terminal_clipboard_write_admitted_with_preview`
- `terminal_clipboard_write_suppressed`
- `terminal_clipboard_read_denied`
- `terminal_remote_clipboard_bridge_admitted_with_preview`
- `terminal_remote_clipboard_bridge_denied`
- `terminal_restore_offered`
- `terminal_restore_admitted_evidence_only`
- `terminal_restore_admitted_fresh_session`
- `terminal_restore_declined`
- `terminal_protocol_violation_observed`

### Schema of record

- Boundary schema: `schemas/terminal/session_restore_metadata.schema.json`
  (with `$defs` that also seat the terminal-session lifecycle,
  shell-integration signal, clipboard-event, and terminal-audit
  vocabularies so one schema is the cross-tool boundary). Conformance
  fixtures live under `fixtures/terminal/`.
- Shell-integration signal registry:
  `artifacts/terminal/shell_integration_signals.yaml` binds each
  signal kind to expected payload, protocol-level requirement, and
  per-shell baseline coverage across Bash, Zsh, Fish, PowerShell,
  and cmd.exe.
- Protocol / clipboard / restore worked cases:
  `fixtures/terminal/protocol_cases.yaml` pins the baseline-only,
  shell-integration, clipboard-trust, and restore scenarios the
  successor ADR MUST pass.

## Consequences

- **Frozen:** the PTY-owner-class vocabulary, the terminal-session
  lifecycle-state vocabulary, the protocol-conformance-level
  vocabulary, the shell-integration-signal-kind vocabulary, the
  terminal-clipboard-write-class vocabulary, the terminal-clipboard-
  suppression-class vocabulary, the terminal-restore-level vocabulary,
  the terminal-restore-decision vocabulary, the denial-reason set, and
  the audit-event id set.
- **Reserved:** the authority-boundary invariants. Kill / restart
  authority, trust-decision authority, admin-policy narrowing ceiling,
  execution-context resolution, and clipboard-suppression decisions
  stay host-owned. A terminal observes and projects these; it MUST
  NOT mint them. A shell escape sequence is never an authority.
- **Reserved:** the process-boundary constraints. Raw PTY bytes, raw
  escape-sequence payloads, raw command lines, raw environment bytes,
  raw clipboard bytes, and raw shell prompts never cross RPC.
  Terminal records cross as typed payloads quoted by ref.
- **Reserved:** the schema-of-record posture. The JSON Schema at
  `schemas/terminal/session_restore_metadata.schema.json` is the cross-
  tool boundary; the shell-integration registry at
  `artifacts/terminal/shell_integration_signals.yaml` binds shell
  families to baseline coverage; the worked fixtures at
  `fixtures/terminal/protocol_cases.yaml` pin the scenarios. No
  external IDL at this milestone.
- **Permitted:** later additive-minor additions to any enumerated set
  (new shell-integration signal kinds, new clipboard-write classes,
  new restore levels, new denial reasons, new audit events) with a
  schema / vocabulary bump.
- **Permitted:** admin policy packs, trust-state narrowing,
  capability-lifecycle markers, compatibility-bridge translation, and
  remote-agent placement rules MAY each narrow a terminal session
  further. None MAY widen.
- **Follow-up:** the successor ADR closes the open questions below
  (concrete emulator choice, OSC allow-list catalogue, keymap default,
  scrollback / search / export semantics, bracketed-paste posture on
  multi-cursor shells, line-protocol budget ceilings, per-shell
  integration script delivery story, and per-shell rerun metadata
  parity) and promotes this seed's `Proposed` status to `Accepted`.
- **Follow-up:** the shell command router, support-export lane,
  mutation-journal lane, session-restore sheet, clipboard-history
  contract, and trust-decision packet each cite this ADR as the
  governing terminal contract. A lane that hides the PTY owner class,
  session lifecycle state, shell-integration protocol level,
  clipboard-suppression class, or restore level on a terminal-
  touching action denies with the appropriate denial reason.
- **Ratifies:** ADR-0001 identity-mode envelope inherited, ADR-0004
  typed RPC payload rules, ADR-0005 subscription authority
  `derived_knowledge`, ADR-0006 VFS path identity for cwd tokens,
  ADR-0007 credential-handle projection and OSC-52 secret posture,
  ADR-0008 admin-policy narrowing ceiling, ADR-0009 execution-context
  resolution, ADR-0011 capability-lifecycle markers for
  `terminal_manual_open` / `terminal_repo_recipe_launch`, ADR-0016
  command-dispatch boundary, ADR-0018 trust-decision packet, ADR-0019
  `terminal-observe` host world and budgets, ADR-0020 remote-agent
  session contract and placement-row `near_code_services.terminal`.

## Alternatives considered

- **Defer terminal vocabulary until the emulator lands.** Rejected:
  the command router, the clipboard-history contract, the session-
  restore sheet, the mutation-journal `evidence_only_no_rerun` claim,
  the capability-lifecycle rows `terminal_manual_open` and
  `terminal_repo_recipe_launch`, the support-export lane's terminal-
  transcript capture, and the remote-agent placement row
  `near_code_services.terminal` already reserve terminal-shaped
  fields. Without a typed seed, each would either stay free-form or
  be minted per-surface.
- **Require shell-integration scripts on every session.** Rejected:
  the product commits to basic PTY usability for Bash, Zsh, Fish,
  PowerShell, and cmd.exe without assuming injection. Making
  integration mandatory would break a user's first-session
  experience when their shell init file is not Aureline-aware.
- **Let each surface mint its own OSC-52 posture.** Rejected: OSC-52
  sits at the intersection of workspace trust, admin policy, secret-
  class redaction, and remote-vs-local boundary cues. A shared rule
  is the only viable cross-surface posture.
- **Allow silent auto-rerun on restore when the command claims to be
  idempotent.** Rejected: idempotency is a property of the command's
  bytes, not its authority. Workspace-trust narrowing, policy pack
  changes, credential rotation, or remote-target replacement between
  the original run and the restore can invalidate the original
  authority. Auto-rerun is therefore never silent.
- **Let remote-agent sessions downgrade to a local PTY when the
  remote dies.** Rejected: a local replacement session carries
  different authority and different target identity; silently
  opening one would bypass the remote-agent reconnect contract.
- **External IDL + codegen (Protobuf, Cap'n Proto, Smithy).**
  Rejected: same reasoning as ADR 0004 through ADR 0020 — no second-
  language consumer yet beyond the JSON Schema boundary. The schema
  export reserves a clean integration point for the session, signal,
  clipboard, and restore records.

The `D-0026` `freeze_lane` default-if-unresolved posture would block
the terminal-command, clipboard-history, session-restore, support-
export terminal-transcript, and remote-agent terminal-placement lanes
from closing the terminal contract at the first-beta milestone until a
successor ADR lands. Accepting the seed's `Proposed` status now — with
its reserved vocabulary, records, signal registry, and worked
fixtures — avoids that freeze by giving the successor ADR concrete
records to compose against.

## Open questions

These MUST be answered by the successor ADR before this seed is
promoted to `Accepted`.

1. **Concrete emulator and renderer grid model.** Which emulator core
   does Aureline ship at the first beta (a bundled crate vs a vetted
   third-party implementation), and how does its grid model compose
   with the renderer's surface-tree?
2. **OSC allow-list catalogue.** Which OSC sequences are on the
   `baseline_plus_osc_allowlist` set (window title, hyperlink, OSC-52
   subject to this ADR, working-directory report, notification bell)?
   The catalogue is reserved but not yet enumerated.
3. **Default keymap.** What is the default terminal keymap (copy,
   paste, kill, restart, find, scrollback navigation) and how does it
   compose with ADR-0016 command-dispatch entries?
4. **Scrollback, search, and export semantics.** What is the scrollback
   retention policy, the search vocabulary, and the export record
   shape (plain text vs ANSI-preserved vs sanitized)? How does it
   honour the redaction envelope?
5. **Bracketed-paste posture on multi-cursor shells.** How does
   bracketed paste compose with multi-cursor editors, with IME
   composition, and with rerun-metadata-driven quick-rerun surfaces?
6. **Line-protocol budget ceilings.** What is the pending-write
   buffer cap per session, per identity mode, per trust state? What
   is the protocol-violation budget before `session_quarantined`?
7. **Per-shell integration script delivery.** How does Aureline ship
   (or refuse to ship) shell-integration scripts for Bash / Zsh /
   Fish / PowerShell / cmd.exe? Opt-in install? User-init-file hint?
   Never-injected?
8. **Per-shell rerun metadata parity.** Which rerun-metadata fields
   are available per shell, which shells carry none, and how do
   command-boundary surfaces degrade when a shell emits partial
   signals?
9. **Clipboard-bridge adapter story on each platform.** How does the
   remote-clipboard-bridge compose with macOS NSPasteboard, Windows
   clipboard, Linux X11/Wayland clipboards, and web host modes? How
   does policy narrow each per platform?
10. **Restore-level UI copy.** Which `terminal_restore_level` is the
    default offer, and how does the restore prompt quote it verbatim
    per the entry-restore object model?

Each question blocks the `Proposed` -> `Accepted` transition and is
tracked in the `decision_history` of `D-0026`.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — "terminals in restricted workspaces
  may be available for manual use, but repo-defined tasks, injected
  launchers, and auto-run scripts remain gated by workspace trust"
  (quoted in `artifacts/governance/decision_index.yaml#D-0023`).
- `.t2/docs/Aureline_Technical_Design_Document.md` — "restore
  layout, titles, and cwd hints only; user reaffirms execution
  intent" (quoted in `docs/adr/0016-shell-windowing-input-accessibility-boundary.md#source-anchors`).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — "IME
  composition across editor, palette, settings, terminal, and rename
  inputs" (quoted in `docs/adr/0016-shell-windowing-input-accessibility-boundary.md#source-anchors`).
- `.t2/docs/Aureline_Milestones_Document.md` — "Dialogs, sheets, and
  permission prompts … return focus to the logical origin" (quoted
  in `docs/adr/0016-shell-windowing-input-accessibility-boundary.md#source-anchors`).

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0026`
- RFC: none (the open-question option space runs down in the successor
  ADR).
- Session-restore metadata and terminal boundary schema:
  `schemas/terminal/session_restore_metadata.schema.json`
- Shell-integration signal registry:
  `artifacts/terminal/shell_integration_signals.yaml`
- Worked terminal fixtures:
  `fixtures/terminal/protocol_cases.yaml`
- RPC envelope this contract rides:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`
- Subscription envelope terminal views ride:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
- VFS path identity cwd tokens bind to:
  `docs/adr/0006-vfs-save-cache-identity.md`
- Secret-broker handle classes OSC-52 denies on:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
- Admin-policy narrowing ceiling this contract honours:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
- Execution-context model terminal launches bind to:
  `docs/adr/0009-execution-context-and-scope.md`
- Capability-lifecycle markers terminal rows project through:
  `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
- Shell / command-dispatch boundary terminal entry points route
  through:
  `docs/adr/0016-shell-windowing-input-accessibility-boundary.md`
- Workspace-trust packet every open resolves:
  `docs/adr/0018-workspace-trust-and-restricted-mode.md`
- Capability-world identity scheme the `terminal-observe` world
  projects through:
  `docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md`
- Remote-agent session contract remote-owned PTYs bind to:
  `docs/adr/0020-remote-agent-contract.md`
- Clipboard / undo-group / reopen-history contract terminal paste
  paths quote:
  `docs/ux/clipboard_history_contract.md`
- Shell-interaction-safety contract every preview and denial cites:
  `docs/ux/shell_interaction_safety_contract.md`
- Entry / restore truth audit restore copy quotes:
  `docs/ux/entry_restore_truth_audit.md`
- Terminal-observe world interface:
  `wit/aureline/terminal-observe.wit`
- Affected lanes:
  `governance_lane:architecture_council`,
  `governance_lane:security_trust_review`,
  `governance_lane:shell_command_system`,
  `governance_lane:accessibility_input_review`,
  `governance_lane:support_export`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance (as a seed at `Status: Proposed`). A successor ADR
promotes this seed to `Accepted` once the open questions are closed
and records the supersession in this section without rewriting the
body above.
