# Ship child-envelope derivation and nested-launch narrowing

This document is the canonical contract for the M5 **child-envelope derivation**
packet: the export-safe record of what happens when an already-authorized M5
execution **spawns a child launch**. The capability-envelope packet states the
authority one issued execution holds; this packet governs the derivation of a
narrower envelope for each nested launch across the notebook, scaffold, request,
database, AI, and debug lanes. A child launch may only ever derive an envelope
that is **narrower than its parent** — never wider, and never the raw OS
environment. Desktop, command, policy, CLI/headless, diagnostics,
support-export, help/About, and release surfaces consume one derivation object
instead of cloning per-lane narrowing prose.

- Implementation: `crates/aureline-policy/src/ship_child_envelope_derivation_nested_launch_narrowing_handle_only_secret_projection_and_no_ambient_privilege_enforcemen/`
- Boundary schema: `schemas/execution-auth/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.schema.json`
- Support export (truth source): `artifacts/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/support_export.json`
- Markdown summary: `artifacts/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen.md`
- Narrowed fixtures: `fixtures/execution-auth/m5/ship-child-envelope-derivation-nested-launch-narrowing-handle-only-secret-projection-and-no-ambient-privilege-enforcemen/`
- Producer / validator: `cargo run -p aureline-policy --example dump_m5_child_envelope_derivation`

## Track invariant

No ambient privilege. A child launch never inherits the raw OS environment and
never fans out full parent authority: `ambient_environment_posture` is never
`raw_os_environment_inherited`, and `inherits_full_parent_authority` is always
false. No AI tool, recipe, extension, browser route, or remote helper
self-issues a child envelope: every derivation for an untrusted-helper actor
carries an externally issued lineage flagged `self_issued_by_executor: false`.
Secret projection into a child defaults to handle-only references; raw secret
material never crosses the derivation boundary. If the platform's
execution-isolation backend cannot honor the child profile, the derivation
**narrows to a stricter profile or becomes visibly unsupported and fails
closed** — it never silently widens or runs unconfined.

## Lanes

Each nested-launch lane spawns a child from a parent execution:

| Lane | Nested launch | Parent matrix surface |
| --- | --- | --- |
| `notebook` | A notebook kernel forking a worker or sub-kernel. | `notebook_kernel` |
| `scaffold` | A scaffold/generator hook running a child generator subprocess. | `scaffold_hook` |
| `request` | A request/API lane fanning out a derived follow-up call. | `request_api_send` |
| `database` | A database action opening a nested session or sub-transaction. | `database_action` |
| `ai` | An AI tool invoking a sub-tool or spawning a helper process. | `ai_tool` |
| `debug` | A debug session launching a debuggee subprocess. | _(none — bounded by the parent snapshot)_ |

A packet missing any lane fails validation (`required_lane_missing`). The debug
lane has no standalone frozen-matrix surface, so its parent authority is bounded
only by the recorded `parent` snapshot rather than a matrix row; the five other
lanes additionally have their parent snapshot checked against the matrix row for
their surface (`parent_capability_widens_matrix`).

## What a derivation carries

Each `M5ChildEnvelopeDerivation` binds a parent-authority snapshot to a derived
child envelope:

| Field | Meaning |
| --- | --- |
| `derivation_id` | Stable id for this nested launch. |
| `lane` | The nested-launch lane. |
| `actor` | The actor class, an export-safe `actor_ref`, and any delegated `on_behalf_of`. |
| `parent` | A snapshot of the parent's granted capabilities, allowed scope, sandbox profile, secret scope, policy epoch, and expiry — the ceiling the child may not exceed. |
| `child` | The derived child envelope: target identity, granted capabilities, allowed scope, handle-only secret refs, secret scope, sandbox profile, policy epoch, and expiry. |
| `ambient_environment_posture` | How the parent's ambient environment reaches the child: `no_environment_inherited`, `allowlisted_handles_only`, or `brokered_environment_refs`. The forbidden `raw_os_environment_inherited` value fails validation. |
| `inherits_full_parent_authority` | Always false. A child never fans out full parent authority. |
| `enforcement_backend` | The execution-isolation backend enforcing the child profile. |
| `enforcement_status` | `enforced`, `narrowed_to_stricter_profile`, or `unsupported_visibly_degraded`. The forbidden `silently_permissive_unsupported` value fails validation. |
| `audit_lineage` | The external issuer class and ref, the parent envelope ref, the approval-ticket ref, the ordered `decision_chain`, and `self_issued_by_executor: false`. |
| `degraded_fallback` | What the child narrows to when its derived authority cannot be honored. |
| `applied_downgrade_triggers` / `applied_narrowings` / `narrowed_below_baseline` | The triggers and dimensions of any runtime downgrade. The flag is true exactly when both lists are non-empty. |

## Narrowing rules

For every derivation the validator proves the child only narrows the parent:

- **Capabilities** — every child capability class is a subset of the parent's
  (`child_capability_widens_parent` otherwise).
- **Scope** — every child allowed root/sink/endpoint is contained within a
  parent scope entry (same kind, label prefix) at an equal-or-narrower access
  mode (`child_scope_widens_parent` otherwise).
- **Sandbox** — the child sandbox profile is equal-or-stricter than the parent's,
  where strictness runs `in_process_trusted_local` < `brokered_network_only` <
  `subprocess_isolated_local` < `container_isolated_local` <
  `isolated_remote_runtime` < `inert_no_execution`
  (`child_sandbox_widens_parent` otherwise).
- **Secret scope** — the child secret scope is equal-or-narrower than the
  parent's (`child_secret_scope_widens_parent` otherwise), and secret projection
  is handle-only and consistent with the declared scope
  (`secret_projection_not_handle_only`, `secret_scope_inconsistent`).
- **Expiry** — the child `expires_at` is no later than the parent's, with a
  non-zero `ttl_seconds` (`child_expiry_exceeds_parent`, `expiry_missing`).
- **Policy epoch** — the child runs under the parent's policy epoch
  (`policy_epoch_mismatch`).

## Downgrade and fail-closed behavior

When a downgrade trigger fires (for example `enforcement_backend_missing` or
`sandbox_profile_unavailable`), the child narrows below its baseline derivation:
`narrowed_below_baseline` is set, `applied_downgrade_triggers` and
`applied_narrowings` are populated, the child tightens to a stricter
(often `inert_no_execution`, secretless, environment-free) profile, and the
`enforcement_status` records `narrowed_to_stricter_profile` or
`unsupported_visibly_degraded`. A backend that ran the child unconfined would be
recorded as `silently_permissive_unsupported` and rejected
(`enforcement_silently_permissive`). The `all_narrowed_derivations.json` fixture
exercises this fail-closed path for every lane.

## Consumers

The packet is the single source of truth for nested-launch narrowing. Its
`consumer_projection` block records that desktop, command/policy, CLI/headless,
support export, diagnostics, help/About, and release-evidence surfaces each
consume the same derivation objects, and that remote and browser-routed surfaces
preserve derivation semantics off-device. Downstream surfaces must ingest these
records instead of re-deriving per-lane narrowing prose.
