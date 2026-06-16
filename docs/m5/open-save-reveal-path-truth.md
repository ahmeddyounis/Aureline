# Open/save/reveal path truth

Aureline's local-first promise has to survive the moment a flow *leaves the
product for a native dialog or a system shell*: the OS open/save dialog, an
in-place save, a Save-As to a new target, a "Reveal in system shell", or an
"Open in default browser". Each of those moments can quietly erase the
canonical-path, read-only, generated, and checkpoint vocabulary the in-product
save and restore flows are careful about. If a system dialog is implicit about
what it targets and what it may overwrite, platform-native convenience becomes a
truth gap: a canonicalized alias opens as if it were the literal file the user
picked, a generated preview is saved over as if it were an editable source, a
read-only destination appears writable, or an overwrite commits without the
checkpoint-aware review an in-product save would require.

This document describes the typed path-truth layer that makes every such flow
explicit and reviewable. Each open/save/reveal flow is projected as one typed
flow that records what the user literally selected, what Aureline resolved it to,
the boundary it sits behind, and how it will (or will not) write — using the same
overwrite-review and checkpoint vocabulary the in-product save and restore flows
use.

The layer reuses the filesystem-identity (canonical-path lineage) and
save-coordination (artifact-save-truth) objects rather than maintaining a
parallel notion of path or checkpoint, and rides alongside the system-open and
file-association intake report
(`artifacts/platform/m5-system-open-and-file-association.md`): that report
governs what happens when the OS hands the app a target; this layer governs the
path/boundary/overwrite truth of the system dialogs and reveal actions the
product itself drives.

## Canonical objects

| Object | Path |
| ------ | ---- |
| Typed consumer | `crates/aureline-workspace/src/m5_open_save_reveal/mod.rs` |
| Headless inspector | `crates/aureline-workspace/src/bin/aureline_workspace_m5_open_save_reveal.rs` |
| Boundary schema | `schemas/platform/m5-path-boundary.schema.json` |
| Report fixture | `fixtures/platform/m5-open-save-reveal/report.json` |
| Support-export fixture | `fixtures/platform/m5-open-save-reveal/support_export.json` |
| Compact fixture | `fixtures/platform/m5-open-save-reveal/compact.txt` |
| Case-export fixtures | `fixtures/platform/m5-open-save-reveal/cases/*.json` |
| Published report | `artifacts/platform/m5-open-save-reveal.md` |
| CI gate | `tools/ci/m5/open_save_reveal_check.py` |

The headless inspector is the only mint-from-truth path. The report fixture and
the published report at `artifacts/platform/m5-open-save-reveal.md` are asserted
bit-for-bit equal to the seeded report by
`crates/aureline-workspace/tests/m5_open_save_reveal_fixtures.rs`.

## Flow kinds

Every system-dialog or reveal flow is one of the five required flow kinds, and
all of them flow through a single typed path:

- `open` — a system open dialog selecting a target to read.
- `save` — an in-place save of the current document to its canonical target.
- `save_as` — a save-as dialog choosing a new target.
- `reveal_in_system_shell` — a "Reveal in system shell" action that opens the OS
  file manager and selects the target.
- `open_in_default_browser` — an "Open in default browser" action that hands a
  target to the default browser.

## Literal vs canonical target

Each flow preserves two target identities so the user can see what they selected
against what Aureline resolved:

- `literal_target_ref` — an export-safe captured ref for the literal target the
  user selected, plus a `literal_format` shape hint (`windows_drive_path`,
  `windows_unc_path`, `posix_path`, `file_uri`, `url`, `unknown`). It is never a
  raw path or secret body; user-visible surfaces render the literal locally.
- `canonical_target_ref` — the canonical identity Aureline resolved the literal
  to, classified into the shared `detected_target_kind`.

The relationship between the two is classified by `path_truth_class`, so a user
can always tell whether a dialog is targeting:

- `literal_is_canonical` — the literal file they picked, which is its own
  canonical target;
- `canonical_alias_resolved` — a canonicalized alias (symlink, case variant, or
  network mapping) that resolves to a different canonical target;
- `boundary_labeled_artifact` — a boundary-labeled generated, remote, or
  read-only artifact rather than a plain local file; or
- `canonical_target_missing` — a target whose canonical identity could not be
  resolved at all.

## Boundary labels

Each flow labels the boundary its target sits behind with `boundary_label` so
platform-native dialog convenience never erases the distinction:

- `local_writable` — a local, writable target.
- `remote_adjacent` — a target reached across a network share or remote-adjacent
  mount.
- `generated` — a generated artifact whose canonical source is elsewhere.
- `read_only` — a read-only destination that cannot be written in place.

## Overwrite posture and checkpoint review

Each flow declares an `write_posture` in the same overwrite-review vocabulary the
in-product save and restore flows use, and an `overwrite_review_ref` shared with
those flows:

- `no_write_action` — open, reveal, and browser flows do not write.
- `create_new_file` — a save-as to a target that does not already exist.
- `overwrite_with_checkpoint` — an in-place overwrite that MUST pin an available
  checkpoint (`checkpoint_availability = pinned` with a `checkpoint_ref`);
  otherwise it is an `overwrite_without_checkpoint_review` blocker.
- `overwrite_review_required` — an overwrite held for explicit review before any
  write commits.
- `write_blocked_read_only` — a write blocked because the destination is
  read-only.
- `export_not_in_place_save` — a generated artifact exported rather than saved in
  place.

A writing posture against a read-only boundary or destination is a
`read_only_write_attempt` blocker, and a generated artifact saved in place is the
distinct `generated_treated_as_in_place_save` blocker. The two never collapse.

## Reveal and browser side effects

`reveal_in_system_shell` and `open_in_default_browser` are kept as stable,
explicit actions. Each discloses its external side effect via `reveal_side_effect`
(`selects_target_in_file_manager` or `opens_default_browser`) and a stable
`reveal_action_label_ref`. A flow that hides the side effect or the label — or an
open/save flow that claims an external side effect it should not have — is a
`reveal_side_effect_hidden` blocker, so a reveal action never hides
platform-specific or boundary-specific behavior.

## Path condition and recovery

The condition of the path/destination at flow time is one of `exact_available`,
`missing_canonical_target`, `network_share_alias`, `generated_output`, or
`read_only_destination`. Any value other than `exact_available` MUST offer at
least one recovery action, and each condition stays a distinct failure:

- a `missing_canonical_target` with no recovery is a `wrong_target_save` blocker;
- a `network_share_alias` with no recovery is an `alias_path_confusion` blocker;
- a `generated_output` with no recovery is a `generated_output_unrecoverable`
  blocker; and
- a `read_only_destination` with no recovery is a
  `read_only_destination_unrecoverable` blocker.

## Incident case exports

The four required failure-path classes are published as standalone case-export
packets under `fixtures/platform/m5-open-save-reveal/cases/`, so support can
reproduce each from typed diagnostics instead of a screenshot:

- `missing_canonical_target.json` — a save-as whose canonical target cannot be
  resolved, held for review with a target picker and the canonical-path detail.
- `network_share_alias.json` — a save through a network-share alias, recovered by
  resolving the share alias and reconnecting the share.
- `generated_output.json` — a save of a generated artifact, exported rather than
  saved in place and offered a regenerate-from-source path.
- `read_only_destination.json` — a save to a read-only destination, blocked and
  offered a writable copy elsewhere.

## Other invariants

- Every flow reuses a `filesystem_identity_ref` and a `save_coordination_ref` —
  the canonical-path lineage and save-coordination objects — so wrong-target
  saves, alias-path confusion, and checkpoint availability stay inspectable in
  diagnostics and support packets.
- Every flow names an `active_profile_owner_ref`, a `trust_checkpoint_ref`, and a
  `canonical_command_ref` — the same command the in-product path runs — so a
  system dialog can never grant more authority than the in-product path; a
  missing trust checkpoint is a `trust_evaluation_bypassed` blocker and a missing
  canonical target ref is a `canonical_path_hidden` blocker.
- A missing `overwrite_review_ref` is a `checkpoint_vocabulary_divergence`
  blocker, so the overwrite/checkpoint language stays shared with the in-product
  save and restore flows.
- Stale evidence on a marketed flow is a blocker so release tooling can narrow
  the surface instead of shipping it as implicitly stable.
- The report cross-links the filesystem-identity, save-coordination,
  restore-continuity, native-desktop matrix, system-entry intake, and Help/About
  surfaces so path, checkpoint, and boundary vocabulary cannot drift
  independently.

## Verification

```sh
cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- validate
cargo test -p aureline-workspace --test m5_open_save_reveal_fixtures
python3 tools/ci/m5/open_save_reveal_check.py
```

Regenerate the fixtures and the published report from the seed:

```sh
cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- report \
  > fixtures/platform/m5-open-save-reveal/report.json
cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- support-export \
  > fixtures/platform/m5-open-save-reveal/support_export.json
cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- compact \
  > fixtures/platform/m5-open-save-reveal/compact.txt
cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- report-md \
  > artifacts/platform/m5-open-save-reveal.md
```
