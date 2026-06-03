# Artifact: Finalize managed-workspace lifecycle truth

## Source

- Doc: `docs/policy/m4/finalize-managed-workspace-lifecycle-truth.md`
- Crate: `crates/aureline-policy/src/finalize_managed_workspace_lifecycle_truth/`
- Schema: `schemas/policy/managed-workspace-descriptor.schema.json`
- Contract ref: `policy:finalize_managed_workspace_lifecycle_truth:v1`

## Summary

This artifact certifies that the managed-workspace lifecycle proof packet:

1. Requires a complete `ManagedWorkspaceDescriptor` on every claimed row.
2. Enforces explicit provisioning-state vocabulary (queued, allocating, booting, attaching, sync_warming, ready, reconnecting, suspended, rebuilding, deleting, failed).
3. Requires suspend/resume checkpoints that state what persists, what drifts, and whether attach is same-live or resumed-snapshot.
4. Requires destructive-operation previews for rebuild, recreate, reset, delete, and extend-TTL flows.
5. Requires declared and qualified fallback paths with passed outage drills.
6. Requires explicit join mode and authority scope on share/handoff tokens.
7. Keeps persistence truth visible in degraded UI and support export.

## Seeded output

The seeded input produces a stable page with two workspace rows:

- `managed-workspace-lifecycle-row:alpha` — Ready state, full snapshot persistence, local and direct-remote fallback qualified, suspend/resume checkpoint present, rebuild plan present, same-live share token present.
- `managed-workspace-lifecycle-row:beta` — Suspended state, files-and-editor persistence, local fallback qualified, resume-to-snapshot checkpoint present.

Both rows qualify `stable` with zero defects.

## Acceptance criteria

- Managed-workspace fixtures prove users can distinguish same-live resume, resumed snapshot, fresh reprovision, reconnect, rebuild, and delete flows without hidden workspace recreation.
- Control-plane outage drills preserve local or direct-remote fallback where promised and keep data-plane truth, persistence class, and join mode visible in the degraded UI and support export.
- Suspend, resume, rebuild, and share flows can explain what persists, what is reprovisioned, what routes/endpoints change, and what policy or billing owner governs the action.
- Any managed-workspace lane lacking current lifecycle proof or fallback honesty narrows automatically below Stable in product copy, docs/help, and release packets.
