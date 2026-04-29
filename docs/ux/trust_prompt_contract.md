# Trust, policy, and permission prompt contract

This document freezes the prompt contract every trust, policy, and
permission surface reads before it asks for authority. The goal is
to remove vague approval copy by making each prompt answer the same
questions with the same target, scope, policy, side-effect, refusal,
and export vocabulary.

The companion artifacts are:

- [`/schemas/trust/prompt_request.schema.json`](../../schemas/trust/prompt_request.schema.json)
  - machine-readable boundary for one `trust_prompt_request_record`.
- [`/fixtures/trust/prompts/`](../../fixtures/trust/prompts/)
  - worked prompt requests for local-continuation denial, policy-locked
  denial, and a named-result approval action.
- [`/docs/ux/prompt_grammar_contract.md`](./prompt_grammar_contract.md)
  - copy and button grammar for destructive, trust, approval, consent,
  publish, promote, rollback, and revoke prompts.
- [`/artifacts/ux/prompt_family_matrix.yaml`](../../artifacts/ux/prompt_family_matrix.yaml)
  - machine-readable prompt-family copy obligations and fixture coverage.

This contract composes with, and does not replace:

- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  for consequence class, permission grant scope, representation class,
  required visible fields, and prompt event lineage.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  for trust states, remembered-decision scope, restricted-mode
  continuity, and escalation cues.
- [`/docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md)
  for authority-ticket issuer rules, side-effect classes, drift
  invalidation, and revocation.
- [`/docs/package/package_action_contract.md`](../package/package_action_contract.md)
  for package and script-risk review prompts.
- [`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md)
  for extension effective-permission prompts.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  for support, freshness, client-scope, and redaction vocabulary.

If this document disagrees with those sources, those sources win and
this document plus the schema and fixtures update in the same change.

## Scope

The contract applies to every prompt that can grant, deny, renew,
explain, or narrow authority:

| Surface class | Typical prompt |
|---|---|
| `extension_prompt` | extension install, update, activation, capability widening |
| `ai_tool_prompt` | AI tool run, AI apply, context egress, mutating plan admission |
| `remote_attach_prompt` | SSH, container, devcontainer, tunnel, debug attach |
| `starter_setup_prompt` | first-run starter, template hook, setup task, repo activator |
| `package_script_risk_review` | package install, script/native-build consent, registry auth |
| `consent_renewal_prompt` | expired grant, trust drift, target drift, policy epoch roll |
| `publish_promote_rollback_review` | publish, promote, rollback, merge, provider-side mutation |
| `policy_admin_prompt` | policy lock, admin override request, trust-root or policy change |
| `provider_handoff_prompt` | browser/device handoff, provider approval ticket, sign-in step-up |
| `trust_elevation_prompt` | workspace trust, restricted-mode exit, remembered trust decision |

Out of scope: visual styling, dialog layout, animation, and component
chrome. A sheet, modal, inline card, CLI prompt, or headless trace may
render differently, but it must collapse to the same record shape.

## Prompt Anatomy

Every prompt MUST answer these questions at the same time as any
approve or deny action is available:

| Question | Schema field |
|---|---|
| What is being requested? | `prompt_questions.what_is_requested`, `capability_groups[]` |
| Who is asking and who owns the authority? | `requester`, `authority_owner` |
| Why is it needed now? | `prompt_questions.why_needed` |
| What changes if allowed? | `prompt_questions.changes_if_allowed`, `side_effect_envelope`, `consequence_blocks[]` |
| What still works if denied? | `prompt_questions.works_if_denied`, `denial_or_degrade` |
| Is the grant for this action, session, workspace, profile, or policy-managed scope? | `target_scope.grant_scope_requested`, `target_scope.scope_filter_class`, `target_scope.remembered_scope_consequence` |
| Where can the user inspect or revoke it later? | `prompt_questions.revocation_route`, `details_action`, `decision_actions[]` |
| What can be copied or exported from this prompt? | `copy_export_posture` |

A prompt that cannot answer one of these questions MUST render as
blocked or details-only. It MUST NOT fall back to a generic "Allow?"
question.

## Frozen Vocabularies

The schema freezes the exact values. This section explains how they
are used.

### Support Class

`support_class` names the product-support posture for the requested
capability, separate from lifecycle or policy state:

`certified`, `supported`, `community`, `experimental`,
`unsupported_here`.

Prompts MUST NOT use support class as authority. A `certified`
capability can still be denied by trust, policy, scope, or target
drift.

### Policy Lock State

`policy_lock.policy_lock_state` names whether the user can affect the
decision:

`unlocked_user_editable`, `policy_ceiling_narrows`,
`policy_locked_no_override`, `policy_injected_grant`,
`policy_source_unavailable_fail_closed`,
`policy_unknown_review_required`.

Policy-locked prompts MUST name `policy_owner_ref`,
`policy_source_ref`, and `user_override_posture`. If the lock forbids
override, the prompt offers details, support/export, or admin-review
paths only. It does not render a fake local approve action.

### Target and Scope

`target_scope.scope_filter_class` reuses the shell scope vocabulary:

`current_root`, `named_workset`, `sparse_slice`, `full_workspace`,
`docs_pack_only`, `policy_limited_view`, `review_workspace`,
`companion_surface`, `remote_workspace`.

`target_scope.grant_scope_requested` reuses the permission grant
scope vocabulary:

`once`, `session`, `workspace`, `profile`, `policy_managed`.

The same target and scope labels MUST appear in product UI, docs/help,
support exports, and CLI/headless traces. A prompt cannot say
"workspace" in the UI and emit `full_workspace` in support output if
the actual scope is `named_workset`.

### Capability and Risk Groups

Prompts support grouped capability and risk classes. A single prompt
MAY request several related capability groups, but each group must
have its own purpose and policy posture:

`workspace_read`, `workspace_write`, `subprocess_or_terminal`,
`network_egress`, `secret_or_identity`, `remote_attach`,
`debug_or_privileged_inspection`, `ai_tool_mutation`,
`extension_host_capability`, `package_or_script_execution`,
`publish_promote_rollback`, `policy_or_admin_change`,
`support_export`, `copy_or_export`, `starter_setup`.

Risk groups are separate:

`first_run_trust`, `capability_widening`, `boundary_crossing`,
`external_side_effect`, `destructive_local_action`,
`credential_projection`, `raw_secret_exposure`, `unsandboxed_script`,
`remote_execution`, `policy_locked`, `remembered_scope`,
`high_blast_radius`, `suspicious_identity`,
`support_export_sensitive`.

Grouping never hides consequence. If one group is high-risk, the
whole prompt must expose the high-risk consequence block and deny
path before any approve action is available.

### Side-Effect Envelope

`side_effect_envelope` binds the visible prompt to the authority
object the runtime will later enforce. It includes:

- `side_effect_class` from the runtime authority vocabulary.
- `consequence_class` from the interaction-safety vocabulary.
- `effect_summary`, `target_effect_refs`, and `external_effect_refs`.
- `undo_or_recovery_class`, `checkpoint_or_rollback_ref`, and
  `authority_ticket_requirement`.
- `requires_preview` and `requires_step_up`.

High-risk prompts cannot be approved, denied, or exported without
explicit target scope, side effect, deny path, and authority owner.
Because those fields are globally required by the schema, consumers
do not need separate high-risk heuristics to know whether the prompt
is complete.

### Refusal and Degrade

Denial is not a generic failure state. `denial_or_degrade` names both
the refusal state and the narrowed capability that remains:

`denial_state_class` values:

`not_denied`, `denied_by_user_continue_local`,
`denied_by_user_read_only`, `denied_by_policy_no_override`,
`denied_missing_scope`, `denied_expired_grant`,
`denied_target_drift`, `cancelled_no_state_change`.

`degraded_capability_class` values:

`no_degrade_available`, `local_only_continues`,
`read_only_inspection_continues`, `draft_saved_locally`,
`preview_only_continues`, `metadata_only_export`,
`open_in_browser_or_provider`, `request_admin_change`,
`install_disabled_capability_removed`.

If local editing, read-only inspection, draft capture, preview, or
metadata export can continue, the prompt MUST say that directly. If
nothing can safely continue, it uses `no_degrade_available` and names
the blocker rather than claiming a general product failure.

## Decision Actions

Every action in `decision_actions[]` has:

- `action_role` - primary approve, deny, safer alternative, details,
  revoke, or admin review.
- `label` - the rendered action text.
- `resulting_state_class` - the state the action creates.
- `platform_mandated_host_flow` - true only when a host platform
  forces a binary host prompt.
- `product_explanatory_label` - required for platform-forced host
  labels.

Consequential decisions MUST use specific labels. Exact labels such
as `Yes`, `No`, `OK`, `Continue`, `Apply`, `Accept`, and `Submit`
are non-conforming unless a platform-mandated host flow forces the
host label and the product renders a specific explanatory label next
to it. Product-native prompts should prefer labels such as
`Grant workspace trust`, `Continue in restricted mode`,
`Attach for this session`, `Use local-only review`, or
`Export metadata only`.

The primary action should name the resulting state, not the internal
operation. `Attach for this session` is better than `Continue`;
`Publish 4 draft comments` is better than `OK`.

## Consequence Blocks

`consequence_blocks[]` MUST include at least:

- `if_allowed` - what changes if the primary action succeeds.
- `if_denied` - what remains available after denial.

Prompts SHOULD also include:

- `if_policy_locked` when a policy source narrows or forbids the
  request.
- `if_grant_remembered` when the grant can persist past the current
  action.
- `if_expired_or_revoked` for renewal prompts and long-lived grants.

Reviewers must be able to see requested capability group,
deny-path behavior, remembered-scope consequences, and revocation
route from this one prompt record.

## Details, Revocation, Copy, and Export

Every prompt has one `details_action`. It points to an inline detail
region, side sheet, inspector route, docs anchor, CLI explain row, or
support export entry. Details are not a replacement for the required
visible fields; they only expose deeper provenance.

Every prompt has one `copy_export_posture`. The default is
`structured_trace_refs_only` or `metadata_only`. Raw prompt text,
raw policy bodies, raw URLs, raw secret material, raw provider
payloads, raw file bodies, and raw token material do not cross this
boundary. If a prompt permits raw copy/export, the record must name
the representation class and the reason that raw export is safe.

## Cross-Surface Projection

The same record projects into:

| Consumer | Required projection |
|---|---|
| Product UI | visible prompt question set, action labels, scope, consequences, deny/degrade path, details action |
| Docs/help | same scope and target vocabulary, same grant/revocation terms |
| Support export | prompt id, surface class, owner/source, policy lock state, capability/risk groups, deny/degrade state, details refs |
| CLI/headless trace | same target and scope vocabulary, same outcome labels, same denial state and recovery action |
| Audit/evidence | prompt id, authority owner/source, side-effect envelope, decision action, revocation route, redaction posture |

No consumer may infer the decision from rendered button text alone.
The `resulting_state_class` and `denial_state_class` are the durable
truth.

## Non-Conformance

A prompt is non-conforming when it:

- asks a consequential question with only `Yes`, `No`, `OK`, or
  `Continue` style labels on a product-owned surface;
- omits what is requested, why it is needed, what changes if allowed,
  what still works if denied, or the grant scope;
- hides the requester, authority owner, policy source, or lock state;
- permits high-risk approval or denial without target scope,
  side-effect envelope, deny/degrade path, and revocation route;
- treats denial as a generic failure where local, read-only, preview,
  draft, or metadata-only work can continue;
- emits different scope vocabulary in UI, docs, support export, and
  CLI/headless traces;
- allows copy/export without a declared representation and redaction
  posture.
