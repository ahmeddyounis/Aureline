# Portable Bundle and Shelf Beta

Portable bundles and shelf entries are read-only change objects for handoff and
resume. They cover offline review, browser companion inspection, incident
follow-up, support export, and local shelf resume with one vocabulary instead
of per-surface packets.

The boundary schema lives at:

- [`/schemas/change/portable_bundle.schema.json`](../../../schemas/change/portable_bundle.schema.json)

Canonical fixtures live at:

- [`/fixtures/review/m3/portable_bundle/`](../../../fixtures/review/m3/portable_bundle/)

Runtime projections live at:

- [`/crates/aureline-change-objects/src/portable_bundle/mod.rs`](../../../crates/aureline-change-objects/src/portable_bundle/mod.rs)
- [`/crates/aureline-shell/src/portable_bundle_inspector/mod.rs`](../../../crates/aureline-shell/src/portable_bundle_inspector/mod.rs)
- [`/crates/aureline-support/src/portable_bundle_handoff/mod.rs`](../../../crates/aureline-support/src/portable_bundle_handoff/mod.rs)

The support-review packet lives at:

- [`/artifacts/support/m3/portable_bundle_handoff_review.md`](../../../artifacts/support/m3/portable_bundle_handoff_review.md)

## Contract

Every portable bundle or shelf entry carries:

| Block | Purpose |
| --- | --- |
| `bundle_id` | Stable opaque bundle identity. |
| `object_class` | `portable_bundle` or `shelf_entry`. |
| `handoff_purpose_class` | Offline review, browser companion, incident follow-up, support export, or review export. |
| `target_identity` | Workspace, repo, worktree, base, head, target, and environment capsule refs. |
| `review_pack` | Review-pack ref, version, digest ref, and parity class. |
| `diff_refs` | Diff lineage refs only; raw diff bodies stay outside the record. |
| `evidence_refs` | Review, validation, incident, browser handoff, provider snapshot, and support refs. |
| `validation_state` | Current or stale validation class plus visible stale labels. |
| `authority_state` | Explicit absence of live bearer authority, ambient credentials, and secret material. |
| `open_modes` | Offline inspect, compare-only reopen, desktop resume after revalidation, browser read-only, or support inspect. |
| `redaction_profile` | Redaction class plus explicit destruction semantics. |
| `support_export_lineage` | Support, review, incident, browser, and mutation-journal refs. |

## UX Rules

- Import and open use the same record. The shell inspector, CLI/headless view,
  support export, browser companion, and review preview read the same
  `portable_change_bundle_record`.
- Imported bundles never claim live provider authority. Provider state is a
  stale snapshot unless the desktop reacquires authority through normal auth
  and revalidation flows.
- Stale validation is visible before any resume. Base, worktree scope,
  review-pack version, environment capsule, provider overlay, and evidence
  snapshot changes render as stale labels.
- Compare-only reopen is always available for stale or imported bundles.
  Desktop resume is available only when `desktop_resume_after_revalidation`
  is present and `revalidation_required_before_resume` is true.
- Browser companion handoff is read-only unless a separate command authority
  path grants a narrowly scoped action. The bundle itself never grants that
  authority.
- Support export preserves target identity, diff refs, evidence refs, redaction
  class, support lineage, and destruction receipts without raw secrets, raw
  credentials, raw remote URLs, or raw absolute paths.

## Fixture Coverage

The fixture set proves:

| Fixture | Coverage |
| --- | --- |
| `offline_review_handoff.json` | Current offline review export with compare-only reopen. |
| `browser_companion_handoff.json` | Browser read-only handoff with provider overlay unavailable and stale labels. |
| `incident_follow_up_stale_validation.json` | Incident follow-up packet with stale environment-capsule evidence. |
| `support_export_shelf_desktop_resume.json` | Shelf entry that can resume on desktop only after review-pack revalidation. |

## Out Of Scope

- Hosted shelf service or multi-user sync.
- Live provider credential transport.
- Hidden cloud-state sync.
- Raw patch, raw path, raw URL, or secret-byte transport in this record.
