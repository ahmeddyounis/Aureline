# M5 device-permission, mic-state, and capture/export contract

Status: frozen for M5 (schema version 1)

This contract governs how Aureline exposes device permission and capture state as
explicit, reversible product surfaces rather than hidden OS side effects. It is
produced by `crates/aureline-shell/src/m5_device_permissions/` and its headless
emitter `aureline_shell_m5_device_permissions`. The checked-in artifacts under
`artifacts/help/m5-device-permission-proof/` are projections of the seed builder;
the Rust `validate()` is the authoritative gate.

Sources: UI/UX Spec §§18.37, 18.44, §23.45, §23.54; TDD §7.13.10, §8.52, §9.43,
§11.2.10, Appendix CI; Milestones v3.1 connected-provider / embedded-boundary
launch controls.

## Schemas

- `schemas/help/m5-device-permission-row.schema.json` — the bundled set: device
  permission rows, mic-state pills, and capture/export reviews.
- `schemas/help/m5-mic-state-pill.schema.json` — the standalone mic-state pill and
  its transcript-correction strip.

## Records

### Device-permission row

Each row names, for one `device_class` (microphone, camera, screen capture, system
audio, clipboard): the `permission_state`, the `controlling_actor` (you, the
operating system, an administrator policy, or a connected provider), the
`processing_locality` cue, the `retention_mode`, the `data_exit_boundary`, whether
`capture_active`, the reversible `available_actions`, and bounded reviewable
storage/retention and actor notes.

Invariants:

- **Capture is never always-on.** `capture_active` may be true only while
  `permission_state` is `granted_in_use`; a row that is granted-in-use must report
  capture active, and no other state may.
- **Local processing is honest.** A row may not claim `local_on_device` processing
  when a provider is in the path — either because the controlling actor is a
  `connected_provider` or because the retention mode is
  `transcript_retained_provider_per_contract`. A `local_on_device` row must keep
  its data-exit boundary at `no_payload_leaves_product`.
- **Every row is reversible.** `open_system_settings` is always offered; a granted
  row also offers `revoke_in_app`; a not-yet-requested / denied / revoked row
  offers `request_access`.

### Mic-state pill

Each pill pins one of the seven `pill_state` values — `idle`, `listening`,
`muted`, `processing`, `needs_confirmation`, `unavailable`, `policy_blocked` — with
a `processing_locality` cue, a `correction_posture` (the transcript-correction
strip), an optional `confidence_cue`, the resolved command's
`command_capability_scope`, and whether a preview is required before commit.

Invariants:

- **Capturing states are legible.** `listening` and `processing` show a visible
  indicator and a real (non-`processing_unavailable`) locality cue.
- **Off states are typed.** `unavailable` and `policy_blocked` carry a typed
  `unavailable_reason` and `processing_unavailable`; `policy_blocked` uses
  `policy_locked_or_blocked`. Available pills carry no reason.
- **High-impact commands are gated.** When `command_capability_scope.is_high_impact`
  (recoverable durable, destructive bulk, or irreversible publish), the pill must
  be `needs_confirmation` with `correction_required_before_commit` and
  `preview_required_before_commit = true`. This routes high-impact spoken commands
  through the same preview/confirmation gate as any other mutating action, with
  transcript correction available before commit. A `needs_confirmation` pill always
  gates commit behind preview + required correction regardless of scope.

### Capture/export review

Each review names the `included_capture_classes`, the `retention_mode`, the
`redaction_state`, the `processing_locality`, the `data_exit_boundary`, and whether
`delete_available` / `export_available`, plus reversible actions.

Invariants:

- **Export is redaction-bounded.** A review may advertise `export_available` only
  when its `redaction_state` allows export (`redacted_before_export` or
  `metadata_refs_only`) and it offers `export_redacted_copy`; `delete_available`
  requires a `delete_now` action. The redaction state and data-exit boundary must
  be consistent, and local processing keeps payload on the machine.

## Set-level coverage

`M5DevicePermissionSet::validate` additionally requires: every device class named
exactly once; every mic-pill state present; every capture class reviewed somewhere;
at least one row not capturing (capture is not all-on); both local and
provider-backed processing represented; at least one high-impact pill proving the
confirmation gate; at least one deletable capture review; the required source
contracts present; and no forbidden raw material in the export.

## Artifacts

- `artifacts/help/m5-device-permission-proof/permission_set.json` — support export.
- `artifacts/help/m5-device-permission-governance.md` — governance summary.
- `artifacts/help/m5-device-permission-rows.csv` — one row per record.
- `fixtures/help/device-permissions/high_impact_confirmation_pill.json`
- `fixtures/help/device-permissions/provider_backed_capture_review.json`

Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- governance
cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- csv
cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- validate
```
