# Connected account, installation grant, delegated credential, and effective-scope resolution beta

This document is the reviewer-facing landing page for the beta projection that
makes provider authority honest on the four claimed beta profiles. It builds
on the connected-provider registry alpha frozen in
[`/crates/aureline-provider/src/registry.rs`](../../../crates/aureline-provider/src/registry.rs)
and on the approval-ticket alpha frozen in
[`/crates/aureline-provider/src/approval_tickets.rs`](../../../crates/aureline-provider/src/approval_tickets.rs).
The contract is owned by
[`/crates/aureline-provider/src/account_scope/mod.rs`](../../../crates/aureline-provider/src/account_scope/mod.rs).

The schema lives at
[`/schemas/providers/effective_scope.schema.json`](../../../schemas/providers/effective_scope.schema.json)
and the source matrix at
[`/artifacts/security/m3/provider_scope/account_scope_matrix.yaml`](../../../artifacts/security/m3/provider_scope/account_scope_matrix.yaml).

## What the projection covers

Every claimed beta page is one
`providers_account_scope_beta_page_record` carrying five record kinds:

- **`providers_account_scope_beta_connected_account_row_record`** — one
  row per claimed signed-in human-account identity. Each row names:
  - Profile (`connected`, `mirror_only`, `offline`, `enterprise_managed`)
  - Acting identity class (`connected_account`)
  - Workspace-bound subject (`subject_ref`, `subject_label`,
    `capability_hash_ref`)
  - Provider host binding (`canonical_host_ref`,
    `tenant_or_org_scope_ref`, `host_label`)
  - Auth source (`human_session`, `installation_grant`,
    `delegated_credential`, `project_scoped_grant`,
    `policy_injected_service`)
  - Lifecycle state (`active`, `reauth_required`, `revoked`,
    `suspended`, `unreachable`) with a typed export-safe note and an
    `observed_at` timestamp.
- **`providers_account_scope_beta_installation_grant_row_record`** — one
  row per claimed installation, app, or project-scoped grant. The row
  names the issuer, the bounded target scope refs, the lifecycle state
  (`installed`, `uninstalled`, `suspended`, `scope_narrowed`,
  `secret_expired`), and an optional managed-policy bundle ref. Acting
  identity class is fixed at `installation_grant`.
- **`providers_account_scope_beta_delegated_credential_row_record`** —
  one row per claimed delegated credential. The row names the delegating
  actor, the on-behalf-of actor, the delegated scope refs, the lifecycle
  state (`active`, `revoked`, `expired`, `scope_narrowed`,
  `delegator_lost_grant`), and the optional expiry horizon. Acting
  identity class is fixed at `delegated_credential`.
- **`providers_account_scope_beta_effective_scope_row_record`** — one
  row per provider-linked row. The resolution names:
  - The provider-linked row and target (`provider_linked_row_ref`,
    `target_ref`, `target_label`).
  - The acting-identity class (`connected_account`,
    `installation_grant`, `delegated_credential`) and the bound
    identity row.
  - The requested action (`read_only_inspection`,
    `human_authored_comment`, `review_decision_publish`,
    `issue_or_work_item_mutation`, `ci_run_or_check_mutation`,
    `release_publish`, `credential_projection`).
  - The provider-declared scope refs and the resolved scope refs (a
    subset).
  - A typed authority decision (`allowed`, `denied`, `browser_only`,
    `local_draft_only`).
  - A typed resolution reason (`allowed`, `allowed_with_downgrade`,
    `allowed_with_browser_handoff`, `allowed_with_deferred_publish`,
    or one of the `denied_*` variants).
  - A typed reapproval route (`system_browser_reauth`,
    `step_up_authenticator`, `account_reselection`,
    `installation_grant_reconsent`, `delegated_credential_reissue`,
    `browser_handoff`, `publish_later_deferred`,
    `admin_review_or_trust_grant`, or `none_required` only when the
    decision is `allowed`).
  - Optional refs that prove the chosen route is real: an
    `approval_ticket_ref` for allowed mutations, a
    `browser_handoff_packet_ref` for `browser_only`, and a
    `publish_later_queue_item_ref` for `local_draft_only`.
- **`providers_account_scope_beta_scope_drift_event_record`** — one
  event per observed scope drift or grant loss. The event names the
  originating identity row, the affected resolution row, a typed
  trigger (`grant_revoked`, `grant_suspended`, `scope_narrowed`,
  `delegated_credential_expired`, `installation_secret_expired`,
  `actor_class_changed`, `host_mismatch_detected`,
  `tenant_or_org_membership_changed`, `policy_epoch_rolled`,
  `trust_state_downgraded`, `freshness_floor_drifted`), and a typed
  forced downgrade (`force_inspect_only`, `force_local_draft_only`,
  `force_browser_handoff_only`, `force_step_up_authenticator`,
  `force_account_reselection`, `force_admin_review`,
  `force_disconnect_until_repair`, or `no_downgrade_required` only for
  benign refreshes). Drift events with a forced downgrade name a
  non-`none_required` reapproval route.

## Acceptance posture

- **Honest acting-identity class.** Every provider-linked resolution
  names exactly one acting-identity class and a bound identity row.
  Identity rows are validated to match their declared class
  (`identity_class_mismatch` defect on drift) and resolutions are
  validated to bind an identity ref that exists on the page
  (`bound_identity_ref_unknown` / `bound_identity_class_mismatch`).
- **Resolved scope is a true subset.** Every resolution names
  `provider_declared_scope_refs` and `resolved_scope_refs`. The
  validator flags `resolved_scope_not_subset_of_declared` and
  `allowed_resolution_without_resolved_scope` so silent scope widening
  cannot pass.
- **Decisions and reasons pair correctly.** `allowed` decisions pair
  with the allowed-family reasons (`allowed`,
  `allowed_with_downgrade`, `allowed_with_browser_handoff`,
  `allowed_with_deferred_publish`); other decisions pair with the
  denied family. The validator surfaces
  `allowed_decision_with_non_allowed_reason` and
  `non_allowed_decision_with_allowed_reason` defects.
- **Closed lifecycles cannot stay allowed.** A resolution that binds
  an identity row whose lifecycle state holds mutation closed
  (`reauth_required`, `revoked`, `suspended`, `unreachable`,
  `uninstalled`, `scope_narrowed`, `secret_expired`, `expired`,
  `delegator_lost_grant`) and emits `allowed` raises
  `allowed_decision_on_closed_lifecycle`.
- **Approval-ticket binding for allowed mutations.** A resolution that
  is `allowed` and whose requested action proposes a mutation must name
  an `approval_ticket_ref`; otherwise the validator surfaces
  `allowed_mutation_without_approval_ticket`.
- **Reapproval route is always real.** Non-allowed decisions must name
  a non-`none_required` reapproval route; `browser_only` decisions must
  carry a `browser_handoff_packet_ref`; `local_draft_only` decisions
  must carry a `publish_later_queue_item_ref`. The validator surfaces
  `non_allowed_decision_missing_reapproval_route`,
  `browser_only_without_handoff_ref`, and
  `local_draft_only_without_queue_ref` defects.
- **Drift forces a visible downgrade.** Every non-benign scope-drift
  trigger forces a typed authority downgrade and a non-`none_required`
  reapproval route. The validator surfaces
  `drift_event_downgrade_missing`,
  `drift_event_reapproval_route_missing`, and the catch-all
  `silent_mutation_authority_retained_after_drift` defects so a drift
  can never keep mutation authority silently.
- **No silent public-endpoint fallback or mutation widening.** Every
  row carries `public_endpoint_fallback_offered=false` and every
  resolution carries `public_endpoint_fallback_taken=false` and
  `mutation_authority_widened=false`. Flips raise
  `public_endpoint_fallback_offered`, `public_endpoint_fallback_taken`,
  and `silent_mutation_authority_widened` defects.
- **No raw token material on rows.** Identity rows carry
  `raw_token_material_present=false`. Flips raise the
  `raw_token_material_present` defect.
- **Local editing preserved.** Every claimed identity row carries
  `local_editing_preserved=true`. Flips raise the
  `local_editing_not_preserved` defect.
- **Fail-closed profile coverage.** All four beta profiles
  (`connected`, `mirror_only`, `offline`, `enterprise_managed`) must
  have at least one claimed row across the page; missing coverage
  surfaces `profile_coverage_missing`.
- **Token drift is caught.** Every typed enum on every record carries a
  string token mirror. Any drift between the typed value and the token
  raises the corresponding `*_token_drift` defect.
- **Lineage-preserved support exports.**
  [`AccountScopeBetaSupportExport`](../../../crates/aureline-provider/src/account_scope/mod.rs)
  wraps the page in a redaction-safe envelope that preserves
  identity lineage (which connected account, installation grant, or
  delegated credential actually acted), resolution lineage (which
  scope was resolved, which decision was made, and why), and drift
  lineage (which trigger forced which downgrade) verbatim. Raw access
  tokens, raw delegated-token bodies, raw provider payloads, and raw
  policy-injector material are excluded because the beta projection
  never carries them.

| Profile               | First-claim authority shape                                                                |
| --------------------- | ------------------------------------------------------------------------------------------ |
| `connected`           | Signed-in human session or live installation grant on the workstation.                     |
| `mirror_only`         | Signed mirror is the only authority; live providers are not reachable.                     |
| `offline`             | Imported snapshot only; no live reauth path until reconnect.                               |
| `enterprise_managed`  | Managed policy authority issues delegated or installation credentials per call.            |

## Verification

- Unit tests at
  [`crates/aureline-provider/src/account_scope/mod.rs`](../../../crates/aureline-provider/src/account_scope/mod.rs)
  cover the seeded page, each typed defect kind, the
  no-raw-token-material invariant, the
  no-silent-public-endpoint-fallback invariant, the
  no-silent-mutation-widening invariant, decision/reason pairing,
  resolved-scope subset enforcement, closed-lifecycle gating,
  browser-handoff / publish-later linkage, drift-forced downgrade,
  profile coverage, and the support-export redaction posture.
- The page builder [`seeded_account_scope_beta_page`] returns a page
  that validates with zero defects on all four beta profiles, includes
  every acting-identity class on a real resolution, and includes a
  drift-forced downgrade so reviewers can replay the scope-drift
  pathway end-to-end.
