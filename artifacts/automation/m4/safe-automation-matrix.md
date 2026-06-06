# Safe Automation Matrix

Canonical packet: `artifacts/automation/m4/safe_automation_qualification/support_export.json`

Schema: `schemas/automation/automation-manifest.schema.json`

Docs: `docs/automation/preview-and-lifecycle.md`

## Controlled Label Vocabulary

| Token | User-facing label | Stable contract |
|---|---|---|
| `macro_safe` | Macro-safe | Local editor/review macro capture and replay only. |
| `recipe_safe` | Recipe-safe | Typed recipe-step insertion with manifest fields preserved. |
| `headless_safe` | Headless-safe | CLI/headless schema contract exists. |
| `ui_only` | UI-only | Interactive surface required; no portable automation claim. |
| `approval_required` | Approval required | Current approval required before execution. |
| `writes_files` | Writes files | File or buffer mutation disclosed before run or insertion. |
| `runs_process` | Runs process | Process or terminal launch disclosed before run or insertion. |
| `network_call` | Network call | Network access disclosed before run or insertion. |
| `remote_mutation` | Remote mutation | Remote target mutation disclosed before run or insertion. |

## Automation Object Qualification

| Object class | Storage form | Required capability floor | Trust requirement | Preview policy | Lifecycle ceiling | Stable claim |
|---|---|---|---|---|---|---|
| `recorded_macro` | `local_user_artifact` | `editor_review_state_replay` | `local_only` | `checkpoint_required_before_replay` | `stable_qualified` | yes, editor/review scope only |
| `workspace_recipe` | `versioned_text_manifest` | `workspace_read`, `filesystem_write` | `trusted_workspace` | `dry_run_required_before_apply` | `stable_labels_only_narrowed_runner` | no |
| `extension_recipe` | `signed_extension_package_manifest` | `extension_invocation`, `workspace_read` | `extension_permission_envelope` | `impact_summary_required` | `dependency_gated` | no |
| `admin_curated_recipe_pack` | `signed_policy_bundle` | `workspace_read`, `admin_policy_mutation` | `policy_provided` | `displayable_plan_required_before_mutation` | `dependency_gated` | no |
| `ephemeral_ai_generated_recipe` | `transient_generated_plan` | `ai_tool_invocation`, `workspace_read` | `trusted_workspace` | `displayable_plan_required_before_mutation` | `labs_only` | no |

## Surface Gates

| Surface action | Required label | Stable scope | Does not execute while inserting/inspecting | Required backing fields |
|---|---|---|---|---|
| `add_to_recipe` | `recipe_safe` | Stable label truth; runner breadth narrowed | yes | storage, capabilities, trust, preview, idempotency, provenance, lifecycle |
| `inspect_descriptor` | none | Stable read-only inspection | yes | storage, capabilities, trust, preview, idempotency, provenance, lifecycle |
| `replay_as_macro` | `macro_safe` | Stable local macro replay | no | storage, capabilities, trust, preview, idempotency, provenance, lifecycle |

## Non-Bypass Rules

- Recorded macros remain profile-local by default and cannot arrive silently from repository content.
- Saved automation artifacts cannot capture secret values, prompt bodies, clipboard content, hidden authority, or undeclared network/process access.
- Add to recipe creates or updates a manifest draft; it does not run a command.
- Broader recipe runners, generated plan saving, workspace automation packs, and extension recipe execution remain below Stable until a separate proof qualifies them.
- Support exports are redacted, non-executable projections that preserve local-only, signed, policy-provided, and support-projection authority classes.
