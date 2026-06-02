# Finalize the open-versus-paid boundary manifest, managed-offering truth, usage export, and offboarding packet

This stable lane publishes one inspectable boundary model that maps every
claimed capability to its boundary class — `open_local`, `managed_hosted`,
`enterprise_governed`, or `not_included` — and ships versioned usage-export
and offboarding packets that disclose quota family, record class, partiality,
retention posture, and tenant-scoped data that does not leave with the local
product. The runtime owner is
`aureline_policy::finalize_open_vs_paid_boundary_and_offboarding`.

The packet ensures that:

- Core local workflows — editing, search, Git, tasks, debugging, local indexing,
  and local-safe AI — remain usable without account creation or active managed
  connectivity.
- Managed or enterprise add-ons are never implied to be part of the open-local
  core.
- Offboarding, org-switch, seat-loss, and deprovision flows explicitly name what
  remains local, what becomes unavailable, what is still exportable, and which
  managed records persist for policy or billing reasons.
- Entitlement or seat loss degrades capabilities visibly rather than silently
  altering commands, routes, or data access.

## Contract

For the stable claim to hold, **all four** of the following conditions must be
verified simultaneously:

1. **Boundary manifest complete** — every claimed capability family carries a
   boundary row with a closed-vocabulary boundary class; no required row is missing.
2. **Local-core independence enforced** — the seven local-core capabilities
   (`editor_core`, `search`, `local_git`, `tasks`, `debugging`, `local_indexing`,
   `local_safe_ai`) are classified as `open_local` and do not depend on a hidden
   managed prerequisite.
3. **Offboarding state disclosed** — every capability classified as
   `managed_hosted` or `enterprise_governed` carries an offboarding packet that
   names what remains local, what becomes unavailable, what is still exportable,
   and which managed records persist for policy or billing reasons.
4. **Usage-export schema version current** — every usage-export packet carries
   the current schema version, a retention label, and a clear machine-readable
   partiality marker.

## Required behavior

`validate_open_vs_paid_boundary_page` rejects a page when its `defects` list is
non-empty.

`audit_open_vs_paid_boundary_page` runs the combined check and returns a typed
`Vec<OpenVsPaidBoundaryDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A `LocalCoreRequiresManagedPrerequisite` defect when a local-core capability is
  classified as `managed_hosted` or `enterprise_governed`. The function returns
  immediately with this defect and skips all other checks.

A surface inconsistency, missing offboarding disclosure, missing usage-export
disclosure, stale schema version, or silent entitlement loss narrows to `Beta`
(not `Withdrawn`) because it prevents claim verification but does not represent
a hard security guardrail.

## Capability family vocabulary

| Family token | Boundary class | Offboarding required | Usage export required |
|---|---|---|---|
| `editor_core` | `open_local` | No | No |
| `search` | `open_local` | No | No |
| `local_git` | `open_local` | No | No |
| `tasks` | `open_local` | No | No |
| `debugging` | `open_local` | No | No |
| `local_indexing` | `open_local` | No | No |
| `local_safe_ai` | `open_local` | No | No |
| `collaboration` | `managed_hosted` | Yes | Yes |
| `managed_ai_routing` | `managed_hosted` | Yes | Yes |
| `admin_dashboard` | `enterprise_governed` | Yes | Yes |
| `policy_enforcement` | `enterprise_governed` | Yes | Yes |
| `extensions_marketplace` | `managed_hosted` | Yes | Yes |
| `support_exports` | `open_local` | No | No |
| `usage_analytics` | `managed_hosted` | Yes | Yes |
| `backup_restore` | `enterprise_governed` | Yes | Yes |

## Offboarding state vocabulary

The offboarding packet uses these closed-vocabulary outcome states:

- `local_only` — Data remains on-device only; no managed copy exists.
- `managed_copy` — A managed copy exists; the local copy remains.
- `queued` — Export or deletion is queued and not yet completed.
- `partial` — Only a partial export or partial deletion is available.
- `blocked_by_hold` — Action is blocked by an administrative or legal hold.
- `policy_retained` — Record is retained for policy, compliance, or audit reasons.
- `outside_platform_scope` — Record is outside the platform scope and not managed.
- `completed` — Offboarding action for this record is fully completed.

## Grace-window states

- `active` — Grace window is active; exports and local continuity are preserved.
- `expired` — Grace window has expired; managed capabilities are paused.
- `export_only` — Only export routes remain available; managed features are paused.
- `degraded` — Local core is preserved but managed features are degraded.

## Usage-export availability

- `full` — Full export is available with all records.
- `partial` — Partial export is available; some records are excluded.
- `unavailable` — Export is unavailable for this record class.

## Retention posture

- `user_owned_immediate` — User-owned data available for immediate export.
- `tenant_retained_policy` — Retained by the tenant for policy or compliance reasons.
- `billing_retained` — Retained for billing or metering reasons.
- `grace_window_exportable` — Available for export during a grace window after cancellation.
- `expired_unavailable` — Grace window expired; export no longer available.

## Boundary

The following material stays outside this packet's support boundary:

- Raw entitlement values, raw seat identifiers, raw quota counters.
- Raw tenant configuration, raw billing records.
- Raw policy rule bodies, raw exception justification text.
- Raw extension or model artifact binaries.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, a count, or a schema-version integer.

## Truth source

| Slice | Canonical source |
|-------|-----------------|
| Boundary manifest | `aureline_policy::finalize_open_vs_paid_boundary_and_offboarding` |
| Usage-export schema | `schemas/policy/usage-export.schema.json` |
| Offboarding packets | `aureline_policy::finalize_open_vs_paid_boundary_and_offboarding` |
| Local-core independence | `docs/governance/open_paid_boundary_and_antilockin_matrix.md` |
| Deployment profile truth | `aureline_policy::stabilize_deployment_and_residency_truth` |
| Deprovision preserves | `aureline_auth::finalize_no_account_local_use_proof_deprovision_preserves` |
| Artifact evidence | `artifacts/policy/m4/finalize-open-vs-paid-boundary-and-offboarding.md` |
