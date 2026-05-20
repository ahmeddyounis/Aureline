# Restore-provenance support packet examples

Support-facing examples for the restore-provenance card that ships in
diagnostics, support exports, and crash-recovery views after a portable-state
import or remembered-state restore. Each example shows what a support or release
reviewer actually sees — and what they never see — when they read a restore
packet.

The canonical record is the `WorkspaceRestoreProvenanceCard` projected by
[`/crates/aureline-workspace/src/serialization/`](../../../../crates/aureline-workspace/src/serialization/),
with the boundary schema in
[`/schemas/workspace/restore_provenance.schema.json`](../../../../schemas/workspace/restore_provenance.schema.json).

The drills these examples are drawn from live in the conformance corpus
[`/fixtures/workspace/m3/portable_state_and_restore_conformance/`](../../../../fixtures/workspace/m3/portable_state_and_restore_conformance/),
and the published compatibility report is
[`/artifacts/compat/m3/portable_state_restore_matrix.md`](../../../compat/m3/portable_state_restore_matrix.md).

## What every restore packet reconstructs — and what it never carries

Each packet reconstructs, from metadata-class tokens and opaque refs only:

- the **source event** (`auto_checkpoint`, `manual_export`, `backup`, `sync`,
  `import`);
- the **producer build** ref that wrote the package;
- the **fidelity result** and its controlled downgrade label (Exact restore,
  Compatible restore, Layout only, Recovered drafts, Evidence only);
- the **schema outcome** (exact / compatible / layout only / manual review);
- the **missing dependencies** that reopened as placeholders, each with a
  preserved stable pane id, last-known provenance label, and safe actions;
- the **redaction class** of every intentional exclusion;
- the diagnostics / support-export / crash-recovery refs that keep the same
  card visible everywhere, and the compare / export refs to the prior artifact.

A restore packet **never** carries: raw secrets, delegated approvals,
provider-issued capability tickets, delegated credentials, live authority
handles, machine-unique trust anchors, raw paths, hostnames, command lines,
logs, source content, provider payload bodies, or off-screen geometry as
authoritative truth. These appear only as **named exclusions** so a reviewer can
see what was deliberately left out.

## Examples

- [`layout_only_import.md`](layout_only_import.md) — an import that reopened a
  missing extension and a missing remote as placeholders (Layout only).
- [`manual_review_schema_drift.md`](manual_review_schema_drift.md) — an import
  whose schema equivalence was missing, held for manual review with the prior
  artifact preserved.
- [`evidence_only_channel_drift.md`](evidence_only_channel_drift.md) — a sync
  between side-by-side installs on different channels (Evidence only).
- [`migration_alpha_to_beta.md`](migration_alpha_to_beta.md) — an older alpha
  package migrated forward without rehydrating live authority or hiding the
  downgrade.
