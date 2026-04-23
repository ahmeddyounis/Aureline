# Skew-smoke seed cases

These fixtures are the seed cases the skew-smoke packet at
[`docs/release/skew_smoke_packet.md`](../../../docs/release/skew_smoke_packet.md)
defines. Each file is a `skew_smoke_case_record` instance that projects
onto the compatibility-row, restore-provenance, recovery-action, and
support-packet-index contracts already frozen in this repository —
the packet does not introduce a new JSON schema.

Every case:

- names one stable `skew_case_id` (for example
  `skew_case:release.side_by_side.stable_preview_coexist`);
- binds one `surface_class` from the frozen
  `skew_surface_class` vocabulary (`side_by_side_install`,
  `state_schema_migration`, `helper_agent_attach`,
  `downgrade_upgrade_rollback`);
- names exactly one `skew_state_class` drawn from the closed set
  (`compatible`, `degraded`, `blocked`, `repairable`,
  `unknown_requires_probe`);
- carries exactly one `outcome_label_class`,
  `blocked_vs_degraded_class`, and `promotion_decision_class` from the
  frozen vocabularies the packet doc defines;
- quotes a `compatibility_row_ref` from
  `artifacts/compat/qualification_matrix_seed.yaml` and a
  `version_skew_register_ref` from
  `artifacts/compat/version_skew_register.yaml`;
- names at least one `support_packet_routing_class` so a support
  export never drops silently into a generic lane;
- lists the `preserved_state_classes` and `capability_impact_classes`
  the surface honours during the case; and
- declares four reviewer-facing explanation strings (user-facing
  summary, compatibility-report summary, support summary, promotion
  summary) so release/support/docs surfaces render skew advice
  verbatim without minting prose.

## Case list

- `side_by_side_stable_preview_coexist.yaml` —
  `skew_case:release.side_by_side.stable_preview_coexist`
- `state_migration_old_to_new_additive.yaml` —
  `skew_case:release.state_schema.old_to_new_additive`
- `state_migration_new_to_old_blocked.yaml` —
  `skew_case:release.state_schema.new_to_old_blocked`
- `helper_agent_attach_skewed_client_degraded.yaml` —
  `skew_case:release.helper_agent_attach.skewed_client_degraded`
- `rollback_prior_channel_build_compatible.yaml` —
  `skew_case:release.rollback.prior_channel_build_compatible`
- `helper_agent_attach_unknown_probe_required.yaml` —
  `skew_case:release.helper_agent_attach.unknown_probe_required`

Every case cites its compatibility row, version-skew-register case,
and (where applicable) recovery rung by stable ref so release and
support review can pivot in O(1) from one skew case → one
compatibility row → one support-packet family → one promotion
decision.
