# Notebook collaboration follow and presenter state with live-versus-captured runtime disclosure — Artifact

## Packet reference

- **Packet file**: `implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure.json`
- **Schema file**: `schemas/notebook/implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure.schema.json`
- **Crate module**: `aureline-notebook::implement_notebook_collaboration_follow_and_presenter_state_with_live_versus_captured_runtime_disclosure`
- **Schema version**: `1`
- **Record kind**: `notebook_collaboration_follow_presenter_packet`

## Coverage

This packet covers the closed vocabularies and worked examples for:

- `NotebookCollaborationFollowState` — per-participant follow posture
- `NotebookPresenterState` — presenter identity, mode, and shared scope
- `NotebookRuntimeDisclosure` — live-versus-captured boundary disclosure

## Downgrade and truth invariants enforced

- Breakaway and degraded follow states require an explicit explanation.
- Active presenters require at least one shared cell or output ref.
- Live and stale runtime disclosures require a kernel session ref.
- Captured output disclosures require a capture timestamp.

## Consumer contract

Downstream docs, help, support exports, and CI surfaces MUST ingest this packet
instead of cloning status text. The packet is embedded in the crate via
`include_str!` and parsed at runtime by
`current_notebook_collaboration_follow_presenter_packet()`.
