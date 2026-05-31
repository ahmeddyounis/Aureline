# Finalize Git and review support-export packets, timeline/chronology truth, and operator playbooks

**Scope:** Make a review workspace's Git and review history exportable as one canonical, redaction-safe packet for daily-driver review and support lanes.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Bind three concerns into a single previewable, attributable, and reversible artifact:

1. **Timeline / chronology truth** — a canonical, strictly ordered event log with explicit clock provenance, attribution, and lineage.
2. **Operator playbooks** — runbooks whose steps are previewable, recoverable, and explicit about authority.
3. **Finalized support-export packet** — a metadata-safe export that cites its source schema and never leaks raw URLs or raw provider payloads.

## Design principles

1. **Chronology is canonical, never approximated.** Every timeline event carries a strictly increasing `sequence_index`. Construction rejects any timeline that is not monotonic. Ordering is truth, not a rendering convenience.
2. **Every event is attributed and clock-sourced.** Each event names its `actor_ref`, `event_source_class`, and `clock_source_class` from the closed vocabulary. Time that was rebuilt rather than observed is labelled `reconstructed_from_lineage` so it is never mistaken for a trusted clock.
3. **Hosted authority is always disclosed.** Events that originate from `provider_linked` or `browser_handoff` sources — or any `provider_publish` event — must set `discloses_hosted_authority`. The stable line never hides hosted mutations behind local chrome.
4. **Lineage resolves.** Every `lineage_parent_ref` must point at a known event, so the chronology graph cannot reference a phantom parent.
5. **Operator playbook steps are recoverable.** A mutating step (`apply_with_checkpoint`, `revert`) must support preview and be either reversible or checkpoint-backed. A step claiming `hosted_provider_mutation` authority must disclose it.
6. **No hidden authority broadening.** A step flagged `would_broaden_authority` is forced non-actionable and narrows the whole timeline below actionable, rather than being silently executable.
7. **Redaction-safe support export.** Raw URLs and raw provider payloads are explicitly forbidden from crossing the support boundary; the packet cites its source schema and carries reopen context.
8. **Inspection projection.** A compact boolean projection surfaces every truth axis for CLI, inspector, and `operator_playbook_view` surfaces.

## Record kinds

| Record kind | Purpose |
|---|---|
| `git_review_timeline_packet` | Top-level packet consumed by review surfaces, operator playbook views, and support exports. |
| `git_review_timeline_truth_record` | Core record binding the review workspace, chronology state, and derived invalidation/blocking reasons. |
| `timeline_event_record` | One chronology event with explicit ordering, clock source, actor, source, lineage, freshness, and hosted-authority disclosure. |
| `operator_playbook_record` | An operator runbook with an ordered step set and derived blocking reasons. |
| `operator_playbook_step_record` | One runbook step with command class, authority class, preview/reversibility/checkpoint posture, and hosted-authority disclosure. |
| `git_review_support_export_packet` | Redaction-safe export preserving timeline and playbook lineage and citation. |
| `git_review_timeline_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Chronology states
- `chronology_current`, `chronology_stale`, `chronology_reconstructed`, `chronology_gap_detected`

### Clock source classes
- `local_monotonic`, `local_wall_clock`, `provider_reported`, `imported_bundle`, `reconstructed_from_lineage`

### Event source classes
- `local_git`, `review_workspace`, `provider_linked`, `browser_handoff`, `migration_import`, `ai_evidence`

### Event kinds
- `commit_recorded`, `review_state_transition`, `merge_landed`, `rebase_applied`, `checkpoint_created`, `provider_publish`, `import_applied`, `comment_posted`, `approval_recorded`, `approval_invalidated`

### Event freshness classes
- `current`, `stale`, `superseded`, `unverified`

### Operator playbook states
- `draft`, `ready`, `in_progress`, `completed`, `blocked`

### Playbook step command classes
- `inspect`, `preview`, `apply_with_checkpoint`, `revert`, `export`, `handoff`, `escalate`

### Playbook step authority classes
- `advisory_only`, `previewable_local_apply`, `checkpointed_reversible`, `requires_human_approval`, `hosted_provider_mutation`

### Invalidation reasons
- `chronology_gap`, `clock_source_unverified`, `event_unattributed`, `non_monotonic_ordering`, `playbook_step_authority_exceeded`, `support_export_stale`, `lineage_break`

## Invariants enforced at construction and validation

- Timeline events strictly increase by `sequence_index`; duplicate ids or sequence indices are rejected.
- Every event carries a non-empty `actor_ref` and a clock source from the closed vocabulary.
- Hosted/provider events must disclose hosted authority.
- Every `lineage_parent_ref` resolves to a known event.
- Every playbook step references a known playbook; step indices strictly increase within a playbook.
- Mutating steps must be previewable and reversible-or-checkpointed.
- A `hosted_provider_mutation` step must disclose hosted authority.
- An authority-broadening step can never be actionable, and it narrows the timeline below actionable.
- The support export keeps `raw_url_export_allowed` and `raw_provider_payload_export_allowed` false and cites `schemas/review/git_review_support_export_timeline.schema.json`.

## Schema and fixtures

- Schema: `schemas/review/git_review_support_export_timeline.schema.json`
- Fixtures: `fixtures/review/m4/finalize-git-and-review-support-export-packets-timeline/`
  - `chronology_current_with_playbook.json`
  - `reconstructed_timeline_from_lineage.json`
  - `blocked_authority_broadening_step.json`

## How to verify

```bash
cargo test -p aureline-review --test finalize_git_and_review_support_export_packets_timeline_alpha
```
