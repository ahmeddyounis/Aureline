# Provisioning hooks, policy-bundle history, and admin-audit export beta

This document is the reviewer-facing landing page for the beta projection that
gives enterprise pilots one auditable provisioning and admin-audit model.

The projection is owned by
[`/crates/aureline-auth/src/provisioning/mod.rs`](../../../crates/aureline-auth/src/provisioning/mod.rs)
and consumed by
[`/crates/aureline-shell/src/admin_audit_export_beta/mod.rs`](../../../crates/aureline-shell/src/admin_audit_export_beta/mod.rs).
The headless inspector lives at
[`/crates/aureline-shell/src/bin/aureline_shell_admin_audit_export_beta.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_admin_audit_export_beta.rs).

## What the projection covers

Every claimed beta row exposes one inspectable record per event:

- **Provisioning hooks.** SCIM (managed and self-hosted) and signed-file
  alternatives (signed mirror, manual signed file, air-gapped signed transfer)
  are explicit about source, freshness, and lifecycle state. Local-advisory
  files and runtime preloads stay separable from managed-authority lanes.
- **Policy-bundle history.** Each transition (applied, staged,
  replaced-by-successor, rolled-back, revoked) names the pack id, version,
  predecessor pack id, source token, signature-state token, signer, and
  applied-at timestamp.
- **Entitlement changes.** Granted, revoked, scope-narrowed, scope-widened,
  seat-added, seat-removed, quota-changed, expiry-shortened, and
  expiry-extended events carry before / after authority tokens, the source
  that authorised the change, and a plain-language explanation.

The same vocabulary surfaces under all four beta profiles:

| Profile               | Authority source                                                |
| --------------------- | --------------------------------------------------------------- |
| `connected`           | Live SCIM and managed origins available.                        |
| `mirror_only`         | Signed mirror is the only authority.                            |
| `offline`             | Air-gapped courier or last-known-good snapshot only.            |
| `enterprise_managed`  | Manual signed-file import drives provisioning and entitlements. |

## Acceptance posture

- Provisioning hooks and signed-file alternatives declare their source,
  freshness (`live`, `cached`, `stale`, `expired`, `missing`), and lifecycle
  state (`active`, `suspended`, `disabled`, `deleted`). Stale, expired, and
  missing postures fail closed and never silently widen managed authority.
- The admin-audit export composes provisioning events, policy-bundle history,
  and entitlement changes into a single page consumed by admin, support,
  shell, and reviewer surfaces. Enterprise docs and support playbooks reuse
  the same token vocabulary.
- Mirror, manual-signed-file, and air-gapped lanes preserve signer id,
  signed-at, fetched-at, valid-until, transport label, and signature blob ref
  verbatim from their upstream bundle. None of these lanes fall back to
  unsigned public endpoints, plaintext secrets, or implicit managed
  assumptions.

## Failure-mode drills

The headless inspector emits typed defects for three deliberate failure modes
under
[`/fixtures/security/m3/admin_audit_export/`](../../../fixtures/security/m3/admin_audit_export/):

- `drill_managed_source_missing_signature.json` — a SCIM provisioning event
  drops its signature blob ref; the validator surfaces
  `managed_source_missing_signature`.
- `drill_history_missing_predecessor.json` — a `replaced_by_successor`
  history event loses its `replaces_pack_id`; the validator surfaces
  `history_transition_missing_predecessor`.
- `drill_public_fallback.json` — a provisioning hook permits an undeclared
  public endpoint fallback; the validator surfaces
  `hidden_public_endpoint_fallback`.

## Headless inspector commands

```sh
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- page
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- provisioning-events
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- policy-bundle-history
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- entitlement-changes
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- defects
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- summary
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- validate
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- drill-managed-source-missing-signature
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- drill-history-missing-predecessor
cargo run -q -p aureline-shell --bin aureline_shell_admin_audit_export_beta -- drill-public-fallback
```

## Stable contract refs

- Record kind: `security_admin_audit_export_beta_page_record`
- Shared contract ref: `security:admin_audit_export_beta:v1`
- Source matrix:
  [`/artifacts/security/m3/admin_audit_exports/admin_audit_matrix.yaml`](../../../artifacts/security/m3/admin_audit_exports/admin_audit_matrix.yaml)
- Baseline support export:
  [`/artifacts/security/m3/admin_audit_exports/baseline_support_export.json`](../../../artifacts/security/m3/admin_audit_exports/baseline_support_export.json)
- Schema:
  [`/schemas/security/admin_audit_export_beta.schema.json`](../../../schemas/security/admin_audit_export_beta.schema.json)

## Support playbook reuse

Support bundles, admin console renderings, and reviewer fixtures share the
same defect-kind vocabulary. A defect emitted by the headless inspector is
identical (record kind, kind token, subject id, field, note) to the defect
admin and support surfaces would report. Support playbooks therefore reference
the same exported vocabulary as the auditing surface — there is no parallel
admin-only language.
