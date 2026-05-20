# Portable-State and Restore-Provenance Conformance (M3 Beta)

This document freezes the certifiable beta contract for portable-state packages
and restore provenance: what is preserved, what is excluded, and how a downgrade
or drift is surfaced. It composes with, and does not replace, the workspace
serialization and portable-state beta contract
([`serialization_and_portable_state_beta.md`](serialization_and_portable_state_beta.md)).

Where the serialization contract defines the package and card boundary, this
document defines the **conformance proof** that the boundary holds across
export, import, schema migration, redaction, and missing-dependency paths — and
the published evidence a beta continuity claim rests on.

## Machine-readable boundaries

- Conformance corpus:
  [`/fixtures/workspace/m3/portable_state_and_restore_conformance/`](../../../fixtures/workspace/m3/portable_state_and_restore_conformance/)
- Restore-provenance schema:
  [`/schemas/workspace/restore_provenance.schema.json`](../../../schemas/workspace/restore_provenance.schema.json)
- Portable-state package schema:
  [`/schemas/workspace/portable_state_package.schema.json`](../../../schemas/workspace/portable_state_package.schema.json)
- Compatibility report schema:
  [`/schemas/workspace/portable_state_compat_report.schema.json`](../../../schemas/workspace/portable_state_compat_report.schema.json)
- Runtime model:
  [`/crates/aureline-workspace/src/serialization/`](../../../crates/aureline-workspace/src/serialization/)
- Conformance harness:
  [`/crates/aureline-qe/src/portable_state_restore/`](../../../crates/aureline-qe/src/portable_state_restore/)

## Published evidence

- Compatibility matrix:
  [`/artifacts/compat/m3/portable_state_restore_matrix.md`](../../../artifacts/compat/m3/portable_state_restore_matrix.md)
- Machine-readable report:
  [`/artifacts/compat/m3/portable_state_restore_report.json`](../../../artifacts/compat/m3/portable_state_restore_report.json)
- Restore-provenance support packet:
  [`/artifacts/support/m3/restore_provenance_examples/`](../../../artifacts/support/m3/restore_provenance_examples/)

CI replays the corpus with
`cargo test -p aureline-qe --test portable_state_restore_conformance` and
publishes the matrix and the restore-provenance packet for the claimed beta
matrix. The conformance suite asserts the matrix and JSON report cover every
drill, so the published evidence cannot drift from the corpus.

## Restore classes (controlled vocabulary)

The restore class is always one of the controlled fidelity labels, and docs,
help, and claim-manifest language MUST quote the label exactly as the runtime
renders it (`WorkspaceRestoreFidelity::display_label`):

| Class | Label | Meaning |
| --- | --- | --- |
| `exact_restore` | Exact restore | Nothing downgraded; no placeholder, translation, or review. |
| `compatible_restore` | Compatible restore | Meaning preserved through a declared compatibility path. |
| `layout_only` | Layout only | Layout and context restored without live authority. |
| `recovered_drafts` | Recovered drafts | Dirty drafts recovered for compare and explicit save. |
| `evidence_only` | Evidence only | Only evidence, transcripts, snapshots, and provenance survived. |

A package or restore row that is **Compatible restore**, **Layout only**, or
held for **Manual review** (Retest pending) must never be described as an exact,
lossless restore. Downgrade language is automatic: it is derived from the same
fidelity label the runtime renders, not authored separately.

## What is preserved, excluded, and surfaced

**Preserved.** The restore-provenance card reconstructs the source event, the
producer build, the schema outcome, the fidelity result, the missing
dependencies (each as a placeholder that keeps its stable pane id, role, surface
class, last-known provenance label, and safe actions), and the diagnostics /
support-export / crash-recovery refs that keep the same card visible everywhere.

**Excluded.** Raw secrets, delegated approvals, provider-issued capability /
approval tickets, delegated credentials, live authority handles, machine-unique
trust anchors, raw paths, hostnames, command lines, logs, source content, and
provider payload bodies are excluded by default and named as intentional
exclusions. Off-screen geometry from monitor-topology drift stays a best-effort
hint, never authoritative truth. No path serializes any of these as authority.

**Surfaced.** When a dependency is missing — extension, remote, provider, live
authority, monitor topology, or a missing schema equivalence — the pane slot
stays in the layout and reopens as a placeholder. A live surface never reruns or
reacquires authority without explicit user action.

## Schema migration and meaning

An older portable-state package is migrated forward through
`WorkspacePortableStatePackage::from_alpha_package`. The migration MUST NOT:

- silently widen meaning (an outcome that was Compatible / Layout only must not
  become Exact);
- rehydrate live authority (the live-authority handle stays a named exclusion);
- suppress a fidelity downgrade (the downgrade label stays visible);
- export machine-local hints as carried authority.

When migration or import changes meaning — including a missing schema
equivalence — the outcome is held for **Manual review** and the prior artifact
stays available for **compare** and **export** so a human can decide. The
`manual_review.schema_drift_preserves_prior` drill and the
`negative.manual_review_missing_compare_export` drill pin both halves of this
rule.

## Missing-surface and drift conditions

The corpus verifies the remembered-state inspector, the portable-state export /
import review sheet, the restore-provenance card, and the missing-surface
placeholder under: missing extension, remote unavailable, missing provider,
monitor-topology drift, policy-blocked import, non-reentrant live surface, a
missing schema equivalence, and side-by-side channel / version drift. Each keeps
the pane slot, names the dependency, and offers safe actions instead of
disappearing.

## Compare / export / clear safety

Compare and export are read-only or build-only and preserve unrelated workspace
content. Clear is scoped to selected remembered-state metadata only; it does not
delete source files, workspace manifests, credential-store entries, unrelated
caches, or broader workspace content. Every package and restore packet passes
redaction review before bytes leave the machine or an imported package is
applied.
