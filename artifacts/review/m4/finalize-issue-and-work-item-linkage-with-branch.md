# Artifact: Finalize issue and work-item linkage with branch

**Task:** Bind work-item detail surfaces, status-transition sheets, offline-handoff continuity, branch/review links, and publish-later continuity into a single coherent work-item linkage finalization packet.
**Status:** Implemented
**Verification class:** Automated functional + Conformance

## Summary

The work-item linkage finalization lane binds review-workspace beta packets, review-stabilization packets, work-item detail surfaces, status-transition sheets, offline-handoff continuities, branch-to-work-item links, review-to-work-item links, publish-later continuities, command-graph operations, and support/export envelopes into a single coherent packet. Every work-item detail surface discloses its write mode, every transition sheet previews provider side effects and preserves local draft on failure, offline handoffs survive restart/reconnect/export-import, and branch/review links are previewable before publish.

## What changed

- New Rust module: `crates/aureline-review/src/finalize_issue_and_work_item_linkage_with_branch/mod.rs`
- New fixtures: `fixtures/review/m4/finalize-issue-and-work-item-linkage-with-branch/`
  - `finalized_current_all_surfaces_present.json`
  - `finalized_offline_handoff_only.json`
  - `finalized_partial_work_item_scope.json`
  - `finalized_stale_provider_overlay.json`
- New tests: `crates/aureline-review/tests/finalize_issue_and_work_item_linkage_with_branch_alpha.rs`
- New schema: `schemas/review/finalize_issue_and_work_item_linkage_with_branch.schema.json`
- New docs: `docs/review/m4/finalize-issue-and-work-item-linkage-with-branch.md`

## Acceptance criteria

- [x] Governed work-item detail surfaces carry explicit write-mode disclosure on every surface.
- [x] Status-transition sheets preview provider side effects before any mutation occurs.
- [x] Status-transition sheets preserve local draft on publish failure.
- [x] Offline-handoff packets are first-class with explicit restart, reconnect, and export/import survival.
- [x] Provider chips disclose write mode so users know whether a surface is read-only, comment/link, full-edit, offline-capture-only, or policy-blocked.
- [x] Branch-to-work-item links are previewable before publish.
- [x] Review-to-work-item links are previewable before publish.
- [x] Publish-later continuities cite queue items and survive restart/reconnect.
- [x] Support/export records keep every `raw_*_export_allowed` flag false.
- [x] The checked-in implementation, fixtures, and proof packet for work-item linkage finalization are current and referenced by the stable proof index.

## How to verify

```bash
cargo test -p aureline-review --test finalize_issue_and_work_item_linkage_with_branch_alpha
```

## Risks / follow-ups

- The module currently consumes `ReviewWorkspaceBetaPacket` and `ReviewStabilizationPacket` as separate inputs. When a unified review-state packet is introduced, the constructor should be updated to consume it directly.
- Provider classes and source classes are modeled as strings; when the provider crate stabilizes its enums, these should be narrowed to typed enums.
- The `write_mode_disclosure_class` field is a string projection. When the work-item governance crate provides strongly-typed write-mode records, the detail surface should consume them directly.
