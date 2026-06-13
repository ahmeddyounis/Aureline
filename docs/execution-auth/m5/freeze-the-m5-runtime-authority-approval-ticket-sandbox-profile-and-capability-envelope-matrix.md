# Freeze the M5 runtime-authority, approval-ticket, sandbox-profile, and capability-envelope matrix

This document is the canonical contract for the frozen M5 runtime-authority
matrix. The matrix locks one machine-readable row per claimed M5 **executing
surface** so that command, policy, secret-broker, remote, and help/support teams
consume one authority object instead of cloning per-surface approval or
capability prose.

- Implementation: `crates/aureline-policy/src/freeze_the_m5_runtime_authority_approval_ticket_sandbox_profile_and_capability_envelope_matrix/`
- Boundary schema: `schemas/execution-auth/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.schema.json`
- Support export (truth source): `artifacts/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/support_export.json`
- Markdown summary: `artifacts/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.md`
- Narrowed fixtures: `fixtures/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/`
- Producer / validator: `cargo run -p aureline-policy --example dump_m5_runtime_authority_matrix`

## Track invariant

No ambient privilege. No AI tool, extension, recipe, browser route, or remote
helper self-issues authority. For every executing surface the **target
identity, sandbox profile, secret scope, policy epoch, expiry, and degraded
fallback** stay inspectable and export-safe. If enforcement cannot be honored,
the surface **narrows or fails closed** instead of silently widening.

## What a row pins

Each `M5RuntimeAuthorityMatrixSurfaceRow` names, for one executing surface:

| Field | Meaning |
| --- | --- |
| `surface` | The claimed M5 executing surface. |
| `qualification` | `stable` / `beta` / `preview` / `experimental` / `unavailable` / `held`. |
| `default_sandbox_profile` | The execution-isolation profile the surface runs under by default. |
| `approval_ticket_posture` | How authority is issued — per-action, per-session, per-scope, standing policy ticket, read-only, or blocked. |
| `allowed_capability_classes` | The capability envelope the surface may exercise. |
| `secret_scope` | `no_secret_access`, `handle_only_delegated`, or `scoped_brokered_secret` — never raw material. |
| `degraded_fallback` | What the surface narrows to when full authority cannot be honored. |
| `unsupported_profile_behavior` | What happens when the default sandbox profile is unsupported on the running platform. |
| `ticket_expiry_seconds` | Expiry for time-bounded approval tickets (zero when not time-bounded). |
| `downgrade_triggers` | The conditions that automatically narrow the surface below its claim. |
| `consumer_surfaces` | Desktop, command, policy, CLI/headless, support, diagnostics, help/About, and release surfaces that must project the same truth. |

## Claimed M5 executing surfaces

`request_api_send`, `database_action`, `notebook_kernel`, `scaffold_hook`,
`preview_server`, `ai_tool`, `recipe`, `browser_routed_action`,
`incident_flow`, and `remote_mutation`. The matrix MUST name every surface; a
packet missing any surface fails validation (`required_surface_missing`).

`ai_tool`, `recipe`, `browser_routed_action`, and `remote_mutation` are
**untrusted helpers**: they may never self-issue authority, so their posture
must be externally issued (an approval ticket, a standing policy ticket, or
blocked). The highest-risk surfaces — browser-routed actions and remote
mutations — run in an isolated remote runtime and **fail closed** when that
runtime is unsupported on the platform.

## Enforced invariants

`M5RuntimeAuthorityMatrixPacket::validate` returns stable violation tokens. The
matrix is rejected (and the row cannot publish) when:

- `required_surface_missing` — a claimed executing surface is absent.
- `surface_row_incomplete` / `capability_envelope_empty` — a row lacks scope,
  source contracts, or capability classes.
- `stable_surface_missing_evidence` — a Stable surface carries no evidence ref.
- `self_issued_authority_forbidden` — an untrusted helper does not require an
  externally issued ticket.
- `elevated_capability_without_ticket` — a surface holds a mutating, process,
  network, or secret capability without an externally issued ticket.
- `ticket_expiry_missing` — a time-bounded ticket posture carries no expiry.
- `secret_scope_inconsistent` — a secret-projecting capability is declared
  without a secret scope.
- `downgrade_triggers_missing` / `consumer_surfaces_missing` — a row cannot
  narrow or be projected.
- `trust_review_incomplete` / `consumer_projection_incomplete` /
  `proof_freshness_incomplete` — a packet-wide invariant is unmet.
- `raw_boundary_material_in_export` — the export contains forbidden material.

## Downgrade and narrowing

Any missing enforcement backend or unclear authority row automatically narrows
the claim before docs, product surfaces, or release packets publish it. The
checked-in fixtures demonstrate auto-narrowing:

- `remote_mutation_unsupported_held.json` — `remote_mutation` narrowed to `held`
  when its isolated remote runtime is unsupported.
- `ai_tool_enforcement_backend_missing_narrowed.json` — `ai_tool` narrowed to
  `held` when its enforcement backend is missing.

Both remain complete, valid packets: narrowing changes the claim, it does not
hide the surface.

## Boundary

The packet is metadata-only. Raw secret material, credential bodies, raw
provider payloads, and live approval-ticket signatures stay outside the support
boundary. The matrix references the runtime-authority, approval-ticket, and
secret-handle contracts by id:

- `schemas/security/runtime_authority_issuer.schema.json`
- `schemas/security/authority_ticket.schema.json`
- `schemas/security/approval_ticket.schema.json`
- `schemas/security/secret_handle.schema.json`
- `schemas/security/m5-secret-boundary-depth.schema.json`

## Regenerating the artifacts

The in-code `frozen_stable_m5_runtime_authority_matrix_packet()` is the single
source of truth. Regenerate the checked-in artifacts after any change:

```sh
cargo run -p aureline-policy --example dump_m5_runtime_authority_matrix \
  > artifacts/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix/support_export.json
cargo run -p aureline-policy --example dump_m5_runtime_authority_matrix -- markdown \
  > artifacts/execution-auth/m5/freeze-the-m5-runtime-authority-approval-ticket-sandbox-profile-and-capability-envelope-matrix.md
```

A unit test asserts the checked-in support export deserializes back to the
frozen in-code packet unchanged, so drift fails CI.
