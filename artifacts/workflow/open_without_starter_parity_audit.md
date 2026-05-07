# Open-without-starter parity audit (Create empty / Open folder / Open workspace / Continue without starter)

This parity audit is the checklist that prevents onboarding, templates,
starters, prebuilds, and scaffold flows from drifting into lock-in lanes.
It exists to catch “starter-only” behavior before broader onboarding
surfaces depend on it.

This audit is normative for:

- template galleries and starter cards,
- preflight summary sheets,
- generation preflight sheets,
- post-create handoffs and failure recovery sheets,
- template-health rows and policy notices,
- Start Center entry lanes that advertise starter-assisted opens.

Companion contract and schema:

- [`/docs/workflow/starter_side_effect_envelope.md`](../../docs/workflow/starter_side_effect_envelope.md)
- [`/artifacts/entry/environment_starter_summary_contract.md`](../../artifacts/entry/environment_starter_summary_contract.md)
- canonical: [`/schemas/entry/starter_action_diff.schema.json`](../../schemas/entry/starter_action_diff.schema.json)
- alias: [`/schemas/workflow/starter_preflight_action.schema.json`](../../schemas/workflow/starter_preflight_action.schema.json)

Seed fixtures:

- canonical: [`/fixtures/entry/starter_side_effect_cases/`](../../fixtures/entry/starter_side_effect_cases/)
- legacy seed corpus: [`/fixtures/workflow/starter_preflight_cases/`](../../fixtures/workflow/starter_preflight_cases/)

## 1. Parity objective

Whenever a surface offers a starter lane, it MUST also offer at least one
same-weight bypass lane, drawn from `bypass_path_id`:

- `bypass.create_empty_workspace`
- `bypass.open_folder_without_starter`
- `bypass.open_workspace_without_starter`
- `bypass.clone_repository_without_starter`
- `bypass.open_prebuild_minimal`
- `bypass.set_up_later`
- `bypass.continue_without_starter`

The “same-weight” requirement is not cosmetic: bypass lanes must be
equally discoverable, equally keyboard reachable, and must not require
extra hidden steps compared to the starter lane.

## 2. Audit checklist (must-pass)

### 2.1 Same-weight bypass presence

- A starter surface advertises at least one bypass lane at the same
  visual hierarchy level as the starter commit action.
- The bypass lane is keyboard reachable with a stable focus order (no
  “mouse-only” bypass).
- The bypass lane is not hidden behind an overflow menu, tooltip, help
  link, or “learn more” drawer.

### 2.2 Bypass lanes remain available under blockers

Under each of the following conditions, the bypass lanes remain visible
and actionable (even if the starter lane is blocked):

- trust review required or trust state restricted
- signature review required / signer continuity review required
- policy narrowing (fleet/admin/workspace trust/provider policy)
- network unavailable / mirror-only / offline subset
- template health stale/failed/unsupported/known issue
- remote provisioning prerequisites unmet

### 2.3 Create empty is truly empty

When a surface offers `bypass.create_empty_workspace`:

- the bypass lane MUST NOT run a scaffold/generator,
  dependency restore, extension install, trust grant, secret/auth request,
  remote provisioning, or script/task execution;
- any suggested next steps (restore dependencies, install extensions)
  MUST be deferred and require explicit user intent later.

### 2.4 Continue without starter is a real recovery path

Whenever a starter lane can fail partially or can be deferred:

- `bypass.continue_without_starter` MUST be offered as a same-weight
  recovery path;
- the recovery posture MUST state what is already applied, what remains
  pending, and what Aureline will not run automatically;
- any remaining high-risk actions (trust, secrets/auth, remote
  provisioning, script execution) MUST require reapproval before running.

### 2.5 Parity is enforced at preflight time

Parity is validated at the preflight boundary, not only at the gallery
card:

- the preflight summary must list bypass lanes as same-weight actions;
- the preflight summary must preview starter actions as a diff against
  the chosen bypass lane.

## 3. Fail conditions (examples)

Any of the following is a parity failure:

- The only visible primary action is `Create` and bypass is buried behind
  “More options”.
- A “Create empty” lane still runs a dependency restore or installs an
  extension.
- A starter lane that requires browser auth only reveals the auth step
  after commit.
- A preflight summary shows “prepare environment” without listing the
  specific actions (dependency restore, extension install, trust grant,
  secret/auth request, remote provisioning, script execution).
- A post-create handoff after failure shows only “Close” with no
  continue-without-starter lane.

## 4. Fixture coverage

The fixture corpus proves the must-pass cases:

- create empty lane with zero actions
- open folder/workspace lanes without starter
- starter-assisted lane adding explicit action rows
- deferred actions marked as requiring later reapproval
- continue-without-starter recovery path present
