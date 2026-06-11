# Grouped update review and rollback checkpoints

This page describes the canonical grouped-update review and rollback packet used
by Aureline's grouped update review sheet, conflict cards, rollback checkpoint
receipts, CLI/headless dry run, Help, support exports, and release evidence. The
source artifact is
`artifacts/deps/m5/grouped-update-and-rollback-review.json`.

A package mutation is never a harmless text edit. Before any grouped update
leaves review, the packet discloses exactly what would change, how far it
reaches, what it would run, and how to undo it.

## Update plans have an explicit intent

Every plan declares one of six distinct grouped-update intents — they are never
collapsed into one generic "update":

| Class | Meaning |
|---|---|
| `direct_bump` | A single direct dependency version bump. |
| `security_patch` | A targeted security patch driven by an advisory. |
| `grouped_refresh` | A grouped refresh of several related dependencies. |
| `lockfile_refresh_only` | A lockfile-only re-resolution with no manifest range change. |
| `major_version_pilot` | A piloted major-version upgrade behind explicit review. |
| `workspace_wide_convergence` | A workspace-wide convergence onto one shared version. |

## Requested versus resolved, before commit

Each plan lists the manifests it would touch and, per package, the requested
range, the currently resolved version, and the version the change would produce.
None of this is hidden until after apply.

## Lockfile-churn estimate

Every plan carries an estimated lockfile-churn class and the number of lockfile
entries it would add, remove, and change in place, so the blast radius is
visible before commit:

| Class | Meaning |
|---|---|
| `single_entry` | A single lockfile entry changes. |
| `localized` | A localized cluster of entries changes. |
| `moderate` | A moderate set of entries changes. |
| `broad` | A broad set of entries changes. |
| `workspace_wide` | Churn reaches across the whole workspace lockfile. |

## Constraint and conflict cards

When a plan would create or expose a constraint problem, it renders conflict
cards rather than burying the problem in resolver output:

| Class | Meaning |
|---|---|
| `version_conflict` | Constraints conflict and cannot be unified. |
| `peer_requirement_conflict` | A peer or shared-range requirement conflicts. |
| `duplicate_versions` | Multiple resolved versions would coexist for one package. |
| `feature_unification` | Multiple requests would unify onto one resolved version. |
| `advisory_or_yank` | A target version carries an advisory or has been yanked. |

## Native-build and install-script disclosure

Each plan discloses whether applying it would run install/lifecycle scripts or a
native build, introduce new egress, or be blocked by policy — and whether the
operator must explicitly acknowledge the risk first:

| Class | Meaning |
|---|---|
| `none_known` | No script or native-build risk is known. |
| `package_scripts` | The change introduces or retains install/lifecycle scripts. |
| `native_build` | The change requires a native build (compiler/toolchain). |
| `new_egress` | The change introduces or widens network egress. |
| `policy_blocked` | Policy blocks the script or native-build behavior. |
| `unknown` | Script or native-build behavior cannot be determined. |

The registry/auth source the plan would reach (public or private registry,
enterprise mirror, local cache, or offline snapshot) and its credential mode are
shown too — the packet records the credential **mode** only, never a credential
body.

## Validation packs

Each plan recommends a validation pack — the checks or commands that should run
before apply — and records the pack's current outcome (`not_run`, `passed`,
`failed`, `partial`, or `skipped_by_policy`).

## Rollback checkpoints are durable receipts

Every plan links to a rollback checkpoint receipt. A broken or partial mutation
leaves a durable receipt — never a transient toast — preserving the affected
manifests, the lockfile identity before and after, and the validation outcome:

| State | Meaning |
|---|---|
| `captured` | A pre-apply checkpoint has been captured. |
| `applied_reversible` | The change applied and the checkpoint remains reversible. |
| `partial_recovery_pending` | A broken or partial mutation left recovery pending. |
| `reverted` | The change was reverted from this checkpoint. |
| `superseded` | The checkpoint was superseded by a newer plan. |

Every receipt offers the same three recovery actions: `revert` the mutation back
to the checkpoint, `open_diff` to inspect what changed, and `export_patch` to
export the change as a patch.

## Consistent across surfaces

Grouped-update review behaves consistently across the desktop review sheet, the
CLI/headless dry run (`aureline deps update --plan <id> --dry-run`), and
support/export artifacts. Each plan records this surface parity and it must hold
for every surface.
