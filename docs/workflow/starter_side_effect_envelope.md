# Starter side-effect envelope and starter-preflight action taxonomy

This document freezes the disclosure contract every **starter**
(template, scaffold, prebuild / warm-start, and any starter-assisted
open lane) MUST satisfy **before** any action mutates:

- workspace files (including generated files and lockfiles),
- dependencies (language packages, toolchains, system packages),
- extensions,
- trust / admission posture,
- secrets / authentication state, or
- remote / managed state (remote images, managed workspaces, hosted
  services).

The goal is to prevent “generic Create” flows from hiding immediate or
deferred side effects: users must see what Aureline will do **now**, what
it will **defer** or only **recommend**, and which steps will require
**reapproval later**.

Companion schema:

- [`/schemas/workflow/starter_preflight_action.schema.json`](../../schemas/workflow/starter_preflight_action.schema.json)

Companion fixtures:

- [`/fixtures/workflow/starter_preflight_cases/`](../../fixtures/workflow/starter_preflight_cases/)

Parity audit:

- [`/artifacts/workflow/open_without_starter_parity_audit.md`](../../artifacts/workflow/open_without_starter_parity_audit.md)

This contract is normative for the envelope + taxonomy. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, or milestone anchors in
§10, those sources win and this document plus its companion schema,
fixtures, and parity-audit artifact update in the same change. Where a
starter surface mints parallel “setup action” labels that do not resolve
through the closed taxonomy in §3, the surface is non-conforming.

## 1. Scope

- Freeze the **starter side-effect envelope** shape (§4) that expresses
  starter behavior as a **diff** between:
  - a **plain open** lane (open/clone/create without starter), and
  - the **starter-assisted** lane.
- Freeze the **starter preflight action taxonomy** (§3) used by the
  envelope to classify every immediate or deferred effect.
- Freeze the parity invariant that every starter flow advertises at
  least one same-weight **open-without-starter** lane (§5) and never
  hides it behind secondary UI.

## 2. Out of scope

- Implementing starter generators, scaffold runners, package adapters, or
  remote provisioning backends.
- Defining final microcopy. This contract pins the closed sets and the
  envelope fields that copy resolves against.

## 3. Starter preflight action taxonomy

Every preflight summary and every post-create handoff that previews
starter behavior classifies actions using the closed set
`starter_preflight_action_class`:

| Action class | What it covers | Non-conforming if… |
|---|---|---|
| `file_generation` | creating/modifying files via a template/scaffold/generator (including lockfiles when the generator writes them) | presented as “prepare workspace” without enumerating file writes |
| `dependency_restore` | restoring language/runtime dependencies from manifests/lockfiles (Cargo, npm, pip, etc.) | collapsed into “set up environment” when the ecosystem is known |
| `package_install` | installing system packages, toolchains, runtimes, or platform components (OS package manager, SDK install, toolchain pack) | hidden behind a generic “install prerequisites” step |
| `extension_install` | installing, restoring, or activating IDE extensions required for the starter | the UI says “installs extensions” without naming which |
| `remote_provisioning` | creating/attaching remote images, containers/devcontainers, or managed-workspace resources | remote/managed behavior appears only after commit |
| `secret_or_auth_request` | requesting secret-broker handles, browser sign-in, auth callbacks, or credential projection | secrets/auth are requested without pre-commit disclosure |
| `trust_grant` | widening workspace trust, admission, or permission scope | trust is implicitly granted as part of Create |
| `script_or_task_execution` | running tasks/scripts/hooks/recipes/macros that execute code or mutate external state | shown as “run setup tasks” without naming execution |

Rules (frozen):

1. **No collapsed Create.** If a starter lane will run any action whose
   class is one of `dependency_restore`, `package_install`,
   `extension_install`, `remote_provisioning`, `secret_or_auth_request`,
   `trust_grant`, or `script_or_task_execution`, the preflight MUST list
   an explicit action row for it.
2. **File writes are never implied.** A starter that writes files MUST
   include at least one `file_generation` row even when the generator is
   “first-party”.
3. **Trust and secrets are never side effects of navigation.** Trust
   grant and secret/auth requests are previewed as actions but MUST NOT
   execute merely because the user opened the preflight.

## 4. Starter side-effect envelope

The envelope is the disclosure payload a surface renders **before**
commit. It is shaped as a diff:

- `plain_open_baseline` — what happens on a same-weight open lane
  (open/clone/create without starter).
- `starter_added_delta` — the actions the starter lane adds beyond the
  plain-open baseline.

Each lane partitions actions into:

- `actions_run_now[]` — actions Aureline will run as part of the commit
  sequence (including actions that must finish before open completes).
- `actions_deferred[]` — actions Aureline will not run immediately (run
  after open, require explicit later invocation, or are recommendation-
  only).

Every action row carries:

- `starter_preflight_action_class` (§3)
- `execution_commitment_class` — whether Aureline will execute, offer,
  or only recommend the action
- `timing_class` — distinguishes “run now” from “deferred” and “skipped
  under bypass”
- `reapproval_posture_class` — whether deferred execution will require
  later reapproval
- `side_effect_scope_class` — the scope the action mutates (workspace,
  profile, device, external provider, etc.)

Rules (frozen):

1. **Explicit now vs deferred.** A preflight that lists actions but does
   not indicate which are `actions_run_now` vs `actions_deferred` is non-
   conforming.
2. **Deferred reapproval is explicit.** If any deferred action will
   require a later prompt (trust, secret/auth, provider auth scope, or
   elevated execution), it MUST be marked with
   `reapproval_posture_class = requires_later_user_reapproval` or
   stronger.
3. **Diff is relative to a real lane.** `plain_open_baseline.bypass_path_id`
   MUST resolve to a bypass lane that exists as a same-weight alternative
   on the surface that renders the envelope. A fictional baseline is
   non-conforming.

## 5. Open-without-starter parity invariant

Every surface that offers a starter lane MUST also offer at least one
same-weight bypass path id from the closed `bypass_path_id` set
(template-and-prebuild disclosure contract).

Required posture:

- bypass options are **keyboard reachable** and **same weight** as the
  starter action (not hidden behind overflow menus, not rendered as
  reduced-contrast footnotes, and not moved below the fold).
- bypass options remain visible even when the starter lane is blocked by
  trust review, policy, signature review, network unavailability, remote
  provisioning failures, or known issues.

The worked parity checklist lives in the artifact:
[`/artifacts/workflow/open_without_starter_parity_audit.md`](../../artifacts/workflow/open_without_starter_parity_audit.md).

## 6. Worked examples

The fixture corpus under
[`/fixtures/workflow/starter_preflight_cases/`](../../fixtures/workflow/starter_preflight_cases/)
provides seeded starter-preflight rows that exercise:

- create empty vs starter lanes,
- open folder/workspace without starter vs starter lanes,
- deferred actions and later reapproval,
- continue-without-starter recovery after a starter lane is deferred or
  partially applied.

## 7. Linked contracts and schemas

- Template gallery / prebuild disclosure contract (bypass paths,
  preflight axes, post-create handoff axes):
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
- Scaffold template-card and generation-preflight contract (immediate vs
  deferred partition, file-write and dependency impact previews):
  [`/docs/scaffolding/template_health_and_preflight_contract.md`](../scaffolding/template_health_and_preflight_contract.md)
- Bootstrap-queue item taxonomy (typed setup actions enqueued by open /
  clone / resume lanes):
  [`/schemas/workspace/bootstrap_queue_item.schema.json`](../../schemas/workspace/bootstrap_queue_item.schema.json)

## 8. Changing this contract

- Adding a new `starter_preflight_action_class`,
  `execution_commitment_class`, `timing_class`,
  `reapproval_posture_class`, or `side_effect_scope_class` value is
  additive-minor and requires updating the companion schema and at least
  one fixture in the same change.
- Repurposing an existing value is breaking and requires a new decision
  row in `artifacts/governance/decision_index.yaml` before any downstream
  surface consumes the new meaning.

## 9. Acceptance summary (non-exhaustive)

- No starter hides managed, remote, secret-bearing, or trust-widening
  behavior behind a generic Create button.
- Preflight disclosure is explicit about what is generated now, what is
  deferred or only recommended, and what recovery path exists.
- Open-without-starter is always a same-weight alternative where the
  product claims it exists.

## 10. Source anchors

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — §6.9 templates/starters
  side-effect envelope; §17.7 same-weight bypass path invariant.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — scaffolding /
  template manifest and preview-first rules.
- `.t2/docs/Aureline_Milestones_Document.md` — authority governance and
  external-effect preview requirements.

