# Workspace Archetype Detection, Readiness Preflight, Admission Checkpoints, and First-Useful-Work Routing

This contract freezes the product truth for workspace archetype detection,
readiness preflight, admission checkpoints, and first-useful-work routing on
the M04 stable line. It composes with:

- `docs/ux/archetype_detection_contract.md` — beta-level detection contract
- `docs/workspace/entry_restore_object_model.md` — entry / restore object model
- `crates/aureline-workspace/src/admission/checkpoint.rs` — live admission
  checkpoint builder
- `schemas/ux/stabilize-workspace-archetype-detection-readiness-preflight.schema.json`
  — boundary schema for the stable record

The machine-readable fixtures live under:

- `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/`

## Scope

- Freeze the stable preflight record that surfaces consume instead of cloning
  status text.
- Enforce source-labeled archetype descriptors with required source classes:
  manifest, bundle_marker, workspace_file, admin_policy,
  extension_contribution, previous_user_choice.
- Keep readiness buckets structurally distinct: blocking_now,
  recommended_soon, optional_later.
- Preserve all six detection outcomes with truthful copy and non-destructive
  fallback routes.
- Make first-useful-work routing inspectable and bounded: no auto-install,
  no auto-trust, no hidden setup, no suppressed review.
- Preserve mixed-root and nested-repo truth with boundary choices.
- Ensure evidence freshness governs certified and probable labels.

## Product Invariant

Every admission surface separates four kinds of truth:

1. **Detected facts** are observations about the workspace, with signal refs
   and freshness.
2. **Recommendations** are proposed bundles, layouts, or setup actions derived
   from facts.
3. **Policy** is the effective trust, admin, network, or entitlement constraint.
4. **User choice** records explicit or remembered decisions, including
   `Set up later`, `Open minimal`, and dismissed suggestions.

A surface that says "configured", "supported", "ready", or "blocked" without
making those four layers inspectable is non-conforming.

## Detection Outcomes

| Outcome | Meaning | Required posture | Forbidden posture |
|---|---|---|---|
| `certified_archetype_match` | Signals match a current certified workflow. | Show archetype ref, support class, evidence age, signal refs, compatible bundle refs, and same-weight bypass actions. | Presenting broad language/framework support without scope or freshness. |
| `probable_archetype` | Strong signals exist, but certification or scope is incomplete. | Show confidence, source signals, recommendation basis, and equal access to plain open. | Auto-installing bundles, extensions, or dependencies as if the match were certain. |
| `mixed_or_ambiguous_workspace` | Multiple roots, stacks, or signals compete. | Offer boundary choices: whole repo, probable project, current folder only, or workset. | Forcing one framework view on a mixed repo. |
| `unknown_or_generic_workspace` | No strong claim is available. | Land in generic shell, keep editing/search available, offer diagnostics as non-blocking guidance. | Suggesting the workspace is unsupported or broken. |
| `restricted_or_policy_blocked` | Useful setup exists, but trust or policy prevents actions. | Separate blocked setup from optional guidance and preserve restricted-mode continuation. | Hiding policy cause behind a generic setup failure. |
| `missing_prerequisite` | Readiness depends on a runtime, container, or remote agent that is unavailable. | Name the prerequisite, boundary, what still works now, and repair scope. | Blocking ordinary file reading/editing behind the prerequisite. |

## Source-Labeled Signals

Required source classes:

- `manifest` — project or tool manifests (package.json, Cargo.toml, etc.)
- `bundle_marker` — workflow-bundle or certified-pack markers
- `workspace_file` — Aureline or third-party workspace / workset files
- `admin_policy` — signed policy bundles, fleet policy, entitlement rules
- `extension_contribution` — detector or workflow hints from extensions
- `previous_user_choice` — remembered routing, deferred setup, dismissed recommendations

Additional source classes (filesystem_layout, lockfile, runtime_probe,
vcs_metadata) may appear for lower-trust facts but do not replace the required
labels above.

## Readiness Buckets

- `blocking_now` — tasks that prevent the requested surface or a required
  safety review (trust review, policy block, import compare, missing remote
  agent for a live deep link).
- `recommended_soon` — tasks that improve fidelity but must not monopolize
  plain editing (dependency restore, extension recommendation, index warmup).
- `optional_later` — additive or dismissable work (docs import, test
  discovery, layout customization).

Every readiness task names:

- task id and class
- bucket
- current state
- execution boundary
- side effects
- source signal refs
- a user-visible continuation or repair hook

## First-Useful-Work Route Matrix

| Entry source | Preferred landing surface | Required explanation |
|---|---|---|
| `single_file_open` | `file_editor_with_root_cues` | Whether the file is standalone, inside a repo, or a workspace candidate. |
| `folder_or_repo_open` | `explorer_plus_readme_or_changed_files` or `nested_root_choice_sheet` | Why the landing was chosen and how to switch. |
| `repository_clone` | `post_clone_handoff` | What was materialized locally and what intentionally did not run. |
| `review_or_incident_deep_link` | `linked_review_incident_or_work_item` | What object was anchored and whether clone/open is still required. |
| `restore_last_session` | `restored_layout_with_placeholders` | What restored exactly, partially, or evidence-only, and what needs review. |
| `imported_state_or_handoff_packet` | `import_compare_or_restore_sheet` | Whether the packet is exact, compatible, layout-only, or evidence-only. |

Route decisions must be reversible through switch options. Restricted or
missing-prerequisite routes must offer `open_minimal`.

## Set Up Later and Remembered Routing

`Set up later`, `Open minimal`, and `Dismiss recommendation` are same-weight
actions wherever Aureline proposes setup. Remembered choices may:

- defer tasks into a visible reminder surface;
- keep a setup checklist exportable;
- remember that the user prefers plain open before setup prompts;
- narrow the next open by skipping optional prompts until a freshness, policy,
  trust, or target change invalidates the memory.

Choosing `Set up later` or remembering a route must never:

- widen workspace trust;
- install packages;
- install or activate extensions;
- contact remotes that the current entry did not already require;
- suppress trust, policy, import, or prerequisite review on a future open;
- hide that a certified/probable claim is stale, partial, or out of scope.

## Safety Invariants

The stable preflight record enforces these as hard builder errors:

- `auto_install_allowed` — must be `false`
- `auto_trust_allowed` — must be `false`
- `hidden_setup_executed` — must be `false`
- `trust_widened` — must be `false`

Any projection that would set any of these to `true` fails the row instead of
shipping.
