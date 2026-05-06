# Update verification + rollback sequence cases

Worked fixtures for the update verification / rollback sequence packet:

- Packet: [`artifacts/release/update_rollback_sequence.yaml`](../../../artifacts/release/update_rollback_sequence.yaml)
- Narrative: [`docs/release/update_verification_and_rollback_sequence.md`](../../../docs/release/update_verification_and_rollback_sequence.md)

Each case:

- names the `variant_path_id` from the sequence packet;
- lists the ordered `checkpoint_hits[]` that occurred; and
- cites stable evidence refs (update manifest, update-ready review, mirror/import
  receipts, checkpoint refs, support bundle refs) without embedding raw payloads
  or private endpoints.

## Case list

- `online_success.yaml` — online official-feed update committed successfully.
- `online_failed_rollback.yaml` — online update fails post-restart and rolls back.
- `offline_or_mirror_success.yaml` — mirror/offline acquisition with integrity receipts.
- `helper_skew_blocked.yaml` — helper negotiation blocks before mutation.
- `side_by_side_success.yaml` — side-by-side channel update with explicit isolation.
- `enterprise_policy_blocked.yaml` — enterprise policy blocks update before apply.

