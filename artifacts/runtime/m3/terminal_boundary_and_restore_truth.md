# Terminal boundary and restore truth — M3 beta artifact

This artifact pins the closed vocabulary, contract surface, and acceptance
posture for the terminal protocol, clipboard, transcript/export, and
non-replay restore truth claimed on M3 beta runtime rows.

## Authoritative sources

- Runtime module: `crates/aureline-terminal/src/protocol_contract/mod.rs`
- Boundary schema (summary): `schemas/runtime/terminal_session_summary.schema.json`
- Boundary schema (export packet): `schemas/runtime/terminal_export_packet.schema.json`
- Fixture set: `fixtures/runtime/m3/terminal_protocol_and_restore/`
- Header strip: `crates/aureline-terminal/src/headers/mod.rs`
- Restore projection: `crates/aureline-terminal/src/restore/mod.rs`
- Shared-terminal control vocabulary: `crates/aureline-runtime/src/shared_terminal_alpha/mod.rs`
- Bounded scrollback: `crates/aureline-terminal/src/scrollback/mod.rs`

## Surface scope

Every claimed M3 beta terminal row — local pane, remote pane, shared pane,
support / transcript export view, and AI-promoted slice — projects the
`terminal_session_summary_record` defined by the runtime module. Surfaces
include:

- The bottom-panel terminal pane chrome and tab chip stack.
- The activity center terminal rows (live, recovered, reconnecting).
- The CLI / headless `aureline terminal` output.
- Support packets and transcript exports.
- AI tool-call surfaces that consume promoted slices.

No surface invents its own vocabulary for clipboard posture, bracketed paste
state, linkification, shared role, restore class, or reconnect drift; every
field is read verbatim from the summary record.

## Closed vocabulary

### Session class (`session_class_token`)

- `local_terminal`
- `remote_terminal`
- `shared_terminal`
- `export_support_view`
- `ai_promoted_slice`

### Live authority (`live_authority_token`)

- `live`
- `warming`
- `reconnecting`
- `recovered_transcript_only`
- `ended_requires_fresh_session`
- `read_only_degraded`
- `shared_narrowed_authority`
- `authority_blocked`
- `inspect_only`

### Clipboard posture (`clipboard_posture_token`)

- `local_allowed_with_preview`
- `remote_bridge_allowed_with_preview`
- `shared_scoped_allowed_with_preview`
- `blocked_by_policy`
- `blocked_by_trust`
- `blocked_by_secret_class`
- `not_applicable`

### Bracketed paste (`bracketed_paste_token`)

- `not_advertised`
- `advertised_disabled`
- `advertised_enabled`
- `forced_no_auto_submit`

### Linkification (`linkification_token`)

- `disabled`
- `metadata_only`
- `enabled_with_boundary_label`

### Shared role (`shared_role_token`)

- `not_shared`
- `owner_host`
- `view_only_observer`
- `follower`
- `active_writer_grantee`
- `approver_non_driving`
- `admin_non_driving`

### Recording / retention (`recording_class_token`)

- `not_recorded`
- `workspace_scoped_retention`
- `support_bundle_retention`
- `admin_scoped_retention`

### Denial reason (`denial_reason_token`)

- `no_denial`
- `admin_policy_blocked`
- `workspace_trust_narrowed`
- `protocol_budget_exceeded`
- `secret_class_detected`
- `restore_root_missing`
- `shared_control_grant_required`
- `target_identity_drift`
- `route_identity_drift`
- `reconnect_required`
- `fresh_session_required`

### Reconnect / restore drift (`reconnect_drift_token`)

- `not_applicable`
- `identity_unchanged`
- `host_or_workspace_drift`
- `toolchain_drift`
- `trust_narrowed`
- `policy_epoch_regressed`
- `unknown_requires_review`

### Export class (`export_class_token`)

- `metadata_only` — provenance, boundary, restore class, timestamps only.
- `support_bundle_scoped` — adds a bounded scrollback snapshot under
  `support_bundle_scoped` redaction.
- `ai_promoted_slice` — adds a typed `terminal_ai_promoted_slice_record`
  bound to a promoted-range provenance and `broadened_capture` redaction.

## Acceptance posture

The contract enforces, via `TerminalSessionSummary::validate()` and
`TerminalExportPacket::validate()`:

1. **Boundary truth is always visible.** Every row resolves to one session
   class and one live-authority class. Users can always tell whether the
   terminal is live, recovered transcript only, reconnecting, read-only
   degraded, or shared with narrowed authority.
2. **No silent clipboard writes.** Every clipboard posture either admits the
   write under a preview surface or refuses it with a typed
   `denial_reason_token` and a class-only `denial_label`. An OSC 52 or remote
   bridge write that does not carry a posture is invalid.
3. **No silent paste submit.** Bracketed paste state is part of the record.
   Export / inspect-only rows force `forced_no_auto_submit` so promoted slices
   and transcript reviews cannot submit silently.
4. **No silent shared widening.** A `shared_terminal` row always cites a
   typed `shared_role_token`. An `active_writer_grantee` row that omits
   `control_grant_ref` is invalid. Observer / follower / approver / admin
   rows never claim write authority by implication.
5. **No silent rerun on restore.** Every row keeps
   `recovery.auto_rerun_forbidden = true`. Recovered transcripts route the
   user through `cmd:terminal.open_fresh_session`; reconnect paths require
   renewed intent.
6. **Reconnect / restore disclose drift.** The `reconnect_drift_token`
   names host/workspace drift, toolchain drift, trust narrowing, policy
   regression, or unknown drift. Drift implies renewed intent before
   further execution.
7. **No raw bodies by default.** Export packets carry
   `raw_bodies_exported_by_default = false`,
   `raw_environment_or_secret_present = false`. Scrollback bodies cross only
   under a typed redaction class. AI-promoted slices cite explicit
   promoted-range provenance with admission ref and timestamps.
8. **Local terminal continuity preserved.** Every row records
   `local_terminal_continuity_preserved = true` so shared-control end or
   degrade never silently widens or narrows the local pane's authority.

## Verification

- Unit tests: `cargo test -p aureline-terminal --lib`
- Fixture conformance: `cargo test -p aureline-terminal --test protocol_contract_fixtures`
- Restore / protocol corpus conformance:
  `cargo test -p aureline-terminal --test restore_conformance`

## Out of scope

The contract does not promise:

- tmux replacement ambitions or advanced terminal theming.
- Full remote orchestration beyond the declared M3 lanes.
- Replay or auto-rerun semantics — the contract forbids them.
- Long-term transcript archive or analytics dashboards.
