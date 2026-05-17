# Workspace Archetype Detection And First Useful Work

This contract describes the beta workspace-detection path from entry action to
first useful surface. It composes with
[`docs/ux/archetype_detection_contract.md`](../../ux/archetype_detection_contract.md),
[`schemas/workspace/archetype_detection.schema.json`](../../../schemas/workspace/archetype_detection.schema.json),
and the detector catalog at
[`artifacts/compat/m3/archetype_detection_matrix.yaml`](../../../artifacts/compat/m3/archetype_detection_matrix.yaml).

## Runtime Shape

The workspace crate owns two layers:

- [`crates/aureline-workspace/src/archetype_detection/`](../../../crates/aureline-workspace/src/archetype_detection/)
  scans bounded workspace markers and returns an inspectable proposal or
  conflict.
- [`crates/aureline-workspace/src/admission/checkpoint.rs`](../../../crates/aureline-workspace/src/admission/checkpoint.rs)
  projects that truth into admission, readiness buckets, same-weight bypasses,
  mixed-boundary choices, and a first-useful-work route.

The detector is read-only. It may recommend a compatible bundle or framework
pack, but it never installs packages, enables extensions, grants trust, or
changes layout. Certified and probable states carry evidence freshness rows with
the scorecard or packet ref, reviewed date when available, stale-after window,
and freshness class.

## Outcome Rules

| Outcome | Route posture |
| --- | --- |
| `certified_archetype_match` | Scoped support claim only when evidence is current; setup remains review-first. |
| `probable_archetype` | Show confidence, sources, and bundle or pack suggestions; plain open remains equal weight. |
| `mixed_or_ambiguous_workspace` | Offer whole repo, probable project, current folder only, and workset choices. |
| `unknown_or_generic_workspace` | Open a generic shell or explorer without implying the workspace is broken. |
| `restricted_or_policy_blocked` | Separate blocked setup from optional guidance and preserve restricted continuation. |
| `missing_prerequisite` | Name the missing local, container, remote, or managed prerequisite while preserving what works now. |

Readiness is always grouped as `blocking_now`, `recommended_soon`, and
`optional_later`. These buckets must not be flattened into a generic setup list.

## First Useful Routing

Entry source decides the first useful surface:

| Entry source | Preferred surface |
| --- | --- |
| `single_file_open` | `file_editor_with_root_cues` |
| `folder_or_repo_open` | `explorer_plus_readme_or_changed_files`, or `nested_root_choice_sheet` for mixed workspaces |
| `repository_clone` | `post_clone_handoff` |
| `review_or_incident_deep_link` | `linked_review_incident_or_work_item` |
| `restore_last_session` | `restored_layout_with_placeholders` |
| `imported_state_or_handoff_packet` | `import_compare_or_restore_sheet` |

Remembered routing is a narrowing hint only. It invalidates on trust, policy,
target identity, or evidence freshness changes and cannot suppress required
review.

## Evidence

Reviewable fixtures live in two places:

- [`fixtures/workspace/entry_routes/`](../../../fixtures/workspace/entry_routes/)
  carries full schema-shaped admission records.
- [`fixtures/workspace/m3/archetype_detection/`](../../../fixtures/workspace/m3/archetype_detection/)
  carries compact detector and routing cases tied to the beta catalog.

Focused verification:

```sh
cargo test -p aureline-workspace --test archetype_detection
cargo test -p aureline-workspace admission::checkpoint --lib
cargo test -p aureline-shell start_center::first_useful_work --lib
```
