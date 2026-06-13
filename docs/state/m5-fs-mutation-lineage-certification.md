# M5 Filesystem and Mutation-Lineage Certification

This document is the reviewer-facing contract for the canonical M5 filesystem,
watch/save, mutation-lineage, state-class-recovery, and deferred-intent
certification packet.

## Scope

The packet lives in `aureline-support`, but it is composed from the existing
state and VFS truth sources:

- `artifacts/state/filesystem_mutation_lineage_matrix.json`
- `artifacts/state/filesystem_truth_review.json`
- `artifacts/state/m5_mutation_lineage.json`
- `artifacts/state/state_class_recovery.json`

The certification does not replace those packets. It answers the publication
question above them: which claimed M5 rows may still publish a broad continuity
claim, and which must narrow to `limited`, `stale`, or
`reconcile_required`.

## Publication rules

- `qualified` is reserved for rows that keep canonical filesystem identity,
  live watch truth, a writable canonical save target, attributable mutation
  lineage, and the required scoped repair path.
- `limited` is used for inspect-only, imported, save-as, compare-only, or
  generator-owned rows that must not inherit an ordinary file claim.
- `stale` is used for rows whose watch or refresh truth is explicitly weaker
  than a fully live editable file, even if their lineage remains attributable.
- `reconcile_required` is used for provider-backed or deferred-write rows that
  remain usable only with explicit reconnect or drift review.

## Downstream consumers

The certification is canonical only if downstream consumers reuse it by
reference. This row is bound to:

- `crates/aureline-shell/src/help/filesystem_continuity.rs`
- `crates/aureline-shell/src/diagnostics/filesystem_continuity.rs`
- `crates/aureline-support/src/m5_fs_mutation_lineage_certification/mod.rs::support_bundle_projection`
- `crates/aureline-shell/src/release_center/filesystem_continuity.rs`

If any consumer stops ingesting this packet, broad filesystem-continuity claims
must narrow.

## Regeneration

```bash
cargo run -p aureline-support --example dump_m5_fs_mutation_lineage_certification -- canonical > artifacts/state/m5-fs-mutation-lineage-certification.json
cargo run -p aureline-support --example dump_m5_fs_mutation_lineage_certification -- missing_recovery_linkage > fixtures/state/m5-fs-mutation-lineage-certification/missing_recovery_linkage.json
```

The reviewer summary at `artifacts/state/m5-fs-mutation-lineage-certification.md`
is maintained alongside the packet and fixtures.
