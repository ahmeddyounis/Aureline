# Certify Storage-Discipline Surfaces, Cache-Governance Inspector, and State-Root Audits on Stable Profiles — proof packet

Reviewer-facing proof packet for the state-root certification
lineage lane: every governed state-root resource class
(`persistent_state_envelope`, `workspace_state_root`, `profile_root`,
`recent_work_root`, `local_history_root`, `restore_checkpoint_root`,
`cache_governance_root`, plus the optional `prebuild_cache_root` and
`mutation_journal_root`) ships a row bound to a stable resource
identity, a non-empty `storage_class_ref` into the cache /
storage-class lineage, a closed audit finding, a non-empty
`audit_transaction_id`, and a non-empty `finding_code`. Every
governed audit surface (`storage_discipline_overview`,
`cache_governance_inspector`, `state_root_audit_panel`,
`cleanup_inventory_audit`, `eviction_rule_audit`,
`headless_audit_cli`, `support_export_audit_section`) ships a row
bound to a reachable surface that discloses non-clean findings and
preserves both the resource lineage refs and the resource trust
state. A dirty / inconclusive / refused finding only ships behind an
explicit `audit_disclosure_ref` plus at least one
`cleanup_surface_refs` entry and at least one
`inspection_hook_refs` entry; a redacted row only ships behind an
explicit `redaction_disclosure_ref`. A destructive cleanup or repair
never fires without the controlled inspection / cleanup / repair
hook table (`inspect_state_root`, `compare_before_cleanup`,
`preview_cleanup`, `preview_repair`, `rollback_cleanup`,
`rollback_repair`, `export_before_cleanup`, `export_before_repair`)
being reachable; a missing hook narrows the record below Stable
with a named reason. The record additionally pins one closed
[`ClaimedStableProfile`] so a `narrowed_below_stable` claim cannot
inherit adjacent green rows. This packet is the stable-line anchor
for this lane; dashboards, docs, Help/About surfaces, and support
exports should ingest the typed sources below rather than cloning
this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/state_root_certification_lineage/`](../../../crates/aureline-workspace/src/state_root_certification_lineage/)
- Schema:
  [`/schemas/workspace/state_root_certification_lineage.schema.json`](../../../schemas/workspace/state_root_certification_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_state_root_certification_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_state_root_certification_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/state_root_certification_lineage/`](../../../fixtures/workspace/m4/state_root_certification_lineage/)
- Fixture generator:
  [`/crates/aureline-workspace/tests/state_root_certification_lineage_fixture_generator.rs`](../../../crates/aureline-workspace/tests/state_root_certification_lineage_fixture_generator.rs)
- Replay gate:
  [`/crates/aureline-workspace/tests/state_root_certification_lineage_replay.rs`](../../../crates/aureline-workspace/tests/state_root_certification_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/certify-storage-discipline-surfaces-cache-governance-inspector-and-state-root-audits.md`](../../../docs/workspace/m4/certify-storage-discipline-surfaces-cache-governance-inspector-and-state-root-audits.md)
- Typed consumer:
  `aureline_workspace::project_state_root_certification_lineage`

## What this packet proves

1. **Resource-class coverage truth.** Each record carries one row
   per governed state-root resource class declaring one closed
   `resource_class`. A corpus missing any of the seven required
   classes narrows the record with `required_resource_class_missing`.

2. **Audit-surface coverage truth.** Each record carries one row
   per governed audit surface declaring one closed
   `audit_surface_kind`. A missing surface narrows with
   `required_audit_surface_missing`.

3. **Storage-class taxonomy truth.** Every audit row references a
   non-empty `storage_class_ref` into the cache / storage-class
   lineage so each audit binds to a governed eviction policy. An
   empty ref narrows with `storage_class_ref_missing`.

4. **Audit-finding honesty.** Every audit row declares one closed
   `audit_finding`; a non-clean finding carries a non-empty
   `audit_disclosure_ref`. A silent non-clean audit narrows with
   `audit_disclosure_missing`.

5. **Redaction honesty.** A `redacted_with_disclosure` redaction
   class requires a non-empty `redaction_disclosure_ref`. A missing
   redaction disclosure narrows with `redaction_disclosure_missing`.

6. **Cleanup-precondition truth.** A `audit_dirty_with_disclosure`,
   `audit_inconclusive_held`, or `audit_refused_unsafe` finding binds
   at least one `cleanup_surface_refs` entry and at least one
   `inspection_hook_refs` entry. A row that omits either narrows
   with `cleanup_precondition_missing`.

7. **Restore-provenance / encoding / trust-state / lineage-refs
   preservation.** Every audit row preserves the restore provenance,
   encoding fidelity, trust state, and lineage refs of the resource
   it audits. Any deviation narrows with
   `restore_provenance_not_preserved`,
   `encoding_fidelity_not_preserved`, `trust_state_not_preserved`,
   or `lineage_refs_not_preserved`.

8. **No-silent-rerun honesty.** Every audit row declares
   `explicit_user_action_required` or `terminal_no_further_run`. A
   `silent_rerun_permitted` posture narrows with
   `rerun_silent_forbidden`. Every state-mutating row ships a
   non-empty `commit_action_id` and `commit_disclosure_id`; missing
   metadata narrows with `commit_action_metadata_missing`.

9. **Audit-transaction id / finding-code pinning.** Every audit row
   pins a non-empty `audit_transaction_id` and a non-empty
   `finding_code` so support, docs, and shiproom packets all
   reference the same truth. Missing pinning narrows with
   `audit_transaction_id_not_pinned` or `finding_code_missing`.

10. **Audit-surface reachability / disclosure honesty.** Every
    required audit surface must be reachable, disclose non-clean
    findings, and preserve both the resource lineage refs and the
    resource trust state. A missing surface narrows with
    `audit_surface_unreachable` or `audit_surface_disclosure_gap`.

11. **Inspection precedes destructive cleanup / repair.** The
    controlled inspection / cleanup / repair hook table
    (`inspect_state_root`, `compare_before_cleanup`,
    `preview_cleanup`, `preview_repair`, `rollback_cleanup`,
    `rollback_repair`, `export_before_cleanup`,
    `export_before_repair`) must be reachable before any destructive
    cleanup or repair commits. A missing hook narrows with
    `inspection_hook_unavailable`.

12. **Support-export honesty.** Each row's support-export projection
    preserves `resource_class`, `audit_finding`, `storage_class_ref`,
    `claimed_profile`, `audit_transaction_id`, `finding_code`, and
    `redaction_class`, and redacts raw secrets, raw artifact bytes,
    approval tickets, delegated credentials, and live authority
    handles. Dropping a field narrows with
    `support_export_fields_dropped`; raising raw material narrows
    with `support_export_redaction_unsafe`.

13. **Claimed-profile honesty.** The record binds one closed
    `claimed_profile`; a `narrowed_below_stable` claim narrows the
    record with `claimed_profile_not_stable` so a row that no longer
    qualifies cannot inherit adjacent green rows.

14. **Producer attribution.** Each record carries the producer ref,
    schema version, capture timestamp, and an integrity hash derived
    from the input identities so import / replay / support pipelines
    can pin the source before applying. A missing producer ref or
    capture timestamp narrows with `producer_attribution_incomplete`.

15. **Lineage and export honesty.** The record sets
    `raw_payload_excluded = true` and carries only opaque refs to
    the source workspace, corpus, and producer. A missing workspace
    or corpus ref narrows with `lineage_export_unsafe`.

## Fixture corpus

The checked-in fixture corpus covers:

- `baseline_state_root_certification_stable.json` — Baseline Stable
  posture with the seven required resource classes and the seven
  required audit surfaces, all audits clean.
- `extended_with_dirty_disclosed_stable.json` — Stable posture with a
  dirty-with-disclosure prebuild-cache audit carrying a redaction
  disclosure plus two cleanup-surface refs and two inspection-hook
  refs, an optional mutation-journal audit row, and the
  `stable_support_export` profile claim.
- `dirty_audit_silent_narrowed.json` — Narrowed below Stable: a dirty
  audit ships without its `audit_disclosure_ref`.
- `audit_silent_rerun_narrowed.json` — Narrowed below Stable: an audit
  row declares `silent_rerun_permitted`.
- `missing_compare_before_cleanup_hook_narrowed.json` — Narrowed
  below Stable: the required `compare_before_cleanup` inspection
  hook is unavailable on the posture.

## Verification

- `cargo test -p aureline-workspace --lib state_root_certification_lineage`
  runs the projection unit tests.
- `cargo test -p aureline-workspace --test state_root_certification_lineage_replay`
  runs the replay gate against the checked-in fixture corpus.
- `STATE_ROOT_CERTIFICATION_LINEAGE_GEN_FIXTURES=1 cargo test -p aureline-workspace --test state_root_certification_lineage_fixture_generator -- generate_fixtures`
  regenerates the fixture corpus when the projection's exact
  serialization changes.
