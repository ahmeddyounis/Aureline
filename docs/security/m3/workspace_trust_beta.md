# Workspace trust and restricted mode beta

The beta workspace-trust projection turns the ADR trust-state matrix into a
single inspectable page consumed by shell, headless inspection, support export,
and reviewer fixtures. It covers the claimed open, run, debug, extension, AI,
provider, review, support, and admin lanes without allowing each lane to invent
its own `is_trusted` check.

## Contract

The shared contract ref is `security:workspace_trust_beta:v1`.

The auth-owned source of truth is
[`crates/aureline-auth/src/workspace_trust/mod.rs`](../../../crates/aureline-auth/src/workspace_trust/mod.rs).
The shell consumer and headless inspector live at
[`crates/aureline-shell/src/workspace_trust_beta/mod.rs`](../../../crates/aureline-shell/src/workspace_trust_beta/mod.rs)
and
[`crates/aureline-shell/src/bin/aureline_shell_workspace_trust_beta.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_workspace_trust_beta.rs).

The page exports:

- `security_workspace_trust_beta_row_record` for live lane rows.
- `security_workspace_trust_beta_support_row_record` for export-safe support rows.
- `security_workspace_trust_beta_defect_record` for validator findings.
- `security_workspace_trust_beta_page_record` for the complete audit page.
- `security_workspace_trust_beta_support_export_record` for support bundles.

## Required behavior

Restricted mode is present before trust on every claimed row. The restricted
floor stays usable: workspace open/restore, editor read/write, local search,
local Git inspection, admin-policy read, and redacted support export remain
available.

Rows that can execute code, mutate the workspace, use identity authority, reach
a provider, attach remote targets, install/update code, or render active
content are blocked, degraded, or approval-gated before trust. The row must
include an escalation cue such as `request_trust_grant_session_only`,
`request_approval_ticket`, or `continue_restricted_no_elevation`.

Trust elevation and trust loss use one audit vocabulary:

| Event | Meaning |
| --- | --- |
| `workspace_trust_granted` | Explicit user, admin, or signed-source trust elevation. |
| `workspace_trust_revoked` | Trust loss or revoke propagated to dependent surfaces. |
| `workspace_trust_policy_narrowed` | Managed policy narrowed a trusted workspace. |
| `workspace_trust_matrix_row_denied` | Matrix denied a surface request. |
| `workspace_trust_matrix_row_admitted` | Matrix admitted a surface request. |

Connected, mirror-only, offline, and enterprise-managed profiles are present on
every row. Mirror-only and offline rows refuse undeclared public endpoint
fallback; enterprise-managed rows apply signed policy narrowing before
dispatch.

## Validation

The validator emits typed `WorkspaceTrustBetaDefectKind` records:

| Defect | When it appears |
| --- | --- |
| `missing_matrix_surface` | A claimed matrix surface is absent from the page. |
| `missing_restricted_mode_availability` | A row does not expose restricted mode before trust. |
| `run_or_mutation_allowed_before_trust` | A run-capable or mutation-capable row is `allowed` before trust. |
| `trust_loss_not_propagated` | Trust loss does not narrow or propagate to shell/runtime/extension/support. |
| `missing_escalation_cue` | A trust-gated row lacks a trust or approval cue. |
| `profile_coverage_missing` | Connected, mirror-only, offline, or enterprise-managed profile coverage is missing. |
| `hidden_public_endpoint_fallback` | A profile permits undeclared public endpoint fallback. |
| `support_row_vocabulary_drift` | Support/export vocabulary differs from the live row. |
| `policy_degraded_widens_trusted` | Policy-degraded authority is wider than trusted authority. |
| `raw_private_material_exposed` | A row would expose raw private or secret material. |

The seeded page has zero defects. The failure drills under
[`fixtures/security/m3/workspace_trust/`](../../../fixtures/security/m3/workspace_trust/)
prove the validator catches a trust bypass, support/export drift, and hidden
public endpoint fallback.

## Reproduce

```sh
cargo run -q -p aureline-shell --bin aureline_shell_workspace_trust_beta -- validate
cargo test -p aureline-auth --lib workspace_trust
cargo test -p aureline-shell --lib workspace_trust_beta
cargo test -p aureline-shell --test workspace_trust_beta_fixtures
```
