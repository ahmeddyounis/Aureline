# Portable-state and restore-provenance conformance corpus

This corpus is the conformance, interoperability, and failure / recovery drill
harness for the M3 portable-state and restore-provenance beta boundary owned by
`aureline-workspace::serialization` (`WorkspaceRestoreProvenanceCard` and
`WorkspacePortableStatePackage`, plus the alpha → beta migration
`WorkspacePortableStatePackage::from_alpha_package`).

It converts the continuity UX promise into a regression-gated proof system: each
drill pins the restore truth a claimed beta continuity row must reproduce — the
restore class and its controlled downgrade label, the source event, the schema
outcome, the missing-surface dependencies that reopen as placeholders, whether
high-risk classes stay named exclusions, and whether the prior artifact stays
available for compare and export when migration changes meaning.

Every drill is loaded by the conformance harness at
[`crates/aureline-qe/src/portable_state_restore/`](../../../../crates/aureline-qe/src/portable_state_restore/)
and replayed by
`cargo test -p aureline-qe --test portable_state_restore_conformance`.

## Single source of truth

`manifest.json` is authoritative. Positive drills MUST parse, validate, and
match **every** `expected_*` field in the manifest. Negative drills MUST FAIL
validation with an error whose message contains `expected_failure_substring`.
The fixtures carry only the scenario records and a `__fixture__` prelude — they
do **not** restate the expectations, so there is exactly one place to read and
audit the pinned truth.

Boundary schemas and runtime model:

- [`/schemas/workspace/restore_provenance.schema.json`](../../../../schemas/workspace/restore_provenance.schema.json)
- [`/schemas/workspace/portable_state_package.schema.json`](../../../../schemas/workspace/portable_state_package.schema.json)
- [`/schemas/workspace/portable_state_alpha.schema.json`](../../../../schemas/workspace/portable_state_alpha.schema.json)
- [`/crates/aureline-workspace/src/serialization/`](../../../../crates/aureline-workspace/src/serialization/)

Beta contract:
[`docs/workspace/m3/portable_state_and_restore_conformance.md`](../../../../docs/workspace/m3/portable_state_and_restore_conformance.md).
Published report:
[`artifacts/compat/m3/portable_state_restore_matrix.md`](../../../../artifacts/compat/m3/portable_state_restore_matrix.md)
(+ machine-readable
[`portable_state_restore_report.json`](../../../../artifacts/compat/m3/portable_state_restore_report.json)).
Support packet:
[`artifacts/support/m3/restore_provenance_examples/`](../../../../artifacts/support/m3/restore_provenance_examples/).

## Coverage axes

| Axis | Drill id |
| --- | --- |
| Exact restore — manual export round-trip | `exact.manual_export_round_trip` |
| Compatible restore — backup schema migration | `compatible.backup_schema_migration` |
| Layout only — import, missing extension + remote | `layout_only.import_missing_extension_and_remote` |
| Recovered drafts — crash auto-checkpoint | `recovered_drafts.crash_auto_checkpoint` |
| Evidence only — sync, non-reentrant live surface | `evidence_only.sync_non_reentrant` |
| Manual review — schema drift preserves prior artifact | `manual_review.schema_drift_preserves_prior` |
| Layout only — monitor-topology drift | `layout_only.monitor_topology_drift` |
| Layout only — policy-blocked import | `layout_only.policy_blocked_import` |
| Evidence only — side-by-side channel/version drift | `evidence_only.channel_version_drift` |
| Schema migration — alpha → beta | `migration.alpha_to_beta_layout_only` |
| Negative — exact restore carries a placeholder | `negative.exact_restore_carries_placeholder` |
| Negative — manual review drops compare/export | `negative.manual_review_missing_compare_export` |
| Negative — placeholder offers no safe action | `negative.placeholder_missing_safe_action` |
| Negative — duplicate placeholder id | `negative.duplicate_placeholder_id` |

## Transverse invariants

The conformance suite also pins, across the whole positive set:

- every restore class (`exact_restore`, `compatible_restore`, `layout_only`,
  `recovered_drafts`, `evidence_only`) keeps a drill;
- every source event (`manual_export`, `backup`, `sync`, `import`,
  `auto_checkpoint`) keeps a drill;
- the missing-surface dependencies (`missing_extension`, `missing_remote`,
  `missing_provider`, `revoked_permission`, `non_reentrant_live_surface`,
  `display_topology_mismatch`, `schema_equivalence_missing`) all keep a drill;
- a meaning-changing schema outcome (`manual_review`) keeps the prior artifact
  available for compare and export;
- the alpha → beta migration drill proves layers stay separated, machine-local
  hints stay excluded, path/host redaction stays available, live authority is
  not rehydrated, and the inspector / export / import surfaces still project;
- the published compatibility report and the restore-provenance support packet
  cover every drill id, so they cannot drift from the corpus.

## Redaction guarantees

Every fixture is metadata-safe: only opaque refs and typed labels cross the
boundary. Raw secrets, delegated approvals, provider-issued capability tickets,
delegated credentials, live authority handles, machine-unique trust anchors, raw
paths, hostnames, command lines, logs, source content, provider payload bodies,
and off-screen geometry as authoritative truth never appear. The runner scans
each fixture for forbidden raw-export tokens before validation. Removing any
positive or negative drill without a replacement is a breaking contract change
for the `workspace.portable_state_and_restore_conformance.beta` corpus.
