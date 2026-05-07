# Data ownership, export-risk, and redaction taxonomy contract

This document freezes one cross-surface taxonomy contract for **who
owns a data object** and **how risky it is to export or preview that
object**. It exists so migration, support export, audit, retention,
and delete flows do not reclassify the same bytes differently per
surface.

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or Design System, those documents win and this document
plus its companion artifacts MUST update in the same change. Where a
downstream exporter, support-bundle composer, sync payload, crash
packet, collaboration archive, AI evidence packet, or managed-service
record mints new class labels for ownership or export risk, this
document wins and the surface is non-conforming.

Companion artifacts:

- [`/artifacts/state/data_classes.yaml`](../../artifacts/state/data_classes.yaml)
  — machine-readable register of ownership classes, export-risk
  classes, and worked surface mappings.
- [`/schemas/support/data_risk_class.schema.json`](../../schemas/support/data_risk_class.schema.json)
  — language-neutral export-risk enum for boundary schemas.
- [`/fixtures/state/data_class_cases/`](../../fixtures/state/data_class_cases/)
  — worked examples showing why every object needs both an ownership
  class and an export-risk class.

This contract composes over existing authoritative vocabularies rather
than replacing them:

- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md)
  and
  [`/artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml)
  — the persisted state-object inventory, including `object_category`,
  `authority`, `backup_before_migrate_rule`, and corruption posture.
- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
  — authority-owner classes (`user_authored_durable_truth`, etc),
  portability classes, and ADR-0007 `redaction_class` vocabulary.
- [`/docs/support/support_bundle_preview_contract.md`](../support/support_bundle_preview_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  — support-bundle `data_class` tokens and export/redaction preview
  semantics.
- [`/docs/governance/storage_and_retention_vocabulary.md`](../governance/storage_and_retention_vocabulary.md)
  and
  [`/artifacts/governance/storage_modes.yaml`](../../artifacts/governance/storage_modes.yaml)
  — storage-mode and retention-mode axes that describe where bytes
  live and how long they last.
- [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — managed retention/export/delete/hold posture. This taxonomy does
  not enforce retention windows.

## One object, two orthogonal axes

**Ownership** answers “who is authoritative for this object and how is
it treated under migration/delete?” **Export risk** answers “what
controls must apply before this representation may be previewed or
exported off-device or across principals?”

These axes MUST NOT be collapsed. A durable object may be high-risk to
export (credentials). A derived object may also be high-risk to export
(raw traces, raw dumps, full transcripts). Tooling and docs must carry
both axes so “durable” is never mistaken for “safe.”

## 1. Ownership classes (data ownership)

Every durable data object that can be migrated, deleted, exported,
audited, or retained MUST resolve to exactly one ownership class from
the closed set below. For persisted state objects, this is the
`object_category` column in `state_objects.yaml`.

Closed vocabulary:

- `user_authored_durable_state`
- `workspace_authored_durable_state`
- `derived_disposable_state`
- `recovery_journal_state`
- `audit_trust_security_state`
- `evidence_support_state`

### Ownership-class semantics (default implications)

The implications below are defaults. Concrete objects may narrow
behavior (stricter export, stronger backups) but may not widen it
without an explicit decision row in the relevant governing contract.

| Ownership class | Authoritative owner (default) | Rebuildability (default) | Backup-before-migrate (default) | Delete/export implications (default) |
|---|---|---|---|---|
| `user_authored_durable_state` | user | not rebuildable; preserve truth | required before destructive migration | export is user right; delete requires preview + rollback where feasible |
| `workspace_authored_durable_state` | workspace / repo scope | not rebuildable; preserve truth | required before destructive migration | export is workspace-scoped; delete requires workspace-scoped review and preserves provenance |
| `derived_disposable_state` | system-derived | rebuildable | not applicable | export is optional and usually summary-only; delete/clear is allowed with truthful “rebuild” posture |
| `recovery_journal_state` | user (held on user’s behalf) | partially rebuildable; preserve until acknowledged | often required (journal can be last known good) | export is opt-in and redaction-heavy; delete/clear requires preview and must not strand recovery promises |
| `audit_trust_security_state` | admin / control / governance | not rebuildable | handled by authority | export is restricted and policy-bound; delete is distinct for local vs managed and may produce receipts |
| `evidence_support_state` | user locally, support/tenant when promoted | not rebuildable; policy-bounded evidence | required before destructive replace where evidence would be lost | export is always previewable-by-manifest; high-risk bodies require explicit opt-in or local-only retention |

## 2. Export-risk classes (data risk)

Every boundary object that can appear in a preview, export manifest, or
cross-surface packet MUST name exactly one export-risk class from the
closed set. These tokens describe the **representation that crosses
the boundary**, not the underlying raw material that may be referenced
by opaque ids.

Closed vocabulary (reused by support-bundle and recovery contracts):

- `metadata_only`
- `environment_adjacent`
- `code_adjacent`
- `high_risk`

### Export-risk semantics (required controls)

| Risk class | What it may contain | Preview/export requirements (minimum) |
|---|---|---|
| `metadata_only` | ids, versions, digests, counters, coarse labels | may be included by default; redaction still applies where required; manifests must stay truthful about omissions |
| `environment_adjacent` | toolchain/route/proxy/host posture summaries, environment fingerprints | may be included only after clear local preview; must avoid raw hostnames/paths/tokens; may promote to managed only under consent-recorded posture |
| `code_adjacent` | filenames, stack traces, bounded snippets, queries, command argument summaries | excluded or strongly redacted by default; requires item-level opt-in before leaving the device; must carry explicit redaction/omission notes |
| `high_risk` | secret-bearing material, raw dumps/cores, full transcripts, raw traces | prohibited or retained local-only by default; export requires a separate high-friction path plus explicit consent/policy gate; raw secret bytes remain forbidden |

## 3. Redaction is separate from risk

Export-risk class is **not** a redaction algorithm. Redaction is a
separate contract that defines what the exported representation looks
like at the field/value level.

Consumers MUST:

1. classify the object with an export-risk class; and
2. apply the relevant redaction contract for the boundary (for
   example ADR-0007 for settings/sync value previews, or the
   support-bundle redaction profile for support exports).

Risk class gates *whether* export/preview is permitted by default; the
redaction class states *how* the permitted bytes are transformed.

## 4. Cross-surface mapping (required)

Surfaces that emit any of the following objects MUST classify them
using this taxonomy and SHOULD cite the `data_classes.yaml` register
row ids in their own contracts:

- settings sync payloads (scope bundles, conflict packets, device
  records)
- crash reports (envelope, dump/core by reference, symbolication
  report)
- audit events (action receipts, denial receipts, managed audit rows)
- collaboration session metadata (session state, archive inventory,
  delete/export status)
- AI prompts/evidence (prompt/result caches, evidence packets, replay
  manifests, audit storage manifests)
- extension install events (install review cases, inventory manifests,
  quarantine receipts)
- support bundles (manifest, preview items, item materialization)
- managed-service records (record-class governed retention/export/delete
  flows)

The mapping rows in
[`/artifacts/state/data_classes.yaml`](../../artifacts/state/data_classes.yaml)
seed defaults for these surfaces and demonstrate how one logical
object can be durable while still unsafe to export.
