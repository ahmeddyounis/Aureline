# Archive Search and Destruction Receipt Alpha Contract

This contract freezes the alpha vocabulary for archive search, redaction
boundaries, locality, and destruction receipts. It composes over the
record-class registry and delete-request state model; it does not replace
retention enforcement, legal-hold workflow, or archive infrastructure.

## Companion Artifacts

- [`/artifacts/governance/archive_redaction_boundary_alpha.yaml`](../../artifacts/governance/archive_redaction_boundary_alpha.yaml)
  is the machine-readable boundary and result vocabulary.
- [`/schemas/governance/destruction_receipt_alpha.schema.json`](../../schemas/governance/destruction_receipt_alpha.schema.json)
  defines receipt and non-receipt disclosure records.
- [`/fixtures/governance/destruction_receipt_alpha/`](../../fixtures/governance/destruction_receipt_alpha/)
  proves completed, partial, hold-blocked, policy-retained, outside-scope,
  manual-local, and redaction-omitted outcomes.
- [`/fixtures/admin/archive_search_boundary_alpha/`](../../fixtures/admin/archive_search_boundary_alpha/)
  is the first admin/support review projection that consumes the contract.
- [`/crates/aureline-shell/src/admin_alpha/mod.rs`](../../crates/aureline-shell/src/admin_alpha/mod.rs)
  carries the runtime-side result vocabulary, receipt refs, policy version,
  verifiers, skipped/retained/out-of-scope refs, and mirror/lag note.

## Result Vocabulary

Admin, support, export, and offboarding surfaces use these result classes:

`completed`, `partial`, `blocked_by_hold`, `policy_retained`,
`outside_platform_scope`, `manual_local_capture_required`,
`omitted_by_redaction`, and the reserved archive-search miss class
`not_found`.

The bounded alpha shell surface exercises the seven live outcomes. The
reserved `not_found` class exists so archive search can later report a
searched-but-missing governed ref without calling it deleted.

## Boundary Rules

Archive or evidence rows must disclose:

- record class and retention owner;
- locality: `local_only`, `managed_copy`, `archived`, `held`,
  `receipt_only`, `outside_platform_scope`, or `redacted_boundary`;
- whether the object is searchable, exportable, or receipt-only;
- the redaction boundary and omitted data classes; and
- chain-of-custody refs sufficient for support or admin review.

Rows carry opaque refs and metadata only. Raw payload bodies, raw
credentials, raw policy bodies, raw prompts, raw URLs, raw tenant names,
raw billing identifiers, and raw hold justifications remain outside this
boundary.

## Receipt Rules

A destructive or cleanup result either links a durable receipt or states
why no receipt can cover the requested scope. Receipt records must include:

- result class and receipt state;
- policy version and retention owner;
- executed time when destruction actually ran;
- verifier refs and chain-of-custody refs;
- destroyed refs, retained refs, skipped-held refs, outside-scope refs,
  manual-local refs, and omitted-by-redaction refs as applicable; and
- mirror, replication, backlog, or lag note.

Product copy must not say an object was fully deleted when a managed copy,
held archive, policy-retained subset, local-only artifact, provider-origin
record, redacted omission, or receipt-only tombstone still exists.

## First Consumer

The admin alpha support projection consumes this contract through
`AdminAlphaSupportExport`. The support-bundle seed includes that export as
metadata-only evidence, preserving result counts and the contract refs so
support can reconstruct the same partial-result truth without scraping UI
text.

## Verification

Run:

```sh
python3 ci/check_archive_destruction_alpha.py --repo-root .
cargo test -p aureline-shell admin_delete_hold_policy_alpha
```
