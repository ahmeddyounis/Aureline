# Finalize managed-workspace lifecycle states, suspend/resume/rebuild/share truth, and local or direct-remote fallback

## Scope

This document defines the stable policy proof packet for managed-workspace lifecycle truth. It validates that every claimed managed-workspace row can explain create, attach, resume, suspend, rebuild, and share state; persistence and billing ownership; and what local or direct-remote path remains when the control plane is unavailable.

## Vocabulary

### Provisioning states

| State | Meaning | User-facing rule |
|---|---|---|
| `queued` | Request accepted, waiting for capacity or policy check | Show queue state and non-destructive cancel path |
| `allocating` | Environment resources being reserved | Target region/tenant remains visible |
| `booting` | Image/VM/container starting | Logs or diagnostics link available |
| `attaching` | Transport/agent channel being established | No implication that the workspace is already interactive |
| `sync_warming` | File/index/session warmup in progress | Interactive status and reduced-capability cues explicit |
| `ready` | Claimed capabilities available | Active workspace card remains inspectable |
| `reconnecting` | Reconnect or reauth in progress | Fallback path visible if qualified |
| `suspended` | State preserved for later resume | Resume vs rebuild distinction remains visible |
| `rebuilding` | Rebuild in progress | Preserved vs reprovisioned state preview visible |
| `deleting` | Deleting in progress | Export-first warnings and policy locks visible |
| `failed` | Provisioning or runtime failed | Diagnostic refs and retry/cancel paths visible |

### Persistence classes

| Class | Meaning |
|---|---|
| `ephemeral` | No state survives stop |
| `files_and_editor_state` | Files and editor state persist across suspend/resume |
| `full_workspace_snapshot` | Full workspace snapshot including processes and terminals |
| `customer_managed_volume` | Customer-managed persistent volume |
| `policy_retention_with_expiry` | Policy-mandated retention with explicit expiry |

### Join modes

| Mode | Meaning | Must not do |
|---|---|---|
| `same_live` | Recipient joins the exact same live environment | Imply recipient gets an isolated copy |
| `resume_snapshot` | Recipient gets a resumed snapshot | Imply live-state continuity that does not exist |
| `fresh_reprovision` | Recipient triggers a fresh reprovision | Imply live-state continuity that does not exist |

### Fallback paths

| Path | Meaning |
|---|---|
| `not_declared` | No fallback is declared |
| `local_workspace` | Local folder/workspace remains available |
| `direct_ssh` | Direct SSH remote target remains available |
| `direct_container` | Direct container/devcontainer target remains available |
| `local_and_direct_remote` | Local and direct-remote both remain available |

## Audit conditions

The proof packet validates seven conditions:

1. **Descriptor completeness** — every claimed managed workspace carries a descriptor with workspace ID, org/tenant, region, persistence class, template version, quota/billing owner, secret model, and expiry policy.
2. **Lifecycle state explicit** — provisioning and runtime states are drawn from the closed vocabulary; no surface may collapse them into one generic loading banner.
3. **Suspend/resume checkpoint honesty** — every suspend or resume carries a checkpoint that names retained state classes, version drift, pinned routes/endpoints, and whether attach is to the same live environment or a resumed snapshot.
4. **Destructive-operation preview** — rebuild, recreate, reset, delete, and extend-TTL flows carry a plan that previews preserved state, reprovisioned state, route revalidation needs, and policy locks.
5. **Fallback path declared** — every profile that claims local or direct-remote fallback carries a declaration; control-plane outage drills keep the fallback visible.
6. **Share/handoff token explicit** — share or handoff tokens carry join mode (`same_live`, `resume_snapshot`, `fresh_reprovision`) and authority scope.
7. **Persistence truth visible in degraded UI** — control-plane outage drills keep persistence class, join mode, and data-plane truth visible in degraded UI and support export.

## Canonical types

The canonical Rust types live in `crates/aureline-policy/src/finalize_managed_workspace_lifecycle_truth/`.

The schema lives at `schemas/policy/managed-workspace-descriptor.schema.json`.

## Support export

The support-export envelope quotes the truth page plus a metadata-safe defect roll-up. Raw workspace IDs, tenant identifiers, region labels, and secret material stay outside the support boundary.
