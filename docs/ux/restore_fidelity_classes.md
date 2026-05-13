# Restore fidelity classes

This page is the UI-facing contract for restore fidelity labels. The
same provenance record drives startup recovery, restore summaries,
diagnostics, and support exports:

[`/schemas/state/restore_provenance_record.schema.json`](../../schemas/state/restore_provenance_record.schema.json)

## Controlled Labels

| UI label | Machine value | Use |
|---|---|---|
| Exact restore | `exact` | Everything selected for restore round-tripped without translation, placeholder fallback, review, or rerun downgrade. |
| Compatible restore | `compatible` | State restored through a declared translation path with rollback, equivalence, compare, and export refs. |
| Layout only | `layout_only` | Window topology, pane identity, and context returned, but live authority or dependencies did not return as live state. |
| Recovered drafts | `recovered_drafts` | Dirty buffers or local-history bodies reopened as drafts that need compare or accept/reject review. |
| Evidence only | `evidence_only` | Restore retained transcripts, snapshots, refs, and provenance only. No live surface was reopened. |

Do not introduce parallel labels such as `partial`, `best effort`,
`recovered`, or `manual review` for this surface. If the behavior does
not fit the closed set, extend the schema and this page in the same
change.

## Provenance Source

Every restore provenance record names the event family that produced the
record. The closed `source_event_class` values are:

| Value | Meaning |
|---|---|
| `auto_checkpoint` | Automatic checkpoint, crash recovery, suspend/wake recovery, or background continuity save. |
| `manual_export` | User-initiated export or portable package. |
| `backup` | Backup or support recovery bundle. |
| `sync` | Managed sync or profile sync snapshot. |
| `import` | Imported profile, layout bundle, or bridge artifact. |

## No-Rerun Downgrade Labels

Restore must never make a terminal, task, debug session, notebook
kernel, preview, remote tunnel, or credential-gated surface look live
by silently rerunning work or reacquiring authority. Use the typed
`restore_without_rerun_downgrades[]` rows for those surfaces.

| UI label | Machine value | Use |
|---|---|---|
| transcript restored; command not rerun | `transcript_only` | Terminal or task output returned as transcript/snapshot evidence only. |
| reconnect required | `reconnect_required` | Remote target, tunnel, kernel host, or endpoint requires explicit reconnect. |
| session ended; command not rerun | `session_ended` | Debug, notebook, terminal, or task session ended and was not reattached. |
| rerun required | `rerun_required` | Preview, task, notebook cell, or extension surface needs explicit rerun to become live. |
| credential unlock needed | `credential_unlock_needed` | Resume is gated by credentials or trust that restore cannot unlock silently. |
| layout adjusted for display change | `layout_adjusted_for_display_change` | Window bounds or placement were clamped after monitor/topology change. |
| resume review needed | `resume_review_needed` | Wake/reconnect/suspend state needs user review before live behavior resumes. |

When `runtime_survived` is false, the row must keep both
`command_rerun_forbidden` and `authority_reacquire_forbidden` set to
true.

## Surface Mapping

The UI reads one provenance object and projects it consistently:

| Surface | Required projection |
|---|---|
| Startup recovery | Show the controlled fidelity label and any missing dependency or no-rerun labels before hydration. |
| Restore summary | Repeat the same fidelity, source event, compare/export refs, and downgrade labels after restore. |
| Diagnostics | Expose the same placeholder cards, preserved artifacts, and migration notes for investigation. |
| Support export | Include the same provenance row so support can explain fidelity downgrades without rehydrating state. |

Fixtures live in
[`/fixtures/workspace/restore_fidelity_cases/`](../../fixtures/workspace/restore_fidelity_cases/).
