# Finalize Schema-Migration and Repair Flows for Workspace, Profile, and Persistent-State Artifacts — proof packet

Reviewer-facing proof packet for the schema-migration and repair
lineage lane: every governed artifact class
(`workspace_state_artifact`, `profile_artifact`,
`recent_work_registry`, `local_history_corpus`,
`restore_checkpoint`, `persistent_state_envelope`, plus the optional
`prebuild_cache_artifact` and `mutation_journal_artifact`) ships a row
bound to a stable artifact identity, a pinned source/target schema
version, a closed migration outcome, a non-empty
repair_transaction_id, and a non-empty finding_code. Every required
repair flow (`inspect_repair`, `rebuild_derived_store`,
`rehydrate_from_packet`, `quarantine_corrupt_artifact`,
`restore_from_checkpoint`, `manual_repair_handoff`) ships a row bound
to a closed repair outcome and an explicit-user-action or
terminal-no-rerun posture with a non-empty commit action id and
disclosure id whenever it mutates persistent state. A lossy migration
or lossy repair only ships behind an explicit disclosure ref; a
redacted row only ships behind an explicit redaction-disclosure ref.
A destructive migration or repair never fires without the controlled
inspection / repair hook table (`inspect_artifact`,
`compare_before_migration`, `preview_migration`, `preview_repair`,
`rollback_migration`, `rollback_repair`, `export_before_migration`,
`export_before_repair`) being reachable; a missing hook narrows the
record below Stable with a named reason. This packet is the
stable-line anchor for this lane; dashboards, docs, Help/About
surfaces, and support exports should ingest the typed sources below
rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/schema_migration_and_repair_lineage/`](../../../crates/aureline-workspace/src/schema_migration_and_repair_lineage/)
- Schema:
  [`/schemas/workspace/schema_migration_and_repair_lineage.schema.json`](../../../schemas/workspace/schema_migration_and_repair_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_schema_migration_and_repair_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_schema_migration_and_repair_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/schema_migration_and_repair_lineage/`](../../../fixtures/workspace/m4/schema_migration_and_repair_lineage/)
- Fixture generator:
  [`/crates/aureline-workspace/tests/schema_migration_and_repair_lineage_fixture_generator.rs`](../../../crates/aureline-workspace/tests/schema_migration_and_repair_lineage_fixture_generator.rs)
- Replay gate:
  [`/crates/aureline-workspace/tests/schema_migration_and_repair_lineage_replay.rs`](../../../crates/aureline-workspace/tests/schema_migration_and_repair_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/finalize-schema-migration-and-repair-flows-for-workspace.md`](../../../docs/workspace/m4/finalize-schema-migration-and-repair-flows-for-workspace.md)
- Typed consumer:
  `aureline_workspace::project_schema_migration_and_repair_lineage`

## What this packet proves

1. **Artifact-class coverage truth.** Each record carries one row per
   governed artifact class declaring one closed `artifact_class`. A
   corpus missing any of the six required classes narrows the record
   with `required_artifact_class_missing`.

2. **Repair-flow coverage truth.** Each record carries one row per
   governed repair flow declaring one closed `repair_flow_kind`. The
   required set is `inspect_repair`, `rebuild_derived_store`,
   `rehydrate_from_packet`, `quarantine_corrupt_artifact`,
   `restore_from_checkpoint`, and `manual_repair_handoff`. A missing
   flow narrows with `required_repair_flow_kind_missing`.

3. **Schema-version pinning truth.** Every migration row pins a
   non-zero `from_schema_version` and `to_schema_version`, and the
   to-version is at least the from-version. A zero version or a
   downgrade narrows with `schema_version_unpinned`.

4. **Migration-outcome honesty.** Every migration row declares one
   closed `migration_outcome`; a lossy or refused outcome carries a
   non-empty `migration_disclosure_ref`. A silent lossy migration
   narrows with `migration_disclosure_missing`.

5. **Repair-outcome honesty.** Every repair flow declares one closed
   `repair_outcome`; a lossy or refused outcome carries a non-empty
   `repair_disclosure_ref`. A silent lossy repair narrows with
   `repair_disclosure_missing`.

6. **Redaction honesty.** A `redacted_with_disclosure` redaction
   class requires a non-empty `redaction_disclosure_ref`. A missing
   redaction disclosure narrows with `redaction_disclosure_missing`.

7. **Restore-provenance / encoding / trust-state preservation.**
   Every migration and repair row preserves the restore provenance,
   encoding fidelity, trust state, and no-rerun semantics of any
   captured remembered actions. Any deviation narrows with
   `restore_provenance_not_preserved`,
   `encoding_fidelity_not_preserved`, `trust_state_not_preserved`, or
   `no_rerun_semantics_not_preserved`.

8. **No-silent-rerun honesty.** Every migration and repair row
   declares `explicit_user_action_required` or
   `terminal_no_further_run`. A `silent_rerun_permitted` posture
   narrows with `rerun_silent_forbidden`. Every state-mutating row
   ships a non-empty `commit_action_id` and `commit_disclosure_id`;
   missing metadata narrows with `commit_action_metadata_missing`.

9. **Repair-transaction-id / finding-code pinning.** Every migration
   and repair row pins a non-empty `repair_transaction_id` and a
   non-empty `finding_code` so support, docs, and shiproom packets
   all reference the same truth. Missing pinning narrows with
   `repair_transaction_id_not_pinned` or `finding_code_missing`.

10. **Inspection precedes destructive migration / repair.** The
    controlled inspection / repair hook table
    (`inspect_artifact`, `compare_before_migration`,
    `preview_migration`, `preview_repair`, `rollback_migration`,
    `rollback_repair`, `export_before_migration`,
    `export_before_repair`) must be reachable before any destructive
    migration or repair commits. A missing hook narrows with
    `inspection_hook_unavailable`.

11. **Support-export honesty.** Each row's support-export projection
    preserves `artifact_class`, `migration_outcome` /
    `repair_flow_class`, `from_schema_version` / `to_schema_version`,
    `repair_transaction_id`, `finding_code`, and `redaction_class`,
    and redacts raw secrets, raw artifact bytes, approval tickets,
    delegated credentials, and live authority handles. Dropping a
    field narrows with `support_export_fields_dropped`; raising raw
    material narrows with `support_export_redaction_unsafe`.

12. **Producer attribution is pinnable for replay.** Each record
    carries the producer ref, the schema version, the capture
    timestamp, and an integrity hash derived from the input
    identities so replay and support pipelines can pin the source
    before applying. Incomplete attribution narrows with
    `producer_attribution_incomplete`.

13. **Lineage and export stay honest.** Every record sets
    `raw_payload_excluded = true` and carries only opaque refs to
    the source workspace, corpus, and producer. An empty workspace
    or corpus ref narrows with `lineage_export_unsafe`.

14. **The record is replay-gated.** The replay gate re-projects each
    fixture and asserts it equals the checked-in `expected`, so the
    projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                                  | Workspace state covered                                                                          | Qualification           | Proves                                                                                                                  |
| -------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ----------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `baseline_schema_migration_and_repair_stable`            | Six required artifact classes + six required repair flows, lossless migrations, lossless repairs | `stable`                | A baseline release-branch corpus can prove the full contract                                                            |
| `extended_with_lossy_disclosed_stable`                   | Adds a lossy-with-disclosure prebuild-cache migration + a lossy-with-disclosure repair           | `stable`                | The lossy-with-disclosure posture and explicit redaction disclosure refs ride safely on the same projection             |
| `lossy_migration_silent_narrowed`                        | A lossy migration omits its `migration_disclosure_ref`                                           | `narrowed_below_stable` | The contract refuses to ship Stable when a lossy migration silently elides its disclosure                               |
| `repair_silent_rerun_narrowed`                           | A repair flow declares `silent_rerun_permitted`                                                  | `narrowed_below_stable` | The contract refuses to ship Stable when a repair flow can silently re-fire                                             |
| `missing_compare_before_migration_hook_narrowed`         | `compare_before_migration` inspection hook unavailable                                           | `narrowed_below_stable` | The contract refuses to ship Stable when a required pre-action hook is missing                                          |

## How to verify

```sh
# Unit + replay gate for the schema-migration and repair lineage projection.
cargo test -p aureline-workspace --lib schema_migration_and_repair_lineage
cargo test -p aureline-workspace --test schema_migration_and_repair_lineage_replay

# Regenerate fixtures (when adding new postures).
SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_GEN_FIXTURES=1 \
  cargo test -p aureline-workspace --test schema_migration_and_repair_lineage_fixture_generator

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_schema_migration_and_repair_lineage -- --lines \
  fixtures/workspace/m4/schema_migration_and_repair_lineage/baseline_schema_migration_and_repair_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and
replay gate above. The lineage record self-describes its stable
qualification: postures that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named
reason, so they never inherit an adjacent green row. No public
scope is widened from this row.
