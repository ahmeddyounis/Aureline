# Session-Scoped Restore Fidelity

Session restore is allowed to bring back layout, local state, transcripts,
checkpoints, and static evidence. It is not allowed to silently rerun commands,
reattach debug or remote authority, resume notebook kernels, restart preview
servers, or imply that a runtime survived when it did not.

The shared schema is
`schemas/recovery/session-restore-fidelity.schema.json`. Runtime-backed panes
and support surfaces consume the same packet instead of minting local
vocabularies.

## Controlled Classes

The controlled restore-fidelity labels are:

| Token | Label | Meaning |
| --- | --- | --- |
| `exact_restore` | Exact restore | Nothing downgraded; no placeholder, translation, or review. |
| `compatible_restore` | Compatible restore | Meaning preserved through a declared compatibility path. |
| `layout_only` | Layout only | Layout and context restored without live authority. |
| `recovered_drafts` | Recovered drafts | Dirty drafts recovered for compare and explicit save. |
| `evidence_only` | Evidence only | Only evidence, transcripts, snapshots, and provenance survived. |

## Placeholder Matrix

The placeholder-state matrix covers:

| Surface | Restore State | Fidelity | Required Truth |
| --- | --- | --- | --- |
| Terminal session | `transcript_restored` | Evidence only | Transcript restored; command not rerun. |
| Task run | `rerun_required` | Layout only | Last run context restored; task waits for explicit rerun. |
| Debug session | `session_ended` | Layout only | Adapter and target are not silently reattached. |
| Notebook kernel | `rerun_required` | Recovered drafts | Drafts and outputs are visible; kernel is not resumed. |
| Preview server | `static_evidence_only` | Evidence only | Last preview evidence is shown without restarting the server. |
| Remote tunnel | `reconnect_available` | Compatible restore | Last-known metadata remains visible; reconnect requires review. |

Every runtime-backed row preserves its pane slot, keeps stale evidence or
last-known metadata visible, forbids auto-rerun, forbids silent reattach,
forbids hidden authority reacquisition, and names the target/runtime before any
rerun or reconnect action.

## Consumer Contract

Terminal, task, debug, notebook, preview, remote, diagnostics, and support/export
surfaces must consume:

- the shared schema ref;
- the placeholder-state matrix;
- the no-hidden-rerun drill corpus;
- the support/export projection.

Any stable row that cannot satisfy those requirements is downgraded below stable
until it can show a truthful placeholder state and explicit next action.

## Support Export

Support exports carry the same placeholder rows and drill refs by metadata-safe
reference. Raw command lines, raw output bodies, raw paths, raw URLs, secrets,
live handles, approval tickets, and authority tokens are excluded by default.

