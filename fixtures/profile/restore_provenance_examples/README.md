# Portable-profile, export-manifest, and restore-provenance fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/state/profile_and_state_map.md`](../../../docs/state/profile_and_state_map.md)
and validated by the schema at
[`/schemas/profile/portable_profile.schema.json`](../../../schemas/profile/portable_profile.schema.json).

Each fixture names the record kind it exercises, the profile mode /
portability class / state authority / fidelity label / post-restore
validator classes / exclusion reasons it covers, and the section of
the profile-and-state-map document it motivates.

**Scope rules**

- Fixtures validate against the single portable-profile schema; they
  do not encode wire bytes, optional-sync payload encryption
  envelopes, ADR-0004 RPC envelopes, or ADR-0005 subscription
  envelopes.
- A new fixture MUST exercise at least one frozen record kind, one
  portability class, one authority class, one fidelity label, one
  profile mode, one state-class id, or one exclusion-reason id, and
  MUST cite the section of the profile-and-state-map document that
  motivates it.
- Monotonic timestamps and stable IDs are opaque; they are chosen to
  read well rather than to reflect any real clock or system state.
- ADR-0001 identity-mode, ADR-0007 secret-class and redaction-class,
  ADR-0008 scope and lifecycle, and the entry-restore object model's
  `restore_level` and `equivalence_map_ref` vocabularies are quoted
  by reference and never redefined.

**Index**

| Fixture | Record kind | Key classes exercised | Doc section |
|---|---|---|---|
| [`portable_profile_file_portable_plain.json`](./portable_profile_file_portable_plain.json) | `portable_profile_artifact_record` | `file_portable_plain` / `portable` + `portable_with_machine_addendum` / `excluded` with `contains_raw_secret_material`, `machine_unique_trust_anchor`, `workspace_trust_approval`, `admin_policy_ownership` | §3 Profile modes, §6 State map rows, §7 Export manifest rules |
| [`export_manifest_plain.json`](./export_manifest_plain.json) | `export_manifest_record` | `file_portable_plain` / `local_file` destination / every required excluded class named / counts-only redaction summary | §7 Export manifest rules |
| [`state_map_row_execution_context_cache.json`](./state_map_row_execution_context_cache.json) | `state_map_row_record` | `execution_context_cache` / `disposable_derived_cache` / `local_only` / `reclaimable_at_any_time` / `clear_allowed` | §6 State map rows |
| [`restore_provenance_exact.json`](./restore_provenance_exact.json) | `restore_provenance_record` | `fidelity_label = exact` / `aureline_portable_profile` / `settings_schema_migration = passed` | §4 Fidelity labels, §8 Restore-provenance rules |
| [`restore_provenance_compatible.json`](./restore_provenance_compatible.json) | `restore_provenance_record` | `fidelity_label = compatible` / `compatible_restore` / `equivalence_map_ref` + `rollback_checkpoint_ref` present / `keybinding_conflict` + `settings_schema_migration` validators | §4 Fidelity labels, §8 Restore-provenance rules |
| [`restore_provenance_layout_only.json`](./restore_provenance_layout_only.json) | `restore_provenance_record` | `fidelity_label = layout_only` / `aureline_session_restore_manifest` / `layout_restore_sanity = passed` alone | §4 Fidelity labels, §8 Restore-provenance rules |
| [`restore_provenance_manual_review.json`](./restore_provenance_manual_review.json) | `restore_provenance_record` | `fidelity_label = manual_review` / `imported_competitor_profile` / `extension_capability = failed_recoverable` + `workspace_trust_gate = failed_blocking` | §4 Fidelity labels, §8 Restore-provenance rules |

**Coverage contract**

Every fidelity label in the schema
(`exact` / `compatible` / `layout_only` / `manual_review`) MUST have
at least one restore-provenance fixture here; every profile mode
exercised by the frozen state-map rules MUST have at least one
portable-profile or export-manifest fixture here.
