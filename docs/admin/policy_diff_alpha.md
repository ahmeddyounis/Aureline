# Admin policy-diff, delete-honesty, and legal-hold alpha

This document describes the bounded admin alpha packet implemented in
`crates/aureline-shell/src/admin_alpha/`. It gives desktop, support,
and admin/export surfaces one shared record for retention-aware delete
review, legal-hold visibility, timezone-aware chronology, and policy
diff preview before apply.

The packet is a projection over existing governance and admin contracts:

- [`docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
- [`docs/governance/retention_deletion_matrix_contract.md`](../governance/retention_deletion_matrix_contract.md)
- [`docs/admin/policy_explainability_contract.md`](./policy_explainability_contract.md)
- [`docs/admin/admin_audit_export_contract.md`](./admin_audit_export_contract.md)
- [`docs/admin/audit_event_explorer_contract.md`](./audit_event_explorer_contract.md)

It does not implement a legal-hold backend, a policy evaluator, a
retention engine, or a hosted admin console.

## Packet Shape

`AdminAlphaPacket` contains:

- flattened `delete_review_rows`;
- one `policy_diff_preview`;
- a desktop projection for shell/admin surfaces; and
- a support-export projection consumed by `SupportSeedSurface`.

Every delete/export row must surface:

- archive-search posture;
- hold selector or hold-scope truth;
- redaction boundary;
- chain-of-custody summary;
- destruction-receipt availability; and
- export-before-delete posture.

Rows that cannot complete must still explain what remains and why. A
held or retained artifact is never summarized as deleted.

## Result Vocabulary

The support export preserves the full result vocabulary:

- `completed`
- `partial`
- `blocked_by_hold`
- `policy_retained`
- `outside_platform_scope`
- `manual_local_capture_required`
- `omitted_by_redaction`

The protected fixture in
[`fixtures/admin/delete_hold_policy_alpha/`](../../fixtures/admin/delete_hold_policy_alpha/)
exercises each state in one support/offboarding review packet.

## Chronology

Every timestamp uses the same representation:

- canonical UTC instant;
- local civil time with explicit offset;
- IANA timezone id;
- offset at the instant;
- source clock class; and
- stable ordering key.

The fixture currently pins `America/Los_Angeles` for the rendered admin
surface while preserving canonical UTC for export.

## Policy Diff Preview

`AdminPolicyDiffPreview` is valid only when:

- the diff is generated before apply;
- apply remains blocked until preview acknowledgement;
- current, baseline, and proposed policy sources are declared; and
- each diff row carries a redaction-safe lifecycle consequence.

Diff rows can link to retention matrix rows, delete-request state rows,
deletion-job fixtures, or admin/audit records. Raw policy bodies,
support bundle bodies, secret material, and tenant directory payloads
remain outside the packet.

## Support Consumer

`SupportSeedSurface::admin_delete_hold_policy_preview` adds the packet
as a metadata-only support bundle row. The row carries result counts,
policy diff id, archive/redaction contract ref, destruction-receipt
schema ref, and redaction posture so support can inspect the same state
vocabulary without scraping UI text or screenshots.

## Verification

Run:

```sh
python3 ci/check_archive_destruction_alpha.py --repo-root .
cargo test -p aureline-shell admin_delete_hold_policy_alpha
```
