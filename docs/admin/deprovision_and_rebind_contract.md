# Deprovision, device rebind, and identity-loss handoff contract

This contract freezes how Aureline explains **identity lifecycle actions**
that remove or rebind managed identity without stranding local work. The
goal is to make enterprise identity actions (seat deprovision, seat
transfer, forced sign-out, account recovery completion, and device
rebind) legible across desktop UI, CLI/headless exports, support packets,
self-hosted admin planes, and managed convenience deployments.

Local-first posture is not negotiable: identity loss MAY revoke managed
capabilities, but it MUST NOT imply that local work vanished, was
silently uploaded elsewhere, or became unrecoverable without an online
console.

## Companion artifacts

- [`/schemas/admin/deprovision_handoff.schema.json`](../../schemas/admin/deprovision_handoff.schema.json)
  — boundary schema for `deprovision_handoff_record`, the machine-readable
  handoff packet for identity-loss transitions.
- [`/schemas/admin/device_rebind_event.schema.json`](../../schemas/admin/device_rebind_event.schema.json)
  — boundary schema for `device_rebind_event_record`, the stable audit
  event emitted when a device rebind is completed and referenced by
  handoff and export surfaces.
- [`/fixtures/admin/identity_handoff_cases/`](../../fixtures/admin/identity_handoff_cases/)
  — worked YAML cases covering deprovision preserving local work, account
  recovery completion with device rebind audit, and admin-forced sign-out
  with export-before-close guidance.

Related upstream contracts this one composes over:

- [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../auth/managed_auth_and_session_continuity_contract.md)
  and [`/schemas/auth/managed_session_state.schema.json`](../../schemas/auth/managed_session_state.schema.json)
  — managed-session state, reauth requirements, forced sign-out posture,
  and local edit/save/undo/export continuity rules.
- [`/docs/identity/offline_entitlement_and_policy_seed.md`](../identity/offline_entitlement_and_policy_seed.md)
  and [`/schemas/identity/entitlement_snapshot.schema.json`](../../schemas/identity/entitlement_snapshot.schema.json)
  — signed entitlement snapshots and offline tolerance semantics.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  — signed policy-bundle cache and last-known-good behavior.
- [`/docs/admin/org_admin_seat_and_fleet_contract.md`](./org_admin_seat_and_fleet_contract.md)
  and [`/schemas/admin/seat_lifecycle_row.schema.json`](../../schemas/admin/seat_lifecycle_row.schema.json)
  — seat lifecycle transitions (transfer/deprovision) and `local_continuity`
  assertions consumed by admin and offboarding surfaces.
- [`/docs/admin/admin_audit_export_contract.md`](./admin_audit_export_contract.md)
  and [`/schemas/admin/admin_audit_export.schema.json`](../../schemas/admin/admin_audit_export.schema.json)
  — tenant-scoped audit export packets and vendor-console independence.
- [`/docs/reliability/local_history_contract.md`](../reliability/local_history_contract.md)
  and [`/docs/reliability/autosave_journal_and_guided_replay_contract.md`](../reliability/autosave_journal_and_guided_replay_contract.md)
  — local-history and dirty-buffer/autosave preservation rules.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  — support export availability and redaction boundary.

Normative product sources for this contract are the managed-auth and
recovery-preserves-local-work rules, offline entitlement behavior, and
sign-out/deprovision UX guarantees in `.t2/docs/`. If this document
disagrees with those sources, the `.t2/docs/` sources win and this
contract plus companion artifacts must be updated in the same change.

## Scope

Frozen at this revision:

- one **handoff packet** shape for identity-loss transitions that answers,
  in one place, what changed, who initiated it, what remains local, what
  becomes unavailable, and what export or recovery path is still offered;
- explicit **local-work preservation rules** for:
  - dirty buffers and autosave journals;
  - local history (timeline/checkpoints);
  - support exports and admin evidence exports; and
  - offline entitlement and signed policy caches across identity loss;
- manual-fallback expectations when normal browser/IdP recovery is
  unavailable; and
- one **device rebind audit event** shape that agrees with the handoff
  packet on actor, scope, and work-preservation state.

Out of scope:

- implementing IdP integrations, SCIM provisioning, seat billing, or
  account-recovery backends; and
- defining vendor-console-only features that hide the remaining local or
  offline recovery path.

## Core principles

1. **Local work is sovereign.** Identity-loss transitions MUST preserve
   local edit, save, undo/redo, and user-owned export, and MUST NOT
   corrupt autosave journals, local history, support exports, or cached
   policy/entitlement artifacts.
2. **No silent upload, no implied deletion.** Copy must never imply that
   local work was uploaded to a managed service, nor that it vanished
   because a managed account changed.
3. **Actor and scope are explicit.** Every handoff and audit record MUST
   name the actor class (user/admin/support/signed update), the tenant or
   org scope, and any affected seat/device/session refs as opaque ids.
4. **Export paths remain reachable.** Offboarding and support export MUST
   remain reachable even without a live managed seat. When export is
   blocked by policy or hold, the record must say so explicitly.
5. **Manual fallback is a first-class path.** When browser/IdP recovery is
   unavailable, the handoff must state which local-only actions and
   offline evidence remain available, and what the next safe action is.

## Identity-loss event classes

This contract covers five event classes. Each class produces a
`deprovision_handoff_record` and MAY also reference adjacent records
(`managed_session_state_record`, `seat_lifecycle_row_record`,
`admin_audit_export_record`, and `device_rebind_event_record`).

| Event class | Typical trigger | Managed impact | Must remain true locally |
|---|---|---|---|
| `deprovision` | seat removed, account deprovisioned | managed capabilities revoked | dirty buffers remain editable/saveable; autosave + local history remain intact; user-owned export remains available |
| `seat_transfer` | seat moves to a new owner | managed capabilities pause/resume under reauth | local work and export remain available on the source device |
| `forced_sign_out` | admin invalidates session, risk response | managed tokens purged; managed actions pause | local work remains available; export-before-close guidance is shown |
| `account_recovery_completion` | recovery finished and account is usable again | managed session scope may change | local caches are not corrupted; user can inspect what changed |
| `device_rebind` | device is rebound after recovery or risk event | device binding rotates; audit event emitted | local work remains available; rebind is auditable without raw identifiers |

## Local-work preservation rules

The handoff record MUST make local preservation explicit across identity
loss. The rules below are written as cross-surface invariants; individual
features may add narrower constraints, but may not weaken these defaults.

### Dirty buffers and autosave journals

- Identity loss MUST NOT close the workspace, discard dirty buffers, or
  bypass autosave journaling.
- Autosave-journal entries remain available for restore and support export
  unless the user explicitly discards them through a destructive,
  export-gated flow.
- A forced sign-out MUST show export-before-close guidance that assumes
  the user may be mid-task with unsaved edits.

### Local history (timeline/checkpoints)

- Local history is a local truth source and MUST NOT be cleared, reset,
  or silently narrowed because identity changed.
- Clear-history actions remain scoped and previewed; identity-loss events
  MUST NOT change the clear-history scope semantics.
- Local-history export and support projection remain available without
  browser sign-in.

### Support exports and evidence availability

- Support exports MUST remain available after identity loss, even when
  managed sign-in is blocked or a seat was removed.
- Handoff records MUST provide export-safe evidence refs (opaque ids) to
  related policy/entitlement snapshots, seat lifecycle rows, and audit
  packets so support can reconstruct the transition offline.

### Offline entitlement and policy caches

- Offline entitlement snapshots and signed policy bundles are preserved
  for inspection and support export; identity loss MUST NOT corrupt or
  silently delete them.
- Deprovisioning and forced sign-out MUST invalidate live managed tokens
  and delegated credentials promptly, but they MUST NOT turn local-only
  operations into sign-in-required operations.
- If a cached snapshot becomes stale/expired, the record must name the
  resulting posture explicitly (continue local-safe; deny new privileged
  managed actions until refresh succeeds).

## Manual fallback (IdP outage / degraded identity)

When the normal system-browser flow or IdP is unavailable, the record
MUST state:

- whether any managed recovery remains possible locally (device code or
  equivalent, credential-store unlock, admin-assisted recovery);
- which actions remain available without sign-in (continue local-only,
  export local work, export support bundle/admin evidence); and
- which evidence remains available offline (last-known-good policy bundle
  metadata, entitlement snapshot metadata, audit packet refs, and the
  handoff record itself).

## Device rebind audit event agreement

`device_rebind_event_record` exists to make device rebind reviewable
without requiring a hosted console. The event MUST agree with the handoff
record on:

- actor class and (redacted) actor ref;
- tenant scope plus affected seat/device refs; and
- whether local-work preservation/export guidance was shown.

The event MUST remain export-safe: no raw device fingerprints, raw user
identifiers, raw IdP payloads, raw tokens, or raw provider URLs.

