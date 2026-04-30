# AI prompt-injection / tainted-input safeguard worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/prompt_injection_and_taint_contract.md`](../../../docs/ai/prompt_injection_and_taint_contract.md)
and the schemas at
[`/schemas/ai/tainted_input_source.schema.json`](../../../schemas/ai/tainted_input_source.schema.json)
and
[`/schemas/ai/approval_action_class.schema.json`](../../../schemas/ai/approval_action_class.schema.json).

Every file is a multi-document YAML stream. The first document is
a `__fixture__` prelude summarising the scenario, the contract
sections it exercises, and the record kinds it produces. The
remaining documents are individual `tainted_input_source_record`,
`prompt_injection_evaluation_record`,
`prompt_injection_taint_audit_event_record`,
`approval_action_class_record`,
`privileged_action_disclosure_record`, and
`approval_action_audit_event_record` instances that conform to
the schemas.

No fixture embeds raw prompt text, raw retrieved-document bodies,
raw terminal / log bodies, raw tool return bodies, raw
user-supplied text or file bytes, raw URLs, raw paths, or raw
credential material. Every such field is an opaque ref.

## Cases

- [`repo_instruction_conflict.yaml`](./repo_instruction_conflict.yaml)
  — A repo-authored `AGENTS.md`-style instruction bundle proposes
  to widen tool permissions; a designated policy file under
  `.aureline/ai/policy/*` declines that widening. Strict precedence
  resolves to the designated policy file's directive; the repo
  text widening attempt records a typed denial on the audit
  stream.
- [`tainted_terminal_output.yaml`](./tainted_terminal_output.yaml)
  — A terminal-command-output capture (build / shell transcript)
  contains injected instructions claiming approval for a follow-on
  `commit_to_repo` action. The capture is `tainted_evidence`; the
  proposed action's `tainted_inputs_drove_proposal` is true; the
  approval-renewal class resolves to
  `mutation_blocked_no_renewal_admitted` for the
  `commit_to_repo` capability driven from a tainted source.
- [`tainted_web_excerpt.yaml`](./tainted_web_excerpt.yaml)
  — A web-search-result excerpt is admitted as
  `tainted_evidence` with the `quoted_as_data_only` fence on the
  assembly. The composed turn proposes only an `inspect_only_read`
  follow-on (`Open source` / `Open diff`); the safeguard's
  downstream effect is `inspect_only_no_mutation_admitted`.
- [`tainted_mcp_response.yaml`](./tainted_mcp_response.yaml)
  — A return value from an MCP server is admitted as
  `tainted_evidence` (`mcp_server_response` source class). The
  return claims a tool-permission grant; the safeguard records
  `tainted_output_attempting_tool_permission_grant` and blocks
  the `capability_widening` follow-on with
  `tainted_evidence_attempted_tool_grant`.
- [`trusted_policy_file.yaml`](./trusted_policy_file.yaml)
  — A designated AI policy file under
  `.aureline/ai/policy/admin_policy.yaml` is admitted with
  non-null signing-evidence to `trusted_policy` taint class and
  `composer_plan_directive` instructional role; the evaluation
  record's `designated_policy_file_resolved` is true; an attempted
  modification of the policy file by a model-driven follow-on
  denies with `policy_file_self_modification_denied`.
- [`approval_renewal_before_mutation.yaml`](./approval_renewal_before_mutation.yaml)
  — A privileged follow-on action's capability widens between
  approval and execution (a `local_reversible_edit` whose ticket
  was admitted now extends to `commit_to_repo` plus
  `external_publish_irreversible`). The safeguard records
  `fresh_approval_required_capability_widened`; once a renewed
  approval ticket is admitted, the action proceeds and the audit
  stream records `privileged_action_executed_after_renewal`.
