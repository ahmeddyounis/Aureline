# Beta Remote Helper Drift-Repair Guidance and Exportable Diagnostics

This document is the reviewer-facing landing page for the beta drift-repair
lane: every claimed remote attach or reconnect exchange that does not fall
into the `adjacent_supported` baseline must surface (a) a typed explanation
of *which kind* of drift the user is looking at and (b) a bounded recovery
action whose authority impact is named, so reviewers, support, and
in-product surfaces read one truth.

The machine-readable boundary lives at
[`/schemas/workspace/remote_drift_repair.schema.json`](../../../schemas/workspace/remote_drift_repair.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/drift_repair/`](../../../crates/aureline-runtime/src/drift_repair/).
The source lane this beta promotes lives at
[`/crates/aureline-runtime/src/remote_helper_skew_beta/`](../../../crates/aureline-runtime/src/remote_helper_skew_beta/)
and is documented at
[`/docs/runtime/m3/remote_helper_skew_beta.md`](./remote_helper_skew_beta.md).

The beta promise:

- every claimed drift record mints one
  [`RemoteDriftRepairGuidance`](../../../crates/aureline-runtime/src/drift_repair/mod.rs)
  derived from a [`RemoteHelperBetaRecord`](../../../crates/aureline-runtime/src/remote_helper_skew_beta/mod.rs);
- the guidance names *which kind* of mismatch the record carries — version,
  capability, auth, route, or target — using a closed vocabulary;
- the guidance names a primary recovery action plus alternative actions from
  a closed vocabulary that includes the spec-required upgrade, downgrade,
  reconnect, and continue-local paths;
- every action stamps its authority impact (`maintains_current`,
  `narrows_authority`, or `requires_reapproval`). No action ever widens
  authority silently;
- the same guidance records can be projected into an exportable
  [`RemoteDriftRepairDiagnosticsPacket`](../../../crates/aureline-runtime/src/drift_repair/mod.rs)
  so reviewers and support read the same drift reasoning users see in-product.

## Drift-reason vocabulary

The closed `drift_reasons` vocabulary explains what the record is about. A
record may carry multiple reasons; reasons are ordered deterministically.

| Token | Meaning |
| --- | --- |
| `version_mismatch` | Client and helper versions disagree outside the supported window, or the visibility class is `probe_required_untested` |
| `capability_mismatch` | One or more capabilities cannot be admitted under the pairing |
| `auth_mismatch` | Trust or credential verification refused the helper boundary |
| `route_mismatch` | Reconnect captured an attach route or transport that is no longer valid |
| `target_mismatch` | The bound target is no longer reachable as a remote target |

The derivation lives in
[`drift_repair::derive_drift_reasons`](../../../crates/aureline-runtime/src/drift_repair/mod.rs).
Adding a token is a vocabulary change that must update the schema, this doc,
and the checked-in fixtures together.

## Repair-action vocabulary

The closed `repair_action` vocabulary names the bounded recovery actions
surfaces and diagnostics packets render:

| Token | When it applies |
| --- | --- |
| `no_repair_required` | Adjacent supported baseline; nothing to fix |
| `continue_narrowed_posture` | Inside the supported window but narrowed; mutation stays off |
| `run_drift_probe` | Untested pairing; run a probe or reattach |
| `reconnect` | Reconnect the helper before any other action |
| `upgrade` | Upgrade or repin client or helper to restore supported skew |
| `downgrade` | Narrow the requested capability set to match the helper |
| `continue_local_only` | Remote authority is refused; local work continues |
| `contact_admin_or_support` | Lane cannot self-repair; escalate |

## Authority-impact vocabulary

Every action stamps a closed authority impact:

| Token | Meaning | Requires re-approval? |
| --- | --- | --- |
| `maintains_current` | Authority is unchanged | no |
| `narrows_authority` | Authority shrinks (e.g. full-remote → local-only) | no |
| `requires_reapproval` | Authority would widen and must be re-approved | yes |

The validator at
[`/ci/check_remote_drift_repair_beta.py`](../../../ci/check_remote_drift_repair_beta.py)
enforces that:

- every `upgrade` action stamps `requires_reapproval`;
- every `requires_reapproval` action sets the `requires_reapproval` flag;
- every `narrows_authority` / `maintains_current` action is paired with the
  expected action class.

This is how the spec's guardrail — *no authority widens silently* — survives
into the schema, the fixtures, and the validator.

## Shared row id

A `RemoteDriftRepairGuidance` exposes a stable `guidance_row_id` of the form
`remote-drift-repair-row:<source-row>`, derived from the
`remote-helper-beta-row:<…>` row of the source record. The same id appears in
the exportable diagnostics packet so support and in-product surfaces
cross-reference one truth.

## Reviewer fixtures

The canonical fixture set lives under
[`/fixtures/runtime/m3/remote_drift/`](../../../fixtures/runtime/m3/remote_drift/)
and exercises one case per drift-reason class plus an adjacent-supported
baseline:

- `adjacent_supported_baseline.json` — no drift reasons; primary
  `no_repair_required`.
- `version_mismatch_upgrade_prompt.json` — `version_mismatch` and
  `capability_mismatch` reasons; primary `upgrade` with
  `requires_reapproval`, alternative `continue_local_only`.
- `capability_mismatch_downgrade_prompt.json` — `capability_mismatch`
  reason; primary `upgrade` with alternative `downgrade` (retryable drops).
- `auth_mismatch_contact_admin.json` — `auth_mismatch` reason; primary
  `contact_admin_or_support`, alternative `continue_local_only`.
- `route_mismatch_reconnect_probe.json` — `version_mismatch`,
  `capability_mismatch`, and `route_mismatch` reasons during a reconnect;
  primary `run_drift_probe`, alternatives `reconnect` and
  `continue_local_only`.
- `target_mismatch_continue_local.json` — `target_mismatch` reason; primary
  `continue_local_only`.

The fixture set is bundled into the exportable diagnostics packet at
[`diagnostics_packet.json`](../../../fixtures/runtime/m3/remote_drift/diagnostics_packet.json),
which reviewers replay through the runtime module to confirm the in-product
reasoning matches the exported reasoning.

The CI validator lives at
[`/ci/check_remote_drift_repair_beta.py`](../../../ci/check_remote_drift_repair_beta.py)
and validates the schema, the manifest coverage, the action / authority
pairing, the diagnostics packet shape, and the harness expectations.

Run the validator:

```sh
python3 ci/check_remote_drift_repair_beta.py --repo-root .
```

The integration test that replays the fixtures through the runtime module
lives at
[`/crates/aureline-runtime/tests/remote_drift_repair_beta.rs`](../../../crates/aureline-runtime/tests/remote_drift_repair_beta.rs).

## Out of scope for this beta

- Starting a real remote agent, helper binary, tunnel broker, or managed
  workspace runtime; the contract is data-only.
- Carrying raw endpoints, hostnames, paths, ports, credentials, helper
  process arguments, transport frames, logs, or support bundle bodies.
- Redefining compatibility rows, skew windows, remote-agent hello state,
  attach lifecycle state, or prebuild freshness vocabulary; those are
  imported from the source contracts and the compat artifacts.
- M6-class collaborative attach control, cross-org sharing, or
  cloud-control-plane productization beyond the bounded beta foundations.
