# OIDC system-browser sign-in, recovery, and session-continuity (beta)

The beta OIDC audit projects every claimed enterprise OIDC row into one
inspectable record so admins, support engineers, and reviewers can see, on
each row, that:

1. **Enterprise issuer source is disclosed.** Every claimed-enterprise row
   quotes a closed `OidcIssuerSourceClass` token plus an issuer label, issuer
   domain label, and JWKS source label. Rows MUST NOT silently fall back to a
   public issuer endpoint; the `public_issuer_fallback_used` flag flips a
   typed defect.
2. **Tenant + workspace binding is visible.** Every claimed-enterprise row
   quotes a closed `OidcTenantBindingClass` token and exposes the bound
   tenant ref, workspace ref, and actor subject ref. The validator rejects
   `tenant_binding_missing` and `workspace_binding_missing` when the
   declared binding requires a ref the row did not provide.
3. **Return-path semantics are explicit.** Every claimed-enterprise row
   quotes a workspace label, target label, requested-action label, a
   [`ReturnModeClass`](../../../crates/aureline-auth/src/browser_callback/mod.rs)
   token, an
   [`ReturnOriginValidationClass`](../../../crates/aureline-auth/src/browser_callback/mod.rs)
   token, a
   [`ReturnTenantOrWorkspaceMatchRule`](../../../crates/aureline-auth/src/browser_callback/mod.rs)
   token, and a stable return-anchor ref.
4. **Session continuity and sign-out preserve local editing.** Rows whose
   session state is `signed_out_local_intact`, `refresh_pending_managed_narrowed`,
   `refresh_expired_managed_blocked`, or `identity_outage_managed_blocked`
   MUST quote a sign-out continuity class that preserves local editing while
   narrowing managed actions. A signed-in active row MUST grant a non-empty
   scope.
5. **Identity outages and denial degrade truthfully.** Rows in an outage,
   denial, or refresh-expired state MUST quote a typed
   `OidcIdentityOutageClass` (not `no_outage`) and a closed
   `OidcRecoveryActionClass` that names the user-visible recovery path; the
   granted authority scope MUST NOT widen the requested scope.
6. **Support-export vocabulary parity.** The support row reuses the same
   closed-vocabulary tokens the live row paints (issuer source, tenant
   binding, return mode, session state, outage class, sign-out continuity,
   recovery action, requested and granted scope). Drift is a contract bug.

The seeded page seeds zero defects; the validator and the headless
inspector are what surface a regression when a row drops a required field,
silently falls back to a public endpoint, widens authority on outage, loses
local editing on sign-out, or drifts vocabulary across the live and support
rows.

## Contract surface

The beta projection ships five record kinds, all under the shared contract
ref `auth:oidc_system_browser_beta:v1`:

- `auth_oidc_system_browser_beta_row_record` — one audited row per claimed
  OIDC scenario. Each row carries a stable `case_id` and `row_id`, the source
  claim row ref, the profile token (connected / mirror_only / offline /
  enterprise_managed), the [`OidcIssuerDisclosure`](../../../crates/aureline-auth/src/oidc/mod.rs)
  (source token, issuer label, domain label, JWKS source label,
  `public_issuer_fallback_used` flag, optional discovery ref), the
  [`OidcTenantBinding`](../../../crates/aureline-auth/src/oidc/mod.rs)
  (binding token, optional tenant / workspace / actor refs), the
  [`OidcReturnPathLabel`](../../../crates/aureline-auth/src/oidc/mod.rs)
  (workspace label, target label, requested-action label, return-mode token,
  origin-validation token, tenant/workspace match-rule token, return-anchor
  ref), the [`OidcSessionContinuityBlock`](../../../crates/aureline-auth/src/oidc/mod.rs)
  (session state, sign-out continuity, local-editing summary, managed-action
  narrowing label, preserve / narrow flags, optional managed session state
  ref), the [`OidcIdentityOutageBlock`](../../../crates/aureline-auth/src/oidc/mod.rs)
  (outage class, reason label, recovery action, recovery label), and the
  requested + granted authority scope.
- `auth_oidc_system_browser_beta_support_row_record` — export-safe support
  row aligned 1:1 with the live row by `row_id`.
- `auth_oidc_system_browser_beta_defect_record` — typed defect emitted by
  the validator.
- `auth_oidc_system_browser_beta_page_record` — top-level page with the
  aggregate summary, live rows, support rows, and defects.
- `auth_oidc_system_browser_beta_support_export_record` — support-export
  wrapper that quotes the page plus a metadata-safe defect roll-up
  (`defect_kinds_present`, `defect_counts_by_kind`,
  `raw_private_material_excluded=true`).

The frozen JSON schema lives at
[`/schemas/auth/oidc_system_browser_beta.schema.json`](../../../schemas/auth/oidc_system_browser_beta.schema.json).

## Audit axes

| Axis | What the row must show |
| --- | --- |
| `enterprise_issuer_source_disclosed` | Claimed-enterprise row quotes an issuer source token plus issuer label, domain label, and JWKS source label; does not silently fall back to a public endpoint. |
| `tenant_and_workspace_binding_disclosed` | Claimed-enterprise row quotes a binding token and the tenant + workspace refs the binding requires. |
| `return_path_labels_preserved` | Row quotes workspace label, target label, requested-action label, return-mode token, origin-validation token, tenant/workspace match-rule token, and a stable return-anchor ref. |
| `session_continuity_preserves_local_editing` | Sign-out, refresh-expired, identity-outage, and auth-denial rows preserve local editing while narrowing managed actions; signed-in active rows grant a non-empty scope. |
| `identity_outage_degrades_truthfully` | Outage / denial / refresh-expired rows quote a typed outage class and a recovery action other than `no_recovery_required`. |
| `no_authority_widening_on_return` | Granted authority scope does not widen the requested authority scope. |
| `support_export_vocabulary_parity` | Support row reuses the same closed-vocabulary tokens as the live row. |

## Defect vocabulary

| Defect kind | When the validator emits it |
| --- | --- |
| `issuer_label_missing` | Claimed-enterprise row missing an issuer label. |
| `issuer_domain_label_missing` | Claimed-enterprise row missing the issuer domain label. |
| `jwks_source_label_missing` | Claimed-enterprise row missing the JWKS source label. |
| `public_issuer_fallback_used` | Row silently fell back to a public issuer endpoint. |
| `tenant_binding_missing` | Tenant-bound row missing the tenant ref. |
| `workspace_binding_missing` | Workspace-bound row missing the workspace ref. |
| `return_anchor_ref_missing` | Non-local row missing the return-anchor ref. |
| `return_mode_missing` | Row missing the return-mode token. |
| `sign_out_or_outage_loses_local_editing` | Sign-out / outage / refresh-expired row narrows local editing. |
| `identity_outage_missing_class` | Outage / denial / refresh-expired row quotes `no_outage` instead of a typed outage class. |
| `outage_recovery_action_missing` | Outage / denial / refresh-expired row does not name a recovery action. |
| `signed_in_active_without_granted_scope` | Signed-in active row claims `no_scope_granted`. |
| `return_widens_authority_scope` | Granted authority scope widens the requested authority scope. |
| `managed_session_state_ref_missing` | Claimed-enterprise row missing the managed session state ref. |
| `support_row_vocabulary_drift` | Support row drifted from the live row on a closed-vocabulary token. |
| `account_free_local_mislabeled` | Account-free local row still quoted a non-local issuer source or tenant binding. |

## Seeded page

The seed covers five protected states across the connected / mirror /
offline / enterprise-managed profiles:

| Row | State |
| --- | --- |
| `oidc:claimed:payments-prod:signed-in` | `signed_in_active` on a managed enterprise issuer with `loopback_port_pinned` origin validation and `must_match_bound_workspace_and_tenant`; read+write scope granted. |
| `oidc:claimed:payments-prod:refresh-expired` | `refresh_expired_managed_blocked`; local editing intact; recovery action `retry_refresh_in_system_browser`. |
| `oidc:claimed:payments-prod:signed-out` | `signed_out_local_intact`; local editing intact; recovery action `resume_in_system_browser`. |
| `oidc:claimed:payments-prod:issuer-unreachable` | `identity_outage_managed_blocked` on the offline profile; degrades to local-only; recovery action `continue_local_without_sign_in`. |
| `oidc:account-free-local` | `account_free_local_no_auth_required`; no issuer, no tenant binding, no remote scope. |

## Where to find the runtime

- Crate: [`/crates/aureline-auth/src/oidc/mod.rs`](../../../crates/aureline-auth/src/oidc/mod.rs)
- Shell consumer: [`/crates/aureline-shell/src/oidc_system_browser_beta/mod.rs`](../../../crates/aureline-shell/src/oidc_system_browser_beta/mod.rs)
- Headless inspector: [`/crates/aureline-shell/src/bin/aureline_shell_oidc_system_browser_beta.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_oidc_system_browser_beta.rs)
- Fixtures: [`/fixtures/auth/m3/oidc_system_browser/`](../../../fixtures/auth/m3/oidc_system_browser/)
- Schema: [`/schemas/auth/oidc_system_browser_beta.schema.json`](../../../schemas/auth/oidc_system_browser_beta.schema.json)
- Integration test: [`/crates/aureline-shell/tests/oidc_system_browser_beta_fixtures.rs`](../../../crates/aureline-shell/tests/oidc_system_browser_beta_fixtures.rs)

## Failure drills

Each `drill_*.json` fixture intentionally trips the validator and is
replayed end-to-end by the integration test:

| Drill | Defect raised |
| --- | --- |
| `drill_public_issuer_fallback.json` | `public_issuer_fallback_used` |
| `drill_sign_out_loses_local_editing.json` | `sign_out_or_outage_loses_local_editing` |
| `drill_outage_widens_authority.json` | `return_widens_authority_scope` |
| `drill_outage_missing_class.json` | `identity_outage_missing_class` |
