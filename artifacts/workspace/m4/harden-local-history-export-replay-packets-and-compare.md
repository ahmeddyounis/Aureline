# Harden Local-History Export/Replay Packets and Compare-to-Disk Flows — proof packet

Reviewer-facing proof packet for the local-history export/replay
packet and compare-to-disk flow lineage lane: every governed export
packet kind (`local_history_entry_export`,
`local_history_group_export`, `restore_checkpoint_export`,
`compare_to_disk_diff_export`,
`support_bundle_local_history_section`, plus the optional
`replay_input_envelope` and `replay_output_envelope`) ships a row
bound to a stable packet identity, a declared body-availability
class, a declared encoding/newline/BOM posture, a non-empty
restore-of ref, mutation-journal ref, actor class, and integrity
hash. Every required replay path (`restore_from_packet`,
`compare_to_disk_replay`, `entry_inspect_replay`,
`group_inspect_replay`, `support_bundle_replay`) discloses which
packet it consumes, the closed compare-to-disk state (for
compare paths), and an explicit-user-action or terminal-no-rerun
posture with a non-empty commit action id and disclosure id whenever
it mutates the workspace. A compare-to-disk replay never silently
treats a `disk_modified_since_packet`, `packet_decoded_recovered`,
`packet_redacted`, or `compare_unavailable_remote_only` state as a
clean match. Raw-body packets only ship behind an explicit
override-disclosure ref. A destructive replay never fires without
the controlled inspection / repair hook table (`inspect_packet`,
`compare_before_replay`, `preview_replay`, `export_packet`,
`rollback_replay`, `repair_packet`) being reachable; a missing hook
narrows the record below Stable with a named reason. This packet
is the stable-line anchor for this lane; dashboards, docs,
Help/About surfaces, and support exports should ingest the typed
sources below rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/local_history_export_replay_lineage/`](../../../crates/aureline-workspace/src/local_history_export_replay_lineage/)
- Schema:
  [`/schemas/workspace/local_history_export_replay_lineage.schema.json`](../../../schemas/workspace/local_history_export_replay_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_local_history_export_replay_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_local_history_export_replay_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/local_history_export_replay_lineage/`](../../../fixtures/workspace/m4/local_history_export_replay_lineage/)
- Fixture generator:
  [`/crates/aureline-workspace/tests/local_history_export_replay_lineage_fixture_generator.rs`](../../../crates/aureline-workspace/tests/local_history_export_replay_lineage_fixture_generator.rs)
- Replay gate:
  [`/crates/aureline-workspace/tests/local_history_export_replay_lineage_replay.rs`](../../../crates/aureline-workspace/tests/local_history_export_replay_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/harden-local-history-export-replay-packets-and-compare.md`](../../../docs/workspace/m4/harden-local-history-export-replay-packets-and-compare.md)
- Typed consumer:
  `aureline_workspace::project_local_history_export_replay_lineage`

## What this packet proves

1. **Packet-kind coverage truth.** Each record carries one row per
   governed export packet declaring one closed `packet_kind`. A corpus
   missing any of the five required kinds narrows the record with
   `required_packet_kind_missing`.

2. **Replay-path coverage truth.** Each record carries one row per
   governed replay path declaring one closed `replay_path_kind`. The
   required set is `restore_from_packet`, `compare_to_disk_replay`,
   `entry_inspect_replay`, `group_inspect_replay`, and
   `support_bundle_replay`. A missing path narrows with
   `required_replay_path_kind_missing`.

3. **Compare-to-disk honesty.** Every compare-to-disk replay declares
   one closed `compare_to_disk_state` and discloses a
   `disk_modified_since_packet`, `packet_decoded_recovered`,
   `packet_redacted`, or `compare_unavailable_remote_only` state to
   the user. A silent disk-modified posture narrows with
   `disk_modified_silently_treated_as_clean`; a missing state narrows
   with `compare_to_disk_state_missing`.

4. **Body-export safety.** Packets default to `metadata_only`;
   `body_object_ref_with_disclosure` and `raw_body_with_disclosure`
   require a non-empty `body_override_disclosure_ref`. Shipping raw
   bytes by default narrows with `body_raw_by_default`; a missing
   override disclosure narrows with `body_override_disclosure_missing`.

5. **Encoding/newline/BOM fidelity.** Every packet preserves the
   source encoding, newline mode, and BOM posture; every replay path
   preserves the encoding fidelity onto disk. Any deviation narrows
   with `encoding_fidelity_not_preserved`.

6. **Restore-provenance preservation.** Every packet carries a
   non-empty `restore_of_ref`, `mutation_journal_ref`, and
   `actor_class`; every replay path preserves the restore provenance
   on the replayed entry. Any deviation narrows with
   `restore_provenance_not_preserved`.

7. **No-silent-rerun honesty.** Every replay path declares
   `explicit_user_action_required` or `terminal_no_further_run`. A
   `silent_rerun_permitted` posture narrows with
   `replay_rerun_silent_forbidden`. Every workspace-mutating replay
   ships a non-empty `commit_action_id` and `commit_disclosure_id`;
   missing metadata narrows with
   `replay_commit_action_metadata_missing`.

8. **Integrity-hash pinning.** Every packet pins a non-empty
   `integrity_hash`; every replay path verifies the hash before
   applying. Missing pinning narrows with `integrity_hash_not_pinned`.

9. **Inspection precedes destructive replay.** The controlled
   inspection / repair hook table
   (`inspect_packet`, `compare_before_replay`, `preview_replay`,
   `export_packet`, `rollback_replay`, `repair_packet`) must be
   reachable before any destructive replay / cleanup commits. A
   missing hook narrows with `inspection_hook_unavailable`.

10. **Support-export honesty.** Each row's support-export projection
    preserves `packet_kind`, `replay_path_class`, `packet_ref`,
    `compare_to_disk_class`, `body_availability_class`,
    `encoding_fidelity_class`, `restore_of_ref`,
    `mutation_journal_ref`, and `integrity_hash`, and redacts raw
    secrets, raw body bytes, approval tickets, delegated credentials,
    and live authority handles. Dropping a field narrows with
    `support_export_fields_dropped`; raising raw material narrows
    with `support_export_redaction_unsafe`.

11. **Producer attribution is pinnable for replay.** Each record
    carries the producer ref, the schema version, the capture
    timestamp, and an integrity hash derived from the input
    identities so replay and support pipelines can pin the source
    before applying. Incomplete attribution narrows with
    `producer_attribution_incomplete`.

12. **Lineage and export stay honest.** Every record sets
    `raw_payload_excluded = true` and carries only opaque refs to
    the source workspace, corpus, and producer. An empty workspace
    or corpus ref narrows with `lineage_export_unsafe`.

13. **The record is replay-gated.** The replay gate re-projects
    each fixture and asserts it equals the checked-in `expected`,
    so the projection cannot drift without failing CI.

## Fixture corpus

| Fixture                                                  | Workspace state covered                                                                  | Qualification           | Proves                                                                                                                  |
| -------------------------------------------------------- | ---------------------------------------------------------------------------------------- | ----------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `baseline_local_history_export_replay_stable`            | Five required packet kinds + five required replay paths, in-sync compare-to-disk          | `stable`                | A baseline release-branch corpus can prove the full contract                                                            |
| `extended_disk_modified_disclosed_stable`                | Adds a raw-body packet with an override disclosure + a disclosed `disk_modified` compare  | `stable`                | The override-disclosure body posture and the disclosed `disk_modified` compare state ride safely on the same projection |
| `disk_modified_silent_narrowed`                          | Compare-to-disk replay reports `disk_modified_since_packet` without disclosing it         | `narrowed_below_stable` | The contract refuses to ship Stable when a compare-to-disk replay silently treats a modified disk as a clean match      |
| `restore_silent_rerun_narrowed`                          | A restore-from-packet replay declares `silent_rerun_permitted`                            | `narrowed_below_stable` | The contract refuses to ship Stable when a replay path can silently re-fire                                             |
| `missing_compare_before_replay_hook_narrowed`            | `compare_before_replay` inspection hook unavailable                                       | `narrowed_below_stable` | The contract refuses to ship Stable when a required pre-action hook is missing                                          |

## How to verify

```sh
# Unit + replay gate for the local-history export/replay lineage projection.
cargo test -p aureline-workspace --lib local_history_export_replay_lineage
cargo test -p aureline-workspace --test local_history_export_replay_lineage_replay

# Regenerate fixtures (when adding new postures).
LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_GEN_FIXTURES=1 \
  cargo test -p aureline-workspace --test local_history_export_replay_lineage_fixture_generator

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_local_history_export_replay_lineage -- --lines \
  fixtures/workspace/m4/local_history_export_replay_lineage/baseline_local_history_export_replay_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and
replay gate above. The lineage record self-describes its stable
qualification: postures that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named
reason, so they never inherit an adjacent green row. No public
scope is widened from this row.
