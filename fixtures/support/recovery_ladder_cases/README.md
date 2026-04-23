# Recovery-ladder seed cases

These fixtures are the seed cases the recovery-ladder packet at
[`docs/support/recovery_ladder_packet.md`](../../../docs/support/recovery_ladder_packet.md)
defines. Each file validates against the
`recovery_ladder_seed_case_record` alternate in
[`schemas/support/recovery_action.schema.json`](../../../schemas/support/recovery_action.schema.json),
which is itself the boundary schema the packet shares with the
companion `recovery_action_record` shape.

Every case:

- names one stable `recovery_action_id` (for example
  `recovery_action:safe_mode.crash_loop_entry`);
- binds one `rung_class` from the frozen support-bundle
  `recovery_rung_class` vocabulary;
- names exactly one `reversal_class` drawn from the closed set
  (`exact_undo`, `compensating_action`, `regeneration`,
  `checkpoint_restore`, `no_undo_export_only`);
- lists the `preserved_state_classes`, `lost_capability_classes`, and
  `escalation_trigger_classes` from the frozen vocabularies the packet
  doc defines;
- binds stable `linkage_refs` onto the support-bundle case, the
  object-issue handoff case, and the Project Doctor finding code it
  belongs to (or declares those refs `null` with an explicit reason in
  `notes`); and
- declares four reviewer-facing explanation strings (preserved work,
  disabled capability, escalation, next step) so export flows can
  render recovery advice verbatim without minting prose.

## Case list

- `crash_loop_safe_mode.yaml` — `recovery_action:safe_mode.crash_loop_entry`
- `suspect_extension_quarantine.yaml` —
  `recovery_action:extension_quarantine.suspect_host_regression`
- `open_without_restore.yaml` —
  `recovery_action:open_without_restore.session_restore_declined`
- `cache_index_repair.yaml` —
  `recovery_action:cache_reset_candidate.cache_index_repair`
- `restricted_mode_fallback.yaml` —
  `recovery_action:restricted_reopen.managed_fallback`

Every case cites its scenario ids by stable ref so support review can
pivot in O(1) from one rung → one support-bundle case → one
escalation case → one Project Doctor finding code.
