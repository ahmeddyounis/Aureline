# Extension lifecycle + quarantine sequence cases

Worked fixtures for the extension lifecycle and quarantine recovery sequence:

- Packet: [`artifacts/extensions/extension_lifecycle_states.yaml`](../../../artifacts/extensions/extension_lifecycle_states.yaml)
- Narrative: [`docs/extensions/extension_lifecycle_and_quarantine_sequence.md`](../../../docs/extensions/extension_lifecycle_and_quarantine_sequence.md)

Each case:

- names a `variant_path_id` from the sequence packet;
- lists the ordered `checkpoint_hits` that occurred; and
- cites stable evidence refs (activation evidence packet ids, trigger rule ids,
  denial reasons, forensic packet refs, recovery rung transition refs, uninstall
  receipts, support export refs) without embedding raw logs, raw dumps, or raw
  artifact bytes.

## Case list

- `first_party_online_crash_loop_quarantine_recovered.yaml` — first-party extension crash loops, quarantines, enters safe mode, clears quarantine, and reactivates.
- `third_party_online_runtime_budget_degraded_then_recovered.yaml` — third-party extension is degraded under budget pressure and returns to active.
- `third_party_online_crash_loop_quarantine_removed.yaml` — third-party extension crash loops, quarantines, enters safe mode + bisect, and is removed.
- `third_party_offline_bundle_crash_loop_quarantine_removed.yaml` — offline-bundle installed extension crash loops and is removed after isolation.
- `third_party_mirror_publisher_blocked.yaml` — mirror-delivered extension is publisher-blocked (install/activation denied) with explainable denial reasons.

