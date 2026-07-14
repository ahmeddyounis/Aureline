# M5 acquisition-evidence and partial-recovery registries

This lane is the evidence-packet + resumable-acquisition implement lane over the frozen
[M5 repository-bootstrap matrix](./m5_repository_bootstrap_contract.md). It turns the *acquisition-evidence*
grammar (how Aureline records the clone / fetch transcript, the warnings and retries, the resulting root
identity, the omitted-or-unfetched state, and the current bootstrap checkpoint of an acquisition path) and the
*partial-recovery* grammar (typed recovery actions that resume an interrupted acquisition, discard partial state,
open the partial root read-only, or merely report status) into registry resolvers that produce export-safe, honest
projections, so the acquisition, git, trust, diagnostics, docs, CLI, and support surfaces resolve one canonical
evidence and recovery truth instead of a per-entry, hand-copied reconstruction. The acquisition evidence and the
partial recovery are separated in runtime and serialized state: the transcript reference, the warnings-and-retries
reference, the resulting-root-identity reference, the omitted-or-unfetched reference, the bootstrap-checkpoint
reference, and the evidence provenance live on the acquisition evidence, while the recovery-action kind, the
recovery site, the state consequence, the lineage consequence, the explicit-action requirement, and the
attribution reference live on the partial recovery, and no recovery action may discard partial state or transcript
lineage during acquisition so a partial or interrupted acquisition stays visible and recoverable rather than
reading as a healthy full checkout.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_acquisition_evidence_and_partial_recovery_registries` (the authoritative validator).
- **Combined schema:**
  `schemas/workspaces/m5-acquisition-evidence-and-partial-recovery-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/workspaces/m5-checkout-plan.schema.json`](../../schemas/workspaces/m5-checkout-plan.schema.json)
  (checkout topology and submodule / LFS hydration reviewed before mutation) and
  [`schemas/workspaces/m5-bootstrap-evidence.schema.json`](../../schemas/workspaces/m5-bootstrap-evidence.schema.json)
  (acquisition evidence and resumable recovery) as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-acquisition-evidence-and-partial-recovery-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:**
  `fixtures/workspaces/m5-acquisition-evidence-and-partial-recovery-registries/`
  (`resume_partial_beta_narrowed.json`, `discard_cleanup_preview_narrowed.json`).

## Two registries

1. **Acquisition evidence** (`resolve_acquisition_evidence_entry`) — publishes one stable acquisition-evidence
   packet per acquisition path: the evidence kind and canonical evidence mode, the clone / fetch transcript
   reference, the warnings-and-retries reference, the resulting-root-identity reference, the omitted-or-unfetched
   reference, the bootstrap-checkpoint reference, and the evidence provenance. A clean entry names a canonical
   registry token, a classified evidence kind, and a repository-bootstrap role, covers the canonical / accessible /
   audit resolution forms, publishes a complete packet, keeps a partial or interrupted acquisition visible, and
   discloses partial-not-full status before any partial-describing packet. Otherwise it degrades honestly — a
   packet that would present partial content as a healthy full checkout before partial-not-full status is disclosed
   degrades to `evidence_overclaims_full_checkout_or_hides_partial_state`.
2. **Partial recovery** (`resolve_partial_recovery_entry`) — keeps the recovery action safe. A clean entry names a
   classified recovery class and provides the complete recovery-action-kind / recovery-site / state-consequence /
   lineage-consequence / explicit-action-requirement / attribution recovery object; a state-mutating action that
   would discard partial state during acquisition, run ungated without an explicit discard or cleanup action, or
   hide what it would do and where degrades to
   `partial_recovery_discards_state_or_lineage_without_explicit_action`.

## Per-item partial-recovery reference

The recovery class carries whether it is state-mutating, and the resolver publishes the full recovery object, so
the registry — never a hand-copied per-entry assumption — is the single source of truth.
`acquisition_evidence_object_is_complete` rejects a packet missing any field,
`acquisition_evidence_discloses_partial_state` rejects a packet that presents partial content as full before
partial-not-full status is disclosed, and `partial_recovery_action_preserves_lineage` rejects an action that
discards partial state during acquisition or runs a state-mutating step ungated.

| recovery class | recovery action kind | recovery site | state consequence | lineage consequence | explicit action requirement |
| --- | --- | --- | --- | --- | --- |
| resume_acquisition | `recovery-action.resume-from-checkpoint` | `site.worktree` | `consequence.continues-partial-state` | `consequence.preserves-transcript-lineage` | `action.explicit-resume-required` |
| discard_partial_state | `recovery-action.discard-and-clean` | `site.git-dir` | `consequence.removes-partial-state` | `consequence.archives-transcript-lineage` | `action.explicit-discard-required` |
| open_read_only_partial_root | `recovery-action.open-partial-root-read-only` | `site.presentation-only` | `consequence.no-state-change` | `consequence.preserves-transcript-lineage` | `action.none-read-only` |
| inert_status_report | `recovery-action.report-partial-status` | `site.presentation-only` | `consequence.no-state-change` | `consequence.preserves-transcript-lineage` | `action.none-inert` |

A state-mutating action that discards state without an explicit action degrades to
`partial_recovery_discards_state_or_lineage_without_explicit_action`, an incomplete evidence packet degrades to
`evidence_packet_incomplete`, and a partial content presented as full degrades to
`evidence_overclaims_full_checkout_or_hides_partial_state`, so a state-discarding action, an incomplete packet, or
an overclaimed full checkout can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Partial or interrupted acquisition remains visible and recoverable rather than looking like missing or
  unsupported data.** Clean evidence entries cover the canonical clone-fetch-transcript / warnings-and-retries /
  resulting-root-identity / omitted-or-unfetched-state / bootstrap-checkpoint kinds and the first shell / entry /
  diagnostics / admin / support surfaces, a packet-incomplete example degrades, an overclaim example degrades, and
  no clean evidence entry presented partial content as full or published an incomplete packet.
- **Evidence packets let support explain what Aureline fetched, skipped, retried, or left partial for the affected
  repository path.** Clean evidence entries publish the transcript, warnings-and-retries, resulting-root-identity,
  omitted-or-unfetched, checkpoint, and provenance fields with full resolution-form coverage.
- **No acquisition failure path discards partial state or transcript lineage without an explicit discard or
  cleanup action.** A recovery action that discards state without an explicit action degrades, a form-incomplete
  example degrades, and no clean recovery entry discards state implicitly or is missing the complete recovery
  object. Clean recovery entries cover the resume / discard / open-read-only / inert-status classes with full
  resolution-form coverage while providing the complete recovery object.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- support-export
cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- csv
cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- report
cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- partial-recovery-table
cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- fixture-resume-partial-beta-narrowed
cargo run -p aureline-ui --example dump_m5_acquisition_evidence_and_partial_recovery_registries -- fixture-discard-cleanup-preview-narrowed
```
