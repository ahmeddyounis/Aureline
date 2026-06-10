# Generation diff review, rollback/delete-generated recovery, and managed-zone honesty

This contract describes the export-safe packet that carries the **generation
review and recovery** truth for a generated project tree: the managed-zone
classification (what is authored, what is generated, what is runtime-only), the
generation-diff review state and overwrite guard, and the rollback or
delete-generated recovery action. The packet is the canonical truth that the
diff-review, run, recovery, diagnostics, and support surfaces ingest instead of
overwriting silently, deleting authored work, or re-describing recovery by hand.

- Boundary schema:
  `schemas/templates/add-generation-diff-review-rollback-or-delete-generated-recovery-and-managed-zone-honesty.schema.json`
- Implementation:
  `crates/aureline-templates/src/add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty/`
- Checked support export:
  `artifacts/templates/m5/add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty/support_export.json`
- Fixtures:
  `fixtures/templates/m5/add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty/`

This packet **references** the upstream generated-project records frozen in
`docs/templates/template_registry_and_scaffold_contract.md` — the
`generated_project_lineage_alpha` and `generated_project_update_semantics`
contracts — by opaque ref (`lineage_ref`, `generated_root_ref`,
`scaffold_run_ref`) rather than embedding them, and reuses their overwrite-guard
vocabulary instead of inventing parallel terms.

## Boundary discipline

The packet is metadata only. Raw file bodies, raw diffs, raw paths, repository
URLs, hook bodies, secrets, and user-authored content never cross this boundary.
Rows carry opaque refs, closed-vocabulary class tokens, and short reviewable
summaries. `validate` rejects any export that leaks obviously forbidden
material.

## Row truth

Each `recovery_row` binds one generated tree to:

- **Managed-zone honesty** — `managed_zone_class` labels the scope as
  `authored_only`, `generated_only`, `generated_then_user_edited`,
  `runtime_only`, `mixed_authored_and_generated`, or
  `zone_unknown_review_required`, so the generated zone is always distinct from
  authored work and recovery never deletes authored content.
- **Generation-diff review** — `diff_review_class` and `diff_preview_ref` keep a
  reviewable diff in front of every write; a `preview_ready` row cites its
  preview. `diff_unavailable_review_required` blocks any write.
- **Overwrite guard** — `overwrite_guard_class` makes the no-silent-overwrite
  posture explicit, from `no_overwrite_needed` through
  `overwrite_requires_three_way_review` to the `overwrite_blocked_*` states.
- **Recovery** — `recovery_action_class` (`rollback_to_checkpoint`,
  `delete_generated_only`, `restore_authored_and_delete_generated`,
  `quarantine_generated`, or a blocked state), `checkpoint_ref` for rollback,
  and `authored_content_protected`. A destructive action (rollback, delete, or
  restore-and-delete) requires `authored_content_protected` to be `true`, and
  `delete_generated_only` may never be scoped over an `authored_only` zone.
- **Support honesty** — `support_class` keeps bridge/heuristic behavior labeled.
  A `bridge_behavior` or `heuristic_mapping` row must disclose a known issue and
  carry the `bridge_behavior_disclosed` downgrade trigger, so framework-pack
  bridge behavior is never presented as exact first-party truth.
- **Downgrade and projection** — `downgrade_triggers`, `consumer_surfaces`, and
  `admitted_for_recovery`. A blocked row (diff unavailable, overwrite hard
  blocked, recovery unavailable, or zone unknown) can never be admitted.

## Downgrade automation

`apply_downgrade_automation` narrows rows from observed runtime signals so a
stale or underqualified row narrows before it is offered, instead of being
hidden or silently overwritten:

- **Unknown lineage** marks the zone unknown, the diff unavailable, and the
  overwrite and recovery blocked, and withdraws admission.
- **Unverified authored-content protection** downgrades a destructive recovery
  to `quarantine_generated` and withdraws admission.
- **A missing diff preview** marks the diff `diff_unavailable_review_required`,
  forbids overwrite without a preview, and withdraws admission.
- **A missing rollback checkpoint** withdraws a `rollback_to_checkpoint` action
  to `no_recovery_available`.
- **Stale proof or a narrowed upstream** withdraws admission.

A narrowed row stays a valid, export-safe packet, so the diff-review and support
surfaces show a current, labeled state rather than an optimistic placeholder.

## Consumers

`current_generation_recovery_export()` reads and validates the checked support
export. It is the first real consumer: a diff-review, recovery, diagnostics, or
support-export surface ingests the canonical packet through it. The two checked
fixtures (`lineage_unknown_blocked.json`, `authored_protection_quarantined.json`)
are valid, narrowed packets that exercise the downgrade behavior the canonical
export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical
builder:

```text
cargo run -p aureline-templates --example dump_generation_recovery -- canonical
cargo run -p aureline-templates --example dump_generation_recovery -- markdown
cargo run -p aureline-templates --example dump_generation_recovery -- lineage_unknown
cargo run -p aureline-templates --example dump_generation_recovery -- authored_protection
```
