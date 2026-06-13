# M5 Filesystem and Mutation-Lineage Certification Fixtures

This directory holds the checked fixture variants for the canonical M5
filesystem/watch/save/mutation-lineage/deferred-intent certification packet.

Files:

- `packet.json` — canonical checked packet
- `missing_recovery_linkage.json` — rows that otherwise qualified narrow when
  their scoped recovery linkage is removed
- `manifest.yaml` — fixture inventory

Regenerate with:

```bash
cargo run -p aureline-support --example dump_m5_fs_mutation_lineage_certification -- canonical > fixtures/state/m5-fs-mutation-lineage-certification/packet.json
cargo run -p aureline-support --example dump_m5_fs_mutation_lineage_certification -- missing_recovery_linkage > fixtures/state/m5-fs-mutation-lineage-certification/missing_recovery_linkage.json
```
