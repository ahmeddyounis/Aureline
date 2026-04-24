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
- [`/fixtures/ai/prompt_plans/`](../../fixtures/ai/prompt_plans/)
  — worked-example corpus covering an inline-completion plan, an
  explain-flow plan, a patch-flow plan, a review-handoff plan, a
  background branch-agent plan, matching request workspaces, and
  one prompt-pack plus one tool-pack manifest each in production
  and one in canary rollout.

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

## 11. Out of scope at this revision

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

The contract freezes the shape those implementations will read
and write; the implementations themselves land in later
milestones.
