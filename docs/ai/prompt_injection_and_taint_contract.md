# Prompt-injection, tainted-context, and privileged-action safeguard contract

This document is the **product-wide contract** for how AI inputs
are classified, how trust / taint / precedence travels with them
across native and external tools, and which downstream actions a
given input class may legitimately drive. It freezes the rule
set that prevents repository text, terminal / log / tool output,
web / docs / search results, MCP or external-tool responses, and
user-pasted content from silently widening permissions, retention,
egress, or trust authority later.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / composer / tool
surface's mint of its own copy, this document wins and the surface
is non-conforming.

The companion artifacts are:

- [`/schemas/ai/tainted_input_source.schema.json`](../../schemas/ai/tainted_input_source.schema.json)
  — boundary schema for the `tainted_input_source_record`,
  `prompt_injection_evaluation_record`, and
  `prompt_injection_taint_audit_event_record` shapes.
- [`/schemas/ai/approval_action_class.schema.json`](../../schemas/ai/approval_action_class.schema.json)
  — boundary schema for the `approval_action_class_record`,
  `privileged_action_disclosure_record`, and
  `approval_action_audit_event_record` shapes.
- [`/fixtures/ai/prompt_injection_cases/`](../../fixtures/ai/prompt_injection_cases/)
  — worked-example corpus covering at least: a repo-instruction
  conflict, tainted terminal output, a tainted web excerpt, a
  tainted MCP response, a designated policy file, and an approval
  renewal before a mutating follow-on action.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/context_assembly_contract.md`](./context_assembly_contract.md) —
  segment class, source class, trust posture, tainted-fence
  strategy, tainted-usage constraints, instructional role, omit /
  block / redaction reasons, and the rule that
  `fenced_tainted_data` never gains instruction authority.
- [`/docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md) —
  provider entry, model entry, execution-locus, region posture,
  retention stance, and the invariant that external-tool output is
  tainted by default and rides the same permission, policy, and
  audit model as native tools. This contract refines the
  *evaluation* of that invariant per input source.
- [`/docs/ai/ai_copy_guardrails_contract.md`](./ai_copy_guardrails_contract.md) —
  evidence-first confidence language and the rule that low-confidence
  proposals MUST remove direct mutation controls.
- [`/docs/auth/credential_state_and_secret_prompt_contract.md`](../auth/credential_state_and_secret_prompt_contract.md) —
  credential states, secret-access prompt vocabulary, and the rule
  that handles, aliases, and source labels may travel where
  allowed but raw secret material never does. AI inputs that quote
  credential state quote the typed handle only.
- [`/docs/ux/prompt_grammar_contract.md`](../ux/prompt_grammar_contract.md) —
  destructive / trust / approval / consent-renewal / publish-
  promote-rollback prompt grammar. Privileged actions admitted by
  this contract render through that grammar; this contract never
  re-mints the prompt copy.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state, policy epoch, and trust state on every
  record.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md) —
  the broker-owned redaction pass runs before any input's bytes
  reach the model or the audit stream.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md) —
  admin policy MAY narrow which input sources, action classes,
  loci, retention stances, and egress destinations the AI surface
  may admit; policy MAY NOT silently widen.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  approval-ticket vocabulary. Every privileged AI-driven action
  above `inspect_only_read` MUST spend an approval ticket through
  the same path native tools use.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  `freshness_class`, `client_scope`, `redaction_class` re-exported
  without modification.
- [`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md) —
  extension-proposed input rides the extension effective-permission
  surface; this contract never promotes extension-proposed bytes to
  trusted instruction authority on its own.

If this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does **not** ship an inference engine, a tool broker,
a sandbox runtime, or any provider-arbitration code. It freezes
the contract those implementations will read and write. The
eventual safeguard crate's Rust types are the schema of record;
the JSON Schema exports are the cross-tool boundary every
non-owning surface reads.

## Why freeze this now

The product has to answer the same question for every AI turn
and every AI-driven follow-on action, long before any model or
tool runtime is wired:

1. *Where did this input byte originate, and what authority class
   does it carry?*
2. *If this input is repo text, terminal output, a web excerpt, an
   MCP response, an extension-proposed payload, a connected-provider
   payload, or a paste, may it tell the model what to do?*
3. *If this input is a designated policy file under `.aureline/ai/*`,
   is it protected from being silently overridden by repo prose,
   model prose, or tainted evidence?*
4. *If the model proposes a follow-on action (write file, run
   command, install, publish, commit, push, dispatch a branch
   agent, widen retention, change egress), did the proposal ride
   tainted evidence into a privileged effect?*
5. *Did the surface require a fresh approval before the action, or
   did it silently reuse an earlier approval whose context has now
   widened?*
6. *Could the action have been taken without the user (or admin)
   seeing where it would actually run?*

Without one frozen contract, every surface is free to invent its
own mapping from "I read some bytes" to "I propose a privileged
action". Repo `AGENTS.md` text starts widening permissions, terminal
output starts dispatching branch agents, MCP responses start
minting citation anchors, prior-turn AI prose starts self-approving
its own follow-on, and the user (or admin) loses any deterministic
way to refuse a path that was never disclosed. This contract closes
that gap with **one closed input-source vocabulary, one closed
taint / trust / precedence model, one closed prohibited-case
vocabulary, and one closed privileged-action approval-renewal /
inspect-only-downgrade / blocked-mutation rule set** every
AI-adjacent surface reads.

## Who reads this document

- **AI / prompt-composer / context-resolver authors** classifying
  every input that enters an assembly, applying the taint /
  precedence rules at composition time, and emitting the
  `tainted_input_source_record` and (for any privileged follow-on)
  the `approval_action_class_record` and
  `privileged_action_disclosure_record` before dispatch.
- **Tool-broker / native-tool / external-tool authors** treating
  tool returns as tainted by default and refusing to grant tool
  permission, dispatch a branch agent, mutate the workspace,
  publish externally, or widen capability on the strength of a
  tainted return alone.
- **Approval / browser-handoff / consent-renewal surface authors**
  reading the `approval_action_class_record` to decide whether the
  current ticket is sufficient or whether a fresh approval is
  required, and rendering through the prompt-grammar contract.
- **Admin / policy / settings surface authors** narrowing which
  input source classes, action capability classes, loci, retention
  stances, and egress destinations are admitted per deployment
  profile.
- **Evidence / replay / support / parity-audit authors** quoting
  the input-source classification, the prohibited-case set, the
  approval-renewal class, and the audit-event id mechanically
  rather than re-deriving the safeguard.
- **Security / emergency-action authors** quarantining a source
  class, withdrawing a tool entry, or freezing an action capability
  class through the audit event stream without re-authoring the
  contract shape.

## 1. Input-source classes

Every byte that reaches an AI assembly resolves to exactly one
`input_source_class`. The set is closed. A surface that observes
an unrecognised input source MUST deny with
`input_source_class_unresolved` rather than silently classify.

| Class                                | Origin                                                                                                      | Default taint                                  |
|--------------------------------------|-------------------------------------------------------------------------------------------------------------|------------------------------------------------|
| `repository_workspace_content`       | Code / docs / comments / config in the workspace; first-party content the user owns.                        | `trusted_first_party_data`                     |
| `repo_instruction_bundle_authored`   | Repo-authored instruction bundle (e.g. AGENTS.md-style file). The composer references it by id.              | `trusted_first_party_data`                     |
| `trusted_policy_file`                | Designated AI policy file under `.aureline/ai/policy/*` admitted by signed admin posture.                    | `trusted_policy`                               |
| `trusted_workspace_pinned_policy`    | Workspace-pinned policy file the workspace owner pinned (does not carry admin authority).                    | `trusted_first_party_data`                     |
| `trusted_user_profile_policy`        | User-profile policy file the user pinned for their own profile.                                              | `trusted_first_party_data`                     |
| `platform_approval_record`           | Typed approval ticket (ADR-0010) the platform minted; addressable, not user prose.                          | `trusted_control_record`                       |
| `platform_control_record`            | Typed admin control / settings record minted by the platform.                                                | `trusted_control_record`                       |
| `docs_pack_excerpt`                  | First-party docs-pack excerpt with a citation anchor.                                                       | `trusted_authority_quote`                      |
| `generated_reference_excerpt`        | Generated-reference excerpt with a citation anchor.                                                         | `trusted_authority_quote`                      |
| `runbook_step_excerpt`               | Runbook step excerpt with a citation anchor.                                                                | `trusted_authority_quote`                      |
| `release_note_excerpt`               | Release note excerpt with a citation anchor.                                                                | `trusted_authority_quote`                      |
| `workspace_search_result`            | Workspace search-result packet ref; not user-authored.                                                       | `trusted_first_party_data`                     |
| `workspace_diagnostic_capture`       | Workspace diagnostic / lint / build / test result capture.                                                   | `trusted_first_party_data`                     |
| `terminal_command_output`            | Terminal / shell command output captured into a transcript.                                                  | `tainted_evidence`                             |
| `log_capture`                        | Log capture from a running service / process / extension.                                                    | `tainted_evidence`                             |
| `tool_call_native_response`          | Return value from a first-party native tool whose locus is `local_in_process` or `local_subprocess_same_device` and whose manifest signature is verified. | `data_only_no_instruction`                    |
| `tool_call_external_response`        | Return value from any external / extension-mediated / connected-provider tool.                              | `tainted_evidence`                             |
| `mcp_server_response`                | Return value from a Model Context Protocol server.                                                          | `tainted_evidence`                             |
| `web_search_result`                  | Web search result excerpt.                                                                                  | `tainted_evidence`                             |
| `external_docs_excerpt`              | Third-party / vendor docs excerpt retrieved on demand.                                                      | `tainted_evidence`                             |
| `connected_provider_payload`         | Connected-provider browser / handoff / callback payload.                                                    | `tainted_evidence`                             |
| `extension_proposed_input`           | Input proposed by an installed extension.                                                                   | `tainted_evidence`                             |
| `collaboration_remote_payload`       | Content authored by a remote collaborator inside a shared session.                                          | `tainted_evidence`                             |
| `user_authored_instruction`          | The active turn prompt the user typed in the composer.                                                      | `data_only_no_instruction`                     |
| `user_supplied_paste`                | Pasted / dropped text or files the user did NOT type as a turn instruction.                                  | `tainted_evidence`                             |
| `user_supplied_file_attachment`      | A user-attached file (binary or text) the user attached to the turn.                                        | `tainted_evidence`                             |
| `ai_prior_turn_response`             | Prior-turn AI output carried over inside the same composer session.                                          | `tainted_evidence`                             |
| `runtime_diagnostic_capture`         | Crash / error / panic capture from a running runtime.                                                        | `tainted_evidence`                             |
| `policy_quarantined_source`          | Any source a policy bundle has quarantined; only a summary ref may be quoted.                                | `policy_quarantined`                           |
| `unknown_unclassified_source`        | The composer cannot classify the source. Fail-closed.                                                       | `unknown_must_treat_as_tainted`                |

Rules (frozen):

1. The default taint column above is the **starting** posture
   before any policy narrowing; admin policy MAY narrow further
   (e.g. `tool_call_native_response` → `tainted_evidence` under a
   restricted profile) but MAY NOT widen it.
2. `user_authored_instruction` is the *only* user-typed posture
   that may carry instruction authority, and only at the
   `user_turn_prompt` priority — see §3 below. Pasted content is
   not user-typed.
3. `trusted_policy_file` and `platform_control_record` /
   `platform_approval_record` are the **only two** classes that
   may carry trust authority above repo prose; everything else
   resolves to data, evidence, or fenced-tainted.
4. `policy_quarantined_source` MUST NOT inline a body — only the
   summary ref. A quarantined source quoted as a body denies with
   `policy_quarantined_body_inlined`.
5. `unknown_unclassified_source` is fail-closed; it is treated
   identically to `tainted_evidence` and additionally raises
   `input_source_class_unresolved` on the audit stream.

## 2. Taint, trust, and precedence model

This section names the closed labels every input record carries and
the rule the composer applies. The schema (`tainted_input_source.schema.json`)
freezes the same vocabulary and the boundary record shape.

### 2.1 Taint classes

```
trusted_policy
trusted_control_record
trusted_authority_quote
trusted_first_party_data
data_only_no_instruction
tainted_evidence
policy_quarantined
unknown_must_treat_as_tainted
```

Rules (frozen):

1. `trusted_policy` is reachable only by `trusted_policy_file`
   (designated `.aureline/ai/policy/*` files) and by
   `platform_control_record` when the control authority class is
   `admin_via_signed_policy` or `platform_control_authority`.
2. `trusted_control_record` is reachable only by typed approval /
   control records minted by the platform itself; ad hoc free-form
   "approval" prose is not a control record.
3. `trusted_authority_quote` is reachable only when the source
   carries a non-empty `citation_anchor_refs` set.
4. `trusted_first_party_data` carries no instruction authority on
   its own; it is data the user owns and the composer treats it as
   `attached_data` or `retrieved_reference` instructional role,
   never as `user_turn_prompt` or above.
5. `data_only_no_instruction` is the in-between class for native-tool
   output and the user's typed turn-prompt body: it is admitted to
   the assembly but the composer plan limits its instructional role
   to the originating role only (the user's turn prompt enters as
   `user_turn_prompt`; native tool output enters as
   `tool_call_response`).
6. `tainted_evidence` rides the tainted fence (`fenced_tainted_data`)
   regardless of whether the user explicitly added it.
7. `policy_quarantined` is summary-ref-only; no body, no inline
   citation, no instruction authority.
8. `unknown_must_treat_as_tainted` is `tainted_evidence` plus a
   typed denial on the audit stream.

### 2.2 Capability and effect labels

Every privileged follow-on action the composer might propose carries
exactly one `action_capability_class` and at least one
`action_effect_class`. The schema (`approval_action_class.schema.json`)
freezes the vocabulary; abbreviated here:

| `action_capability_class`         | Typical effect classes                                                                                |
|-----------------------------------|-------------------------------------------------------------------------------------------------------|
| `inspect_only_read`               | `no_state_change`                                                                                     |
| `local_reversible_edit`           | `local_state_change_reversible`                                                                       |
| `local_destructive_edit`          | `local_state_change_irreversible`                                                                     |
| `branch_agent_dispatch`           | `local_state_change_reversible` and / or `external_state_change_reversible` depending on dispatch.    |
| `commit_to_repo`                  | `local_state_change_reversible` (commit; revert exists), `external_state_change_irreversible` (push). |
| `external_publish_reversible`     | `external_state_change_reversible`                                                                    |
| `external_publish_irreversible`   | `external_state_change_irreversible`                                                                  |
| `policy_or_trust_mutation`        | `policy_state_changed` or `trust_state_widened`                                                       |
| `capability_widening`             | `permission_widened`                                                                                  |
| `retention_widening`              | `retention_widened`                                                                                   |
| `egress_widening`                 | `egress_widened`                                                                                      |
| `cross_workspace_recall`          | `local_state_change_reversible` plus an explicit cross-workspace flag                                 |
| `provider_route_change`           | `egress_widened` and / or `retention_widened` depending on the new route                              |
| `approval_authority_grant`        | `permission_widened` plus `policy_state_changed`                                                      |
| `automation_admission_only`       | `no_state_change` (admission only; no spend)                                                          |
| `unknown_must_block`              | Fail-closed; the action is never admitted from this state.                                            |

Rules (frozen):

1. Every privileged action above `inspect_only_read` MUST spend an
   approval ticket through the same path native tools use
   (ADR-0010); a tool / surface that bypasses the approval path
   denies with `approval_path_bypass_attempted`.
2. `unknown_must_block` is fail-closed; an action whose capability
   class is unresolved is denied, not best-effort admitted.
3. `automation_admission_only` is admission-time only — admitting
   an automation recipe is a `no_state_change` event; the recipe's
   later spend is evaluated separately.

### 2.3 Execution-location badges

Every privileged action MUST advertise one
`execution_location_badge_class` (re-exported from the
external-tool registry vocabulary):

```
local_in_process_badge
local_subprocess_same_device_badge
local_sandboxed_container_same_device_badge
local_companion_service_loopback_badge
remote_vendor_managed_service_badge
remote_self_hosted_service_badge
enterprise_gateway_brokered_service_badge
extension_provided_locus_badge
air_gapped_signed_bundle_badge
mocked_test_locus_badge
unknown_locus_must_be_disclosed_badge
```

Rules (frozen):

1. Every privileged action's disclosure surface MUST render the
   badge above adjacent chips, sheets, or summary rows; surfaces
   that fold the badge into a single icon still emit the
   addressable field.
2. `unknown_locus_must_be_disclosed_badge` is fail-closed and
   blocks the action with `ambiguous_execution_locus_must_disclose`.
3. A change of badge class between approval and execution is
   itself a renewal trigger (see §2.5).

### 2.4 Context-priority order

The composer plan resolves instruction-following authority strictly
in the following order (highest to lowest); a lower-priority class
NEVER overrides a higher-priority class. This ordering is closed.

```
1. system_scaffold
2. composer_plan_directive
3. designated_policy_file               (.aureline/ai/policy/*; signed admin authority)
4. platform_control_record              (typed approval / control authority)
5. repo_instruction_bundle              (AGENTS.md-style; first-party authored)
6. trusted_workspace_pinned_policy
7. trusted_user_profile_policy
8. user_turn_prompt                     (the active turn prompt, user-typed)
9. user_prior_turn_prompt
10. ai_prior_turn_response
11. tool_call_response                  (native tool output only)
12. retrieved_reference                 (citation-anchor-backed quote)
13. attached_data
14. fenced_tainted_data                 (never gains instruction authority)
```

Rules (frozen):

1. The ordering is **strict precedence**, not a guideline. A repo
   `AGENTS.md` line that contradicts the designated policy file
   loses; the designated policy file wins. Tainted evidence that
   contradicts the user turn prompt loses; the user turn prompt
   wins. The model's prior-turn response that contradicts the user
   turn prompt loses; the user turn prompt wins.
2. `fenced_tainted_data` NEVER gains instruction authority,
   regardless of where it appears, what it contains, or what the
   model's prior turn said about it.
3. `repo_instruction_bundle` MAY NOT widen permissions, retention,
   egress, or trust authority (see §4 — prohibited cases). It MAY
   narrow.
4. `tool_call_response` is admitted at instruction priority **only**
   when the response originated from a native tool whose row
   declares `trusted_first_party_local_tool_output` posture; every
   other tool response is `fenced_tainted_data`.

### 2.5 Designated policy-file rule

A `.aureline/ai/policy/*` file is a designated policy file when
**all** of the following hold:

1. The file path matches `.aureline/ai/policy/*` (or
   `.aureline/ai/policy/**/*` for nested admin bundles) under the
   workspace root.
2. The file's signing evidence resolves to an admitted admin signer
   (per ADR-0008 / ADR-0010); the signing evidence ref is non-null.
3. The file's `policy_file_role_class` is `designated_policy_file`.

Designated policy files carry trust authority **above** repo
instruction bundles and **above** all evidence / data classes.
Non-conforming when:

- A surface promotes an unsigned or workspace-pinned-only file to
  `trusted_policy` posture.
- A repo instruction bundle attempts to grant itself
  `trusted_policy` posture.
- A model response or a tainted evidence segment proposes to
  *modify* a designated policy file. Such a proposal is a
  `policy_file_self_modification_attempted` denial; modification
  of a designated policy file is reachable only by the
  `admin_via_signed_policy` or `platform_control_authority` actor
  class through the prompt-grammar contract's policy-mutation
  family.
- Designated policy files travel through any sink (support bundle,
  evidence packet, claim manifest) without their signing evidence
  ref. A designated policy file quoted without its signing
  evidence ref denies with
  `designated_policy_file_signing_evidence_missing`.

### 2.6 No-self-approve rule

The model MUST NOT authorize its own follow-on action. Concretely:

1. The `actor_class_for_action` on every privileged
   `approval_action_class_record` resolves from the user, the
   admin, the platform control authority, or a signed automation
   recipe. The values `model_attempt_self_authorize_denied`,
   `tainted_input_attempt_authorize_denied`, and
   `prior_turn_response_attempt_authorize_denied` exist only to
   record an attempted self-approval and emit a typed denial.
2. AI prose that says "approved", "authorized", "go ahead",
   "proceed", or otherwise asserts an approval is **not** an
   approval. The only admitted approval surface is the typed
   approval ticket (ADR-0010).
3. The rule applies identically across native tools and external
   tools. An external-tool return that says "the user approved"
   is `tainted_evidence`; the platform's own
   `platform_approval_record` is the only authoritative
   confirmation.
4. A surface that admits a privileged follow-on whose
   `actor_class_for_action` resolves to one of the three denial
   classes denies with `model_self_authorization_denied`,
   `tainted_input_authorization_denied`, or
   `prior_turn_response_authorization_denied` and the action does
   not proceed.

### 2.7 Approval renewal rule

The closed `approval_renewal_class` vocabulary is:

```
no_approval_required_inspect_only
existing_session_approval_sufficient
existing_per_invocation_approval_sufficient
fresh_approval_required_consequence_widened
fresh_approval_required_locus_changed
fresh_approval_required_capability_widened
fresh_approval_required_retention_widened
fresh_approval_required_egress_widened
fresh_approval_required_actor_changed
fresh_approval_required_after_tainted_input
fresh_approval_required_after_policy_epoch_change
mutation_blocked_no_renewal_admitted
```

A previously granted approval ticket is **insufficient** and a
fresh approval is required when **any** of the following changes
between approval and execution:

- the `action_capability_class` (e.g. from `local_reversible_edit`
  to `external_publish_irreversible`);
- any `action_effect_class` newly listed (e.g. `retention_widened`
  appears for the first time);
- the `execution_location_badge_class`;
- the `actor_class_for_action`;
- the policy epoch (ADR-0008);
- the workspace-trust state (ADR-0001);
- the assembly's `tainted_segment_count` or the prohibited-case
  set since approval (a tainted input that arrived after approval
  is treated as a context change).

`mutation_blocked_no_renewal_admitted` is the canonical state when
admin policy refuses to admit any renewal path — the action is
blocked and the surface routes the user to the prompt-grammar
contract's denial family.

### 2.8 Inspect-only downgrade rule

The closed `downgrade_class` vocabulary is:

```
no_downgrade_required
inspect_only_downgrade_after_tainted_input
inspect_only_downgrade_after_low_confidence
inspect_only_downgrade_after_validation_missing
inspect_only_downgrade_after_freshness_floor_unmet
inspect_only_downgrade_after_locus_unknown
mutation_blocked_no_downgrade_admitted
```

A privileged action MUST be downgraded to `inspect_only_read`
when **any** of the following hold:

1. The proposal cites tainted evidence as its driving rationale.
2. The proposal's confidence label resolves to `Low confidence`
   (per the AI copy guardrails contract).
3. The proposal's validation state is `Validation not run`,
   `Validation failed`, or `Validation mixed` and the action is
   above `local_reversible_edit`.
4. The freshness floor for the proposal's evidence is unmet.
5. The execution-location badge resolves to
   `unknown_locus_must_be_disclosed_badge`.

`mutation_blocked_no_downgrade_admitted` is reachable when admin
policy refuses to admit any downgrade path — the surface blocks
and routes to denial.

### 2.9 Blocked-mutation rule

A privileged mutation is **blocked** (no inspect-only downgrade,
no fresh-approval admission) when **any** of the following hold:

1. The proposal's driving rationale is `tainted_evidence` and the
   capability class is one of `policy_or_trust_mutation`,
   `capability_widening`, `retention_widening`, `egress_widening`,
   `provider_route_change`, `approval_authority_grant`, or
   `cross_workspace_recall`. Tainted evidence MUST NOT directly
   drive any of those.
2. The proposal targets modification of a designated policy file
   and the actor class is not `admin_via_signed_policy` or
   `platform_control_authority`.
3. The proposal attempts to grant tool permission, dispatch a
   branch agent, publish externally, or commit to the repo on the
   strength of a tainted input alone.
4. The proposal's `execution_location_badge_class` cannot be
   resolved (`unknown_locus_must_be_disclosed_badge`).
5. The actor class resolves to one of the three denial classes
   (`model_attempt_self_authorize_denied`,
   `tainted_input_attempt_authorize_denied`, or
   `prior_turn_response_attempt_authorize_denied`).

A blocked mutation emits `prompt_injection_mutation_blocked` and
the typed denial; the surface MUST route through the prompt-grammar
contract's denial family rather than fabricating "Continue", "OK",
or "Apply" copy.

## 3. Per-input-source projection rules (frozen)

The table below pins the maximum trust authority and the maximum
admitted instructional role per input-source class. A surface
that exceeds the maximum is non-conforming.

| Input-source class                  | Max trust authority   | Max instructional role     | Notes                                                                                                            |
|-------------------------------------|-----------------------|----------------------------|------------------------------------------------------------------------------------------------------------------|
| `repository_workspace_content`      | `trusted_first_party_data` | `attached_data`         | Repo code/content; never instruction authority on its own.                                                       |
| `repo_instruction_bundle_authored`  | `trusted_first_party_data` | `repo_instruction_bundle` | First-party authored; below `trusted_policy_file`. May narrow; never widen.                                      |
| `trusted_policy_file`               | `trusted_policy`      | `composer_plan_directive` (or higher when admin signs)  | Designated policy file; signed admin authority required.                                          |
| `trusted_workspace_pinned_policy`   | `trusted_first_party_data` | `repo_instruction_bundle` | Workspace owner pinned; below repo instruction bundle.                                                          |
| `trusted_user_profile_policy`       | `trusted_first_party_data` | `repo_instruction_bundle` | User-profile pinned; below trusted_workspace_pinned_policy.                                                     |
| `platform_approval_record`          | `trusted_control_record` | `composer_plan_directive` | Typed approval ticket; reads as control state, not user prose.                                                  |
| `platform_control_record`           | `trusted_control_record` | `composer_plan_directive` | Typed admin policy / control surface record.                                                                    |
| `docs_pack_excerpt`                 | `trusted_authority_quote` | `retrieved_reference`   | Authoritative quote; citation anchor required.                                                                   |
| `generated_reference_excerpt`       | `trusted_authority_quote` | `retrieved_reference`   | Same.                                                                                                            |
| `runbook_step_excerpt`              | `trusted_authority_quote` | `retrieved_reference`   | Same.                                                                                                            |
| `release_note_excerpt`              | `trusted_authority_quote` | `retrieved_reference`   | Same.                                                                                                            |
| `workspace_search_result`           | `trusted_first_party_data` | `attached_data`        | Re-derives nothing; quotes the result packet by id.                                                              |
| `workspace_diagnostic_capture`      | `trusted_first_party_data` | `attached_data`        | Diagnostic capture from the workspace.                                                                           |
| `terminal_command_output`           | `tainted_evidence`    | `fenced_tainted_data`   | Untrusted; default fence is `quoted_as_data_only`.                                                               |
| `log_capture`                       | `tainted_evidence`    | `fenced_tainted_data`   | Same.                                                                                                            |
| `tool_call_native_response`         | `data_only_no_instruction` | `tool_call_response`  | Admitted only when the tool is `trusted_first_party_local_tool` per the external-tool registry.                  |
| `tool_call_external_response`       | `tainted_evidence`    | `fenced_tainted_data`   | Default for every external / extension / connected-provider tool.                                                |
| `mcp_server_response`               | `tainted_evidence`    | `fenced_tainted_data`   | Same.                                                                                                            |
| `web_search_result`                 | `tainted_evidence`    | `fenced_tainted_data`   | Default fence: `quoted_as_data_only`.                                                                            |
| `external_docs_excerpt`             | `tainted_evidence`    | `fenced_tainted_data`   | Same; never `trusted_authority_quote` unless a citation anchor admits it.                                        |
| `connected_provider_payload`        | `tainted_evidence`    | `fenced_tainted_data`   | Same.                                                                                                            |
| `extension_proposed_input`          | `tainted_evidence`    | `fenced_tainted_data`   | Per ADR-0012; the composer never promotes extension-proposed bytes silently.                                     |
| `collaboration_remote_payload`      | `tainted_evidence`    | `fenced_tainted_data`   | Per the context-assembly contract.                                                                               |
| `user_authored_instruction`         | `data_only_no_instruction` | `user_turn_prompt`    | Only the actively-typed turn prompt; not pasted content.                                                          |
| `user_supplied_paste`               | `tainted_evidence`    | `fenced_tainted_data`   | Pastes are not instructions.                                                                                     |
| `user_supplied_file_attachment`     | `tainted_evidence`    | `fenced_tainted_data`   | Attached files are evidence, not instruction.                                                                    |
| `ai_prior_turn_response`            | `tainted_evidence`    | `fenced_tainted_data`   | Carry-over output rides the fence (the model never self-approves, §2.6).                                         |
| `runtime_diagnostic_capture`        | `tainted_evidence`    | `fenced_tainted_data`   | Crash / panic captures are tainted by default.                                                                   |
| `policy_quarantined_source`         | `policy_quarantined`  | `fenced_tainted_data` (summary-ref-only) | Body never inlined.                                                                                  |
| `unknown_unclassified_source`       | `unknown_must_treat_as_tainted` | `fenced_tainted_data` | Fail-closed.                                                                                              |

## 4. Prohibited cases (closed)

The closed `prohibited_case_class` vocabulary is the set of cases
that MUST be denied. The schema names every case as a typed value
so the audit trail records *which* prohibition fired, not "denied".

| Case                                                          | Description                                                                                                                                                       |
|---------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `model_self_authorization_attempted`                          | The model's prose claims to authorize a privileged follow-on. Denied with `model_self_authorization_denied`.                                                       |
| `repo_text_attempts_permission_widening`                      | Repo `AGENTS.md` / instruction bundle text proposes to widen permissions. Denied with `repo_text_widening_attempted`.                                              |
| `repo_text_attempts_retention_widening`                       | Same, for retention. Denied with `repo_text_widening_attempted`.                                                                                                  |
| `repo_text_attempts_egress_widening`                          | Same, for egress. Denied with `repo_text_widening_attempted`.                                                                                                     |
| `repo_text_attempts_provider_route_change`                    | Same, for provider route. Denied with `repo_text_widening_attempted`.                                                                                             |
| `repo_text_attempts_trust_state_widening`                     | Same, for workspace-trust state. Denied with `repo_text_widening_attempted`.                                                                                      |
| `hidden_cross_workspace_recall_attempted`                     | Cross-workspace recall without an explicit signed grant or platform control record. Denied with `cross_workspace_recall_denied_no_grant`.                          |
| `ambiguous_execution_locus_disclosed`                         | The action's execution-location badge resolves to `unknown_locus_must_be_disclosed_badge`. Denied with `ambiguous_execution_locus_must_disclose`.                  |
| `tainted_output_directly_driving_privileged_mutation`         | Tainted evidence is named as the proposal's driving rationale for a `policy_or_trust_mutation` / widening / cross-workspace / publish capability class. Denied.    |
| `tainted_output_minting_citation_anchor`                      | Tainted evidence proposes to mint or fabricate a citation anchor. Denied with `tainted_evidence_minting_citation_anchor`.                                          |
| `tainted_output_attempting_tool_permission_grant`             | Tainted evidence proposes to grant tool permission. Denied with `tainted_evidence_attempted_tool_grant`.                                                          |
| `tainted_output_attempting_branch_agent_dispatch`             | Tainted evidence proposes to dispatch a branch agent. Denied with `tainted_evidence_attempted_branch_agent_dispatch`.                                              |
| `tainted_output_attempting_external_publish`                  | Tainted evidence proposes to publish externally. Denied with `tainted_evidence_attempted_external_publish`.                                                       |
| `policy_file_self_modification_attempted`                     | A model response or tainted-evidence segment proposes to modify a designated policy file. Denied with `policy_file_self_modification_denied`.                      |
| `prior_turn_response_attempting_self_approval`                | A prior-turn AI response carries an "approved" claim. Denied with `prior_turn_response_authorization_denied`.                                                     |
| `external_tool_response_attempting_native_trust_promotion`    | An external tool's return claims first-party trust posture. Denied with `tainted_evidence_attempted_trust_promotion`.                                              |
| `approval_path_bypass_attempted`                              | A surface attempts a privileged action without spending an approval ticket. Denied with `approval_path_bypass_attempted`.                                          |

## 5. Audit, redaction, and boundary posture

Process-boundary constraints (frozen):

1. `tainted_input_source_record`,
   `prompt_injection_evaluation_record`,
   `prompt_injection_taint_audit_event_record`,
   `approval_action_class_record`,
   `privileged_action_disclosure_record`, and
   `approval_action_audit_event_record` cross the RPC boundary as
   typed payloads (ADR-0004). Raw retrieved-document bodies, raw
   terminal / log bodies, raw tool return bodies, raw prompt text,
   raw user-supplied text or file bytes, raw URLs, raw paths, and
   raw credential material never cross.
2. Mutation-journal entries, save manifests, support bundles, and
   claim manifests name the safeguard ids (`tainted_input_source_id`,
   `prompt_injection_evaluation_id`, `approval_action_class_id`,
   `privileged_action_disclosure_id`) and the typed vocabulary
   only.
3. Crash dumps and core files MUST NOT inherit unresolved
   safeguard records; a crash that lands mid-evaluation discards
   the record rather than persisting a partial axis set.
4. Any downstream surface that reads a safeguard record MUST
   preserve every prohibited-case value, taint class, capability
   class, effect class, location badge, renewal class, and
   downgrade class verbatim. Dropping any axis on handoff denies
   with `safeguard_record_axis_dropped_on_handoff`.

Audit streams (frozen):

| Stream                            | Frozen event ids                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
|-----------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `prompt_injection_taint`          | `prompt_injection_input_source_classified`, `prompt_injection_taint_class_assigned`, `prompt_injection_origin_locus_disclosed`, `prompt_injection_prohibited_case_observed`, `prompt_injection_downstream_effect_applied`, `prompt_injection_approval_renewal_required`, `prompt_injection_mutation_blocked`, `prompt_injection_branch_agent_dispatch_blocked`, `prompt_injection_cross_workspace_recall_blocked`, `prompt_injection_designated_policy_file_resolved`, `prompt_injection_denial_emitted`, `tainted_input_source_schema_version_bumped`                                  |
| `approval_action_class`           | `approval_action_classified`, `execution_location_badge_disclosed`, `context_priority_resolved`, `approval_renewal_required_observed`, `inspect_only_downgrade_applied`, `mutation_blocked_observed`, `model_self_authorization_denied_observed`, `tainted_input_authorization_denied_observed`, `prior_turn_response_authorization_denied_observed`, `cross_workspace_recall_denied_observed`, `policy_file_self_modification_denied_observed`, `ambiguous_execution_locus_denied_observed`, `approval_path_bypass_attempted_observed`, `approval_action_class_schema_version_bumped` |

Redaction defaults (frozen):

| Sink                          | Default inclusion                                                                                                                                                                                                                                          |
|-------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                  | Safeguard ids, typed enum values, audit-event ids, denial reasons. No raw bodies, paths, URLs, prompt text, or tool return bytes.                                                                                                                          |
| `traces_local`                | Same as `logs_local`. Span names MUST NOT include raw bodies / paths / URLs.                                                                                                                                                                               |
| `support_bundle`              | Full per-axis values, full prohibited-case enumeration, full approval-renewal / downgrade enumeration, opaque refs to assembly / segment / evidence / tool-call records. Raw bodies excluded.                                                              |
| `evidence_packet`             | Same as `support_bundle`; quoted by id from the AI evidence packet.                                                                                                                                                                                        |
| `claim_manifest`              | Full per-axis values. Raw bodies never included.                                                                                                                                                                                                           |
| `mutation_journal_entry`      | Ids, capability class, effect classes, location badge, renewal class, downgrade class. No raw bodies / URLs.                                                                                                                                               |
| `save_manifest`               | Same as `mutation_journal_entry`.                                                                                                                                                                                                                          |
| `crash_dump`                  | Opt-in only; redaction scan precedes packaging; denied by default for records whose `policy_context` references a managed policy bundle.                                                                                                                  |
| `terminal_transcript`         | Safeguard id and audit-event id only.                                                                                                                                                                                                                      |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

## 6. Schema-of-record posture

The eventual safeguard crate's Rust types are the source of truth.
The JSON Schema exports at
`schemas/ai/tainted_input_source.schema.json` and
`schemas/ai/approval_action_class.schema.json` are the cross-tool
boundary every non-owning surface reads. Adding a new
input-source class, taint class, prohibited-case class,
action-capability class, action-effect class, execution-location
badge class, context-priority class, approval-renewal class,
downgrade class, actor-class-for-action, audit-event id, or denial
reason is additive-minor and bumps the relevant `*_schema_version`;
repurposing any existing value is breaking and requires a new
decision row.

## 7. Non-goals at this milestone

Out of scope until a superseding decision row opens:

- Working AI inference; tool brokers; sandbox runtimes. This
  contract reserves the input-source / taint / approval-renewal /
  downgrade / blocked-mutation rule shape; the AI / tool / sandbox
  crates wire enforcement later.
- Provider routing, connected-provider auth flow bodies, and
  extension-effective-permission resolution bodies (those are
  reserved by ADR-0010 / ADR-0012).
- Prompt-composer UX. This contract reserves the safeguard
  evaluation record; the composer UX wires it later.
- The eventual safeguard crate's Rust types; the JSON Schema
  exports reserve the boundary shape until the crate lands.

These lines move only by opening a new decision row, not by
editing this contract.

## 8. Reuse guarantee

This contract is reusable by composer, inline-apply,
branch-agent, review-handoff, palette / command, and
support-replay flows without redefining input-source, taint,
prohibited-case, capability, effect, location, priority, renewal,
or downgrade vocabulary. A new AI-adjacent surface MUST:

1. Quote the input-source / taint / origin-locus / capability /
   effect / location-badge / context-priority / approval-renewal /
   downgrade / actor-class-for-action / prohibited-case
   vocabularies above verbatim.
2. Emit one `tainted_input_source_record` per input that reaches
   the assembly and one `prompt_injection_evaluation_record` per
   composed turn; emit one `approval_action_class_record` and one
   `privileged_action_disclosure_record` per privileged follow-on
   action proposed.
3. Preserve each axis as a separately addressable field on the
   record even when the UI folds them into one chip.
4. Honour the strict precedence order in §2.4; promoting a
   lower-priority input class above a higher-priority class is
   non-conforming.
5. Honour the no-self-approve rule (§2.6); model prose, tainted
   evidence, and prior-turn response NEVER carry approval
   authority.

## 9. Acceptance mapping

| Acceptance clause                                                                                                                                          | Resolved by                                                                                                                                                                                                                                                |
|------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Every AI input and tool-output fixture can be classified as trusted policy, control record, data, or tainted evidence with explicit downstream effect.     | §1 (input-source classes) + §2.1 (taint classes) + §3 (per-input-source projection rules) + the schema's `tainted_input_source_record` + the fixture corpus.                                                                                              |
| The contract preserves designated policy-file rules across native and external tools.                                                                       | §2.4 (precedence order) + §2.5 (designated policy-file rule) + §4 (`policy_file_self_modification_attempted`) + the schema's `policy_file_role_class` + the trusted-policy-file fixture.                                                                  |
| The contract preserves the no-self-approve rule across native and external tools.                                                                          | §2.6 (no-self-approve rule) + §4 (`model_self_authorization_attempted`, `prior_turn_response_attempting_self_approval`) + the approval-action schema's denial actor classes + the approval-renewal fixture.                                               |
| Fixtures cover repo instruction conflict, tainted terminal output, web excerpt, MCP response, trusted policy file, and approval renewal before mutation.   | `/fixtures/ai/prompt_injection_cases/`.                                                                                                                                                                                                                    |
