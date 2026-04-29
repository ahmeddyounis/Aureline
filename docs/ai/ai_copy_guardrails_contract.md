# AI copy guardrails, confidence language, and explain/action contract

This document is the product-wide contract for wording on AI-adjacent
surfaces. It freezes how certainty, evidence, context, teaching, and
action proposals are described before assistant features spread across
editors, review, help, docs, run, and support surfaces.

The contract is normative. Where this document disagrees with the
source product, architecture, technical-design, UI/UX, or design-system
specification, the source wins and this document updates in the same
change. Where a downstream AI surface invents looser copy, this
document wins and the surface is non-conforming.

Companion artifacts:

- [`/artifacts/ai/approved_ai_terms.yaml`](../../artifacts/ai/approved_ai_terms.yaml)
  - machine-readable register of preferred terms, labels, and action
  controls for high-trust AI surfaces.
- [`/artifacts/ai/forbidden_ai_terms.yaml`](../../artifacts/ai/forbidden_ai_terms.yaml)
  - machine-readable rejection list for overclaiming, review-free
  mutation, false validation, false freshness, and route-obscuring copy.
- [`/fixtures/ai/copy_guardrail_cases/`](../../fixtures/ai/copy_guardrail_cases/)
  - worked examples covering cited educational answers, low-confidence
  proposals, stale-doc explanations, and rejected overclaims.

This contract composes with and does not replace:

- [`docs/ai/context_assembly_contract.md`](./context_assembly_contract.md)
  for included, omitted, pinned, redacted, policy-blocked, and tainted
  context segments plus route and spend truth.
- [`docs/ai/evidence_replayability_contract.md`](./evidence_replayability_contract.md)
  for graded replayability, omitted capture classes, provider
  availability, and mutation lineage.
- [`docs/ai/provider_model_registry_contract.md`](./provider_model_registry_contract.md)
  for provider, model, execution-locus, region, retention, quota, and
  data-class disclosure.
- [`docs/ai/prompt_composer_contract.md`](./prompt_composer_contract.md)
  for prompt-composer plans, request workspaces, prompt packs, and tool
  packs.
- [`docs/ai/model_graduation_and_budget_contract.md`](./model_graduation_and_budget_contract.md)
  for rollout state, budget routing, route fallback, and route selection
  disclosure.
- [`docs/governance/truth_and_degraded_state_vocabulary.md`](../governance/truth_and_degraded_state_vocabulary.md)
  for `ai_inferred_truth`, degraded-state tokens, and the rule that AI
  answers are never claim-bearing on their own.
- [`docs/ux/contextual_teaching_contract.md`](../ux/contextual_teaching_contract.md)
  for teaching surfaces, source refs, and the rule that `Explain` and
  `Do` remain separate controls.
- [`docs/ux/shell_interaction_safety_contract.md`](../ux/shell_interaction_safety_contract.md)
  for preview, apply, validation, revert, and `ai_apply_canvas`
  behavior.
- [`docs/docs_integrity/citation_and_reference_contract.md`](../docs_integrity/citation_and_reference_contract.md)
  and [`schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  for citation anchors.
- [`artifacts/copy/controlled_glossary.yaml`](../../artifacts/copy/controlled_glossary.yaml)
  for shared state-label and button-label discipline.

## Scope

This contract applies to any trust-sensitive AI surface. A surface is
trust-sensitive when AI wording could influence a user to:

- trust an explanation about code, docs, policy, runtime state, or
  provider state;
- accept an edit, refactor, quick fix, review comment, generated test,
  docs suggestion, command, package change, run, sandbox execution, or
  provider handoff;
- decide whether evidence is current, complete, validated, replayable,
  safe, or policy-admitted;
- publish, export, submit, or share AI-derived content.

Examples include assistant answers, inline assist, educational AI,
architecture explainers, review summaries, action proposal cards,
AI-generated docs suggestions, test-generation suggestions, run
helpers, provider-route disclosure sheets, replay explanations, support
exports, and help surfaces that summarize an AI answer.

Out of scope: model prompting strategy, visual styling, ranking logic,
and runtime implementation. This document governs copy and control
semantics only.

## Required copy facts

Every rendered AI answer, AI suggestion, or AI action proposal MUST have
machine-readable backing for these facts, even when the UI projects them
as chips, labels, or compact rows:

| Fact | Required source | Copy rule |
|---|---|---|
| Truth class | `truth_class:ai_inferred_truth` or a directly quoted non-AI anchor | AI wording is provisional unless it is an attributed quote. |
| Evidence refs | citation anchors, file/symbol refs, search result packet refs, run/test refs, replay refs, or provider disclosure refs | Claims name evidence before confidence. |
| Context posture | included, omitted, pinned, redacted, policy-blocked, partial, or tainted context classes | Copy names meaningful gaps instead of implying full context. |
| Confidence label | approved term from the confidence vocabulary | Confidence is qualitative and reasoned, not a magic score. |
| Validation state | validation plan and outcome when present | "Validated", "passed", or "safe" require a named validation basis. |
| Provider route | provider/model/locus/region/retention/data-class disclosure when the answer left local inference or used a routed model | Route disclosure cannot be hidden behind generic AI branding. |
| Action boundary | explain, source navigation, preview, diff, sandbox, review, or apply | Explanation controls never perform direct mutation. |
| Reversibility | checkpoint, revert class, sandbox boundary, or manual-review state for mutating proposals | Action copy names the reversible next step or blocks direct apply. |

Missing one of these facts does not always block rendering. It does
block overclaiming. For example, an uncited answer may render as a
low-confidence explanation with an omitted-context disclosure; it may
not render as an evidence-backed recommendation.

## Evidence-first confidence language

AI confidence copy MUST follow this order:

1. Name the evidence basis.
2. State the inference.
3. Name the confidence label and reason.
4. Name relevant gaps, freshness limits, or omitted context.
5. Offer the next safe action.

Preferred pattern:

```text
Based on {evidence refs}, {inference}. Confidence: {approved label},
because {reason}. Context limits: {omitted/partial/stale facts}. Next:
{safe action}.
```

Surfaces may compress the pattern into chips, but the same facts must be
available through the detail route, support export, and accessibility
label.

### Confidence labels

The approved confidence labels are registered in
[`approved_ai_terms.yaml`](../../artifacts/ai/approved_ai_terms.yaml).
They have these reserved meanings:

| Label | Meaning | Required copy |
|---|---|---|
| `Evidence-backed` | The claim cites at least one authoritative anchor and no known contradiction. | Name the anchor type and scope. |
| `Inferred` | The model connected evidence into an explanation or estimate. | Name the inference basis and avoid claim-bearing wording. |
| `Low confidence` | Evidence is missing, conflicting, stale, partial, or below the surface's confidence floor. | Name the limiting reason and remove direct apply. |
| `Validation not run` | No validation plan produced an outcome. | Do not imply tests, lint, build, or policy checks passed. |
| `Validation passed` | A named validation plan produced a passed outcome. | Name the validation plan or detail route. |
| `Needs review` | Human or policy review remains required before mutation or publication. | Name the review surface or next safe action. |

Numerical confidence MAY appear only when it comes from a calibrated
non-AI or model-eval record with a schema-defined meaning. A surface may
not invent "92% confident" from model prose.

## Preferred and forbidden wording

High-trust AI surfaces SHOULD use the approved terms rather than
surface-local synonyms. In particular:

- Use `Suggested` for model-proposed guidance that has not been
  accepted, applied, or validated.
- Use `Draft` for generated content or patches held outside canonical
  source truth.
- Use `Validation not run`, `Validation running`, `Validation failed`,
  `Validation mixed`, or `Validation passed` only from a validation
  record.
- Use `Low confidence` when evidence is stale, partial, omitted by
  policy, contradicted, or below the confidence floor.
- Use `Provider routed` or the provider-disclosure chip when route,
  region, retention, quota, cost, or data-class facts matter.
- Use `Open source`, `Prepare preview`, `Open diff`, and
  `Start sandbox run` for bounded, non-live-worktree next steps.

The forbidden register blocks wording that implies perfection,
guaranteed success, exhaustive context, current evidence, review-free
mutation, or hidden validation. Examples:

- "guaranteed", "perfect", "no risk", "always correct";
- "validated", "tested", "passing", or "safe" without a validation
  plan and outcome;
- "complete context", "all files", "nothing else affected" without a
  scope object that proves it;
- "auto-apply", "fix automatically", "no review needed", "self
  approved";
- "current", "up to date", "latest docs" when freshness is stale,
  unverified, or unknown;
- "local", "private", "free", or "first-party" when the provider route
  record says otherwise or is absent.

The complete rejection list lives in
[`forbidden_ai_terms.yaml`](../../artifacts/ai/forbidden_ai_terms.yaml).

## Context and evidence disclosure

AI copy MUST stay aligned with the same truth classes used elsewhere in
the product.

### Uncertainty

When the surface cannot establish a claim, it MUST say what is missing
or unresolved. Use "I could not confirm", "not enough cited evidence",
or `Low confidence` with a reason. Do not replace uncertainty with a
generic "maybe" if a typed reason exists.

### Partial context

Partial-context copy MUST name the covered scope and the excluded scope.
Examples:

- "Covered: current workset and two cited files. Excluded: generated
  files and tests outside this workset."
- "The index is partial; results outside the active slice are not
  represented."

Do not say "this project", "the repo", or "all callers" unless the
scope record proves that breadth.

### Omitted context

Omitted-context copy MUST name the omitted class and reason when known:
budget, policy, redaction, user deselection, unsupported source, stale
freshness floor, or tainted-source fence. A surface may summarize counts
in compact UI, but detail and export views must preserve the typed
reason.

### Stale docs or stale generated references

Stale-doc copy MUST name:

- source class, such as docs pack, mirrored runbook, generated
  reference, or release note;
- last-known-good or checked-at time when available;
- stale reason or freshness floor;
- refresh, resync, open-source, or policy-explainer route.

It MUST NOT call the answer current, latest, up to date, or validated by
fresh docs.

### Replayed results

Replay copy MUST name the replay grade, provider availability, and
mutation status from the replayability contract. A `partial_replay_*`
or `non_replayable_*` result can still be useful evidence, but it cannot
be worded as a deterministic rerun.

### Provider-routed answers

Provider-routed copy MUST preserve the provider-selection disclosure.
At minimum, the detail route or adjacent chips name provider identity,
model identity, execution locus, region posture, retention stance, and
data-class allowlist. If the selected route was not the cheapest
qualifying route, or if a fallback was taken, copy MUST name the route
selection reason or fallback state.

## Explain vs action controls

Teaching and guided-help surfaces are explanatory. They do not gain
authority by being useful.

| Control | Boundary | May do | Must not do |
|---|---|---|---|
| `Explain` | Read-only explanation | Summarize cited evidence, name limits, ask for more context | Mutate files, run commands, grant trust, approve provider calls, or apply patches |
| `Open source` | Read-only navigation | Move focus to cited source, docs anchor, symbol, diff base, run record, or evidence packet | Edit or regenerate the source |
| `Prepare preview` | Review preparation | Create or refresh a preview packet, request workspace draft, or scoped review surface | Write to canonical source, apply changes, commit, publish, or run live hooks |
| `Open diff` | Read-only review | Show an existing draft, proposed patch, generated change, or compare view | Apply the diff or mark it approved |
| `Start sandbox run` | Isolated execution | Run validation in a declared sandbox, side branch, worktree, or ephemeral environment | Run against the live worktree, publish, install, deploy, or mutate external provider state |

Direct mutation actions are separate controls. Labels such as `Apply
diff`, `Run in current workspace`, `Write file`, `Install`, `Publish`,
`Commit`, `Push`, or `Approve provider action` require the matching
interaction-safety packet, approval ticket, validation posture, and
recovery path. They MUST NOT be rendered as an explanation control or
as the only action on a low-confidence proposal.

Opaque labels are non-conforming on trust-sensitive AI surfaces:
`Fix it`, `Do it`, `Make changes`, `Autofix`, `Magic fix`, `One-click
fix`, `Continue`, `OK`, and `Apply` without the object and review
posture.

## Surface rules

### Educational AI answers

Educational answers MUST cite files, symbols, docs, commands, or packet
refs for repository truth. They may teach concepts, but they may not
turn explanation into mutation. The default controls are `Explain` and
`Open source`. `Prepare preview`, `Open diff`, or `Start sandbox run`
may appear only when the answer proposes a bounded follow-up.

### Guided help and why-unavailable explainers

Guided help MUST name the blocked or unfamiliar action, the owning
boundary, the typed reason, and the next safe action. It may route to
policy details, source, docs, preview, diff, or sandbox. It must not
silently enable trust, widen policy, run hooks, or apply mutations.

### Action proposal cards

Action proposals MUST render as `Suggested`, `Draft`, or `Needs review`
until accepted through a review surface. They MUST name:

- target identity and scope;
- evidence refs and context gaps;
- confidence label and reason;
- validation state;
- reversible next step or recovery path.

Low-confidence proposals MUST remove direct mutation controls. Their
next steps are source inspection, preview preparation, diff review,
sandbox validation, or follow-up context collection.

### Validation and apply surfaces

Validation wording is tied to a validation plan. A surface may say
`Validation passed` only when `validation_plan_ref` is non-null and the
validation outcome is `passed`. `Validation not run`, `failed`, `mixed`,
or `manual review required` remain distinct states and may not be
softened into "ready".

Apply surfaces MUST name the object, scope, consequence class,
validation state, and recovery path. Generated or AI-proposed apply
surfaces MUST preserve citation anchors for authoritative material.

## Rejection conditions

A trust-sensitive AI surface is non-conforming when it:

- uses a forbidden phrase from the forbidden term register without a
  narrower, approved contextual meaning;
- implies perfection, guaranteed success, exhaustive context, or no
  risk;
- implies validation without a validation plan and outcome;
- presents a suggested or draft change as already approved, applied, or
  claim-bearing;
- collapses `Explain`, `Open source`, `Prepare preview`, `Open diff`,
  `Start sandbox run`, and direct mutation into one control;
- hides route, provider, retention, region, cost, or data-class facts
  that the provider-selection disclosure requires;
- hides stale, partial, omitted, replayed, or provider-unavailable
  context behind generic confidence wording;
- offers direct mutation from a low-confidence action proposal;
- lets model-generated prose self-authorize policy, trust, permission,
  provider, or mutation authority.

## Change discipline

Adding a preferred term, forbidden phrase class, confidence label, or
copy case is additive when it narrows behavior and includes a fixture.
Repurposing an existing term is breaking and requires migration notes in
the same change.

The term registries are intentionally separate:

- Approved terms tell product surfaces what to say.
- Forbidden terms tell reviewers and lint tools what to reject.

Fixtures must include at least one accepted case and one rejected case
for every new high-risk term family.

## Acceptance mapping

| Acceptance clause | Resolved by |
|---|---|
| AI surfaces cannot imply perfection, guaranteed success, or review-free mutation in trust-sensitive contexts. | Forbidden register, rejection conditions, and action proposal rules. |
| Teaching surfaces separate explanation from action. | Explain/action control table and educational/guided-help rules. |
| Action proposals name reversible next steps instead of opaque auto-mutate buttons. | Action proposal card rules and approved action controls. |
| Uncertainty, partial context, omitted context, stale docs, replayed results, and provider-routed answers align with product truth classes. | Context/evidence disclosure rules and source contract references. |
| Fixtures cover educational answer with citations, low-confidence proposal, stale-doc explanation, and rejected overclaiming phrase. | `/fixtures/ai/copy_guardrail_cases/`. |
