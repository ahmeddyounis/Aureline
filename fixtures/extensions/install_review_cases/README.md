# Extension install-review cases (effective permission + scope diff + publisher continuity)

This directory contains worked fixtures for the reviewer-facing objects
defined by:

- [`docs/extensions/effective_permission_review_contract.md`](../../../docs/extensions/effective_permission_review_contract.md)
- [`schemas/extensions/effective_permission_review.schema.json`](../../../schemas/extensions/effective_permission_review.schema.json)
- [`docs/verification/install_review_packet.md`](../../../docs/verification/install_review_packet.md)

Each fixture file is a multi-document YAML bundle:

1. `publisher_continuity_packet_record`
2. `manifest_scope_diff_record`
3. `effective_permission_review_sheet_record`
4. `extension_install_review_case_record` (ties the ids together)

The corpus exists to keep install/update/restore explanations consistent
across UI, CLI/headless review, offline restore previews, and support
exports — without per-surface prose or registry-specific field sets.

## Case index

| Fixture | Covers |
|---|---|
| `policy_narrowed_permissions.yaml` | Policy narrows a declared scope to step-up and denies another scope. |
| `dependency_smuggled_capability.yaml` | Dependency closure widens effective permission beyond the top-level manifest. |
| `publisher_transfer_continuity.yaml` | Publisher transfer/successor continuity remains visible and reviewable. |
| `revoked_mirror_metadata.yaml` | Broken mirror continuity and revoked mirror-promotion metadata renders explicitly. |
| `offline_review_unavailable_reverify.yaml` | Offline review where signature reverify is unavailable and truth is explicitly downgraded. |
| `restored_install_state.yaml` | Restored workspace/profile state preserves continuity refs and shows scope impact. |

