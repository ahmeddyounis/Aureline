# Beta Remote Helper Capability Negotiation and Skew-Window Enforcement

This document is the reviewer-facing landing page for the beta remote-helper
contract: every claimed beta remote attach or reconnect exchange minted a
typed, redaction-safe record that surfaces, support exports, and compatibility
reports read so users can answer "which remote capabilities are admitted right
now, and what is the next safe action if they are not?" without forking truth
per surface.

The machine-readable boundary lives at
[`/schemas/providers/remote_capabilities.schema.json`](../../../schemas/providers/remote_capabilities.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/remote_helper_skew_beta/`](../../../crates/aureline-runtime/src/remote_helper_skew_beta/).
The alpha primitive this beta layer promotes lives at
[`/crates/aureline-runtime/src/capability_negotiation/`](../../../crates/aureline-runtime/src/capability_negotiation/)
and the alpha contract is documented at
[`/docs/remote/helper_negotiation_alpha.md`](../../remote/helper_negotiation_alpha.md).

The beta promise:

- every claimed attach or reconnect exchange mints one
  [`RemoteHelperBetaRecord`](../../../crates/aureline-runtime/src/remote_helper_skew_beta/mod.rs)
  that names the lifecycle phase, the visible skew posture, the typed repair
  path, and the negotiated capability rows;
- unsupported skew fails closed: the record refuses remote mutation, names a
  truthful repair path (upgrade, repin, probe, reattach, local-only, or
  contact-admin), and never leaks raw versions, endpoints, or credentials;
- the same `(envelope_id, row_id)` pair carries through the support-export
  bundle and the compatibility-report row, so reviewers, support, and release
  consumers see one truth instead of three forks.

## Lifecycle phases

| Phase | Token | Meaning |
| --- | --- | --- |
| Initial attach | `attach` | Helper or remote agent was newly started for this exchange |
| Reconnect | `reconnect` | Session loss or version change triggered a renegotiation; `reconnect_attempt` is non-zero |

A reconnect carries the same record shape as an initial attach so support and
compatibility consumers do not branch on lifecycle phase to read truth.

## Visible skew posture

The closed `skew_visibility` vocabulary is what users, reviewers, and support
read. It is derived from the alpha
[`CompatibilityWindowStatus`](../../../crates/aureline-runtime/src/capability_negotiation/mod.rs)
and the alpha [`NegotiationOutcome`](../../../crates/aureline-runtime/src/capability_negotiation/mod.rs):

| Visibility | Meaning | Fails closed for mutation |
| --- | --- | --- |
| `adjacent_supported` | Inside the declared adjacent window; full remote admitted | no |
| `narrowed_supported_window` | Inside the supported window but narrowed posture | no (mutation already off) |
| `probe_required_untested` | Untested or best-effort pair; probe or reattach required | yes |
| `outside_supported_window` | Outside the declared supported window; refused until upgrade or repin | yes |

The derivation is total: adding a status or outcome variant is a build-time
signal in
[`RemoteHelperSkewVisibilityClass::derive`](../../../crates/aureline-runtime/src/remote_helper_skew_beta/mod.rs).

## Typed repair paths

Every non-supported visibility class names a truthful repair path. The
vocabulary is closed so status, support, and compatibility surfaces never
invent free-form recovery copy:

| Repair path | When |
| --- | --- |
| `no_repair_required` | `adjacent_supported` |
| `continue_narrowed_posture` | `narrowed_supported_window` |
| `run_drift_probe_or_reattach` | `probe_required_untested` (attach or reconnect) |
| `upgrade_or_repin` | `outside_supported_window` with helper-backed posture available |
| `continue_local_only` | `narrowed_supported_window` or `outside_supported_window` reduced to local-only |
| `contact_admin_or_support` | `outside_supported_window` with a fully blocked posture |

## Shared row id

A `RemoteHelperBetaRecord` exposes a stable `row_id` of the form
`remote-helper-beta-row:<alpha-row>`. The same id appears in:

- the [`RemoteHelperBetaCompatibilityRow`](../../../crates/aureline-runtime/src/remote_helper_skew_beta/mod.rs)
  projection compatibility reports consume;
- the [`RemoteHelperBetaSupportExport`](../../../crates/aureline-runtime/src/remote_helper_skew_beta/mod.rs)
  bundle support packets embed.

Reviewers and support can therefore cross-reference both surfaces by id without
re-deriving negotiation truth.

## Visible version disclosure

The record carries an opaque `visible_version_state` block with:

- `client_version` and `helper_version` tokens (e.g. `client@2026.05`,
  `agent@2026.05`); raw user-agent strings and raw process arguments are
  forbidden;
- `selected_protocol_version` (the protocol token the negotiation selected);
- `skew_case_ref`, `skew_window_declaration_ref`, and `compatibility_row_ref`
  imported from the canonical
  [`artifacts/compat/skew_windows.yaml`](../../../artifacts/compat/skew_windows.yaml),
  [`artifacts/compat/version_skew_register.yaml`](../../../artifacts/compat/version_skew_register.yaml),
  and
  [`artifacts/compat/qualification_matrix_seed.yaml`](../../../artifacts/compat/qualification_matrix_seed.yaml)
  sources.

Raw URLs, hostnames, IP addresses, ports, paths, query strings, headers,
tokens, environment values, and secret bytes never appear in this record.

## Reviewer fixtures

The canonical fixture set lives under
[`/fixtures/runtime/m3/remote_helper_skew_beta/`](../../../fixtures/runtime/m3/remote_helper_skew_beta/)
and exercises three named cases:

- `attach_adjacent_supported.json` — initial attach with `adjacent_supported`
  visibility, `no_repair_required` path, and full remote mutation;
- `attach_unsupported_skew_blocked.json` — initial attach with helper outside
  the supported window; mutation is refused, the repair path is
  `upgrade_or_repin`, and the support and compatibility consumers see the
  same `row_id`;
- `reconnect_probe_required.json` — reconnect on a same-major helper that has
  not been probed; visibility is `probe_required_untested` and the repair path
  is `run_drift_probe_or_reattach`.

The CI validator lives at
[`/ci/check_remote_helper_skew_beta.py`](../../../ci/check_remote_helper_skew_beta.py)
and validates the schema, the manifest coverage, and the support/compatibility
row-id pairing.

Run the validator:

```sh
python3 ci/check_remote_helper_skew_beta.py --repo-root .
```

The integration test that replays the fixtures through the runtime module
lives at
[`/crates/aureline-runtime/tests/remote_helper_skew_beta.rs`](../../../crates/aureline-runtime/tests/remote_helper_skew_beta.rs).

## Out of scope for this beta

- Starting a real remote agent, helper binary, tunnel broker, or managed
  workspace runtime; the contract is data-only.
- Carrying raw endpoints, hostnames, paths, ports, credentials, helper
  process arguments, transport frames, logs, or support bundle bodies.
- Redefining compatibility rows, skew windows, remote-agent hello state,
  attach lifecycle state, or prebuild freshness vocabulary; those are
  imported from the alpha contracts and the compat artifacts.
- M6-class collaborative attach control, cross-org sharing, or
  cloud-control-plane productization beyond the bounded beta foundations.
