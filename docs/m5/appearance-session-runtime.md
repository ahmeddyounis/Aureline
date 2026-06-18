# M5 appearance-session runtime (companion doc)

This page is the companion to the M5 appearance-session runtime audit. It
freezes how the live appearance state becomes **one inspectable object** and how
every appearance change — preview, apply, cancel, validation failure, revert,
and OS signal — flows through **one explicit checkpoint** instead of ad-hoc
per-surface toggles. Appearance changes stay atomic and reversible from a single
checkpoint, and follow-system, resolved mode, accent source, text scale,
density, reduced-motion posture, and preview/revert posture stay inspectable
rather than inferred from current pixels.

The live session object, the checkpoint ledger, the transition ledger, and the
per-surface bindings all come from one checked-in truth path — the runtime
report — so the live appearance inspector, the docs/help and support-export
surfaces, the release-center packets, and the CI gate never disagree on what is
active right now or on how a change applies and reverts.

Authoritative artifacts:

- [`/artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md`](../../artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md)
  — the rendered audit (`artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md`).
- [`/fixtures/ux/m5/live-appearance-change/report.json`](../../fixtures/ux/m5/live-appearance-change/report.json)
  — the JSON snapshot (`fixtures/ux/m5/live-appearance-change/report.json`) every surface consumes.
- [`/fixtures/ux/m5/live-appearance-change/support_export.json`](../../fixtures/ux/m5/live-appearance-change/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/ux/appearance-session.schema.json`](../../schemas/ux/appearance-session.schema.json)
  — the boundary schema (`schemas/ux/appearance-session.schema.json`) the fixtures conform to.
- [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json)
  — the canonical appearance-session / checkpoint record schema this lane re-exports its vocabulary from.
- [`/tools/ci/m5/appearance_session_check.py`](../../tools/ci/m5/appearance_session_check.py)
  — the CI gate (`tools/ci/m5/appearance_session_check.py`) that keeps the audit fresh and honest.

## The live appearance session

The runtime mints exactly one `shell_m5_appearance_session_record` per session.
It is the canonical "what is active right now" object every surface reads
instead of inferring appearance from its own pixels:

- the active theme-package ref and revision ref;
- the follow-system posture (`follow_system`, `manual_override`,
  `managed_policy_override`, or `unavailable_platform_signal`);
- the resolved theme class, contrast mode, and accent source;
- the density class and text scale (with its source);
- the reduced-motion posture and the source that determined it;
- the current preview state and — when a preview or committed change is active —
  the single current checkpoint and rollback refs.

Golden-evidence packs, screenshots, support export, and diagnostics all name the
same `session_ref`, so the appearance the runtime used is never ambiguous.

## One checkpoint-aware state machine

Every appearance change flows through one explicit
`shell_m5_appearance_session_checkpoint_record`, and each preview / apply /
revert operation is one edge of a single state machine. The audit records each
edge as a `shell_m5_appearance_session_transition_record`:

| Op | Legal from | To | Apply state |
| -- | ---------- | -- | ----------- |
| `open_preview` | `not_previewing` | `preview_pending_validation` | `checkpoint_created` |
| `preflight_passed` | `preview_pending_validation` | `preview_live` | `preview_live` |
| `commit_preview` | `preview_live` | `preview_committed` | `committed` |
| `cancel_preview` | `preview_pending_validation`, `preview_live` | `not_previewing` | `reverted` |
| `validation_failed` | `preview_pending_validation`, `preview_live` | `preview_failed_reverted` | `preflight_failed` |
| `revert_committed` | `preview_committed` | `rollback_applied` | `reverted` |
| `os_signal_applied` | `not_previewing`, `preview_committed` | `preview_committed` | `committed` |

A transition that names no checkpoint, names a checkpoint not in the ledger, or
lands on a state the operation does not allow is a blocker — that is exactly the
half-updated state the lane exists to prevent. A `validation_failed` transition
**must** auto-revert to `preview_failed_reverted`.

## Restart-or-reload posture is explicit

When a platform or surface cannot apply a change live, the posture is disclosed,
never silent:

- the checkpoint and transition carry a reload / restart atomicity class
  (`surface_reload_from_single_checkpoint` or
  `full_restart_from_single_checkpoint`) and a matching rollback-path class;
- a surface whose `live_apply_capability` is not `applies_live` sets
  `restart_or_reload_disclosed`.

A change that requires a reload or restart but claims the live
`single_checkpoint_atomic` class — or a surface that needs a reload/restart but
does not disclose it — is a blocker.

## What the validator rejects

The audit fails the gate when any blocking finding remains:

- `transition_without_checkpoint`, `transition_unknown_checkpoint` — a change
  that does not flow through one explicit, resolvable checkpoint.
- `transition_illegal_state`, `validation_failure_not_reverted` — an illegal
  state-machine edge, or a validation failure that did not auto-revert.
- `transition_restart_reload_undisclosed`, `transition_atomicity_mismatch`,
  `checkpoint_restart_reload_undisclosed`, `surface_restart_reload_undisclosed`
  — a silent or mismatched restart-or-reload requirement.
- `transition_non_reversible`, `checkpoint_non_reversible`,
  `checkpoint_missing_rollback_path` — a change that is not reversible from a
  single checkpoint, or a checkpoint with no usable rollback path.
- `session_preview_without_checkpoint`, `session_rollback_without_ref`,
  `session_unknown_current_checkpoint` — a live session that is not
  self-consistent.
- `surface_not_on_session`, `surface_session_ref_mismatch`,
  `surface_missing_appearance_anchor`, `surface_missing_accessibility_note`,
  `surface_unknown_checkpoint` — a surface that paints its own appearance
  outside the shared session model.

## Consuming the audit

The cross-surface hardening matrix, the docs/help and support-export surfaces,
the release-center packets, and the sync/import and extension-inspection
surfaces ingest the checked-in `report.json` directly when they need to name the
live appearance session or reason about how a change applies and reverts. They
read these objects rather than restating appearance behaviour manually. In the
clean checked-in audit there are zero blocking findings.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- validate
cargo test -p aureline-shell --test m5_appearance_session_fixtures
python3 tools/ci/m5/appearance_session_check.py --repo-root .
```
