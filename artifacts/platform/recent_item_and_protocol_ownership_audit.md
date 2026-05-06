# Recent-item + protocol-handler ownership audit packet

Reviewer-facing audit packet for the desktop entry points that are most likely to drift under side-by-side installs, portable mode, and OS-level affordances:

- recent-item registration (OS recents, dock/taskbar recents, launcher recents)
- protocol handlers (deep links, auth callbacks, review/work-item links)
- file associations (workspace manifests and other file-extension classes)
- notification click-through and badge activation
- dock/taskbar/jump-action “open” affordances

The goal is to prevent:

- **last-writer-wins ownership** (a newer install silently taking over the canonical handler),
- **wrong-target reopen** (a reopen path that lands on the wrong channel or generic Start Center),
- and **silent drift** (users discovering handler changes only after something breaks).

This packet is narrative. It does not mint vocabulary. The machine-readable source-of-truth lives in the companion ledgers below.

## Companion ledgers + fixtures (source of truth)

| Artifact | Role |
|---|---|
| `artifacts/platform/file_association_ownership_matrix.yaml` | Machine-readable ownership matrix for file associations, recent-item registration, protocol handlers, and notification click-through across side-by-side relations. |
| `artifacts/platform/protocol_handler_ownership_matrix.yaml` | Protocol-handler focused view (schemes, shared default, and reopen drills) that references the full matrix row ids. |
| `artifacts/platform/system_affordance_route_audit.md` | Route table binding every OS-facing entry path to a canonical command, target identity, replay posture, and bounded fallback. |
| `artifacts/platform/desktop_summary_surface_matrix.yaml` | Profile-level contract for dock/taskbar/launcher summary surfaces (recents and jump actions) and their exact-target reopen rules. |
| `artifacts/release/channel_ownership_audit.yaml` | Side-by-side coexistence audit: which channel owns recent items, protocol-handler schemes, and file-association candidates per relation. |
| `artifacts/release/state_root_map.yaml` | Per-channel placeholders for recent-item namespaces and protocol-handler scheme placeholders, plus shared-scheme resolution rules. |
| `artifacts/release/install_update_about_truth_packet.md` | Install/update/About disclosures; installer summary must reveal handler ownership during install/update, not after. |
| `artifacts/release/portable_mode_limitations.yaml` | Portable-mode forbidden host mutation rules for file associations, protocol handlers, shell recents, and shell hooks. |

Fixtures (worked drills):

- `fixtures/platform/system_affordance_cases/` — OS-level cases (file association open, notification click-through, etc).
- `fixtures/platform/deep_link_replay_cases/` — deep-link replay-deny proofs (expired/consumed/origin-mismatch/target-drifted).
- `fixtures/platform/exact_target_reopen_cases/` — exact-target reopen proofs (success or fail-closed with bounded recovery) across local files, workspaces, remote targets, review handoff, and auth callbacks.

## Ownership model (what must stay true)

### 1) Recents are **per channel**

- Side-by-side channels must **not merge** OS-visible recents into one list.
- Each recent entry must remain attributable to the **owning channel** so reopening does not “wander” into the wrong channel.
- Portable mode may keep *portable-local* recent state, but must **not write** a machine-global recent-items list on the host.

The authoritative rows are in `recent_item_rows` inside `artifacts/platform/file_association_ownership_matrix.yaml`.

### 2) Protocol handlers are **per-channel suffixed schemes** plus an explicit shared default

Protocol scheme ownership is split into two classes:

- **Per-channel suffixed schemes** (e.g. `aureline-stable://`, `aureline-preview://`): always unambiguous and never require a shared-default choice.
- **Shared default scheme** (`aureline://`): resolves only via an explicit user/admin selected default, and any change is **preview-required** (or **blocked by policy** under managed locks).

The authoritative rows are in `protocol_handler_rows` inside `artifacts/platform/file_association_ownership_matrix.yaml`, with a protocol-only view in `artifacts/platform/protocol_handler_ownership_matrix.yaml`.

### 3) File associations are **candidate registrations**, not silent takeovers

Where multiple channels can coexist:

- channels register as **candidate handlers**,
- the default is **user/admin selectable**, and
- ownership changes must be **previewable** (never last-writer-wins).

Portable must not register file associations on the host; it can still open the same targets *inside* the portable session via in-product commands.

The authoritative rows are in `file_association_rows` inside `artifacts/platform/file_association_ownership_matrix.yaml`.

### 4) Notifications + badges reopen via **event lineage**, not ad hoc routing

Notification click-through must reopen the **durable object** (review thread, activity event, run) via event lineage. Quick actions must not mutate state without routing through the in-product review path.

The authoritative rows are in `notification_badge_rows` inside `artifacts/platform/file_association_ownership_matrix.yaml`.

## Preview-before-change rules (install / update / import)

Ownership drift is only acceptable when it is **previewed** and **attributable**:

1) **Install neighbor channel** (side-by-side)
   - Must surface an ownership review sheet for shared defaults (file association default + shared protocol scheme).
   - Must not rewrite recent-item registration into a merged list.

2) **Update/repair that would change ownership**
   - Must preview the proposed owner changes before applying them.
   - Must not retroactively flip existing deep links/recents to a different handler without review.

3) **Import settings/profile from another channel**
   - Must preview whether the import would alter: shared protocol default, default file associations, or notification/badge affordances.
   - Default is “import preferences without changing OS ownership”; explicit reassignment requires review.

4) **Uninstall the current default owner**
   - Must preview the post-uninstall handler state and provide bounded choices (keep current, reassign, or learn more).
   - Must not silently rebind to a different channel.

The drill rows for install/update/uninstall/policy-lock live in `ownership_change_drills` inside `artifacts/platform/file_association_ownership_matrix.yaml`.

## Exact-target reopen contract (success or fail closed)

Every OS-facing reopen path must either:

- resolve to the **exact target** (workspace manifest, recent-work entry, review thread, managed workspace, auth session), *or*
- **fail closed** with one bounded recovery action (review sheet, locate, cached context, activity center, default browser restart), never a silent redirect.

Worked proofs live in:

- `fixtures/platform/exact_target_reopen_cases/` (success + unavailable-target cases)
- `fixtures/platform/deep_link_replay_cases/` (replay-deny cases)

If a target is stale/moved/unavailable, the reopen path must keep the literal intent visible (origin, source surface, route class, target identity, handler owner, replay posture) and route to recovery without widening authority.
