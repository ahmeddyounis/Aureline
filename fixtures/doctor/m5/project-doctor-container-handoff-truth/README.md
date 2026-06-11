# Fixtures: Project Doctor container handoff truth

This directory contains fixture metadata for the
`project_doctor_container_handoff_truth` packet.

The canonical full corpus is checked in at:

`artifacts/doctor/m5/project-doctor-container-handoff-truth.json`

## Coverage

- **All three workflow surfaces** carry container-route share scenarios:
  `remote_preview`, `incident_workflow`, and `companion_follow`.
- **Both route kinds** appear: `published_port` and `tunnel`. **Every audience
  scope** appears: `local_only`, `lan`, `authenticated_team`, `org`, and
  `public`. **Every policy posture** appears: `policy_allowed`,
  `policy_restricted`, and `policy_blocked`.
- **Every time bound** (`session_bound`, `time_boxed`, `deadline`) and **every
  revocation state** (`active`, `expired`, `revoked`) appear. Every route is
  time-bound (non-empty `expires_at_ref`) and exposes a non-empty
  `revocation_action_ref`, so no route can behave like a durable silent share.
- **Every handoff channel** (`browser`, `companion`), **every liveness**
  (`live`, `snapshot`), and **every mutation scope** (`read_only`,
  `bounded_write`) appear. Every bounded-write handoff is `approval_gated`, so
  there is no unrestricted mutate channel.
- **Every handoff posture and reason** appears: `share_live`/`none` (team port
  and local-only port), `share_with_disclosure` for `audience_public`,
  `bounded_write_requires_approval`, `environment_mutation_disclosed`, and
  `policy_restricted`, `share_snapshot_only` for `route_revoked` and
  `route_expired`, and `blocked_offer_alternative` for `policy_blocked`.
- **The non-inheriting handoff gate** is provable: every scenario's published
  `published_handoff_posture` and `published_handoff_reason` equal the posture
  recomputed from its own policy posture, revocation state, audience scope, write
  scope, and disclosed environment mutation. Tampering with any input (revoking a
  `share_live` route, dropping approval gating from a bounded-write handoff)
  makes the published posture diverge and fails validation.
- **No durable opaque shares**: every handoff preserves owner/origin, engine,
  target, route, and service identity; every revoked or expired route collapses
  to a snapshot handoff with visible revocation and revocation evidence; and the
  writable-mount and lifecycle/install-script disclosure survives into `reopen`,
  `attach`, `rebuild`, `export`, and `support_bundle` flows.
- **No dead ends**: every scenario offers a non-empty stay-local alternative,
  including the policy-blocked one.
- **Cross-surface parity**: every scenario renders on `desktop_sheet`,
  `cli_inspect`, `headless_json`, `browser_handoff`, `companion_handoff`,
  `support_export`, and `incident_packet`, carries the locale-invariant
  `machine_meaning_keys`, and is metadata-safe (`redaction_class:
  metadata_safe_default`, `raw_private_material_excluded: true`,
  `overcapture_excluded: true`).
