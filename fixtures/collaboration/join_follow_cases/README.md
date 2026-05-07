# Collaboration join-manifest + shared-debug follow worked-example corpus

This directory holds worked examples for the join-manifest and
guest-visible session-manifest contract frozen in:

- `artifacts/collaboration/join_shared_debug_sequence.md`
- `schemas/collaboration/session_manifest.schema.json`

The scenarios in this corpus are intentionally **pre-implementation**.
They exist so collaboration UI, debug UI, policy packets, and
chronology/audit projections can share the same **state** and **action**
names without inventing per-surface variants.

Each YAML file contains:

- a `__fixture__` summary (scenario + exercised contracts), and
- a `records` array containing a mixed set of record kinds.

Record kinds are validated by their own boundary schemas:

- join/session manifest: `schemas/collaboration/session_manifest.schema.json`
- retention/recording disclosure: `schemas/collaboration/session_policy_manifest.schema.json`,
  `schemas/collaboration/recorded_artifact_row.schema.json`,
  `schemas/collaboration/delete_path_status.schema.json`
- follow/presenter: `schemas/collaboration/follow_and_presenter_state.schema.json`
- explicit control: `schemas/collaboration/control_grant.schema.json`
- shared debug view stream: `schemas/execution/debug_session.schema.json`

No fixture embeds raw buffer text, raw terminal bytes, raw debug payloads,
raw URLs, raw absolute paths, raw user identifiers, raw billing-account
ids, raw API keys, raw OAuth tokens, raw mTLS material, raw model weights,
raw pack bytes, or raw provider payloads.

## Cases

- `join_denied_policy_denies.yaml` — guest sees retention + control posture pre-join; policy denies join.
- `join_approved_view_only_follow_and_debug_view.yaml` — join approved, guest remains view-only; follow enabled and shared debug overlay is inspect-only.
- `control_requested_granted_revoked.yaml` — control request triggers a consent event; an explicit debug-lane grant is minted then revoked (immediate, non-replayable).
- `retention_recording_change_mid_session.yaml` — retention/recording posture changes mid-session with re-consent; disclosure stays reconstructable.

