# Workspace Archetype Detection Contract

This contract freezes the product truth for workspace-archetype
detection, readiness preflight, admission checkpoints, and first-useful-
work routing. It composes with the entry / restore object model in
[`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md),
the source-acquisition and bootstrap seed in
[`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../workspace/source_acquisition_and_bootstrap_seed.md),
and the workflow-bundle / certified-archetype model described in the
UI/UX specification.

The machine-readable schema lives at:

- [`/schemas/workspace/archetype_detection.schema.json`](../../schemas/workspace/archetype_detection.schema.json)

The worked fixtures live under:

- [`/fixtures/workspace/entry_routes/`](../../fixtures/workspace/entry_routes/)

This document is normative at the UX, schema, support-export, and
CLI/headless boundary. Where it disagrees with the PRD, architecture,
technical design, UI/UX specification, or style guide, those documents
win and this contract must update in the same change.

## Scope

- Freeze the detection outcomes Aureline may surface after a project
  entry action.
- Freeze source-labeled detection signals when they materially affect
  trust, support language, setup recommendations, policy treatment, or
  route selection.
- Freeze readiness buckets and admission-checkpoint fields so blocked
  setup, optional guidance, and user preference never collapse into one
  onboarding list.
- Freeze first-useful-work route expectations for single-file open,
  folder or repo open, clone, review or incident deep link, restore last
  session, and imported state / handoff packet.
- Freeze `Set up later` and remembered-routing behavior so a preference
  can narrow or defer behavior, but never silently widens authority.

Out of scope: detector implementation, setup execution, package
installation, language-server activation, environment materialization,
or final microcopy.

## Product Invariant

Every admission surface separates four kinds of truth:

1. **Detected facts** are observations about the workspace or entry
   packet, with signal refs and freshness.
2. **Recommendations** are proposed bundles, layouts, setup actions, or
   workflow entries derived from facts.
3. **Policy** is the effective trust, admin, network, or entitlement
   constraint that can narrow or block actions.
4. **User choice** records explicit current or remembered decisions,
   including `Set up later`, `Open minimal`, and dismissed suggestions.

A surface that says "configured", "supported", "ready", or "blocked"
without making those four layers inspectable is non-conforming.

## Detection Outcomes

| Outcome | Meaning | Required posture | Forbidden posture |
| --- | --- | --- | --- |
| `certified_archetype_match` | Signals match a shipped or certified workflow strongly enough to make a scoped support claim. | Show archetype ref, support class, evidence age, signal refs, compatible bundle refs, and same-weight bypass actions. | Presenting broad language/framework support without scope or freshness. |
| `probable_archetype` | Strong signals exist, but certification, evidence, or scope is incomplete. | Show confidence, source signals, recommendation basis, and equal access to plain open. | Auto-installing bundles, extensions, or dependencies as if the match were certain. |
| `mixed_or_ambiguous_workspace` | Multiple roots, stacks, or language/framework signals compete. | Offer boundary choices such as whole repo, probable project, current folder only, or workset. | Forcing one framework view on a mixed repo. |
| `unknown_or_generic_workspace` | No strong claim is available. | Land in generic shell, keep editing/search available, and offer diagnostics as non-blocking guidance. | Suggesting the workspace is unsupported or broken. |
| `restricted_or_policy_blocked` | Useful setup exists, but trust or policy prevents one or more actions. | Separate blocked setup from optional guidance and preserve restricted-mode continuation. | Hiding policy cause behind a generic setup failure. |
| `missing_prerequisite` | Readiness depends on a runtime, package manager, container, kernel, or remote agent that is unavailable. | Name the prerequisite, boundary, what still works now, and whether the repair is local, remote, or managed. | Blocking ordinary file reading/editing behind the prerequisite. |

## Detection Signals

Detection signals are source-labeled when they materially affect trust,
support language, recommendations, policy, or route selection. Required
source classes are:

- `manifest` for project or tool manifests such as package, Cargo,
  module, notebook, or task declarations.
- `bundle_marker` for workflow-bundle or certified-pack markers.
- `workspace_file` for Aureline or third-party workspace / workset
  files.
- `import_packet` for portable state, handoff, migration, review, or
  support packets.
- `admin_policy` for signed policy bundles, fleet policy, entitlement,
  network, or managed-workspace rules.
- `extension_contribution` for detector or workflow hints contributed
  by installed extensions.
- `previous_user_choice` for remembered routing, deferred setup, root
  selection, or dismissed recommendations.

Additional source classes may appear for lower-trust facts, but they do
not replace the required labels above. Signal summaries must be redacted
and export-safe; raw absolute paths, raw URLs with credentials, raw
manifest bodies, command lines, secrets, or package-lock contents do not
cross this boundary.

## Readiness Buckets

Readiness work is grouped into exactly three buckets:

- `blocking_now` contains tasks that prevent the requested first useful
  surface or a required safety review. Examples: trust review for a
  mutating setup step, policy block, missing remote agent for a live
  deep link, or import compare required before restore.
- `recommended_soon` contains tasks that improve fidelity but must not
  monopolize plain editing or safe inspection. Examples: dependency
  restore, extension recommendation, package manager selection, index
  warmup, or devcontainer build.
- `optional_later` contains additive setup. Examples: docs import, AI
  context warmup, nonessential extension install, optional test
  discovery, or layout customization.

Every readiness task names:

- task id and class;
- bucket;
- current state;
- execution boundary (`local_machine`, `container`, `remote_agent`,
  `managed_workspace`, `browser_handoff`, or `no_execution`);
- side effects, including whether it reads, writes, installs, downloads,
  starts processes, contacts remotes, widens trust, or widens network;
- source signal refs and affected recommendation / policy refs;
- a user-visible continuation or repair hook.

Blocked setup and optional guidance are different objects. An admission
checkpoint that has a blocked setup item may still carry optional
guidance, but the rendered UI must keep them visually and structurally
distinct.

## Admission Checkpoint

Every post-entry detection result emits one admission checkpoint with at
least:

- `admission_checkpoint_id`
- `entry_action_ref`
- `entry_source`
- `target_kind`
- `resulting_mode`
- `root_identity_ref`
- `workspace_scope_ref`
- `trust_state`
- `admission_class`
- `detected_fact_refs`
- `recommendation_refs`
- `policy_block_refs`
- `blocked_setup_refs`
- `optional_guidance_refs`
- `continue_without_action`
- `plain_open_available`

The checkpoint is the support, CLI, and UI explanation point for why
Aureline opened a file, offered a trust review, suggested a bundle,
blocked setup, or routed to a restore/import sheet. It never records a
trust grant merely because setup was deferred or a route was remembered.

### Admission Examples

**Blocked setup distinct from optional guidance**

- Detection outcome: `restricted_or_policy_blocked`
- Blocking task: `dependency_restore` with
  `state = blocked_by_policy`, execution boundary `local_machine`,
  side effects `installs_packages` and `contacts_remote`
- Optional guidance: `index_warmup` with side effect
  `reads_workspace`
- Admission action: `continue_in_restricted_mode`

The surface may offer "Continue in restricted mode" and show that file
reading, editing, and local diagnostics remain available. It must not
render the optional index task as blocked by policy, and it must not
hide the package-install block as a generic failure.

**Missing prerequisite without implying broken state**

- Detection outcome: `missing_prerequisite`
- Blocking task: `remote_agent_reconnect` with
  `state = missing_prerequisite`
- Available now: `linked_review_incident_or_work_item` opens read-only
  review context; clone/open remains a separate step
- Optional guidance: local clone or handoff export

The surface says the live remote action is unavailable, not that the
review link or workspace is unsupported.

**Generic workspace remains honest**

- Detection outcome: `unknown_or_generic_workspace`
- Blocking tasks: none
- Recommended tasks: diagnostics and optional index warmup
- First useful route: generic shell or editor/explorer

The surface avoids framework badges, avoids "unsupported" copy, and
keeps ordinary editing available.

## First-Useful-Work Route Matrix

| Entry source | Preferred first useful surface | Required explanation |
| --- | --- | --- |
| `single_file_open` | `file_editor_with_root_cues` | Whether the file is standalone, inside a detected repo, or a workspace candidate. |
| `folder_or_repo_open` | `explorer_plus_readme_or_changed_files` or `nested_root_choice_sheet` | Why the landing was chosen and how to switch to plain explorer or a start page. |
| `repository_clone` | `post_clone_handoff` | What was materialized locally and what intentionally did not run. |
| `review_or_incident_deep_link` | `linked_review_incident_or_work_item` | What object was anchored and whether clone/open is still required for full interaction. |
| `restore_last_session` | `restored_layout_with_placeholders` | What restored exactly, partially, or evidence-only, and what needs review. |
| `imported_state_or_handoff_packet` | `import_compare_or_restore_sheet` | Whether the packet is exact, compatible, layout-only, or evidence-only. |

Route decisions must be reversible through switch options such as
`open_plain_explorer`, `open_last_file`, `open_readme`,
`open_changed_files`, `review_trust`, `choose_root_or_workset`,
`compare_import`, or `open_minimal`.

## Set Up Later And Remembered Routing

`Set up later`, `Open minimal`, and `Dismiss recommendation` are
same-weight actions wherever Aureline proposes bundle adoption,
dependency restore, extension install, remote attach, or framework-
specific setup.

Choosing `Set up later` may:

- defer tasks into a visible reminder surface;
- keep a setup checklist exportable;
- remember that the user prefers plain open before setup prompts;
- narrow the next open by skipping optional prompts until a freshness,
  policy, trust, or target change invalidates the memory.

Choosing `Set up later` or remembering a route must never:

- widen workspace trust;
- install packages;
- install or activate extensions;
- contact remotes that the current entry did not already require;
- suppress trust, policy, import, or prerequisite review on a future
  open;
- hide that a certified/probable claim is stale, partial, or out of
  scope.

Remembered choices are applied as narrowing hints only. If a remembered
route conflicts with current policy, trust, target identity, schema
version, or signal freshness, the choice becomes
`previous_choice_expired_requires_review` or
`previous_choice_conflicts_with_policy` and the user sees a review path.
