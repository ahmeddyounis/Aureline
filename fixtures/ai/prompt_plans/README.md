# AI prompt-composer plan, request-workspace, and prompt/tool-pack worked fixtures

This directory holds worked examples for the contract frozen in
[`/docs/ai/prompt_composer_contract.md`](../../../docs/ai/prompt_composer_contract.md)
and the schemas at
[`/schemas/ai/prompt_composer_plan.schema.json`](../../../schemas/ai/prompt_composer_plan.schema.json),
[`/schemas/ai/request_workspace.schema.json`](../../../schemas/ai/request_workspace.schema.json),
and
[`/schemas/ai/prompt_tool_pack_manifest.schema.json`](../../../schemas/ai/prompt_tool_pack_manifest.schema.json).

Every file is a JSON document. A fixture that carries a single
record is a JSON object conforming to one of the three boundary
schemas (with an `__fixture__` prelude summarising the scenario,
contract sections exercised, and record kinds produced). A fixture
that carries several related records is a JSON object with
`__fixture__` plus a `records` array whose entries each conform to
one of the three boundary schemas.

No fixture embeds raw prompt text, raw retrieved-document bodies,
raw terminal / log bodies, raw generated artifact bytes, raw
request / response payloads, raw user-supplied text, raw workspace
paths, raw URLs, raw endpoint hostnames, raw API keys, raw OAuth
tokens, raw mTLS material, raw model weights, raw pack bytes, or
raw credential material. Every such field is an opaque ref or a
structured redaction-safe readout.

## Cases

| Scenario file                                              | Axis exercised                                                                                                                |
|------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------|
| `inline_completion_plan.json`                              | Prompt-composer plan in production; no tools admitted; fenced tainted-data block at slot 4; narrows provider + route + data + retention |
| `patch_flow_plan.json`                                     | Prompt-composer plan in production; admits repo instruction + check bundles; allowlisted_subset_of_policy tool narrowing      |
| `review_flow_request_workspace.json`                       | Request-workspace record for a review flow; review_workspace scope filter; redacted export posture; review-handoff link       |
| `patch_flow_working_set_and_evidence_link.json`            | Request-workspace + two working-set bindings + two evidence links (assembly and replay) for a patch flow                      |
| `first_party_default_prompt_pack.json`                     | Prompt-pack manifest in production (v3.7.1); sha256 digest; bundles four plans; three changelog entries                       |
| `first_party_default_tool_pack.json`                       | Tool-pack manifest in production (v1.4.2); blake3 digest; bundles three tool-entry refs; narrows deployment profiles          |
| `canary_prompt_pack_rollout.json`                          | Prompt-pack manifest in canary rollout (v3.8.0-canary.2); canary_cohort_ref required; one changelog entry                     |

Every fixture declares its scenario and contract sections
exercised via the `__fixture__` prelude so later coverage audits
can confirm each vocabulary member is hit at least once.

## Acceptance-criteria coverage

The seeded cases cover the three acceptance criteria named in the
spec:

- **Reviewer reconstruction without model response.**
  `inline_completion_plan.json` and `patch_flow_plan.json` spell
  out the ordered sections, admission policies, admitted source /
  trust / data sets, pinned segment refs, and fenced tainted slot
  — a reviewer reconstructs the exact prompt shape from these ids
  alone.
- **Repo-defined instructions / checks may narrow, not widen.**
  `patch_flow_plan.json` cites repo instruction bundles and check
  bundles by id in their own sections and declares
  `plan_narrows_to_listed_subset` on every narrowing axis. The
  plan's admitted provider / route / data / retention / tool
  subsets are strict subsets of the governing policy bundle.
- **Pack versions are explicit; replay and downgrade explanations
  are mechanical.** `first_party_default_prompt_pack.json` and
  `first_party_default_tool_pack.json` declare stable `pack_id`,
  `pack_version_label`, sha256 / blake3 `content_digest`, and
  typed `changelog_entries` (one `fixed_safety_defect`, one
  `narrowed_tool_allowlist`, one `added_new_tool_binding`, etc.).
  `canary_prompt_pack_rollout.json` shows a canary rollout state
  carrying the required `canary_cohort_ref`.

## Validation

Every fixture in this directory validates against its owning
schema under Draft 2020-12. A small Python harness (using
`jsonschema.Draft202012Validator`) is sufficient for local checks;
CI / governance validators consume the same schemas.
