# AI prompt-composer plan, request-workspace, and prompt/tool-pack manifest contract

This document is the **product-wide contract** for how an AI turn's
prompt is assembled against a governed composer plan, how the explain
/ review / patch / test-generation / refactor / inline-completion /
background-branch-agent / review-handoff / support-replay flows
accumulate work into an isolated request workspace before dispatch,
and how the prompts and external-tool bindings those flows read are
packaged, versioned, signed, and rolled out through prompt-pack and
tool-pack manifests.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream AI / composer / review /
patch / tool-gateway surface's mint of its own copy, this document
wins and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ai/prompt_composer_plan.schema.json`](../../schemas/ai/prompt_composer_plan.schema.json)
  — boundary schema for the `prompt_composer_plan_record`,
  `prompt_composer_plan_section_record`, and
  `prompt_composer_plan_audit_event_record` shapes.
- [`/schemas/ai/request_workspace.schema.json`](../../schemas/ai/request_workspace.schema.json)
  — boundary schema for the `request_workspace_record`,
  `request_workspace_working_set_record`,
  `request_workspace_evidence_link_record`, and
  `request_workspace_audit_event_record` shapes.
- [`/schemas/ai/prompt_tool_pack_manifest.schema.json`](../../schemas/ai/prompt_tool_pack_manifest.schema.json)
  — boundary schema for the `prompt_pack_manifest_record`,
  `tool_pack_manifest_record`, and
  `prompt_tool_pack_manifest_audit_event_record` shapes.
- [`/schemas/ai/prompt_composer_session.schema.json`](../../schemas/ai/prompt_composer_session.schema.json)
  — boundary schema for the user-facing composer-session side:
  `prompt_composer_session_descriptor`,
  `prompt_composer_turn_draft_descriptor`,
  `prompt_composer_mention_descriptor`,
  `prompt_composer_attachment_descriptor`,
  `prompt_composer_slash_command_invocation_record`,
  `prompt_composer_predispatch_disclosure_record`, and
  `prompt_composer_session_audit_event_record`. The descriptor
  records share their id namespaces with the lighter
  `prompt_composer_session_record` /
  `prompt_composer_turn_draft_record` /
  `prompt_composer_mention_record` /
  `prompt_composer_attachment_record` carried on
  `schemas/ai/context_assembly.schema.json`; the descriptors are
  the schema-of-record for the composer-UI side and the
  context-assembly versions remain the assembly-side projection.
- [`/schemas/ai/request_workspace_ref.schema.json`](../../schemas/ai/request_workspace_ref.schema.json)
  — boundary schema for the typed cross-schema
  `request_workspace_ref_record` reference. Every surface that
  needs to point at a request workspace from outside its owning
  schema (the composer-session contract, the context-assembly
  contract, evidence packets, route / spend receipts,
  branch-agent dispatch packets, review-handoff packets, replay
  evidence) emits one of these typed references rather than an
  unscoped `opaque_id`.
- [`/fixtures/ai/prompt_plans/`](../../fixtures/ai/prompt_plans/)
  — worked-example corpus covering an inline-completion plan, an
  explain-flow plan, a patch-flow plan, a review-handoff plan, a
  background branch-agent plan, matching request workspaces, and
  one prompt-pack plus one tool-pack manifest each in production
  and one in canary rollout.
- [`/fixtures/ai/prompt_composer_cases/`](../../fixtures/ai/prompt_composer_cases/)
  — worked-example corpus for the composer session / turn-draft /
  mention / attachment / slash-command / pre-dispatch disclosure
  side, covering an empty draft, a single-file ask, a cross-repo
  ask with omitted context, a tainted-pasted-content ask, and a
  draft that launches a background branch-agent flow.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/ai/context_assembly_contract.md`](./context_assembly_contract.md) —
  the ordered-segment, trust-posture, fence-strategy, tainted-usage
  constraint, omit-reason, block-reason, redaction-reason,
  instructional-role, mention-kind, attachment-kind, turn-draft
  state, dispatch-target, provider-class, route-path-class,
  cost-visibility-class, instruction-bundle-kind, check-bundle-kind,
  and tool-call-outcome vocabularies are authored there and
  re-exported here without redefinition. Every AI turn still mints
  exactly one `ai_context_assembly_record`; this contract pins
  which `composer_plan_ref` and `request_workspace_ref` that
  record may cite.
- [`/docs/ai/evidence_replayability_contract.md`](./evidence_replayability_contract.md) —
  evidence packets already quote the assembly, the composer session,
  the route / spend receipts, and the tainted-fence set. This
  contract adds two more ids every packet MUST echo: the plan id
  with its version label, and the pack id with its version label.
  Downstream replay / support / parity tooling keys off those ids.
- [`/docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md) —
  provider / model / local-model-pack / external-tool rows. The
  plan narrows against these rows and may never widen; the
  tool-pack manifest bundles external-tool entries into a versioned
  unit.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md) —
  workspace-trust state and `deployment_profile_class` ride the
  `policy_context` on every record.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md) —
  `credential_handle_denied_always` and
  `secret_projection_denied_always` remain on every plan's and
  every workspace's denied-always set; the broker-owned redaction
  pass runs before bytes reach any persistent or exportable sink.
- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md) —
  admin policy MAY narrow which provider classes, routes, data
  classes, retention stances, and tools the plan and workspace
  admit; policy MAY NOT silently widen.
- [`/docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md) —
  `scope_filter_class` / `execution_context_id` re-exported
  without modification.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md) —
  every invocation the plan admits against a connected-provider
  route still spends an approval ticket; the plan narrows the
  admitted tools, it does not bypass the ticket.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md) —
  `redaction_class`, `freshness_class`, and `client_scope` are
  re-exported without modification.
- [`/docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md) —
  extension-authored prompt packs and tool packs ride the
  extension-author publisher class and the extension effective-
  permission surface; the manifest references the extension
  manifest by id.

If this document disagrees with those sources, those sources win and
this document plus the schemas are updated in the same change.

This document does not ship a live prompt-composer, a request-
workspace implementation, a pack-signing pipeline, a pack-distribution
channel, or a live pack-admission runtime. It freezes the contract
those implementations will read and write. The eventual
prompt-composer / request-workspace / pack-gateway crates' Rust types
are the schema of record; the JSON Schema exports here are the
cross-tool boundary every non-owning surface reads.

## Why freeze this now

Without one frozen contract, the product is free to let every AI
feature invent its own "system prompt", its own "retrieved context
block", its own scratch workspace for draft patches and review
threads, and its own notion of what "the prompt pack for this
deployment" means. The consequences are concrete:

1. A reviewer opens an evidence packet for a misbehaving turn and
   cannot answer *how was this prompt assembled* without reading the
   model's final response and reverse-engineering the sections — the
   very thing the replayability contract already forbids.
2. A repo-defined instruction file (`AGENTS.md`, `CONTRIBUTING.md`,
   a check script) quietly gains the authority to re-admit a tool,
   widen a data class, or lower a trust posture because no contract
   forces repo instructions to *narrow only*.
3. Explain, review, patch, and test-generation flows end up with
   three different scratch areas, three different export postures,
   and three different retention stories — so the clear-data /
   export / legal-hold controls cannot describe the product's
   behaviour.
4. Prompt text and tool bindings drift across deployments because
   no one can point at a single stable pack id, a version label, a
   signed digest, and a rollout state; evidence fixtures cite
   "the prompts" rather than "pack P@v3.7.1 digest sha256:…".

This contract closes the gap with **three governed boundary
schemas, one identity spine across them, and one additive-minor
versioning discipline**.

## Who reads this document

- **AI / prompt-composer / context-resolver authors** minting one
  `prompt_composer_plan_record` per governed plan, citing one
  `plan_pack_ref` per plan, and narrowing against the effective
  policy bundle without widening.
- **Explain / review / patch / test-generation / refactor /
  inline-completion / background-branch-agent / review-handoff
  authors** minting one `request_workspace_record` per flow,
  accumulating typed working-set bindings, linking evidence
  packets, and respecting the privacy / export posture.
- **Prompt-pack and tool-pack publisher authors** (first-party
  publisher, operator publisher, enterprise customer operator
  publisher, extension author publisher, user-imported publisher)
  minting one manifest per pack version, declaring a signing
  posture, a compatibility window, a rollout state, and a
  changelog.
- **Admin / policy / settings surface authors** narrowing which
  plans, packs, tools, data classes, providers, and flows a
  deployment profile admits.
- **Support / replay / parity-audit / claim-manifest authors**
  reconstructing every AI turn with the plan id, plan version,
  pack id, pack version, workspace id, assembly id, and evidence
  packet set quoted mechanically — no free-form prose required.
- **Security / emergency-action authors** quarantining or
  withdrawing a plan, a workspace, or a pack through the typed
  audit stream without re-authoring the schemas.

## 1. One contract, three boundary schemas, one identity spine

### 1.1 Frozen records

| Schema                                                    | Primary record kinds                                                                                                                      |
|-----------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| `schemas/ai/prompt_composer_plan.schema.json`             | `prompt_composer_plan_record`, `prompt_composer_plan_section_record`, `prompt_composer_plan_audit_event_record`                           |
| `schemas/ai/request_workspace.schema.json`                | `request_workspace_record`, `request_workspace_working_set_record`, `request_workspace_evidence_link_record`, `request_workspace_audit_event_record` |
| `schemas/ai/prompt_tool_pack_manifest.schema.json`        | `prompt_pack_manifest_record`, `tool_pack_manifest_record`, `prompt_tool_pack_manifest_audit_event_record`                                |

### 1.2 Identity spine across all three

Every AI invocation still mints exactly one
`ai_context_assembly_record` on
`schemas/ai/context_assembly.schema.json`. That record already
carries `composer_plan_ref` and `request_workspace_ref`. This
contract adds the following mechanical resolution rule: every
evidence packet, route receipt, spend receipt, and replay packet
that echoes the assembly MUST also echo —

- `plan_id` + `plan_version_label` (+ `content_digest` when
  present) resolved through `composer_plan_ref`;
- `plan_pack_ref` + pack `pack_version_label` (+
  `content_digest` when present) resolved through the plan's
  `plan_pack_ref`;
- `request_workspace_id` + `flow_class` +
  `privacy_export_posture_class` resolved through
  `request_workspace_ref`;
- (when the turn admitted tools) the `tool_pack_ref` and
  `pack_version_label` of each pack that contributed tools.

A reviewer reconstructs *which ordered sections were composed,
which data classes were admitted, which plan and pack versions
were live, which scope the flow ran under, and which evidence
packets were bound to the workspace* from those ids alone — not
from the final model response.

## 2. The prompt-composer plan

### 2.1 Ordered sections, one role each

Every plan carries an ordered list of sections, each with one
frozen `section_role` (`system_scaffold_prologue`,
`repo_instruction_bundle_block`, `repo_check_bundle_block`,
`workspace_pinned_instructions_block`,
`user_profile_instructions_block`, `pinned_context_block`,
`retrieved_reference_block`, `citation_anchor_block`,
`attached_data_block`, `ai_prior_turn_context_block`,
`tool_call_result_block`, `fenced_tainted_data_block`,
`user_turn_prompt_block`, `tool_calling_schema_block`,
`output_contract_epilogue`, `deterministic_replay_epilogue`,
`composer_plan_directive`, `deployment_profile_prologue`,
`workspace_trust_prologue`) and one
`admission_policy_class`. Section slot indices MUST be unique and
MUST start at 0.

`fenced_tainted_data_block` is the only section that MAY set
`hosts_tainted_data = true`. Tainted segments on the assembly
route to that section and ride the default fence strategy the
section declares. Silent placement of a tainted segment outside
the fenced section denies with
`tainted_segment_placed_outside_fenced_section`.

### 2.2 Admission policy is typed

A section declares exactly one admission policy:
`admit_only_listed_source_classes`,
`admit_listed_trust_postures_only`, `admit_only_pinned_segments`,
`admit_only_citation_anchor_backed`,
`admit_only_instruction_bundle_refs`,
`admit_only_check_bundle_refs`,
`admit_only_fenced_tainted_segments`, `deny_all_unless_pinned`,
or `deny_all_section_disabled`. A section that cannot resolve one
denies with `admission_policy_unresolved`.

### 2.3 Omission reasons are typed

When a candidate segment is admitted by policy but dropped under
budget / scope / freshness pressure, the plan declares the default
omission reason the composer should apply
(`plan_default_budget_exceeded`,
`plan_default_scope_excludes_target`,
`plan_default_freshness_floor_unmet`,
`plan_default_dedup_against_pinned`,
`plan_default_resolver_lowered_priority`,
`plan_default_policy_narrows_scope`,
`plan_default_user_deselected`,
`plan_default_pin_pressure_released_unpinned`). The resolver
writes the matching `omit_reason` (from
`schemas/ai/context_assembly.schema.json`) onto the omitted
segment; the context-assembly contract's "no silent omission"
rule still applies.

### 2.4 Repo-defined instructions and checks

Every plan may list `instruction_bundle_refs` (e.g.
`repo_agents_file`, `repo_contributing_file`,
`repo_style_guide`, `repo_review_checklist`, `repo_test_plan`,
`workspace_pinned_instructions`, `user_profile_instructions`,
`extension_injected_instructions`) and `check_bundle_refs` (e.g.
`repo_test_suite`, `repo_lint_suite`, `repo_type_check`,
`repo_format_check`, `repo_custom_check_script`,
`workspace_validation_plan`, `extension_injected_check`) by id.

**Repo-defined bundles may only narrow, never widen.** The plan
cites them by id; the plan does not re-mint their authority. A
repo bundle that tries to add a provider class, a route path, a
data class, a retention stance, or a tool that is not in the
effective policy bundle's admitted set is denied at admission
time with `plan_attempted_widening`. There is no silent widening
path.

### 2.5 Tool allowlist narrows, never widens

Every plan declares one `tool_narrowing_posture_class`
(`no_tools_admitted`, `allowlisted_subset_of_policy`,
`explicit_denylist_narrows_policy`, `inherits_policy_set`, or
`disabled_no_tool_posture`) and the matching subset / denial
lists. The policy bundle's admitted tool set is the ceiling; the
plan's `tool_allowlist_refs` is a subset of that ceiling.

A plan that admits a tool not currently admitted by the effective
policy bundle denies with `plan_tool_not_admitted_by_policy`.

### 2.6 Redaction posture on every plan

Every plan declares exactly one `redaction_class`
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, or `signing_evidence_only`). The
redaction class applies to every section output, every tool-call
lineage entry, and every evidence link the plan produces. The
broker-owned redaction pass (ADR-0007) runs before bytes reach
any sink; raw prompt text, raw document bodies, raw URLs, raw
paths, and raw credential material never cross this boundary
regardless of redaction class.

### 2.7 Plan lifecycle and replacement

A plan moves through the lifecycle states `plan_draft` →
`plan_in_review` → `plan_approved_not_yet_in_production` →
`plan_in_production` → (optionally)
`plan_deprecated_with_replacement` →
`plan_superseded_by_replacement`. `plan_withdrawn_by_operator`,
`plan_quarantined_pending_review`, and
`plan_disabled_policy_blocked` are the emergency / suspension
states. Only `plan_in_production` may be cited on a live composer
session; any other state denies with `plan_not_in_production`.

## 3. The request workspace

### 3.1 One flow, one workspace

Every AI flow opens exactly one `request_workspace_record` with
one `flow_class` (`explain_flow`, `review_flow`, `patch_flow`,
`test_generation_flow`, `refactor_flow`, `inline_completion_flow`,
`doc_generation_flow`, `diagnostic_triage_flow`,
`background_branch_agent_flow`, `review_handoff_flow`,
`support_replay_flow`, `mocked_test_flow`, `disabled_no_flow`).
The workspace is the isolated staging scope into which draft
patches, review threads, search-result packets, diagnostic sets,
generated artifacts, terminal transcript bindings, and log-span
bindings accumulate.

### 3.2 Scope binding is typed

The workspace declares one `scope_filter_class` re-exported from
`schemas/runtime/execution_context.schema.json` (ADR-0009):
`current_root`, `named_workset`, `sparse_slice`, `full_workspace`,
`docs_pack_only`, `policy_limited_view`, `review_workspace`,
`companion_surface`, or `remote_workspace`.

A segment admitted to a downstream assembly MUST respect this
scope filter. A segment whose source class falls outside the
workspace's scope filter denies with
`workspace_scope_exceeds_policy_allowance` — the plan narrows
inside the workspace's scope; it never widens past it.

### 3.3 Working-set bindings

Every working-set binding on the workspace names one
`working_set_kind` (`workspace_file_slice_set`,
`workspace_symbol_set`, `workspace_search_result_packet_ref`,
`workspace_diagnostic_set`, `workspace_mutation_journal_window`,
`workspace_graph_summary_ref`, `docs_pack_excerpt_set`,
`citation_anchor_set`, `patch_draft_ref`,
`review_thread_binding`, `test_candidate_set`,
`generated_artifact_set`, `terminal_transcript_binding`,
`log_span_binding`, `request_response_binding`,
`branch_agent_evidence_bundle_ref`,
`connected_provider_resource_binding`,
`user_supplied_attachment_set`) and carries one opaque
`binding_ref` per entry. Raw paths, raw file bodies, raw URLs,
raw transcripts, and raw credential material never appear; the
schema carries ids and typed vocabulary only.

### 3.4 Evidence refs travel with the workspace

Every `ai_evidence_packet_record`, branch-agent evidence bundle,
review-handoff evidence packet, and replay evidence record that
the flow produces is linked back to the workspace through a
`request_workspace_evidence_link_record` whose
`evidence_link_class` names the evidence kind. Reviewers walk the
link set to reconstruct the evidence trail without opening each
working-set binding individually.

### 3.5 Privacy / export posture

Every workspace declares exactly one
`privacy_export_posture_class`
(`included_by_default_in_support_bundle`,
`included_metadata_only_in_support_bundle`,
`opt_in_only_admin_export`,
`excluded_by_default_from_support_bundle`,
`excluded_always_support_bundle`,
`excluded_always_claim_manifest`,
`excluded_always_legal_hold_release`, or
`redacted_in_support_bundle_restricted`) and one
`retention_stance_within_workspace_class`
(`no_retention_session_scoped`,
`bounded_retention_session_bounded`,
`bounded_retention_user_scoped`,
`bounded_retention_admin_scoped`,
`bounded_retention_legal_hold_only`,
`retention_tied_to_assembly_lifetime`,
`retention_tied_to_evidence_lifetime`, or
`retention_policy_blocked`). A workspace whose posture is
`excluded_always_*` lists the matching sinks on
`excluded_always_from_supports`; an attempt to re-admit to those
sinks denies with `workspace_privacy_posture_widening_forbidden`.

### 3.6 Lifecycle, suspension, quarantine

A workspace moves monotonically through `workspace_open_active`
→ `workspace_open_idle` → `workspace_closed_completed` /
`workspace_closed_cancelled` / `workspace_closed_superseded`.
`workspace_suspended_policy_change` is entered on a
policy-epoch roll; no dispatch MAY proceed until the workspace
resumes. `workspace_quarantined_pending_review` denies every
dispatch. `workspace_disabled_policy_blocked` is the canonical
state for a workspace the admin policy has refused.

### 3.7 Isolation posture

The workspace declares one `isolation_posture_class`
(`isolated_per_session`, `isolated_per_turn`,
`shared_across_session_turns`,
`shared_across_branch_agent_dispatch`,
`shared_across_review_handoff`,
`shared_across_support_replay_only`). `isolated_per_turn` is the
default for explain / inline-completion flows;
`shared_across_session_turns` is admitted for patch / review /
refactor flows that deliberately accumulate work across turns.
`shared_across_branch_agent_dispatch` and
`shared_across_review_handoff` are used only on dispatches that
hand the workspace off to a background branch-agent or review
canvas.

## 4. The prompt-pack and tool-pack manifests

### 4.1 Stable id + version + digest

Every pack manifest carries a stable `pack_id` that is stable
across version bumps, a machine-readable `pack_version_label`
that changes on each version, and — on every
`pack_canary_staged_rollout` or `pack_in_production` manifest —
a `content_digest` naming the digest algorithm (`sha_256`,
`sha_512`, `blake3_256`) and lowercase hex digest of the pack's
canonical serialisation. Evidence fixtures cite the pack by
`pack_id`, `pack_version_label`, and (when available)
`content_digest`.

### 4.2 Owner and publisher

Every manifest names a `publisher_class`
(`first_party_product_publisher`,
`first_party_operator_publisher`,
`enterprise_customer_operator_publisher`,
`extension_author_publisher`, `user_imported_publisher`,
`mocked_test_publisher`, `disabled_no_publisher`) and an opaque
`publisher_identity_ref`. Raw operator identifiers, raw URLs,
and raw credentials never appear.

### 4.3 Compatibility notes are structured

Every manifest carries a `compatibility_window_record` listing
one or more `compatibility_posture_class` values
(`compatible_with_listed_schema_versions`,
`compatible_with_listed_deployment_profiles`,
`compatible_with_listed_provider_registry_revisions`,
`compatible_with_listed_external_tool_registry_revisions`,
`compatible_with_listed_runtime_build_identity`,
`incompatible_build_drift_detected`,
`compatibility_unknown_unverified`, or
`compatibility_withdrawn`) plus minimum schema versions (for the
prompt-composer plan schema, the request-workspace schema, the
context-assembly schema, the external-tool-registry schema, and
the provider-registry schema), plus the admitted deployment
profiles and client scopes. A tool-pack manifest MUST declare a
non-null `minimum_external_tool_registry_schema_version`.
`incompatible_build_drift_detected` and
`compatibility_withdrawn` deny production citation with
`pack_incompatible_build_drift`.

### 4.4 Rollout state

The manifest names exactly one `rollout_state_class` drawn from
a closed set: `pack_draft`, `pack_in_review`,
`pack_approved_not_yet_in_production`,
`pack_canary_staged_rollout` (requires a `canary_cohort_ref`),
`pack_in_production`, `pack_deprecated_with_replacement`
(requires a `replacement_pack_ref`),
`pack_withdrawn_by_operator`, `pack_superseded_by_replacement`
(requires a `supersedes_pack_ref`),
`pack_quarantined_pending_review`, or
`pack_disabled_policy_blocked`. Only `pack_in_production` and
`pack_canary_staged_rollout` MAY be cited on a live composer
session; anything else denies with `pack_not_in_production`,
`pack_quarantined_cannot_cite`, `pack_withdrawn_cannot_cite`, or
`pack_superseded_cannot_cite`.

### 4.5 Changelog linkage is mechanical

Every version (other than the very first) carries a
`changelog_entries` array whose entries name exactly one
`changelog_entry_class` (`added_new_plan`,
`updated_existing_plan`, `removed_plan`,
`added_new_tool_binding`, `updated_existing_tool_binding`,
`removed_tool_binding`, `narrowed_data_class_allowlist`,
`narrowed_provider_class_allowlist`,
`narrowed_route_path_class_allowlist`,
`narrowed_tool_allowlist`, `fixed_safety_defect`,
`bumped_compatibility_window`, `withdrawn_for_safety_review`,
`superseded_by_replacement_pack`, or
`editorial_only_no_behaviour_change`) and optionally an
`affected_plan_ref` / `affected_tool_entry_ref`. Support / replay
/ claim-manifest tooling diffs two pack versions mechanically
from this list; a free-form "what's new" paragraph is not a
substitute and is not part of the contract.

### 4.6 Pack ↔ plan and pack ↔ tool consistency

A `prompt_pack_manifest_record` names every plan in the pack on
`included_plan_refs`; every plan cites the pack on
`plan_pack_ref`. A plan cited under a pack that does not list it
denies with `pack_contains_prompt_or_tool_not_in_listed_refs`.
A `tool_pack_manifest_record` names every external-tool entry on
`included_tool_entry_refs`; a tool cited under a pack that does
not list it denies with the same reason.

## 5. Reviewer reconstruction from evidence

A reviewer who opens an evidence packet for an AI turn MUST be
able to answer — **without reading the final model response** —
the following questions purely from schema citations:

1. *Which composer plan version governed this turn?* From the
   evidence packet's `composer_plan_ref`, resolve
   `prompt_composer_plan_record.plan_version_label` (and
   `content_digest` when present).
2. *Which pack version shipped that plan?* From the plan's
   `plan_pack_ref`, resolve
   `prompt_pack_manifest_record.pack_version_label` (and
   `content_digest` when present).
3. *Which request workspace accumulated the working set?* From
   the evidence packet's `request_workspace_ref` (carried on the
   assembly), resolve `request_workspace_record.flow_class`,
   `scope_filter_class`, `privacy_export_posture_class`, and the
   typed working-set bindings.
4. *Which ordered sections were composed, with which admission
   policy and which admitted source / trust postures?* From
   `prompt_composer_plan_record.ordered_sections`.
5. *Which tools were admitted?* From
   `tool_narrowing_posture_class` plus `tool_allowlist_refs` and
   `tool_denylist_refs`; each resolves to an external-tool
   registry row and one tool-pack manifest by id and version.
6. *Which repo-defined instructions / checks fed the turn?* From
   the plan's `instruction_bundle_refs` and `check_bundle_refs`.
7. *Which omitted / redacted / policy-blocked / tainted
   segments were named explicitly?* From the assembly's segment
   records as before — but now the plan's
   `default_omission_reason_class` on each section lets a
   reviewer confirm the omit reasons were typed by the plan
   rather than invented at resolution time.

If any step fails to resolve, the evidence packet denies with a
typed reason from the contract's denial-reason vocabulary (for
example `composer_plan_ref_missing_on_assembly`,
`plan_pack_ref_unresolvable` via
`repo_instruction_bundle_ref_unresolvable`, or
`pack_digest_mismatch`). There is no "best-effort" reconstruction
path.

## 6. Narrowing discipline (MUST narrow, MUST NOT widen)

The narrowing posture is mechanical and non-negotiable:

- The effective **policy bundle** (ADR-0001 / ADR-0008) is the
  authoritative ceiling on admitted provider classes, route
  path classes, data classes, retention stances, tool entries,
  client scopes, and deployment profiles.
- A **prompt-composer plan** MAY narrow each axis to a strict
  subset of the policy bundle's admitted set, or declare
  `plan_inherits_policy_set` on that axis, or declare
  `plan_denies_all` on that axis. It MAY NOT admit any value
  outside the policy bundle's set.
- A **repo-defined instruction bundle** / **check bundle** MAY
  narrow the plan further (for example, a `repo_test_plan`
  pinning a narrower set of tests) but MAY NOT widen.
- A **request workspace** MAY narrow further again (for example,
  a `review_workspace` refusing to admit `workspace_file_slice`
  bindings outside the PR diff) but MAY NOT widen past the
  plan's admitted sets.
- A **pack manifest** MAY narrow which deployment profiles,
  client scopes, provider classes, or tools it ships against,
  but it MAY NOT widen past the boundary schemas' vocabularies.

Any attempt to widen at any level is denied with
`plan_attempted_widening`,
`workspace_privacy_posture_widening_forbidden`, or
`pack_deployment_profile_not_admitted` (and the matching
typed reason for the axis).

`credential_handle_denied_always` and
`secret_projection_denied_always` are on every denied-always set
on every plan, workspace, and pack; any attempt to admit them
denies with `plan_data_class_denied_always_violation` or
`workspace_data_class_denied_always_violation`.

## 7. Replay and downgrade explanations

Support and replay tooling reads evidence packets and answers
*why was this turn dispatched against plan version X of pack
version Y?* by:

1. Following the evidence packet's `composer_plan_ref` and
   `plan_pack_ref`.
2. Reading the plan's and pack's `rollout_state_class` at mint
   time.
3. Walking the pack's `changelog_entries` list between the cited
   version and the current in-production version. Each entry
   names exactly one `changelog_entry_class`; support tooling
   renders those classes verbatim as the downgrade /
   replay explanation.
4. Emitting one `prompt_tool_pack_manifest_downgrade_disclosed`
   audit event on the pack's audit stream so the replay artefact
   is linked to the audit trail.

This is the mechanical substitute for free-form "why did we roll
back the pack" prose; the free-form prose is optional release-
notes detail, not part of the contract.

## 8. Audit-stream reuse

The three schemas each declare an audit stream
(`prompt_composer_plan_audit_event_record`,
`request_workspace_audit_event_record`,
`prompt_tool_pack_manifest_audit_event_record`) that reuses the
existing `ai_context_*` audit-stream conventions (one typed
`audit_event_id`, one `policy_context`, one `minted_at`, nullable
refs for the record kinds the event touches, and a
`denial_reason` on every `*_denial_emitted` event). Support /
replay / claim-manifest / admin-reconciliation tooling already
knows how to read those streams.

## 9. Acceptance-criteria mapping

| Acceptance criterion                                                                                          | How the contract meets it                                                                                                                                                                                                       |
|---------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| A reviewer can reconstruct how a prompt was assembled without needing the final model response.              | Sections 1.2 and 5: every evidence packet echoes plan id + plan version + pack id + pack version + workspace id, and reviewers walk `ordered_sections` + `admission_policy_class` + segment records to reconstruct the prompt. |
| Repo-defined instructions or checks can narrow, but not widen, provider/data/tool permissions vs. policy.    | Section 6: the narrowing discipline is mechanical; widening at any level denies with `plan_attempted_widening` or the matching axis-typed reason. Repo bundles are cited by id and never re-mint authority.                    |
| Prompt/tool-pack versions are explicit in evidence fixtures and support replay or downgrade explanations.    | Section 4 + Section 7: every pack manifest carries `pack_id` + `pack_version_label` + `content_digest`; every evidence packet echoes those ids; pack changelog entries are typed with `changelog_entry_class`.                 |

## 10. Additive-minor change discipline

Adding a new `section_role`, `plan_purpose_class`,
`admission_policy_class`, `omission_reason_default_class`,
`narrowing_mode_class`, `plan_lifecycle_state_class`,
`tool_narrowing_posture_class`, `flow_class`, `working_set_kind`,
`privacy_export_posture_class`,
`retention_stance_within_workspace_class`,
`lifecycle_state_class`, `isolation_posture_class`,
`evidence_link_class`, `pack_class`, `publisher_class`,
`signing_posture_class`, `rollout_state_class`,
`compatibility_posture_class`, `changelog_entry_class`,
`audit_event_id`, or `denial_reason` value is **additive-minor**
and bumps the corresponding per-schema
`<name>_schema_version` const. Repurposing an existing value is
breaking and requires a new decision row plus a major schema
version bump.

## 11. The composer-session side: descriptors and the typed `request_workspace_ref`

Sections 1–10 freeze the **plan / workspace / pack** backbone that
governs an AI turn. Sections 11–16 freeze the **composer-session
side** — the user-facing surface where the user opens a draft,
mentions a symbol or a file, attaches a diagnostic bundle, runs a
slash command, reads the pre-dispatch disclosure, and sends the
turn. The composer-session contract sits *above* the plan /
workspace / pack layer: every composer session cites exactly one
plan and exactly one request workspace, every turn draft cites the
same, and every pre-dispatch disclosure renders verbatim the
required disclosure fields the user must read before the turn
leaves the composer.

### 11.1 Why a separate descriptor schema

The assembly contract
([`schemas/ai/context_assembly.schema.json`](../../schemas/ai/context_assembly.schema.json))
already reserves typed `prompt_composer_session_record`,
`prompt_composer_turn_draft_record`,
`prompt_composer_mention_record`, and
`prompt_composer_attachment_record` slots so later AI UX work
cannot smuggle scope or trust decisions into ad-hoc payloads.
Those records remain — they are the **assembly-side projection**
the assembly schema reads. The composer-session contract adds the
**descriptor** records on
[`schemas/ai/prompt_composer_session.schema.json`](../../schemas/ai/prompt_composer_session.schema.json)
that share the same `composer_session_id`, `turn_draft_id`,
`mention_id`, and `attachment_id` namespaces but carry the richer
shape the composer surface needs:

- explicit `disposition_class` per mention / attachment (attached,
  pinned, omitted, fenced, policy-blocked, redacted, denied);
- explicit `data_classes` per mention / attachment;
- explicit `tainted_fence_strategy` and `tainted_usage_constraints`
  on every fenced attachment;
- the typed `prompt_composer_slash_command_invocation_record`;
- the typed `prompt_composer_predispatch_disclosure_record`;
- the default `account_provider_path_class`, default account /
  provider / model identity refs, and default
  `approval_posture_class` on the session.

A descriptor record is the schema-of-record for the composer-UI
side; a corresponding context-assembly record is the assembly-side
projection. Both validate independently and share the same id;
neither widens what the other admits.

### 11.2 The typed `request_workspace_ref`

The previous revision used an unscoped `opaque_id` for the
`request_workspace_ref` field on the assembly. The composer-session
contract upgrades that to the typed
`request_workspace_ref_record` on
[`schemas/ai/request_workspace_ref.schema.json`](../../schemas/ai/request_workspace_ref.schema.json).
Every composer session, every turn-draft descriptor, and every
pre-dispatch disclosure record carries one of these typed
references rather than an unscoped id. Every evidence packet,
route receipt, spend receipt, branch-agent dispatch packet,
review-handoff packet, and replay-evidence record SHOULD do the
same.

The typed reference echoes the owning workspace's
`flow_class`, `scope_filter_class`, `isolation_posture_class`,
`privacy_export_posture_class`,
`retention_stance_within_workspace_class`, and
`lifecycle_state_class` so a reviewer answers "which flow, which
scope, which export posture, which lifecycle state" without
resolving the owning record. The reference is a snapshot at mint
time; downstream reads MUST re-resolve the owning record on
[`schemas/ai/request_workspace.schema.json`](../../schemas/ai/request_workspace.schema.json)
before dispatch and MUST deny on drift (`flow_class_drift_detected`,
`scope_filter_class_drift_detected`,
`isolation_posture_drift_detected`,
`privacy_export_posture_drift_detected`,
`retention_stance_drift_detected`, or
`lifecycle_state_drift_detected`) rather than silently picking one
side.

The reference also carries one `ref_origin_class` naming the
surface that minted it
(`minted_by_composer_session`,
`minted_by_composer_turn_draft`, `minted_by_assembly`,
`minted_by_route_receipt`, `minted_by_spend_receipt`,
`minted_by_evidence_packet`, `minted_by_branch_agent_dispatch`,
`minted_by_review_handoff`, `minted_by_replay_evidence`,
`minted_by_support_replay`, `minted_by_audit_event`) plus the
matching surface-specific ref (composer session ref, composer
turn-draft ref, assembly id ref, branch-agent dispatch ref,
review-handoff ref, evidence packet ref) so a reviewer can diff
which surfaces observed which lifecycle / posture for the same
underlying workspace.

## 12. Mentions and attachments: typed kinds and dispositions

### 12.1 The mention vocabulary

Every typed reference the user places in a composer turn draft is
a `prompt_composer_mention_descriptor` carrying exactly one
`mention_kind`. The vocabulary covers:

- workspace code:
  `symbol_mention`, `file_mention`, `file_slice_mention`,
  `buffer_slice_mention`, `editor_selection_mention`,
  `workset_mention`, `execution_context_mention`;
- search and graph:
  `search_result_mention`, `graph_node_mention`,
  `graph_edge_mention`, `graph_summary_mention`;
- docs and runbooks:
  `docs_anchor_mention`, `citation_anchor_mention`,
  `runbook_step_mention`;
- diagnostics and runtime:
  `diagnostic_mention`, `diagnostic_set_mention`,
  `terminal_transcript_mention`, `log_span_mention`,
  `request_response_mention`, `generated_artifact_mention`;
- notebooks:
  `notebook_cell_mention`, `notebook_output_mention`;
- collaboration and review:
  `review_thread_mention`, `branch_agent_dispatch_mention`,
  `connected_provider_resource_mention`,
  `collaboration_participant_mention`,
  `extension_resource_mention`;
- work tracking:
  `work_item_mention`, `work_item_relation_mention`.

A surface that resolves a free-text string into a path or URL
without minting a typed mention is non-conforming. Raw paths, raw
URLs, and raw symbol bodies never appear on the wire — every
mention carries a typed `target_ref` and (when admitted) typed
`data_classes` only.

### 12.2 The attachment vocabulary

Every typed attachment in a composer turn draft is a
`prompt_composer_attachment_descriptor` carrying exactly one
`attachment_kind`. The vocabulary covers:

- documents and references:
  `retrieved_document`, `docs_pack_excerpt_attachment`,
  `citation_anchor_bundle`;
- workspace excerpts:
  `editor_selection_excerpt`, `buffer_slice_excerpt`,
  `file_slice_excerpt`, `graph_summary_excerpt`,
  `workspace_slice_bundle`;
- diagnostics and runtime captures:
  `diagnostic_bundle`, `terminal_log_capture`, `log_span_capture`,
  `request_response_payload`, `generated_artifact_excerpt`;
- notebooks:
  `notebook_cell_excerpt`, `notebook_output_excerpt`;
- user-supplied content:
  `user_supplied_text`, `user_supplied_file`;
- collaboration / external:
  `branch_agent_evidence_bundle`, `review_thread_bundle`,
  `work_item_summary_bundle`,
  `connected_provider_resource_bundle`,
  `extension_resource_bundle`.

Attachments that name `retrieved_document`,
`docs_pack_excerpt_attachment`, or `citation_anchor_bundle` MUST
list at least one `citation_anchor_ref`; reviewers walk those
anchors to confirm the authority claim. Other attachment kinds MAY
omit `citation_anchor_refs` when the attachment is not authority-
backed (e.g. a user-supplied paste).

### 12.3 Dispositions: attached, omitted, pinned, fenced, blocked, redacted, denied

Every mention and every attachment carries exactly one
`disposition_class` from
`schemas/ai/prompt_composer_session.schema.json`:

- `attached_default` — admitted with no extra posture;
- `pinned_by_user` / `pinned_by_composer_plan` — survives budget
  pressure;
- `omitted_under_budget`, `omitted_outside_scope`,
  `omitted_freshness_floor_unmet`,
  `omitted_dedup_against_pinned`, `omitted_user_deselected`,
  `omitted_policy_narrows_scope` — typed omission reasons that
  ride the assembly's `omit_reason` vocabulary so a reviewer can
  cross-walk composer-side omits to assembly-side omits;
- `fenced_tainted_default`, `fenced_tainted_user_pasted`,
  `fenced_tainted_tool_return`,
  `fenced_tainted_remote_collaborator`,
  `fenced_tainted_connected_provider`,
  `fenced_tainted_extension_proposed` — fenced under the tainted-
  content fence; the descriptor MUST also name a
  `tainted_fence_strategy` and a non-empty
  `tainted_usage_constraints` set re-exported from the
  context-assembly contract verbatim;
- `policy_blocked_workspace_trust`,
  `policy_blocked_admin_policy`,
  `policy_blocked_extension_permission`,
  `policy_blocked_connected_provider_policy`,
  `policy_blocked_secret_projection` — blocked by typed policy;
- `redacted_under_broker_pass` — admitted with the broker-owned
  redaction pass (ADR-0007) applied;
- `denied_data_class_denied_always` — the canonical disposition
  for a mention / attachment that resolved to
  `credential_handle_denied_always` or
  `secret_projection_denied_always`; the composer denies rather
  than silently dropping.

A mention or attachment whose `disposition_class` is unresolved
denies with `attachment_disposition_unresolved`. A `fenced_tainted_*`
disposition without a fence strategy denies with
`tainted_attachment_missing_fence_strategy`. A `policy_blocked_*`
disposition without a typed block reason on the resulting assembly
segment denies with `attachment_disposition_requires_block_reason`.

### 12.4 Editor selections, graph references, docs snippets, diagnostics, work items, notebook cells, terminal/log excerpts

The contract pins exactly which mention / attachment kinds the
composer uses to carry each source class. The surface MUST emit
the typed mention or attachment; minting an `opaque_id` reference
without one of the typed kinds denies with
`mention_kind_unresolved` or `attachment_kind_unresolved`.

| Source                         | Typed mention kind                               | Typed attachment kind                           | Default disposition | Notes                                                                 |
|--------------------------------|--------------------------------------------------|-------------------------------------------------|---------------------|-----------------------------------------------------------------------|
| Editor selection               | `editor_selection_mention`                       | `editor_selection_excerpt`                      | `attached_default`  | Trusted first-party. Carries `workspace_code_slice_allowed`.          |
| File reference                 | `file_mention` / `file_slice_mention`            | `file_slice_excerpt`                            | `attached_default`  | Trusted first-party. Buffer-side reads use `buffer_slice_*`.          |
| Graph reference                | `graph_node_mention` / `graph_edge_mention` / `graph_summary_mention` | `graph_summary_excerpt`        | `attached_default`  | Trusted first-party. Carries `workspace_graph_summary_allowed`.       |
| Docs snippet                   | `docs_anchor_mention` / `citation_anchor_mention` / `runbook_step_mention` | `docs_pack_excerpt_attachment` / `citation_anchor_bundle` | `attached_default` | Trusted authority. MUST carry `citation_anchor_refs`.                 |
| Diagnostic                     | `diagnostic_mention` / `diagnostic_set_mention`  | `diagnostic_bundle`                             | `attached_default`  | Trusted first-party. Carries `workspace_diagnostic_allowed`.          |
| Work-item link                 | `work_item_mention` / `work_item_relation_mention` | `work_item_summary_bundle`                    | `attached_default`  | Trusted first-party when authored locally; untrusted when imported.   |
| Notebook cell                  | `notebook_cell_mention` / `notebook_output_mention` | `notebook_cell_excerpt` / `notebook_output_excerpt` | `attached_default` | Cell inputs are trusted; outputs of executed cells follow the same posture as `generated_artifact_excerpt` (untrusted unless reviewed). |
| Terminal / log excerpt         | `terminal_transcript_mention` / `log_span_mention` | `terminal_log_capture` / `log_span_capture`   | `fenced_tainted_default` | Untrusted log capture by default; downstream usage constraints include `must_not_gain_tool_permission` and `must_preserve_fence_in_downstream_packet`. |
| Pasted external content        | `extension_resource_mention` / `connected_provider_resource_mention` (rare) | `user_supplied_text` / `user_supplied_file` | `fenced_tainted_user_pasted` | Untrusted user-supplied. Composer plan applies fence strategy `instruction_stripped` or `quoted_as_data_only`. |

The above is a default; admins / repo-defined instruction bundles
MAY narrow further (e.g. omit notebook outputs entirely under
`policy_limited_view`) but MAY NOT widen.

## 13. Slash commands

Every slash command the user runs in the composer is a typed
`prompt_composer_slash_command_invocation_record` carrying:

- one `slash_command_class` from a closed vocabulary
  (`explain_slash_command`, `review_slash_command`,
  `patch_slash_command`, `test_generation_slash_command`,
  `refactor_slash_command`, `doc_generation_slash_command`,
  `diagnostic_triage_slash_command`,
  `background_branch_agent_slash_command`,
  `review_handoff_slash_command`, `search_slash_command`,
  `navigate_slash_command`, `attach_slash_command`,
  `pin_slash_command`, `unpin_slash_command`,
  `omit_slash_command`, `scope_change_slash_command`,
  `model_change_slash_command`, `provider_change_slash_command`,
  `approval_request_slash_command`,
  `approval_revoke_slash_command`,
  `support_replay_slash_command`,
  `extension_provided_slash_command`,
  `user_authored_macro_slash_command`,
  `disabled_no_slash_command`);
- one `slash_command_identity_ref` (opaque id of the concrete
  slash-command identity);
- an optional `canonical_command_id_ref` so the slash command
  surfaces the same governance row as the palette / menu / CLI
  surfaces under M00-374's invocation-result-and-parity contract;
- a typed `argument_refs` array of opaque argument ids — raw
  argument strings never appear;
- typed `produced_mention_refs` and `produced_attachment_refs`
  arrays so a reviewer can answer "which mentions and attachments
  did this slash command produce";
- one `invocation_state` from the closed vocabulary
  (`invocation_open`, `invocation_argument_validation_pending`,
  `invocation_disclosure_pending`,
  `invocation_approval_pending`, `invocation_dispatched`,
  `invocation_completed_success`,
  `invocation_completed_partial`, `invocation_failed`,
  `invocation_cancelled`, `invocation_superseded`,
  `invocation_denied_by_policy`);
- a non-null `denial_reason` whenever
  `invocation_state = invocation_denied_by_policy`.

Slash commands authored by extensions
(`extension_provided_slash_command`) ride the extension manifest
permission surface (ADR-0012); slash commands authored by users
as macros (`user_authored_macro_slash_command`) ride the macro-
authoring controls. Both kinds may only narrow what the
first-party command vocabulary admits — they may not widen.

## 14. The pre-dispatch disclosure card

Every turn draft that is not in `draft` state MUST cite a
`prompt_composer_predispatch_disclosure_record`. The disclosure
record is the typed pre-dispatch card the composer renders
verbatim before a turn leaves the composer; the user reads it,
acknowledges it, and only then does the dispatch fire.

### 14.1 Required disclosure fields

The disclosure's `disclosure_fields` array MUST contain at least
the following typed fields (every fixture and every conformant
implementation enforces this):

- `scope_filter_disclosed` — names the request workspace's
  `scope_filter_class`;
- `target_context_disclosed` — names the typed mentions /
  attachments / pins that establish the target context;
- `active_account_disclosed` — names the active account identity
  ref (or the `no_account_local_only` lane);
- `active_provider_disclosed` — names the active provider identity
  ref (or the `disabled_no_provider` lane);
- `active_model_disclosed` — names the active model identity ref;
- `route_path_disclosed` — names the route path class on the
  attached `route_plan_placeholder_ref`;
- `cost_visibility_disclosed` — names the cost visibility class
  on the attached `route_plan_placeholder_ref` /
  `spend_plan_placeholder_ref`;
- `approval_posture_disclosed` — names the
  `approval_posture_class` and (when relevant) the approval
  ticket ref;
- `request_workspace_ref_disclosed` — discloses the typed
  `request_workspace_ref_record` (echoing flow / scope / posture
  / lifecycle).

A disclosure that lacks any required field denies with
`required_disclosure_field_missing`.

### 14.2 Optional disclosure fields

The disclosure MAY also include:

- `branch_agent_dispatch_intent_disclosed` (required when the
  draft's `dispatch_target_class = background_branch_agent`);
- `review_handoff_intent_disclosed` (required when the draft's
  `dispatch_target_class = review_handoff`);
- `freshness_class_disclosed`;
- `redaction_class_disclosed`;
- `instruction_bundle_refs_disclosed`,
  `check_bundle_refs_disclosed`;
- `tool_allowlist_disclosed`,
  `data_class_allowlist_disclosed`;
- `policy_epoch_disclosed`,
  `deployment_profile_disclosed`,
  `workspace_trust_state_disclosed`;
- `tainted_attachment_count_disclosed`,
  `omitted_attachment_count_disclosed`,
  `policy_blocked_attachment_count_disclosed`,
  `redacted_attachment_count_disclosed`.

The four count fields are always disclosed even when the count is
zero — this lets the audit stream prove a turn was minted with the
disclosure rendered, not that the user "happened not to see" the
count.

### 14.3 Route / spend receipt placeholders

The disclosure record carries placeholder opaque refs for the
`ai_route_plan_record`, `ai_spend_plan_record`,
`ai_route_receipt_record`, `ai_spend_receipt_record`,
`ai_branch_agent_dispatch_record`, and review-handoff packet that
will be minted at dispatch:

- `route_plan_placeholder_ref`,
  `spend_plan_placeholder_ref` — set on
  `disclosure_state = disclosure_ready` and onward;
- `route_receipt_placeholder_ref`,
  `spend_receipt_placeholder_ref` — null pre-dispatch; populated
  when the disclosure is reused as the audit record for the
  dispatch;
- `branch_agent_dispatch_placeholder_ref` — required (non-null)
  when `branch_agent_dispatch_intent_disclosed` is in
  `disclosure_fields`;
- `review_handoff_placeholder_ref` — required (non-null) when
  `review_handoff_intent_disclosed` is in `disclosure_fields`.

The placeholders mean the composer never has to invent ad-hoc glue
fields for "this draft will turn into route receipt X" once the
dispatch happens — the same record carries the ids.

### 14.4 Disclosure state lifecycle

Disclosures move through the closed lifecycle
`disclosure_drafting` → `disclosure_ready` →
`disclosure_revealed_to_user` → `disclosure_user_acknowledged` →
(optionally) `disclosure_dispatch_locked_pending_approval` →
`disclosure_dispatch_authorised`. `disclosure_cancelled`,
`disclosure_denied_by_policy`, and `disclosure_superseded` are the
suspension / terminal states; a disclosure in
`disclosure_denied_by_policy` MUST carry a non-null
`denial_reason`.

## 15. Composer-side reviewer reconstruction

A reviewer who opens a composer-side audit packet MUST be able to
answer — without reading the final model response or the raw user
prompt text — the following questions purely from descriptor
records on `schemas/ai/prompt_composer_session.schema.json`:

1. *Which composer session was open?* From the
   `prompt_composer_session_descriptor`'s `composer_session_id`,
   `composer_plan_ref`, and `request_workspace_ref` (the typed
   record).
2. *Which turn drafts were minted?* From
   `active_turn_draft_refs` and the per-draft
   `prompt_composer_turn_draft_descriptor` records.
3. *Which mentions and attachments rode each draft, and with
   which disposition?* From `mention_descriptor_refs` and
   `attachment_descriptor_refs` plus each descriptor's
   `disposition_class`, `data_classes`, `tainted_fence_strategy`,
   `tainted_usage_constraints`, and (for attachments)
   `citation_anchor_refs` / `freshness_class`.
4. *Which slash commands fired?* From
   `slash_command_invocation_refs` plus each invocation's
   `slash_command_class`, `argument_refs`,
   `produced_mention_refs`, `produced_attachment_refs`, and
   `invocation_state`.
5. *Which pre-dispatch disclosure was rendered, and was it
   acknowledged?* From `predispatch_disclosure_ref` plus the
   disclosure's `disclosure_state`,
   `account_provider_path_class`, account / provider / model
   identity refs, `approval_posture_class`, and the typed
   `disclosure_fields` array.
6. *Which receipts / dispatches were minted post-dispatch?* From
   the draft's `route_receipt_ref`, `spend_receipt_ref`,
   `branch_agent_dispatch_ref`, and `review_handoff_ref` (and
   the disclosure's matching placeholder refs).

If any step fails to resolve, the audit packet denies with the
matching typed reason from
`schemas/ai/prompt_composer_session.schema.json`'s `denial_reason`
vocabulary. There is no "best-effort" composer-side reconstruction
path.

## 16. Composer-side narrowing discipline

The composer-side surface adds two more rungs to the narrowing
discipline of section 6:

- A **composer session** MAY narrow further still (for example, a
  session opened with `default_account_provider_path_class =
  no_account_local_only` refusing to accept any
  `connected_provider_payload_allowed` data class on a mention or
  attachment) but MAY NOT widen past the request workspace's
  admitted sets.
- A **turn-draft descriptor** MAY narrow per-draft (for example,
  a `patch_flow` draft refusing to admit
  `extension_proposed_context_allowed` for one specific draft)
  but MAY NOT widen past the session's admitted sets.

Any attempt to widen at the composer-session or turn-draft level
denies with `mention_data_class_denied_always_violation`,
`attachment_data_class_denied_always_violation`, or
`required_disclosure_field_missing` (as appropriate). A
`request_workspace_ref` whose echoed flow / scope / posture /
lifecycle disagrees with the owning workspace at resolution time
denies with the matching `*_drift_detected` reason on
`schemas/ai/request_workspace_ref.schema.json`.

## 17. Additive-minor change discipline (composer-session)

Adding a new `mention_kind`, `attachment_kind`,
`slash_command_class`, `slash_command_invocation_state`,
`attachment_disposition_class`, `disclosure_field_class`,
`approval_posture_class`, `account_provider_path_class`,
`predispatch_disclosure_state_class`, `turn_draft_state`,
`dispatch_target_class`, `audit_event_id`, or `denial_reason`
value on `schemas/ai/prompt_composer_session.schema.json` is
**additive-minor** and bumps the
`prompt_composer_session_schema_version` const. Adding a new
`flow_class`, `scope_filter_class`, `isolation_posture_class`,
`privacy_export_posture_class`,
`retention_stance_within_workspace_class`,
`lifecycle_state_class`, `ref_origin_class`, `audit_event_id`, or
`denial_reason` value on
`schemas/ai/request_workspace_ref.schema.json` is **additive-minor**
and bumps the `request_workspace_ref_schema_version` const.
Repurposing an existing value on either schema is breaking and
requires a new decision row plus a major schema version bump.

## 18. Out of scope at this revision

Explicitly out of scope:

- Shipping a prompt-composer UI or a pack admin console.
- Building the pack-distribution channel, mirror continuity, or
  signing pipeline.
- Implementing the live request-workspace / patch-draft /
  review-thread / terminal-transcript binding runtime.
- Authoring concrete prompt text, retrieval strategies, or tool
  binding schemas for any specific flow.
- Fixing a concrete provider / model pairing for any specific
  plan.
- Implementing the live composer surface, the slash-command
  parser, the pre-dispatch disclosure renderer, or working
  inference execution.
- Provider-specific message rendering or final composer UI
  chrome.

The contract freezes the shape those implementations will read
and write; the implementations themselves land in later
milestones.
