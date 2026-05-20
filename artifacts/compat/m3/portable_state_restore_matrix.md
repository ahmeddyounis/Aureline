# Portable-state and restore-provenance compatibility matrix

This matrix is the published, claimed-beta compatibility report for workspace
portable-state packages and restore provenance. It is generated from, and gated
against, the conformance corpus
[`fixtures/workspace/m3/portable_state_and_restore_conformance/`](../../../fixtures/workspace/m3/portable_state_and_restore_conformance/)
and the machine-readable report
[`portable_state_restore_report.json`](portable_state_restore_report.json)
(schema:
[`schemas/workspace/portable_state_compat_report.schema.json`](../../../schemas/workspace/portable_state_compat_report.schema.json)).

Every row is replayed by
`cargo test -p aureline-qe --test portable_state_restore_conformance`. A row may
not be edited here without a matching corpus drill: the conformance suite asserts
this matrix and the JSON report cover every drill id.

Corpus id: `workspace.portable_state_and_restore_conformance.beta`

## Restore classes across export / import / migration / redaction / missing-dependency

| Drill | Restore class (downgrade label) | Source event | Schema outcome | Missing-surface dependencies | Prior artifact preserved | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `exact.manual_export_round_trip` | Exact restore | manual_export | exact | — | n/a | pass |
| `compatible.backup_schema_migration` | Compatible restore | backup | compatible | — | yes | pass |
| `layout_only.import_missing_extension_and_remote` | Layout only | import | layout_only | missing_extension, missing_remote | yes | pass |
| `recovered_drafts.crash_auto_checkpoint` | Recovered drafts | auto_checkpoint | compatible | — | yes | pass |
| `evidence_only.sync_non_reentrant` | Evidence only | sync | layout_only | non_reentrant_live_surface | yes | pass |
| `manual_review.schema_drift_preserves_prior` | Compatible restore | import | manual_review | schema_equivalence_missing | required | pass |
| `layout_only.monitor_topology_drift` | Layout only | auto_checkpoint | layout_only | display_topology_mismatch | yes | pass |
| `layout_only.policy_blocked_import` | Layout only | import | layout_only | revoked_permission | yes | pass |
| `evidence_only.channel_version_drift` | Evidence only | sync | manual_review | missing_provider | required | pass |
| `migration.alpha_to_beta_layout_only` | Layout only | import | compatible | missing_extension, missing_remote, non_reentrant_live_surface | yes | pass |

## Schema-migration drill (alpha → beta)

`migration.alpha_to_beta_layout_only` migrates an older alpha portable-state
package forward through
`WorkspacePortableStatePackage::from_alpha_package` and proves the migration:

- keeps the four state layers separated (workspace authority, window topology,
  profile defaults, machine-local hints);
- keeps machine-local hints **excluded** from export (never carried authority);
- keeps path and host redaction available;
- keeps the live-authority handle a **named exclusion** — it is never
  rehydrated as live authority;
- keeps the fidelity downgrade visible (Layout only) rather than silently
  widening to Exact;
- keeps the prior artifact available for compare and export;
- still projects the remembered-state inspector and the export / import review
  sheets without error.

## Negative drills (must be rejected before a beta row hardens)

| Drill | Rejected because | Failure substring | Status |
| --- | --- | --- | --- |
| `negative.exact_restore_carries_placeholder` | Exact restore cannot carry a missing-surface placeholder | `carried placeholders` | pass |
| `negative.manual_review_missing_compare_export` | Manual review must keep the prior artifact for compare/export | `restore_card.compare_ref` | pass |
| `negative.placeholder_missing_safe_action` | A placeholder must offer at least one safe recovery action | `has no action` | pass |
| `negative.duplicate_placeholder_id` | Placeholder ids must be unique so the layout is unambiguous | `duplicate placeholder` | pass |

## Redaction review

No row serializes raw secrets, delegated approvals, provider-issued capability
tickets, or off-screen geometry as authoritative truth. Secrets, delegated
approvals, approval / capability tickets, live authority handles, and
machine-unique trust anchors stay **named exclusions**; off-screen geometry from
monitor-topology drift stays a best-effort hint. The runner additionally scans
every fixture for forbidden raw-export tokens before validation.

## Downgrade language

Docs, help, and claim-manifest language must quote the controlled downgrade
label exactly as the runtime renders it (`WorkspaceRestoreFidelity::display_label`):
**Exact restore**, **Compatible restore**, **Layout only**, **Recovered
drafts**, **Evidence only**. A package or restore row that is Compatible restore,
Layout only, or held for manual review (Retest pending) must not be described as
an exact, lossless restore.
