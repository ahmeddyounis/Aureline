# Local-history export/replay packet and compare-to-disk lineage — contract

This is the companion contract document for the local-history
export/replay packet and compare-to-disk lineage record. It explains
the closed vocabularies, the pillar invariants, and the narrowing
posture the projection enforces. The canonical truth source is the
projection in
[`/crates/aureline-workspace/src/local_history_export_replay_lineage/`](../../../crates/aureline-workspace/src/local_history_export_replay_lineage/)
and the proof packet at
[`/artifacts/workspace/m4/harden-local-history-export-replay-packets-and-compare.md`](../../../artifacts/workspace/m4/harden-local-history-export-replay-packets-and-compare.md);
this document supports human review and does not redeclare contract
truth that the typed projection already carries.

## Vocabularies

### Export packet kinds

The packet-kind enumeration is closed:

- `local_history_entry_export` — one entry exported as a metadata-safe packet.
- `local_history_group_export` — a group of entries exported as a metadata-safe packet.
- `restore_checkpoint_export` — a named restore checkpoint exported as a metadata-safe packet.
- `compare_to_disk_diff_export` — a compare-to-disk diff packet pinned for inspection or replay.
- `support_bundle_local_history_section` — the local-history section of a support bundle.
- `replay_input_envelope` — *(optional)* an input envelope driving an external replay.
- `replay_output_envelope` — *(optional)* an output envelope captured by a replay run.

The first five are required on every Stable corpus.

### Replay path kinds

The replay-path enumeration is closed:

- `restore_from_packet` — restore a buffer or workspace from an exported packet (workspace-mutating).
- `compare_to_disk_replay` — compare a pinned packet to the current on-disk contents.
- `entry_inspect_replay` — inspect a single entry packet (read-only).
- `group_inspect_replay` — inspect a group packet (read-only).
- `support_bundle_replay` — replay the local-history section of a support bundle for diagnostic inspection.

All five are required on every Stable corpus.

### Compare-to-disk states

The compare-to-disk-state enumeration is closed:

- `in_sync_with_packet` — the packet matches the on-disk bytes exactly.
- `disk_modified_since_packet` — the disk content has changed since the packet was minted.
- `packet_decoded_recovered` — the packet was decoded via a recovery path; bytes are not lossless.
- `packet_redacted` — the packet redacts the body; the compare surface must say so.
- `compare_unavailable_remote_only` — compare-to-disk is unavailable because the source is remote-only.
- `local_only_packet` — the packet is local-only (e.g. an untitled buffer).

The states `disk_modified_since_packet`, `packet_decoded_recovered`,
`packet_redacted`, and `compare_unavailable_remote_only` must be
disclosed to the user. Silent acceptance narrows the record below
Stable.

### Body-availability classes

- `metadata_only` — default; only metadata-safe fields ship.
- `body_object_ref_with_disclosure` — content-addressed body object ref ships with an override disclosure.
- `raw_body_with_disclosure` — raw body bytes ship with an override disclosure (high-friction override path).
- `body_excluded_by_policy` — body intentionally excluded by policy.

`body_object_ref_with_disclosure` and `raw_body_with_disclosure`
require a non-empty `body_override_disclosure_ref`.

### Replay rerun postures

- `explicit_user_action_required` — replay requires an explicit user commit before running.
- `terminal_no_further_run` — replay is terminal and does not re-fire.
- `silent_rerun_permitted` — replay may re-fire silently. **Forbidden on Stable rows.**

### Encoding fidelity classes

- `utf8_lf`, `utf8_crlf`, `utf8_bom_lf`, `utf8_bom_crlf` — UTF-8 with the declared newline mode and BOM posture.
- `non_utf8_declared_encoding` — a declared non-UTF-8 encoding preserved verbatim.
- `binary_safe` — binary or large-file packets handled in binary-safe mode.

Every packet additionally pins `encoding_preserved`,
`newline_preserved`, and `bom_preserved` flags; every replay path
pins `preserves_encoding_fidelity`.

### Inspection hook classes

The pre-action inspection / repair hook table is closed:
`inspect_packet`, `compare_before_replay`, `preview_replay`,
`export_packet`, `rollback_replay`, `repair_packet`. All six must be
reachable before any destructive replay or cleanup commits.

## Pillars

1. **Packet-kind coverage truth.**
2. **Replay-path coverage truth.**
3. **Compare-to-disk honesty.**
4. **Body-export safety.**
5. **Encoding/newline/BOM fidelity preservation.**
6. **Restore-provenance preservation.**
7. **No-silent-rerun honesty.**
8. **Integrity-hash pinning.**
9. **Pre-action inspection-hook honesty.**
10. **Support-export honesty.**
11. **Producer attribution.**
12. **Lineage and export honesty.**

Each pillar is enforced by a narrow reason in the projection: a
posture that cannot prove the pillar narrows below Stable with a
named reason, so it never inherits an adjacent green row.

## Replay safety

The projection self-describes its stable qualification, so a posture
that cannot prove the contract sets
`stable_qualification.level = narrowed_below_stable` with a named
list of `narrow_reasons`. Support exports and replay pipelines should
refuse to apply a record whose
`producer_attribution.producer_attribution_complete` is false or
whose `raw_payload_excluded` is not true.
