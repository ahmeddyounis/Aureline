# Artifact: Finalize Git and review support-export packets, timeline/chronology truth, and operator playbooks

**Task:** Finalize Git and review support-export packets, timeline/chronology truth, and operator playbooks as a daily-driver credibility lane.
**Status:** Implemented
**Verification class:** Automated functional + Conformance / interoperability suite + Design QA / UX validation + Release evidence review

## Summary

The Git/review support-export timeline lane projects a review workspace's Git and review history into one canonical, redaction-safe packet. It binds three concerns: a strictly ordered chronology of timeline events with explicit clock provenance and attribution; operator playbooks whose steps stay previewable, recoverable, and explicit about authority; and a finalized support-export packet that cites its source schema and keeps raw URLs and raw provider payloads off the support boundary. Chronology is canonical truth — never approximated and never silently re-ordered — and hosted/provider authority is always disclosed rather than hidden behind local chrome.

## What changed

- New Rust module: `crates/aureline-review/src/finalize_git_and_review_support_export_packets_timeline/mod.rs`
- Re-exported from `crates/aureline-review/src/lib.rs`
- New schema: `schemas/review/git_review_support_export_timeline.schema.json`
- New fixtures: `fixtures/review/m4/finalize-git-and-review-support-export-packets-timeline/`
  - `chronology_current_with_playbook.json`
  - `reconstructed_timeline_from_lineage.json`
  - `blocked_authority_broadening_step.json`
- New tests: `crates/aureline-review/tests/finalize_git_and_review_support_export_packets_timeline_alpha.rs`
- New docs: `docs/review/m4/finalize-git-and-review-support-export-packets-timeline.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current on the release branch and referenced by the stable review docs index.
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable: authority-broadening playbook steps are forced non-actionable and narrow the timeline, and unverified clock sources block operator-truth claims.
- [x] Daily Git/review or migration workflows stay previewable, attributable, and reversible: every mutating playbook step is previewable and reversible-or-checkpointed.
- [x] Provider-linked or browser-handoff behavior is explicit about freshness and ownership: hosted events and hosted-authority steps must disclose hosted authority or construction fails.
- [x] Timeline/chronology truth is canonical: events strictly increase by sequence index, every event is attributed and clock-sourced, and lineage parents resolve.
- [x] Support export is redaction-safe: raw URL and raw provider payload exports are forbidden and the packet cites its source schema.

## Guardrails honored

- Hosted/provider mutations are never hidden behind local chrome — `discloses_hosted_authority` is mandatory for provider-linked, browser-handoff, and provider-publish events, and for `hosted_provider_mutation` steps.
- Reconstructed chronology is labelled (`reconstructed_from_lineage` clock source, `chronology_reconstructed` state) rather than masqueraded as a trusted clock.
- Public scope is not widened: the module consumes the existing `ReviewWorkspaceBetaPacket` and adds one bounded packet family.

## How to verify

```bash
cargo test -p aureline-review --test finalize_git_and_review_support_export_packets_timeline_alpha
cargo test -p aureline-review --lib finalize_git_and_review
```

## Risks / follow-ups

- The module consumes `ReviewWorkspaceBetaPacket`. When a unified review-state packet lands, the constructor should consume it directly.
- Clock sources and event kinds are modeled as closed string vocabularies; when the Git and provider crates stabilize typed clock/event enums, these should be narrowed to typed enums.
- Chronology-gap detection is declarative (`chronology_gap_detected` is supplied by the producer). A later revision can derive gap detection from non-contiguous sequence indices once the upstream event source guarantees contiguous numbering.
