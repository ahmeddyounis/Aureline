# Quiet-hours-suppression bundle — evidence companion

Human-readable companion to
[`/fixtures/activity/m5-quiet-hours-suppression/canonical_bundle.json`](../../fixtures/activity/m5-quiet-hours-suppression/canonical_bundle.json)
and its boundary schema
[`/schemas/activity/m5-quiet-hours-suppression.schema.json`](../../schemas/activity/m5-quiet-hours-suppression.schema.json).
It gives reviewers the frozen surface, policy, signal, and decision tables without
reading the JSON. The contract narrative lives in
[`/docs/activity/m5-quiet-hours-suppression.md`](../../docs/activity/m5-quiet-hours-suppression.md).

- Bundle id: `m5-quiet-hours-suppression:bundle:0001`
- Record kind: `m5_quiet_hours_suppression_bundle`
- Binds back to: `m5-attention-routing:matrix:0001`
- Surfaces: 4 · Policies: 9 · Signals: 7 · Decisions: 63 · Ledger entries: 155 · Invariants: 16

## Governed surfaces

One policy governs all four surfaces. The in-app activity center always shows the
durable authoritative record; the out-of-window surfaces are mirrors governed by the
same policy.

| Surface | Privacy ceiling | Default redaction | Durable authoritative |
| --- | --- | --- | --- |
| `in_app_activity_center` | managed_sensitive | metadata_safe_default | yes |
| `os_native_notification` | summary_safe | summary_only | no |
| `browser_companion` | workspace_sensitive | redacted_payload | no |
| `mobile_companion` | summary_safe | summary_only | no |

## Suppression-policy corpus

Each policy exercises one suppression input (plus a clear baseline and a layered case),
so every suppression source is covered.

| Policy | Quiet-hours | DND | Presentation | Lock-screen | Admin | Endpoint |
| --- | --- | --- | --- | --- | --- | --- |
| `clear` | off | off | off | unlocked | unmanaged | compliant |
| `quiet_hours` | active | off | off | unlocked | unmanaged | compliant |
| `do_not_disturb` | off | on | off | unlocked | unmanaged | compliant |
| `presentation` | off | off | presenting | unlocked | unmanaged | compliant |
| `lock_screen` | off | off | off | locked | unmanaged | compliant |
| `managed_restricted` | off | off | off | unlocked | managed_restricted | compliant |
| `managed_locked` | off | off | off | unlocked | managed_locked | compliant |
| `managed_endpoint_noncompliant` | off | off | off | unlocked | managed_default | non_compliant |
| `quiet_hours_locked_managed` | active | off | off | locked | managed_restricted | compliant |

## Signal corpus

A representative set of attention signals across subsystems, severities, privacy
classes, and named consequences.

| Signal | Subsystem | Severity | Privacy | Consequence | High-importance |
| --- | --- | --- | --- | --- | --- |
| `task.completed` | task_runner | minor_success | summary_safe | none | no |
| `support.export_ready` | support | informational | summary_safe | none | no |
| `collab.review_requested` | collaboration | handoff_actionable | workspace_sensitive | none | yes |
| `ai.awaiting_approval` | ai | handoff_actionable | workspace_sensitive | approval_required | yes |
| `route.policy_warning` | managed_policy | handoff_actionable | managed_sensitive | route_warning | yes |
| `trust.provider_changed` | sync | handoff_actionable | security_critical | trust_change | yes |
| `security.credential_revoked` | security | security_advisory | security_critical | security_advisory | yes |

## Worked decisions — OS surface under each policy

The OS notification disposition for each signal under each policy. The in-app activity
center is always `shown`; companion surfaces follow the same policy (companions are also
withheld under `managed_locked`).

| Signal \\ Policy | clear | quiet_hours | dnd | presentation | lock_screen | restricted | locked | endpoint | qh+lock+mgd |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `task.completed` | shown | withheld | withheld | withheld | shown | downgraded | shown | withheld | withheld |
| `support.export_ready` | shown | withheld | withheld | withheld | shown | downgraded | shown | withheld | withheld |
| `collab.review_requested` | shown | withheld | withheld | withheld | downgraded | downgraded | shown | withheld | withheld |
| `ai.awaiting_approval` | shown | downgraded | downgraded | downgraded | downgraded | downgraded | shown | withheld | downgraded |
| `route.policy_warning` | shown | downgraded | downgraded | downgraded | downgraded | downgraded | shown | withheld | downgraded |
| `trust.provider_changed` | shown | downgraded | downgraded | downgraded | downgraded | downgraded | shown | withheld | downgraded |
| `security.credential_revoked` | shown | downgraded | downgraded | downgraded | downgraded | downgraded | shown | withheld | downgraded |

Note how, under `quiet_hours`, `collab.review_requested` is **withheld** (high-importance
but it names no consequence) while every named high-importance signal —
`ai.awaiting_approval`, `route.policy_warning`, and `trust.provider_changed` — is
**downgraded** because it escapes with a redacted summary that names its scope and
consequence. A security advisory always escapes; `managed_locked` withholds the two
companion surfaces (but not the OS notification); and `managed_endpoint_noncompliant`
withholds every out-of-window surface. Every delivered out-of-window outcome keeps a
redaction at least as strong as the surface's normal treatment, so a downgrade never
widens privacy.

## Computed invariants (all hold)

| Invariant |
| --- |
| `suppression.parity_one_policy_all_surfaces` |
| `suppression.in_app_durable_record_always` |
| `suppression.explains_every_surface` |
| `suppression.three_dispositions_exercised` |
| `suppression.every_source_exercised` |
| `suppression.security_never_silenced` |
| `suppression.high_importance_escapes_only_when_named` |
| `suppression.escape_names_scope_and_consequence` |
| `suppression.withheld_keeps_durable_record_and_reopen` |
| `suppression.separate_from_audit_history` |
| `suppression.audit_trail_for_blocked_high_importance` |
| `suppression.downgrade_never_widens_privacy` |
| `suppression.state_is_matrix_suppression_state` |
| `suppression.decisions_reproducible` |
| `suppression.matrix_bound` |
| `suppression.support_export_safe` |

The freeze gate `crates/aureline-activity/tests/m5_quiet_hours_suppression.rs` rebuilds
the bundle in code and asserts it equals this fixture byte-for-byte; an inconsistent
edit flips an invariant or fails the round-trip.
