# Title/context bar identity contract

This contract freezes the canonical identity tuple behind Aureline's
title/context bar, native window title, workspace status item, and
support/export representation. It exists so workspace name, root or repo
identity, branch, trust, host, profile, route state, and degraded or
recovery state are read from one state object instead of being recomputed
per surface.

Machine-readable companions:

- [`/schemas/ux/title_context_bar_state.schema.json`](../../schemas/ux/title_context_bar_state.schema.json)
  defines the boundary record for the canonical state object.
- [`/fixtures/ux/title_context_bar_examples/`](../../fixtures/ux/title_context_bar_examples/)
  contains worked restricted, partial-open, detached-repo, multi-root,
  missing-host, and mixed local-plus-remote examples.

Runtime wiring:

- Canonical record projection: `crates/aureline-shell/src/chrome/title_context_bar.rs`
- Live shell consumer: `crates/aureline-shell/src/bootstrap/native_shell.rs`
- Latest runtime snapshot (written opportunistically): `.logs/ux/title_context_bar_state.json`

This contract composes with, and does not replace:

- [`/docs/ux/shell_zone_and_density_contract.md`](./shell_zone_and_density_contract.md)
  for shell-zone ids and collapse behavior.
- [`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  for transport posture, deployment profile, identity mode, and
  environment status strip semantics.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for trust-state vocabulary and restricted-mode guarantees.
- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  for route truth and authority-delta rules.
- [`/docs/ux/host_identity_contract.md`](./host_identity_contract.md)
  for the detailed host identity chip, path-truth presentation,
  boundary-change banner, copy-path/run-here labels, and host lineage
  that project into this tuple's `host_identity` group.
- [`/docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  for degraded-state tokens.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  for export and redaction posture.

Where this document disagrees with those upstream sources, the upstream
source wins and this document plus the schema and fixtures must update
in the same change.

## Scope

This contract covers one record:
`title_context_bar_state_record`.

The record answers four questions:

1. What workspace or root is active?
2. Which repository, branch, trust state, host, profile, and route state
   are currently in force?
3. Is any part of that identity degraded, partial, missing, restricted,
   or in recovery?
4. Which canonical fields are shown in the title/context bar, native
   window title, workspace status item, and support/export packet?

Out of scope: final visual styling, platform-specific native titlebar
implementation, window-control placement, and renderer crate types.

## Canonical identity tuple

Every shell window with an open or opening workspace owns exactly one
current `title_context_bar_state_record`. The tuple contains these
field groups:

| Group | Required fields | Purpose |
| --- | --- | --- |
| `workspace_identity` | workspace/root label, workspace kind, lifecycle state, root summary | Names the current work boundary, even when only part of it is ready. |
| `repo_identity` | repo state, repo ref, branch label/ref, revision ref when applicable | Names VCS context without implying repo metadata exists when detached or missing. |
| `trust_identity` | trust state, source, review action, last change | Keeps restricted, trusted, degraded, and identity-gated postures visible. |
| `host_identity` | host class, host state, target label/ref, boundary note | Distinguishes local, SSH, container, managed, browser bridge, service-plane, mixed, and unknown host truth. |
| `profile_identity` | profile label/ref, profile mode, deployment profile, identity mode | Shows which profile and deployment envelope are changing behavior. |
| `route_state` | route kind, route ref, route label, authority delta, freshness | Names whether the current shell is ordinary editing, entry/restore, remote attach, deep-link review, support export, or recovery. |
| `degraded_or_recovery_state` | degraded tokens, recovery mode, last-failure ref, recovery action refs | Carries partial, stale, offline, policy, read-only, restricted, and recovery truth. |
| `field_visibility` | per-field visibility and overflow placement | Defines what is primary visible, condensed, inspector-only, native-title-only, support-export-only, redacted, or not applicable. |
| `surface_projections` | field bindings for every consuming surface | Proves the title/context bar, native title, status item, and support/export representation are projections of the same tuple. |

No surface may add a private "current workspace", "current branch",
"current host", "current trust", "current profile", or "current route"
field outside this tuple. If a new identity axis becomes necessary, it
is added to the canonical record first and then projected to surfaces.

## Field visibility

The backing tuple always keeps the identity fields distinct. Rendering
may compress them, but compression changes visibility, not meaning.

| Visibility | Meaning |
| --- | --- |
| `primary_visible` | Rendered directly in the title/context bar in normal chrome. |
| `condensed_visible` | Rendered as a chip, icon+accessible label, abbreviated text, or overflow summary in compact chrome. |
| `inspector_only` | Not directly shown in the title/context bar, but reachable from the workspace status item or detail inspector. |
| `native_title_only` | Included in the platform window title but not direct shell chrome. |
| `support_export_only` | Serialized for support/export but intentionally not visible in live chrome. |
| `hidden_redacted` | Present only as an opaque ref or redacted marker because live or export policy forbids display. |
| `omitted_not_applicable` | Absent because the field does not apply to the current workspace. |

Required primary or condensed identity:

- workspace/root label;
- workspace lifecycle state when not `workspace.ready`;
- trust state when not ordinary trusted local editing;
- host class when not ordinary local editing, when details are missing,
  or when host state is degraded;
- active profile when it is temporary, safe mode, imported, or policy
  managed;
- route state when the route is entry, restore, deep-link review,
  remote attach, support export, or recovery;
- degraded or recovery state when any degraded token is active.

Repository and branch fields are visible when a single attached repo is
authoritative. In multi-root or detached metadata states they may be
condensed into a summary, with per-root/per-repo detail moved to the
workspace status item or inspector. The title/context bar must not
invent one "current branch" for a mixed workspace.

## Surface projections

The `surface_projections` block binds every consumer to canonical field
paths.

| Surface | Required projection |
| --- | --- |
| `title_context_bar` | Glanceable identity and boundary changes: workspace/root label, non-ready lifecycle state, repo/branch summary when applicable, trust, host, profile, route, degraded/recovery state. |
| `native_window_title` | Privacy-safe platform title derived from workspace/root label plus high-risk boundary labels. It may be shorter, but it must not contradict shell chrome. |
| `workspace_status_item` | Inspectable status entry that opens the narrowest useful detail surface and exposes all canonical identity fields not directly visible. |
| `support_export` | Redacted evidence representation preserving canonical tokens, timestamps, refs, and projection field list so a support reader can reconstruct what the shell showed. |

Projection rules:

1. Every projection lists canonical `field_paths`. Private surface-local
   fields are non-conforming.
2. A projection may omit only fields whose `field_visibility` is
   `omitted_not_applicable`, `hidden_redacted`, or not assigned to that
   surface.
3. The support/export projection preserves the same token values shown
   in live chrome. It may redact labels, but it may not replace
   `restricted`, `mixed_local_plus_remote`, or `workspace.partially_ready`
   with generic prose.
4. Native window titles are lower-fidelity projections. They must carry
   enough identity to avoid unsafe task switching, but they do not own
   canonical truth.

## State-change latency

Host, trust, profile, and route-state changes are boundary changes. The
state store must publish them before or in the same commit that admits
the behavior change. The title/context bar and workspace status item
must refresh on the next shell frame after the subscription event. The
native window title may follow on the next platform-title update, but it
must not lag long enough to make a restricted, remote, recovery, or
profile-switched workspace look like ordinary local editing.

Support/export captures snapshot the canonical record with
`updated_at`, and any export created during a transition must either
capture the old record before the transition or the new record after the
transition. Mixed old/new field sets are non-conforming.

## Forbidden occupants

The title/context bar is identity chrome. These occupants are
non-conforming:

- transient alerts, repeated progress pings, or toast-equivalent errors;
- tutorial, onboarding, marketing, release-note, or account-nag content;
- workflow-local action strips for AI, Git, test, debug, notebooks, or
  extensions;
- extension branding or vanity status that displaces first-party
  boundary state;
- generic warning badges that do not cite canonical trust, host,
  profile, route, degraded, or recovery fields.

Workflow surfaces may expose actions in the main workspace, inspector,
status item drawer, command palette, or contextual banner. They may not
take over the title/context bar unless the action changes the canonical
identity tuple itself.

## Required examples

The fixture corpus covers the acceptance-critical cases:

- restricted mode;
- partial open;
- detached repository metadata;
- multi-root workspace with mixed repo/trust state;
- missing host details;
- mixed local-plus-remote session.

Each example proves that the same canonical tuple can render through
title chrome, native window title, workspace status entry, and
support/export without inventing surface-local fields.
